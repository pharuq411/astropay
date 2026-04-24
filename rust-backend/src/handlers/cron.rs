use axum::{
    Json,
    extract::{Path, State, Query},
    http::HeaderMap,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{Value, json};
use tokio_postgres::types::Json as PgJson;
use tracing::warn;
use uuid::Uuid;

use crate::{
    AppState,
    auth::authorize_cron_request,
    error::AppError,
    models::Invoice,
    money_state::{
        INVOICE_ALREADY_TRANSITIONED_REASON, InvoicePaidOutcome, mark_invoice_paid_and_queue_payout,
    },
    stellar::{
        PaymentScanResult, confirm_transaction, fetch_treasury_payments,
        find_payment_for_invoice, invoice_is_expired, is_valid_account_public_key,
        TransactionStatus,
    },
    settle::{backoff_seconds, is_backoff_elapsed},
};

pub(crate) const PAYOUT_DEAD_LETTER_THRESHOLD: i32 = 5;
const DEFAULT_SETTLE_BATCH_SIZE: i64 = 50;
const RECONCILE_PAGE_SIZE: i64 = 100;
const SETTLE_PAGE_SIZE: i64 = 100;
const ORPHAN_SCAN_LIMIT: u32 = 50;

#[derive(Deserialize)]
pub struct DryRunParams {
    #[serde(default)]
    pub dry_run: bool,
    #[serde(default)]
    pub batch_size: Option<i64>,
}

#[derive(Deserialize)]
pub struct ReplayRequest {
    #[serde(rename = "publicId")]
    pub public_id: String,
    #[serde(default)]
    pub dry_run: bool,
}

#[derive(Deserialize)]
pub struct OrphanParams {
    pub limit: Option<u32>,
}

pub async fn reconcile(
    State(state): State<AppState>,
    headers: HeaderMap,
    axum::extract::Query(params): axum::extract::Query<DryRunParams>,
) -> Result<Json<Value>, AppError> {
    authorize_cron_request(&state.config.cron_secret, &headers)?;
    let dry_run = params.dry_run;
    let scan_window_hours = state.config.reconcile_scan_window_hours;
    let mut client = state.pool.get().await?;

    let mut results: Vec<Value> = Vec::new();

    let mut cursor_created_at: DateTime<Utc> = DateTime::UNIX_EPOCH;
    let mut cursor_id: Uuid = Uuid::nil();

    loop {
        let rows = client
            .query(
                "SELECT * FROM invoices
                 WHERE status = 'pending'
                   AND (created_at, id) > ($1, $2)
                   AND ($4::bigint = 0 OR created_at >= NOW() - ($4::bigint * INTERVAL '1 hour'))
                 ORDER BY created_at ASC, id ASC
                 LIMIT $3",
                &[&cursor_created_at, &cursor_id, &RECONCILE_PAGE_SIZE, &scan_window_hours],
            )
            .await?;

        let page_len = rows.len();
        if page_len == 0 {
            break;
        }

        let invoices: Vec<Invoice> = rows.iter().map(Invoice::from_row).collect();

        for invoice in invoices {
            cursor_created_at = invoice.created_at;
            cursor_id = invoice.id;

            if invoice_is_expired(&invoice, Utc::now()) {
                if !dry_run {
                    client
                        .execute(
                            "UPDATE invoices SET status = 'expired', updated_at = NOW() \
                             WHERE id = $1 AND status = 'pending'",
                            &[&invoice.id],
                        )
                        .await?;
                }
                results.push(json!({ "publicId": invoice.public_id, "action": "expired" }));
                continue;
            }

            match find_payment_for_invoice(&state.config, &invoice).await {
                Err(AppError::HorizonUnavailable) => {
                    warn!(public_id = %invoice.public_id, "Horizon unavailable during reconcile — skipping");
                    results.push(json!({ "publicId": invoice.public_id, "action": "skipped_horizon_unavailable" }));
                }
                Err(e) => return Err(e),
                Ok(PaymentScanResult::NotFound) => {
                    results.push(json!({ "publicId": invoice.public_id, "action": "pending" }));
                }
                Ok(PaymentScanResult::AssetMismatch(mismatch)) => {
                    if !dry_run {
                        client.execute(
                            "INSERT INTO payment_events (invoice_id, event_type, payload) VALUES ($1, $2, $3)",
                            &[&invoice.id, &"payment_asset_mismatch", &json!(mismatch)],
                        ).await?;
                    }
                    results.push(json!({ "publicId": invoice.public_id, "action": "asset_mismatch" }));
                }
                Ok(PaymentScanResult::Match(payment)) => {
                    let transaction = client.transaction().await?;
                    let outcome = mark_invoice_paid_and_queue_payout(
                        &transaction,
                        invoice.id,
                        &payment.hash,
                        &payment.payment,
                    ).await?;
                    transaction.commit().await?;

                    match outcome {
                        InvoicePaidOutcome::Applied { payout_queued, .. } => {
                            results.push(json!({ "publicId": invoice.public_id, "action": "paid", "txHash": payment.hash, "payoutQueued": payout_queued }));
                        }
                        InvoicePaidOutcome::AlreadyTransitioned => {
                            results.push(json!({ "publicId": invoice.public_id, "action": "skipped", "reason": "already_transitioned" }));
                        }
                    }
                }
            }
        }

        if (page_len as i64) < RECONCILE_PAGE_SIZE {
            break;
        }
    }

    // Confirmation logic
    let submitted_rows = client
        .query(
            "SELECT id, transaction_hash FROM payouts WHERE status = 'submitted' AND transaction_hash IS NOT NULL LIMIT 100",
            &[],
        ).await?;

    let mut confirmed_count = 0;
    for row in submitted_rows {
        let payout_id: Uuid = row.get("id");
        let tx_hash: String = row.get("transaction_hash");
        if let Ok(TransactionStatus::Success) = confirm_transaction(&state.config, &tx_hash).await {
            if !dry_run {
                client.execute("UPDATE payouts SET status = 'confirmed', updated_at = NOW() WHERE id = $1", &[&payout_id]).await?;
            }
            confirmed_count += 1;
        }
    }

    let body = json!({
        "dryRun": dry_run,
        "scanned": results.len(),
        "confirmed": confirmed_count,
        "results": results,
    });

    if !dry_run {
        let _ = client.execute(
            "INSERT INTO cron_runs (job_type, started_at, finished_at, success, metadata) VALUES ('reconcile', NOW(), NOW(), true, $1)",
            &[&PgJson(&body)]
        ).await;
    }

    Ok(Json(body))
}

pub async fn purge_sessions(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    authorize_cron_request(&state.config.cron_secret, &headers)?;
    let mut client = state.pool.get().await?;
    let deleted = client.execute("DELETE FROM sessions WHERE expires_at < NOW()", &[]).await?;
    let body = json!({ "deleted": deleted });
    
    let _ = client.execute(
        "INSERT INTO cron_runs (job_type, started_at, finished_at, success, metadata) VALUES ('purge_sessions', NOW(), NOW(), true, $1)",
        &[&PgJson(&body)]
    ).await;

    Ok(Json(body))
}

pub async fn archive(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    authorize_cron_request(&state.config.cron_secret, &headers)?;
    let mut client = state.pool.get().await?;
    let retention_days = state.config.archive_retention_days;
    let transaction = client.transaction().await?;

    let moved_invoices = transaction.query(
        "WITH moved AS (
           DELETE FROM invoices WHERE status = 'settled' AND settled_at < NOW() - ($1::int * INTERVAL '1 day')
           RETURNING *
         )
         INSERT INTO archived_invoices (
           id, public_id, merchant_id, description, amount_cents, currency,
           asset_code, asset_issuer, destination_public_key, memo, status,
           gross_amount_cents, platform_fee_cents, net_amount_cents,
           expires_at, paid_at, settled_at, transaction_hash, settlement_hash,
           checkout_url, qr_data_url, last_checkout_attempt_at, metadata,
           created_at, updated_at
         )
         SELECT 
           id, public_id, merchant_id, description, amount_cents, currency,
           asset_code, asset_issuer, destination_public_key, memo, status,
           gross_amount_cents, platform_fee_cents, net_amount_cents,
           expires_at, paid_at, settled_at, transaction_hash, settlement_hash,
           checkout_url, qr_data_url, last_checkout_attempt_at, metadata,
           created_at, updated_at
         FROM moved RETURNING id",
        &[&retention_days],
    ).await?;

    let count = moved_invoices.len();
    if count > 0 {
        let ids: Vec<Uuid> = moved_invoices.iter().map(|r| r.get(0)).collect();

        // Move payouts
        transaction.execute(
            "INSERT INTO archived_payouts (
                id, invoice_id, merchant_id, destination_public_key, amount_cents,
                asset_code, asset_issuer, status, transaction_hash, failure_reason,
                failure_count, last_failure_at, last_failure_reason, created_at, updated_at
            )
            SELECT 
                id, invoice_id, merchant_id, destination_public_key, amount_cents,
                asset_code, asset_issuer, status, transaction_hash, failure_reason,
                failure_count, last_failure_at, last_failure_reason, created_at, updated_at
            FROM payouts WHERE invoice_id = ANY($1)",
            &[&ids]
        ).await?;
        transaction.execute("DELETE FROM payouts WHERE invoice_id = ANY($1)", &[&ids]).await?;

        // Move payment events
        transaction.execute(
            "INSERT INTO archived_payment_events (id, invoice_id, event_type, payload, created_at)
            SELECT id, invoice_id, event_type, payload, created_at
            FROM payment_events WHERE invoice_id = ANY($1)",
            &[&ids]
        ).await?;
        transaction.execute("DELETE FROM payment_events WHERE invoice_id = ANY($1)", &[&ids]).await?;
    }

    transaction.commit().await?;
    let body = json!({ "archivedCount": count, "retentionDays": retention_days });
    
    let _ = client.execute(
        "INSERT INTO cron_runs (job_type, started_at, finished_at, success, metadata) VALUES ('archive', NOW(), NOW(), true, $1)",
        &[&PgJson(&body)]
    ).await;

    Ok(Json(body))
}

pub async fn settle(
    State(state): State<AppState>,
    headers: HeaderMap,
    axum::extract::Query(params): axum::extract::Query<DryRunParams>,
) -> Result<Json<Value>, AppError> {
    authorize_cron_request(&state.config.cron_secret, &headers)?;
    let mut client = state.pool.get().await?;
    let batch_size = params.batch_size.unwrap_or(DEFAULT_SETTLE_BATCH_SIZE).max(1);
    let mut dead_lettered = 0;
    let mut requeued = 0;

    let rows = client.query(
        "SELECT * FROM payouts WHERE status = 'failed' ORDER BY updated_at ASC LIMIT $1",
        &[&batch_size]
    ).await?;

    for row in rows {
        let payout_id: Uuid = row.get("id");
        let failure_count: i32 = row.get("failure_count");
        let new_count = failure_count + 1;

        if new_count >= PAYOUT_DEAD_LETTER_THRESHOLD {
            client.execute("UPDATE payouts SET status = 'dead_lettered', failure_count = $2 WHERE id = $1", &[&payout_id, &new_count]).await?;
            dead_lettered += 1;
        } else {
            client.execute("UPDATE payouts SET status = 'queued', failure_count = $2 WHERE id = $1", &[&payout_id, &new_count]).await?;
            requeued += 1;
        }
    }

    let body = json!({ "deadLettered": dead_lettered, "requeued": requeued });
    Ok(Json(body))
}

pub async fn replay_payout(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(payout_id): Path<Uuid>,
) -> Result<Json<Value>, AppError> {
    authorize_cron_request(&state.config.cron_secret, &headers)?;
    let mut client = state.pool.get().await?;
    client.execute("UPDATE payouts SET status = 'queued', failure_count = 0 WHERE id = $1", &[&payout_id]).await?;
    Ok(Json(json!({ "payoutId": payout_id, "status": "queued" })))
}

pub async fn orphan_payments(
    State(state): State<AppState>,
    headers: HeaderMap,
    axum::extract::Query(params): axum::extract::Query<OrphanParams>,
) -> Result<Json<Value>, AppError> {
    authorize_cron_request(&state.config.cron_secret, &headers)?;
    let limit = params.limit.unwrap_or(ORPHAN_SCAN_LIMIT).min(200);
    let treasury_payments = fetch_treasury_payments(&state.config, limit).await?;
    Ok(Json(json!({ "scanned": treasury_payments.len() })))
}

pub async fn replay_invoice(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<ReplayRequest>,
) -> Result<Json<Value>, AppError> {
    authorize_cron_request(&state.config.cron_secret, &headers)?;
    Ok(Json(json!({ "publicId": body.public_id, "status": "replayed" })))
}

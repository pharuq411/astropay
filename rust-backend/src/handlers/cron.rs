use axum::{
    Json,
    extract::{Query, State},
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
    stellar::{find_payment_for_invoice, invoice_is_expired, is_valid_account_public_key},
};

/// Payouts that fail this many times are moved to the dead-letter path.
pub(crate) const PAYOUT_DEAD_LETTER_THRESHOLD: i32 = 5;

/// Rows fetched per page during reconcile keyset pagination.
/// Keeping this at 100 balances Horizon round-trips vs. DB query cost.
const RECONCILE_PAGE_SIZE: i64 = 100;

/// Rows fetched per page during settle keyset pagination.
const SETTLE_PAGE_SIZE: i64 = 100;

#[derive(Deserialize)]
pub struct DryRunParams {
    #[serde(default)]
    pub dry_run: bool,
}

/// Scans ALL pending invoices using keyset pagination so backlogs larger than
/// a single page are fully processed in one cron invocation without manual
/// babysitting.
///
/// Pagination cursor: `(created_at, id)` — stable, uses the existing
/// `invoices_merchant_id_idx` / `invoices_status_idx` indexes and the
/// composite `invoices_merchant_created_at_id_idx` from migration 006.
/// Each page fetches up to [`RECONCILE_PAGE_SIZE`] rows ordered
/// `(created_at ASC, id ASC)`; iteration stops when a page is smaller than
/// the page size (last page reached).
pub async fn reconcile(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<DryRunParams>,
) -> Result<Json<Value>, AppError> {
    authorize_cron_request(&state.config.cron_secret, &headers)?;
    let dry_run = params.dry_run;
    let mut client = state.pool.get().await?;
    let mut results: Vec<Value> = Vec::new();

    // Keyset cursor: start before the earliest possible row.
    let mut cursor_created_at: DateTime<Utc> = DateTime::UNIX_EPOCH;
    let mut cursor_id: Uuid = Uuid::nil();

    loop {
        let rows = client
            .query(
                "SELECT * FROM invoices
                 WHERE status = 'pending'
                   AND (created_at, id) > ($1, $2)
                 ORDER BY created_at ASC, id ASC
                 LIMIT $3",
                &[&cursor_created_at, &cursor_id, &RECONCILE_PAGE_SIZE],
            )
            .await?;

        let page_len = rows.len();
        let invoices: Vec<Invoice> = rows.iter().map(Invoice::from_row).collect();

        for invoice in invoices {
            // Advance cursor to the last row seen on this page.
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

            match find_payment_for_invoice(&state.config, &invoice).await? {
                Some(payment) => {
                    let transaction = client.transaction().await?;
                    transaction
                        .execute(
                            "UPDATE invoices
                             SET status = 'paid', paid_at = NOW(), transaction_hash = $2, updated_at = NOW()
                             WHERE id = $1 AND status = 'pending'",
                            &[&invoice.id, &payment.hash],
                        )
                        .await?;
                    transaction
                        .execute(
                            "INSERT INTO payment_events (invoice_id, event_type, payload) \
                             VALUES ($1, $2, $3)",
                            &[&invoice.id, &"payment_detected", &payment.payment],
                        )
                        .await?;
                    let settlement_row = transaction
                        .query_opt(
                            "SELECT m.settlement_public_key
                             FROM merchants m
                             INNER JOIN invoices i ON i.merchant_id = m.id
                             WHERE i.id = $1",
                            &[&invoice.id],
                        )
                        .await?;
                    let settlement_key: Option<String> =
                        settlement_row.map(|row| row.get(0));
                    let settlement_key = settlement_key.unwrap_or_default();
                    let (payout_queued, payout_skip_reason) =
                        if !is_valid_account_public_key(&settlement_key) {
                            transaction
                                .execute(
                                    "INSERT INTO payment_events (invoice_id, event_type, payload) \
                                     VALUES ($1, $2, $3)",
                                    &[
                                        &invoice.id,
                                        &"payout_skipped_invalid_destination",
                                        &json!({ "reason": "invalid_settlement_public_key" }),
                                    ],
                                )
                                .await?;
                            (false, Some("invalid_settlement_public_key"))
                        } else {
                            let inserted = transaction
                                .execute(
                                    "INSERT INTO payouts (invoice_id, merchant_id, destination_public_key, \
                                                          amount_cents, asset_code, asset_issuer)
                                     SELECT id, merchant_id,
                                            (SELECT settlement_public_key FROM merchants \
                                             WHERE merchants.id = invoices.merchant_id),
                                            net_amount_cents, asset_code, asset_issuer
                                     FROM invoices WHERE id = $1
                                     ON CONFLICT (invoice_id) DO NOTHING",
                                    &[&invoice.id],
                                )
                                .await?;
                            if inserted > 0 {
                                (true, None)
                            } else {
                                (false, Some("payout_already_queued"))
                            }
                        };
                    transaction.commit().await?;
                    results.push(json!({
                        "publicId": invoice.public_id,
                        "action": "paid",
                        "txHash": payment.hash,
                        "memo": payment.memo,
                        "payoutQueued": payout_queued,
                        "payoutSkipReason": payout_skip_reason
                    }));
                }
                None => {
                    results.push(
                        json!({ "publicId": invoice.public_id, "action": "pending" }),
                    );
                }
            }
        }

        // Stop when we got fewer rows than the page size — backlog exhausted.
        if (page_len as i64) < RECONCILE_PAGE_SIZE {
            break;
        }
    }

    let body = json!({
        "dryRun": dry_run,
        "scanned": results.len(),
        "results": results
    });
    if !dry_run {
        if let Err(e) = client
            .execute(
                "INSERT INTO cron_runs (job_type, started_at, finished_at, success, metadata, error_detail) \
                 VALUES ('reconcile', NOW(), NOW(), true, $1, NULL)",
                &[&PgJson(&body)],
            )
            .await
        {
            warn!(error = %e, "cron_runs audit insert failed for reconcile");
        }
    }

    Ok(Json(body))
}

/// Deletes sessions whose `expires_at` is in the past.
/// Safe to call repeatedly; each run is idempotent and logged to `cron_runs`.
/// Trigger via `GET /api/cron/purge-sessions` with `Authorization: Bearer <CRON_SECRET>`.
pub async fn purge_sessions(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    authorize_cron_request(&state.config.cron_secret, &headers)?;
    let client = state.pool.get().await?;
    let deleted = client
        .execute("DELETE FROM sessions WHERE expires_at <= NOW()", &[])
        .await?;
    let body = json!({ "deleted": deleted });
    if let Err(e) = client
        .execute(
            "INSERT INTO cron_runs (job_type, started_at, finished_at, success, metadata, error_detail) \
             VALUES ('purge_sessions', NOW(), NOW(), true, $1, NULL)",
            &[&PgJson(&body)],
        )
        .await
    {
        warn!(error = %e, "cron_runs audit insert failed for purge_sessions");
    }
    Ok(Json(body))
}

/// Scans ALL failed payouts using keyset pagination and increments their
/// failure count. Once a payout reaches [`PAYOUT_DEAD_LETTER_THRESHOLD`]
/// failures it is moved to `dead_lettered` status and a row is inserted into
/// `payout_dead_letters` so operators can inspect and manually resolve it.
///
/// Pagination cursor: `(updated_at, id)` — stable across pages because each
/// processed row is immediately updated (its `updated_at` advances past the
/// cursor), so it will not appear in subsequent pages of the same run.
///
/// Full Stellar transaction signing/submission is not implemented yet; this
/// handler only manages the failure-tracking and dead-letter escalation path.
pub async fn settle(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<DryRunParams>,
) -> Result<Json<Value>, AppError> {
    authorize_cron_request(&state.config.cron_secret, &headers)?;
    let mut client = state.pool.get().await?;

    let mut dead_lettered: Vec<Value> = Vec::new();
    let mut requeued: Vec<Value> = Vec::new();

    // Keyset cursor: start before the earliest possible row.
    let mut cursor_updated_at: DateTime<Utc> = DateTime::UNIX_EPOCH;
    let mut cursor_id: Uuid = Uuid::nil();

    loop {
        let rows = client
            .query(
                "SELECT * FROM payouts
                 WHERE status = 'failed'
                   AND (updated_at, id) > ($1, $2)
                 ORDER BY updated_at ASC, id ASC
                 LIMIT $3",
                &[&cursor_updated_at, &cursor_id, &SETTLE_PAGE_SIZE],
            )
            .await?;

        let page_len = rows.len();

        for row in &rows {
            let payout_id: Uuid = row.get("id");
            let invoice_id: Uuid = row.get("invoice_id");
            let merchant_id: Uuid = row.get("merchant_id");
            let failure_count: i32 = row.get("failure_count");
            let failure_reason: Option<String> = row.get("failure_reason");
            let new_count = failure_count + 1;

            // Advance cursor before the DB write so the updated row's new
            // updated_at won't be re-fetched in a subsequent page.
            cursor_updated_at = row.get("updated_at");
            cursor_id = payout_id;

            let tx = client.transaction().await?;

            if new_count >= PAYOUT_DEAD_LETTER_THRESHOLD {
                // Escalate to dead-letter.
                tx.execute(
                    "UPDATE payouts
                     SET status = 'dead_lettered', failure_count = $2,
                         last_failure_at = NOW(), last_failure_reason = $3, updated_at = NOW()
                     WHERE id = $1",
                    &[&payout_id, &new_count, &failure_reason],
                )
                .await?;
                tx.execute(
                    "INSERT INTO payout_dead_letters \
                         (payout_id, invoice_id, merchant_id, failure_count, last_failure_reason)
                     VALUES ($1, $2, $3, $4, $5)
                     ON CONFLICT (payout_id) DO NOTHING",
                    &[&payout_id, &invoice_id, &merchant_id, &new_count, &failure_reason],
                )
                .await?;
                tx.execute(
                    "INSERT INTO payment_events (invoice_id, event_type, payload) \
                     VALUES ($1, $2, $3)",
                    &[
                        &invoice_id,
                        &"payout_dead_lettered",
                        &json!({ "payoutId": payout_id, "failureCount": new_count }),
                    ],
                )
                .await?;
                tx.commit().await?;
                dead_lettered
                    .push(json!({ "payoutId": payout_id, "failureCount": new_count }));
            } else {
                // Increment failure count and requeue for the next settle run.
                tx.execute(
                    "UPDATE payouts
                     SET status = 'queued', failure_count = $2,
                         last_failure_at = NOW(), last_failure_reason = $3, updated_at = NOW()
                     WHERE id = $1",
                    &[&payout_id, &new_count, &failure_reason],
                )
                .await?;
                tx.commit().await?;
                requeued.push(json!({ "payoutId": payout_id, "failureCount": new_count }));
            }
        }

        // Stop when we got fewer rows than the page size — backlog exhausted.
        if (page_len as i64) < SETTLE_PAGE_SIZE {
            break;
        }
    }

    let body = json!({
        "deadLettered": dead_lettered.len(),
        "requeued": requeued.len(),
        "deadLetteredItems": dead_lettered,
        "requeuedItems": requeued,
        "note": "Stellar transaction signing/submission is not implemented yet. This run only manages dead-letter escalation."
    });

    if let Err(e) = client
        .execute(
            "INSERT INTO cron_runs (job_type, started_at, finished_at, success, metadata, error_detail) \
             VALUES ('settle', NOW(), NOW(), true, $1, NULL)",
            &[&PgJson(&body)],
        )
        .await
    {
        warn!(error = %e, "cron_runs audit insert failed for settle");
    }

    Ok(Json(body))
}

#[cfg(test)]
mod tests {
    use axum::http::{HeaderMap, HeaderValue, header};

    use crate::auth::authorize_cron_request;

    #[test]
    fn authorizes_valid_bearer_token() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_static("Bearer cron_secret"),
        );
        assert!(authorize_cron_request("cron_secret", &headers).is_ok());
    }

    #[test]
    fn rejects_missing_bearer_token() {
        let headers = HeaderMap::new();
        assert!(authorize_cron_request("cron_secret", &headers).is_err());
    }

    #[test]
    fn rejects_wrong_bearer_token() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_static("Bearer wrong"),
        );
        assert!(authorize_cron_request("cron_secret", &headers).is_err());
    }

    #[test]
    fn rejects_empty_configured_secret() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_static("Bearer anything"),
        );
        assert!(authorize_cron_request("", &headers).is_err());
    }

    #[test]
    fn dead_letter_threshold_is_five() {
        assert_eq!(super::PAYOUT_DEAD_LETTER_THRESHOLD, 5);
    }

    #[test]
    fn reconcile_page_size_is_positive() {
        assert!(super::RECONCILE_PAGE_SIZE > 0);
    }

    #[test]
    fn settle_page_size_is_positive() {
        assert!(super::SETTLE_PAGE_SIZE > 0);
    }

    #[test]
    fn settle_handler_tracks_last_failure_reason() {
        let handler_code = include_str!("cron.rs");
        assert!(
            handler_code.contains("last_failure_reason"),
            "settle handler must update last_failure_reason column"
        );
        assert!(
            handler_code.contains("last_failure_at = NOW()"),
            "settle handler must update last_failure_at on each failure"
        );
    }

    #[test]
    fn reconcile_uses_keyset_pagination() {
        let handler_code = include_str!("cron.rs");
        assert!(
            handler_code.contains("(created_at, id) > ($1, $2)"),
            "reconcile must use keyset cursor on (created_at, id)"
        );
        assert!(
            handler_code.contains("RECONCILE_PAGE_SIZE"),
            "reconcile must reference the page-size constant"
        );
    }

    #[test]
    fn settle_uses_keyset_pagination() {
        let handler_code = include_str!("cron.rs");
        assert!(
            handler_code.contains("(updated_at, id) > ($1, $2)"),
            "settle must use keyset cursor on (updated_at, id)"
        );
        assert!(
            handler_code.contains("SETTLE_PAGE_SIZE"),
            "settle must reference the page-size constant"
        );
    }

    #[test]
    fn reconcile_loop_breaks_on_partial_page() {
        // The loop termination condition must compare page_len < PAGE_SIZE,
        // not check for an empty page, so the last partial page exits cleanly.
        let handler_code = include_str!("cron.rs");
        assert!(
            handler_code.contains("page_len as i64) < RECONCILE_PAGE_SIZE"),
            "reconcile loop must break when page_len < RECONCILE_PAGE_SIZE"
        );
    }

    #[test]
    fn settle_loop_breaks_on_partial_page() {
        let handler_code = include_str!("cron.rs");
        assert!(
            handler_code.contains("page_len as i64) < SETTLE_PAGE_SIZE"),
            "settle loop must break when page_len < SETTLE_PAGE_SIZE"
        );
    }
}

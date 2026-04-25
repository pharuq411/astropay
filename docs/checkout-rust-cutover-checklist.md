# Checkout Rust Cutover Checklist (AP-248)

This document names the tests, rollout gates, and rollback triggers required
before the Rust backend owns the checkout flow end-to-end. Today Next.js owns
checkout XDR generation and submission; Rust returns `501 Not Implemented` for
those routes.

---

## What "checkout" means here

| Route | Current owner | Rust status |
|---|---|---|
| `POST /api/invoices/:id/checkout` — build + submit XDR | Next.js | `501` |
| `GET /api/cron/settle` — merchant settlement execution | Next.js | `501` |
| `GET /api/cron/reconcile` — Horizon payment detection | **Rust** | ✅ live |
| `POST /api/webhooks/stellar` — webhook payment marking | **Rust** | ✅ live |

---

## Required tests before cutover

### 1. XDR generation unit tests
- Build a valid Stellar `PaymentOp` XDR for a known invoice fixture and assert the base64 output is stable.
- Cover: correct asset code/issuer, correct destination, correct amount (cents → stroops), correct memo.
- Cover: `amount_cents = 0` and negative amounts are rejected before XDR is built.

### 2. XDR submission integration tests
- Submit the built XDR to Testnet Horizon and assert the response contains a `transaction_hash`.
- Assert the invoice transitions from `pending` → `paid` after a successful submission.
- Assert a duplicate submission (same memo) returns the existing `transaction_hash` without creating a second payment event.

### 3. Settlement execution tests
- `GET /api/cron/settle` processes a `queued` payout, signs with `PLATFORM_TREASURY_SECRET_KEY`, submits to Horizon, and marks the payout `settled`.
- Assert `settled_at >= paid_at` (constraint from migration `015`).
- Assert a missing or invalid `PLATFORM_TREASURY_SECRET_KEY` returns a clear error and does not submit any transaction.

### 4. End-to-end smoke test (Testnet)
- Register merchant → create invoice → load checkout page → submit payment → confirm invoice status is `paid` → confirm payout is `settled`.
- Run against Testnet, not Mainnet.

### 5. Existing Rust test suite must stay green
```bash
cd rust-backend && cargo test
```

---

## Rollout gates (all must pass before flipping traffic)

1. **`cargo test` passes** with no ignored tests related to checkout or settlement.
2. **Testnet smoke test passes** end-to-end (see §4 above).
3. **`npm run typecheck && npm run build`** still passes in `usdc-payment-link-tool/` — the Next.js app remains the web frontend.
4. **Environment variables confirmed present** on the target deployment:
   - `PLATFORM_TREASURY_SECRET_KEY`
   - `PLATFORM_TREASURY_PUBLIC_KEY`
   - `HORIZON_URL`
   - `NETWORK_PASSPHRASE`
   - `STELLAR_NETWORK`
5. **`PLATFORM_TREASURY_PUBLIC_KEY` matches `PLATFORM_TREASURY_SECRET_KEY`** — verify with `stellar-sdk` or `stellar-cli` before deploy; a mismatch causes every settlement to fail silently.
6. **Migration `015_invoice_settled_after_paid_check.sql` applied** — the DB will reject any settlement that precedes payment.
7. **Cron schedule confirmed** — `vercel.json` / `railway.json` cron entries point at the Rust service, not the Next.js route handlers.

---

## Rollback triggers

Roll back immediately (revert traffic to Next.js checkout routes) if any of the following occur within the first 24 hours after cutover:

| Trigger | Signal |
|---|---|
| Invoice stuck in `pending` after confirmed Testnet payment | Reconcile cron not detecting payment |
| Payout stuck in `queued` for > 30 min after invoice marked `paid` | Settle cron not running or failing silently |
| Any `500` or `501` from `/api/invoices/:id/checkout` | XDR build/submit not implemented or crashing |
| `settled_at < paid_at` constraint violation in Postgres | Settlement logic writing timestamps out of order |
| `PLATFORM_TREASURY_SECRET_KEY` logged or leaked in any response | Immediate security rollback + key rotation |

### Rollback procedure

1. Re-point the reverse proxy / Vercel rewrite rules to the Next.js route handlers for checkout and cron routes.
2. Confirm `npm run db:migrate` is current on the Next.js side.
3. File a post-mortem issue before re-attempting cutover.

---

## After cutover

Once Rust owns checkout and settlement in production:

- Remove the `501` stubs from `rust-backend/src/handlers/`.
- Delete or archive the corresponding Next.js route handlers (`app/api/invoices/[id]/checkout/route.ts`, `app/api/cron/settle/route.ts`).
- Update `vercel.json` and `railway.json` to remove cron entries that now belong to the Rust service.
- Update `usdc-payment-link-tool/DEPLOY_CHECKLIST.md` to reflect the reduced Next.js surface.

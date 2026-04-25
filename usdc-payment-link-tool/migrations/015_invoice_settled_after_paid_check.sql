-- AP-189: Enforce payment-before-settlement ordering on invoices.
--
-- settled_at must not precede paid_at. Both columns are nullable; the
-- constraint only fires when both are non-NULL, so pending/paid-only rows
-- are unaffected.

ALTER TABLE invoices
    ADD CONSTRAINT invoices_settled_after_paid_check
    CHECK (settled_at IS NULL OR paid_at IS NULL OR settled_at >= paid_at);

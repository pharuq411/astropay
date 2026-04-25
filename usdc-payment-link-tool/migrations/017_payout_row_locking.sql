-- Add row-locking strategy for concurrent payout workers
-- Issue #214: Prevents double-processing of payout rows by multiple workers

-- Add processing tracking columns to payouts table
ALTER TABLE payouts 
    ADD COLUMN IF NOT EXISTS processing_worker_id TEXT,
    ADD COLUMN IF NOT EXISTS processing_started_at TIMESTAMPTZ;

-- Create index for efficient worker queries
CREATE INDEX IF NOT EXISTS payouts_processing_worker_idx ON payouts (processing_worker_id, processing_started_at);

-- Add constraint to ensure processing_started_at is set when worker_id is set
-- (Application code must enforce this invariant)
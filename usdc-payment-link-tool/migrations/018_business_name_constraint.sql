-- Add constraint preventing empty business_name
-- Issue #220: Ensures merchants cannot persist blank business names

-- Add check constraint to prevent empty or whitespace-only business names
ALTER TABLE merchants 
    ADD CONSTRAINT IF NOT EXISTS merchants_business_name_not_empty 
    CHECK (LENGTH(TRIM(business_name)) > 0);
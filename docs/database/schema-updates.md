# Database Schema Updates

## Row-Locking Strategy for Concurrent Payout Workers (Issue #214)

### Overview
Added row-level locking mechanism to prevent concurrent payout workers from double-processing the same payout.

### Implementation
- **Migration**: `017_payout_row_locking.sql`
- **New columns**: `processing_worker_id`, `processing_started_at`
- **Functions**: `claim_payout_for_processing()`, `release_payout_from_processing()`

### Usage
```rust
// Claim a payout for processing
let claimed = claim_payout_for_processing(&client, payout_id, "worker-1").await?;
if claimed {
    // Process the payout
    // ...
    // Release when done
    release_payout_from_processing(&client, payout_id).await?;
}
```

## PGSSL Mode Validation (Issue #217)

### Overview
Added startup validation for PostgreSQL SSL configuration to catch invalid modes early.

### Implementation
- **Function**: `validate_ssl_mode()`
- **Valid modes**: disable, allow, prefer, require, verify-ca, verify-full
- **Integration**: Called during pool creation in `create_pool()`

### Error Handling
Invalid SSL modes will cause the application to fail at startup with a clear error message listing valid options.

## Business Name Constraint (Issue #220)

### Overview
Added database constraint to prevent empty or whitespace-only business names.

### Implementation
- **Migration**: `018_business_name_constraint.sql`
- **Constraint**: `merchants_business_name_not_empty`
- **Rule**: `LENGTH(TRIM(business_name)) > 0`

### Behavior
Any attempt to insert or update a merchant with an empty business name will be rejected by the database.

## Performance Test Fixtures (Issue #219)

### Overview
Added large-volume test dataset for performance testing and load simulation.

### Implementation
- **Migration**: `019_performance_test_fixtures.sql`
- **Data volume**: 10 merchants, 10,000 invoices, ~1,000 payouts, 50,000 events
- **Cleanup function**: `cleanup_performance_test_data()`

### Usage
```sql
-- View test data summary
SELECT * FROM performance_test_summary;

-- Clean up test data
SELECT cleanup_performance_test_data();
```

### Test Data Characteristics
- Realistic distribution of invoice statuses
- Varied payout states including failures
- Multiple event types per invoice
- Merchant data spread across multiple businesses
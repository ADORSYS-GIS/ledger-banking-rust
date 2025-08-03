# Database Cleanup System for Test Isolation

## Overview

This directory contains a complete database cleanup system designed to solve test data pollution issues by providing reliable database reset functionality between tests.

## Files

### Core Components

- **`commons.rs`** - Database utilities and helper functions
  - `cleanup_database()` - Main cleanup function
  - `create_test_person()` - Create standard test person
  - `create_test_account()` - Create standard test account  
  - `setup_test_db()` - Complete test environment setup

- **`fixtures/cleanup.sql`** - PostgreSQL cleanup script
  - Conditional table truncation using `information_schema`
  - Foreign key constraint handling with `CASCADE`
  - Safe execution with `DO` blocks

### Example Usage

- **`cleanup_demo.rs`** - Working examples of cleanup patterns
- **`example_with_cleanup.rs`** - Original examples (may have concurrency issues)

## Usage Patterns

### Basic Pattern

```rust
#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_with_cleanup() {
    // Setup
    let (pool, person_id, account_id) = setup_test_db().await;
    cleanup_database(&pool).await; // Start with clean state
    
    // Recreate prerequisites after cleanup
    let person_id = create_test_person(&pool).await;
    let account_id = create_test_account(&pool, person_id).await;
    
    // Your test logic here...
    
    // Optional cleanup at end
    cleanup_database(&pool).await;
}
```

### Advanced Pattern

```rust
#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_isolated_operations() {
    let (pool, _person_id, _account_id) = setup_test_db().await;
    cleanup_database(&pool).await; // Ensure clean start
    
    // Recreate test data as needed
    let person_id = create_test_person(&pool).await;
    let account_id = create_test_account(&pool, person_id).await;
    
    // Test operations with guaranteed isolation
    let repo = WorkflowRepositoryImpl::new(pool.clone());
    
    // Create test data
    let workflow = create_workflow_for_test(Uuid::new_v4(), account_id, person_id);
    repo.create_workflow(&workflow).await.expect("Failed to create workflow");
    
    // Verify operations
    let count = repo.count_all_workflows().await.expect("Failed to count");
    assert_eq!(count, 1, "Should have exactly 1 workflow");
    
    // Cleanup
    cleanup_database(&pool).await;
}
```

## Technical Details

### Cleanup Script Features

- **Conditional Execution**: Only truncates tables that exist
- **Foreign Key Safety**: Uses `CASCADE` to handle dependencies
- **Transaction Safety**: Disables foreign key checks temporarily
- **Error Resilience**: Continues execution even if some tables don't exist

### Foreign Key Dependencies

The cleanup system properly handles these key relationships:
- `account_workflows.account_id` → `accounts.account_id`
- `account_workflows.initiated_by` → `persons.person_id`
- `accounts.updated_by` → `persons.person_id`

### Test Prerequisites

Most workflow tests require:
1. **Person record** - For `initiated_by` foreign key
2. **Account record** - For `account_id` foreign key
3. **Proper cleanup** - Between tests for isolation

## Best Practices

### ✅ Do This

1. **Always cleanup first**: `cleanup_database(&pool).await` at test start
2. **Recreate prerequisites**: Use helper functions after cleanup
3. **Use fixed UUIDs**: For predictable test data
4. **Single-threaded tests**: Add `-- --test-threads=1` if needed

### ❌ Avoid This

1. **Don't assume data exists**: Always recreate after cleanup
2. **Don't skip cleanup**: Data pollution will cause random failures
3. **Don't rely on insertion order**: Use explicit verification
4. **Don't use VACUUM**: Not supported in transaction blocks

## Testing the System

### Run Individual Tests
```bash
env DATABASE_URL=postgresql://user:password@localhost:5432/mydb \
cargo test test_cleanup_isolation_demo --features postgres_tests
```

### Run All Cleanup Tests
```bash
env DATABASE_URL=postgresql://user:password@localhost:5432/mydb \
cargo test --test cleanup_demo --features postgres_tests
```

### Debug with Output
```bash
env DATABASE_URL=postgresql://user:password@localhost:5432/mydb \
cargo test test_workflow_crud_basic --features postgres_tests -- --nocapture
```

## Status

- ✅ **Core cleanup system**: Fully functional
- ✅ **Individual tests**: Pass reliably 
- ✅ **Foreign key handling**: Complete
- ✅ **Error resilience**: Handles missing tables
- ⚠️ **Concurrent tests**: May have isolation issues (use `--test-threads=1`)

## Migration from UUID-based Isolation

**Old approach (UUID-based):**
```rust
let unique_id = Uuid::new_v4();
let product_code = format!("FD{}", &unique_id.to_string()[0..6]);
```

**New approach (cleanup-based):**
```rust
cleanup_database(&pool).await;
let person_id = create_test_person(&pool).await;
let account_id = create_test_account(&pool, person_id).await;
```

## Future Improvements

1. **TestDatabaseGuard**: Implement proper async Drop when Rust supports it
2. **Parallel Test Support**: Investigate database-per-test approach
3. **Performance**: Optimize cleanup script for large schemas
4. **Documentation**: Add more complex workflow examples

## Troubleshooting

### Foreign Key Violations
- Ensure `create_test_person()` and `create_test_account()` are called after cleanup
- Verify both functions complete successfully before creating workflows

### Missing Tables
- Update `cleanup.sql` to include new tables with conditional checks
- Use `information_schema.tables` pattern for safety

### Test Flakiness
- Add debug output to verify prerequisite data exists
- Use `--test-threads=1` to eliminate concurrency issues
- Verify cleanup completes successfully before test operations
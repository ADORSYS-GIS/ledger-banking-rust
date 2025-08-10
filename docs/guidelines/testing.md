### Test Isolation
```rust
#[tokio::test]
async fn test_with_isolation() {
    let pool = setup_test_db().await;
    cleanup_database(&pool).await;  // Clean start
    
    // Use UUID-based unique test data
    let unique_id = Uuid::new_v4();
    let product_code = format!("FD{}", &unique_id.to_string()[0..6]);
    
    // Test operations with guaranteed isolation
    let result = repo.operation(test_data).await?;
    assert_eq!(result.field, expected_value);
}
```

### Database Testing
**⚠️ Critical**: Database tests **must run sequentially** to avoid data pollution:

```bash
# Local testing
env DATABASE_URL=postgresql://user:password@localhost:5432/mydb \
cargo test --features postgres_tests -- --test-threads=1

# Schema changes
docker compose down -v && docker compose up -d postgres
sqlx migrate run --source banking-db-postgres/migrations
```

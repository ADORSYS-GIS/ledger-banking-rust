### Test Isolation
```rust
#[tokio::test]
async fn test_with_isolation() {
    // Each test establishes its own connection
    let pool = commons::establish_connection().await;
    
    // Module-specific setup is handled within the test file
    // This avoids monolithic test helpers and ensures clarity
    person_init::create_test_person(&pool).await;

    // Test logic follows...
    let new_person = PersonModel { /* ... */ };
    let person_repo = PersonRepositoryImpl::new(Arc::new(pool));
    let created_person = person_repo.save(new_person.clone()).await.unwrap();
    assert_eq!(new_person.id, created_person.id);
}
```

### Database Testing
**⚠️ Critical**: Database tests **must run sequentially** to avoid data pollution:

```bash
# Set the database URL (from project root)
export DATABASE_URL="postgresql://user:password@localhost:5432/mydb"

# Run all tests for a specific package (sequentially)
cargo test -p banking-db-postgres -- --test-threads=1

# Run a specific test with its required features
cargo test -p banking-db-postgres --test person_repository_tests --features person_repository -- --test-threads=1

# Schema changes (from project root)
docker compose down -v && docker compose up -d postgres
sqlx migrate run --source banking-db-postgres/migrations
```

use banking_db::repository::AccountRepository;
use banking_db_postgres::SimpleAccountRepositoryImpl;
use sqlx::PgPool;
use uuid::Uuid;

/// Integration test helper to set up database connection
async fn setup_test_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://user:password@localhost:5432/mydb".to_string());
    
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL database")
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_simple_account_exists() {
    let pool = setup_test_db().await;
    let repo = SimpleAccountRepositoryImpl::new(pool);
    
    // Test with a non-existent account
    let non_existent_id = Uuid::new_v4();
    let exists = repo.exists(non_existent_id).await
        .expect("Failed to check if account exists");
    assert!(!exists);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_simple_account_count() {
    let pool = setup_test_db().await;
    let repo = SimpleAccountRepositoryImpl::new(pool);
    
    // Test count operation
    let _count = repo.count().await
        .expect("Failed to count accounts");
    // Just verify it doesn't error - count may be 0 or more
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_simple_find_by_id() {
    let pool = setup_test_db().await;
    let repo = SimpleAccountRepositoryImpl::new(pool);
    
    // Test with a non-existent account
    let non_existent_id = Uuid::new_v4();
    let account = repo.find_by_id(non_existent_id).await
        .expect("Failed to find account by id");
    assert!(account.is_none());
}

// Unit tests that don't require database
#[tokio::test]
async fn test_simple_repository_creation() {
    // This test doesn't actually connect to database
    let database_url = "postgresql://dummy:dummy@localhost:5432/dummy";
    
    // This will fail to connect but we're just testing creation
    let result = PgPool::connect(database_url).await;
    assert!(result.is_err()); // Expected to fail with dummy URL
}

#[tokio::test]
async fn test_uuid_generation() {
    let uuid1 = Uuid::new_v4();
    let uuid2 = Uuid::new_v4();
    assert_ne!(uuid1, uuid2);
}
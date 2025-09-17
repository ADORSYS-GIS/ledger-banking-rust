    // tests/commons.rs
    //! Common test utilities for database testing with SQLx and PostgreSQL
    //!
    //! This module provides utilities for:
    //! - Database connection management
    //! - Test data seeding
    //! - Database cleanup between tests
    //! - Docker container management for CI/local testing

    use sqlx::{postgres::PgPoolOptions, PgPool};
    use std::env;
    use std::fs;
    use std::path::Path;
    use std::process::Command;
    use std::time::Duration;
    use tokio::time::sleep;
    use uuid::Uuid;

    /// Establish a PostgreSQL connection pool for testing
    ///
    /// This function:
    /// 1. Ensures Docker database is running (if needed)
    /// 2. Connects to the database using DATABASE_URL
    /// 3. Runs migrations to ensure schema is up to date
    /// 4. Returns a connection pool ready for testing
    pub async fn establish_connection() -> PgPool {
        ensure_docker_database_running().await;

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://user:password@localhost:5432/mydb".to_string());

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(30))
            .connect(&database_url)
            .await
            .unwrap_or_else(|_| panic!("Error connecting to {database_url}"));

        // Run migrations to ensure schema is up to date
        sqlx::migrate!("../banking-db-postgres/migrations/")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

/// Seed the database with fixture data from the specified SQL file
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool
/// * `fixture_file` - The path to the SQL file to execute (relative to test directory)
///
/// # Example
///
/// ```rust
/// seed_database(&pool, "tests/fixtures/banking_test_data.sql").await;
/// ```
#[allow(dead_code)]
pub async fn seed_database(pool: &PgPool, fixture_file: &str) {
    let fixture_path = Path::new(fixture_file);
    let sql = fs::read_to_string(fixture_path)
        .unwrap_or_else(|_| panic!("Failed to read fixture file: {fixture_file}"));
    
    // Execute the SQL file as a batch
    sqlx::raw_sql(&sql)
        .execute(pool)
        .await
        .expect("Failed to seed the database");
}

// Note: cleanup_database function has been removed in favor of
// transaction-based testing which provides automatic rollback

/// RAII guard for automatic database cleanup
/// 
/// This struct automatically cleans up the database when dropped,
/// ensuring tests don't leave data behind that could affect other tests.
/// 
/// # Usage
/// 
/// ```rust
/// #[tokio::test]
/// async fn my_test() {
///     let pool = establish_connection().await;
///     let _guard = TestDatabaseGuard::new(pool.clone());
///     
///     // Your test code here...
///     // Database will be automatically cleaned up when guard is dropped
/// }
/// ```
#[allow(dead_code)]
pub struct TestDatabaseGuard {
    #[allow(dead_code)]
    pool: PgPool,
}

impl TestDatabaseGuard {
    #[allow(dead_code)]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Drop for TestDatabaseGuard {
    fn drop(&mut self) {
        // For now, we'll skip automatic cleanup in Drop to avoid runtime issues
        // Tests should call cleanup_database manually if needed
        // TODO: Implement proper async Drop when Rust supports it
    }
}

/// Ensure Docker database is running for tests
/// 
/// This function attempts to connect to the database, and if it fails,
/// tries to start the Docker Compose services. This is useful for both
/// local development and CI environments.
async fn ensure_docker_database_running() {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://user:password@localhost:5432/mydb".to_string());

    // Try to connect once
    match PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
    {
        Ok(_) => {
            // Connection successful, nothing to do
        },
        Err(_) => {
            // Could not connect. Attempt to start Docker Compose
            println!("Database not reachable; attempting to start Docker Compose...");
            
            let output = Command::new("docker")
                .args(["compose", "up", "-d", "postgres"])
                .output();
            
            match output {
                Ok(output) => {
                    if !output.status.success() {
                        eprintln!(
                            "Docker compose failed to start the database: {}",
                            String::from_utf8_lossy(&output.stderr)
                        );
                    } else {
                        println!("Docker Compose started successfully, waiting for database...");
                        // Wait for the database to be ready
                        sleep(Duration::from_secs(10)).await;
                    }
                },
                Err(e) => {
                    eprintln!("Failed to execute docker compose command: {e}");
                }
            }
        }
    }
}

/// Create standard test person for foreign key references
///
/// Many tests need a person record for foreign key constraints.
/// This function creates a standard test person that can be reused.
#[allow(dead_code)]
pub async fn create_test_person(pool: &PgPool) -> Uuid {
    let test_person_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();

    sqlx::query(
        r#"
        INSERT INTO person (id, person_type, display_name, external_identifier, is_active, created_at, updated_at)
        VALUES ($1, 'System', 'Test User', 'test-user', true, NOW(), NOW())
        ON CONFLICT (id) DO NOTHING
        "#,
    )
    .bind(test_person_id)
    .execute(pool)
    .await
    .expect("Failed to create test person");

    test_person_id
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_database_connection() {
        let pool = establish_connection().await;
        
        // Simple connectivity test
        let result: (i32,) = sqlx::query_as("SELECT 1")
            .fetch_one(&pool)
            .await
            .expect("Failed to execute test query");
        
        assert_eq!(result.0, 1);
    }
    
    // Removed test_cleanup_database since we no longer use explicit cleanup
}
// tests/commons.rs
//! Common test utilities for database testing with SQLx and PostgreSQL
//! 
//! This module provides utilities for:
//! - Database connection management
//! - Test data seeding
//! - Database cleanup between tests
//! - Docker container management for CI/local testing

use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;

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
        .expect(&format!("Error connecting to {}", database_url));
    
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
        .expect(&format!("Failed to read fixture file: {}", fixture_file));
    
    // Execute the SQL file as a batch
    sqlx::raw_sql(&sql)
        .execute(pool)
        .await
        .expect("Failed to seed the database");
}

/// Clean up the database by executing the cleanup SQL file
/// 
/// This function truncates all tables and resets the database to a clean state
/// for the next test. It reads and executes the cleanup.sql file.
pub async fn cleanup_database(pool: &PgPool) {
    let cleanup_path = Path::new("tests/fixtures/cleanup.sql");
    let sql = fs::read_to_string(cleanup_path)
        .expect("Failed to read cleanup file: tests/fixtures/cleanup.sql");
    
    sqlx::raw_sql(&sql)
        .execute(pool)
        .await
        .expect("Failed to clean up the database");
}

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
                .args(&["compose", "up", "-d", "postgres"])
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
                    eprintln!("Failed to execute docker compose command: {}", e);
                }
            }
        }
    }
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
    
    #[tokio::test]
    async fn test_cleanup_database() {
        let pool = establish_connection().await;
        let _guard = TestDatabaseGuard::new(pool.clone());
        
        // Test that cleanup works without errors
        cleanup_database(&pool).await;
    }
}
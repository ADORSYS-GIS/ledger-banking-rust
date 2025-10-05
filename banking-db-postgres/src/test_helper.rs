//! Test helper module for transaction-based test isolation
//!
//! This module provides utilities for running tests within database transactions
//! that are automatically rolled back, ensuring perfect test isolation without
//! the need for explicit cleanup operations.

use banking_db::repository::{PersonRepos, UnitOfWork, UnitOfWorkSession};
use crate::repository::unit_of_work_impl::PostgresUnitOfWork;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::time::Duration;

/// Test context that provides a transactional database session
/// 
/// This struct holds a UnitOfWork session that will be automatically
/// rolled back when dropped, ensuring test isolation.
pub struct TestContext<S: UnitOfWorkSession<sqlx::Postgres>> {
    pub session: S,
}

impl<S: UnitOfWorkSession<sqlx::Postgres>> TestContext<S> {
    /// Get the person repositories from the session
    pub fn person_repos(&self) -> &S::PersonRepos {
        self.session.person_repos()
    }
}

/// Setup a test context with a transactional database session
///
/// This function creates a new database connection pool, starts a transaction,
/// and returns a TestContext that will automatically roll back the transaction
/// when dropped.
///
/// # Example
///
/// ```rust
/// #[tokio::test]
/// async fn test_example() -> Result<(), Box<dyn std::error::Error>> {
///     let ctx = setup_test_context().await?;
///     let person_repo = ctx.person_repos().persons();
///
///     // Perform test operations...
///     // All changes will be rolled back when ctx is dropped
///
///     Ok(())
/// }
/// ```
pub async fn setup_test_context() -> Result<TestContext<crate::repository::unit_of_work_impl::PostgresUnitOfWorkSession>, Box<dyn std::error::Error + Send + Sync>> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://user:password@localhost:5432/mydb".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(30))
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let uow = PostgresUnitOfWork::new(Arc::new(pool)).await;
    let session = uow.begin().await?;

    Ok(TestContext { session })
}

/// Setup a shared unit of work for tests that need to share state
/// 
/// This function is useful for tests that need to set up data in one transaction
/// and then start a new transaction for the actual test. The returned UnitOfWork
/// can be used to begin multiple sessions.
#[allow(dead_code)]
pub async fn setup_shared_uow() -> Result<PostgresUnitOfWork, Box<dyn std::error::Error + Send + Sync>> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://user:password@localhost:5432/mydb".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(PostgresUnitOfWork::new(Arc::new(pool)).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use heapless::String as HeaplessString;
    use banking_db::models::person::{PersonModel, PersonType};
    use banking_db::repository::PersonRepository;

    #[tokio::test]
    async fn test_transaction_rollback() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // First, create a person in a transaction that will be rolled back
        let test_id = Uuid::new_v4();
        {
            let ctx = setup_test_context().await?;
            let person_repo = ctx.person_repos().persons();
            
            let person = PersonModel {
                id: test_id,
                person_type: PersonType::Natural,
                display_name: HeaplessString::try_from("Test Rollback").unwrap(),
                external_identifier: Some(HeaplessString::try_from("ROLLBACK_TEST").unwrap()),
                entity_reference_count: 0,
                organization_person_id: None,
                messaging_info1: None,
                messaging_info2: None,
                messaging_info3: None,
                messaging_info4: None,
                messaging_info5: None,
                department: None,
                location_id: None,
                duplicate_of_person_id: None,
            };
            
            let audit_log_id = Uuid::new_v4();
            person_repo.save(person, audit_log_id).await?;
            
            // Verify it exists within the transaction
            assert!(person_repo.exists_by_id(test_id).await?);
        } // Transaction is rolled back here when ctx is dropped
        
        // Now verify the person doesn't exist in a new transaction
        {
            let ctx = setup_test_context().await?;
            let person_repo = ctx.person_repos().persons();
            
            // Should not exist because the previous transaction was rolled back
            assert!(!person_repo.exists_by_id(test_id).await?);
        }
        
        Ok(())
    }
}
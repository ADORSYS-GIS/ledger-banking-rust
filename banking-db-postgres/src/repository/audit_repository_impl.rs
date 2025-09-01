use async_trait::async_trait;
use banking_db::{models::audit::AuditLogModel, repository::audit_repository::AuditLogRepository};
use sqlx::{Postgres, Row};
use uuid::Uuid;

// Import the new Executor enum
use crate::repository::executor::Executor;

pub struct AuditLogRepositoryImpl {
    // The struct now holds our generic Executor
    executor: Executor,
}

impl AuditLogRepositoryImpl {
    // The constructor now accepts the Executor
    pub fn new(executor: Executor) -> Self {
        Self { executor }
    }
}

#[async_trait]
impl AuditLogRepository<Postgres> for AuditLogRepositoryImpl {
    async fn create(&self, audit_log: &AuditLogModel) -> Result<AuditLogModel, sqlx::Error> {
        let query = sqlx::query(
            r#"
            INSERT INTO audit_log (id, updated_at, updated_by_person_id)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(audit_log.id)
        .bind(audit_log.updated_at)
        .bind(audit_log.updated_by_person_id);

        // Match on the executor type to run the query
        match &self.executor {
            Executor::Pool(pool) => {
                query.execute(&**pool).await?;
            }
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.execute(&mut **tx).await?;
            }
        };

        Ok(audit_log.clone())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<AuditLogModel>, sqlx::Error> {
        let query = sqlx::query(
            r#"
            SELECT * FROM audit_log WHERE id = $1
            "#,
        )
        .bind(id);

        let row = match &self.executor {
            Executor::Pool(pool) => query.fetch_optional(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_optional(&mut **tx).await?
            }
        };

        match row {
            Some(row) => Ok(Some(AuditLogModel {
                id: row.get("id"),
                updated_at: row.get("updated_at"),
                updated_by_person_id: row.get("updated_by_person_id"),
            })),
            None => Ok(None),
        }
    }
}
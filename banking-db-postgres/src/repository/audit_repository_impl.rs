use async_trait::async_trait;
use banking_db::{models::audit::AuditLogModel, repository::audit_repository::AuditLogRepository};
use sqlx::{PgPool, Postgres, Row};
use std::sync::Arc;
use uuid::Uuid;

pub struct AuditLogRepositoryImpl {
    pool: Arc<PgPool>,
}

impl AuditLogRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditLogRepository<Postgres> for AuditLogRepositoryImpl {
    async fn create(&self, audit_log: &AuditLogModel) -> Result<AuditLogModel, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO audit_log (id, updated_at, updated_by_person_id)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(audit_log.id)
        .bind(audit_log.updated_at)
        .bind(audit_log.updated_by_person_id)
        .execute(&*self.pool)
        .await?;

        Ok(audit_log.clone())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<AuditLogModel>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT * FROM audit_log WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await?;

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
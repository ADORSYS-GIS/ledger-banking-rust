use async_trait::async_trait;
use uuid::Uuid;
use crate::models::audit::AuditLogModel;

#[async_trait]
pub trait AuditLogRepository: Send + Sync {
    async fn create(&self, audit_log: &AuditLogModel) -> Result<AuditLogModel, sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AuditLogModel>, sqlx::Error>;
}
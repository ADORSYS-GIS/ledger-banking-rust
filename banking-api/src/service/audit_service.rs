use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::audit::AuditLog;

#[async_trait]
pub trait AuditService: Send + Sync {
    async fn create_audit_log(&self, updated_by_person_id: Uuid) -> Result<AuditLog, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_audit_log_by_id(&self, id: Uuid) -> Result<Option<AuditLog>, Box<dyn std::error::Error + Send + Sync>>;
}
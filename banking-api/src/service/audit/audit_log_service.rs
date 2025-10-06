use crate::domain::audit::AuditLog;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum AuditLogServiceError {
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

pub type AuditLogServiceResult<T> = Result<T, AuditLogServiceError>;

#[async_trait]
pub trait AuditLogService: Send + Sync {
    async fn create_audit_log(
        &self,
        updated_by_person_id: Uuid,
    ) -> AuditLogServiceResult<AuditLog>;
    async fn find_audit_log_by_id(&self, id: Uuid) -> AuditLogServiceResult<Option<AuditLog>>;
}
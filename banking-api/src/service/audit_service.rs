use crate::domain::audit::AuditLog;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum AuditServiceError {
    #[error("Invalid audit type: {0}")]
    InvalidAuditType(String),
    #[error("Duplicate audit record for entity {entity_id} version {version}")]
    DuplicateAuditRecord { entity_id: Uuid, version: i32 },
    #[error("Invalid version sequence: expected {expected}, got {actual}")]
    InvalidVersionSequence { expected: i32, actual: i32 },
    #[error("Hash mismatch for entity {entity_id} version {version} - potential tampering detected")]
    HashMismatch { entity_id: Uuid, version: i32 },
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

pub type AuditServiceResult<T> = Result<T, AuditServiceError>;

#[async_trait]
pub trait AuditService: Send + Sync {
    async fn create_audit_log(
        &self,
        updated_by_person_id: Uuid,
    ) -> AuditServiceResult<AuditLog>;
    async fn find_audit_log_by_id(&self, id: Uuid) -> AuditServiceResult<Option<AuditLog>>;
}
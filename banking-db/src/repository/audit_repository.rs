use crate::models::audit::AuditLogModel;
use async_trait::async_trait;
use sqlx::Database;
use std::error::Error;
use std::fmt;
use uuid::Uuid;

/// Domain-specific errors for Audit repository operations
#[derive(Debug)]
pub enum AuditDomainError {
    /// Invalid audit type
    InvalidAuditType(String),

    /// Audit record already exists
    DuplicateAuditRecord { entity_id: Uuid, version: i32 },

    /// Invalid version sequence
    InvalidVersionSequence { expected: i32, actual: i32 },

    /// Hash mismatch (potential tampering)
    HashMismatch { entity_id: Uuid, version: i32 },

    /// Generic repository error
    RepositoryError(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for AuditDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAuditType(audit_type) => {
                write!(f, "Invalid audit type: {}", audit_type)
            }
            Self::DuplicateAuditRecord { entity_id, version } => {
                write!(
                    f,
                    "Duplicate audit record for entity {} version {}",
                    entity_id, version
                )
            }
            Self::InvalidVersionSequence { expected, actual } => {
                write!(
                    f,
                    "Invalid version sequence: expected {}, got {}",
                    expected, actual
                )
            }
            Self::HashMismatch { entity_id, version } => {
                write!(
                    f,
                    "Hash mismatch for entity {} version {} - potential tampering detected",
                    entity_id, version
                )
            }
            Self::RepositoryError(err) => {
                write!(f, "Repository error: {}", err)
            }
        }
    }
}

impl Error for AuditDomainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RepositoryError(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for AuditDomainError {
    fn from(err: sqlx::Error) -> Self {
        Self::RepositoryError(Box::new(err))
    }
}

/// Result type using AuditDomainError
pub type AuditResult<T> = Result<T, AuditDomainError>;

#[async_trait]
pub trait AuditLogRepository<DB: Database>: Send + Sync {
    async fn create(&self, audit_log: &AuditLogModel) -> AuditResult<AuditLogModel>;
    async fn find_by_id(&self, id: Uuid) -> AuditResult<Option<AuditLogModel>>;
}
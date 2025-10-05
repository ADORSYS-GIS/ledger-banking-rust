use crate::models::audit::AuditLogModel;
use async_trait::async_trait;
use sqlx::Database;
use std::error::Error;
use std::fmt;
use uuid::Uuid;

#[derive(Debug)]
pub enum AuditLogRepositoryError {
    RepositoryError(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for AuditLogRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RepositoryError(err) => write!(f, "Repository error: {err}"),
        }
    }
}

impl Error for AuditLogRepositoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RepositoryError(err) => Some(err.as_ref()),
        }
    }
}

impl From<sqlx::Error> for AuditLogRepositoryError {
    fn from(err: sqlx::Error) -> Self {
        Self::RepositoryError(Box::new(err))
    }
}

pub type AuditLogResult<T> = Result<T, AuditLogRepositoryError>;

#[async_trait]
pub trait AuditLogRepository<DB: Database>: Send + Sync {
    async fn create(&self, audit_log: &AuditLogModel) -> AuditLogResult<AuditLogModel>;
    async fn find_by_id(&self, id: Uuid) -> AuditLogResult<Option<AuditLogModel>>;
}
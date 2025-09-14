use crate::domain::person::Messaging;
use crate::domain::AuditLog;
use async_trait::async_trait;
use heapless::String as HeaplessString;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum MessagingServiceError {
    #[error("Messaging not found: {0}")]
    NotFound(Uuid),
    #[error("Duplicate messaging entry: {0}")]
    DuplicateEntry(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("An unexpected error occurred")]
    UnexpectedError,
}

pub type MessagingServiceResult<T> = Result<T, MessagingServiceError>;

#[async_trait]
pub trait MessagingService: Send + Sync {
    async fn create_messaging(
        &self,
        messaging: Messaging,
        audit_log: AuditLog,
    ) -> MessagingServiceResult<Messaging>;
    async fn fix_messaging(&self, messaging: Messaging) -> MessagingServiceResult<Messaging>;
    async fn find_messaging_by_id(&self, id: Uuid) -> MessagingServiceResult<Option<Messaging>>;
    async fn find_messaging_by_value(
        &self,
        value: HeaplessString<100>,
    ) -> MessagingServiceResult<Option<Messaging>>;
}
use async_trait::async_trait;
use sqlx::Database;
use std::error::Error;
use std::fmt;
use uuid::Uuid;

use crate::models::person::{MessagingIdxModel, MessagingModel};

#[derive(Debug)]
pub enum MessagingRepositoryError {
    NotFound(Uuid),
    DuplicateEntry(String),
    DatabaseError(sqlx::Error),
}

impl fmt::Display for MessagingRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(id) => write!(f, "Messaging with id {id} not found"),
            Self::DuplicateEntry(value) => write!(f, "Duplicate messaging entry: {value}"),
            Self::DatabaseError(err) => write!(f, "Database error: {err}"),
        }
    }
}

impl Error for MessagingRepositoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::DatabaseError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for MessagingRepositoryError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::Database(db_err) => {
                if db_err.is_unique_violation() {
                    return Self::DuplicateEntry(db_err.constraint().unwrap_or("").to_string());
                }
            }
            sqlx::Error::RowNotFound => {
                return Self::DatabaseError(err);
            }
            _ => {}
        }
        Self::DatabaseError(err)
    }
}

pub type MessagingResult<T> = Result<T, MessagingRepositoryError>;

#[async_trait]
pub trait MessagingRepository<DB: Database>: Send + Sync {
    async fn save(
        &self,
        messaging: MessagingModel,
        audit_log_id: Uuid,
    ) -> MessagingResult<MessagingModel>;
    async fn load(&self, id: Uuid) -> MessagingResult<MessagingModel>;
    async fn find_by_id(&self, id: Uuid) -> MessagingResult<Option<MessagingIdxModel>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> MessagingResult<Vec<MessagingIdxModel>>;
    async fn exists_by_id(&self, id: Uuid) -> MessagingResult<bool>;
    async fn find_ids_by_value(&self, value: &str) -> MessagingResult<Vec<Uuid>>;
}
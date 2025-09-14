use crate::domain::person::Person;
use crate::domain::AuditLog;
use async_trait::async_trait;
use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub type PersonServiceResult<T> = Result<T, PersonServiceError>;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum PersonServiceError {
    #[error("Invalid organizational hierarchy: {0}")]
    InvalidHierarchy(String),
    #[error("Duplicate external identifier: {0}")]
    DuplicateExternalId(String),
    #[error("Cannot delete due to {0} dependent records")]
    CascadeDeleteBlocked(usize),
    #[error("Organization not found: {0}")]
    OrganizationNotFound(Uuid),
    #[error("Location not found: {0}")]
    LocationNotFound(Uuid),
    #[error("Referenced person for duplicate not found: {0}")]
    DuplicatePersonNotFound(Uuid),
    #[error("Invalid person type change from {from} to {to}")]
    InvalidPersonTypeChange { from: String, to: String },
    #[error("Messaging reference not found: {0}")]
    MessagingNotFound(Uuid),
    #[error("Batch validation failed for {failed_ids_count} records: {errors}")]
    BatchValidationFailed {
        failed_ids_count: usize,
        errors: String,
    },
    #[error("Duplicate person: {0}")]
    DuplicatePerson(String),
    #[error("Underlying repository error: {0}")]
    RepositoryError(String),
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

#[async_trait]
pub trait PersonService: Send + Sync {
    async fn create_person(
        &self,
        person: Person,
        audit_log: AuditLog,
    ) -> PersonServiceResult<Person>;
    async fn find_person_by_id(&self, id: Uuid) -> PersonServiceResult<Option<Person>>;
    async fn get_persons_by_external_identifier(
        &self,
        external_identifier: HeaplessString<50>,
    ) -> PersonServiceResult<Vec<Person>>;
}
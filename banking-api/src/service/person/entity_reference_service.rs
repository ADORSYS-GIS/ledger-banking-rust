use crate::domain::person::EntityReference;
use crate::domain::AuditLog;
use async_trait::async_trait;
use heapless::String as HeaplessString;
use std::error::Error;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum EntityReferenceServiceError {
    #[error("Person not found with id: {0}")]
    PersonNotFound(Uuid),
    #[error("Duplicate reference for person_id: {person_id}, reference_type: {reference_type}, reference_external_id: {reference_external_id}")]
    DuplicateReference {
        person_id: Uuid,
        reference_type: String,
        reference_external_id: String,
    },
    #[error("Repository error: {0}")]
    RepositoryError(Box<dyn Error + Send + Sync>),
}

pub type EntityReferenceServiceResult<T> = Result<T, EntityReferenceServiceError>;

#[async_trait]
pub trait EntityReferenceService: Send + Sync {
    async fn create_entity_reference(
        &self,
        entity_reference: EntityReference,
        audit_log: AuditLog,
    ) -> EntityReferenceServiceResult<EntityReference>;
    async fn fix_entity_reference(
        &self,
        entity_reference: EntityReference,
    ) -> EntityReferenceServiceResult<EntityReference>;
    async fn find_entity_reference_by_id(&self, id: Uuid) -> EntityReferenceServiceResult<Option<EntityReference>>;
    async fn find_entity_references_by_person_id(
        &self,
        person_id: Uuid,
    ) -> EntityReferenceServiceResult<Vec<EntityReference>>;
    async fn find_entity_references_by_reference_external_id(
        &self,
        reference_external_id: HeaplessString<50>,
    ) -> EntityReferenceServiceResult<Vec<EntityReference>>;
}
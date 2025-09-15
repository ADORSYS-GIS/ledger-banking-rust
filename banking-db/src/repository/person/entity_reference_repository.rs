use async_trait::async_trait;
use sqlx::Database;
use uuid::Uuid;
use crate::models::person::{EntityReferenceIdxModel, EntityReferenceModel};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum EntityReferenceRepositoryError {
    PersonNotFound(Uuid),
    DuplicateReference {
        person_id: Uuid,
        reference_type: String,
        reference_external_id: String,
    },
    RepositoryError(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for EntityReferenceRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PersonNotFound(id) => write!(f, "Person not found with id: {}", id),
            Self::DuplicateReference {
                person_id,
                reference_type,
                reference_external_id,
            } => write!(
                f,
                "Duplicate reference for person_id: {}, reference_type: {}, reference_external_id: {}",
                person_id, reference_type, reference_external_id
            ),
            Self::RepositoryError(e) => write!(f, "Repository error: {}", e),
        }
    }
}

impl Error for EntityReferenceRepositoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RepositoryError(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

pub type EntityReferenceResult<T> = Result<T, EntityReferenceRepositoryError>;

#[async_trait]
pub trait EntityReferenceRepository<DB: Database>: Send + Sync {
    async fn save(
        &self,
        entity_ref: EntityReferenceModel,
        audit_log_id: Uuid,
    ) -> EntityReferenceResult<EntityReferenceModel>;
    async fn load(&self, id: Uuid) -> EntityReferenceResult<EntityReferenceModel>;
    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> EntityReferenceResult<Option<EntityReferenceIdxModel>>;
    async fn find_by_person_id(
        &self,
        person_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> EntityReferenceResult<Vec<EntityReferenceIdxModel>>;
    async fn find_by_reference_external_id(
        &self,
        reference_external_id: &str,
        page: i32,
        page_size: i32,
    ) -> EntityReferenceResult<Vec<EntityReferenceIdxModel>>;
    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> EntityReferenceResult<Vec<EntityReferenceIdxModel>>;
    async fn exists_by_id(&self, id: Uuid) -> EntityReferenceResult<bool>;
    async fn find_ids_by_person_id(
        &self,
        person_id: Uuid,
    ) -> EntityReferenceResult<Vec<Uuid>>;
}
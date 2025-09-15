use crate::domain::person::Locality;
use async_trait::async_trait;
use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum LocalityServiceError {
    #[error("Country subdivision not found with id: {0}")]
    CountrySubdivisionNotFound(Uuid),
    #[error("Duplicate locality code '{code}' for country subdivision '{country_subdivision_id}'")]
    DuplicateCode {
        country_subdivision_id: Uuid,
        code: String,
    },
    #[error("Locality not found with id: {0}")]
    LocalityNotFound(Uuid),
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
}


pub type LocalityServiceResult<T> = Result<T, LocalityServiceError>;

#[async_trait]
pub trait LocalityService: Send + Sync {
    async fn create_locality(&self, locality: Locality) -> LocalityServiceResult<Locality>;
    async fn fix_locality(&self, locality: Locality) -> LocalityServiceResult<Locality>;
    async fn find_locality_by_id(&self, id: Uuid) -> LocalityServiceResult<Option<Locality>>;
    async fn find_localities_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
    ) -> LocalityServiceResult<Vec<Locality>>;
    async fn find_locality_by_code(
        &self,
        country_id: Uuid,
        code: HeaplessString<50>,
    ) -> LocalityServiceResult<Option<Locality>>;
}
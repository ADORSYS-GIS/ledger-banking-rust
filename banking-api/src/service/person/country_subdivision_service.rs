use crate::domain::person::CountrySubdivision;
use async_trait::async_trait;
use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum CountrySubdivisionServiceError {
    #[error("Country not found: {0}")]
    CountryNotFound(Uuid),
    #[error("Duplicate subdivision code '{code}' for country {country_id}")]
    DuplicateCode { country_id: Uuid, code: String },
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

#[async_trait]
pub trait CountrySubdivisionService: Send + Sync {
    async fn create_country_subdivision(
        &self,
        country_subdivision: CountrySubdivision,
    ) -> Result<CountrySubdivision, CountrySubdivisionServiceError>;
    async fn fix_country_subdivision(
        &self,
        country_subdivision: CountrySubdivision,
    ) -> Result<CountrySubdivision, CountrySubdivisionServiceError>;
    async fn find_country_subdivision_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<CountrySubdivision>, CountrySubdivisionServiceError>;
    async fn find_country_subdivisions_by_country_id(
        &self,
        country_id: Uuid,
    ) -> Result<Vec<CountrySubdivision>, CountrySubdivisionServiceError>;
    async fn find_country_subdivision_by_code(
        &self,
        country_id: Uuid,
        code: HeaplessString<10>,
    ) -> Result<Option<CountrySubdivision>, CountrySubdivisionServiceError>;
}
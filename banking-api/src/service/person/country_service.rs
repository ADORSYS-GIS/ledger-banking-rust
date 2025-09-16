use crate::domain::person::Country;
use async_trait::async_trait;
use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum CountryServiceError {
    #[error("Country not found: {0}")]
    CountryNotFound(Uuid),
    #[error("Duplicate country ISO2: {0}")]
    DuplicateCountryISO2(String),
    #[error("Invalid country ISO2: {0}")]
    InvalidCountryISO2(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

#[async_trait]
pub trait CountryService: Send + Sync {
    async fn create_country(&self, country: Country) -> Result<Country, CountryServiceError>;
    async fn fix_country(&self, country: Country) -> Result<Country, CountryServiceError>;
    async fn find_country_by_id(&self, id: Uuid) -> Result<Option<Country>, CountryServiceError>;
    async fn find_country_by_iso2(
        &self,
        iso2: HeaplessString<2>,
    ) -> Result<Option<Country>, CountryServiceError>;
    async fn get_all_countries(&self) -> Result<Vec<Country>, CountryServiceError>;
}
use async_trait::async_trait;
use sqlx::Database;
use std::error::Error;
use uuid::Uuid;

use crate::models::person::{CountryIdxModel, CountryModel};

#[derive(Debug)]
pub enum CountryRepositoryError {
    CountryNotFound(Uuid),
    DuplicateCountryISO2(String),
    InvalidCountryISO2(String),
    RepositoryError(Box<dyn Error + Send + Sync>),
}

impl std::fmt::Display for CountryRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CountryNotFound(id) => write!(f, "Country not found: {}", id),
            Self::DuplicateCountryISO2(iso2) => write!(f, "Duplicate country ISO2: {}", iso2),
            Self::InvalidCountryISO2(iso2) => write!(f, "Invalid country ISO2: {}", iso2),
            Self::RepositoryError(e) => write!(f, "Repository error: {}", e),
        }
    }
}

impl Error for CountryRepositoryError {}

pub type CountryResult<T> = Result<T, CountryRepositoryError>;

#[async_trait]
pub trait CountryRepository<DB: Database>: Send + Sync {
    async fn save(&self, country: CountryModel) -> CountryResult<CountryModel>;
    async fn load(&self, id: Uuid) -> CountryResult<CountryModel>;
    async fn find_by_id(&self, id: Uuid) -> CountryResult<Option<CountryIdxModel>>;
    async fn find_by_iso2(
        &self,
        iso2: &str,
        page: i32,
        page_size: i32,
    ) -> CountryResult<Vec<CountryIdxModel>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> CountryResult<Vec<CountryIdxModel>>;
    async fn exists_by_id(&self, id: Uuid) -> CountryResult<bool>;
    async fn find_ids_by_iso2(&self, iso2: &str) -> CountryResult<Vec<Uuid>>;
}
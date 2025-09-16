use async_trait::async_trait;
use sqlx::Database;
use std::error::Error;
use std::fmt;
use uuid::Uuid;

use crate::models::person::{CountrySubdivisionIdxModel, CountrySubdivisionModel};

#[derive(Debug)]
pub enum CountrySubdivisionRepositoryError {
    CountryNotFound(Uuid),
    DuplicateCode { country_id: Uuid, code: String },
    RepositoryError(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for CountrySubdivisionRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CountryNotFound(id) => write!(f, "Country not found: {id}"),
            Self::DuplicateCode { country_id, code } => {
                write!(
                    f,
                    "Duplicate subdivision code '{code}' for country {country_id}"
                )
            }
            Self::RepositoryError(err) => write!(f, "Repository error: {err}"),
        }
    }
}

impl Error for CountrySubdivisionRepositoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RepositoryError(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

pub type CountrySubdivisionResult<T> = Result<T, CountrySubdivisionRepositoryError>;

#[async_trait]
pub trait CountrySubdivisionRepository<DB: Database>: Send + Sync {
    async fn save(
        &self,
        country_subdivision: CountrySubdivisionModel,
    ) -> CountrySubdivisionResult<CountrySubdivisionModel>;
    async fn load(&self, id: Uuid) -> CountrySubdivisionResult<CountrySubdivisionModel>;
    async fn find_by_id(&self, id: Uuid) -> CountrySubdivisionResult<Option<CountrySubdivisionIdxModel>>;
    async fn find_by_country_id(
        &self,
        country_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> CountrySubdivisionResult<Vec<CountrySubdivisionIdxModel>>;
    async fn find_by_code(
        &self,
        country_id: Uuid,
        code: &str,
    ) -> CountrySubdivisionResult<Option<CountrySubdivisionIdxModel>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> CountrySubdivisionResult<Vec<CountrySubdivisionIdxModel>>;
    async fn exists_by_id(&self, id: Uuid) -> CountrySubdivisionResult<bool>;
    async fn find_ids_by_country_id(&self, country_id: Uuid) -> CountrySubdivisionResult<Vec<Uuid>>;
}
use async_trait::async_trait;
use sqlx::Database;
use std::error::Error;
use uuid::Uuid;

use crate::models::person::{LocalityIdxModel, LocalityModel};

#[derive(Debug)]
pub enum LocalityRepositoryError {
    CountrySubdivisionNotFound(Uuid),
    DuplicateCode {
        country_subdivision_id: Uuid,
        code: String,
    },
    LocalityNotFound(Uuid),
    RepositoryError(Box<dyn Error + Send + Sync>),
}

impl std::fmt::Display for LocalityRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocalityRepositoryError::CountrySubdivisionNotFound(id) => {
                write!(f, "Country subdivision not found with id: {}", id)
            }
            LocalityRepositoryError::DuplicateCode {
                country_subdivision_id,
                code,
            } => {
                write!(
                    f,
                    "Duplicate locality code '{}' for country subdivision '{}'",
                    code, country_subdivision_id
                )
            }
            LocalityRepositoryError::LocalityNotFound(id) => {
                write!(f, "Locality not found with id: {}", id)
            }
            LocalityRepositoryError::RepositoryError(err) => {
                write!(f, "Repository error: {}", err)
            }
        }
    }
}

impl Error for LocalityRepositoryError {}

pub type LocalityResult<T> = Result<T, LocalityRepositoryError>;

#[async_trait]
pub trait LocalityRepository<DB: Database>: Send + Sync {
    async fn save(&self, locality: LocalityModel) -> LocalityResult<LocalityModel>;
    async fn load(&self, id: Uuid) -> LocalityResult<LocalityModel>;
    async fn find_by_id(&self, id: Uuid) -> LocalityResult<Option<LocalityIdxModel>>;
    async fn find_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> LocalityResult<Vec<LocalityIdxModel>>;
    async fn find_by_code(
        &self,
        country_id: Uuid,
        code: &str,
    ) -> LocalityResult<Option<LocalityIdxModel>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> LocalityResult<Vec<LocalityIdxModel>>;
    async fn exists_by_id(&self, id: Uuid) -> LocalityResult<bool>;
    async fn find_ids_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
    ) -> LocalityResult<Vec<Uuid>>;
}
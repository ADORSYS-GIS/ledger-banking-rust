use async_trait::async_trait;
use sqlx::Database;
use uuid::Uuid;
use crate::models::person::{CountrySubdivisionIdxModel, CountrySubdivisionModel};

#[async_trait]
pub trait CountrySubdivisionRepository<DB: Database>: Send + Sync {
    async fn save(
        &self,
        country_subdivision: CountrySubdivisionModel,
    ) -> Result<CountrySubdivisionModel, sqlx::Error>;
    async fn load(&self, id: Uuid) -> Result<CountrySubdivisionModel, sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CountrySubdivisionIdxModel>, sqlx::Error>;
    async fn find_by_country_id(
        &self,
        country_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<CountrySubdivisionIdxModel>, sqlx::Error>;
    async fn find_by_code(
        &self,
        country_id: Uuid,
        code: &str,
    ) -> Result<Option<CountrySubdivisionIdxModel>, sqlx::Error>;
    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<CountrySubdivisionIdxModel>, Box<dyn std::error::Error + Send + Sync>>;
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_ids_by_country_id(
        &self,
        country_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn std::error::Error + Send + Sync>>;
}
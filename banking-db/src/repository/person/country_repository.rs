use async_trait::async_trait;
use sqlx::Database;
use uuid::Uuid;
use crate::models::person::{CountryIdxModel, CountryModel};

#[async_trait]
pub trait CountryRepository<DB: Database>: Send + Sync {
    async fn save(&self, country: CountryModel) -> Result<CountryModel, sqlx::Error>;
    async fn load(&self, id: Uuid) -> Result<CountryModel, sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CountryIdxModel>, sqlx::Error>;
    async fn find_by_iso2(
        &self,
        iso2: &str,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<CountryIdxModel>, sqlx::Error>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<CountryIdxModel>, sqlx::Error>;
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_ids_by_iso2(
        &self,
        iso2: &str,
    ) -> Result<Vec<Uuid>, Box<dyn std::error::Error + Send + Sync>>;
}
use async_trait::async_trait;
use sqlx::Database;
use uuid::Uuid;
use crate::models::person::{LocalityIdxModel, LocalityModel};

#[async_trait]
pub trait LocalityRepository<DB: Database>: Send + Sync {
    async fn save(&self, locality: LocalityModel) -> Result<LocalityModel, sqlx::Error>;
    async fn load(&self, id: Uuid) -> Result<LocalityModel, sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<LocalityIdxModel>, sqlx::Error>;
    async fn find_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<LocalityIdxModel>, sqlx::Error>;
    async fn find_by_code(
        &self,
        country_id: Uuid,
        code: &str,
    ) -> Result<Option<LocalityIdxModel>, sqlx::Error>;
    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<LocalityIdxModel>, Box<dyn std::error::Error + Send + Sync>>;
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_ids_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn std::error::Error + Send + Sync>>;
}
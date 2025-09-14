use async_trait::async_trait;
use sqlx::Database;
use uuid::Uuid;
use crate::models::person::{EntityReferenceIdxModel, EntityReferenceModel};

#[async_trait]
pub trait EntityReferenceRepository<DB: Database>: Send + Sync {
    async fn save(
        &self,
        entity_ref: EntityReferenceModel,
        audit_log_id: Uuid,
    ) -> Result<EntityReferenceModel, sqlx::Error>;
    async fn load(&self, id: Uuid) -> Result<EntityReferenceModel, sqlx::Error>;
    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<EntityReferenceIdxModel>, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_by_person_id(
        &self,
        person_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<EntityReferenceIdxModel>, sqlx::Error>;
    async fn find_by_reference_external_id(
        &self,
        reference_external_id: &str,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<EntityReferenceIdxModel>, sqlx::Error>;
    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<EntityReferenceIdxModel>, Box<dyn std::error::Error + Send + Sync>>;
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_ids_by_person_id(
        &self,
        person_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn std::error::Error + Send + Sync>>;
}
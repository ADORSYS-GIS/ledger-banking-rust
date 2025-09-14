use async_trait::async_trait;
use sqlx::Database;
use uuid::Uuid;
use crate::models::person::{MessagingIdxModel, MessagingModel};

#[async_trait]
pub trait MessagingRepository<DB: Database>: Send + Sync {
    async fn save(
        &self,
        messaging: MessagingModel,
        audit_log_id: Uuid,
    ) -> Result<MessagingModel, sqlx::Error>;
    async fn load(&self, id: Uuid) -> Result<MessagingModel, sqlx::Error>;
    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<MessagingIdxModel>, Box<dyn std::error::Error + Send + Sync>>;

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<MessagingIdxModel>, Box<dyn std::error::Error + Send + Sync>>;

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;

    async fn find_ids_by_value(
        &self,
        value: &str,
    ) -> Result<Vec<Uuid>, Box<dyn std::error::Error + Send + Sync>>;
}
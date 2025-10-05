use crate::repository::person::location_repository::LocationRepositoryImpl;
use async_trait::async_trait;
use banking_db::models::person::LocationModel;
use banking_db::repository::BatchRepository;
use sqlx::Postgres;
use std::error::Error;
use uuid::Uuid;

#[async_trait]
impl BatchRepository<Postgres, LocationModel> for LocationRepositoryImpl {
    async fn create_batch(
        &self,
        items: Vec<LocationModel>,
        audit_log_id: Uuid,
    ) -> Result<Vec<LocationModel>, Box<dyn Error + Send + Sync>> {
        crate::repository::person::location_repository::create_batch::create_batch(
            self,
            items,
            audit_log_id,
        )
        .await
    }

    async fn load_batch(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<Option<LocationModel>>, Box<dyn Error + Send + Sync>> {
        crate::repository::person::location_repository::load_batch::load_batch(self, ids).await
    }

    async fn update_batch(
        &self,
        items: Vec<LocationModel>,
        audit_log_id: Uuid,
    ) -> Result<Vec<LocationModel>, Box<dyn Error + Send + Sync>> {
        crate::repository::person::location_repository::update_batch::update_batch(
            self,
            items,
            audit_log_id,
        )
        .await
    }

    async fn delete_batch(&self, ids: &[Uuid]) -> Result<usize, Box<dyn Error + Send + Sync>> {
        crate::repository::person::location_repository::delete_batch::delete_batch(self, ids).await
    }
}
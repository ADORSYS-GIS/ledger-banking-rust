use crate::repository::person::entity_reference_repository::EntityReferenceRepositoryImpl;
use async_trait::async_trait;
use banking_db::models::person::EntityReferenceModel;
use banking_db::repository::BatchRepository;
use sqlx::Postgres;
use std::error::Error;
use uuid::Uuid;

#[async_trait]
impl BatchRepository<Postgres, EntityReferenceModel> for EntityReferenceRepositoryImpl {
    async fn create_batch(
        &self,
        items: Vec<EntityReferenceModel>,
        audit_log_id: Uuid,
    ) -> Result<Vec<EntityReferenceModel>, Box<dyn Error + Send + Sync>> {
        crate::repository::person::entity_reference_repository::create_batch::create_batch(
            self,
            items,
            audit_log_id,
        )
        .await
    }

    async fn load_batch(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<Option<EntityReferenceModel>>, Box<dyn Error + Send + Sync>> {
        crate::repository::person::entity_reference_repository::load_batch::load_batch(self, ids)
            .await
    }

    async fn update_batch(
        &self,
        items: Vec<EntityReferenceModel>,
        audit_log_id: Uuid,
    ) -> Result<Vec<EntityReferenceModel>, Box<dyn Error + Send + Sync>> {
        crate::repository::person::entity_reference_repository::update_batch::update_batch(
            self,
            items,
            audit_log_id,
        )
        .await
    }

    async fn delete_batch(&self, ids: &[Uuid]) -> Result<usize, Box<dyn Error + Send + Sync>> {
        crate::repository::person::entity_reference_repository::delete_batch::delete_batch(
            self, ids,
        )
        .await
    }
}
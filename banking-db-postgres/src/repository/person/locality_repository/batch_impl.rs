// FILE: banking-db-postgres/src/repository/person/locality_repository_batch_impl.rs

use crate::repository::person::locality_repository::repo_impl::LocalityRepositoryImpl;
use async_trait::async_trait;
use banking_db::models::person::LocalityModel;
use banking_db::repository::BatchRepository;
use sqlx::Postgres;
use std::error::Error;
use uuid::Uuid;

#[async_trait]
impl BatchRepository<Postgres, LocalityModel> for LocalityRepositoryImpl {
    async fn create_batch(
        &self,
        items: Vec<LocalityModel>,
        audit_log_id: Uuid,
    ) -> Result<Vec<LocalityModel>, Box<dyn Error + Send + Sync>> {
        crate::repository::person::locality_repository::create_batch::create_batch(
            self,
            items,
            audit_log_id,
        )
        .await
    }

    async fn load_batch(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<Option<LocalityModel>>, Box<dyn Error + Send + Sync>> {
        crate::repository::person::locality_repository::load_batch::load_batch(self, ids).await
    }

    async fn update_batch(
        &self,
        items: Vec<LocalityModel>,
        audit_log_id: Uuid,
    ) -> Result<Vec<LocalityModel>, Box<dyn Error + Send + Sync>> {
        crate::repository::person::locality_repository::update_batch::update_batch(
            self,
            items,
            audit_log_id,
        )
        .await
    }

    async fn delete_batch(&self, ids: &[Uuid]) -> Result<usize, Box<dyn Error + Send + Sync>> {
        crate::repository::person::locality_repository::delete_batch::delete_batch(self, ids).await
    }
}
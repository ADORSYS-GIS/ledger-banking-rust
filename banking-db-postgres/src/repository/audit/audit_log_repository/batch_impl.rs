use async_trait::async_trait;
use banking_db::{
    models::audit::AuditLogModel,
    repository::{
        batch_repository::BatchRepository,
    },
};
use sqlx::Postgres;
use std::error::Error;
use uuid::Uuid;

use crate::repository::{audit::audit_log_repository::repo_impl::AuditLogRepositoryImpl};

#[async_trait]
impl BatchRepository<Postgres, AuditLogModel> for AuditLogRepositoryImpl {
    async fn create_batch(
        &self,
        items: Vec<AuditLogModel>,
        _audit_log_id: Uuid,
    ) -> Result<Vec<AuditLogModel>, Box<dyn Error + Send + Sync>> {
        super::create_batch::create_batch(&self.executor, items).await
    }

    async fn load_batch(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<Option<AuditLogModel>>, Box<dyn Error + Send + Sync>> {
        super::load_batch::load_batch(&self.executor, ids).await
    }

    async fn update_batch(
        &self,
        _items: Vec<AuditLogModel>,
        _audit_log_id: Uuid,
    ) -> Result<Vec<AuditLogModel>, Box<dyn Error + Send + Sync>> {
        unimplemented!("Audit logs are immutable and cannot be updated in batch.")
    }

    async fn delete_batch(
        &self,
        ids: &[Uuid],
    ) -> Result<usize, Box<dyn Error + Send + Sync>> {
        super::delete_batch::delete_batch(&self.executor, ids).await
    }
}

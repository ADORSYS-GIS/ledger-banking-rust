// FILE: banking-db-postgres/src/repository/person/country_repository/batch_impl.rs

use crate::repository::person::country_repository::repo_impl::CountryRepositoryImpl;
use async_trait::async_trait;
use banking_db::models::person::CountryModel;
use banking_db::repository::BatchRepository;
use sqlx::Postgres;
use std::error::Error;
use uuid::Uuid;

use super::{create_batch, delete_batch, load_batch, update_batch};

#[async_trait]
impl BatchRepository<Postgres, CountryModel> for CountryRepositoryImpl {
    async fn create_batch(
        &self,
        items: Vec<CountryModel>,
        audit_log_id: Uuid,
    ) -> Result<Vec<CountryModel>, Box<dyn Error + Send + Sync>> {
        create_batch::create_batch(self, items, audit_log_id).await
    }

    async fn load_batch(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<Option<CountryModel>>, Box<dyn Error + Send + Sync>> {
        load_batch::load_batch(self, ids).await
    }

    async fn update_batch(
        &self,
        items: Vec<CountryModel>,
        audit_log_id: Uuid,
    ) -> Result<Vec<CountryModel>, Box<dyn Error + Send + Sync>> {
        update_batch::update_batch(self, items, audit_log_id).await
    }

    async fn delete_batch(&self, ids: &[Uuid]) -> Result<usize, Box<dyn Error + Send + Sync>> {
        delete_batch::delete_batch(self, ids).await
    }
}
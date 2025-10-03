use crate::repository::person::country_subdivision_repository::CountrySubdivisionRepositoryImpl;
use async_trait::async_trait;
use banking_db::models::person::CountrySubdivisionModel;
use banking_db::repository::BatchRepository;
use sqlx::Postgres;
use std::error::Error;
use uuid::Uuid;

#[async_trait]
impl BatchRepository<Postgres, CountrySubdivisionModel> for CountrySubdivisionRepositoryImpl {
    async fn create_batch(
        &self,
        items: Vec<CountrySubdivisionModel>,
        _audit_log_id: Uuid,
    ) -> Result<Vec<CountrySubdivisionModel>, Box<dyn Error + Send + Sync>> {
        crate::repository::person::country_subdivision_repository::create_batch::create_batch(self, items, _audit_log_id).await
    }

    async fn load_batch(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<Option<CountrySubdivisionModel>>, Box<dyn Error + Send + Sync>> {
        crate::repository::person::country_subdivision_repository::load_batch::load_batch(self, ids).await
    }

    async fn update_batch(
        &self,
        items: Vec<CountrySubdivisionModel>,
        _audit_log_id: Uuid,
    ) -> Result<Vec<CountrySubdivisionModel>, Box<dyn Error + Send + Sync>> {
        crate::repository::person::country_subdivision_repository::update_batch::update_batch(self, items, _audit_log_id).await
    }

    async fn delete_batch(&self, ids: &[Uuid]) -> Result<usize, Box<dyn Error + Send + Sync>> {
        crate::repository::person::country_subdivision_repository::delete_batch::delete_batch(self, ids).await
    }
}
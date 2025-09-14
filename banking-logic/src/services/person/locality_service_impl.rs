use async_trait::async_trait;
use banking_api::domain::person::Locality;
use banking_api::service::LocalityService;
use banking_api::BankingResult;
use heapless::String as HeaplessString;
use sqlx::Database;
use uuid::Uuid;

use crate::mappers::person_mapper::{ToDomain, ToModel};
use crate::services::repositories::Repositories;

pub struct LocalityServiceImpl<DB: Database> {
    repositories: Repositories<DB>,
}

impl<DB: Database> LocalityServiceImpl<DB> {
    pub fn new(repositories: Repositories<DB>) -> Self {
        Self { repositories }
    }
}

#[async_trait]
impl<DB: Database + Send + Sync> LocalityService for LocalityServiceImpl<DB> {
    async fn create_locality(&self, locality: Locality) -> BankingResult<Locality> {
        let model = locality.to_model();
        let saved_model = self.repositories.locality_repository.save(model).await?;
        Ok(saved_model.to_domain())
    }

    async fn fix_locality(&self, locality: Locality) -> BankingResult<Locality> {
        self.create_locality(locality).await
    }

    async fn find_locality_by_id(&self, id: Uuid) -> BankingResult<Option<Locality>> {
        let model_idx = self.repositories.locality_repository.find_by_id(id).await?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .locality_repository
                .load(idx.locality_id)
                .await?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn find_localities_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
    ) -> BankingResult<Vec<Locality>> {
        let model_ixes = self
            .repositories
            .locality_repository
            .find_by_country_subdivision_id(country_subdivision_id, 1, 1000)
            .await?;
        let mut localities = Vec::new();
        for idx in model_ixes {
            let locality_model = self
                .repositories
                .locality_repository
                .load(idx.locality_id)
                .await?;
            localities.push(locality_model.to_domain());
        }
        Ok(localities)
    }

    async fn find_locality_by_code(
        &self,
        country_id: Uuid,
        code: HeaplessString<50>,
    ) -> BankingResult<Option<Locality>> {
        let model_idx = self
            .repositories
            .locality_repository
            .find_by_code(country_id, code.as_str())
            .await?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .locality_repository
                .load(idx.locality_id)
                .await?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }
}
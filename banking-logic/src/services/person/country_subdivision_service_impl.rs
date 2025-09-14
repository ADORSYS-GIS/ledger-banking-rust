use async_trait::async_trait;
use banking_api::domain::person::CountrySubdivision;
use banking_api::service::CountrySubdivisionService;
use banking_api::BankingResult;
use heapless::String as HeaplessString;
use sqlx::Database;
use uuid::Uuid;

use crate::mappers::person_mapper::{ToDomain, ToModel};
use crate::services::repositories::Repositories;

pub struct CountrySubdivisionServiceImpl<DB: Database> {
    repositories: Repositories<DB>,
}

impl<DB: Database> CountrySubdivisionServiceImpl<DB> {
    pub fn new(repositories: Repositories<DB>) -> Self {
        Self { repositories }
    }
}

#[async_trait]
impl<DB: Database + Send + Sync> CountrySubdivisionService
    for CountrySubdivisionServiceImpl<DB>
{
    async fn create_country_subdivision(
        &self,
        country_subdivision: CountrySubdivision,
    ) -> BankingResult<CountrySubdivision> {
        let model = country_subdivision.to_model();
        let saved_model = self
            .repositories
            .country_subdivision_repository
            .save(model)
            .await?;
        Ok(saved_model.to_domain())
    }

    async fn fix_country_subdivision(
        &self,
        country_subdivision: CountrySubdivision,
    ) -> BankingResult<CountrySubdivision> {
        self.create_country_subdivision(country_subdivision).await
    }

    async fn find_country_subdivision_by_id(
        &self,
        id: Uuid,
    ) -> BankingResult<Option<CountrySubdivision>> {
        let model_idx = self
            .repositories
            .country_subdivision_repository
            .find_by_id(id)
            .await?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .country_subdivision_repository
                .load(idx.country_subdivision_id)
                .await?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn find_country_subdivisions_by_country_id(
        &self,
        country_id: Uuid,
    ) -> BankingResult<Vec<CountrySubdivision>> {
        let model_ixes = self
            .repositories
            .country_subdivision_repository
            .find_by_country_id(country_id, 1, 1000)
            .await?;
        let mut subdivisions = Vec::new();
        for idx in model_ixes {
            let subdivision_model = self
                .repositories
                .country_subdivision_repository
                .load(idx.country_subdivision_id)
                .await?;
            subdivisions.push(subdivision_model.to_domain());
        }
        Ok(subdivisions)
    }

    async fn find_country_subdivision_by_code(
        &self,
        country_id: Uuid,
        code: HeaplessString<10>,
    ) -> BankingResult<Option<CountrySubdivision>> {
        let model_idx = self
            .repositories
            .country_subdivision_repository
            .find_by_code(country_id, code.as_str())
            .await?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .country_subdivision_repository
                .load(idx.country_subdivision_id)
                .await?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }
}
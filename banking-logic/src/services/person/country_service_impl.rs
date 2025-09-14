use async_trait::async_trait;
use banking_api::domain::person::Country;
use banking_api::service::CountryService;
use banking_api::BankingResult;
use heapless::String as HeaplessString;
use sqlx::Database;
use uuid::Uuid;

use crate::mappers::person_mapper::{ToDomain, ToModel};
use crate::services::repositories::Repositories;

pub struct CountryServiceImpl<DB: Database> {
    repositories: Repositories<DB>,
}

impl<DB: Database> CountryServiceImpl<DB> {
    pub fn new(repositories: Repositories<DB>) -> Self {
        Self { repositories }
    }
}

#[async_trait]
impl<DB: Database + Send + Sync> CountryService for CountryServiceImpl<DB> {
    async fn create_country(&self, country: Country) -> BankingResult<Country> {
        let model = country.to_model();
        let saved_model = self.repositories.country_repository.save(model).await?;
        Ok(saved_model.to_domain())
    }

    async fn fix_country(&self, country: Country) -> BankingResult<Country> {
        self.create_country(country).await
    }

    async fn find_country_by_id(&self, id: Uuid) -> BankingResult<Option<Country>> {
        let model_idx = self.repositories.country_repository.find_by_id(id).await?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .country_repository
                .load(idx.country_id)
                .await?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn find_country_by_iso2(&self, iso2: HeaplessString<2>) -> BankingResult<Option<Country>> {
        let model_ixes = self
            .repositories
            .country_repository
            .find_by_iso2(iso2.as_str(), 1, 1)
            .await?;
        if let Some(idx) = model_ixes.into_iter().next() {
            let model = self
                .repositories
                .country_repository
                .load(idx.country_id)
                .await?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn get_all_countries(&self) -> BankingResult<Vec<Country>> {
        let model_ixes = self.repositories.country_repository.find_by_ids(&[]).await?;
        let mut countries = Vec::new();
        for idx in model_ixes {
            let country_model = self
                .repositories
                .country_repository
                .load(idx.country_id)
                .await?;
            countries.push(country_model.to_domain());
        }
        Ok(countries)
    }
}
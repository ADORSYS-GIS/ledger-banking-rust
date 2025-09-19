use async_trait::async_trait;
use banking_api::domain::person::Country;
use banking_api::service::country_service::{CountryService, CountryServiceError};
use banking_db::repository::person::country_repository::CountryRepositoryError;
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

fn map_domain_error_to_service_error(error: CountryRepositoryError) -> CountryServiceError {
    match error {
        CountryRepositoryError::CountryNotFound(id) => CountryServiceError::CountryNotFound(id),
        CountryRepositoryError::ManyCountriesNotFound(ids) => {
            CountryServiceError::RepositoryError(format!("Countries not found: {ids:?}"))
        }
        CountryRepositoryError::ManyCountriesExist(ids) => {
            CountryServiceError::RepositoryError(format!("Countries exist: {ids:?}"))
        }
        CountryRepositoryError::DuplicateCountryISO2(iso2) => {
            CountryServiceError::DuplicateCountryISO2(iso2)
        }
        CountryRepositoryError::InvalidCountryISO2(iso2) => {
            CountryServiceError::InvalidCountryISO2(iso2)
        }
        CountryRepositoryError::RepositoryError(e) => {
            CountryServiceError::RepositoryError(e.to_string())
        }
    }
}

#[async_trait]
impl<DB: Database + Send + Sync> CountryService for CountryServiceImpl<DB> {
    async fn create_country(&self, country: Country) -> Result<Country, CountryServiceError> {
        let model = country.to_model();
        let saved_model = self
            .repositories
            .country_repository
            .save(model)
            .await
            .map_err(map_domain_error_to_service_error)?;
        Ok(saved_model.to_domain())
    }

    async fn fix_country(&self, country: Country) -> Result<Country, CountryServiceError> {
        self.create_country(country).await
    }

    async fn find_country_by_id(&self, id: Uuid) -> Result<Option<Country>, CountryServiceError> {
        let model_idx = self
            .repositories
            .country_repository
            .find_by_id(id)
            .await
            .map_err(map_domain_error_to_service_error)?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .country_repository
                .load(idx.country_id)
                .await
                .map_err(map_domain_error_to_service_error)?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn find_country_by_iso2(
        &self,
        iso2: HeaplessString<2>,
    ) -> Result<Option<Country>, CountryServiceError> {
        let model_ixes = self
            .repositories
            .country_repository
            .find_by_iso2(iso2.as_str(), 1, 1)
            .await
            .map_err(map_domain_error_to_service_error)?;
        if let Some(idx) = model_ixes.into_iter().next() {
            let model = self
                .repositories
                .country_repository
                .load(idx.country_id)
                .await
                .map_err(map_domain_error_to_service_error)?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn get_all_countries(&self) -> Result<Vec<Country>, CountryServiceError> {
        let model_ixes = self
            .repositories
            .country_repository
            .find_by_ids(&[])
            .await
            .map_err(map_domain_error_to_service_error)?;
        let mut countries = Vec::new();
        for idx in model_ixes {
            let country_model = self
                .repositories
                .country_repository
                .load(idx.country_id)
                .await
                .map_err(map_domain_error_to_service_error)?;
            countries.push(country_model.to_domain());
        }
        Ok(countries)
    }
}
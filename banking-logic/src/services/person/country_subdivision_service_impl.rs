use async_trait::async_trait;
use banking_api::domain::person::CountrySubdivision;
use banking_api::service::person::country_subdivision_service::{
    CountrySubdivisionService, CountrySubdivisionServiceError,
};
use banking_db::repository::person::country_subdivision_repository::CountrySubdivisionRepositoryError;
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

fn map_domain_error_to_service_error(
    error: CountrySubdivisionRepositoryError,
) -> CountrySubdivisionServiceError {
    match error {
        CountrySubdivisionRepositoryError::CountryNotFound(id) => {
            CountrySubdivisionServiceError::CountryNotFound(id)
        }
        CountrySubdivisionRepositoryError::DuplicateCode { country_id, code } => {
            CountrySubdivisionServiceError::DuplicateCode { country_id, code }
        }
        CountrySubdivisionRepositoryError::RepositoryError(err) => {
            CountrySubdivisionServiceError::RepositoryError(err.to_string())
        }
    }
}

#[async_trait]
impl<DB: Database + Send + Sync> CountrySubdivisionService
    for CountrySubdivisionServiceImpl<DB>
{
    async fn create_country_subdivision(
        &self,
        country_subdivision: CountrySubdivision,
    ) -> Result<CountrySubdivision, CountrySubdivisionServiceError> {
        let model = country_subdivision.to_model();
        let saved_model = self
            .repositories
            .country_subdivision_repository
            .save(model)
            .await
            .map_err(map_domain_error_to_service_error)?;
        Ok(saved_model.to_domain())
    }

    async fn fix_country_subdivision(
        &self,
        country_subdivision: CountrySubdivision,
    ) -> Result<CountrySubdivision, CountrySubdivisionServiceError> {
        self.create_country_subdivision(country_subdivision).await
    }

    async fn find_country_subdivision_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<CountrySubdivision>, CountrySubdivisionServiceError> {
        let model_idx = self
            .repositories
            .country_subdivision_repository
            .find_by_id(id)
            .await
            .map_err(map_domain_error_to_service_error)?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .country_subdivision_repository
                .load(idx.country_subdivision_id)
                .await
                .map_err(map_domain_error_to_service_error)?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn find_country_subdivisions_by_country_id(
        &self,
        country_id: Uuid,
    ) -> Result<Vec<CountrySubdivision>, CountrySubdivisionServiceError> {
        let model_ixes = self
            .repositories
            .country_subdivision_repository
            .find_by_country_id(country_id, 1, 1000)
            .await
            .map_err(map_domain_error_to_service_error)?;
        let mut subdivisions = Vec::new();
        for idx in model_ixes {
            let subdivision_model = self
                .repositories
                .country_subdivision_repository
                .load(idx.country_subdivision_id)
                .await
                .map_err(map_domain_error_to_service_error)?;
            subdivisions.push(subdivision_model.to_domain());
        }
        Ok(subdivisions)
    }

    async fn find_country_subdivision_by_code(
        &self,
        country_id: Uuid,
        code: HeaplessString<10>,
    ) -> Result<Option<CountrySubdivision>, CountrySubdivisionServiceError> {
        let model_idx = self
            .repositories
            .country_subdivision_repository
            .find_by_code(country_id, code.as_str())
            .await
            .map_err(map_domain_error_to_service_error)?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .country_subdivision_repository
                .load(idx.country_subdivision_id)
                .await
                .map_err(map_domain_error_to_service_error)?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }
}
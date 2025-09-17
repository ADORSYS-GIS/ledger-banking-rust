use async_trait::async_trait;
use banking_api::domain::person::Locality;
use banking_api::service::{LocalityService, LocalityServiceError, LocalityServiceResult};
use banking_db::repository::person::locality_repository::LocalityRepositoryError;
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
    async fn create_locality(&self, locality: Locality) -> LocalityServiceResult<Locality> {
        let model = locality.to_model();
        let saved_model = self
            .repositories
            .locality_repository
            .save(model)
            .await
            .map_err(map_domain_error_to_service_error)?;
        Ok(saved_model.to_domain())
    }

    async fn fix_locality(&self, locality: Locality) -> LocalityServiceResult<Locality> {
        self.create_locality(locality).await
    }

    async fn find_locality_by_id(&self, id: Uuid) -> LocalityServiceResult<Option<Locality>> {
        let model_idx = self
            .repositories
            .locality_repository
            .find_by_id(id)
            .await
            .map_err(map_domain_error_to_service_error)?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .locality_repository
                .load(idx.locality_id)
                .await
                .map_err(map_domain_error_to_service_error)?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn find_localities_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
    ) -> LocalityServiceResult<Vec<Locality>> {
        let model_ixes = self
            .repositories
            .locality_repository
            .find_by_country_subdivision_id(country_subdivision_id, 1, 1000)
            .await
            .map_err(map_domain_error_to_service_error)?;
        let mut localities = Vec::new();
        for idx in model_ixes {
            let locality_model = self
                .repositories
                .locality_repository
                .load(idx.locality_id)
                .await
                .map_err(map_domain_error_to_service_error)?;
            localities.push(locality_model.to_domain());
        }
        Ok(localities)
    }

    async fn find_locality_by_code(
        &self,
        country_id: Uuid,
        code: HeaplessString<50>,
    ) -> LocalityServiceResult<Option<Locality>> {
        let model_idx = self
            .repositories
            .locality_repository
            .find_by_code(country_id, code.as_str())
            .await
            .map_err(map_domain_error_to_service_error)?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .locality_repository
                .load(idx.locality_id)
                .await
                .map_err(map_domain_error_to_service_error)?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }
}

fn map_domain_error_to_service_error(error: LocalityRepositoryError) -> LocalityServiceError {
    match error {
        LocalityRepositoryError::CountrySubdivisionNotFound(id) => {
            LocalityServiceError::CountrySubdivisionNotFound(id)
        }
        LocalityRepositoryError::DuplicateCode {
            country_subdivision_id,
            code,
        } => LocalityServiceError::DuplicateCode {
            country_subdivision_id,
            code,
        },
        LocalityRepositoryError::LocalityNotFound(id) => LocalityServiceError::LocalityNotFound(id),
        LocalityRepositoryError::DuplicateLocation(msg) => {
            LocalityServiceError::RepositoryError(msg)
        }
        LocalityRepositoryError::RepositoryError(err) => {
            LocalityServiceError::RepositoryError(err.to_string())
        }
    }
}
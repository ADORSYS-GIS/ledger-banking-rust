use async_trait::async_trait;
use banking_api::domain::person::Location;
use banking_api::service::{LocationService, LocationServiceError, LocationServiceResult};
use banking_db::repository::LocationDomainError;
use sqlx::Database;
use uuid::Uuid;

use crate::mappers::person_mapper::{ToDomain, ToModel};
use crate::services::repositories::Repositories;

pub struct LocationServiceImpl<DB: Database> {
    repositories: Repositories<DB>,
}

impl<DB: Database> LocationServiceImpl<DB> {
    pub fn new(repositories: Repositories<DB>) -> Self {
        Self { repositories }
    }
}

fn map_domain_error_to_service_error(error: LocationDomainError) -> LocationServiceError {
    match error {
        LocationDomainError::LocalityNotFound(id) => LocationServiceError::LocalityNotFound(id),
        LocationDomainError::InvalidLocationType(loc_type) => {
            LocationServiceError::InvalidLocationType(loc_type)
        }
        LocationDomainError::InvalidCoordinates {
            latitude,
            longitude,
        } => LocationServiceError::InvalidCoordinates {
            latitude,
            longitude,
        },
        LocationDomainError::DuplicateLocation {
            street,
            locality_id,
        } => LocationServiceError::DuplicateLocation {
            street,
            locality_id,
        },
        LocationDomainError::RepositoryError(err) => LocationServiceError::RepositoryError(err),
    }
}

#[async_trait]
impl<DB: Database + Send + Sync> LocationService for LocationServiceImpl<DB> {
    async fn create_location(
        &self,
        location: Location,
        audit_log: banking_api::domain::AuditLog,
    ) -> LocationServiceResult<Location> {
        let model = location.to_model();
        let saved_model = self
            .repositories
            .location_repository
            .save(model, audit_log.id)
            .await
            .map_err(map_domain_error_to_service_error)?;
        Ok(saved_model.to_domain())
    }

    async fn fix_location(&self, location: Location) -> LocationServiceResult<Location> {
        let audit_log = banking_api::domain::AuditLog {
            id: Uuid::new_v4(),
            updated_at: chrono::Utc::now(),
            updated_by_person_id: Uuid::new_v4(), // Placeholder
        };
        self.create_location(location, audit_log).await
    }

    async fn find_location_by_id(&self, id: Uuid) -> LocationServiceResult<Option<Location>> {
        let model_idx = self
            .repositories
            .location_repository
            .find_by_id(id)
            .await
            .map_err(map_domain_error_to_service_error)?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .location_repository
                .load(idx.location_id)
                .await
                .map_err(map_domain_error_to_service_error)?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn find_locations_by_locality_id(
        &self,
        locality_id: Uuid,
    ) -> LocationServiceResult<Vec<Location>> {
        let model_ixes = self
            .repositories
            .location_repository
            .find_by_locality_id(locality_id, 1, 1000)
            .await
            .map_err(map_domain_error_to_service_error)?;
        let mut locations = Vec::new();
        for idx in model_ixes {
            let location_model = self
                .repositories
                .location_repository
                .load(idx.location_id)
                .await
                .map_err(map_domain_error_to_service_error)?;
            locations.push(location_model.to_domain());
        }
        Ok(locations)
    }
}
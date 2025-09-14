use async_trait::async_trait;
use sqlx::Database;
use std::error::Error;
use std::fmt;
use uuid::Uuid;

use crate::models::person::{LocationIdxModel, LocationModel};

/// Domain-specific errors for Location repository operations
#[derive(Debug)]
pub enum LocationRepositoryError {
    /// Locality not found
    LocalityNotFound(Uuid),

    /// Invalid location type
    InvalidLocationType(String),

    /// Invalid coordinates
    InvalidCoordinates { latitude: f64, longitude: f64 },

    /// Duplicate location for same address
    DuplicateLocation {
        street: String,
        locality_id: Uuid,
    },

    /// Generic repository error
    RepositoryError(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for LocationRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LocalityNotFound(id) => {
                write!(f, "Locality not found: {}", id)
            }
            Self::InvalidLocationType(loc_type) => {
                write!(f, "Invalid location type: {}", loc_type)
            }
            Self::InvalidCoordinates {
                latitude,
                longitude,
            } => {
                write!(f, "Invalid coordinates: ({}, {})", latitude, longitude)
            }
            Self::DuplicateLocation {
                street,
                locality_id,
            } => {
                write!(
                    f,
                    "Duplicate location: {} in locality {}",
                    street, locality_id
                )
            }
            Self::RepositoryError(err) => {
                write!(f, "Repository error: {}", err)
            }
        }
    }
}

impl Error for LocationRepositoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RepositoryError(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

/// Result type using LocationRepositoryError
pub type LocationResult<T> = Result<T, LocationRepositoryError>;

#[async_trait]
pub trait LocationRepository<DB: Database>: Send + Sync {
    async fn save(
        &self,
        location: LocationModel,
        audit_log_id: Uuid,
    ) -> LocationResult<LocationModel>;
    async fn load(&self, id: Uuid) -> LocationResult<LocationModel>;
    async fn find_by_id(&self, id: Uuid) -> LocationResult<Option<LocationIdxModel>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> LocationResult<Vec<LocationIdxModel>>;
    async fn find_by_locality_id(
        &self,
        locality_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> LocationResult<Vec<LocationIdxModel>>;
    async fn exists_by_id(&self, id: Uuid) -> LocationResult<bool>;
    async fn find_ids_by_locality_id(&self, locality_id: Uuid) -> LocationResult<Vec<Uuid>>;
}
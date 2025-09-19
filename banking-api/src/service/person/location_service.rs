use crate::domain::person::Location;
use crate::domain::AuditLog;
use async_trait::async_trait;
use std::error::Error;
use std::fmt;
use uuid::Uuid;

/// Domain-specific errors for Location service operations
#[derive(Debug)]
pub enum LocationServiceError {
    /// Locality not found
    LocalityNotFound(Uuid),

    /// Invalid location type
    InvalidLocationType(String),

    /// Invalid coordinates
    InvalidCoordinates { latitude: f64, longitude: f64 },

    /// Many locations exist
    ManyLocationsExist(Vec<Uuid>),

    /// Many locations not found
    ManyLocationsNotFound(Vec<Uuid>),

    /// Location not found
    LocationNotFound(Uuid),

    /// Duplicate location for same address
    DuplicateLocation {
        street: String,
        locality_id: Uuid,
    },

    /// Generic repository error
    RepositoryError(Box<dyn Error + Send + Sync>),

    /// Generic service error
    ServiceError(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for LocationServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LocalityNotFound(id) => write!(f, "Locality not found: {id}"),
            Self::InvalidLocationType(loc_type) => {
                write!(f, "Invalid location type: {loc_type}")
            }
            Self::InvalidCoordinates {
                latitude,
                longitude,
            } => write!(f, "Invalid coordinates: ({latitude}, {longitude})"),
            Self::DuplicateLocation {
                street,
                locality_id,
            } => write!(
                f,
                "Duplicate location: {street} in locality {locality_id}"
            ),
            Self::RepositoryError(err) => write!(f, "Repository error: {err}"),
            Self::ServiceError(err) => write!(f, "Service error: {err}"),
            Self::ManyLocationsExist(ids) => {
                write!(f, "Locations with these IDs already exist: {ids:?}")
            }
            Self::ManyLocationsNotFound(ids) => {
                write!(f, "Locations with these IDs not found: {ids:?}")
            }
            Self::LocationNotFound(id) => {
                write!(f, "Location with this ID not found: {id}")
            }
        }
    }
}

impl Error for LocationServiceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RepositoryError(err) => Some(err.as_ref()),
            Self::ServiceError(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

/// Result type using LocationServiceError
pub type LocationServiceResult<T> = Result<T, LocationServiceError>;

#[async_trait]
pub trait LocationService: Send + Sync {
    async fn create_location(
        &self,
        location: Location,
        audit_log: AuditLog,
    ) -> LocationServiceResult<Location>;
    async fn fix_location(&self, location: Location) -> LocationServiceResult<Location>;
    async fn find_location_by_id(&self, id: Uuid) -> LocationServiceResult<Option<Location>>;
    async fn find_locations_by_locality_id(
        &self,
        locality_id: Uuid,
    ) -> LocationServiceResult<Vec<Location>>;
}
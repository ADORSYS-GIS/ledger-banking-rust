use heapless::{String as HeaplessString};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// # Service Trait
/// - FQN: banking-api/src/service/person/location_service.rs/LocationService
/// # Documentation
/// - Location structure for geographical locations
/// # Nature
/// - Immutable: 
///     - A location can be fixed if eroneous, but does not change base on the holder.  
///     - A customer changing address receives a new location.
///     - Many customers can share the same location object 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// # Trait method
    /// - find_location_by_id
    /// # Nature
    /// - primary index
    pub id: Uuid,
    /// # Trait method
    /// - find_locations_by_street_line1
    /// # Documentation
    /// - Structured location components - 4 street lines
    pub street_line1: HeaplessString<50>,
    pub street_line2: Option<HeaplessString<50>>,
    pub street_line3: Option<HeaplessString<50>>,
    pub street_line4: Option<HeaplessString<50>>,
    /// # Trait method
    /// - find_locations_by_locality_id
    pub locality_id: Uuid,
    pub postal_code: Option<HeaplessString<20>>,
    
    /// Geographical coordinates (decimal degrees)
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub accuracy_meters: Option<f32>,    
    
    /// # Trait method
    /// - find_location_by_type_and_locality
    /// # Documentation
    /// - Location type for categorization
    pub location_type: LocationType,
}

/// Type of location for categorization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocationType {
    /// Home/residential location
    Residential,
    /// Business/office location
    Business,
    /// Mailing location (P.O. Box, etc.)
    Mailing,
    /// Temporary location
    Temporary,
    /// Branch/agency location
    Branch,
    /// Community location
    Community,
    /// Other location types
    Other,
}
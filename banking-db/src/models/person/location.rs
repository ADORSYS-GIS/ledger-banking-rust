use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use std::collections::HashMap;

/// Database model for location type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "location_type", rename_all = "PascalCase")]
pub enum LocationType {
    Residential,
    Business,
    Mailing,
    Temporary,
    Branch,
    Community,
    Other,
}

/// # Repository Trait
/// - FQN: banking-db/src/repository/location_repository.rs/LocationRepository
/// 
/// # Index: LocationIdxModel
/// ## Repository Trait
/// - FQN: banking-db/src/repository/location_repository.rs/LocationRepository
/// ## Trait method
/// - create_idx
/// - load_idxes
/// ## Pg Trigger
/// - CREATE
/// - UPDATE
/// ## Cache: LocationIdxModelCache
/// - Mutable Set of Mutable Records
/// - Concurent
/// 
/// # Audit: LocationAuditModel
/// ## Repository Trait
/// - FQN: banking-db/src/repository/location_repository.rs/LocationRepository
/// ## Trait method
/// - create_audit
/// ## Additional field: `pub version: i32`
/// ### Nature
/// - composite-primary with self.id
/// ## Additional field: `pub hash: i64,`
/// ## Additional field: `pub audit_log_id: Uuid,`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LocationModel {
    /// # Trait method
    /// - find_by_id
    /// - find_by_ids
    /// - exists_by_id
    /// 
    /// # Index: location_id
    /// ## Nature
    /// - primary
    /// 
    /// # Audit: location_id
    /// ## Nature
    /// - composite-primary with self.version
    /// ## Trait method
    /// - find_audits_by_id
    pub id: Uuid,

    /// # Trait method
    /// - find_ids_by_street_line1
    /// # Documentation
    /// - Structured location components - 4 street lines
    pub street_line1: HeaplessString<50>,
    pub street_line2: Option<HeaplessString<50>>,
    pub street_line3: Option<HeaplessString<50>>,
    pub street_line4: Option<HeaplessString<50>>,

    /// # Trait method
    /// - find_ids_by_locality_id
    /// - find_by_locality_id
    /// 
    /// # Index:
    /// ## Nature
    /// - secondary
    /// ## Constraint
    /// - exists(LocalityModel.id)
    pub locality_id: Uuid,
    pub postal_code: Option<HeaplessString<20>>,

    /// Geographical coordinates (decimal degrees)
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub accuracy_meters: Option<f32>,
    
    /// # Trait method
    /// - find_ids_by_location_type
    /// - find_by_location_type
    /// - find_location_by_type_and_locality
    /// 
    /// # Documentation
    /// - Location type for categorization
    #[serde(serialize_with = "serialize_location_type", deserialize_with = "deserialize_location_type")]
    pub location_type: LocationType,
}

/// # Repository Trait
/// - FQN: banking-db/src/repository/location_repository.rs/LocationRepository
/// # Trait method
/// - create_audit
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LocationAuditModel {
    /// # Nature
    /// - composite-primary with self.version
    /// # Trait method
    /// - find_audits_by_id
    pub location_id: Uuid,

    /// # Nature
    /// - composite-primary with self.id
    pub version: i32,

    pub hash: i64,

    pub street_line1: HeaplessString<50>,
    pub street_line2: Option<HeaplessString<50>>,
    pub street_line3: Option<HeaplessString<50>>,
    pub street_line4: Option<HeaplessString<50>>,

    pub locality_id: Uuid,
    pub postal_code: Option<HeaplessString<20>>,

    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub accuracy_meters: Option<f32>,

    #[serde(serialize_with = "serialize_location_type", deserialize_with = "deserialize_location_type")]
    pub location_type: LocationType,

    pub audit_log_id: Uuid,
}

// Serialization functions for LocationType
pub fn serialize_location_type<S>(location_type: &LocationType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match location_type {
        LocationType::Residential => "residential",
        LocationType::Business => "business",
        LocationType::Mailing => "mailing",
        LocationType::Temporary => "temporary",
        LocationType::Branch => "branch",
        LocationType::Community => "community",
        LocationType::Other => "other",
    };
    serializer.serialize_str(type_str)
}

pub fn deserialize_location_type<'de, D>(deserializer: D) -> Result<LocationType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "residential" => Ok(LocationType::Residential),
        "business" => Ok(LocationType::Business),
        "mailing" => Ok(LocationType::Mailing),
        "temporary" => Ok(LocationType::Temporary),
        "branch" => Ok(LocationType::Branch),
        "community" => Ok(LocationType::Community),
        "other" => Ok(LocationType::Other),
        _ => Err(serde::de::Error::custom(format!("Unknown location type: {s}"))),
    }
}

/// # Repository Trait
/// - FQN: banking-db/src/repository/location_repository.rs/LocationRepository
/// # Trait method
/// - create_idx
/// - load_idxes
/// # Pg Trigger
/// - CREATE
/// - UPDATE
/// # Cache: LocationIdxModelCache
/// - Mutable Set of Mutable Records
/// - Concurent
#[derive(Debug, Clone, FromRow)]
pub struct LocationIdxModel {
    /// # Nature
    /// - primary
    pub location_id: Uuid,
    /// # Nature
    /// - secondary
    pub locality_id: Uuid,
    pub version: i32,
    pub hash: i64,
}

pub struct LocationIdxModelCache {
    by_id: HashMap<Uuid, LocationIdxModel>,
    by_locality_id: HashMap<Uuid, Vec<Uuid>>,
}

impl LocationIdxModelCache {
    pub fn new(items: Vec<LocationIdxModel>) -> Result<Self, &'static str> {
        let mut by_id = HashMap::new();
        let mut by_locality_id: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

        for item in items {
            let primary_key = item.location_id;
            if by_id.contains_key(&primary_key) {
                return Err("Duplicate primary key: location_id");
            }

            by_locality_id
                .entry(item.locality_id)
                .or_default()
                .push(primary_key);

            by_id.insert(primary_key, item);
        }

        Ok(LocationIdxModelCache {
            by_id,
            by_locality_id,
        })
    }

    pub fn add(&mut self, item: LocationIdxModel) {
        let primary_key = item.location_id;
        if self.by_id.contains_key(&primary_key) {
            self.update(item);
            return;
        }

        self.by_locality_id
            .entry(item.locality_id)
            .or_default()
            .push(primary_key);
        self.by_id.insert(primary_key, item);
    }

    pub fn remove(&mut self, location_id: &Uuid) -> Option<LocationIdxModel> {
        if let Some(item) = self.by_id.remove(location_id) {
            if let Some(ids) = self.by_locality_id.get_mut(&item.locality_id) {
                ids.retain(|&id| id != *location_id);
                if ids.is_empty() {
                    self.by_locality_id.remove(&item.locality_id);
                }
            }
            return Some(item);
        }
        None
    }

    pub fn update(&mut self, item: LocationIdxModel) {
        self.remove(&item.location_id);
        self.add(item);
    }

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        self.by_id.contains_key(primary_key)
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<LocationIdxModel> {
        self.by_id.get(primary_key).cloned()
    }

    pub fn get_by_locality_id(&self, key: &Uuid) -> Option<&Vec<Uuid>> {
        self.by_locality_id.get(key)
    }
}
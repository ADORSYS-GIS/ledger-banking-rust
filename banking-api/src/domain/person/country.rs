use heapless::{String as HeaplessString};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// # Service Trait
/// - FQN: banking-db/src/models/person/country.rs/CountryService
/// # Nature
/// - RuntimeImmutable: Creation, Modification requires reload of caches
/// # Documentation
/// - Country structure with ISO 3166-1 alpha-2 code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    /// # Trait method
    /// - find_country_by_id
    /// # Nature
    /// - primary index
    pub id: Uuid,
    /// # Documentation
    /// - ISO 3166-1 alpha-2 country code (e.g., "CM", "US", "GB")
    /// # Trait method
    /// - find_country_by_iso2
    /// # Nature
    /// - unique
    pub iso2: HeaplessString<2>,
    pub name_l1: HeaplessString<100>,
    pub name_l2: Option<HeaplessString<100>>,
    pub name_l3: Option<HeaplessString<100>>,
}
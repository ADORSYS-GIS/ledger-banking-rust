use heapless::{String as HeaplessString};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// # Service Trait
/// - FQN: banking-db/src/models/person/country_subdivision.rs/CountrySubdivisionService
/// # Nature
/// - RuntimeImmutable: Creation, Modification requires reload of caches
/// # Documentation
/// - Country structure with ISO 3166-1 alpha-2 code
/// - CountrySubdivision structure with multilingual support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountrySubdivision {
    /// # Trait method
    /// - find_country_subdivision_by_id
    /// # Nature
    /// - primary index
    pub id: Uuid,
    /// # Trait method
    /// - find_country_subdivision_by_country_id
    pub country_id: Uuid,
    /// # Documentation
    /// - If non existant the first 10 chars of the name_l1
    /// # Trait method
    /// - find_country_subdivision_by_code
    /// # Nature
    /// - unique
    pub code: HeaplessString<10>,
    pub name_l1: HeaplessString<100>,
    pub name_l2: Option<HeaplessString<100>>,
    pub name_l3: Option<HeaplessString<100>>,
}
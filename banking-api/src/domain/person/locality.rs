use heapless::{String as HeaplessString};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// # Service Trait
/// - FQN: banking-api/src/service/person/locality_service.rs/LocalityService
/// # Nature
/// - RuntimeImmutable: Creation, Modification requires reload of caches
/// # Documentation
/// - Locality structure with multilingual support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Locality {
    /// # Trait method
    /// - find_locality_by_id
    /// # Nature
    /// - primary index
    pub id: Uuid,
    /// # Trait method
    /// - find_localities_by_country_subdivision_id
    pub country_subdivision_id: Uuid,
    /// # Documentation
    /// - If empty, normalized value of the primary language name_l1
    /// # Trait method
    /// - find_locality_by_code
    /// # Nature
    /// - unique
    pub code: HeaplessString<50>,
    /// Locality name in primary language
    pub name_l1: HeaplessString<50>,
    /// Locality name in second language
    pub name_l2: Option<HeaplessString<50>>,
    /// Locality name in third language
    pub name_l3: Option<HeaplessString<50>>,
}
use heapless::{String as HeaplessString};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::person::common_enums::RelationshipRole;

/// # Service Trait
/// - FQN: banking-api/src/service/person/entity_reference_service.rs/EntityReferenceService
/// # Documentation
/// - Entity reference table for managing person-to-entity relationships
/// # Nature
/// - Mutable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityReference {
    /// # Trait method
    /// - find_entity_reference_by_id
    /// # Nature
    /// - primary index
    pub id: Uuid,

    /// # Documentation
    /// - References Person.person_id
    /// # Trait method
    /// - find_entity_references_by_person_id
    pub person_id: Uuid,
    
    /// # Documentation
    /// - Type of entity relationship
    /// # Trait method
    /// - find_entity_reference_by_person_and_role
    pub entity_role: RelationshipRole,
    
    /// # Documentation
    /// - External identifier for the reference (e.g., customer ID, employee ID)
    /// # Trait method
    /// - find_entity_reference_by_reference_external_id
    pub reference_external_id: HeaplessString<50>,
    
    pub reference_details_l1: Option<HeaplessString<50>>,
    pub reference_details_l2: Option<HeaplessString<50>>,
    pub reference_details_l3: Option<HeaplessString<50>>,

}
use heapless::{String as HeaplessString};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::person::common_enums::{MessagingType, PersonType};

/// # Service Trait
/// - FQN: banking-api/src/service/person/person_service.rs/PersonService
/// # Documentation
/// - Represents a person throughout the system for audit and tracking purposes
/// # Nature
/// - Mutable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    /// # Trait method
    /// - find_person_by_id
    /// # Nature
    /// - Primary index
    pub id: Uuid,
    
    /// # Documentation
    /// Type of person (natural, legal, etc.)
    pub person_type: PersonType,
    
    /// # Documentation
    /// Display name of the person
    pub display_name: HeaplessString<100>,
    
    /// # Trait method
    /// get_persons_by_external_identifier
    /// # Documentation
    /// External identifier (e.g., employee ID, badge number, system ID)
    pub external_identifier: Option<HeaplessString<50>>,

    pub entity_reference_count: i32,
    
    /// # Documentation
    /// References another Person.person_id for organizational hierarchy
    pub organization_person_id: Option<Uuid>,
    
    /// # Documentation
    /// References to Messaging.messaging_id (up to 5 messaging methods)
    pub messaging1_id: Option<Uuid>,
    pub messaging1_type: Option<MessagingType>,
    pub messaging2_id: Option<Uuid>,
    pub messaging2_type: Option<MessagingType>,
    pub messaging3_id: Option<Uuid>,
    pub messaging3_type: Option<MessagingType>,
    pub messaging4_id: Option<Uuid>,
    pub messaging4_type: Option<MessagingType>,
    pub messaging5_id: Option<Uuid>,
    pub messaging5_type: Option<MessagingType>,
    
    /// # Documentation
    /// Department within organization
    pub department: Option<HeaplessString<50>>,
    
    /// # Documentation
    /// References Location.location_id for person's location
    pub location_id: Option<Uuid>,
    
    /// # Documentation
    /// Reference to another Person if this is a duplicate
    pub duplicate_of_person_id: Option<Uuid>,
}
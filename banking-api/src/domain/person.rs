use heapless::{String as HeaplessString};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// # Service Trait
/// - FQN: banking-api/src/service/person_service.rs/PersonService
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

/// # Service Trait
/// - FQN: banking-api/src/service/person_service.rs/PersonService
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

/// # Service Trait
/// - FQN: banking-api/src/service/person_service.rs/PersonService
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

/// # Service Trait
/// - FQN: banking-api/src/service/person_service.rs/PersonService
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
    pub version: i32,
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
    pub audit_log_id: Uuid,
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

/// Type of messaging/communication method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessagingType {
    /// Email location
    Email,
    /// Phone number (mobile or landline)
    Phone,
    /// SMS/Text messaging
    Sms,
    /// WhatsApp messaging
    WhatsApp,
    /// Telegram messaging
    Telegram,
    /// Skype
    Skype,
    /// Microsoft Teams
    Teams,
    /// Signal messaging
    Signal,
    /// WeChat
    WeChat,
    /// Viber
    Viber,
    /// Facebook Messenger
    Messenger,
    /// LinkedIn messaging
    LinkedIn,
    /// Slack
    Slack,
    /// Discord
    Discord,
    /// Other messaging type (specify in other_type field)
    Other,
}

/// # Service Trait
/// - FQN: banking-api/src/service/person_service.rs/PersonService
/// # Documentation
/// - Messaging/communication identifier for a person
/// # Nature
/// - Immutable: 
///     - A messaging information can be fixed if eroneous, but does not change base on the holder. 
///     - A customer changing phone number receives an association with a new record.
///     - Many customers can share the same messaging object 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Messaging {
    /// # Trait method
    /// - find_messaging_by_id
    /// # Nature
    /// - primary index
    pub id: Uuid,
    pub version: i32,
    /// # Documentation
    /// - Type of messaging/communication method
    pub messaging_type: MessagingType,
    /// # Trait method
    /// - find_messaging_by_value
    /// # Documentation
    /// - The actual messaging identifier/location (email, phone, username, etc.)
    pub value: HeaplessString<100>,
    /// # Documentation
    /// - Description of the messaging type when MessagingType::Other is used
    pub other_type: Option<HeaplessString<20>>,
    pub audit_log_id: Uuid,
}

/// Type of person being referenced in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersonType {
    /// Natural person (human individual)
    Natural,
    /// Legal entity (corporation, institution)
    Legal,
    /// System or automated process
    System,
    /// External integration or API
    Integration,
    /// Unknown or unspecified
    Unknown,
}

/// Type of entity that the person is referenced as
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipRole {
    /// Customer reference
    Customer,
    /// Employee reference
    Employee,
    /// Shareholder reference
    Shareholder,
    /// Director or board member
    Director,
    /// Beneficial owner
    BeneficialOwner,
    /// Agent or representative
    Agent,
    /// Vendor or supplier
    Vendor,
    /// Partner organization
    Partner,
    /// Regulatory contact
    RegulatoryContact,
    /// Emergency contact
    EmergencyContact,
    /// System administrator
    SystemAdmin,
    /// Other entity type
    Other,
}

/// # Service Trait
/// - FQN: banking-api/src/service/person_service.rs/PersonService
/// # Documentation
/// - Entity reference table for managing person-to-entity relationships
/// # Nature
/// - Mutable 
///     - version field used to track changes.
///     - store a copy in table EntityReferenceAudit during modification. 
/// - AuditLog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityReference {
    /// # Trait method
    /// - find_entity_reference_by_id
    /// # Nature
    /// - primary index
    pub id: Uuid,

    /// # Documentation
    /// - version number, increased whenever a reference changes.
    /// - change triggers storage of old version in an audit database.
    pub version: i32,

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

    pub audit_log_id: Uuid,
}

/// # Service Trait
/// - FQN: banking-api/src/service/person_service.rs/PersonService
/// # Documentation
/// - Represents a person throughout the system for audit and tracking purposes
/// # Nature
/// - Mutable 
///     - version field used to track changes.
///     - store a copy in table EntityReferenceAudit during modification. 
/// - AuditLog
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

    pub audit_log_id: Uuid,    
}

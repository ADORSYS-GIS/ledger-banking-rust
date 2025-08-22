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
/// CountrySubdivision structure with multilingual support
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

impl Location {
    /// Create a new location
    pub fn new(
        id: Uuid,
        location_type: LocationType,
    ) -> Self {
        Self {
            id,
            street_line1: HeaplessString::new(),
            street_line2: None,
            street_line3: None,
            street_line4: None,
            locality_id: Uuid::new_v4(),
            postal_code: None,
            latitude: None,
            longitude: None,
            accuracy_meters: None,
            location_type,
        }
    }
    
    /// Builder for creating an Location with optional fields
    pub fn builder(
        id: Uuid,
        location_type: LocationType,
    ) -> LocationBuilder {
        LocationBuilder::new(id, location_type)
    }
    
    /// Set geographical coordinates
    pub fn set_coordinates(&mut self, latitude: Decimal, longitude: Decimal) {
        self.latitude = Some(latitude);
        self.longitude = Some(longitude);
    }
    
    /// Set geographical coordinates with accuracy
    pub fn set_coordinates_with_accuracy(&mut self, latitude: Decimal, longitude: Decimal, accuracy_meters: f32) {
        self.latitude = Some(latitude);
        self.longitude = Some(longitude);
        self.accuracy_meters = Some(accuracy_meters);
    }
    
    /// Check if location has valid coordinates
    pub fn has_coordinates(&self) -> bool {
        self.latitude.is_some() && self.longitude.is_some()
    }
}

/// Builder for Location
pub struct LocationBuilder {
    id: Uuid,
    street_line1: Option<String>,
    street_line2: Option<String>,
    street_line3: Option<String>,
    street_line4: Option<String>,
    locality_id: Uuid,
    postal_code: Option<String>,
    latitude: Option<Decimal>,
    longitude: Option<Decimal>,
    accuracy_meters: Option<f32>,
    location_type: LocationType,
}

impl LocationBuilder {
    pub fn new(
        id: Uuid,
        location_type: LocationType,
    ) -> Self {
        Self {
            id,
            street_line1: None,
            street_line2: None,
            street_line3: None,
            street_line4: None,
            locality_id: Uuid::new_v4(),
            postal_code: None,
            latitude: None,
            longitude: None,
            accuracy_meters: None,
            location_type,
        }
    }
    
    pub fn street_line1(mut self, line: impl AsRef<str>) -> Self {
        self.street_line1 = Some(line.as_ref().to_string());
        self
    }
    
    pub fn street_line2(mut self, line: impl AsRef<str>) -> Self {
        self.street_line2 = Some(line.as_ref().to_string());
        self
    }
    
    pub fn street_line3(mut self, line: impl AsRef<str>) -> Self {
        self.street_line3 = Some(line.as_ref().to_string());
        self
    }
    
    pub fn street_line4(mut self, line: impl AsRef<str>) -> Self {
        self.street_line4 = Some(line.as_ref().to_string());
        self
    }
    
    pub fn locality_id(mut self, locality_id: Uuid) -> Self {
        self.locality_id = locality_id;
        self
    }
    
    pub fn postal_code(mut self, code: impl AsRef<str>) -> Self {
        self.postal_code = Some(code.as_ref().to_string());
        self
    }
    
    pub fn coordinates(mut self, latitude: Decimal, longitude: Decimal) -> Self {
        self.latitude = Some(latitude);
        self.longitude = Some(longitude);
        self
    }
    
    pub fn coordinates_with_accuracy(mut self, latitude: Decimal, longitude: Decimal, accuracy_meters: f32) -> Self {
        self.latitude = Some(latitude);
        self.longitude = Some(longitude);
        self.accuracy_meters = Some(accuracy_meters);
        self
    }
        
    pub fn build(self) -> Result<Location, &'static str> {
        let street_line1 = self.street_line1
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "Street line 1 exceeds maximum length")?
            .unwrap_or_default();
            
        let street_line2 = self.street_line2
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "Street line 2 exceeds maximum length")?;
            
        let street_line3 = self.street_line3
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "Street line 3 exceeds maximum length")?;
            
        let street_line4 = self.street_line4
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "Street line 4 exceeds maximum length")?;
            
        let postal_code = self.postal_code
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "Postal code exceeds maximum length")?;
            
        Ok(Location {
            id: self.id,
            street_line1,
            street_line2,
            street_line3,
            street_line4,
            locality_id: self.locality_id,
            postal_code,
            latitude: self.latitude,
            longitude: self.longitude,
            accuracy_meters: self.accuracy_meters,
            location_type: self.location_type,
        })
    }
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
}

impl Messaging {
    /// Create a new messaging entry
    pub fn new(
        messaging_type: MessagingType,
        value: impl AsRef<str>,
    ) -> Result<Self, &'static str> {
        let value = HeaplessString::try_from(value.as_ref())
            .map_err(|_| "Messaging value exceeds maximum length")?;
            
        Ok(Self {
            id: Uuid::new_v4(),
            messaging_type,
            value,
            other_type: None,
        })
    }
    
    /// Create a new messaging entry with custom type description
    pub fn new_with_other_type(
        value: impl AsRef<str>,
        other_type: impl AsRef<str>,
    ) -> Result<Self, &'static str> {
        let value = HeaplessString::try_from(value.as_ref())
            .map_err(|_| "Messaging value exceeds maximum length")?;
        let other_type = HeaplessString::try_from(other_type.as_ref())
            .map_err(|_| "Other type description exceeds maximum length")?;
            
        Ok(Self {
            id: Uuid::new_v4(),
            messaging_type: MessagingType::Other,
            value,
            other_type: Some(other_type),
        })
    }
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
    pub version: u16,

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

impl EntityReference {
    /// Create a new entity reference
    pub fn new(
        person_id: Uuid,
        entity_role: RelationshipRole,
        reference_external_id: HeaplessString<50>,
        audit_log_id: Uuid,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            version: 0,
            person_id,
            entity_role,
            reference_external_id,
            reference_details_l1: None,
            reference_details_l2: None,
            reference_details_l3: None,
            audit_log_id,
        }
    }

    /// Update reference details with multi-language support
    pub fn set_reference_details(
        &mut self,
        details_l1: Option<HeaplessString<50>>,
        details_l2: Option<HeaplessString<50>>,
        details_l3: Option<HeaplessString<50>>,
        audit_log_id: Uuid,
    ) {
        self.reference_details_l1 = details_l1;
        self.reference_details_l2 = details_l2;
        self.reference_details_l3 = details_l3;
        
        self.audit_log_id = audit_log_id;
    }
}

/// # Service Trait
/// - FQN: banking-api/src/service/person_service.rs/PersonService
/// # Documentation
/// - Entity reference audit table for storing changes on entity references.
/// # Nature
/// - Immutable 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityReferenceAudit {
    /// # Trait method
    /// - find_entity_reference_by_id
    /// # Nature
    /// - Coumpound primary index with self.version
    pub id: Uuid,

    /// # Nature
    /// - Coumpound primary index with version
    pub version: u16,

    /// # Documentation
    /// - References Person.person_id
    /// # Trait method
    /// - find_entity_references_audit_by_person_id
    pub person_id: Uuid,
    
    /// # Documentation
    /// - Type of entity relationship
    /// # Trait method
    /// - find_entity_reference_audit_by_person_and_role
    pub entity_role: RelationshipRole,
    
    /// # Documentation
    /// - External identifier for the reference (e.g., customer ID, employee ID)
    pub reference_external_id: HeaplessString<50>,
    
    /// Reference details in language 1
    pub reference_details_l1: Option<HeaplessString<50>>,
    /// Reference details in language 2
    pub reference_details_l2: Option<HeaplessString<50>>,
    /// Reference details in language 3
    pub reference_details_l3: Option<HeaplessString<50>>,

    pub audit_log_id: Uuid
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
    
    pub version: u16,

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

    pub entity_reference_count: u8,
    
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

impl Person {
    /// Creates a new Person
    pub fn new(
        id: Uuid,
        version: u16,
        person_type: PersonType,
        display_name: impl AsRef<str>,
        audit_log_id: Uuid,
    ) -> Result<Self, &'static str> {
        let display_name = HeaplessString::try_from(display_name.as_ref())
            .map_err(|_| "Display name exceeds maximum length")?;
            
        Ok(Self {
            id,
            version,
            person_type,
            display_name,
            external_identifier: None,
            organization_person_id: None,
            entity_reference_count: 0,
            messaging1_id: None,
            messaging1_type: None,
            messaging2_id: None,
            messaging2_type: None,
            messaging3_id: None,
            messaging3_type: None,
            messaging4_id: None,
            messaging4_type: None,
            messaging5_id: None,
            messaging5_type: None,
            department: None,
            location_id: None,
            duplicate_of_person_id: None,
            audit_log_id,
        })
    }
    
    /// Builder for creating a Person with optional fields
    pub fn builder(id: Uuid, version:u16, person_type: PersonType, display_name: impl AsRef<str>, audit_log_id: Uuid) -> PersonBuilder {
        PersonBuilder::new(id, version, person_type, display_name, audit_log_id)
    }
    
    /// Add a messaging method reference to the person
    pub fn add_messaging_reference(&mut self, messaging_id: Uuid, messaging_type: Option<MessagingType>) -> Result<(), &'static str> {
        if self.messaging1_id.is_none() {
            self.messaging1_id = Some(messaging_id);
            self.messaging1_type = messaging_type;
        } else if self.messaging2_id.is_none() {
            self.messaging2_id = Some(messaging_id);
            self.messaging2_type = messaging_type;
        } else if self.messaging3_id.is_none() {
            self.messaging3_id = Some(messaging_id);
            self.messaging3_type = messaging_type;
        } else if self.messaging4_id.is_none() {
            self.messaging4_id = Some(messaging_id);
            self.messaging4_type = messaging_type;
        } else if self.messaging5_id.is_none() {
            self.messaging5_id = Some(messaging_id);
            self.messaging5_type = messaging_type;
        } else {
            return Err("Maximum 5 messaging entries allowed");
        }
        Ok(())
    }
    
    /// Remove a messaging method reference by ID
    pub fn remove_messaging_reference(&mut self, messaging_id: Uuid) -> bool {
        if self.messaging1_id == Some(messaging_id) {
            self.messaging1_id = None;
            self.messaging1_type = None;
        } else if self.messaging2_id == Some(messaging_id) {
            self.messaging2_id = None;
            self.messaging2_type = None;
        } else if self.messaging3_id == Some(messaging_id) {
            self.messaging3_id = None;
            self.messaging3_type = None;
        } else if self.messaging4_id == Some(messaging_id) {
            self.messaging4_id = None;
            self.messaging4_type = None;
        } else if self.messaging5_id == Some(messaging_id) {
            self.messaging5_id = None;
            self.messaging5_type = None;
        } else {
            return false;
        }
        true
    }
    
    /// Check if person has a specific messaging method reference
    pub fn has_messaging_reference(&self, messaging_id: Uuid) -> bool {
        self.messaging1_id == Some(messaging_id) ||
        self.messaging2_id == Some(messaging_id) ||
        self.messaging3_id == Some(messaging_id) ||
        self.messaging4_id == Some(messaging_id) ||
        self.messaging5_id == Some(messaging_id)
    }
    
    /// Get all messaging reference IDs
    pub fn get_messaging_references(&self) -> Vec<Uuid> {
        let mut refs = Vec::new();
        if let Some(id) = self.messaging1_id { refs.push(id); }
        if let Some(id) = self.messaging2_id { refs.push(id); }
        if let Some(id) = self.messaging3_id { refs.push(id); }
        if let Some(id) = self.messaging4_id { refs.push(id); }
        if let Some(id) = self.messaging5_id { refs.push(id); }
        refs
    }
    
    /// Get count of messaging references
    pub fn messaging_count(&self) -> usize {
        let mut count = 0;
        if self.messaging1_id.is_some() { count += 1; }
        if self.messaging2_id.is_some() { count += 1; }
        if self.messaging3_id.is_some() { count += 1; }
        if self.messaging4_id.is_some() { count += 1; }
        if self.messaging5_id.is_some() { count += 1; }
        count
    }
}

/// Builder for Person
pub struct PersonBuilder {
    id: Uuid,
    version: u16,
    person_type: PersonType,
    display_name: String,
    external_identifier: Option<String>,
    organization_person_id: Option<Uuid>,
    entity_reference_count: u8,
    messaging: Vec<Uuid>,
    department: Option<String>,
    location_id: Option<Uuid>,
    duplicate_of_person_id: Option<Uuid>,
    audit_log_id: Uuid,
}

impl PersonBuilder {
    pub fn new(id: Uuid, version: u16, person_type: PersonType, display_name: impl AsRef<str>, audit_log_id: Uuid) -> Self {
        Self {
            id,
            version,
            person_type,
            display_name: display_name.as_ref().to_string(),
            external_identifier: None,
            organization_person_id: None,
            entity_reference_count: 0,
            messaging: Vec::new(),
            department: None,
            location_id: None,
            duplicate_of_person_id: None,
            audit_log_id,
        }
    }
    
    pub fn external_identifier(mut self, identifier: impl AsRef<str>) -> Self {
        self.external_identifier = Some(identifier.as_ref().to_string());
        self
    }
    
    pub fn organization_person_id(mut self, organization_id: Uuid) -> Self {
        self.organization_person_id = Some(organization_id);
        self
    }
    
    pub fn add_messaging_reference(mut self, messaging_id: Uuid) -> Self {
        self.messaging.push(messaging_id);
        self
    }
    
    pub fn department(mut self, department: impl AsRef<str>) -> Self {
        self.department = Some(department.as_ref().to_string());
        self
    }
    
    pub fn location_id(mut self, location_id: Uuid) -> Self {
        self.location_id = Some(location_id);
        self
    }
    
    pub fn duplicate_of_person_id(mut self, person_id: Uuid) -> Self {
        self.duplicate_of_person_id = Some(person_id);
        self
    }
    
    pub fn build(self) -> Result<Person, &'static str> {
        let display_name = HeaplessString::try_from(self.display_name.as_str())
            .map_err(|_| "Display name exceeds maximum length")?;
            
        let external_identifier = self.external_identifier
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "External identifier exceeds maximum length")?;
            
        let department = self.department
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "Department exceeds maximum length")?;
            
        // Handle messaging entries (limit to 5)
        let messaging_ids: Vec<_> = self.messaging.into_iter().take(5).collect();
        let (msg1_id, msg2_id, msg3_id, msg4_id, msg5_id) = (
            messaging_ids.first().copied(),
            messaging_ids.get(1).copied(),
            messaging_ids.get(2).copied(),
            messaging_ids.get(3).copied(),
            messaging_ids.get(4).copied(),
        );
            
        Ok(Person {
            id: self.id,
            version: self.version,
            person_type: self.person_type,
            display_name,
            external_identifier,
            organization_person_id: self.organization_person_id,
            entity_reference_count: self.entity_reference_count,
            messaging1_id: msg1_id,
            messaging1_type: None,  // Type will be set separately if needed
            messaging2_id: msg2_id,
            messaging2_type: None,
            messaging3_id: msg3_id,
            messaging3_type: None,
            messaging4_id: msg4_id,
            messaging4_type: None,
            messaging5_id: msg5_id,
            messaging5_type: None,
            department,
            location_id: self.location_id,
            duplicate_of_person_id: self.duplicate_of_person_id,
            audit_log_id: self.audit_log_id,
        })
    }
}

/// # Service Trait
/// - FQN: banking-api/src/service/person_service.rs/PersonService
/// # Documentation
/// - Audit tracking for person.
/// # Nature
/// - Immutable 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonAudit {
    /// # Trait method
    /// - find_person_audit_by_id
    /// # Nature
    /// - Compound index with version
    pub id: Uuid,
    
    pub version: u16,

    /// # Documentation
    /// Type of person (natural, legal, etc.)
    pub person_type: PersonType,
    
    /// # Documentation
    /// Display name of the person
    pub display_name: HeaplessString<100>,
    
    /// # Trait method
    /// get_persons_audit_by_external_identifier
    /// # Documentation
    /// External identifier (e.g., employee ID, badge number, system ID)
    pub external_identifier: Option<HeaplessString<50>>,

    pub entity_reference_count: u8,
    
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

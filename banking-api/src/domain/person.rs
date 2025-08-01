use chrono::{DateTime, Utc};
use heapless::{String as HeaplessString};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Country structure with ISO 3166-1 alpha-2 code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    pub country_id: Uuid,
    /// ISO 3166-1 alpha-2 country code (e.g., "CM", "US", "GB")
    pub iso2: HeaplessString<2>,
    /// Country name in primary language
    pub name_l1: HeaplessString<100>,
    /// Country name in second language
    pub name_l2: Option<HeaplessString<100>>,
    /// Country name in third language
    pub name_l3: Option<HeaplessString<100>>,
    /// Whether this country is currently active
    pub is_active: bool,
    /// Audit fields
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// References Person.person_id
    pub created_by: Uuid,
    /// References Person.person_id
    pub updated_by: Uuid,
}

/// State/Province structure with multilingual support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateProvince {
    pub state_id: Uuid,
    /// References Country.country_id
    pub country_id: Uuid,
    /// State/province name in primary language
    pub name_l1: HeaplessString<100>,
    /// State/province name in second language
    pub name_l2: Option<HeaplessString<100>>,
    /// State/province name in third language
    pub name_l3: Option<HeaplessString<100>>,
    /// Whether this state/province is currently active
    pub is_active: bool,
    /// Audit fields
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// References Person.person_id
    pub created_by: Uuid,
    /// References Person.person_id
    pub updated_by: Uuid,
}

/// City structure with multilingual support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct City {
    pub city_id: Uuid,
    /// References Country.country_id
    pub country_id: Uuid,
    /// References StateProvince.state_id (optional for countries without states/provinces)
    pub state_id: Option<Uuid>,
    /// City name in primary language
    pub name_l1: HeaplessString<100>,
    /// City name in second language
    pub name_l2: Option<HeaplessString<100>>,
    /// City name in third language
    pub name_l3: Option<HeaplessString<100>>,
    /// Whether this city is currently active
    pub is_active: bool,
    /// Audit fields
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// References Person.person_id
    pub created_by: Uuid,
    /// References Person.person_id
    pub updated_by: Uuid,
}

/// Address structure for geographical locations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub address_id: Uuid,
    /// Structured address components - 4 street lines
    pub street_line1: HeaplessString<50>,
    pub street_line2: HeaplessString<50>,
    pub street_line3: HeaplessString<50>,
    pub street_line4: HeaplessString<50>,
    /// References City.city_id (city contains country and state references)
    pub city_id: Option<Uuid>,
    pub postal_code: Option<HeaplessString<20>>,
    
    /// Geographical coordinates (decimal degrees)
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub accuracy_meters: Option<f32>,
    
    
    /// Address type for categorization
    pub address_type: AddressType,
    
    /// Whether this address is currently active/valid
    pub is_active: bool,
    
    /// Audit fields
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// References Person.person_id
    pub created_by: Uuid,
    /// References Person.person_id
    pub updated_by: Uuid,
}

impl Address {
    /// Create a new address
    pub fn new(
        address_id: Uuid,
        address_type: AddressType,
        created_by: Uuid,
    ) -> Self {
        Self {
            address_id,
            street_line1: HeaplessString::new(),
            street_line2: HeaplessString::new(),
            street_line3: HeaplessString::new(),
            street_line4: HeaplessString::new(),
            city_id: None,
            postal_code: None,
            latitude: None,
            longitude: None,
            accuracy_meters: None,
            address_type,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by,
            updated_by: created_by,
        }
    }
    
    /// Builder for creating an Address with optional fields
    pub fn builder(
        address_id: Uuid,
        address_type: AddressType,
        created_by: Uuid,
    ) -> AddressBuilder {
        AddressBuilder::new(address_id, address_type, created_by)
    }
    
    /// Set geographical coordinates
    pub fn set_coordinates(&mut self, latitude: Decimal, longitude: Decimal) {
        self.latitude = Some(latitude);
        self.longitude = Some(longitude);
        self.updated_at = Utc::now();
    }
    
    /// Set geographical coordinates with accuracy
    pub fn set_coordinates_with_accuracy(&mut self, latitude: Decimal, longitude: Decimal, accuracy_meters: f32) {
        self.latitude = Some(latitude);
        self.longitude = Some(longitude);
        self.accuracy_meters = Some(accuracy_meters);
        self.updated_at = Utc::now();
    }
    
    /// Check if address has valid coordinates
    pub fn has_coordinates(&self) -> bool {
        self.latitude.is_some() && self.longitude.is_some()
    }
}

/// Builder for Address
pub struct AddressBuilder {
    address_id: Uuid,
    street_line1: Option<String>,
    street_line2: Option<String>,
    street_line3: Option<String>,
    street_line4: Option<String>,
    city_id: Option<Uuid>,
    postal_code: Option<String>,
    latitude: Option<Decimal>,
    longitude: Option<Decimal>,
    accuracy_meters: Option<f32>,
    address_type: AddressType,
    is_active: bool,
    created_by: Uuid,
}

impl AddressBuilder {
    pub fn new(
        address_id: Uuid,
        address_type: AddressType,
        created_by: Uuid,
    ) -> Self {
        Self {
            address_id,
            street_line1: None,
            street_line2: None,
            street_line3: None,
            street_line4: None,
            city_id: None,
            postal_code: None,
            latitude: None,
            longitude: None,
            accuracy_meters: None,
            address_type,
            is_active: true,
            created_by,
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
    
    pub fn city_id(mut self, city_id: Uuid) -> Self {
        self.city_id = Some(city_id);
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
    
    
    pub fn is_active(mut self, active: bool) -> Self {
        self.is_active = active;
        self
    }
    
    pub fn build(self) -> Result<Address, &'static str> {
        let street_line1 = self.street_line1
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "Street line 1 exceeds maximum length")?
            .unwrap_or_default();
            
        let street_line2 = self.street_line2
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "Street line 2 exceeds maximum length")?
            .unwrap_or_default();
            
        let street_line3 = self.street_line3
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "Street line 3 exceeds maximum length")?
            .unwrap_or_default();
            
        let street_line4 = self.street_line4
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "Street line 4 exceeds maximum length")?
            .unwrap_or_default();
            
        let postal_code = self.postal_code
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "Postal code exceeds maximum length")?;
            
        Ok(Address {
            address_id: self.address_id,
            street_line1,
            street_line2,
            street_line3,
            street_line4,
            city_id: self.city_id,
            postal_code,
            latitude: self.latitude,
            longitude: self.longitude,
            accuracy_meters: self.accuracy_meters,
            address_type: self.address_type,
            is_active: self.is_active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: self.created_by,
            updated_by: self.created_by,
        })
    }
}

/// Type of address for categorization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AddressType {
    /// Home/residential address
    Residential,
    /// Business/office address
    Business,
    /// Mailing address (P.O. Box, etc.)
    Mailing,
    /// Temporary address
    Temporary,
    /// Branch/agency location
    Branch,
    /// Other address types
    Other,
}

/// Type of messaging/communication method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessagingType {
    /// Email address
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

/// Messaging/communication identifier for a person
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Messaging {
    /// Unique identifier for this messaging record
    pub messaging_id: Uuid,
    /// Type of messaging/communication method
    pub messaging_type: MessagingType,
    /// The actual messaging identifier/address (email, phone, username, etc.)
    pub value: HeaplessString<100>,
    /// Description of the messaging type when MessagingType::Other is used
    pub other_type: Option<HeaplessString<20>>,
    /// Whether this messaging method is currently active/valid
    pub is_active: bool,
    /// Priority order for this messaging method (1 = highest priority)
    pub priority: Option<u8>,
    /// Audit fields
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
            messaging_id: Uuid::new_v4(),
            messaging_type,
            value,
            other_type: None,
            is_active: true,
            priority: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
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
            messaging_id: Uuid::new_v4(),
            messaging_type: MessagingType::Other,
            value,
            other_type: Some(other_type),
            is_active: true,
            priority: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
    
    /// Set priority for this messaging method
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = Some(priority);
        self
    }
    
    /// Set active status
    pub fn with_active_status(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
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

/// Entity reference table for managing person-to-entity relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityReference {
    /// Unique identifier for this entity reference
    pub id: Uuid,
    /// References Person.person_id
    pub person_id: Uuid,
    /// Type of entity relationship
    pub entity_role: RelationshipRole,
    /// External identifier for the reference (e.g., customer ID, employee ID)
    pub reference_external_id: Option<HeaplessString<50>>,
    /// Reference details in language 1
    pub reference_details_l1: Option<HeaplessString<50>>,
    /// Reference details in language 2
    pub reference_details_l2: Option<HeaplessString<50>>,
    /// Reference details in language 3
    pub reference_details_l3: Option<HeaplessString<50>>,
    /// Whether this entity reference is currently active
    pub is_active: bool,
    /// When this reference was created
    pub created_at: DateTime<Utc>,
    /// When this reference was last updated
    pub updated_at: DateTime<Utc>,
    /// Who created this reference
    pub created_by: Uuid,
    /// Who last updated this reference
    pub updated_by: Uuid,
}

impl EntityReference {
    /// Create a new entity reference
    pub fn new(
        person_id: Uuid,
        entity_role: RelationshipRole,
        reference_external_id: Option<HeaplessString<50>>,
        created_by: Uuid,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            person_id,
            entity_role,
            reference_external_id,
            reference_details_l1: None,
            reference_details_l2: None,
            reference_details_l3: None,
            is_active: true,
            created_at: now,
            updated_at: now,
            created_by,
            updated_by: created_by,
        }
    }

    /// Update reference details with multi-language support
    pub fn set_reference_details(
        &mut self,
        details_l1: Option<HeaplessString<50>>,
        details_l2: Option<HeaplessString<50>>,
        details_l3: Option<HeaplessString<50>>,
        updated_by: Uuid,
    ) {
        self.reference_details_l1 = details_l1;
        self.reference_details_l2 = details_l2;
        self.reference_details_l3 = details_l3;
        self.updated_at = Utc::now();
        self.updated_by = updated_by;
    }

    /// Deactivate this entity reference
    pub fn deactivate(&mut self, updated_by: Uuid) {
        self.is_active = false;
        self.updated_at = Utc::now();
        self.updated_by = updated_by;
    }

    /// Reactivate this entity reference
    pub fn reactivate(&mut self, updated_by: Uuid) {
        self.is_active = true;
        self.updated_at = Utc::now();
        self.updated_by = updated_by;
    }
}

/// Represents a person throughout the system for audit and tracking purposes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    /// Unique identifier for this person reference
    pub person_id: Uuid,
    
    /// Type of person (natural, legal, system, etc.)
    pub person_type: PersonType,
    
    /// Display name of the person
    pub display_name: HeaplessString<100>,
    
    /// External identifier (e.g., employee ID, badge number, system ID)
    pub external_identifier: Option<HeaplessString<50>>,
    
    /// References another Person.person_id for organizational hierarchy
    pub organization: Option<Uuid>,
    
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
    
    /// Department within organization
    pub department: Option<HeaplessString<50>>,
    
    /// References Address.address_id for person's location
    pub location: Option<Uuid>,
    
    /// Reference to another Person if this is a duplicate
    pub duplicate_of: Option<Uuid>,
    
    
    /// Whether this person reference is currently active
    pub is_active: bool,
    
    /// When this person reference was created
    pub created_at: DateTime<Utc>,
    
    /// When this person reference was last updated
    pub updated_at: DateTime<Utc>,
}

impl Person {
    /// Creates a new Person
    pub fn new(
        person_id: Uuid,
        person_type: PersonType,
        display_name: impl AsRef<str>,
    ) -> Result<Self, &'static str> {
        let display_name = HeaplessString::try_from(display_name.as_ref())
            .map_err(|_| "Display name exceeds maximum length")?;
            
        Ok(Self {
            person_id,
            person_type,
            display_name,
            external_identifier: None,
            organization: None,
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
            location: None,
            duplicate_of: None,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
    
    /// Creates a system user reference
    pub fn system() -> Self {
        Self {
            person_id: Uuid::nil(), // Use nil UUID for system
            person_type: PersonType::System,
            display_name: HeaplessString::try_from("SYSTEM").unwrap(),
            external_identifier: None,
            organization: None,
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
            location: None,
            duplicate_of: None,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    
    /// Builder for creating a Person with optional fields
    pub fn builder(person_id: Uuid, person_type: PersonType, display_name: impl AsRef<str>) -> PersonBuilder {
        PersonBuilder::new(person_id, person_type, display_name)
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
        self.updated_at = Utc::now();
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
        self.updated_at = Utc::now();
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
    person_id: Uuid,
    person_type: PersonType,
    display_name: String,
    external_identifier: Option<String>,
    organization: Option<Uuid>,
    messaging: Vec<Uuid>,
    department: Option<String>,
    location: Option<Uuid>,
    duplicate_of: Option<Uuid>,
    is_active: bool,
}

impl PersonBuilder {
    pub fn new(person_id: Uuid, person_type: PersonType, display_name: impl AsRef<str>) -> Self {
        Self {
            person_id,
            person_type,
            display_name: display_name.as_ref().to_string(),
            external_identifier: None,
            organization: None,
            messaging: Vec::new(),
            department: None,
            location: None,
            duplicate_of: None,
            is_active: true,
        }
    }
    
    pub fn external_identifier(mut self, identifier: impl AsRef<str>) -> Self {
        self.external_identifier = Some(identifier.as_ref().to_string());
        self
    }
    
    pub fn organization(mut self, organization_id: Uuid) -> Self {
        self.organization = Some(organization_id);
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
    
    pub fn location(mut self, address_id: Uuid) -> Self {
        self.location = Some(address_id);
        self
    }
    
    pub fn duplicate_of(mut self, person_id: Uuid) -> Self {
        self.duplicate_of = Some(person_id);
        self
    }
    
    pub fn is_active(mut self, active: bool) -> Self {
        self.is_active = active;
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
            person_id: self.person_id,
            person_type: self.person_type,
            display_name,
            external_identifier,
            organization: self.organization,
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
            location: self.location,
            duplicate_of: self.duplicate_of,
            is_active: self.is_active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}

/// Common person references for system operations
pub struct SystemPersons {
    /// System user for automated processes
    pub system: Uuid,
    /// Migration user for data migrations
    pub migration: Uuid,
    /// API integration user
    pub api_integration: Uuid,
    /// Batch processing user
    pub batch_processor: Uuid,
}

impl Default for SystemPersons {
    fn default() -> Self {
        Self {
            system: Uuid::nil(),
            migration: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
            api_integration: Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
            batch_processor: Uuid::parse_str("00000000-0000-0000-0000-000000000003").unwrap(),
        }
    }
}
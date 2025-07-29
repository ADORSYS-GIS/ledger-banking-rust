use chrono::{DateTime, Utc};
use heapless::{String as HeaplessString, Vec as HeaplessVec};
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
    /// Structured address components
    pub street_address: Option<HeaplessString<200>>,
    /// References City.city_id (city contains country and state references)
    pub city_id: Option<Uuid>,
    pub postal_code: Option<HeaplessString<20>>,
    
    /// Geographical coordinates (decimal degrees)
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    
    /// Verbal description for African locations ("near the big baobab tree, 2km from the market")
    pub address_detail: Option<HeaplessString<200>>,
    
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
            street_address: None,
            city_id: None,
            postal_code: None,
            latitude: None,
            longitude: None,
            address_detail: None,
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
    
    /// Check if address has valid coordinates
    pub fn has_coordinates(&self) -> bool {
        self.latitude.is_some() && self.longitude.is_some()
    }
}

/// Builder for Address
pub struct AddressBuilder {
    address_id: Uuid,
    street_address: Option<String>,
    city_id: Option<Uuid>,
    postal_code: Option<String>,
    latitude: Option<Decimal>,
    longitude: Option<Decimal>,
    address_detail: Option<String>,
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
            street_address: None,
            city_id: None,
            postal_code: None,
            latitude: None,
            longitude: None,
            address_detail: None,
            address_type,
            is_active: true,
            created_by,
        }
    }
    
    pub fn street_address(mut self, address: impl AsRef<str>) -> Self {
        self.street_address = Some(address.as_ref().to_string());
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
    
    pub fn address_detail(mut self, detail: impl AsRef<str>) -> Self {
        self.address_detail = Some(detail.as_ref().to_string());
        self
    }
    
    pub fn is_active(mut self, active: bool) -> Self {
        self.is_active = active;
        self
    }
    
    pub fn build(self) -> Result<Address, &'static str> {
        let street_address = self.street_address
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "Street address exceeds maximum length")?;
            
        let postal_code = self.postal_code
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "Postal code exceeds maximum length")?;
            
        let address_detail = self.address_detail
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "Address detail exceeds maximum length")?;
            
        Ok(Address {
            address_id: self.address_id,
            street_address,
            city_id: self.city_id,
            postal_code,
            latitude: self.latitude,
            longitude: self.longitude,
            address_detail,
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
    
    /// References to Messaging.messaging_id (up to 20 messaging methods)
    pub messaging: HeaplessVec<Uuid, 20>,
    
    /// Department within organization
    pub department: Option<HeaplessString<50>>,
    
    /// References Address.address_id for person's location
    pub location: Option<Uuid>,
    
    /// Reference to another Person if this is a duplicate
    pub duplicate_of: Option<Uuid>,
    
    /// Reference to related entity (customer_id, employee_id, etc.)
    pub entity_reference: Option<Uuid>,
    
    /// Entity type for the reference (e.g., "customer", "employee", "shareholder")
    pub entity_type: Option<HeaplessString<50>>,
    
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
            messaging: HeaplessVec::new(),
            department: None,
            location: None,
            duplicate_of: None,
            entity_reference: None,
            entity_type: None,
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
            messaging: HeaplessVec::new(),
            department: None,
            location: None,
            duplicate_of: None,
            entity_reference: None,
            entity_type: None,
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
    pub fn add_messaging_reference(&mut self, messaging_id: Uuid) -> Result<(), &'static str> {
        self.messaging.push(messaging_id)
            .map_err(|_| "Maximum 20 messaging entries allowed")?;
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Remove a messaging method reference by ID
    pub fn remove_messaging_reference(&mut self, messaging_id: Uuid) -> bool {
        if let Some(pos) = self.messaging.iter().position(|&id| id == messaging_id) {
            self.messaging.swap_remove(pos);
            self.updated_at = Utc::now();
            true
        } else {
            false
        }
    }
    
    /// Check if person has a specific messaging method reference
    pub fn has_messaging_reference(&self, messaging_id: Uuid) -> bool {
        self.messaging.contains(&messaging_id)
    }
    
    /// Get all messaging reference IDs
    pub fn get_messaging_references(&self) -> &[Uuid] {
        &self.messaging
    }
    
    /// Get count of messaging references
    pub fn messaging_count(&self) -> usize {
        self.messaging.len()
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
    entity_reference: Option<Uuid>,
    entity_type: Option<String>,
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
            entity_reference: None,
            entity_type: None,
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
    
    pub fn entity_reference(mut self, entity_id: Uuid, entity_type: impl AsRef<str>) -> Self {
        self.entity_reference = Some(entity_id);
        self.entity_type = Some(entity_type.as_ref().to_string());
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
            
        let entity_type = self.entity_type
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "Entity type exceeds maximum length")?;
            
        let department = self.department
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "Department exceeds maximum length")?;
            
        // Convert Vec<Uuid> to HeaplessVec<Uuid, 20>
        let mut messaging_vec = HeaplessVec::new();
        for messaging_id in self.messaging {
            messaging_vec.push(messaging_id)
                .map_err(|_| "Maximum 20 messaging entries allowed")?;
        }
            
        Ok(Person {
            person_id: self.person_id,
            person_type: self.person_type,
            display_name,
            external_identifier,
            organization: self.organization,
            messaging: messaging_vec,
            department,
            location: self.location,
            duplicate_of: self.duplicate_of,
            entity_reference: self.entity_reference,
            entity_type,
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
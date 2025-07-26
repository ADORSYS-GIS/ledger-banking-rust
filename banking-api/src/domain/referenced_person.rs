use chrono::{DateTime, Utc};
use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

/// Contact information for a referenced person
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub email: Option<HeaplessString<100>>,
    pub phone: Option<HeaplessString<20>>,
    pub department: Option<HeaplessString<50>>,
    pub office_location: Option<HeaplessString<100>>,
}

/// Represents a person referenced throughout the system for audit and tracking purposes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferencedPerson {
    /// Unique identifier for this person reference
    pub person_id: Uuid,
    
    /// Type of person (natural, legal, system, etc.)
    pub person_type: PersonType,
    
    /// Display name of the person
    pub display_name: HeaplessString<100>,
    
    /// External identifier (e.g., employee ID, badge number, system ID)
    pub external_identifier: Option<HeaplessString<50>>,
    
    /// Organization/department for employees or company name for legal entities
    pub organization: Option<HeaplessString<100>>,
    
    /// Contact information for audit/compliance purposes
    pub contact_info: Option<ContactInfo>,
    
    /// Reference to another ReferencedPerson if this is a duplicate
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

impl ReferencedPerson {
    /// Creates a new ReferencedPerson
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
            contact_info: None,
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
            contact_info: None,
            duplicate_of: None,
            entity_reference: None,
            entity_type: None,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    
    /// Builder for creating a ReferencedPerson with optional fields
    pub fn builder(person_id: Uuid, person_type: PersonType, display_name: impl AsRef<str>) -> ReferencedPersonBuilder {
        ReferencedPersonBuilder::new(person_id, person_type, display_name)
    }
}

/// Builder for ReferencedPerson
pub struct ReferencedPersonBuilder {
    person_id: Uuid,
    person_type: PersonType,
    display_name: String,
    external_identifier: Option<String>,
    organization: Option<String>,
    contact_info: Option<ContactInfo>,
    duplicate_of: Option<Uuid>,
    entity_reference: Option<Uuid>,
    entity_type: Option<String>,
    is_active: bool,
}

impl ReferencedPersonBuilder {
    pub fn new(person_id: Uuid, person_type: PersonType, display_name: impl AsRef<str>) -> Self {
        Self {
            person_id,
            person_type,
            display_name: display_name.as_ref().to_string(),
            external_identifier: None,
            organization: None,
            contact_info: None,
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
    
    pub fn organization(mut self, org: impl AsRef<str>) -> Self {
        self.organization = Some(org.as_ref().to_string());
        self
    }
    
    pub fn contact_info(mut self, contact: ContactInfo) -> Self {
        self.contact_info = Some(contact);
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
    
    pub fn build(self) -> Result<ReferencedPerson, &'static str> {
        let display_name = HeaplessString::try_from(self.display_name.as_str())
            .map_err(|_| "Display name exceeds maximum length")?;
            
        let external_identifier = self.external_identifier
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "External identifier exceeds maximum length")?;
            
        let organization = self.organization
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "Organization exceeds maximum length")?;
            
        let entity_type = self.entity_type
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| "Entity type exceeds maximum length")?;
            
        Ok(ReferencedPerson {
            person_id: self.person_id,
            person_type: self.person_type,
            display_name,
            external_identifier,
            organization,
            contact_info: self.contact_info,
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
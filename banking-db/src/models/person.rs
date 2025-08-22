use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;

/// Database model for location type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "location_type", rename_all = "PascalCase")]
pub enum LocationType {
    Residential,
    Business,
    Mailing,
    Temporary,
    Branch,
    Community,
    Other,
}

/// Database model for messaging type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "messaging_type", rename_all = "PascalCase")]
pub enum MessagingType {
    Email,
    Phone,
    Sms,
    WhatsApp,
    Telegram,
    Skype,
    Teams,
    Signal,
    WeChat,
    Viber,
    Messenger,
    LinkedIn,
    Slack,
    Discord,
    Other,
}

/// Database model for person type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "person_type", rename_all = "PascalCase")]
pub enum PersonType {
    Natural,
    Legal,
    System,
    Integration,
    Unknown,
}

impl std::fmt::Display for PersonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersonType::Natural => write!(f, "Natural"),
            PersonType::Legal => write!(f, "Legal"),
            PersonType::System => write!(f, "System"),
            PersonType::Integration => write!(f, "Integration"),
            PersonType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Database model for person entity type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "person_entity_type", rename_all = "PascalCase")]
pub enum RelationshipRole {
    Customer,
    Employee,
    Shareholder,
    Director,
    BeneficialOwner,
    Agent,
    Vendor,
    Partner,
    RegulatoryContact,
    EmergencyContact,
    SystemAdmin,
    Other,
}

/// # Repository Trait
/// - FQN: banking-db/src/repository/person_repository.rs/CountryRepository
/// # Nature
/// - RuntimeImmutable: Creation, Modification requires reload of caches
/// - Cacheable: ApplicationScope
/// # Documentation
/// - Country structure with ISO 3166-1 alpha-2 code
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CountryModel {
    /// # Trait method
    /// - find_by_id
    /// - find_by_ids
    /// - exists_by_id
    /// # Nature
    /// - primary index
    pub id: Uuid,
    /// # Documentation
    /// - ISO 3166-1 alpha-2 country code (e.g., "CM", "US", "GB")
    /// # Trait method
    /// - find_ids_by_iso2
    /// - find_by_iso2
    /// # Nature
    /// - unique
    pub iso2: HeaplessString<2>,
    pub name_l1: HeaplessString<100>,
    pub name_l2: Option<HeaplessString<100>>,
    pub name_l3: Option<HeaplessString<100>>,
}

/// # Service Trait
/// - FQN: banking-db/src/repository/person_repository.rs/CountrySubdivisionRepository
/// # Nature
/// - RuntimeImmutable: Creation, Modification requires reload of caches
/// # Documentation
/// - Database model for CountrySubdivision
/// - CountrySubdivision structure with multilingual support
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CountrySubdivisionModel {
    /// # Trait methods
    /// - find_by_id
    /// - find_by_ids
    /// - exists_by_id
    /// # Nature
    /// - primary index
    pub id: Uuid,
    /// # Trait method
    /// - find_ids_by_country_id
    /// - find_by_country_id
    pub country_id: Uuid,
    /// # Documentation
    /// - if non existant the first 10 chars of the name_l1
    /// # Trait method
    /// - find_by_code
    ///     - code: self.code
    /// # Nature
    /// - unique
    pub code: HeaplessString<10>,
    pub name_l1: HeaplessString<100>,
    pub name_l2: Option<HeaplessString<100>>,
    pub name_l3: Option<HeaplessString<100>>,
}

/// # Service Trait
/// - FQN: banking-db/src/repository/person_repository.rs/LocalityRepository
/// # Nature
/// - RuntimeImmutable: Creation, Modification requires reload of caches
/// # Documentation
/// - Database model for Locality
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LocalityModel {
    /// # Trait methods
    /// - find_by_id
    /// - find_by_ids
    /// - exists_by_id
    /// # Nature
    /// - primary index
    pub id: Uuid,
    /// # Trait method
    /// - find_ids_by_country_subdivision_id
    /// - find_by_country_subdivision_id
    pub country_subdivision_id: Option<Uuid>,
    /// # Documentation
    /// - If non existant, country subdivision code '_' the first 10 chars of the name_l1
    /// # Trait method
    /// - find_by_code
    ///     - code: self.code
    /// # Nature
    /// - unique
    pub code: HeaplessString<50>,
    pub name_l1: HeaplessString<50>,
    pub name_l2: Option<HeaplessString<50>>,
    pub name_l3: Option<HeaplessString<50>>,
}

/// # Service Trait
/// - FQN: banking-db/src/repository/person_repository.rs/LocationRepository
/// # Nature
/// - Immutable: 
///     - A location can be fixed if eroneous, but does not change base on the holder.  
///     - A customer changing address receives a new location.
///     - Many customers can share the same location object 
/// - AuditLog
/// # Documentation
/// - Database model for Location
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LocationModel {
    /// # Trait method
    /// - find_by_id
    /// - find_by_ids
    /// - exists_by_id
    /// # Nature
    /// - primary index
    pub id: Uuid,
    /// # Trait method
    /// - find_ids_by_street_line1
    /// # Documentation
    /// - Structured location components - 4 street lines
    pub street_line1: HeaplessString<50>,
    pub street_line2: Option<HeaplessString<50>>,
    pub street_line3: Option<HeaplessString<50>>,
    pub street_line4: Option<HeaplessString<50>>,
    /// # Trait method
    /// - find_ids_by_locality_id
    /// - find_by_locality_id
    pub locality_id: Uuid,
    pub postal_code: Option<HeaplessString<20>>,

    /// Geographical coordinates (decimal degrees)
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub accuracy_meters: Option<f32>,

    
    /// # Trait method
    /// - find_ids_by_location_type
    /// - find_by_location_type
    /// - find_location_by_type_and_locality
    /// # Documentation
    /// - Location type for categorization
    #[serde(serialize_with = "serialize_location_type", deserialize_with = "deserialize_location_type")]
    pub location_type: LocationType,
}

/// # Service Trait
/// - FQN: banking-db/src/repository/person_repository.rs/MessagingRepository
/// # Nature
/// - Immutable: 
///     - A location can be fixed if eroneous, but does not change base on the holder.  
///     - A customer changing phone number receives an association with a new record.
///     - Many customers can share the same messaging object 
/// - AuditLog
/// # Documentation
/// - Database model for Messaging
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MessagingModel {
    /// # Trait method
    /// - find_by_id
    /// - find_by_ids
    /// - exists_by_id
    /// # Nature
    /// - primary index
    pub id: Uuid,
    /// # Documentation
    /// - Type of messaging/communication method
    #[serde(serialize_with = "serialize_messaging_type", deserialize_with = "deserialize_messaging_type")]
    pub messaging_type: MessagingType,
    /// # Trait method
    /// - find_ids_by_value
    /// # Documentation
    /// - The actual messaging identifier/location (email, phone, username, etc.)
    pub value: HeaplessString<100>,
    /// # Documentation
    /// - Description of the messaging type when MessagingType::Other is used
    pub other_type: Option<HeaplessString<20>>,

    pub audit_log_id: Uuid,
}

/// # Service Trait
/// - FQN: banking-db/src/repository/person_repository.rs/EntityReferenceRepository
/// # Documentation
/// - Entity reference table for managing person-to-entity relationships
/// - Database model for EntityReference
/// # Nature
/// - Mutable 
///     - version field used to track changes.
///     - store a copy in table EntityReferenceAuditModel during modification. 
/// - AuditLog
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EntityReferenceModel {
    /// # Trait method
    /// - find_by_id
    /// - find_by_ids
    /// - exists_by_id
    /// # Nature
    /// - primary index
    pub id: Uuid,

    /// # Documentation
    /// - version number, increased whenever a reference changes.
    /// - change triggers storage of old version in an audit database.
    pub version: u16,

    /// # Documentation
    /// - References PersonModel.person_id
    /// # Trait method
    /// - find_ids_by_person_id
    /// - find_by_person_id
    pub person_id: Uuid,

    /// # Documentation
    /// - Type of entity relationship
    #[serde(serialize_with = "serialize_person_entity_type", deserialize_with = "deserialize_person_entity_type")]
    pub entity_role: RelationshipRole,

    /// # Documentation
    /// - External identifier for the reference (e.g., customer ID, employee ID)
    /// # Trait method
    /// - find_by_reference_external_id
    pub reference_external_id: HeaplessString<50>,

    pub reference_details_l1: Option<HeaplessString<50>>,
    pub reference_details_l2: Option<HeaplessString<50>>,
    pub reference_details_l3: Option<HeaplessString<50>>,

    pub audit_log_id: Uuid,
}

/// # Service Trait
/// - FQN: banking-db/src/repository/person_repository.rs/EntityReferenceAuditRepository
/// # Documentation
/// - Entity reference audit table for storing changes on entity references.
/// # Nature
/// - Immutable 
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EntityReferenceAuditModel {
    /// # Trait method
    /// - find_by_id
    /// - find_by_ids
    /// # Nature
    /// - Coumpound primary index with self.version
    pub id: Uuid,

    /// # Documentation
    /// - Coumpound primary index with self.version
    pub version: u16,

    /// # Documentation
    /// - find_ids_by_person_id
    /// - find_by_person_id
    pub person_id: Uuid,

    /// # Documentation
    /// - Type of entity relationship
    #[serde(serialize_with = "serialize_person_entity_type", deserialize_with = "deserialize_person_entity_type")]
    pub entity_role: RelationshipRole,

    /// # Documentation
    /// - External identifier for the reference (e.g., customer ID, employee ID)
    /// # Trait method
    /// - find_by_reference_external_id
    pub reference_external_id: HeaplessString<50>,

    pub reference_details_l1: Option<HeaplessString<50>>,
    pub reference_details_l2: Option<HeaplessString<50>>,
    pub reference_details_l3: Option<HeaplessString<50>>,

    pub audit_log_id: Uuid,
}

/// # Service Trait
/// - FQN: banking-db/src/repository/person_repository.rs/EntityReferenceAuditRepository
/// # Documentation
/// - Database model for person
/// - Represents a person throughout the system for audit and tracking purposes
/// # Nature
/// - Mutable 
///     - version field used to track changes.
///     - store a copy in table EntityReferenceAudit during modification. 
/// - AuditLog
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PersonModel {
    /// # Trait method
    /// - find_by_id
    /// - find_by_ids
    /// - exists_by_id
    /// # Nature
    /// - Primary index
    pub id: Uuid,
    
    pub version: u16,

    #[serde(serialize_with = "serialize_person_type", deserialize_with = "deserialize_person_type")]
    pub person_type: PersonType,
    
    pub display_name: HeaplessString<100>,

    /// # Trait method
    /// - get_ids_by_external_identifier
    /// - get_by_external_identifier
    /// # Documentation
    /// External identifier (e.g., employee ID, badge number, system ID)
    pub external_identifier: Option<HeaplessString<50>>,

    /// # Trait method
    /// - get_by_entity_reference
    pub entity_reference_count: u8,
    
    /// # Documentation
    /// References PersonModel.person_id for organizational hierarchy
    pub organization_person_id: Option<Uuid>,
    
    /// # Documentation
    /// References to MessagingModel.messaging_id (up to 5 messaging methods)
    pub messaging1_id: Option<Uuid>,
    #[serde(serialize_with = "serialize_messaging_type_option", deserialize_with = "deserialize_messaging_type_option")]
    pub messaging1_type: Option<MessagingType>,
    pub messaging2_id: Option<Uuid>,
    #[serde(serialize_with = "serialize_messaging_type_option", deserialize_with = "deserialize_messaging_type_option")]
    pub messaging2_type: Option<MessagingType>,
    pub messaging3_id: Option<Uuid>,
    #[serde(serialize_with = "serialize_messaging_type_option", deserialize_with = "deserialize_messaging_type_option")]
    pub messaging3_type: Option<MessagingType>,
    pub messaging4_id: Option<Uuid>,
    #[serde(serialize_with = "serialize_messaging_type_option", deserialize_with = "deserialize_messaging_type_option")]
    pub messaging4_type: Option<MessagingType>,
    pub messaging5_id: Option<Uuid>,
    #[serde(serialize_with = "serialize_messaging_type_option", deserialize_with = "deserialize_messaging_type_option")]
    pub messaging5_type: Option<MessagingType>,
    
    /// # Documentation
    /// Department within organization
    pub department: Option<HeaplessString<50>>,

    /// # Documentation
    /// References LocationModel.location_id for person's location
    pub location_id: Option<Uuid>,
    
    pub duplicate_of_person_id: Option<Uuid>,
}

// Serialization functions for PersonType
fn serialize_person_type<S>(person_type: &PersonType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match person_type {
        PersonType::Natural => "Natural",
        PersonType::Legal => "Legal",
        PersonType::System => "System",
        PersonType::Integration => "Integration",
        PersonType::Unknown => "Unknown",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_person_type<'de, D>(deserializer: D) -> Result<PersonType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Natural" => Ok(PersonType::Natural),
        "Legal" => Ok(PersonType::Legal),
        "System" => Ok(PersonType::System),
        "Integration" => Ok(PersonType::Integration),
        "unknown" => Ok(PersonType::Unknown),
        _ => Err(serde::de::Error::custom(format!("Unknown person type: {s}"))),
    }
}

// Serialization functions for LocationType
fn serialize_location_type<S>(location_type: &LocationType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match location_type {
        LocationType::Residential => "residential",
        LocationType::Business => "business",
        LocationType::Mailing => "mailing",
        LocationType::Temporary => "temporary",
        LocationType::Branch => "branch",
        LocationType::Community => "community",
        LocationType::Other => "other",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_location_type<'de, D>(deserializer: D) -> Result<LocationType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "residential" => Ok(LocationType::Residential),
        "business" => Ok(LocationType::Business),
        "mailing" => Ok(LocationType::Mailing),
        "temporary" => Ok(LocationType::Temporary),
        "branch" => Ok(LocationType::Branch),
        "community" => Ok(LocationType::Community),
        "other" => Ok(LocationType::Other),
        _ => Err(serde::de::Error::custom(format!("Unknown location type: {s}"))),
    }
}

// Serialization functions for MessagingType
fn serialize_messaging_type<S>(messaging_type: &MessagingType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match messaging_type {
        MessagingType::Email => "email",
        MessagingType::Phone => "phone",
        MessagingType::Sms => "sms",
        MessagingType::WhatsApp => "whatsapp",
        MessagingType::Telegram => "telegram",
        MessagingType::Skype => "skype",
        MessagingType::Teams => "teams",
        MessagingType::Signal => "signal",
        MessagingType::WeChat => "wechat",
        MessagingType::Viber => "viber",
        MessagingType::Messenger => "messenger",
        MessagingType::LinkedIn => "linkedin",
        MessagingType::Slack => "slack",
        MessagingType::Discord => "discord",
        MessagingType::Other => "other",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_messaging_type<'de, D>(deserializer: D) -> Result<MessagingType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "email" => Ok(MessagingType::Email),
        "phone" => Ok(MessagingType::Phone),
        "sms" => Ok(MessagingType::Sms),
        "whatsapp" => Ok(MessagingType::WhatsApp),
        "telegram" => Ok(MessagingType::Telegram),
        "skype" => Ok(MessagingType::Skype),
        "teams" => Ok(MessagingType::Teams),
        "signal" => Ok(MessagingType::Signal),
        "wechat" => Ok(MessagingType::WeChat),
        "viber" => Ok(MessagingType::Viber),
        "messenger" => Ok(MessagingType::Messenger),
        "linkedin" => Ok(MessagingType::LinkedIn),
        "slack" => Ok(MessagingType::Slack),
        "discord" => Ok(MessagingType::Discord),
        "other" => Ok(MessagingType::Other),
        _ => Err(serde::de::Error::custom(format!("Unknown messaging type: {s}"))),
    }
}

// Serialization functions for Option<MessagingType>
fn serialize_messaging_type_option<S>(messaging_type: &Option<MessagingType>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match messaging_type {
        Some(msg_type) => serialize_messaging_type(msg_type, serializer),
        None => serializer.serialize_none(),
    }
}

fn deserialize_messaging_type_option<'de, D>(deserializer: D) -> Result<Option<MessagingType>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        Some(s) => match s.as_str() {
            "email" => Ok(Some(MessagingType::Email)),
            "phone" => Ok(Some(MessagingType::Phone)),
            "sms" => Ok(Some(MessagingType::Sms)),
            "whatsapp" => Ok(Some(MessagingType::WhatsApp)),
            "telegram" => Ok(Some(MessagingType::Telegram)),
            "skype" => Ok(Some(MessagingType::Skype)),
            "teams" => Ok(Some(MessagingType::Teams)),
            "signal" => Ok(Some(MessagingType::Signal)),
            "wechat" => Ok(Some(MessagingType::WeChat)),
            "viber" => Ok(Some(MessagingType::Viber)),
            "messenger" => Ok(Some(MessagingType::Messenger)),
            "linkedin" => Ok(Some(MessagingType::LinkedIn)),
            "slack" => Ok(Some(MessagingType::Slack)),
            "discord" => Ok(Some(MessagingType::Discord)),
            "other" => Ok(Some(MessagingType::Other)),
            _ => Err(serde::de::Error::custom(format!("Unknown messaging type: {s}"))),
        },
        None => Ok(None),
    }
}

// Serialization functions for RelationshipRole
fn serialize_person_entity_type<S>(entity_role: &RelationshipRole, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match entity_role {
        RelationshipRole::Customer => "customer",
        RelationshipRole::Employee => "employee",
        RelationshipRole::Shareholder => "shareholder",
        RelationshipRole::Director => "director",
        RelationshipRole::BeneficialOwner => "beneficialowner",
        RelationshipRole::Agent => "agent",
        RelationshipRole::Vendor => "vendor",
        RelationshipRole::Partner => "partner",
        RelationshipRole::RegulatoryContact => "regulatorycontact",
        RelationshipRole::EmergencyContact => "emergencycontact",
        RelationshipRole::SystemAdmin => "systemadmin",
        RelationshipRole::Other => "other",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_person_entity_type<'de, D>(deserializer: D) -> Result<RelationshipRole, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "customer" => Ok(RelationshipRole::Customer),
        "employee" => Ok(RelationshipRole::Employee),
        "shareholder" => Ok(RelationshipRole::Shareholder),
        "director" => Ok(RelationshipRole::Director),
        "beneficialowner" => Ok(RelationshipRole::BeneficialOwner),
        "agent" => Ok(RelationshipRole::Agent),
        "vendor" => Ok(RelationshipRole::Vendor),
        "partner" => Ok(RelationshipRole::Partner),
        "regulatorycontact" => Ok(RelationshipRole::RegulatoryContact),
        "emergencycontact" => Ok(RelationshipRole::EmergencyContact),
        "systemadmin" => Ok(RelationshipRole::SystemAdmin),
        "other" => Ok(RelationshipRole::Other),
        _ => Err(serde::de::Error::custom(format!("Unknown person entity type: {s}"))),
    }
}

/// # Cache
/// - Immutable Set of Immutable Records
/// - Application Scope
/// - Concurent
/// - Primary
///     - country_id
/// - Unique Mandatory Lookup Fields
///     - iso2
/// # Documentation
/// - Index model for Country
#[derive(Debug, Clone, FromRow)]
pub struct CountryIdxModel {
    pub country_id: Uuid,
    pub iso2: HeaplessString<2>,
}

pub struct CountryIdxModelCache {
    by_id: HashMap<Uuid, CountryIdxModel>,
    by_iso2: HashMap<HeaplessString<2>, Uuid>,
}

impl CountryIdxModelCache {
    pub fn new(
        items: Vec<CountryIdxModel>,
    ) -> Result<Arc<Self>, &'static str> {
        let mut by_id = HashMap::new();
        let mut by_iso2 = HashMap::new();

        for item in items {
            let primary_key = item.country_id;
            if by_id.contains_key(&primary_key) {
                return Err("Duplicate primary key: country_id");
            }

            if by_iso2.contains_key(&item.iso2) {
                return Err("Duplicate unique index value: iso2");
            }
            by_iso2.insert(item.iso2.clone(), primary_key);
            
            by_id.insert(primary_key, item);
        }

        Ok(Arc::new(CountryIdxModelCache {
            by_id,
            by_iso2,
        }))
    }

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        self.by_id.contains_key(primary_key)
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<CountryIdxModel> {
        self.by_id.get(primary_key).cloned()
    }

    pub fn get_by_iso2(&self, key: &HeaplessString<2>) -> Option<Uuid> {
        self.by_iso2.get(key).copied()
    }
}


/// # Cache
/// - Immutable Set of Immutable Records
/// - Application Scope
/// - Concurent
/// - Primary
///     - country_subdivision_id
/// - Unique Mandatory Lookup Fields
///     - code_hash
/// - Non Unique Mandatory Lookup Fields
///     - country_id
/// # Documentation
/// - Index model for CountrySubdivisionIdxModel
#[derive(Debug, Clone, FromRow)]
pub struct CountrySubdivisionIdxModel {
    pub country_subdivision_id: Uuid,
    pub country_id: Uuid,
    pub code_hash: i64,
}

pub struct CountrySubdivisionIdxModelCache {
    by_id: HashMap<Uuid, CountrySubdivisionIdxModel>,
    by_code_hash: HashMap<i64, Uuid>,
    by_country_id: HashMap<Uuid, Vec<Uuid>>,
}

impl CountrySubdivisionIdxModelCache {
    pub fn new(
        items: Vec<CountrySubdivisionIdxModel>,
    ) -> Result<Arc<Self>, &'static str> {
        let mut by_id = HashMap::new();
        let mut by_code_hash = HashMap::new();
        let mut by_country_id = HashMap::new();

        for item in items {
            let primary_key = item.country_subdivision_id;
            if by_id.contains_key(&primary_key) {
                return Err("Duplicate primary key: country_subdivision_id");
            }

            if by_code_hash.contains_key(&item.code_hash) {
                return Err("Duplicate unique index value: code_hash");
            }
            by_code_hash.insert(item.code_hash, primary_key);

            by_country_id
                .entry(item.country_id)
                .or_insert_with(Vec::new)
                .push(primary_key);
            
            by_id.insert(primary_key, item);
        }

        Ok(Arc::new(CountrySubdivisionIdxModelCache {
            by_id,
            by_code_hash,
            by_country_id,
        }))
    }

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        self.by_id.contains_key(primary_key)
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<CountrySubdivisionIdxModel> {
        self.by_id.get(primary_key).cloned()
    }

    pub fn get_by_code_hash(&self, key: &i64) -> Option<Uuid> {
        self.by_code_hash.get(key).copied()
    }

    pub fn get_by_country_id(&self, key: &Uuid) -> Option<&Vec<Uuid>> {
        self.by_country_id.get(key)
    }
}

/// # Cache
/// - Immutable Set of Immutable Records
/// - Application Scope
/// - Concurent
/// - Primary
///     - locality_id
/// - Unique Mandatory Lookup Fields
///     - code_hash
/// - Non Unique Mandatory Lookup Fields
///     - country_subdivision_id
/// # Documentation
/// - Index model for Locality
/// - Application Scope Cache
#[derive(Debug, Clone, FromRow)]
pub struct LocalityIdxModel {
    pub locality_id: Uuid,
    pub country_subdivision_id: Uuid,
    pub code_hash: i64,
}

pub struct LocalityIdxModelCache {
    by_id: HashMap<Uuid, LocalityIdxModel>,
    by_code_hash: HashMap<i64, Uuid>,
    by_country_subdivision_id: HashMap<Uuid, Vec<Uuid>>,
}

impl LocalityIdxModelCache {
    pub fn new(
        items: Vec<LocalityIdxModel>,
    ) -> Result<Arc<Self>, &'static str> {
        let mut by_id = HashMap::new();
        let mut by_code_hash = HashMap::new();
        let mut by_country_subdivision_id = HashMap::new();

        for item in items {
            let primary_key = item.locality_id;
            if by_id.contains_key(&primary_key) {
                return Err("Duplicate primary key: locality_id");
            }

            if by_code_hash.contains_key(&item.code_hash) {
                return Err("Duplicate unique index value: code_hash");
            }
            by_code_hash.insert(item.code_hash, primary_key);

            by_country_subdivision_id
                .entry(item.country_subdivision_id)
                .or_insert_with(Vec::new)
                .push(primary_key);
            
            by_id.insert(primary_key, item);
        }

        Ok(Arc::new(LocalityIdxModelCache {
            by_id,
            by_code_hash,
            by_country_subdivision_id,
        }))
    }

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        self.by_id.contains_key(primary_key)
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<LocalityIdxModel> {
        self.by_id.get(primary_key).cloned()
    }

    pub fn get_by_code_hash(&self, key: &i64) -> Option<Uuid> {
        self.by_code_hash.get(key).copied()
    }

    pub fn get_by_country_subdivision_id(&self, key: &Uuid) -> Option<&Vec<Uuid>> {
        self.by_country_subdivision_id.get(key)
    }
}


/// # Cache
/// - Mutable Set of Immutable Records
/// - Application Scope + Notification DB Table (new records)
/// - Primary 
///     - location_id
/// - Non Unique Lookup Fields
///     - locality_id
/// # Documentation
/// - Index model for Location
#[derive(Debug, Clone, FromRow)]
pub struct LocationIdxModel {
    pub location_id: Uuid,
    pub locality_id: Option<Uuid>,
}

pub struct LocationIdxModelCache {
    by_id: HashMap<Uuid, LocationIdxModel>,
    by_locality_id: HashMap<Uuid, Vec<Uuid>>,
}

impl LocationIdxModelCache {
    pub fn new(
        items: Vec<LocationIdxModel>,
    ) -> Result<Arc<Self>, &'static str> {
        let mut by_id = HashMap::new();
        let mut by_locality_id = HashMap::new();

        for item in items {
            let primary_key = item.location_id;
            if by_id.contains_key(&primary_key) {
                return Err("Duplicate primary key: location_id");
            }

            if let Some(locality_id) = item.locality_id {
                by_locality_id
                    .entry(locality_id)
                    .or_insert_with(Vec::new)
                    .push(primary_key);
            }
            
            by_id.insert(primary_key, item);
        }

        Ok(Arc::new(LocationIdxModelCache {
            by_id,
            by_locality_id,
        }))
    }

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        self.by_id.contains_key(primary_key)
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<LocationIdxModel> {
        self.by_id.get(primary_key).cloned()
    }

    pub fn get_by_locality_id(&self, key: &Uuid) -> Option<&Vec<Uuid>> {
        self.by_locality_id.get(key)
    }
}


/// # Cache
/// - Mutable Set of Immutable Records
/// - Application Scope + Notification DB Table (new records)
/// - Primary 
///     - messaging_id
/// - Unique Mandatory Lookup Fields
///     - value_hash
/// # Documentation
/// - Index model for Messaging
#[derive(Debug, Clone, FromRow)]
pub struct MessagingIdxModel {
    pub messaging_id: Uuid,
    pub value_hash: i64,
}

pub struct MessagingIdxModelCache {
    by_id: HashMap<Uuid, MessagingIdxModel>,
    by_value_hash: HashMap<i64, Uuid>,
}

impl MessagingIdxModelCache {
    pub fn new(
        items: Vec<MessagingIdxModel>,
    ) -> Result<Arc<Self>, &'static str> {
        let mut by_id = HashMap::new();
        let mut by_value_hash = HashMap::new();

        for item in items {
            let primary_key = item.messaging_id;
            if by_id.contains_key(&primary_key) {
                return Err("Duplicate primary key: messaging_id");
            }

            if by_value_hash.contains_key(&item.value_hash) {
                return Err("Duplicate unique index value: value_hash");
            }
            by_value_hash.insert(item.value_hash, primary_key);
            
            by_id.insert(primary_key, item);
        }

        Ok(Arc::new(MessagingIdxModelCache {
            by_id,
            by_value_hash,
        }))
    }

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        self.by_id.contains_key(primary_key)
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<MessagingIdxModel> {
        self.by_id.get(primary_key).cloned()
    }

    pub fn get_by_value_hash(&self, key: &i64) -> Option<Uuid> {
        self.by_value_hash.get(key).copied()
    }
}


/// # Cache
/// - Mutable Set of Mutable Records
/// - Application Scope + Entry count hint in PersonModel.
/// - Primary
///     - entity_reference_id
/// - Non Unique Mandatory Lookup Fields
///     - person_id
/// # Documentation
/// - Index model for EntityReference
#[derive(Debug, Clone, FromRow)]
pub struct EntityReferenceIdxModel {
    pub entity_reference_id: Uuid,
    pub person_id: Uuid,
}

pub struct EntityReferenceIdxModelCache {
    by_id: HashMap<Uuid, EntityReferenceIdxModel>,
    by_person_id: HashMap<Uuid, Vec<Uuid>>,
}

impl EntityReferenceIdxModelCache {
    pub fn new(
        items: Vec<EntityReferenceIdxModel>,
    ) -> Result<Arc<Self>, &'static str> {
        let mut by_id = HashMap::new();
        let mut by_person_id = HashMap::new();

        for item in items {
            let primary_key = item.entity_reference_id;
            if by_id.contains_key(&primary_key) {
                return Err("Duplicate primary key: entity_reference_id");
            }

            by_person_id
                .entry(item.person_id)
                .or_insert_with(Vec::new)
                .push(primary_key);
            
            by_id.insert(primary_key, item);
        }

        Ok(Arc::new(EntityReferenceIdxModelCache {
            by_id,
            by_person_id,
        }))
    }

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        self.by_id.contains_key(primary_key)
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<EntityReferenceIdxModel> {
        self.by_id.get(primary_key).cloned()
    }

    pub fn get_by_person_id(&self, key: &Uuid) -> Option<&Vec<Uuid>> {
        self.by_person_id.get(key)
    }
}


/// # Cache
/// - Mutable Set of Mutable Records
///     - Application Scope + Notification Table (new records)
/// - Primary 
///     - person_id
/// - Non Unique Optional Lookup Fields
///     - external_identifier_hash
/// # Documentation
/// - Index model for Person
#[derive(Debug, Clone, FromRow)]
pub struct PersonIdxModel {
    pub person_id: Uuid,
    /// # Nature
    /// - Mutable
    pub external_identifier_hash: Option<i64>,
}

pub struct PersonIdxModelCache {
    by_id: HashMap<Uuid, PersonIdxModel>,
    by_external_identifier_hash: HashMap<i64, Vec<Uuid>>,
}

impl PersonIdxModelCache {
    pub fn new(
        items: Vec<PersonIdxModel>,
    ) -> Result<Arc<Self>, &'static str> {
        let mut by_id = HashMap::new();
        let mut by_external_identifier_hash = HashMap::new();

        for item in items {
            let primary_key = item.person_id;
            if by_id.contains_key(&primary_key) {
                return Err("Duplicate primary key: person_id");
            }

            if let Some(hash) = item.external_identifier_hash {
                by_external_identifier_hash
                    .entry(hash)
                    .or_insert_with(Vec::new)
                    .push(primary_key);
            }
            
            by_id.insert(primary_key, item);
        }

        Ok(Arc::new(PersonIdxModelCache {
            by_id,
            by_external_identifier_hash,
        }))
    }

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        self.by_id.contains_key(primary_key)
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<PersonIdxModel> {
        self.by_id.get(primary_key).cloned()
    }

    pub fn get_by_external_identifier_hash(&self, key: &i64) -> Option<&Vec<Uuid>> {
        self.by_external_identifier_hash.get(key)
    }
}

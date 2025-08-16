use chrono::{DateTime, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Database model for address type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "address_type", rename_all = "PascalCase")]
pub enum AddressType {
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

/// Database model for Country
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CountryModel {
    pub id: Uuid,
    pub iso2: HeaplessString<2>,
    pub name_l1: HeaplessString<100>,
    pub name_l2: Option<HeaplessString<100>>,
    pub name_l3: Option<HeaplessString<100>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by_person_id: Uuid,
    pub updated_by_person_id: Uuid,
}

/// Database model for StateProvince
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StateProvinceModel {
    pub id: Uuid,
    pub country_id: Uuid,
    pub name_l1: HeaplessString<100>,
    pub name_l2: Option<HeaplessString<100>>,
    pub name_l3: Option<HeaplessString<100>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by_person_id: Uuid,
    pub updated_by_person_id: Uuid,
}

/// Database model for City
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CityModel {
    pub id: Uuid,
    pub country_id: Uuid,
    pub state_id: Option<Uuid>,
    pub name_l1: HeaplessString<100>,
    pub name_l2: Option<HeaplessString<100>>,
    pub name_l3: Option<HeaplessString<100>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by_person_id: Uuid,
    pub updated_by_person_id: Uuid,
}

/// Database model for Address
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AddressModel {
    pub id: Uuid,
    pub street_line1: HeaplessString<50>,
    pub street_line2: HeaplessString<50>,
    pub street_line3: HeaplessString<50>,
    pub street_line4: HeaplessString<50>,
    pub city_id: Option<Uuid>,
    pub postal_code: Option<HeaplessString<20>>,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub accuracy_meters: Option<f32>,
    #[serde(serialize_with = "serialize_address_type", deserialize_with = "deserialize_address_type")]
    pub address_type: AddressType,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by_person_id: Uuid,
    pub updated_by_person_id: Uuid,
}

/// Database model for Messaging
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MessagingModel {
    pub id: Uuid,
    #[serde(serialize_with = "serialize_messaging_type", deserialize_with = "deserialize_messaging_type")]
    pub messaging_type: MessagingType,
    pub value: HeaplessString<100>,
    pub other_type: Option<HeaplessString<20>>,
    pub is_active: bool,
    pub priority: Option<u8>, // Changed from i16 to u8 to match domain model
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for EntityReference
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EntityReferenceModel {
    pub id: Uuid,
    pub person_id: Uuid,
    #[serde(serialize_with = "serialize_person_entity_type", deserialize_with = "deserialize_person_entity_type")]
    pub entity_role: RelationshipRole,
    pub reference_external_id: Option<HeaplessString<50>>,
    pub reference_details_l1: Option<HeaplessString<50>>,
    pub reference_details_l2: Option<HeaplessString<50>>,
    pub reference_details_l3: Option<HeaplessString<50>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by_person_id: Uuid,
    pub updated_by_person_id: Uuid,
}

/// Database model for person
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PersonModel {
    pub id: Uuid,
    
    #[serde(serialize_with = "serialize_person_type", deserialize_with = "deserialize_person_type")]
    pub person_type: PersonType,
    
    pub display_name: HeaplessString<100>,
    pub external_identifier: Option<HeaplessString<50>>,
    /// References PersonModel.person_id for organizational hierarchy
    pub organization_person_id: Option<Uuid>,
    
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
    
    pub department: Option<HeaplessString<50>>,
    /// References AddressModel.address_id for person's location
    pub location_address_id: Option<Uuid>,
    
    pub duplicate_of_person_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

// Serialization functions for AddressType
fn serialize_address_type<S>(address_type: &AddressType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match address_type {
        AddressType::Residential => "residential",
        AddressType::Business => "business",
        AddressType::Mailing => "mailing",
        AddressType::Temporary => "temporary",
        AddressType::Branch => "branch",
        AddressType::Community => "community",
        AddressType::Other => "other",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_address_type<'de, D>(deserializer: D) -> Result<AddressType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "residential" => Ok(AddressType::Residential),
        "business" => Ok(AddressType::Business),
        "mailing" => Ok(AddressType::Mailing),
        "temporary" => Ok(AddressType::Temporary),
        "branch" => Ok(AddressType::Branch),
        "community" => Ok(AddressType::Community),
        "other" => Ok(AddressType::Other),
        _ => Err(serde::de::Error::custom(format!("Unknown address type: {s}"))),
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


/// Index model for Country
#[derive(Debug, Clone, FromRow)]
pub struct CountryIdxModel {
    pub country_id: Uuid,
    pub iso2: HeaplessString<2>,
    pub is_active: bool,
}

/// Index model for StateProvince
#[derive(Debug, Clone, FromRow)]
pub struct StateProvinceIdxModel {
    pub state_province_id: Uuid,
    pub country_id: Uuid,
    pub is_active: bool,
}

/// Index model for City
#[derive(Debug, Clone, FromRow)]
pub struct CityIdxModel {
    pub city_id: Uuid,
    pub country_id: Uuid,
    pub state_id: Option<Uuid>,
    pub is_active: bool,
}

/// Index model for Address
#[derive(Debug, Clone, FromRow)]
pub struct AddressIdxModel {
    pub address_id: Uuid,
    pub address_type: AddressType,
    pub is_active: bool,
    pub city_id: Option<Uuid>,
}

/// Index model for Messaging
#[derive(Debug, Clone, FromRow)]
pub struct MessagingIdxModel {
    pub messaging_id: Uuid,
    pub messaging_type: MessagingType,
    pub is_active: bool,
}

/// Index model for EntityReference
#[derive(Debug, Clone, FromRow)]
pub struct EntityReferenceIdxModel {
    pub entity_reference_id: Uuid,
    pub person_id: Uuid,
    pub entity_role: RelationshipRole,
    pub is_active: bool,
}

/// Index model for Person
#[derive(Debug, Clone, FromRow)]
pub struct PersonIdxModel {
    pub person_id: Uuid,
    pub person_type: PersonType,
    pub is_active: bool,
}

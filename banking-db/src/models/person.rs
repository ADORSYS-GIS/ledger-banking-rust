use chrono::{DateTime, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Database model for address type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "address_type", rename_all = "lowercase")]
pub enum AddressType {
    Residential,
    Business,
    Mailing,
    Temporary,
    Branch,
    Other,
}

/// Database model for messaging type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "messaging_type", rename_all = "lowercase")]
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
#[sqlx(type_name = "person_type", rename_all = "lowercase")]
pub enum PersonType {
    Natural,
    Legal,
    System,
    Integration,
    Unknown,
}

/// Database model for Country
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CountryModel {
    pub country_id: Uuid,
    pub iso2: HeaplessString<2>,
    pub name_l1: HeaplessString<100>,
    pub name_l2: Option<HeaplessString<100>>,
    pub name_l3: Option<HeaplessString<100>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

/// Database model for StateProvince
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StateProvinceModel {
    pub state_id: Uuid,
    pub country_id: Uuid,
    pub name_l1: HeaplessString<100>,
    pub name_l2: Option<HeaplessString<100>>,
    pub name_l3: Option<HeaplessString<100>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

/// Database model for City
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CityModel {
    pub city_id: Uuid,
    pub country_id: Uuid,
    pub state_id: Option<Uuid>,
    pub name_l1: HeaplessString<100>,
    pub name_l2: Option<HeaplessString<100>>,
    pub name_l3: Option<HeaplessString<100>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

/// Database model for Address
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AddressModel {
    pub address_id: Uuid,
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
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

/// Database model for Messaging
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MessagingModel {
    pub messaging_id: Uuid,
    #[serde(serialize_with = "serialize_messaging_type", deserialize_with = "deserialize_messaging_type")]
    pub messaging_type: MessagingType,
    pub value: HeaplessString<100>,
    pub other_type: Option<HeaplessString<20>>,
    pub is_active: bool,
    pub priority: Option<i16>, // Database uses i16 for small integers
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for person
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PersonModel {
    pub person_id: Uuid,
    
    #[serde(serialize_with = "serialize_person_type", deserialize_with = "deserialize_person_type")]
    pub person_type: PersonType,
    
    pub display_name: HeaplessString<100>,
    pub external_identifier: Option<HeaplessString<50>>,
    /// References PersonModel.person_id for organizational hierarchy
    pub organization: Option<Uuid>,
    
    /// References to MessagingModel.messaging_id (stored as JSON array in database)
    pub messaging: serde_json::Value, // Will store Vec<Uuid> as JSON
    
    pub department: Option<HeaplessString<50>>,
    /// References AddressModel.address_id for person's location
    pub location: Option<Uuid>,
    
    pub duplicate_of: Option<Uuid>,
    pub entity_reference: Option<Uuid>,
    pub entity_type: Option<HeaplessString<50>>,
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
        PersonType::Natural => "natural",
        PersonType::Legal => "legal",
        PersonType::System => "system",
        PersonType::Integration => "integration",
        PersonType::Unknown => "unknown",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_person_type<'de, D>(deserializer: D) -> Result<PersonType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "natural" => Ok(PersonType::Natural),
        "legal" => Ok(PersonType::Legal),
        "system" => Ok(PersonType::System),
        "integration" => Ok(PersonType::Integration),
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


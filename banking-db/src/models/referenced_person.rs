use chrono::{DateTime, Utc};
use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use banking_api::domain;

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

/// Database model for contact information
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ContactInfoModel {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub department: Option<String>,
    pub office_location: Option<String>,
}

/// Database model for referenced person
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ReferencedPersonModel {
    pub person_id: Uuid,
    
    #[serde(serialize_with = "crate::utils::serialize_person_type")]
    #[serde(deserialize_with = "crate::utils::deserialize_person_type")]
    pub person_type: PersonType,
    
    pub display_name: String,
    pub external_identifier: Option<String>,
    pub organization: Option<String>,
    
    // Store contact info as JSON
    pub contact_info: Option<sqlx::types::Json<ContactInfoModel>>,
    
    pub duplicate_of: Option<Uuid>,
    pub entity_reference: Option<Uuid>,
    pub entity_type: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Conversion implementations
impl From<ContactInfoModel> for domain::ContactInfo {
    fn from(model: ContactInfoModel) -> Self {
        Self {
            email: model.email.and_then(|s| HeaplessString::try_from(s.as_str()).ok()),
            phone: model.phone.and_then(|s| HeaplessString::try_from(s.as_str()).ok()),
            department: model.department.and_then(|s| HeaplessString::try_from(s.as_str()).ok()),
            office_location: model.office_location.and_then(|s| HeaplessString::try_from(s.as_str()).ok()),
        }
    }
}

impl From<domain::ContactInfo> for ContactInfoModel {
    fn from(contact: domain::ContactInfo) -> Self {
        Self {
            email: contact.email.map(|s| s.to_string()),
            phone: contact.phone.map(|s| s.to_string()),
            department: contact.department.map(|s| s.to_string()),
            office_location: contact.office_location.map(|s| s.to_string()),
        }
    }
}

impl From<ReferencedPersonModel> for domain::ReferencedPerson {
    fn from(model: ReferencedPersonModel) -> Self {
        Self {
            person_id: model.person_id,
            person_type: model.person_type.into(),
            display_name: HeaplessString::try_from(model.display_name.as_str()).unwrap_or_default(),
            external_identifier: model.external_identifier.and_then(|s| HeaplessString::try_from(s.as_str()).ok()),
            organization: model.organization.and_then(|s| HeaplessString::try_from(s.as_str()).ok()),
            contact_info: model.contact_info.map(|json| json.0.into()),
            duplicate_of: model.duplicate_of,
            entity_reference: model.entity_reference,
            entity_type: model.entity_type.and_then(|s| HeaplessString::try_from(s.as_str()).ok()),
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<domain::ReferencedPerson> for ReferencedPersonModel {
    fn from(person: domain::ReferencedPerson) -> Self {
        Self {
            person_id: person.person_id,
            person_type: person.person_type.into(),
            display_name: person.display_name.to_string(),
            external_identifier: person.external_identifier.map(|s| s.to_string()),
            organization: person.organization.map(|s| s.to_string()),
            contact_info: person.contact_info.map(|c| sqlx::types::Json(c.into())),
            duplicate_of: person.duplicate_of,
            entity_reference: person.entity_reference,
            entity_type: person.entity_type.map(|s| s.to_string()),
            is_active: person.is_active,
            created_at: person.created_at,
            updated_at: person.updated_at,
        }
    }
}

// PersonType conversions
impl From<PersonType> for domain::PersonType {
    fn from(model: PersonType) -> Self {
        match model {
            PersonType::Natural => domain::PersonType::Natural,
            PersonType::Legal => domain::PersonType::Legal,
            PersonType::System => domain::PersonType::System,
            PersonType::Integration => domain::PersonType::Integration,
            PersonType::Unknown => domain::PersonType::Unknown,
        }
    }
}

impl From<domain::PersonType> for PersonType {
    fn from(person_type: domain::PersonType) -> Self {
        match person_type {
            domain::PersonType::Natural => PersonType::Natural,
            domain::PersonType::Legal => PersonType::Legal,
            domain::PersonType::System => PersonType::System,
            domain::PersonType::Integration => PersonType::Integration,
            domain::PersonType::Unknown => PersonType::Unknown,
        }
    }
}
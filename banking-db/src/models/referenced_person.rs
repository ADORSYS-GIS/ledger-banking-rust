use chrono::{DateTime, Utc};
use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

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

/// Database model for referenced person
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ReferencedPersonModel {
    pub person_id: Uuid,
    
    #[serde(serialize_with = "crate::utils::serialize_person_type")]
    #[serde(deserialize_with = "crate::utils::deserialize_person_type")]
    pub person_type: PersonType,
    
    pub display_name: HeaplessString<100>,
    pub external_identifier: Option<HeaplessString<50>>,
    pub organization: Option<HeaplessString<100>>,
    
    // Contact information fields
    pub email: Option<HeaplessString<100>>,
    pub phone: Option<HeaplessString<20>>,
    pub department: Option<HeaplessString<50>>,
    pub office_location: Option<HeaplessString<100>>,
    
    pub duplicate_of: Option<Uuid>,
    pub entity_reference: Option<Uuid>,
    pub entity_type: Option<HeaplessString<50>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


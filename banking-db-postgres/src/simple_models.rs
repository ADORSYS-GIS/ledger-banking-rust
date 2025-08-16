use banking_db::models::person::{MessagingType, PersonType};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct PersonModelSqlx {
    pub id: Uuid,
    pub person_type: PersonType,
    pub display_name: String,
    pub external_identifier: Option<String>,
    pub organization_person_id: Option<Uuid>,
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
    pub department: Option<String>,
    pub location_address_id: Option<Uuid>,
    pub duplicate_of_person_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
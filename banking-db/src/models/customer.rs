use chrono::{DateTime, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Database model for Customer table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct CustomerModel {
    pub customer_id: Uuid,
    pub customer_type: String,
    pub full_name: HeaplessString<255>,
    pub id_type: String,
    pub id_number: HeaplessString<50>,
    pub risk_rating: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    /// References ReferencedPerson.person_id
    pub updated_by: Uuid,
}

/// Database model for Customer Portfolio summary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct CustomerPortfolioModel {
    pub customer_id: Uuid,
    pub total_accounts: i64,
    pub total_balance: Decimal,
    pub last_activity_date: Option<DateTime<Utc>>,
    pub risk_score: Option<Decimal>,
    pub kyc_status: String,
    pub sanctions_checked: bool,
    pub last_screening_date: Option<DateTime<Utc>>,
}

/// Database model for Customer documents
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct CustomerDocumentModel {
    pub document_id: Uuid,
    pub customer_id: Uuid,
    pub document_type: String,
    pub document_path: Option<String>,
    pub status: String,
    pub uploaded_at: DateTime<Utc>,
    /// References ReferencedPerson.person_id
    pub uploaded_by: Uuid,
    pub verified_at: Option<DateTime<Utc>>,
    /// References ReferencedPerson.person_id
    pub verified_by: Option<Uuid>,
}

/// Database model for Customer audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct CustomerAuditModel {
    pub audit_id: Uuid,
    pub customer_id: Uuid,
    pub field_name: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub changed_at: DateTime<Utc>,
    /// References ReferencedPerson.person_id
    pub changed_by: Uuid,
    pub reason: Option<String>,
}
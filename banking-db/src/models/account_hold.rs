use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use heapless::String as HeaplessString;
pub use banking_api::domain::{
    HoldType, HoldStatus, HoldPriority
};

/// Database model for Account Holds
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountHoldModel {
    pub id: Uuid,
    pub account_id: Uuid,
    pub amount: Decimal,
    pub hold_type: HoldType,
    /// References ReasonAndPurpose.id
    pub reason_id: Uuid,
    /// Additional context beyond the standard reason
    pub additional_details: Option<HeaplessString<200>>,
    /// References Person.person_id
    pub placed_by_person_id: Uuid,
    pub placed_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: HoldStatus,
    pub released_at: Option<DateTime<Utc>>,
    /// References Person.person_id
    pub released_by_person_id: Option<Uuid>,
    pub priority: HoldPriority,
    pub source_reference: Option<HeaplessString<100>>,
    pub automatic_release: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for Account Hold Summary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountHoldSummaryModel {
    pub id: Uuid,
    pub account_balance_calculation_id: Uuid,
    pub hold_type: HoldType,
    pub total_amount: Decimal,
    pub hold_count: u32,
    pub priority: HoldPriority,
}

/// Database model for Hold Release Request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountHoldReleaseRequestModel {
    pub id: Uuid,
    pub hold_id: Uuid,
    pub release_amount: Option<Decimal>, // For partial releases
    /// References ReasonAndPurpose.id for release
    pub release_reason_id: Uuid,
    /// Additional context for release
    pub release_additional_details: Option<HeaplessString<200>>,
    /// References Person.person_id
    pub released_by_person_id: Uuid,
    pub override_authorization: bool,
}

/// Database model for Account Hold Expiry Job
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountHoldExpiryJobModel {
    pub id: Uuid,
    pub processing_date: NaiveDate,
    pub expired_holds_count: u32,
    pub total_released_amount: Decimal,
    pub processed_at: DateTime<Utc>,
    pub errors: Vec<HeaplessString<100>>,
}

/// Database model for Place Hold Request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct PlaceHoldRequestModel {
    pub id: Uuid,
    pub account_id: Uuid,
    pub hold_type: HoldType,
    pub amount: Decimal,
    /// References ReasonAndPurpose.id - required field
    pub reason_id: Uuid,
    /// Additional context beyond the standard reason
    pub additional_details: Option<HeaplessString<200>>,
    /// References Person.person_id
    pub placed_by_person_id: Uuid,
    pub expires_at: Option<DateTime<Utc>>,
    pub priority: HoldPriority,
    pub source_reference: Option<HeaplessString<100>>,
}
/// Database model for Account Balance Calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountBalanceCalculationModel {
    pub id: Uuid,
    pub account_id: Uuid,
    pub current_balance: Decimal,
    pub available_balance: Decimal,
    pub overdraft_limit: Option<Decimal>,
    pub total_holds: Decimal,
    pub active_hold_count: i32,
    pub calculation_timestamp: DateTime<Utc>,
}

/// Database model for Hold Release Record
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct HoldReleaseRecordModel {
    pub id: Uuid,
    pub hold_id: Uuid,
    pub release_amount: Decimal,
    pub release_reason_id: Uuid,
    pub released_by_person_id: Uuid,
    pub released_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct HoldPrioritySummary {
    pub priority: String,
    pub total_amount: Decimal,
    pub hold_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct HoldOverrideRecord {
    pub id: Uuid,
    pub account_id: Uuid,
    pub overridden_holds: Vec<Uuid>,
    pub override_amount: Decimal,
    pub authorized_by: Uuid,
    pub override_reason_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct HoldAnalyticsSummary {
    pub total_holds_placed: i64,
    pub total_amount_placed: Decimal,
    pub total_holds_released: i64,
    pub total_amount_released: Decimal,
    pub average_hold_duration_seconds: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct HighHoldRatioAccount {
    pub account_id: Uuid,
    pub customer_id: Uuid,
    pub current_balance: Decimal,
    pub total_holds: Decimal,
    pub hold_ratio: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct JudicialHoldReportData {
    pub hold_id: Uuid,
    pub account_id: Uuid,
    pub customer_name: String,
    pub amount: Decimal,
    pub court_reference: String,
    pub placed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct HoldAgingBucket {
    pub bucket: String, // e.g., "0-30 days"
    pub hold_count: i64,
    pub total_amount: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct HoldValidationError {
    pub hold_id: Uuid,
    pub error_type: String,
    pub details: String,
}
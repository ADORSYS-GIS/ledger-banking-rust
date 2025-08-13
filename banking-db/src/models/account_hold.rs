use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use heapless::String as HeaplessString;
use std::str::FromStr;

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
    pub errors_01: Option<HeaplessString<100>>,
    pub errors_02: Option<HeaplessString<100>>,
    pub errors_03: Option<HeaplessString<100>>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "hold_type", rename_all = "PascalCase")]
pub enum HoldType {
    UnclearedFunds,
    JudicialLien,
    LoanPledge,
    ComplianceHold,
    AdministrativeHold,
    FraudHold,
    PendingAuthorization,
    OverdraftReserve,
    CardAuthorization,
    Other,
}

impl FromStr for HoldType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UnclearedFunds" => Ok(HoldType::UnclearedFunds),
            "JudicialLien" => Ok(HoldType::JudicialLien),
            "LoanPledge" => Ok(HoldType::LoanPledge),
            "ComplianceHold" => Ok(HoldType::ComplianceHold),
            "AdministrativeHold" => Ok(HoldType::AdministrativeHold),
            "FraudHold" => Ok(HoldType::FraudHold),
            "PendingAuthorization" => Ok(HoldType::PendingAuthorization),
            "OverdraftReserve" => Ok(HoldType::OverdraftReserve),
            "CardAuthorization" => Ok(HoldType::CardAuthorization),
            "Other" => Ok(HoldType::Other),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "hold_status", rename_all = "PascalCase")]
pub enum HoldStatus {
    Active,
    Released,
    Expired,
    Cancelled,
    PartiallyReleased,
}

impl FromStr for HoldStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(HoldStatus::Active),
            "Released" => Ok(HoldStatus::Released),
            "Expired" => Ok(HoldStatus::Expired),
            "Cancelled" => Ok(HoldStatus::Cancelled),
            "PartiallyReleased" => Ok(HoldStatus::PartiallyReleased),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "hold_priority", rename_all = "PascalCase")]
pub enum HoldPriority {
    Critical,
    High,
    Standard,
    Medium,
    Low,
}

impl FromStr for HoldPriority {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Critical" => Ok(HoldPriority::Critical),
            "High" => Ok(HoldPriority::High),
            "Standard" => Ok(HoldPriority::Standard),
            "Medium" => Ok(HoldPriority::Medium),
            "Low" => Ok(HoldPriority::Low),
            _ => Err(()),
        }
    }
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
use banking_api::domain::account_hold::PlaceHoldRequest;

impl From<(PlaceHoldRequest, uuid::Uuid)> for AccountHoldModel {
    fn from((request, id): (PlaceHoldRequest, uuid::Uuid)) -> Self {
        AccountHoldModel {
            id,
            account_id: request.account_id,
            amount: request.amount,
            hold_type: request.hold_type.into(),
            reason_id: request.reason_id,
            additional_details: request.additional_details,
            placed_by_person_id: request.placed_by_person_id,
            placed_at: chrono::Utc::now(),
            expires_at: request.expires_at,
            status: banking_api::domain::account_hold::HoldStatus::Active.into(),
            released_at: None,
            released_by_person_id: None,
            priority: request.priority.into(),
            source_reference: request.source_reference,
            automatic_release: request.expires_at.is_some(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}
use banking_api::domain::account_hold::{HoldType as ApiHoldType, HoldStatus as ApiHoldStatus, HoldPriority as ApiHoldPriority};

// From implementations for HoldType
impl From<ApiHoldType> for HoldType {
    fn from(api_type: ApiHoldType) -> Self {
        match api_type {
            ApiHoldType::UnclearedFunds => HoldType::UnclearedFunds,
            ApiHoldType::JudicialLien => HoldType::JudicialLien,
            ApiHoldType::LoanPledge => HoldType::LoanPledge,
            ApiHoldType::ComplianceHold => HoldType::ComplianceHold,
            ApiHoldType::AdministrativeHold => HoldType::AdministrativeHold,
            ApiHoldType::FraudHold => HoldType::FraudHold,
            ApiHoldType::PendingAuthorization => HoldType::PendingAuthorization,
            ApiHoldType::OverdraftReserve => HoldType::OverdraftReserve,
            ApiHoldType::CardAuthorization => HoldType::CardAuthorization,
            ApiHoldType::Other => HoldType::Other,
        }
    }
}

impl From<HoldType> for ApiHoldType {
    fn from(db_type: HoldType) -> Self {
        match db_type {
            HoldType::UnclearedFunds => ApiHoldType::UnclearedFunds,
            HoldType::JudicialLien => ApiHoldType::JudicialLien,
            HoldType::LoanPledge => ApiHoldType::LoanPledge,
            HoldType::ComplianceHold => ApiHoldType::ComplianceHold,
            HoldType::AdministrativeHold => ApiHoldType::AdministrativeHold,
            HoldType::FraudHold => ApiHoldType::FraudHold,
            HoldType::PendingAuthorization => ApiHoldType::PendingAuthorization,
            HoldType::OverdraftReserve => ApiHoldType::OverdraftReserve,
            HoldType::CardAuthorization => ApiHoldType::CardAuthorization,
            HoldType::Other => ApiHoldType::Other,
        }
    }
}

// From implementations for HoldStatus
impl From<ApiHoldStatus> for HoldStatus {
    fn from(api_status: ApiHoldStatus) -> Self {
        match api_status {
            ApiHoldStatus::Active => HoldStatus::Active,
            ApiHoldStatus::Released => HoldStatus::Released,
            ApiHoldStatus::Expired => HoldStatus::Expired,
            ApiHoldStatus::Cancelled => HoldStatus::Cancelled,
            ApiHoldStatus::PartiallyReleased => HoldStatus::PartiallyReleased,
        }
    }
}

impl From<HoldStatus> for ApiHoldStatus {
    fn from(db_status: HoldStatus) -> Self {
        match db_status {
            HoldStatus::Active => ApiHoldStatus::Active,
            HoldStatus::Released => ApiHoldStatus::Released,
            HoldStatus::Expired => ApiHoldStatus::Expired,
            HoldStatus::Cancelled => ApiHoldStatus::Cancelled,
            HoldStatus::PartiallyReleased => ApiHoldStatus::PartiallyReleased,
        }
    }
}

// From implementations for HoldPriority
impl From<ApiHoldPriority> for HoldPriority {
    fn from(api_priority: ApiHoldPriority) -> Self {
        match api_priority {
            ApiHoldPriority::Critical => HoldPriority::Critical,
            ApiHoldPriority::High => HoldPriority::High,
            ApiHoldPriority::Standard => HoldPriority::Standard,
            ApiHoldPriority::Medium => HoldPriority::Medium,
            ApiHoldPriority::Low => HoldPriority::Low,
        }
    }
}

impl From<HoldPriority> for ApiHoldPriority {
    fn from(db_priority: HoldPriority) -> Self {
        match db_priority {
            HoldPriority::Critical => ApiHoldPriority::Critical,
            HoldPriority::High => ApiHoldPriority::High,
            HoldPriority::Standard => ApiHoldPriority::Standard,
            HoldPriority::Medium => ApiHoldPriority::Medium,
            HoldPriority::Low => ApiHoldPriority::Low,
        }
    }
}
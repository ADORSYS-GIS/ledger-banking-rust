use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;
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
    #[serde(
        serialize_with = "serialize_hold_type",
        deserialize_with = "deserialize_hold_type"
    )]
    pub hold_type: HoldType,
    /// References ReasonAndPurpose.id
    pub reason_id: Uuid,
    /// Additional context beyond the standard reason
    pub additional_details: Option<HeaplessString<200>>,
    /// References Person.person_id
    pub placed_by_person_id: Uuid,
    pub placed_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(
        serialize_with = "serialize_hold_status",
        deserialize_with = "deserialize_hold_status"
    )]
    pub status: HoldStatus,
    pub released_at: Option<DateTime<Utc>>,
    /// References Person.person_id
    pub released_by_person_id: Option<Uuid>,
    #[serde(
        serialize_with = "serialize_hold_priority",
        deserialize_with = "deserialize_hold_priority"
    )]
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
    #[serde(
        serialize_with = "serialize_hold_type",
        deserialize_with = "deserialize_hold_type"
    )]
    pub hold_type: HoldType,
    pub total_amount: Decimal,
    pub hold_count: i32,
    #[serde(
        serialize_with = "serialize_hold_priority",
        deserialize_with = "deserialize_hold_priority"
    )]
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
    pub expired_holds_count: i32,
    pub total_released_amount: Decimal,
    pub processed_at: DateTime<Utc>,
    pub errors: Option<Vec<String>>,
}

/// Database model for Place Hold Request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct PlaceHoldRequestModel {
    pub id: Uuid,
    pub account_id: Uuid,
    #[serde(
        serialize_with = "serialize_hold_type",
        deserialize_with = "deserialize_hold_type"
    )]
    pub hold_type: HoldType,
    pub amount: Decimal,
    /// References ReasonAndPurpose.id - required field
    pub reason_id: Uuid,
    /// Additional context beyond the standard reason
    pub additional_details: Option<HeaplessString<200>>,
    /// References Person.person_id
    pub placed_by_person_id: Uuid,
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(
        serialize_with = "serialize_hold_priority",
        deserialize_with = "deserialize_hold_priority"
    )]
    pub priority: HoldPriority,
    pub source_reference: Option<HeaplessString<100>>,
}

fn serialize_hold_type<S>(hold_type: &HoldType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let type_str = match hold_type {
        HoldType::UnclearedFunds => "UnclearedFunds",
        HoldType::JudicialLien => "JudicialLien",
        HoldType::LoanPledge => "LoanPledge",
        HoldType::ComplianceHold => "ComplianceHold",
        HoldType::AdministrativeHold => "AdministrativeHold",
        HoldType::FraudHold => "FraudHold",
        HoldType::PendingAuthorization => "PendingAuthorization",
        HoldType::OverdraftReserve => "OverdraftReserve",
        HoldType::CardAuthorization => "CardAuthorization",
        HoldType::Other => "Other",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_hold_type<'de, D>(deserializer: D) -> Result<HoldType, D::Error>
where
    D: Deserializer<'de>,
{
    let type_str = String::deserialize(deserializer)?;
    match type_str.as_str() {
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
        _ => Err(serde::de::Error::custom(format!("Invalid hold type: {type_str}"))),
    }
}

fn serialize_hold_status<S>(status: &HoldStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let status_str = match status {
        HoldStatus::Active => "Active",
        HoldStatus::Released => "Released",
        HoldStatus::Expired => "Expired",
        HoldStatus::Cancelled => "Cancelled",
        HoldStatus::PartiallyReleased => "PartiallyReleased",
    };
    serializer.serialize_str(status_str)
}

fn deserialize_hold_status<'de, D>(deserializer: D) -> Result<HoldStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let status_str = String::deserialize(deserializer)?;
    match status_str.as_str() {
        "Active" => Ok(HoldStatus::Active),
        "Released" => Ok(HoldStatus::Released),
        "Expired" => Ok(HoldStatus::Expired),
        "Cancelled" => Ok(HoldStatus::Cancelled),
        "PartiallyReleased" => Ok(HoldStatus::PartiallyReleased),
        _ => Err(serde::de::Error::custom(format!("Invalid hold status: {status_str}"))),
    }
}

fn serialize_hold_priority<S>(priority: &HoldPriority, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let priority_str = match priority {
        HoldPriority::Critical => "Critical",
        HoldPriority::High => "High",
        HoldPriority::Standard => "Standard",
        HoldPriority::Medium => "Medium",
        HoldPriority::Low => "Low",
    };
    serializer.serialize_str(priority_str)
}

fn deserialize_hold_priority<'de, D>(deserializer: D) -> Result<HoldPriority, D::Error>
where
    D: Deserializer<'de>,
{
    let priority_str = String::deserialize(deserializer)?;
    match priority_str.as_str() {
        "Critical" => Ok(HoldPriority::Critical),
        "High" => Ok(HoldPriority::High),
        "Standard" => Ok(HoldPriority::Standard),
        "Medium" => Ok(HoldPriority::Medium),
        "Low" => Ok(HoldPriority::Low),
        _ => Err(serde::de::Error::custom(format!("Invalid hold priority: {priority_str}"))),
    }
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
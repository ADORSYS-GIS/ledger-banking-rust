use chrono::{DateTime, NaiveDate, Utc};
use heapless::{String as HeaplessString};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountHold {
    pub id: Uuid,
    pub account_id: Uuid,
    pub amount: Decimal,
    pub hold_type: HoldType,
    /// References ReasonAndPurpose.id - required field
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
    pub source_reference: Option<HeaplessString<100>>, // External reference for judicial holds, etc.
    pub automatic_release: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HoldType {
    /// Funds pending clearance
    UnclearedFunds,
    /// Court-ordered judicial lien
    JudicialLien,
    /// Loan collateral pledge
    LoanPledge,
    /// Regulatory compliance hold
    ComplianceHold,
    /// Administrative hold by bank staff
    AdministrativeHold,
    /// Fraud investigation hold
    FraudHold,
    /// Pending transaction authorization
    PendingAuthorization,
    /// Overdraft protection reserve
    OverdraftReserve,
    /// Card authorization hold
    CardAuthorization,
    /// Other types
    Other,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HoldStatus {
    Active,
    Released,
    Expired,
    Cancelled,
    PartiallyReleased,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HoldPriority {
    /// Must be honored first (judicial, regulatory)
    Critical,
    /// Standard business hold
    High,
    /// Standard priority hold
    Standard,
    /// Lower priority administrative hold
    Medium,
    /// Lowest priority, can be overridden
    Low,
}

/// Hold release request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountHoldReleaseRequest {
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

/// Batch hold processing for expired holds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountHoldExpiryJob {
    pub id: Uuid,
    pub processing_date: NaiveDate,
    pub expired_holds_count: u32,
    pub total_released_amount: Decimal,
    pub processed_at: DateTime<Utc>,
    pub errors: Vec<HeaplessString<100>>,
}

/// Request parameters for placing a hold on an account
#[derive(Debug, Clone)]
pub struct PlaceHoldRequest {
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

impl std::fmt::Display for HoldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HoldType::UnclearedFunds => write!(f, "UnclearedFunds"),
            HoldType::JudicialLien => write!(f, "JudicialLien"),
            HoldType::LoanPledge => write!(f, "LoanPledge"),
            HoldType::ComplianceHold => write!(f, "ComplianceHold"),
            HoldType::AdministrativeHold => write!(f, "AdministrativeHold"),
            HoldType::FraudHold => write!(f, "FraudHold"),
            HoldType::PendingAuthorization => write!(f, "PendingAuthorization"),
            HoldType::OverdraftReserve => write!(f, "OverdraftReserve"),
            HoldType::CardAuthorization => write!(f, "CardAuthorization"),
            HoldType::Other => write!(f, "Other"),
        }
    }
}

impl std::fmt::Display for HoldStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HoldStatus::Active => write!(f, "Active"),
            HoldStatus::Released => write!(f, "Released"),
            HoldStatus::Expired => write!(f, "Expired"),
            HoldStatus::Cancelled => write!(f, "Cancelled"),
            HoldStatus::PartiallyReleased => write!(f, "PartiallyReleased"),
        }
    }
}

impl std::fmt::Display for HoldPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HoldPriority::Critical => write!(f, "Critical"),
            HoldPriority::High => write!(f, "High"),
            HoldPriority::Standard => write!(f, "Standard"),
            HoldPriority::Medium => write!(f, "Medium"),
            HoldPriority::Low => write!(f, "Low"),
        }
    }
}
/// Real-time balance calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalanceCalculation {
    pub id: Uuid,
    pub account_id: Uuid,
    pub current_balance: Decimal,
    pub available_balance: Decimal,
    pub overdraft_limit: Option<Decimal>,
    pub total_holds: Decimal,
    pub active_hold_count: u32,
    pub calculation_timestamp: DateTime<Utc>,
}

/// Summary of hold amounts by type for balance calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountHoldSummary {
    pub id: Uuid,
    pub account_balance_calculation_id: Uuid,
    pub hold_type: HoldType,
    pub total_amount: Decimal,
    pub hold_count: u32,
    pub priority: HoldPriority,
}
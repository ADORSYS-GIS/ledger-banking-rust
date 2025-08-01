use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

// Import enums from domain layer
use banking_api::domain::fee::{
    FeeType, FeeCategory, FeeCalculationMethod, FeeTriggerEvent, 
    FeeApplicationStatus, FeeJobType, FeeJobStatus
};

/// Database model for fee applications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeApplicationModel {
    pub fee_application_id: Uuid,
    pub account_id: Uuid,
    pub transaction_id: Option<Uuid>,
    #[serde(serialize_with = "serialize_fee_type", deserialize_with = "deserialize_fee_type")]
    pub fee_type: FeeType,
    #[serde(serialize_with = "serialize_fee_category", deserialize_with = "deserialize_fee_category")]
    pub fee_category: FeeCategory,
    pub product_code: HeaplessString<12>,
    pub fee_code: HeaplessString<12>,
    pub description: HeaplessString<200>,
    pub amount: Decimal,
    pub currency: HeaplessString<3>,
    #[serde(serialize_with = "serialize_calculation_method", deserialize_with = "deserialize_calculation_method")]
    pub calculation_method: FeeCalculationMethod,
    pub calculation_base_amount: Option<Decimal>,
    pub fee_rate: Option<Decimal>,
    #[serde(serialize_with = "serialize_trigger_event", deserialize_with = "deserialize_trigger_event")]
    pub trigger_event: FeeTriggerEvent,
    #[serde(serialize_with = "serialize_application_status", deserialize_with = "deserialize_application_status")]
    pub status: FeeApplicationStatus,
    pub applied_at: DateTime<Utc>,
    pub value_date: NaiveDate,
    pub reversal_deadline: Option<DateTime<Utc>>,
    pub waived: bool,
    pub waived_by: Option<Uuid>,
    /// References ReasonAndPurpose.id for waiver reason
    pub waived_reason_id: Option<Uuid>,
    pub applied_by: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Database model for fee waivers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeWaiverModel {
    pub waiver_id: Uuid,
    pub fee_application_id: Uuid,
    pub account_id: Uuid,
    pub waived_amount: Decimal,
    /// References ReasonAndPurpose.id for waiver reason
    pub reason_id: Uuid,
    /// Additional context for waiver
    pub additional_details: Option<HeaplessString<200>>,
    pub waived_by: Uuid,
    pub waived_at: DateTime<Utc>,
    pub approval_required: bool,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
}

/// Database model for fee processing jobs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeProcessingJobModel {
    pub job_id: Uuid,
    #[serde(serialize_with = "serialize_job_type", deserialize_with = "deserialize_job_type")]
    pub job_type: FeeJobType,
    pub job_name: String,
    pub schedule_expression: String,
    pub target_fee_categories: String, // JSON array
    pub target_products: Option<String>, // JSON array
    pub processing_date: NaiveDate,
    #[serde(serialize_with = "serialize_job_status", deserialize_with = "deserialize_job_status")]
    pub status: FeeJobStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub accounts_processed: i32,
    pub fees_applied: i32,
    pub total_amount: Decimal,
    pub errors: Option<String>, // JSON array of error messages
    pub created_at: DateTime<Utc>,
}

/// Database model for enhanced account holds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeAccountHoldModel {
    pub hold_id: Uuid,
    pub account_id: Uuid,
    pub amount: Decimal,
    pub hold_type: String, // UnclearedFunds, JudicialLien, LoanPledge, etc.
    /// References ReasonAndPurpose.id for hold reason
    pub reason_id: Uuid,
    /// Additional context for hold
    pub additional_details: Option<HeaplessString<200>>,
    pub placed_by: Uuid,
    pub placed_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: String, // Active, Released, Expired, Cancelled, PartiallyReleased
    pub released_at: Option<DateTime<Utc>>,
    pub released_by: Option<Uuid>,
    pub priority: String, // Critical, High, Medium, Low
    pub source_reference: Option<String>,
    pub automatic_release: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for hold release records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldReleaseRecordModel {
    pub release_record_id: Uuid,
    pub hold_id: Uuid,
    pub release_amount: Decimal,
    /// References ReasonAndPurpose.id for release reason
    pub release_reason_id: Uuid,
    /// Additional context for release
    pub release_additional_details: Option<HeaplessString<200>>,
    pub released_by: Uuid,
    pub released_at: DateTime<Utc>,
    pub is_partial_release: bool,
    pub remaining_amount: Decimal,
}

/// Database model for hold expiry jobs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldExpiryJobModel {
    pub job_id: Uuid,
    pub processing_date: NaiveDate,
    pub expired_holds_count: i32,
    pub total_released_amount: Decimal,
    pub processed_at: DateTime<Utc>,
    pub errors: Option<String>, // JSON array
}

/// Database model for balance calculations (for caching/audit)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceCalculationModel {
    pub calculation_id: Uuid,
    pub account_id: Uuid,
    pub current_balance: Decimal,
    pub available_balance: Decimal,
    pub overdraft_limit: Option<Decimal>,
    pub total_holds: Decimal,
    pub active_hold_count: i32,
    pub calculation_timestamp: DateTime<Utc>,
    pub hold_breakdown: String, // JSON serialized hold summary
}

/// Database model for product fee schedules (cached from Product Catalog)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFeeScheduleModel {
    pub schedule_id: Uuid,
    pub product_code: HeaplessString<12>,
    pub fee_schedule_data: String, // JSON serialized fee schedule
    pub effective_from: NaiveDate,
    pub effective_to: Option<NaiveDate>,
    pub last_updated: DateTime<Utc>,
    pub version: i32,
}

/// Database model for fee calculation cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeCalculationCacheModel {
    pub cache_id: Uuid,
    pub product_code: HeaplessString<12>,
    pub fee_code: HeaplessString<12>,
    pub calculation_key: String, // Hash of calculation parameters
    pub calculated_amount: Decimal,
    pub base_amount: Option<Decimal>,
    pub calculation_context: Option<String>,
    pub cached_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

// Custom serialization functions for enum compatibility
fn serialize_fee_type<S>(value: &FeeType, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        FeeType::EventBased => "EventBased",
        FeeType::Periodic => "Periodic",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_fee_type<'de, D>(deserializer: D) -> Result<FeeType, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "EventBased" => Ok(FeeType::EventBased),
        "Periodic" => Ok(FeeType::Periodic),
        _ => Err(serde::de::Error::custom(format!("Invalid FeeType: {s}"))),
    }
}

fn serialize_fee_category<S>(value: &FeeCategory, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        FeeCategory::Transaction => "Transaction",
        FeeCategory::Maintenance => "Maintenance",
        FeeCategory::Service => "Service",
        FeeCategory::Penalty => "Penalty",
        FeeCategory::Card => "Card",
        FeeCategory::Loan => "Loan",
        FeeCategory::Regulatory => "Regulatory",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_fee_category<'de, D>(deserializer: D) -> Result<FeeCategory, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Transaction" => Ok(FeeCategory::Transaction),
        "Maintenance" => Ok(FeeCategory::Maintenance),
        "Service" => Ok(FeeCategory::Service),
        "Penalty" => Ok(FeeCategory::Penalty),
        "Card" => Ok(FeeCategory::Card),
        "Loan" => Ok(FeeCategory::Loan),
        "Regulatory" => Ok(FeeCategory::Regulatory),
        _ => Err(serde::de::Error::custom(format!("Invalid FeeCategory: {s}"))),
    }
}

fn serialize_calculation_method<S>(value: &FeeCalculationMethod, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        FeeCalculationMethod::Fixed => "Fixed",
        FeeCalculationMethod::Percentage => "Percentage",
        FeeCalculationMethod::Tiered => "Tiered",
        FeeCalculationMethod::BalanceBased => "BalanceBased",
        FeeCalculationMethod::RuleBased => "RuleBased",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_calculation_method<'de, D>(deserializer: D) -> Result<FeeCalculationMethod, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Fixed" => Ok(FeeCalculationMethod::Fixed),
        "Percentage" => Ok(FeeCalculationMethod::Percentage),
        "Tiered" => Ok(FeeCalculationMethod::Tiered),
        "BalanceBased" => Ok(FeeCalculationMethod::BalanceBased),
        "RuleBased" => Ok(FeeCalculationMethod::RuleBased),
        _ => Err(serde::de::Error::custom(format!("Invalid FeeCalculationMethod: {s}"))),
    }
}

fn serialize_trigger_event<S>(value: &FeeTriggerEvent, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        FeeTriggerEvent::AtmWithdrawal => "AtmWithdrawal",
        FeeTriggerEvent::PosTraction => "PosTraction",
        FeeTriggerEvent::WireTransfer => "WireTransfer",
        FeeTriggerEvent::OnlineTransfer => "OnlineTransfer",
        FeeTriggerEvent::CheckDeposit => "CheckDeposit",
        FeeTriggerEvent::InsufficientFunds => "InsufficientFunds",
        FeeTriggerEvent::OverdraftUsage => "OverdraftUsage",
        FeeTriggerEvent::BelowMinimumBalance => "BelowMinimumBalance",
        FeeTriggerEvent::AccountOpening => "AccountOpening",
        FeeTriggerEvent::AccountClosure => "AccountClosure",
        FeeTriggerEvent::AccountMaintenance => "AccountMaintenance",
        FeeTriggerEvent::StatementGeneration => "StatementGeneration",
        FeeTriggerEvent::MonthlyMaintenance => "MonthlyMaintenance",
        FeeTriggerEvent::QuarterlyMaintenance => "QuarterlyMaintenance",
        FeeTriggerEvent::AnnualMaintenance => "AnnualMaintenance",
        FeeTriggerEvent::CardIssuance => "CardIssuance",
        FeeTriggerEvent::CardReplacement => "CardReplacement",
        FeeTriggerEvent::CardActivation => "CardActivation",
        FeeTriggerEvent::Manual => "Manual",
        FeeTriggerEvent::Regulatory => "Regulatory",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_trigger_event<'de, D>(deserializer: D) -> Result<FeeTriggerEvent, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "AtmWithdrawal" => Ok(FeeTriggerEvent::AtmWithdrawal),
        "PosTraction" => Ok(FeeTriggerEvent::PosTraction),
        "WireTransfer" => Ok(FeeTriggerEvent::WireTransfer),
        "OnlineTransfer" => Ok(FeeTriggerEvent::OnlineTransfer),
        "CheckDeposit" => Ok(FeeTriggerEvent::CheckDeposit),
        "InsufficientFunds" => Ok(FeeTriggerEvent::InsufficientFunds),
        "OverdraftUsage" => Ok(FeeTriggerEvent::OverdraftUsage),
        "BelowMinimumBalance" => Ok(FeeTriggerEvent::BelowMinimumBalance),
        "AccountOpening" => Ok(FeeTriggerEvent::AccountOpening),
        "AccountClosure" => Ok(FeeTriggerEvent::AccountClosure),
        "AccountMaintenance" => Ok(FeeTriggerEvent::AccountMaintenance),
        "StatementGeneration" => Ok(FeeTriggerEvent::StatementGeneration),
        "MonthlyMaintenance" => Ok(FeeTriggerEvent::MonthlyMaintenance),
        "QuarterlyMaintenance" => Ok(FeeTriggerEvent::QuarterlyMaintenance),
        "AnnualMaintenance" => Ok(FeeTriggerEvent::AnnualMaintenance),
        "CardIssuance" => Ok(FeeTriggerEvent::CardIssuance),
        "CardReplacement" => Ok(FeeTriggerEvent::CardReplacement),
        "CardActivation" => Ok(FeeTriggerEvent::CardActivation),
        "Manual" => Ok(FeeTriggerEvent::Manual),
        "Regulatory" => Ok(FeeTriggerEvent::Regulatory),
        _ => Err(serde::de::Error::custom(format!("Invalid FeeTriggerEvent: {s}"))),
    }
}

fn serialize_application_status<S>(value: &FeeApplicationStatus, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        FeeApplicationStatus::Applied => "Applied",
        FeeApplicationStatus::Pending => "Pending",
        FeeApplicationStatus::Waived => "Waived",
        FeeApplicationStatus::Reversed => "Reversed",
        FeeApplicationStatus::Failed => "Failed",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_application_status<'de, D>(deserializer: D) -> Result<FeeApplicationStatus, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Applied" => Ok(FeeApplicationStatus::Applied),
        "Pending" => Ok(FeeApplicationStatus::Pending),
        "Waived" => Ok(FeeApplicationStatus::Waived),
        "Reversed" => Ok(FeeApplicationStatus::Reversed),
        "Failed" => Ok(FeeApplicationStatus::Failed),
        _ => Err(serde::de::Error::custom(format!("Invalid FeeApplicationStatus: {s}"))),
    }
}

fn serialize_job_type<S>(value: &FeeJobType, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        FeeJobType::DailyMaintenance => "DailyMaintenance",
        FeeJobType::MonthlyMaintenance => "MonthlyMaintenance",
        FeeJobType::QuarterlyMaintenance => "QuarterlyMaintenance",
        FeeJobType::AnnualMaintenance => "AnnualMaintenance",
        FeeJobType::AdHocFeeApplication => "AdHocFeeApplication",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_job_type<'de, D>(deserializer: D) -> Result<FeeJobType, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "DailyMaintenance" => Ok(FeeJobType::DailyMaintenance),
        "MonthlyMaintenance" => Ok(FeeJobType::MonthlyMaintenance),
        "QuarterlyMaintenance" => Ok(FeeJobType::QuarterlyMaintenance),
        "AnnualMaintenance" => Ok(FeeJobType::AnnualMaintenance),
        "AdHocFeeApplication" => Ok(FeeJobType::AdHocFeeApplication),
        _ => Err(serde::de::Error::custom(format!("Invalid FeeJobType: {s}"))),
    }
}

fn serialize_job_status<S>(value: &FeeJobStatus, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        FeeJobStatus::Scheduled => "Scheduled",
        FeeJobStatus::Running => "Running",
        FeeJobStatus::Completed => "Completed",
        FeeJobStatus::Failed => "Failed",
        FeeJobStatus::Cancelled => "Cancelled",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_job_status<'de, D>(deserializer: D) -> Result<FeeJobStatus, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Scheduled" => Ok(FeeJobStatus::Scheduled),
        "Running" => Ok(FeeJobStatus::Running),
        "Completed" => Ok(FeeJobStatus::Completed),
        "Failed" => Ok(FeeJobStatus::Failed),
        "Cancelled" => Ok(FeeJobStatus::Cancelled),
        _ => Err(serde::de::Error::custom(format!("Invalid FeeJobStatus: {s}"))),
    }
}
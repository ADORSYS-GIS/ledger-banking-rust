use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Fee Application Record - tracks individual fee applications
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct FeeApplication {
    pub id: Uuid,
    pub account_id: Uuid,
    pub transaction_id: Option<Uuid>, // For event-based fees
    pub fee_type: FeeType,
    pub fee_category: FeeCategory,
    pub product_id: Uuid,
    pub fee_code: HeaplessString<12>,
    pub description: HeaplessString<200>,
    pub amount: Decimal,
    pub currency: HeaplessString<3>,
    pub calculation_method: FeeCalculationMethod,
    pub calculation_base_amount: Option<Decimal>,
    pub fee_rate: Option<Decimal>,
    pub trigger_event: FeeTriggerEvent,
    pub status: FeeApplicationStatus,
    pub applied_at: DateTime<Utc>,
    pub value_date: NaiveDate,
    pub reversal_deadline: Option<DateTime<Utc>>,
    pub waived: bool,
    /// References Person.person_id
    pub waived_by: Option<Uuid>,
    /// References ReasonAndPurpose.id for waiver reason
    pub waived_reason_id: Option<Uuid>,
    /// References Person.person_id
    pub applied_by: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Fee Type Classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeeType {
    /// Real-time fees applied during transaction processing
    EventBased,
    /// Periodic fees applied via batch processing
    Periodic,
}

/// Fee Categories for business logic
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FeeCategory {
    #[default]
    /// Transaction-related fees
    Transaction,
    /// Account maintenance fees
    Maintenance,
    /// Service fees
    Service,
    /// Overdraft and penalty fees
    Penalty,
    /// Card-related fees
    Card,
    /// Loan-related fees
    Loan,
    /// Compliance and regulatory fees
    Regulatory,
}

/// Methods for calculating fee amounts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeeCalculationMethod {
    /// Fixed amount regardless of transaction
    Fixed,
    /// Percentage of transaction amount
    Percentage,
    /// Tiered based on amount ranges
    Tiered,
    /// Balance-based calculation
    BalanceBased,
    /// Complex calculation requiring external rules
    RuleBased,
}

/// Events that can trigger fee applications
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FeeTriggerEvent {
    // Transaction-based triggers
    AtmWithdrawal,
    PosTraction,
    WireTransfer,
    OnlineTransfer,
    CheckDeposit,
    InsufficientFunds,
    OverdraftUsage,
    BelowMinimumBalance,
    
    // Account-based triggers
    AccountOpening,
    AccountClosure,
    AccountMaintenance,
    StatementGeneration,
    
    // Time-based triggers
    MonthlyMaintenance,
    QuarterlyMaintenance,
    AnnualMaintenance,
    
    // Card-based triggers
    CardIssuance,
    CardReplacement,
    CardActivation,
    
    // Other triggers
    Manual,
    Regulatory,
}

/// Status of fee application
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeeApplicationStatus {
    Applied,
    Pending,
    Waived,
    Reversed,
    Failed,
}

/// Fee Schedule from Product Catalog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFeeSchedule {
    pub product_id: Uuid,
    pub fees: Vec<ProductFee>,
    pub effective_from: NaiveDate,
    pub effective_to: Option<NaiveDate>,
}

/// Individual fee configuration in product catalog
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ProductFee {
    pub fee_code: HeaplessString<12>,
    pub description: HeaplessString<200>,
    pub fee_category: FeeCategory,
    pub trigger_event: FeeTriggerEvent,
    pub calculation_method: FeeCalculationMethod,
    pub fixed_amount: Option<Decimal>,
    pub percentage_rate: Option<Decimal>,
    pub minimum_amount: Option<Decimal>,
    pub maximum_amount: Option<Decimal>,
    pub currency: HeaplessString<3>,
    pub frequency: AccountFeeFrequency,
    pub tier_schedule: Option<Vec<FeeTier>>,
    pub conditions: Vec<FeeCondition>,
    pub gl_code: HeaplessString<10>,
    pub waivable: bool,
    pub active: bool,
}

/// Fee application frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountFeeFrequency {
    /// Per transaction/event
    PerEvent,
    /// Once per day maximum
    Daily,
    /// Weekly recurring
    Weekly,
    /// Monthly recurring
    Monthly,
    /// Quarterly recurring
    Quarterly,
    /// Annual recurring
    Annual,
    /// One-time only
    OneTime,
}

/// Tiered fee calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeTier {
    pub tier_name: String,
    pub min_amount: Decimal,
    pub max_amount: Option<Decimal>,
    pub fee_amount: Decimal,
    pub fee_percentage: Option<Decimal>,
}

/// Conditions for fee application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeCondition {
    pub condition_type: FeeConditionType,
    pub operator: ComparisonOperator,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeeConditionType {
    MinimumBalance,
    MaximumBalance,
    TransactionAmount,
    TransactionCount,
    AccountAge,
    CustomerStatus,
    ProductType,
    Channel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    In,
    NotIn,
}

/// Batch fee processing job configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeProcessingJob {
    pub id: Uuid,
    pub job_type: FeeJobType,
    pub job_name: String,
    pub schedule_expression: HeaplessString<200>, // Cron expression
    pub target_fee_categories_01: FeeCategory,
    pub target_fee_categories_02: FeeCategory,
    pub target_fee_categories_03: FeeCategory,
    pub target_fee_categories_04: FeeCategory,
    pub target_fee_categories_05: FeeCategory,
    pub target_product_id_01: Option<Uuid>,
    pub target_product_id_02: Option<Uuid>,
    pub target_product_id_03: Option<Uuid>,
    pub target_product_id_04: Option<Uuid>,
    pub target_product_id_05: Option<Uuid>,
    pub processing_date: NaiveDate,
    pub status: FeeJobStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub accounts_processed: u32,
    pub fees_applied: u32,
    pub total_amount: Decimal,
    pub errors_01: HeaplessString<200>,
    pub errors_02: HeaplessString<200>,
    pub errors_03: HeaplessString<200>,
    pub errors_04: HeaplessString<200>,
    pub errors_05: HeaplessString<200>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeeJobType {
    DailyMaintenance,
    MonthlyMaintenance,
    QuarterlyMaintenance,
    AnnualMaintenance,
    AdHocFeeApplication,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeeJobStatus {
    Scheduled,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Fee waiver record for audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeWaiver {
    pub id: Uuid,
    pub fee_application_id: Uuid,
    pub account_id: Uuid,
    pub waived_amount: Decimal,
    /// References ReasonAndPurpose.id for waiver reason
    pub reason_id: Uuid,
    /// Additional context for waiver
    pub additional_details: Option<HeaplessString<200>>,
    /// References Person.person_id
    pub waived_by: Uuid,
    pub waived_at: DateTime<Utc>,
    pub approval_required: bool,
    /// References Person.person_id
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
}

// ============================================================================
// DISPLAY AND FROMSTR IMPLEMENTATIONS FOR DATABASE COMPATIBILITY
// ============================================================================

impl std::fmt::Display for FeeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeeType::EventBased => write!(f, "EventBased"),
            FeeType::Periodic => write!(f, "Periodic"),
        }
    }
}

impl std::str::FromStr for FeeType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EventBased" => Ok(FeeType::EventBased),
            "Periodic" => Ok(FeeType::Periodic),
            _ => Err(format!("Invalid FeeType: {s}")),
        }
    }
}

impl std::fmt::Display for FeeCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeeCategory::Transaction => write!(f, "Transaction"),
            FeeCategory::Maintenance => write!(f, "Maintenance"),
            FeeCategory::Service => write!(f, "Service"),
            FeeCategory::Penalty => write!(f, "Penalty"),
            FeeCategory::Card => write!(f, "Card"),
            FeeCategory::Loan => write!(f, "Loan"),
            FeeCategory::Regulatory => write!(f, "Regulatory"),
        }
    }
}

impl std::str::FromStr for FeeCategory {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Transaction" => Ok(FeeCategory::Transaction),
            "Maintenance" => Ok(FeeCategory::Maintenance),
            "Service" => Ok(FeeCategory::Service),
            "Penalty" => Ok(FeeCategory::Penalty),
            "Card" => Ok(FeeCategory::Card),
            "Loan" => Ok(FeeCategory::Loan),
            "Regulatory" => Ok(FeeCategory::Regulatory),
            _ => Err(format!("Invalid FeeCategory: {s}")),
        }
    }
}

impl std::fmt::Display for FeeCalculationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeeCalculationMethod::Fixed => write!(f, "Fixed"),
            FeeCalculationMethod::Percentage => write!(f, "Percentage"),
            FeeCalculationMethod::Tiered => write!(f, "Tiered"),
            FeeCalculationMethod::BalanceBased => write!(f, "BalanceBased"),
            FeeCalculationMethod::RuleBased => write!(f, "RuleBased"),
        }
    }
}

impl std::str::FromStr for FeeCalculationMethod {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Fixed" => Ok(FeeCalculationMethod::Fixed),
            "Percentage" => Ok(FeeCalculationMethod::Percentage),
            "Tiered" => Ok(FeeCalculationMethod::Tiered),
            "BalanceBased" => Ok(FeeCalculationMethod::BalanceBased),
            "RuleBased" => Ok(FeeCalculationMethod::RuleBased),
            _ => Err(format!("Invalid FeeCalculationMethod: {s}")),
        }
    }
}

impl std::fmt::Display for FeeTriggerEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeeTriggerEvent::AtmWithdrawal => write!(f, "AtmWithdrawal"),
            FeeTriggerEvent::PosTraction => write!(f, "PosTraction"),
            FeeTriggerEvent::WireTransfer => write!(f, "WireTransfer"),
            FeeTriggerEvent::OnlineTransfer => write!(f, "OnlineTransfer"),
            FeeTriggerEvent::CheckDeposit => write!(f, "CheckDeposit"),
            FeeTriggerEvent::InsufficientFunds => write!(f, "InsufficientFunds"),
            FeeTriggerEvent::OverdraftUsage => write!(f, "OverdraftUsage"),
            FeeTriggerEvent::BelowMinimumBalance => write!(f, "BelowMinimumBalance"),
            FeeTriggerEvent::AccountOpening => write!(f, "AccountOpening"),
            FeeTriggerEvent::AccountClosure => write!(f, "AccountClosure"),
            FeeTriggerEvent::AccountMaintenance => write!(f, "AccountMaintenance"),
            FeeTriggerEvent::StatementGeneration => write!(f, "StatementGeneration"),
            FeeTriggerEvent::MonthlyMaintenance => write!(f, "MonthlyMaintenance"),
            FeeTriggerEvent::QuarterlyMaintenance => write!(f, "QuarterlyMaintenance"),
            FeeTriggerEvent::AnnualMaintenance => write!(f, "AnnualMaintenance"),
            FeeTriggerEvent::CardIssuance => write!(f, "CardIssuance"),
            FeeTriggerEvent::CardReplacement => write!(f, "CardReplacement"),
            FeeTriggerEvent::CardActivation => write!(f, "CardActivation"),
            FeeTriggerEvent::Manual => write!(f, "Manual"),
            FeeTriggerEvent::Regulatory => write!(f, "Regulatory"),
        }
    }
}

impl std::str::FromStr for FeeTriggerEvent {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
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
            _ => Err(format!("Invalid FeeTriggerEvent: {s}")),
        }
    }
}

impl std::fmt::Display for FeeApplicationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeeApplicationStatus::Applied => write!(f, "Applied"),
            FeeApplicationStatus::Pending => write!(f, "Pending"),
            FeeApplicationStatus::Waived => write!(f, "Waived"),
            FeeApplicationStatus::Reversed => write!(f, "Reversed"),
            FeeApplicationStatus::Failed => write!(f, "Failed"),
        }
    }
}

impl std::str::FromStr for FeeApplicationStatus {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Applied" => Ok(FeeApplicationStatus::Applied),
            "Pending" => Ok(FeeApplicationStatus::Pending),
            "Waived" => Ok(FeeApplicationStatus::Waived),
            "Reversed" => Ok(FeeApplicationStatus::Reversed),
            "Failed" => Ok(FeeApplicationStatus::Failed),
            _ => Err(format!("Invalid FeeApplicationStatus: {s}")),
        }
    }
}

impl std::fmt::Display for FeeJobType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeeJobType::DailyMaintenance => write!(f, "DailyMaintenance"),
            FeeJobType::MonthlyMaintenance => write!(f, "MonthlyMaintenance"),
            FeeJobType::QuarterlyMaintenance => write!(f, "QuarterlyMaintenance"),
            FeeJobType::AnnualMaintenance => write!(f, "AnnualMaintenance"),
            FeeJobType::AdHocFeeApplication => write!(f, "AdHocFeeApplication"),
        }
    }
}

impl std::str::FromStr for FeeJobType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DailyMaintenance" => Ok(FeeJobType::DailyMaintenance),
            "MonthlyMaintenance" => Ok(FeeJobType::MonthlyMaintenance),
            "QuarterlyMaintenance" => Ok(FeeJobType::QuarterlyMaintenance),
            "AnnualMaintenance" => Ok(FeeJobType::AnnualMaintenance),
            "AdHocFeeApplication" => Ok(FeeJobType::AdHocFeeApplication),
            _ => Err(format!("Invalid FeeJobType: {s}")),
        }
    }
}

impl std::fmt::Display for FeeJobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeeJobStatus::Scheduled => write!(f, "Scheduled"),
            FeeJobStatus::Running => write!(f, "Running"),
            FeeJobStatus::Completed => write!(f, "Completed"),
            FeeJobStatus::Failed => write!(f, "Failed"),
            FeeJobStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

impl std::str::FromStr for FeeJobStatus {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Scheduled" => Ok(FeeJobStatus::Scheduled),
            "Running" => Ok(FeeJobStatus::Running),
            "Completed" => Ok(FeeJobStatus::Completed),
            "Failed" => Ok(FeeJobStatus::Failed),
            "Cancelled" => Ok(FeeJobStatus::Cancelled),
            _ => Err(format!("Invalid FeeJobStatus: {s}")),
        }
    }
}
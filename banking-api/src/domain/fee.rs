use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Fee Application Record - tracks individual fee applications
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct FeeApplication {
    pub fee_application_id: Uuid,
    pub account_id: Uuid,
    pub transaction_id: Option<Uuid>, // For event-based fees
    pub fee_type: FeeType,
    pub fee_category: FeeCategory,
    pub product_code: String,
    pub fee_code: String,
    #[validate(length(max = 255))]
    pub description: String,
    pub amount: Decimal,
    pub currency: String,
    pub calculation_method: FeeCalculationMethod,
    pub calculation_base_amount: Option<Decimal>,
    pub fee_rate: Option<Decimal>,
    pub trigger_event: FeeTriggerEvent,
    pub status: FeeApplicationStatus,
    pub applied_at: DateTime<Utc>,
    pub value_date: NaiveDate,
    pub reversal_deadline: Option<DateTime<Utc>>,
    pub waived: bool,
    /// References ReferencedPerson.person_id
    pub waived_by: Option<Uuid>,
    /// References ReasonAndPurpose.id for waiver reason
    pub waived_reason_id: Option<Uuid>,
    /// References ReferencedPerson.person_id
    pub applied_by: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Fee Type Classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeeType {
    /// Real-time fees applied during transaction processing
    EventBased,
    /// Periodic fees applied via batch processing
    Periodic,
}

/// Fee Categories for business logic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeeCategory {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub product_code: String,
    pub fees: Vec<ProductFee>,
    pub effective_from: NaiveDate,
    pub effective_to: Option<NaiveDate>,
}

/// Individual fee configuration in product catalog
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ProductFee {
    pub fee_code: String,
    #[validate(length(max = 255))]
    pub description: String,
    pub fee_category: FeeCategory,
    pub trigger_event: FeeTriggerEvent,
    pub calculation_method: FeeCalculationMethod,
    pub fixed_amount: Option<Decimal>,
    pub percentage_rate: Option<Decimal>,
    pub minimum_amount: Option<Decimal>,
    pub maximum_amount: Option<Decimal>,
    pub currency: String,
    pub frequency: FeeFrequency,
    pub tier_schedule: Option<Vec<FeeTier>>,
    pub conditions: Vec<FeeCondition>,
    pub gl_code: String,
    pub waivable: bool,
    pub active: bool,
}

/// Fee application frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeeFrequency {
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
    pub job_id: Uuid,
    pub job_type: FeeJobType,
    pub job_name: String,
    pub schedule_expression: String, // Cron expression
    pub target_fee_categories: Vec<FeeCategory>,
    pub target_products: Option<Vec<String>>,
    pub processing_date: NaiveDate,
    pub status: FeeJobStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub accounts_processed: u32,
    pub fees_applied: u32,
    pub total_amount: Decimal,
    pub errors: Vec<String>,
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
    pub waiver_id: Uuid,
    pub fee_application_id: Uuid,
    pub account_id: Uuid,
    pub waived_amount: Decimal,
    /// References ReasonAndPurpose.id for waiver reason
    pub reason_id: Uuid,
    /// Additional context for waiver
    pub additional_details: Option<HeaplessString<200>>,
    /// References ReferencedPerson.person_id
    pub waived_by: Uuid,
    pub waived_at: DateTime<Utc>,
    pub approval_required: bool,
    /// References ReferencedPerson.person_id
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
}
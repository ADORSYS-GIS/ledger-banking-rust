use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Comprehensive loan servicing functionality
/// Building upon the loan fields in the Unified Account Model
/// Amortization schedule for loan installment planning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmortizationSchedule {
    pub id: Uuid,
    pub loan_account_id: Uuid,
    pub original_principal: Decimal,
    pub interest_rate: Decimal,
    pub term_months: u32,
    pub installment_amount: Decimal,
    pub first_payment_date: NaiveDate,
    pub maturity_date: NaiveDate,
    pub total_interest: Decimal,
    pub total_payments: Decimal,
    pub schedule_entries: Vec<AmortizationEntry>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
}

/// Individual installment in the amortization schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmortizationEntry {
    pub id: Uuid,
    pub schedule_id: Uuid,
    pub installment_number: u32,
    pub due_date: NaiveDate,
    pub opening_principal_balance: Decimal,
    pub installment_amount: Decimal,
    pub principal_component: Decimal,
    pub interest_component: Decimal,
    pub closing_principal_balance: Decimal,
    pub cumulative_principal_paid: Decimal,
    pub cumulative_interest_paid: Decimal,
    pub payment_status: InstallmentStatus,
    pub paid_date: Option<NaiveDate>,
    pub paid_amount: Option<Decimal>,
    pub days_overdue: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstallmentStatus {
    Scheduled,
    Due,
    PartiallyPaid,
    Paid,
    Overdue,
    WriteOff,
}

/// Loan delinquency tracking and management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanDelinquency {
    pub id: Uuid,
    pub loan_account_id: Uuid,
    pub delinquency_start_date: NaiveDate,
    pub current_dpd: u32, // Days Past Due
    pub highest_dpd: u32,
    pub delinquency_stage: DelinquencyStage,
    pub overdue_principal: Decimal,
    pub overdue_interest: Decimal,
    pub penalty_interest_accrued: Decimal,
    pub total_overdue_amount: Decimal,
    pub last_payment_date: Option<NaiveDate>,
    pub last_payment_amount: Option<Decimal>,
    pub collection_actions: Vec<CollectionAction>,
    pub restructuring_eligibility: bool,
    pub npl_date: Option<NaiveDate>, // Non-Performing Loan classification
    pub provisioning_amount: Decimal,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DelinquencyStage {
    Current,        // 0-30 DPD
    Stage1,         // 31-60 DPD
    Stage2,         // 61-90 DPD
    Stage3,         // 91-180 DPD
    NonPerforming,  // 180+ DPD
    WriteOff,
}

/// Collection actions taken for delinquent loans
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CollectionAction {
    pub id: Uuid,
    pub delinquency_id: Uuid,
    pub loan_account_id: Uuid,
    pub action_type: CollectionActionType,
    pub action_date: NaiveDate,
    pub due_date: Option<NaiveDate>,
    pub description: HeaplessString<500>,
    pub amount_demanded: Option<Decimal>,
    pub response_received: bool,
    pub response_date: Option<NaiveDate>,
    pub response_details: Option<HeaplessString<500>>,
    pub follow_up_required: bool,
    pub follow_up_date: Option<NaiveDate>,
    pub action_status: ActionStatus,
    pub assigned_to: Uuid, // References Person.person_id
    pub created_by: Uuid, // References Person.person_id
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollectionActionType {
    EmailReminder,
    SmsNotification,
    PhoneCall,
    LetterNotice,
    LegalNotice,
    FieldVisit,
    PaymentPlan,
    Restructuring,
    LegalAction,
    AssetRecovery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionStatus {
    Planned,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

/// Loan payment processing and allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanPayment {
    pub id: Uuid,
    pub loan_account_id: Uuid,
    pub payment_date: NaiveDate,
    pub payment_amount: Decimal,
    pub payment_type: PaymentType,
    pub payment_method: PaymentMethod,
    pub allocation: PaymentAllocation,
    pub payment_status: PaymentStatus,
    pub external_reference: Option<HeaplessString<100>>,
    pub processed_by: Uuid, // References Person.person_id
    pub processed_at: DateTime<Utc>,
    pub reversal_info: Option<PaymentReversal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentType {
    Regular,
    Prepayment,
    PartialPayment,
    Restructured,
    Settlement,
    Recovery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethod {
    BankTransfer,
    DirectDebit,
    Check,
    Cash,
    OnlinePayment,
    MobilePayment,
    StandingInstruction,
}

/// Detailed payment allocation across loan components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentAllocation {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub penalty_interest_payment: Decimal,
    pub overdue_interest_payment: Decimal,
    pub current_interest_payment: Decimal,
    pub principal_payment: Decimal,
    pub fees_payment: Decimal,
    pub charges_payment: Decimal,
    pub excess_amount: Decimal,
    pub prepayment_handling: Option<PrepaymentHandling>,
}

/// Prepayment handling options and results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrepaymentHandling {
    pub handling_type: PrepaymentType,
    pub excess_amount: Decimal,
    pub new_outstanding_principal: Decimal,
    pub term_reduction_months: Option<u32>,
    pub new_installment_amount: Option<Decimal>,
    pub new_maturity_date: Option<NaiveDate>,
    pub schedule_regenerated: bool,
    pub customer_choice: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrepaymentType {
    TermReduction,      // Apply excess to principal, reduce term
    InstallmentReduction, // Apply excess to principal, reduce installment amount
    HoldInSuspense,     // Hold excess for future installments
    Refund,            // Refund to customer
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentStatus {
    Processed,
    Pending,
    Failed,
    Reversed,
    PartiallyAllocated,
}

/// Payment reversal tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentReversal {
    pub id: Uuid,
    pub original_payment_id: Uuid,
    /// References ReasonAndPurpose.id for reversal reason
    pub reversal_reason_id: Uuid,
    /// Additional context for reversal
    pub additional_details: Option<HeaplessString<200>>,
    pub reversed_amount: Decimal,
    pub reversed_by: Uuid, // References Person.person_id
    pub reversed_at: DateTime<Utc>,
    pub schedule_adjusted: bool,
}

/// Loan restructuring and modification
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoanRestructuring {
    pub id: Uuid,
    pub loan_account_id: Uuid,
    pub restructuring_type: RestructuringType,
    pub request_date: NaiveDate,
    pub effective_date: Option<NaiveDate>,
    /// References ReasonAndPurpose.id for restructuring reason
    pub restructuring_reason_id: Uuid,
    /// Additional context for restructuring
    pub additional_details: Option<HeaplessString<200>>,
    
    // Original loan terms
    pub original_principal: Decimal,
    pub original_interest_rate: Decimal,
    pub original_term_months: u32,
    pub original_installment: Decimal,
    
    // New loan terms
    pub new_principal: Option<Decimal>,
    pub new_interest_rate: Option<Decimal>,
    pub new_term_months: Option<u32>,
    pub new_installment: Option<Decimal>,
    pub new_maturity_date: Option<NaiveDate>,
    
    // Restructuring details
    pub moratorium_period: Option<u32>, // Months
    pub capitalized_interest: Option<Decimal>,
    pub waived_penalty_amount: Option<Decimal>,
    pub approval_status: LoanApprovalStatus,
    pub approved_by: Option<Uuid>, // References Person.person_id
    pub approved_at: Option<DateTime<Utc>>,
    pub conditions: Vec<HeaplessString<500>>,
    pub created_by: Uuid, // References Person.person_id
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RestructuringType {
    PaymentHoliday,
    TermExtension,
    RateReduction,
    PrincipalReduction,
    InstallmentReduction,
    InterestCapitalization,
    FullRestructuring,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoanApprovalStatus {
    Pending,
    Approved,
    Rejected,
    ConditionallyApproved,
    RequiresCommitteeApproval,
}

/// EOD loan processing job for delinquency management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanDelinquencyJob {
    pub id: Uuid,
    pub processing_date: NaiveDate,
    pub loans_processed: u32,
    pub new_delinquent_loans: u32,
    pub recovered_loans: u32,
    pub npl_classifications: u32,
    pub total_penalty_interest: Decimal,
    pub collection_actions_triggered: u32,
    pub notifications_sent: u32,
    pub status: super::casa::ProcessingJobStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub errors: Vec<HeaplessString<200>>,
}

/// Loan portfolio summary for management reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanPortfolioSummary {
    pub summary_date: NaiveDate,
    pub total_loans: u32,
    pub total_outstanding_principal: Decimal,
    pub total_accrued_interest: Decimal,
    pub portfolio_by_stage: LoanStageBreakdown,
    pub delinquency_metrics: DelinquencyMetrics,
    pub collection_efficiency: CollectionEfficiency,
    pub provisioning_summary: ProvisioningSummary,
    pub average_loan_size: Decimal,
    pub average_interest_rate: Decimal,
    pub portfolio_yield: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanStageBreakdown {
    pub current_loans: (u32, Decimal),      // Count, Amount
    pub stage1_loans: (u32, Decimal),       // 31-60 DPD
    pub stage2_loans: (u32, Decimal),       // 61-90 DPD
    pub stage3_loans: (u32, Decimal),       // 91-180 DPD
    pub npl_loans: (u32, Decimal),          // 180+ DPD
    pub written_off_loans: (u32, Decimal),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelinquencyMetrics {
    pub par_30: Decimal,  // Portfolio at Risk 30+
    pub par_60: Decimal,  // Portfolio at Risk 60+
    pub par_90: Decimal,  // Portfolio at Risk 90+
    pub npl_ratio: Decimal,
    pub average_dpd: f64,
    pub collection_rate: Decimal,
    pub cure_rate: Decimal, // Loans returning to current status
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionEfficiency {
    pub total_collections: Decimal,
    pub collection_target: Decimal,
    pub efficiency_ratio: Decimal,
    pub average_collection_time: f64,
    pub successful_actions_ratio: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisioningSummary {
    pub total_provisions: Decimal,
    pub stage1_provisions: Decimal,
    pub stage2_provisions: Decimal,
    pub stage3_provisions: Decimal,
    pub write_off_amount: Decimal,
    pub provision_coverage_ratio: Decimal,
}

/// Payment frequency for loan installments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentFrequency {
    Weekly,
    BiWeekly,
    Monthly,
    Quarterly,
    SemiAnnually,
    Annually,
}

/// Amortization calculation methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AmortizationMethod {
    EqualInstallments,      // Equal monthly payments
    EqualPrincipal,         // Equal principal, declining interest
    InterestOnly,           // Interest-only with balloon payment
    BulletPayment,          // Single payment at maturity
}

/// Request parameters for generating an amortization schedule
#[derive(Debug, Clone, Validate)]
pub struct GenerateAmortizationScheduleRequest {
    pub loan_account_id: Uuid,
    pub principal_amount: Decimal,
    pub annual_interest_rate: Decimal,
    pub term_months: u32,
    pub first_payment_date: NaiveDate,
    pub payment_frequency: PaymentFrequency,
    pub calculation_method: AmortizationMethod,
}

/// Request parameters for creating a collection action
#[derive(Debug, Clone, Validate)]
pub struct CreateCollectionActionRequest {
    pub loan_account_id: Uuid,
    pub action_type: CollectionActionType,
    pub description: HeaplessString<500>,
    pub amount_demanded: Option<Decimal>,
    pub due_date: Option<NaiveDate>,
    pub assigned_to: Uuid, // References Person.person_id
    pub created_by: Uuid, // References Person.person_id
}
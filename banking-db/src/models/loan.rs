use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

use crate::models::casa::ProcessingJobStatus;

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
    #[serde(serialize_with = "serialize_installment_status", deserialize_with = "deserialize_installment_status")]
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
    #[serde(serialize_with = "serialize_delinquency_stage", deserialize_with = "deserialize_delinquency_stage")]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionAction {
    pub id: Uuid,
    pub delinquency_id: Uuid,
    pub loan_account_id: Uuid,
    #[serde(serialize_with = "serialize_collection_action_type", deserialize_with = "deserialize_collection_action_type")]
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
    #[serde(serialize_with = "serialize_action_status", deserialize_with = "deserialize_action_status")]
    pub action_status: ActionStatus,
    pub assigned_to: Uuid, // References Person.person_id
    pub created_by_person_id: Uuid, // References Person.person_id
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
    #[serde(serialize_with = "serialize_payment_type", deserialize_with = "deserialize_payment_type")]
    pub payment_type: PaymentType,
    #[serde(serialize_with = "serialize_payment_method", deserialize_with = "deserialize_payment_method")]
    pub payment_method: PaymentMethod,
    pub allocation: PaymentAllocation,
    #[serde(serialize_with = "serialize_payment_status", deserialize_with = "deserialize_payment_status")]
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
    #[serde(serialize_with = "serialize_prepayment_type", deserialize_with = "deserialize_prepayment_type")]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanRestructuring {
    pub id: Uuid,
    pub loan_account_id: Uuid,
    #[serde(serialize_with = "serialize_restructuring_type", deserialize_with = "deserialize_restructuring_type")]
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
    #[serde(serialize_with = "serialize_loan_approval_status", deserialize_with = "deserialize_loan_approval_status")]
    pub approval_status: LoanApprovalStatus,
    pub approved_by: Option<Uuid>, // References Person.person_id
    pub approved_at: Option<DateTime<Utc>>,
    pub conditions: Vec<HeaplessString<500>>,
    pub created_by_person_id: Uuid, // References Person.person_id
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
    #[serde(serialize_with = "crate::models::casa::serialize_processing_job_status", deserialize_with = "crate::models::casa::deserialize_processing_job_status")]
    pub status: ProcessingJobStatus,
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

// Custom serialization functions for database compatibility

fn serialize_installment_status<S>(value: &InstallmentStatus, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        InstallmentStatus::Scheduled => "Scheduled",
        InstallmentStatus::Due => "Due",
        InstallmentStatus::PartiallyPaid => "PartiallyPaid",
        InstallmentStatus::Paid => "Paid",
        InstallmentStatus::Overdue => "Overdue",
        InstallmentStatus::WriteOff => "WriteOff",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_installment_status<'de, D>(deserializer: D) -> Result<InstallmentStatus, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Scheduled" => Ok(InstallmentStatus::Scheduled),
        "Due" => Ok(InstallmentStatus::Due),
        "PartiallyPaid" => Ok(InstallmentStatus::PartiallyPaid),
        "Paid" => Ok(InstallmentStatus::Paid),
        "Overdue" => Ok(InstallmentStatus::Overdue),
        "WriteOff" => Ok(InstallmentStatus::WriteOff),
        _ => Err(serde::de::Error::unknown_variant(&s, &["Scheduled", "Due", "PartiallyPaid", "Paid", "Overdue", "WriteOff"])),
    }
}

fn serialize_delinquency_stage<S>(value: &DelinquencyStage, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        DelinquencyStage::Current => "Current",
        DelinquencyStage::Stage1 => "Stage1",
        DelinquencyStage::Stage2 => "Stage2",
        DelinquencyStage::Stage3 => "Stage3",
        DelinquencyStage::NonPerforming => "NonPerforming",
        DelinquencyStage::WriteOff => "WriteOff",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_delinquency_stage<'de, D>(deserializer: D) -> Result<DelinquencyStage, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Current" => Ok(DelinquencyStage::Current),
        "Stage1" => Ok(DelinquencyStage::Stage1),
        "Stage2" => Ok(DelinquencyStage::Stage2),
        "Stage3" => Ok(DelinquencyStage::Stage3),
        "NonPerforming" => Ok(DelinquencyStage::NonPerforming),
        "WriteOff" => Ok(DelinquencyStage::WriteOff),
        _ => Err(serde::de::Error::unknown_variant(&s, &["Current", "Stage1", "Stage2", "Stage3", "NonPerforming", "WriteOff"])),
    }
}

fn serialize_collection_action_type<S>(value: &CollectionActionType, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        CollectionActionType::EmailReminder => "EmailReminder",
        CollectionActionType::SmsNotification => "SmsNotification",
        CollectionActionType::PhoneCall => "PhoneCall",
        CollectionActionType::LetterNotice => "LetterNotice",
        CollectionActionType::LegalNotice => "LegalNotice",
        CollectionActionType::FieldVisit => "FieldVisit",
        CollectionActionType::PaymentPlan => "PaymentPlan",
        CollectionActionType::Restructuring => "Restructuring",
        CollectionActionType::LegalAction => "LegalAction",
        CollectionActionType::AssetRecovery => "AssetRecovery",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_collection_action_type<'de, D>(deserializer: D) -> Result<CollectionActionType, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "EmailReminder" => Ok(CollectionActionType::EmailReminder),
        "SmsNotification" => Ok(CollectionActionType::SmsNotification),
        "PhoneCall" => Ok(CollectionActionType::PhoneCall),
        "LetterNotice" => Ok(CollectionActionType::LetterNotice),
        "LegalNotice" => Ok(CollectionActionType::LegalNotice),
        "FieldVisit" => Ok(CollectionActionType::FieldVisit),
        "PaymentPlan" => Ok(CollectionActionType::PaymentPlan),
        "Restructuring" => Ok(CollectionActionType::Restructuring),
        "LegalAction" => Ok(CollectionActionType::LegalAction),
        "AssetRecovery" => Ok(CollectionActionType::AssetRecovery),
        _ => Err(serde::de::Error::unknown_variant(&s, &["EmailReminder", "SmsNotification", "PhoneCall", "LetterNotice", "LegalNotice", "FieldVisit", "PaymentPlan", "Restructuring", "LegalAction", "AssetRecovery"])),
    }
}

fn serialize_action_status<S>(value: &ActionStatus, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        ActionStatus::Planned => "Planned",
        ActionStatus::InProgress => "InProgress",
        ActionStatus::Completed => "Completed",
        ActionStatus::Failed => "Failed",
        ActionStatus::Cancelled => "Cancelled",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_action_status<'de, D>(deserializer: D) -> Result<ActionStatus, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Planned" => Ok(ActionStatus::Planned),
        "InProgress" => Ok(ActionStatus::InProgress),
        "Completed" => Ok(ActionStatus::Completed),
        "Failed" => Ok(ActionStatus::Failed),
        "Cancelled" => Ok(ActionStatus::Cancelled),
        _ => Err(serde::de::Error::unknown_variant(&s, &["Planned", "InProgress", "Completed", "Failed", "Cancelled"])),
    }
}

fn serialize_payment_type<S>(value: &PaymentType, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        PaymentType::Regular => "Regular",
        PaymentType::Prepayment => "Prepayment",
        PaymentType::PartialPayment => "PartialPayment",
        PaymentType::Restructured => "Restructured",
        PaymentType::Settlement => "Settlement",
        PaymentType::Recovery => "Recovery",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_payment_type<'de, D>(deserializer: D) -> Result<PaymentType, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Regular" => Ok(PaymentType::Regular),
        "Prepayment" => Ok(PaymentType::Prepayment),
        "PartialPayment" => Ok(PaymentType::PartialPayment),
        "Restructured" => Ok(PaymentType::Restructured),
        "Settlement" => Ok(PaymentType::Settlement),
        "Recovery" => Ok(PaymentType::Recovery),
        _ => Err(serde::de::Error::unknown_variant(&s, &["Regular", "Prepayment", "PartialPayment", "Restructured", "Settlement", "Recovery"])),
    }
}

fn serialize_payment_method<S>(value: &PaymentMethod, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        PaymentMethod::BankTransfer => "BankTransfer",
        PaymentMethod::DirectDebit => "DirectDebit",
        PaymentMethod::Check => "Check",
        PaymentMethod::Cash => "Cash",
        PaymentMethod::OnlinePayment => "OnlinePayment",
        PaymentMethod::MobilePayment => "MobilePayment",
        PaymentMethod::StandingInstruction => "StandingInstruction",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_payment_method<'de, D>(deserializer: D) -> Result<PaymentMethod, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "BankTransfer" => Ok(PaymentMethod::BankTransfer),
        "DirectDebit" => Ok(PaymentMethod::DirectDebit),
        "Check" => Ok(PaymentMethod::Check),
        "Cash" => Ok(PaymentMethod::Cash),
        "OnlinePayment" => Ok(PaymentMethod::OnlinePayment),
        "MobilePayment" => Ok(PaymentMethod::MobilePayment),
        "StandingInstruction" => Ok(PaymentMethod::StandingInstruction),
        _ => Err(serde::de::Error::unknown_variant(&s, &["BankTransfer", "DirectDebit", "Check", "Cash", "OnlinePayment", "MobilePayment", "StandingInstruction"])),
    }
}

fn serialize_prepayment_type<S>(value: &PrepaymentType, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        PrepaymentType::TermReduction => "TermReduction",
        PrepaymentType::InstallmentReduction => "InstallmentReduction",
        PrepaymentType::HoldInSuspense => "HoldInSuspense",
        PrepaymentType::Refund => "Refund",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_prepayment_type<'de, D>(deserializer: D) -> Result<PrepaymentType, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "TermReduction" => Ok(PrepaymentType::TermReduction),
        "InstallmentReduction" => Ok(PrepaymentType::InstallmentReduction),
        "HoldInSuspense" => Ok(PrepaymentType::HoldInSuspense),
        "Refund" => Ok(PrepaymentType::Refund),
        _ => Err(serde::de::Error::unknown_variant(&s, &["TermReduction", "InstallmentReduction", "HoldInSuspense", "Refund"])),
    }
}

fn serialize_payment_status<S>(value: &PaymentStatus, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        PaymentStatus::Processed => "Processed",
        PaymentStatus::Pending => "Pending",
        PaymentStatus::Failed => "Failed",
        PaymentStatus::Reversed => "Reversed",
        PaymentStatus::PartiallyAllocated => "PartiallyAllocated",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_payment_status<'de, D>(deserializer: D) -> Result<PaymentStatus, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Processed" => Ok(PaymentStatus::Processed),
        "Pending" => Ok(PaymentStatus::Pending),
        "Failed" => Ok(PaymentStatus::Failed),
        "Reversed" => Ok(PaymentStatus::Reversed),
        "PartiallyAllocated" => Ok(PaymentStatus::PartiallyAllocated),
        _ => Err(serde::de::Error::unknown_variant(&s, &["Processed", "Pending", "Failed", "Reversed", "PartiallyAllocated"])),
    }
}

fn serialize_restructuring_type<S>(value: &RestructuringType, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        RestructuringType::PaymentHoliday => "PaymentHoliday",
        RestructuringType::TermExtension => "TermExtension",
        RestructuringType::RateReduction => "RateReduction",
        RestructuringType::PrincipalReduction => "PrincipalReduction",
        RestructuringType::InstallmentReduction => "InstallmentReduction",
        RestructuringType::InterestCapitalization => "InterestCapitalization",
        RestructuringType::FullRestructuring => "FullRestructuring",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_restructuring_type<'de, D>(deserializer: D) -> Result<RestructuringType, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "PaymentHoliday" => Ok(RestructuringType::PaymentHoliday),
        "TermExtension" => Ok(RestructuringType::TermExtension),
        "RateReduction" => Ok(RestructuringType::RateReduction),
        "PrincipalReduction" => Ok(RestructuringType::PrincipalReduction),
        "InstallmentReduction" => Ok(RestructuringType::InstallmentReduction),
        "InterestCapitalization" => Ok(RestructuringType::InterestCapitalization),
        "FullRestructuring" => Ok(RestructuringType::FullRestructuring),
        _ => Err(serde::de::Error::unknown_variant(&s, &["PaymentHoliday", "TermExtension", "RateReduction", "PrincipalReduction", "InstallmentReduction", "InterestCapitalization", "FullRestructuring"])),
    }
}

fn serialize_loan_approval_status<S>(value: &LoanApprovalStatus, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        LoanApprovalStatus::Pending => "Pending",
        LoanApprovalStatus::Approved => "Approved",
        LoanApprovalStatus::Rejected => "Rejected",
        LoanApprovalStatus::ConditionallyApproved => "ConditionallyApproved",
        LoanApprovalStatus::RequiresCommitteeApproval => "RequiresCommitteeApproval",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_loan_approval_status<'de, D>(deserializer: D) -> Result<LoanApprovalStatus, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Pending" => Ok(LoanApprovalStatus::Pending),
        "Approved" => Ok(LoanApprovalStatus::Approved),
        "Rejected" => Ok(LoanApprovalStatus::Rejected),
        "ConditionallyApproved" => Ok(LoanApprovalStatus::ConditionallyApproved),
        "RequiresCommitteeApproval" => Ok(LoanApprovalStatus::RequiresCommitteeApproval),
        _ => Err(serde::de::Error::unknown_variant(&s, &["Pending", "Approved", "Rejected", "ConditionallyApproved", "RequiresCommitteeApproval"])),
    }
}
use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::customer::{KycStatus, RiskRating};
use crate::domain::transaction::TransactionType;

/// CASA (Current & Savings Account) specialized functionality
/// Building upon the Unified Account Model
/// Overdraft facility configuration and management
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct OverdraftFacility {
    pub id: Uuid,
    pub account_id: Uuid,
    pub approved_limit: Decimal,
    pub current_utilized: Decimal,
    pub available_limit: Decimal,
    pub interest_rate: Decimal, // Debit interest rate for overdrawn amounts
    pub facility_status: OverdraftStatus,
    pub approval_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub approved_by_person_id: Uuid, // References Person.person_id
    pub review_frequency: ReviewFrequency,
    pub next_review_date: NaiveDate,
    pub security_required: bool,
    pub security_details: Option<HeaplessString<200>>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OverdraftStatus {
    Active,
    Suspended,
    Expired,
    UnderReview,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReviewFrequency {
    Monthly,
    Quarterly,
    SemiAnnually,
    Annually,
}

/// Overdraft utilization tracking for interest calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverdraftUtilization {
    pub id: Uuid,
    pub account_id: Uuid,
    pub utilization_date: NaiveDate,
    pub opening_balance: Decimal, // Negative for overdrawn
    pub closing_balance: Decimal, // Negative for overdrawn
    pub average_daily_balance: Decimal,
    pub days_overdrawn: u32,
    pub interest_accrued: Decimal,
    pub interest_rate: Decimal,
    pub capitalized: bool,
    pub capitalization_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

/// Overdraft interest calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverdraftInterestCalculation {
    pub id: Uuid,
    pub account_id: Uuid,
    pub calculation_period_start: NaiveDate,
    pub calculation_period_end: NaiveDate,
    pub average_overdrawn_balance: Decimal,
    pub applicable_rate: Decimal,
    pub days_calculated: u32,
    pub interest_amount: Decimal,
    pub compounding_frequency: CompoundingFrequency,
    pub capitalization_due: bool,
    pub calculated_at: DateTime<Utc>,
    pub calculated_by_person_id: Uuid, // References Person.person_id
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompoundingFrequency {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
}

/// CASA account summary for comprehensive reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CasaAccountSummary {
    pub account_id: Uuid,
    pub account_type: super::AccountType,
    pub current_balance: Decimal,
    pub available_balance: Decimal,
    
    // Overdraft information (for current accounts)
    pub overdraft_facility: Option<OverdraftFacility>,
    pub is_overdrawn: bool,
    pub overdrawn_amount: Option<Decimal>,
    pub days_overdrawn: Option<u32>,
    
    // Interest information
    pub credit_interest_accrued: Decimal, // Positive balances
    pub debit_interest_accrued: Decimal,  // Overdrawn balances
    pub last_interest_posting_date: Option<NaiveDate>,
    pub next_interest_posting_date: Option<NaiveDate>,
    
    // Activity summary
    pub last_transaction_date: Option<NaiveDate>,
    pub mtd_transaction_count: u32,
    pub mtd_debit_amount: Decimal,
    pub mtd_credit_amount: Decimal,
    
    // Account status
    pub dormancy_risk: DormancyRisk,
    pub compliance_status: CasaComplianceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DormancyRisk {
    Active,
    AtRisk,
    Dormant,
    Inactive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CasaComplianceStatus {
    pub kyc_status: KycStatus,
    pub last_kyc_update: Option<NaiveDate>,
    pub aml_risk_rating: RiskRating,
    pub regulatory_alerts_01: HeaplessString<200>,
    pub regulatory_alerts_02: HeaplessString<200>,
    pub regulatory_alerts_03: HeaplessString<200>,
    pub regulatory_alerts_04: HeaplessString<200>,
    pub regulatory_alerts_05: HeaplessString<200>,
}

/// Daily overdraft processing job for EOD
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverdraftProcessingJob {
    pub id: Uuid,
    pub processing_date: NaiveDate,
    pub accounts_processed: u32,
    pub total_interest_accrued: Decimal,
    pub accounts_capitalized: u32,
    pub total_capitalized_amount: Decimal,
    pub status: ProcessingJobStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub errors_01: HeaplessString<200>,
    pub errors_02: HeaplessString<200>,
    pub errors_03: HeaplessString<200>,
    pub errors_04: HeaplessString<200>,
    pub errors_05: HeaplessString<200>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessingJobStatus {
    Scheduled,
    Running,
    Completed,
    Failed,
    PartiallyCompleted,
}

/// Overdraft limit adjustment request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct OverdraftLimitAdjustment {
    pub id: Uuid,
    pub account_id: Uuid,
    pub current_limit: Decimal,
    pub requested_limit: Decimal,
    /// References ReasonAndPurpose.id for adjustment reason
    pub adjustment_reason_id: Uuid,
    /// Additional context for adjustment
    pub additional_details: Option<HeaplessString<200>>,
    pub required_document01_id: Option<Uuid>,
    pub required_document02_id: Option<Uuid>,
    pub required_document03_id: Option<Uuid>,
    pub required_document04_id: Option<Uuid>,
    pub required_document05_id: Option<Uuid>,
    pub required_document06_id: Option<Uuid>,
    pub required_document07_id: Option<Uuid>,
    pub requested_by_person_id: Uuid, // References Person.person_id
    pub requested_at: DateTime<Utc>,
    pub approval_status: CasaApprovalStatus,
    pub approved_by_person_id: Option<Uuid>, // References Person.person_id
    pub approved_at: Option<DateTime<Utc>>,
    pub approval_notes: Option<HeaplessString<500>>,
    pub effective_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CasaApprovalStatus {
    Pending,
    Approved,
    Rejected,
    RequiresAdditionalDocuments,
    UnderReview,
}

/// Transaction validation context for CASA accounts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CasaTransactionValidation {
    pub account_id: Uuid,
    pub transaction_amount: Decimal,
    pub transaction_type: TransactionType,
    pub current_balance: Decimal,
    pub available_balance: Decimal,
    pub overdraft_limit: Option<Decimal>,
    pub post_transaction_balance: Decimal,
    pub overdraft_utilization: Option<Decimal>,
    pub validation_result: CasaValidationResult,
    pub validation_message_01: HeaplessString<200>,
    pub validation_message_02: HeaplessString<200>,
    pub validation_message_03: HeaplessString<200>,
    pub requires_authorization: bool,
    pub authorization_level: Option<AuthorizationLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CasaValidationResult {
    Approved,
    Rejected,
    RequiresAuthorization,
    RequiresHoldRelease,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthorizationLevel {
    Teller,
    Supervisor,
    Manager,
    CreditCommittee,
}

/// Interest posting record for CASA accounts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterestPostingRecord {
    pub id: Uuid,
    pub account_id: Uuid,
    pub posting_date: NaiveDate,
    pub interest_type: InterestType,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub principal_amount: Decimal,
    pub interest_rate: Decimal,
    pub days_calculated: u32,
    pub interest_amount: Decimal,
    pub tax_withheld: Option<Decimal>,
    pub net_amount: Decimal,
    pub posting_status: PostingStatus,
    pub posted_by_person_id: Uuid, // References Person.person_id
    pub posted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterestType {
    CreditInterest,  // Positive balance interest
    DebitInterest,   // Overdraft interest
    PenaltyInterest, // Additional penalty
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PostingStatus {
    Calculated,
    Posted,
    Reversed,
    Adjusted,
}

/// Request parameters for creating an overdraft facility
#[derive(Debug, Clone, Validate)]
pub struct CreateOverdraftFacilityRequest {
    pub account_id: Uuid,
    pub approved_limit: Decimal,
    pub interest_rate: Decimal,
    pub approved_by_person_id: Uuid, // References Person.person_id
    pub expiry_date: Option<NaiveDate>,
    pub security_required: bool,
    pub security_details: Option<HeaplessString<200>>,
}
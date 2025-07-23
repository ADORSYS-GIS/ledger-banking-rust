use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// CASA (Current & Savings Account) specialized functionality
/// Building upon the Unified Account Model
/// Overdraft facility configuration and management
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct OverdraftFacility {
    pub facility_id: Uuid,
    pub account_id: Uuid,
    pub approved_limit: Decimal,
    pub current_utilized: Decimal,
    pub available_limit: Decimal,
    pub interest_rate: Decimal, // Debit interest rate for overdrawn amounts
    pub facility_status: OverdraftStatus,
    pub approval_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    #[validate(length(max = 100))]
    pub approved_by: String,
    pub review_frequency: ReviewFrequency,
    pub next_review_date: NaiveDate,
    pub security_required: bool,
    #[validate(length(max = 255))]
    pub security_details: Option<String>,
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
    pub utilization_id: Uuid,
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
    pub calculation_id: Uuid,
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
    pub calculated_by: String,
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
    pub kyc_status: String,
    pub last_kyc_update: Option<NaiveDate>,
    pub aml_risk_rating: String,
    pub regulatory_alerts: Vec<String>,
}

/// Daily overdraft processing job for EOD
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverdraftProcessingJob {
    pub job_id: Uuid,
    pub processing_date: NaiveDate,
    pub accounts_processed: u32,
    pub total_interest_accrued: Decimal,
    pub accounts_capitalized: u32,
    pub total_capitalized_amount: Decimal,
    pub status: ProcessingJobStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub errors: Vec<String>,
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
    pub adjustment_id: Uuid,
    pub account_id: Uuid,
    pub current_limit: Decimal,
    pub requested_limit: Decimal,
    pub adjustment_reason: String,
    pub supporting_documents: Vec<String>,
    pub requested_by: String,
    pub requested_at: DateTime<Utc>,
    pub approval_status: CasaApprovalStatus,
    pub approved_by: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
    pub approval_notes: Option<String>,
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
    pub transaction_type: String, // Debit, Credit
    pub current_balance: Decimal,
    pub available_balance: Decimal,
    pub overdraft_limit: Option<Decimal>,
    pub post_transaction_balance: Decimal,
    pub overdraft_utilization: Option<Decimal>,
    pub validation_result: CasaValidationResult,
    pub validation_messages: Vec<String>,
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
    pub posting_id: Uuid,
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
    pub posted_by: String,
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
    #[validate(length(max = 100))]
    pub approved_by: String,
    pub expiry_date: Option<NaiveDate>,
    pub security_required: bool,
    #[validate(length(max = 255))]
    pub security_details: Option<String>,
}
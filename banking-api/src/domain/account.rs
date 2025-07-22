use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Account {
    pub account_id: Uuid,
    #[validate(length(min = 1, max = 50))]
    pub product_code: String,
    pub account_type: AccountType,
    pub account_status: AccountStatus,
    pub signing_condition: SigningCondition,
    #[validate(length(min = 3, max = 3))]
    pub currency: String,
    pub open_date: NaiveDate,
    pub domicile_branch_id: Uuid,
    
    // Balance fields
    pub current_balance: Decimal,
    pub available_balance: Decimal,
    pub accrued_interest: Decimal,
    pub overdraft_limit: Option<Decimal>,
    
    // Loan-specific fields (nullable for non-loan accounts)
    pub original_principal: Option<Decimal>,
    pub outstanding_principal: Option<Decimal>,
    pub loan_interest_rate: Option<Decimal>,
    pub loan_term_months: Option<i32>,
    pub disbursement_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    pub installment_amount: Option<Decimal>,
    pub next_due_date: Option<NaiveDate>,
    pub penalty_rate: Option<Decimal>,
    #[validate(length(max = 100))]
    pub collateral_id: Option<String>,
    #[validate(length(max = 255))]
    pub loan_purpose: Option<String>,

    // Account lifecycle management (from enhancements)
    pub close_date: Option<NaiveDate>,
    pub last_activity_date: Option<NaiveDate>,
    pub dormancy_threshold_days: Option<i32>,
    pub reactivation_required: bool,
    #[validate(length(max = 255))]
    pub pending_closure_reason: Option<String>,
    pub disbursement_instructions: Option<DisbursementInstructions>,
    
    // Enhanced audit trail
    #[validate(length(max = 100))]
    pub status_changed_by: Option<String>,
    #[validate(length(max = 255))]
    pub status_change_reason: Option<String>,
    pub status_change_timestamp: Option<DateTime<Utc>>,
    
    // Audit fields
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    #[validate(length(min = 1, max = 100))]
    pub updated_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountType { 
    Savings, 
    Current, 
    Loan 
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AccountStatus { 
    PendingApproval,
    Active, 
    Dormant,
    Frozen,
    PendingClosure,
    Closed,
    PendingReactivation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SigningCondition { 
    None,
    AnyOwner,
    AllOwners
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DisbursementInstructions {
    pub method: DisbursementMethod,
    pub target_account: Option<Uuid>,
    #[validate(length(max = 255))]
    pub cash_pickup_location: Option<String>,
    #[validate(length(max = 100))]
    pub authorized_recipient: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisbursementMethod {
    Transfer,
    CashWithdrawal,
    Check,
    HoldFunds,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AccountHold {
    pub hold_id: Uuid,
    pub account_id: Uuid,
    pub amount: Decimal,
    pub hold_type: HoldType,
    #[validate(length(max = 255))]
    pub reason: String,
    pub placed_by: String,
    pub placed_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: HoldStatus,
    pub released_at: Option<DateTime<Utc>>,
    pub released_by: Option<String>,
    pub priority: HoldPriority,
    pub source_reference: Option<String>, // External reference for judicial holds, etc.
    pub automatic_release: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HoldStatus {
    Active,
    Released,
    Expired,
    Cancelled,
    PartiallyReleased,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HoldPriority {
    /// Must be honored first (judicial, regulatory)
    Critical,
    /// Standard business hold
    High,
    /// Lower priority administrative hold
    Medium,
    /// Lowest priority, can be overridden
    Low,
}

/// Real-time balance calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceCalculation {
    pub account_id: Uuid,
    pub current_balance: Decimal,
    pub available_balance: Decimal,
    pub overdraft_limit: Option<Decimal>,
    pub total_holds: Decimal,
    pub active_hold_count: u32,
    pub calculation_timestamp: DateTime<Utc>,
    pub hold_breakdown: Vec<HoldSummary>,
}

/// Summary of hold amounts by type for balance calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldSummary {
    pub hold_type: HoldType,
    pub total_amount: Decimal,
    pub hold_count: u32,
    pub priority: HoldPriority,
}

/// Hold release request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct HoldReleaseRequest {
    pub hold_id: Uuid,
    pub release_amount: Option<Decimal>, // For partial releases
    pub release_reason: String,
    pub released_by: String,
    pub override_authorization: bool,
}

/// Batch hold processing for expired holds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldExpiryJob {
    pub job_id: Uuid,
    pub processing_date: NaiveDate,
    pub expired_holds_count: u32,
    pub total_released_amount: Decimal,
    pub processed_at: DateTime<Utc>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusChangeRecord {
    pub change_id: Uuid,
    pub account_id: Uuid,
    pub old_status: Option<AccountStatus>,
    pub new_status: AccountStatus,
    pub reason: String,
    pub changed_by: String,
    pub changed_at: DateTime<Utc>,
    pub system_triggered: bool,
}
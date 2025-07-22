use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Database model for Account table with enhanced fields from banking enhancements
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountModel {
    pub account_id: Uuid,
    pub product_code: String,
    pub account_type: String,
    pub account_status: String,
    pub signing_condition: String,
    pub currency: String,
    pub open_date: NaiveDate,
    pub domicile_branch_id: Uuid,
    
    // Balance fields
    pub current_balance: Decimal,
    pub available_balance: Decimal,
    pub accrued_interest: Decimal,
    pub overdraft_limit: Option<Decimal>,
    
    // Loan-specific fields
    pub original_principal: Option<Decimal>,
    pub outstanding_principal: Option<Decimal>,
    pub loan_interest_rate: Option<Decimal>,
    pub loan_term_months: Option<i32>,
    pub disbursement_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    pub installment_amount: Option<Decimal>,
    pub next_due_date: Option<NaiveDate>,
    pub penalty_rate: Option<Decimal>,
    pub collateral_id: Option<String>,
    pub loan_purpose: Option<String>,

    // Account lifecycle management (from enhancements)
    pub close_date: Option<NaiveDate>,
    pub last_activity_date: Option<NaiveDate>,
    pub dormancy_threshold_days: Option<i32>,
    pub reactivation_required: bool,
    pub pending_closure_reason: Option<String>,
    
    // Enhanced audit trail
    pub status_changed_by: Option<String>,
    pub status_change_reason: Option<String>,
    pub status_change_timestamp: Option<DateTime<Utc>>,
    
    // Audit fields
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: String,
}

/// Database model for Account Ownership
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountOwnershipModel {
    pub ownership_id: Uuid,
    pub account_id: Uuid,
    pub customer_id: Uuid,
    pub ownership_type: String,
    pub ownership_percentage: Option<Decimal>,
    pub created_at: DateTime<Utc>,
}

/// Database model for Account Relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountRelationshipModel {
    pub relationship_id: Uuid,
    pub account_id: Uuid,
    pub entity_id: Uuid,
    pub entity_type: String,
    pub relationship_type: String,
    pub status: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

/// Database model for Account Mandates
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountMandateModel {
    pub mandate_id: Uuid,
    pub account_id: Uuid,
    pub grantee_customer_id: Uuid,
    pub permission_type: String,
    pub transaction_limit: Option<Decimal>,
    pub approval_group_id: Option<Uuid>,
    pub status: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

/// Database model for Account Holds
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountHoldModel {
    pub hold_id: Uuid,
    pub account_id: Uuid,
    pub amount: Decimal,
    pub reason: String,
    pub placed_by: String,
    pub placed_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: String,
    pub released_by: Option<String>,
    pub released_at: Option<DateTime<Utc>>,
}

/// Database model for Account Status History (from enhancements)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountStatusHistoryModel {
    pub history_id: Uuid,
    pub account_id: Uuid,
    pub old_status: Option<String>,
    pub new_status: String,
    pub change_reason: String,
    pub changed_by: String,
    pub changed_at: DateTime<Utc>,
    pub system_triggered: bool,
    pub created_at: DateTime<Utc>,
}

/// Database model for Final Settlements (from enhancements)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountFinalSettlementModel {
    pub settlement_id: Uuid,
    pub account_id: Uuid,
    pub settlement_date: NaiveDate,
    pub current_balance: Decimal,
    pub accrued_interest: Decimal,
    pub closure_fees: Decimal,
    pub final_amount: Decimal,
    pub disbursement_method: String,
    pub disbursement_reference: Option<String>,
    pub processed_by: String,
    pub created_at: DateTime<Utc>,
}

/// Database model for Final Settlement (alias for compatibility)
pub type FinalSettlementModel = AccountFinalSettlementModel;

/// Database model for Status Change (alias for compatibility) 
pub type StatusChangeModel = AccountStatusHistoryModel;
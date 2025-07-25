use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// View model that includes resolved reason text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonView {
    pub id: Uuid,
    pub code: String,
    pub text: String,  // Resolved based on user's language preference
    pub requires_details: bool,
    pub additional_details: Option<String>,
    pub severity: Option<String>,
    pub category: String,
    pub context: String,
}

impl ReasonView {
    // This method will be implemented when banking-db integration is complete
    // pub fn from_reason_and_purpose(
    //     reason: &ReasonAndPurpose, 
    //     language_code: &[u8; 3],
    //     additional_details: Option<&str>
    // ) -> Self
    
    // This method will be implemented when banking-db integration is complete
    // pub fn from_reason_with_fallback(
    //     reason: &ReasonAndPurpose,
    //     preferred_languages: &[[u8; 3]],
    //     additional_details: Option<&str>
    // ) -> Self
}

/// Account view model with resolved reasons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountView {
    pub account_id: Uuid,
    pub product_code: String,
    pub account_type: String,
    pub account_status: String,
    pub currency: String,
    pub current_balance: rust_decimal::Decimal,
    pub available_balance: rust_decimal::Decimal,
    
    // Resolved reason fields
    pub loan_purpose: Option<ReasonView>,
    pub pending_closure_reason: Option<ReasonView>,
    pub status_change_reason: Option<ReasonView>,
    
    // Other fields...
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Account hold view model with resolved reason
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountHoldView {
    pub hold_id: Uuid,
    pub account_id: Uuid,
    pub amount: rust_decimal::Decimal,
    pub hold_type: String,
    pub status: String,
    pub placed_by: Uuid, // References ReferencedPerson.person_id
    pub placed_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    
    // Resolved reason
    pub reason: ReasonView,
    
    pub released_at: Option<chrono::DateTime<chrono::Utc>>,
    pub released_by: Option<Uuid>, // References ReferencedPerson.person_id
}

/// Fee waiver view model with resolved reason
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeWaiverView {
    pub waiver_id: Uuid,
    pub fee_application_id: Uuid,
    pub account_id: Uuid,
    pub waived_amount: rust_decimal::Decimal,
    pub waived_by: Uuid, // References ReferencedPerson.person_id
    pub waived_at: chrono::DateTime<chrono::Utc>,
    
    // Resolved reason
    pub reason: ReasonView,
    
    pub approval_required: bool,
    pub approved_by: Option<Uuid>, // References ReferencedPerson.person_id
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Transaction audit view model with resolved reason
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionAuditView {
    pub audit_id: Uuid,
    pub transaction_id: Uuid,
    pub action_type: String,
    pub performed_by: Uuid, // References ReferencedPerson.person_id
    pub performed_at: chrono::DateTime<chrono::Utc>,
    pub old_status: Option<String>,
    pub new_status: Option<String>,
    
    // Resolved reason
    pub reason: Option<ReasonView>,
}

/// SAR (Suspicious Activity Report) view model with resolved reason
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarDataView {
    pub sar_id: Uuid,
    pub customer_id: Uuid,
    pub supporting_transactions: Vec<Uuid>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    
    // Resolved reason
    pub reason: ReasonView,
}

/// Loan restructuring view model with resolved reason
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanRestructuringView {
    pub restructuring_id: Uuid,
    pub loan_account_id: Uuid,
    pub restructuring_type: String,
    pub request_date: chrono::NaiveDate,
    pub effective_date: Option<chrono::NaiveDate>,
    
    // Resolved reason
    pub restructuring_reason: ReasonView,
    
    // Original loan terms
    pub original_principal: rust_decimal::Decimal,
    pub original_interest_rate: rust_decimal::Decimal,
    pub original_term_months: u32,
    pub original_installment: rust_decimal::Decimal,
    
    // New loan terms
    pub new_principal: Option<rust_decimal::Decimal>,
    pub new_interest_rate: Option<rust_decimal::Decimal>,
    pub new_term_months: Option<u32>,
    pub new_installment: Option<rust_decimal::Decimal>,
}

/// Workflow approval view model with resolved rejection reason
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowApprovalView {
    pub workflow_id: Uuid,
    pub account_id: Uuid,
    pub workflow_type: String,
    pub current_step: String,
    pub status: String,
    pub initiated_by: Uuid, // References ReferencedPerson.person_id
    pub initiated_at: chrono::DateTime<chrono::Utc>,
    pub timeout_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    
    // Resolved rejection reason
    pub rejection_reason: Option<ReasonView>,
}
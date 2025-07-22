use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Transaction {
    pub transaction_id: Uuid,
    pub account_id: Uuid,
    #[validate(length(min = 1, max = 20))]
    pub transaction_code: String,
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    #[validate(length(min = 3, max = 3))]
    pub currency: String,
    #[validate(length(min = 1, max = 500))]
    pub description: String,
    #[validate(length(min = 1, max = 50))]
    pub channel_id: String,
    pub terminal_id: Option<Uuid>,
    pub agent_user_id: Option<Uuid>,
    pub transaction_date: DateTime<Utc>,
    pub value_date: NaiveDate,
    pub status: TransactionStatus,
    #[validate(length(min = 1, max = 100))]
    pub reference_number: String,
    #[validate(length(max = 100))]
    pub external_reference: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub gl_code: String,
    pub requires_approval: bool,
    pub approval_status: Option<TransactionApprovalStatus>,
    pub risk_score: Option<Decimal>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionType { 
    Credit, 
    Debit 
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus { 
    Pending, 
    Posted, 
    Reversed, 
    Failed,
    AwaitingApproval,
    ApprovalRejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionApprovalStatus { 
    Pending, 
    Approved, 
    Rejected, 
    PartiallyApproved 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRequest {
    pub account_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    pub currency: String,
    pub description: String,
    pub channel: ChannelType,
    pub terminal_id: Option<Uuid>,
    pub initiator_id: String,
    pub external_reference: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResult {
    pub transaction_id: Uuid,
    pub reference_number: String,
    pub gl_entries: Vec<GlEntry>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn new(is_valid: bool, errors: Vec<String>, warnings: Vec<String>) -> Self {
        Self { is_valid, errors, warnings }
    }
    
    pub fn success() -> Self {
        Self::new(true, vec![], vec![])
    }
    
    pub fn failure(errors: Vec<String>) -> Self {
        Self::new(false, errors, vec![])
    }
    
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
    
    pub fn get_failure_reasons(&self) -> Vec<String> {
        self.errors.clone()
    }
    
    pub fn add_check(&mut self, field: &str, is_valid: bool, message: String) {
        if !is_valid {
            self.errors.push(format!("{}: {}", field, message));
            self.is_valid = false;
        } else {
            self.warnings.push(format!("{}: {}", field, message));
        }
    }
    
    pub fn merge(&mut self, other: ValidationResult) {
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
        self.is_valid = self.is_valid && other.is_valid;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlEntry {
    pub entry_id: Uuid,
    pub account_code: String,
    pub debit_amount: Option<Decimal>,
    pub credit_amount: Option<Decimal>,
    pub currency: String,
    pub description: String,
    pub reference_number: String,
    pub transaction_id: Uuid,
    pub value_date: NaiveDate,
    pub posting_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalWorkflow {
    pub workflow_id: Uuid,
    pub transaction_id: Uuid,
    pub required_approvers: Vec<Uuid>,
    pub received_approvals: Vec<Approval>,
    pub status: TransactionWorkflowStatus,
    pub timeout_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    pub approval_id: Uuid,
    pub approver_id: Uuid,
    pub approved_at: DateTime<Utc>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionWorkflowStatus { 
    Pending, 
    Approved, 
    Rejected, 
    TimedOut 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelType {
    MobileApp,
    AgentTerminal,
    ATM,
    InternetBanking,
    BranchTeller,
    USSD,
    ApiGateway,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermittedOperation {
    Credit,
    Debit,
    InterestPosting,
    FeeApplication,
    ClosureSettlement,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionAudit {
    pub audit_id: Uuid,
    pub transaction_id: Uuid,
    pub action_type: String,
    pub performed_by: String,
    pub performed_at: DateTime<Utc>,
    pub old_status: Option<String>,
    pub new_status: Option<String>,
    pub reason: Option<String>,
    pub details: Option<String>,
}
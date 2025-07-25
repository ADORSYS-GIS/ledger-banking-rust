use blake3::Hash;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AccountWorkflow {
    pub workflow_id: Uuid,
    pub account_id: Uuid,
    pub workflow_type: WorkflowType,
    pub current_step: WorkflowStep,
    pub status: WorkflowStatus,
    #[validate(length(min = 1, max = 100))]
    pub initiated_by: String,
    pub initiated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub steps_completed: Vec<WorkflowStepRecord>,
    #[validate(length(max = 500))]
    pub next_action_required: Option<String>,
    pub timeout_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowType {
    AccountOpening,
    AccountClosure,
    AccountReactivation,
    ComplianceVerification,
    MultiPartyApproval,
}

impl std::fmt::Display for WorkflowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkflowType::AccountOpening => write!(f, "AccountOpening"),
            WorkflowType::AccountClosure => write!(f, "AccountClosure"),
            WorkflowType::AccountReactivation => write!(f, "AccountReactivation"),
            WorkflowType::ComplianceVerification => write!(f, "ComplianceVerification"),
            WorkflowType::MultiPartyApproval => write!(f, "MultiPartyApproval"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WorkflowStep {
    InitiateRequest,
    ComplianceCheck,
    DocumentVerification,
    ApprovalRequired,
    FinalSettlement,
    Completed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WorkflowStatus {
    InProgress,
    PendingAction,
    Completed,
    Failed,
    Cancelled,
    TimedOut,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WorkflowStepRecord {
    pub step: WorkflowStep,
    pub completed_at: DateTime<Utc>,
    #[validate(length(min = 1, max = 100))]
    pub completed_by: String,
    #[validate(length(max = 1000))]
    pub notes: Option<String>,
    pub supporting_documents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AccountOpeningRequest {
    pub customer_id: Uuid,
    #[validate(length(min = 1, max = 50))]
    pub product_code: String,
    pub initial_deposit: Option<Decimal>,
    #[validate(length(min = 1, max = 50))]
    pub channel: String,
    #[validate(length(min = 1, max = 100))]
    pub initiated_by: String,
    pub supporting_documents: Vec<DocumentReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ClosureRequest {
    pub reason: ClosureReason,
    pub disbursement_instructions: super::account::DisbursementInstructions,
    #[validate(length(min = 1, max = 100))]
    pub requested_by: String,
    pub force_closure: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClosureReason {
    CustomerRequest,
    Regulatory,
    Compliance,
    Dormancy,
    SystemMaintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalSettlement {
    pub current_balance: Decimal,
    pub accrued_interest: Decimal,
    pub pending_fees: Decimal,
    pub closure_fees: Decimal,
    pub final_amount: Decimal,
    pub requires_disbursement: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DormancyAssessment {
    pub is_eligible: bool,
    pub last_activity_date: Option<chrono::NaiveDate>,
    pub days_inactive: i32,
    pub threshold_days: i32,
    pub product_specific_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DocumentReference {
    pub document_id: Hash,
    #[validate(length(min = 1, max = 50))]
    pub document_type: String,
    pub document_path: Option<Hash>,
}

impl DocumentReference {
    /// Create new document reference with hash-based ID
    pub fn new(document_type: String, content: &[u8]) -> Self {
        Self {
            document_id: blake3::hash(content),
            document_type,
            document_path: None,
        }
    }
    
    /// Create document reference from path content
    pub fn with_path(document_type: String, content: &[u8], path_content: &[u8]) -> Self {
        Self {
            document_id: blake3::hash(content),
            document_type,
            document_path: Some(blake3::hash(path_content)),
        }
    }
    
    /// Get document ID as hex string for display/logging
    pub fn document_id_hex(&self) -> String {
        self.document_id.to_hex().to_string()
    }
    
    /// Get document path as hex string if available
    pub fn document_path_hex(&self) -> Option<String> {
        self.document_path.map(|hash| hash.to_hex().to_string())
    }
}
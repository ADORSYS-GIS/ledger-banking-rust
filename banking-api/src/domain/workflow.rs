use blake3::Hash;
use chrono::{DateTime, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountWorkflow {
    pub id: Uuid,
    pub account_id: Uuid,
    pub workflow_type: WorkflowType,
    pub current_step: WorkflowStep,
    pub status: WorkflowStatus,
    /// References Person.person_id
    pub initiated_by: Uuid,
    pub initiated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub steps_completed: Vec<WorkflowStepRecord>,
    pub next_action_required: Option<HeaplessString<500>>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStepRecord {
    pub step: WorkflowStep,
    pub completed_at: DateTime<Utc>,
    /// References Person.person_id
    pub completed_by: Uuid,
    pub notes: Option<HeaplessString<500>>,
    pub supporting_documents: Vec<HeaplessString<100>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountOpeningRequest {
    pub customer_id: Uuid,
    pub product_code: HeaplessString<12>,
    pub initial_deposit: Option<Decimal>,
    pub channel: HeaplessString<50>,
    /// References Person.person_id
    pub initiated_by: Uuid,
    pub supporting_documents: Vec<DocumentReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClosureRequest {
    pub reason: ClosureReason,
    pub disbursement_instructions: super::account::DisbursementInstructions,
    /// References Person.person_id
    pub requested_by: Uuid,
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
    pub product_specific_rules: Vec<HeaplessString<200>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentReference {
    pub document_id: Hash,
    pub document_type: HeaplessString<50>,
    pub document_path: Option<Hash>,
}

impl DocumentReference {
    /// Create new document reference with hash-based ID
    pub fn new(document_type: &str, content: &[u8]) -> Result<Self, &'static str> {
        let doc_type = HeaplessString::try_from(document_type)
            .map_err(|_| "Document type exceeds maximum length of 50 characters")?;
        Ok(Self {
            document_id: blake3::hash(content),
            document_type: doc_type,
            document_path: None,
        })
    }
    
    /// Create document reference from path content
    pub fn with_path(document_type: &str, content: &[u8], path_content: &[u8]) -> Result<Self, &'static str> {
        let doc_type = HeaplessString::try_from(document_type)
            .map_err(|_| "Document type exceeds maximum length of 50 characters")?;
        Ok(Self {
            document_id: blake3::hash(content),
            document_type: doc_type,
            document_path: Some(blake3::hash(path_content)),
        })
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
use chrono::{DateTime, Utc};
use heapless::String as HeaplessString;
use uuid::Uuid;

/// Account Workflow database model
#[derive(Debug, Clone)]
pub struct AccountWorkflowModel {
    pub workflow_id: Uuid,
    pub account_id: Uuid,
    pub workflow_type: String, // AccountOpening, AccountClosure, AccountReactivation, ComplianceVerification, MultiPartyApproval
    pub current_step: String,  // InitiateRequest, ComplianceCheck, DocumentVerification, ApprovalRequired, FinalSettlement, Completed
    pub status: String,        // InProgress, PendingAction, Completed, Failed, Cancelled, TimedOut
    pub initiated_by: String,
    pub initiated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub next_action_required: Option<String>,
    pub timeout_at: Option<DateTime<Utc>>,
    pub metadata: Option<String>, // JSON with workflow-specific data
    pub priority: String,         // Low, Medium, High, Critical
    pub assigned_to: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: String,
}

/// Workflow Step Record database model
#[derive(Debug, Clone)]
pub struct WorkflowStepRecordModel {
    pub record_id: Uuid,
    pub workflow_id: Uuid,
    pub step_name: String,
    pub step_status: String, // InProgress, Completed, Failed, Skipped
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub completed_by: Option<String>,
    pub notes: Option<String>,
    pub supporting_documents: Option<String>, // JSON array
    pub duration_seconds: Option<i64>,
    pub error_details: Option<String>,
    pub retry_count: i32,
    pub created_at: DateTime<Utc>,
}

/// Approval Workflow database model (for multi-party approvals)
#[derive(Debug, Clone)]
pub struct ApprovalWorkflowModel {
    pub workflow_id: Uuid,
    pub transaction_id: Option<Uuid>,
    pub account_id: Option<Uuid>,
    pub approval_type: String, // TransactionApproval, AccountOpening, AccountClosure
    pub required_approvers: String, // JSON array of approver IDs
    pub received_approvals: String, // JSON array of approval records
    pub minimum_approvals: i32,
    pub current_approvals: i32,
    pub status: String, // Pending, Approved, Rejected, TimedOut
    pub initiated_by: String,
    pub initiated_at: DateTime<Utc>,
    pub timeout_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    /// References ReasonAndPurpose.id for rejection reason
    pub rejection_reason_id: Option<Uuid>,
    pub metadata: Option<String>, // JSON with approval-specific data
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
}

/// Transaction Approval database model
#[derive(Debug, Clone)]
pub struct WorkflowTransactionApprovalModel {
    pub approval_id: Uuid,
    pub workflow_id: Uuid,
    pub transaction_id: Uuid,
    pub approver_id: Uuid,
    pub approval_action: String, // Approved, Rejected, Delegated
    pub approved_at: DateTime<Utc>,
    pub approval_notes: Option<HeaplessString<512>>,
    pub approval_method: String, // Manual, Digital, Biometric
    pub approval_location: Option<String>,
    pub approval_device_info: Option<String>, // JSON with device information
    pub created_at: DateTime<Utc>,
}

/// Account Status History database model (enhanced audit trail)
#[derive(Debug, Clone)]
pub struct WorkflowStatusChangeModel {
    pub history_id: Uuid,
    pub account_id: Uuid,
    pub old_status: Option<String>,
    pub new_status: String,
    /// References ReasonAndPurpose.id for status change
    pub change_reason_id: Uuid,
    /// Additional context for status change
    pub additional_context: Option<HeaplessString<200>>,
    pub changed_by: String,
    pub changed_at: DateTime<Utc>,
    pub system_triggered: bool,
    pub workflow_id: Option<Uuid>, // Link to associated workflow
    pub supporting_documents: Option<String>, // JSON array
    pub approval_required: bool,
    pub approved_by: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
use chrono::{DateTime, Utc};
use heapless::String as HeaplessString;
use uuid::Uuid;

/// Account Workflow database model
#[derive(Debug, Clone)]
pub struct AccountWorkflowModel {
    pub workflow_id: Uuid,
    pub account_id: Uuid,
    pub workflow_type: HeaplessString<50>, // AccountOpening, AccountClosure, AccountReactivation, ComplianceVerification, MultiPartyApproval
    pub current_step: HeaplessString<50>,  // InitiateRequest, ComplianceCheck, DocumentVerification, ApprovalRequired, FinalSettlement, Completed
    pub status: HeaplessString<20>,        // InProgress, PendingAction, Completed, Failed, Cancelled, TimedOut
    pub initiated_by: HeaplessString<100>,
    pub initiated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub next_action_required: Option<HeaplessString<500>>,
    pub timeout_at: Option<DateTime<Utc>>,
    pub metadata: Option<HeaplessString<2000>>, // JSON with workflow-specific data
    pub priority: HeaplessString<20>,         // Low, Medium, High, Critical
    pub assigned_to: Option<HeaplessString<100>>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: HeaplessString<100>,
}

/// Workflow Step Record database model
#[derive(Debug, Clone)]
pub struct WorkflowStepRecordModel {
    pub record_id: Uuid,
    pub workflow_id: Uuid,
    pub step_name: HeaplessString<50>,
    pub step_status: HeaplessString<20>, // InProgress, Completed, Failed, Skipped
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub completed_by: Option<HeaplessString<100>>,
    pub notes: Option<HeaplessString<1000>>,
    pub supporting_documents: Option<HeaplessString<2000>>, // JSON array
    pub duration_seconds: Option<i64>,
    pub error_details: Option<HeaplessString<1000>>,
    pub retry_count: i32,
    pub created_at: DateTime<Utc>,
}

/// Approval Workflow database model (for multi-party approvals)
#[derive(Debug, Clone)]
pub struct ApprovalWorkflowModel {
    pub workflow_id: Uuid,
    pub transaction_id: Option<Uuid>,
    pub account_id: Option<Uuid>,
    pub approval_type: HeaplessString<50>, // TransactionApproval, AccountOpening, AccountClosure
    pub required_approvers: HeaplessString<1000>, // JSON array of approver IDs
    pub received_approvals: HeaplessString<2000>, // JSON array of approval records
    pub minimum_approvals: i32,
    pub current_approvals: i32,
    pub status: HeaplessString<20>, // Pending, Approved, Rejected, TimedOut
    pub initiated_by: HeaplessString<100>,
    pub initiated_at: DateTime<Utc>,
    pub timeout_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    /// References ReasonAndPurpose.id for rejection reason
    pub rejection_reason_id: Option<Uuid>,
    pub metadata: Option<HeaplessString<2000>>, // JSON with approval-specific data
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
    pub approval_action: HeaplessString<20>, // Approved, Rejected, Delegated
    pub approved_at: DateTime<Utc>,
    pub approval_notes: Option<HeaplessString<512>>,
    pub approval_method: HeaplessString<20>, // Manual, Digital, Biometric
    pub approval_location: Option<HeaplessString<100>>,
    pub approval_device_info: Option<HeaplessString<500>>, // JSON with device information
    pub created_at: DateTime<Utc>,
}

/// Account Status History database model (enhanced audit trail)
#[derive(Debug, Clone)]
pub struct WorkflowStatusChangeModel {
    pub history_id: Uuid,
    pub account_id: Uuid,
    pub old_status: Option<HeaplessString<30>>,
    pub new_status: HeaplessString<30>,
    /// References ReasonAndPurpose.id for status change
    pub change_reason_id: Uuid,
    /// Additional context for status change
    pub additional_context: Option<HeaplessString<200>>,
    pub changed_by: HeaplessString<100>,
    pub changed_at: DateTime<Utc>,
    pub system_triggered: bool,
    pub workflow_id: Option<Uuid>, // Link to associated workflow
    pub supporting_documents: Option<HeaplessString<2000>>, // JSON array
    pub approval_required: bool,
    pub approved_by: Option<HeaplessString<100>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
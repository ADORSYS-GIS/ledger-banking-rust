use blake3::Hash;
use chrono::{DateTime, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Database representation of WorkflowType enum
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkflowTypeModel {
    AccountOpening,
    AccountClosure,
    LoanApplication,
    LoanDisbursement,
    TransactionApproval,
    ComplianceCheck,
    KycUpdate,
    DocumentVerification,
    CreditDecision,
    CollateralValuation,
    InterestRateChange,
    FeeWaiver,
    LimitChange,
    StatusChange,
    ManualIntervention,
}

impl std::fmt::Display for WorkflowTypeModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkflowTypeModel::AccountOpening => write!(f, "AccountOpening"),
            WorkflowTypeModel::AccountClosure => write!(f, "AccountClosure"),
            WorkflowTypeModel::LoanApplication => write!(f, "LoanApplication"),
            WorkflowTypeModel::LoanDisbursement => write!(f, "LoanDisbursement"),
            WorkflowTypeModel::TransactionApproval => write!(f, "TransactionApproval"),
            WorkflowTypeModel::ComplianceCheck => write!(f, "ComplianceCheck"),
            WorkflowTypeModel::KycUpdate => write!(f, "KycUpdate"),
            WorkflowTypeModel::DocumentVerification => write!(f, "DocumentVerification"),
            WorkflowTypeModel::CreditDecision => write!(f, "CreditDecision"),
            WorkflowTypeModel::CollateralValuation => write!(f, "CollateralValuation"),
            WorkflowTypeModel::InterestRateChange => write!(f, "InterestRateChange"),
            WorkflowTypeModel::FeeWaiver => write!(f, "FeeWaiver"),
            WorkflowTypeModel::LimitChange => write!(f, "LimitChange"),
            WorkflowTypeModel::StatusChange => write!(f, "StatusChange"),
            WorkflowTypeModel::ManualIntervention => write!(f, "ManualIntervention"),
        }
    }
}

impl std::str::FromStr for WorkflowTypeModel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AccountOpening" => Ok(WorkflowTypeModel::AccountOpening),
            "AccountClosure" => Ok(WorkflowTypeModel::AccountClosure),
            "LoanApplication" => Ok(WorkflowTypeModel::LoanApplication),
            "LoanDisbursement" => Ok(WorkflowTypeModel::LoanDisbursement),
            "TransactionApproval" => Ok(WorkflowTypeModel::TransactionApproval),
            "ComplianceCheck" => Ok(WorkflowTypeModel::ComplianceCheck),
            "KycUpdate" => Ok(WorkflowTypeModel::KycUpdate),
            "DocumentVerification" => Ok(WorkflowTypeModel::DocumentVerification),
            "CreditDecision" => Ok(WorkflowTypeModel::CreditDecision),
            "CollateralValuation" => Ok(WorkflowTypeModel::CollateralValuation),
            "InterestRateChange" => Ok(WorkflowTypeModel::InterestRateChange),
            "FeeWaiver" => Ok(WorkflowTypeModel::FeeWaiver),
            "LimitChange" => Ok(WorkflowTypeModel::LimitChange),
            "StatusChange" => Ok(WorkflowTypeModel::StatusChange),
            "ManualIntervention" => Ok(WorkflowTypeModel::ManualIntervention),
            _ => Err(format!("Invalid workflow type: {s}")),
        }
    }
}

/// Database representation of WorkflowStep enum
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WorkflowStepModel {
    InitiateRequest,
    ComplianceCheck,
    DocumentVerification,
    ApprovalRequired,
    FinalSettlement,
    Completed,
}

impl std::fmt::Display for WorkflowStepModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkflowStepModel::InitiateRequest => write!(f, "InitiateRequest"),
            WorkflowStepModel::ComplianceCheck => write!(f, "ComplianceCheck"),
            WorkflowStepModel::DocumentVerification => write!(f, "DocumentVerification"),
            WorkflowStepModel::ApprovalRequired => write!(f, "ApprovalRequired"),
            WorkflowStepModel::FinalSettlement => write!(f, "FinalSettlement"),
            WorkflowStepModel::Completed => write!(f, "Completed"),
        }
    }
}

impl std::str::FromStr for WorkflowStepModel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "InitiateRequest" => Ok(WorkflowStepModel::InitiateRequest),
            "ComplianceCheck" => Ok(WorkflowStepModel::ComplianceCheck),
            "DocumentVerification" => Ok(WorkflowStepModel::DocumentVerification),
            "ApprovalRequired" => Ok(WorkflowStepModel::ApprovalRequired),
            "FinalSettlement" => Ok(WorkflowStepModel::FinalSettlement),
            "Completed" => Ok(WorkflowStepModel::Completed),
            _ => Err(format!("Invalid workflow step: {s}")),
        }
    }
}

/// Database representation of WorkflowStatus enum
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum WorkflowStatusModel {
    InProgress,
    PendingAction,
    Completed,
    Failed,
    Cancelled,
    TimedOut,
}

impl std::fmt::Display for WorkflowStatusModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkflowStatusModel::InProgress => write!(f, "InProgress"),
            WorkflowStatusModel::PendingAction => write!(f, "PendingAction"),
            WorkflowStatusModel::Completed => write!(f, "Completed"),
            WorkflowStatusModel::Failed => write!(f, "Failed"),
            WorkflowStatusModel::Cancelled => write!(f, "Cancelled"),
            WorkflowStatusModel::TimedOut => write!(f, "TimedOut"),
        }
    }
}

impl std::str::FromStr for WorkflowStatusModel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "InProgress" => Ok(WorkflowStatusModel::InProgress),
            "PendingAction" => Ok(WorkflowStatusModel::PendingAction),
            "Completed" => Ok(WorkflowStatusModel::Completed),
            "Failed" => Ok(WorkflowStatusModel::Failed),
            "Cancelled" => Ok(WorkflowStatusModel::Cancelled),
            "TimedOut" => Ok(WorkflowStatusModel::TimedOut),
            _ => Err(format!("Invalid workflow status: {s}")),
        }
    }
}

/// Database representation of ClosureReason enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClosureReasonModel {
    CustomerRequest,
    Regulatory,
    Compliance,
    Dormancy,
    SystemMaintenance,
}

/// Account Workflow database model
#[derive(Debug, Clone)]
pub struct AccountWorkflowModel {
    pub id: Uuid,
    pub account_id: Uuid,
    pub workflow_type: WorkflowTypeModel,
    pub current_step: WorkflowStepModel,
    pub status: WorkflowStatusModel,
    /// References Person.person_id
    pub initiated_by: Uuid,
    pub initiated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub next_action_required: Option<HeaplessString<500>>,
    pub timeout_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
}

/// Workflow Step Record database model
#[derive(Debug, Clone)]
pub struct WorkflowStepRecordModel {
    pub step: WorkflowStepModel,
    pub completed_at: DateTime<Utc>,
    /// References Person.person_id
    pub completed_by: Uuid,
    pub notes: Option<HeaplessString<500>>,
    pub supporting_documents: Vec<HeaplessString<100>>,
}

/// Account Opening Request database model
#[derive(Debug, Clone)]
pub struct AccountOpeningRequestModel {
    pub customer_id: Uuid,
    pub product_id: Uuid,
    pub initial_deposit: Option<Decimal>,
    pub channel: HeaplessString<50>,
    /// References Person.person_id
    pub initiated_by: Uuid,
    pub supporting_documents: Vec<DocumentReferenceModel>,
}

/// Closure Request database model
#[derive(Debug, Clone)]
pub struct ClosureRequestModel {
    pub reason: ClosureReasonModel,
    /// References Person.person_id
    pub requested_by: Uuid,
    pub force_closure: bool,
}

/// Final Settlement database model
#[derive(Debug, Clone)]
pub struct WorkflowFinalSettlementModel {
    pub current_balance: Decimal,
    pub accrued_interest: Decimal,
    pub pending_fees: Decimal,
    pub closure_fees: Decimal,
    pub final_amount: Decimal,
    pub requires_disbursement: bool,
}

/// Dormancy Assessment database model
#[derive(Debug, Clone)]
pub struct DormancyAssessmentModel {
    pub is_eligible: bool,
    pub last_activity_date: Option<chrono::NaiveDate>,
    pub days_inactive: i32,
    pub threshold_days: i32,
    pub product_specific_rules: Vec<HeaplessString<200>>,
}

/// Document Reference database model
#[derive(Debug, Clone)]
pub struct DocumentReferenceModel {
    pub document_id: Hash,
    pub document_type: HeaplessString<50>,
    pub document_path: Option<Hash>,
}

/// Workflow Transaction Approval Model (for database operations)
#[derive(Debug, Clone)]
pub struct WorkflowTransactionApprovalModel {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub transaction_id: Uuid,
    pub approver_id: Uuid,
    pub approval_action: HeaplessString<20>, // Approved, Rejected, Delegated
    pub approved_at: DateTime<Utc>,
    pub approval_notes: Option<HeaplessString<512>>,
    pub approval_method: HeaplessString<20>, // Manual, Digital, Biometric
    pub approval_location: Option<HeaplessString<100>>,
    pub created_at: DateTime<Utc>,
}

/// Approval Workflow Model (for multi-party approvals)
#[derive(Debug, Clone)]
pub struct ApprovalWorkflowModel {
    pub id: Uuid,
    pub transaction_id: Option<Uuid>,
    pub account_id: Option<Uuid>,
    pub approval_type: HeaplessString<50>, // TransactionApproval, AccountOpening, AccountClosure
    pub minimum_approvals: i32,
    pub current_approvals: i32,
    pub status: WorkflowStatusModel,
    pub initiated_by: Uuid,
    pub initiated_at: DateTime<Utc>,
    pub timeout_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub rejection_reason_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
}


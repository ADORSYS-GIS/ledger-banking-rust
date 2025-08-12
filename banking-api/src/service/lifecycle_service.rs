use async_trait::async_trait;
use heapless::String as HeaplessString;
use uuid::Uuid;

use crate::{
    domain::{
        AccountWorkflow, AccountOpeningRequest, ClosureRequest, 
        FinalSettlement, DormancyAssessment, AccountStatus, 
        AccountStatusChangeRecord, KycResult
    },
    error::BankingResult,
};

/// Enhanced Account Lifecycle Service from banking enhancements
#[async_trait]
pub trait AccountLifecycleService: Send + Sync {
    /// Account origination workflow
    async fn initiate_account_opening(&self, request: AccountOpeningRequest) -> BankingResult<AccountWorkflow>;
    async fn complete_kyc_verification(&self, account_id: Uuid, verification_result: KycResult) -> BankingResult<()>;
    async fn activate_account(&self, account_id: Uuid, authorized_by: Uuid) -> BankingResult<()>;
    
    /// Dormancy management (automated)
    async fn check_dormancy_eligibility(&self, account_id: Uuid) -> BankingResult<DormancyAssessment>;
    async fn mark_account_dormant(&self, account_id: Uuid, system_triggered: bool) -> BankingResult<()>;
    
    /// Reactivation workflow (requires human intervention)
    async fn initiate_reactivation(&self, account_id: Uuid, requested_by: Uuid) -> BankingResult<AccountWorkflow>;
    async fn complete_mini_kyc(&self, account_id: Uuid, verification_result: KycResult) -> BankingResult<()>;
    
    /// Account closure workflow
    async fn initiate_closure(&self, account_id: Uuid, closure_request: ClosureRequest) -> BankingResult<AccountWorkflow>;
    async fn calculate_final_settlement(&self, account_id: Uuid) -> BankingResult<FinalSettlement>;
    async fn process_final_disbursement(&self, account_id: Uuid, disbursement: crate::domain::DisbursementInstructions) -> BankingResult<()>;
    async fn finalize_closure(&self, account_id: Uuid) -> BankingResult<()>;
    
    /// Status management with reason ID validation
    async fn update_account_status(&self, account_id: Uuid, new_status: AccountStatus, reason_id: Uuid, additional_context: Option<&str>, authorized_by: Uuid) -> BankingResult<()>;
    
    /// Legacy method - deprecated, use update_account_status with reason_id instead
    #[deprecated(note = "Use update_account_status with reason_id instead")]
    async fn update_account_status_legacy(&self, account_id: Uuid, new_status: AccountStatus, reason: HeaplessString<500>, authorized_by: Uuid) -> BankingResult<()>;
    async fn get_status_history(&self, account_id: Uuid) -> BankingResult<Vec<AccountStatusChangeRecord>>;

    /// Workflow management
    async fn find_workflow_by_id(&self, workflow_id: Uuid) -> BankingResult<Option<AccountWorkflow>>;
    async fn find_workflows_by_account(&self, account_id: Uuid) -> BankingResult<Vec<AccountWorkflow>>;
    async fn update_workflow_status(&self, workflow_id: Uuid, status: crate::domain::WorkflowStatus) -> BankingResult<()>;
    
    /// Workflow step progression
    async fn advance_workflow_step(&self, workflow_id: Uuid, completed_by: Uuid, notes: Option<HeaplessString<500>>) -> BankingResult<()>;
    /// Reject workflow with reason ID validation
    async fn reject_workflow(&self, workflow_id: Uuid, reason_id: Uuid, additional_details: Option<&str>, rejected_by: Uuid) -> BankingResult<()>;
    
    /// Legacy method - deprecated, use reject_workflow with reason_id instead
    #[deprecated(note = "Use reject_workflow with reason_id instead")]
    async fn reject_workflow_legacy(&self, workflow_id: Uuid, reason: HeaplessString<500>, rejected_by: Uuid) -> BankingResult<()>;
    
    /// Account lifecycle queries
    async fn find_pending_activations(&self) -> BankingResult<Vec<AccountWorkflow>>;
    async fn find_pending_closures(&self) -> BankingResult<Vec<AccountWorkflow>>;
    async fn find_accounts_eligible_for_dormancy(&self, threshold_days: i32) -> BankingResult<Vec<Uuid>>;
    
    /// Batch processing for EOD
    async fn batch_process_dormancy(&self, processing_date: chrono::NaiveDate) -> BankingResult<crate::service::DormancyReport>;
    async fn batch_process_closures(&self, processing_date: chrono::NaiveDate) -> BankingResult<crate::service::MaintenanceReport>;
    
    /// Compliance integration
    async fn trigger_compliance_check(&self, account_id: Uuid, check_type: ComplianceCheckType) -> BankingResult<()>;
    async fn handle_compliance_result(&self, account_id: Uuid, result: ComplianceCheckResult) -> BankingResult<()>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComplianceCheckResult {
    pub check_id: Uuid,
    pub account_id: Uuid,
    pub check_type: ComplianceCheckType,
    pub result: LifecycleComplianceResult,
    pub details: Option<HeaplessString<500>>,
    pub performed_at: chrono::DateTime<chrono::Utc>,
    pub requires_manual_review: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ComplianceCheckType {
    KycVerification,
    AmlScreening,
    SanctionsCheck,
    PepScreening,
    DocumentVerification,
    AddressVerification,
    IdentityVerification,
    SourceOfFunds,
    RiskAssessment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LifecycleComplianceResult {
    Pass,
    Fail,
    Warning,
    RequiresEscalation,
}
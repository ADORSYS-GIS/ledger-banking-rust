use async_trait::async_trait;
use banking_api::BankingResult;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};

use crate::models::{AccountWorkflowModel, WorkflowStepRecordModel};

#[async_trait]
pub trait WorkflowRepository: Send + Sync {
    /// Account Workflow Operations
    async fn create_workflow(&self, workflow: &AccountWorkflowModel) -> BankingResult<AccountWorkflowModel>;
    async fn update_workflow(&self, workflow: AccountWorkflowModel) -> BankingResult<AccountWorkflowModel>;
    async fn find_workflow_by_id(&self, workflow_id: Uuid) -> BankingResult<Option<AccountWorkflowModel>>;
    async fn find_workflows_by_account(&self, account_id: Uuid) -> BankingResult<Vec<AccountWorkflowModel>>;
    async fn find_active_workflow(&self, account_id: Uuid, workflow_type: &str) -> BankingResult<Option<AccountWorkflowModel>>;
    async fn find_workflows_by_type(&self, workflow_type: &str) -> BankingResult<Vec<AccountWorkflowModel>>;
    async fn find_workflows_by_status(&self, status: &str) -> BankingResult<Vec<AccountWorkflowModel>>;
    async fn find_workflows_by_initiator(&self, initiated_by: &str) -> BankingResult<Vec<AccountWorkflowModel>>;
    
    /// Workflow Status Management
    async fn update_workflow_status(&self, workflow_id: Uuid, status: &str, notes: &str) -> BankingResult<()>;
    async fn update_workflow_step(&self, workflow_id: Uuid, current_step: &str) -> BankingResult<()>;
    async fn advance_workflow_step(&self, workflow_id: Uuid, step: &str, notes: &str) -> BankingResult<()>;
    async fn complete_workflow(&self, workflow_id: Uuid, completion_notes: &str) -> BankingResult<()>;
    async fn fail_workflow(&self, workflow_id: Uuid, failure_reason: &str) -> BankingResult<()>;
    async fn cancel_workflow(&self, workflow_id: Uuid, reason: &str) -> BankingResult<()>;
    
    /// Workflow Step Record Operations
    async fn add_step_record(&self, step_record: WorkflowStepRecordModel) -> BankingResult<WorkflowStepRecordModel>;
    async fn find_step_records_by_workflow(&self, workflow_id: Uuid) -> BankingResult<Vec<WorkflowStepRecordModel>>;
    async fn find_latest_step_record(&self, workflow_id: Uuid) -> BankingResult<Option<WorkflowStepRecordModel>>;
    
    /// Workflow Monitoring and Management
    async fn find_pending_workflows(&self) -> BankingResult<Vec<AccountWorkflowModel>>;
    async fn find_in_progress_workflows(&self) -> BankingResult<Vec<AccountWorkflowModel>>;
    async fn find_expired_workflows(&self, reference_time: DateTime<Utc>) -> BankingResult<Vec<AccountWorkflowModel>>;
    async fn find_workflows_requiring_action(&self, action_type: &str) -> BankingResult<Vec<AccountWorkflowModel>>;
    
    /// Account Opening Workflows
    async fn find_account_opening_workflows(&self, status: Option<&str>) -> BankingResult<Vec<AccountWorkflowModel>>;
    async fn find_pending_kyc_workflows(&self) -> BankingResult<Vec<AccountWorkflowModel>>;
    async fn find_pending_document_verification(&self) -> BankingResult<Vec<AccountWorkflowModel>>;
    
    /// Account Closure Workflows
    async fn find_account_closure_workflows(&self, status: Option<&str>) -> BankingResult<Vec<AccountWorkflowModel>>;
    async fn find_pending_final_settlement(&self) -> BankingResult<Vec<AccountWorkflowModel>>;
    async fn find_pending_disbursement(&self) -> BankingResult<Vec<AccountWorkflowModel>>;
    
    /// Account Reactivation Workflows
    async fn find_reactivation_workflows(&self, status: Option<&str>) -> BankingResult<Vec<AccountWorkflowModel>>;
    async fn find_pending_mini_kyc(&self) -> BankingResult<Vec<AccountWorkflowModel>>;
    
    /// Compliance Workflows
    async fn find_compliance_workflows(&self, status: Option<&str>) -> BankingResult<Vec<AccountWorkflowModel>>;
    async fn find_workflows_by_customer_risk(&self, risk_rating: &str) -> BankingResult<Vec<AccountWorkflowModel>>;
    
    /// Multi-Party Approval Workflows
    async fn find_approval_workflows(&self, status: Option<&str>) -> BankingResult<Vec<AccountWorkflowModel>>;
    async fn find_workflows_by_approver(&self, approver_id: &str) -> BankingResult<Vec<AccountWorkflowModel>>;
    async fn find_workflows_awaiting_approval(&self, approver_id: &str) -> BankingResult<Vec<AccountWorkflowModel>>;
    
    /// Workflow Analytics and Reporting
    async fn get_workflow_metrics(&self, from_date: NaiveDate, to_date: NaiveDate) -> BankingResult<WorkflowMetricsReport>;
    async fn get_workflow_performance(&self, workflow_type: &str, from_date: NaiveDate, to_date: NaiveDate) -> BankingResult<WorkflowPerformanceReport>;
    async fn get_workflow_bottlenecks(&self) -> BankingResult<Vec<WorkflowBottleneckReport>>;
    async fn get_average_completion_time(&self, workflow_type: &str) -> BankingResult<Option<f64>>;
    
    /// Workflow Cleanup and Maintenance
    async fn cleanup_completed_workflows(&self, retention_days: i32) -> BankingResult<i64>;
    async fn cleanup_cancelled_workflows(&self, retention_days: i32) -> BankingResult<i64>;
    async fn find_stale_workflows(&self, stale_threshold_hours: i32) -> BankingResult<Vec<AccountWorkflowModel>>;
    
    /// Batch Operations
    async fn bulk_update_workflow_status(&self, workflow_ids: Vec<Uuid>, status: &str) -> BankingResult<i64>;
    async fn bulk_timeout_expired_workflows(&self, reference_time: DateTime<Utc>) -> BankingResult<i64>;
    
    /// Utility Operations
    async fn workflow_exists(&self, workflow_id: Uuid) -> BankingResult<bool>;
    async fn count_workflows_by_type(&self, workflow_type: &str) -> BankingResult<i64>;
    async fn count_workflows_by_status(&self, status: &str) -> BankingResult<i64>;
    async fn count_pending_workflows(&self) -> BankingResult<i64>;
    async fn list_workflows(&self, offset: i64, limit: i64) -> BankingResult<Vec<AccountWorkflowModel>>;
    async fn count_all_workflows(&self) -> BankingResult<i64>;
}

/// Supporting structures for workflow reporting
pub struct WorkflowMetricsReport {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_workflows_created: i64,
    pub total_workflows_completed: i64,
    pub total_workflows_cancelled: i64,
    pub total_workflows_in_progress: i64,
    pub average_completion_time_hours: f64,
    pub workflows_by_type: Vec<WorkflowTypeMetrics>,
}

pub struct WorkflowTypeMetrics {
    pub workflow_type: String,
    pub total_created: i64,
    pub total_completed: i64,
    pub total_cancelled: i64,
    pub average_completion_time_hours: f64,
}

pub struct WorkflowPerformanceReport {
    pub workflow_type: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_workflows: i64,
    pub completed_workflows: i64,
    pub completion_rate_percentage: f64,
    pub average_completion_time_hours: f64,
    pub median_completion_time_hours: f64,
    pub fastest_completion_hours: f64,
    pub slowest_completion_hours: f64,
}

pub struct WorkflowBottleneckReport {
    pub workflow_step: String,
    pub workflow_type: String,
    pub average_time_spent_hours: f64,
    pub workflows_stuck_count: i64,
    pub max_time_stuck_hours: f64,
}
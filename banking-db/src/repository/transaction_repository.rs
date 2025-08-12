use async_trait::async_trait;
use banking_api::BankingResult;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, NaiveDate};

use crate::models::{TransactionModel};
use crate::models::workflow::{ApprovalWorkflowModel, WorkflowTransactionApprovalModel};

#[async_trait]
pub trait TransactionRepository: Send + Sync {
    /// Create a new transaction record
    async fn create(&self, transaction: TransactionModel) -> BankingResult<TransactionModel>;
    
    /// Update existing transaction record
    async fn update(&self, transaction: TransactionModel) -> BankingResult<TransactionModel>;
    
    /// Find transaction by ID
    async fn find_by_id(&self, transaction_id: Uuid) -> BankingResult<Option<TransactionModel>>;
    
    /// Find transactions by account ID
    async fn find_by_account_id(&self, account_id: Uuid, from_date: Option<NaiveDate>, to_date: Option<NaiveDate>) -> BankingResult<Vec<TransactionModel>>;
    
    /// Find transactions by account ID with date range
    async fn find_by_account_date_range(&self, account_id: Uuid, from_date: NaiveDate, to_date: NaiveDate) -> BankingResult<Vec<TransactionModel>>;
    
    /// Find transactions by reference number
    async fn find_by_reference(&self, reference_number: &str) -> BankingResult<Option<TransactionModel>>;
    
    /// Find transactions by external reference
    async fn find_by_external_reference(&self, external_reference: &str) -> BankingResult<Vec<TransactionModel>>;
    
    /// Find transactions by status
    async fn find_by_status(&self, status: &str) -> BankingResult<Vec<TransactionModel>>;
    
    /// Find transactions requiring approval
    async fn find_requiring_approval(&self) -> BankingResult<Vec<TransactionModel>>;
    
    /// Find transactions by terminal ID
    async fn find_by_terminal_id(&self, terminal_id: Uuid, from_date: Option<NaiveDate>, to_date: Option<NaiveDate>) -> BankingResult<Vec<TransactionModel>>;
    
    /// Find transactions by agent user ID
    async fn find_by_agent_person_id(&self, agent_person_id: Uuid, from_date: Option<NaiveDate>, to_date: Option<NaiveDate>) -> BankingResult<Vec<TransactionModel>>;
    
    /// Find transactions by channel
    async fn find_by_channel(&self, channel_id: &str, from_date: Option<NaiveDate>, to_date: Option<NaiveDate>) -> BankingResult<Vec<TransactionModel>>;
    
    /// Update transaction status
    async fn update_status(&self, transaction_id: Uuid, status: &str, reason: &str) -> BankingResult<()>;
    
    /// Update approval status
    async fn update_approval_status(&self, transaction_id: Uuid, approval_status: &str) -> BankingResult<()>;
    
    /// Find last customer-initiated transaction for an account (for dormancy calculation)
    async fn find_last_customer_transaction(&self, account_id: Uuid) -> BankingResult<Option<TransactionModel>>;
    
    /// Calculate daily transaction volume for terminal
    async fn calculate_daily_volume_by_terminal(&self, terminal_id: Uuid, date: NaiveDate) -> BankingResult<Decimal>;
    
    /// Calculate daily transaction volume for branch
    async fn calculate_daily_volume_by_branch(&self, branch_id: Uuid, date: NaiveDate) -> BankingResult<Decimal>;
    
    /// Calculate daily transaction volume for network
    async fn calculate_daily_volume_by_network(&self, network_id: Uuid, date: NaiveDate) -> BankingResult<Decimal>;
    
    /// Reverse a transaction
    async fn reverse_transaction(&self, original_transaction_id: Uuid, reversal_transaction: TransactionModel) -> BankingResult<TransactionModel>;
    
    /// Find transactions for reconciliation
    async fn find_for_reconciliation(&self, channel_id: &str, date: NaiveDate) -> BankingResult<Vec<TransactionModel>>;
    
    /// Approval Workflow Operations
    async fn create_workflow(&self, workflow: ApprovalWorkflowModel) -> BankingResult<ApprovalWorkflowModel>;
    async fn find_workflow_by_id(&self, workflow_id: Uuid) -> BankingResult<Option<ApprovalWorkflowModel>>;
    async fn find_workflow_by_transaction(&self, transaction_id: Uuid) -> BankingResult<Option<ApprovalWorkflowModel>>;
    async fn update_workflow_status(&self, workflow_id: Uuid, status: &str) -> BankingResult<()>;
    async fn find_pending_workflows(&self) -> BankingResult<Vec<ApprovalWorkflowModel>>;
    async fn find_expired_workflows(&self, reference_time: DateTime<Utc>) -> BankingResult<Vec<ApprovalWorkflowModel>>;
    
    /// Transaction Approval Operations
    async fn create_approval(&self, approval: WorkflowTransactionApprovalModel) -> BankingResult<WorkflowTransactionApprovalModel>;
    async fn find_approvals_by_workflow(&self, workflow_id: Uuid) -> BankingResult<Vec<WorkflowTransactionApprovalModel>>;
    async fn find_approvals_by_approver(&self, approver_person_id: Uuid) -> BankingResult<Vec<WorkflowTransactionApprovalModel>>;
    async fn count_approvals_for_workflow(&self, workflow_id: Uuid) -> BankingResult<i64>;
    
    /// Utility Operations
    async fn exists(&self, transaction_id: Uuid) -> BankingResult<bool>;
    async fn count_by_account(&self, account_id: Uuid, from_date: Option<NaiveDate>, to_date: Option<NaiveDate>) -> BankingResult<i64>;
    async fn list(&self, offset: i64, limit: i64) -> BankingResult<Vec<TransactionModel>>;
    async fn count(&self) -> BankingResult<i64>;
}
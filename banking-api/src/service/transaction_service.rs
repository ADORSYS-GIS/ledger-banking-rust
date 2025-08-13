use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
    domain::{
        Transaction, TransactionType, TransactionValidationResult, TransactionApprovalWorkflow,
        PermittedOperation, TransactionRequest, TransactionResult, FinalSettlement
    },
    error::BankingResult,
};

#[async_trait]
pub trait TransactionService: Send + Sync {
    /// Process a transaction through the full pipeline
    async fn process_transaction(&self, transaction: Transaction) -> BankingResult<Transaction>;
    
    /// Validate transaction limits
    async fn validate_transaction_limits(&self, transaction: &Transaction) -> BankingResult<TransactionValidationResult>;
    
    /// Reverse a posted transaction with reason ID validation
    async fn reverse_transaction(&self, transaction_id: Uuid, reason_id: Uuid, additional_details: Option<&str>) -> BankingResult<()>;
    
    /// Legacy method - deprecated, use reverse_transaction with reason_id instead
    #[deprecated(note = "Use reverse_transaction with reason_id instead")]
    async fn reverse_transaction_legacy(&self, transaction_id: Uuid, reason: String) -> BankingResult<()>;
    
    /// Find transactions for an account within a date range
    async fn find_transactions_by_account(&self, account_id: Uuid, from: NaiveDate, to: NaiveDate) -> BankingResult<Vec<Transaction>>;
    
    /// Multi-party authorization workflow
    async fn initiate_approval_workflow(&self, transaction: Transaction) -> BankingResult<TransactionApprovalWorkflow>;
    async fn approve_transaction(&self, transaction_id: Uuid, approver_person_id: Uuid) -> BankingResult<()>;

    /// Status-aware transaction validation (from enhancements)
    async fn validate_account_transactional_status(&self, account_id: Uuid, transaction_type: TransactionType) -> BankingResult<TransactionValidationResult>;
    
    /// Get permitted operations for an account
    async fn get_permitted_operations(&self, account_id: Uuid) -> BankingResult<Vec<PermittedOperation>>;
    
    /// Final settlement operations
    async fn process_closure_transaction(&self, account_id: Uuid, settlement: FinalSettlement) -> BankingResult<Transaction>;
    /// Reverse pending transactions with reason ID validation
    async fn reverse_pending_transactions(&self, account_id: Uuid, reason_id: Uuid, additional_details: Option<&str>) -> BankingResult<Vec<Transaction>>;
    
    /// Legacy method - deprecated, use reverse_pending_transactions with reason_id instead
    #[deprecated(note = "Use reverse_pending_transactions with reason_id instead")]
    async fn reverse_pending_transactions_legacy(&self, account_id: Uuid, reason: String) -> BankingResult<Vec<Transaction>>;

    /// Process transaction request
    async fn process_transaction_request(&self, request: TransactionRequest) -> BankingResult<TransactionResult>;

    /// Find transaction by ID
    async fn find_transaction_by_id(&self, transaction_id: Uuid) -> BankingResult<Option<Transaction>>;

    /// Find transactions by reference number
    async fn find_transaction_by_reference(&self, reference_number: &str) -> BankingResult<Option<Transaction>>;

    /// Get transaction history for audit
    async fn get_transaction_audit_trail(&self, transaction_id: Uuid) -> BankingResult<Vec<TransactionAuditEntry>>;

    /// Update transaction status
    async fn update_transaction_status(&self, transaction_id: Uuid, status: crate::domain::TransactionStatus, reason: String) -> BankingResult<()>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransactionAuditEntry {
    pub audit_id: Uuid,
    pub transaction_id: Uuid,
    pub action: String,
    pub performed_by_person_id: String,
    pub performed_at: chrono::DateTime<chrono::Utc>,
    pub details: Option<String>,
}
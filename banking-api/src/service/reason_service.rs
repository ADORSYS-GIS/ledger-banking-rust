use crate::error::BankingResult;
use crate::domain::{AccountStatus, TransactionStatus, SarData, LoanRestructuring, PaymentReversal, FeeWaiver};
use uuid::Uuid;

// Note: ReasonAndPurpose types will be imported when the banking-db dependency is properly configured
// For now, we'll use placeholder types or move these to the implementation layer

// Note: This trait will be fully implemented when banking-db integration is complete
// For now, we'll define the interface to establish the contract

/// Enhanced service methods for integrating with existing banking services
#[async_trait::async_trait]
pub trait EnhancedAccountService: Send + Sync {
    /// Get account with resolved reasons in user's language
    async fn get_account_view(
        &self,
        account_id: Uuid,
        user_language: &[u8; 3],
    ) -> BankingResult<crate::domain::AccountView>;
    
    /// Close account with reason validation
    async fn close_account_with_reason(
        &self,
        account_id: Uuid,
        reason_id: Uuid,
        additional_details: Option<&str>,
        closed_by: Uuid, // References ReferencedPerson.person_id
    ) -> BankingResult<()>;
    
    /// Place hold with reason validation
    async fn place_hold_with_reason(
        &self,
        account_id: Uuid,
        amount: rust_decimal::Decimal,
        reason_id: Uuid,
        additional_details: Option<&str>,
        placed_by: Uuid, // References ReferencedPerson.person_id
    ) -> BankingResult<Uuid>;
    
    /// Update account status with reason validation
    async fn update_account_status_with_reason(
        &self,
        account_id: Uuid,
        new_status: AccountStatus,
        reason_id: Uuid,
        additional_context: Option<&str>,
        updated_by: Uuid, // References ReferencedPerson.person_id
    ) -> BankingResult<()>;
}

/// Enhanced transaction service methods
#[async_trait::async_trait]
pub trait EnhancedTransactionService: Send + Sync {
    /// Reverse transaction with reason validation
    async fn reverse_transaction_with_reason(
        &self,
        transaction_id: Uuid,
        reason_id: Uuid,
        additional_details: Option<&str>,
        reversed_by: Uuid, // References ReferencedPerson.person_id
    ) -> BankingResult<()>;
    
    /// Update transaction status with reason validation
    async fn update_transaction_status_with_reason(
        &self,
        transaction_id: Uuid,
        status: TransactionStatus,
        reason_id: Uuid,
        additional_context: Option<&str>,
        updated_by: Uuid, // References ReferencedPerson.person_id
    ) -> BankingResult<()>;
}

/// Enhanced compliance service methods
#[async_trait::async_trait]
pub trait EnhancedComplianceService: Send + Sync {
    /// Generate SAR with reason validation
    async fn generate_sar_with_reason(
        &self,
        customer_id: Uuid,
        reason_id: Uuid,
        additional_details: Option<&str>,
        generated_by: Uuid, // References ReferencedPerson.person_id
    ) -> BankingResult<SarData>;
}

/// Enhanced loan service methods
#[async_trait::async_trait]
pub trait EnhancedLoanService: Send + Sync {
    /// Restructure loan with reason validation
    async fn restructure_loan_with_reason(
        &self,
        loan_account_id: Uuid,
        restructuring_reason_id: Uuid,
        additional_details: Option<&str>,
        requested_by: Uuid, // References ReferencedPerson.person_id
    ) -> BankingResult<LoanRestructuring>;
    
    /// Reverse loan payment with reason validation
    async fn reverse_payment_with_reason(
        &self,
        payment_id: Uuid,
        reversal_reason_id: Uuid,
        additional_details: Option<&str>,
        reversed_by: Uuid, // References ReferencedPerson.person_id
    ) -> BankingResult<PaymentReversal>;
}

/// Enhanced fee service methods
#[async_trait::async_trait]
pub trait EnhancedFeeService: Send + Sync {
    /// Request fee waiver with reason validation
    async fn request_fee_waiver_with_reason(
        &self,
        fee_application_id: Uuid,
        reason_id: Uuid,
        additional_details: Option<&str>,
        requested_by: Uuid, // References ReferencedPerson.person_id
    ) -> BankingResult<FeeWaiver>;
}

// Helper functions will be implemented when ReasonAndPurpose types are available
// This provides the interface contract for future implementation
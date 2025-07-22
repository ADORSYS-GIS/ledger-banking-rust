use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub type BankingResult<T> = Result<T, BankingError>;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum BankingError {
    // Account-related errors
    #[error("Account not found: {0}")]
    AccountNotFound(Uuid),
    
    #[error("Account {account_id} is frozen: {frozen_reason}")]
    AccountFrozen { 
        account_id: Uuid, 
        frozen_reason: String 
    },
    
    #[error("Account {account_id} was closed on {closure_date}")]
    AccountClosed { 
        account_id: Uuid, 
        closure_date: NaiveDate 
    },
    
    #[error("Insufficient funds in account {account_id}: requested {requested}, available {available}")]
    InsufficientFunds { 
        account_id: Uuid, 
        requested: Decimal, 
        available: Decimal 
    },

    #[error("Account {account_id} is not operational: {reason}")]
    AccountNotOperational {
        account_id: Uuid,
        reason: String,
    },

    #[error("Account {account_id} is not in a transactional state")]
    AccountNotTransactional {
        account_id: Uuid,
    },
    
    // Customer-related errors
    #[error("Customer not found: {0}")]
    CustomerNotFound(Uuid),
    
    #[error("Customer {customer_id} is deceased (date of death: {date_of_death})")]
    CustomerDeceased { 
        customer_id: Uuid, 
        date_of_death: NaiveDate 
    },
    
    #[error("Customer {customer_id} is blacklisted: {blacklist_reason}")]
    CustomerBlacklisted { 
        customer_id: Uuid, 
        blacklist_reason: String 
    },
    
    // Transaction-related errors
    #[error("Transaction limit exceeded: attempted {attempted}, limit {limit} for {limit_type:?}")]
    TransactionLimitExceeded { 
        limit: Decimal, 
        attempted: Decimal, 
        limit_type: LimitType 
    },
    
    #[error("Invalid signature: required {required_signatories:?}, provided {provided_signatories:?}")]
    InvalidSignature { 
        required_signatories: Vec<Uuid>, 
        provided_signatories: Vec<Uuid> 
    },
    
    #[error("Approval required for transaction {transaction_id}: required approvers {required_approvers:?}")]
    ApprovalRequired { 
        transaction_id: Uuid, 
        required_approvers: Vec<Uuid> 
    },
    
    // Compliance-related errors
    #[error("Compliance violation: {violation_type} for customer {customer_id:?}")]
    ComplianceViolation { 
        violation_type: String, 
        customer_id: Option<Uuid> 
    },
    
    #[error("KYC incomplete for customer {customer_id}: missing documents {missing_documents:?}")]
    KycIncomplete { 
        customer_id: Uuid, 
        missing_documents: Vec<String> 
    },
    
    #[error("Sanctions match for customer {customer_id}: {match_details}")]
    SanctionsMatch { 
        customer_id: Uuid, 
        match_details: String 
    },
    
    // Product and system errors
    #[error("Invalid product code: {0}")]
    InvalidProductCode(String),
    
    #[error("Product catalog unavailable for {product_code}, fallback used: {fallback_used}")]
    ProductCatalogUnavailable { 
        product_code: String, 
        fallback_used: bool 
    },
    
    #[error("Business day calculation error for date {date} in jurisdiction {jurisdiction}")]
    BusinessDayCalculationError { 
        date: NaiveDate, 
        jurisdiction: String 
    },
    
    // Network and infrastructure
    #[error("Network error: {error_details}, retry possible: {retry_possible}")]
    NetworkError { 
        error_details: String, 
        retry_possible: bool 
    },
    
    #[error("Database constraint violation: {constraint} - {details}")]
    DatabaseConstraintViolation { 
        constraint: String, 
        details: String 
    },

    // Validation errors
    #[error("Validation error in {field}: {message}")]
    ValidationError {
        field: String,
        message: String,
    },

    #[error("Invalid enum value: {value} for field {field}")]
    InvalidEnumValue {
        value: String,
        field: String,
    },

    // Transaction errors  
    #[error("Invalid transaction amount: {0}")]
    InvalidTransactionAmount(String),
    
    #[error("Transaction not found: {0}")]
    TransactionNotFound(String),
    
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    
    // Date/Time errors
    #[error("Date calculation error: {0}")]
    DateCalculationError(String),
    
    // Document errors
    #[error("Duplicate identity document: {0}")]
    DuplicateIdentityDocument(String),
    
    // Authorization errors
    #[error("Unauthorized operation: {0}")]
    UnauthorizedOperation(String),

    // Internal errors
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<anyhow::Error> for BankingError {
    fn from(err: anyhow::Error) -> Self {
        BankingError::Internal(err.to_string())
    }
}

#[cfg(feature = "sqlx")]
impl From<sqlx::Error> for BankingError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => BankingError::Internal("Database row not found".to_string()),
            sqlx::Error::Database(ref db_err) => {
                if let Some(constraint) = db_err.constraint() {
                    BankingError::DatabaseConstraintViolation {
                        constraint: constraint.to_string(),
                        details: db_err.message().to_string(),
                    }
                } else {
                    BankingError::Internal(format!("Database error: {}", db_err.message()))
                }
            },
            _ => BankingError::Internal(format!("Database error: {err}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LimitType { 
    Daily, 
    Monthly, 
    PerTransaction, 
    Terminal, 
    Branch, 
    Network 
}
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub type BankingResult<T> = Result<T, BankingError>;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum BankingError {
    #[error("Location error: {0}")]
    LocationError(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Person service error: {0}")]
    PersonServiceError(#[from] crate::service::PersonServiceError),
    #[error("Country service error: {0}")]
    CountryServiceError(#[from] crate::service::CountryServiceError),
    #[error("Country subdivision service error: {0}")]
    CountrySubdivisionServiceError(#[from] crate::service::CountrySubdivisionServiceError),
    #[error("Locality service error: {0}")]
    LocalityServiceError(#[from] crate::service::LocalityServiceError),
    #[error("Audit service error: {0}")]
    AuditLogServiceError(#[from] crate::service::audit::audit_log_service::AuditLogServiceError),
    // Account-related errors
    #[error("Account not found: {0}")]
    AccountNotFound(Uuid),

    #[error("Account {account_id} is frozen: {frozen_reason}")]
    AccountFrozen {
        account_id: Uuid,
        frozen_reason: String,
    },

    #[error("Account {account_id} was closed on {closure_date}")]
    AccountClosed {
        account_id: Uuid,
        closure_date: NaiveDate,
    },

    #[error("Insufficient funds in account {account_id}: requested {requested}, available {available}")]
    InsufficientFunds {
        account_id: Uuid,
        requested: Decimal,
        available: Decimal,
    },

    #[error("Account {account_id} is not operational: {reason}")]
    AccountNotOperational {
        account_id: Uuid,
        reason: String,
    },

    #[error("Account {account_id} is not in a transactional state")]
    AccountNotTransactional { account_id: Uuid },

    // Customer-related errors
    #[error("Customer not found: {0}")]
    CustomerNotFound(Uuid),

    #[error("Customer {customer_id} is deceased (date of death: {date_of_death})")]
    CustomerDeceased {
        customer_id: Uuid,
        date_of_death: NaiveDate,
    },

    #[error("Customer {customer_id} is blacklisted: {blacklist_reason}")]
    CustomerBlacklisted {
        customer_id: Uuid,
        blacklist_reason: String,
    },

    // Transaction-related errors
    #[error("Transaction limit exceeded: attempted {attempted}, limit {limit} for {limit_type:?}")]
    TransactionLimitExceeded {
        limit: Decimal,
        attempted: Decimal,
        limit_type: LimitType,
    },

    #[error("Invalid signature: required {required_signatories:?}, provided {provided_signatories:?}")]
    InvalidSignature {
        required_signatories: Vec<Uuid>,
        provided_signatories: Vec<Uuid>,
    },

    #[error("Approval required for transaction {transaction_id}: required approvers {required_approvers:?}")]
    ApprovalRequired {
        transaction_id: Uuid,
        required_approvers: Vec<Uuid>,
    },

    // Compliance-related errors
    #[error("Compliance violation: {violation_type} for customer {customer_id:?}")]
    ComplianceViolation {
        violation_type: String,
        customer_id: Option<Uuid>,
    },

    #[error("KYC incomplete for customer {customer_id}: missing documents {missing_documents:?}")]
    KycIncomplete {
        customer_id: Uuid,
        missing_documents: Vec<String>,
    },

    #[error("Sanctions match for customer {customer_id}: {match_details}")]
    SanctionsMatch {
        customer_id: Uuid,
        match_details: String,
    },

    // Agent Network Hierarchy Limit Violations
    #[error("Branch limit violation: branch {limit_type} limit ({branch_limit}) exceeds network limit ({network_limit})")]
    BranchLimitExceedsNetwork {
        branch_limit: Decimal,
        network_limit: Decimal,
        limit_type: String, // "transaction" or "daily"
    },

    #[error("Terminal limit violation: terminal {limit_type} limit ({terminal_limit}) exceeds branch limit ({branch_limit})")]
    TerminalLimitExceedsBranch {
        terminal_limit: Decimal,
        branch_limit: Decimal,
        limit_type: String, // "transaction" or "daily"
    },

    #[error("Agent network entity inactive: {entity_type} {entity_id} has status '{status}'")]
    AgentNetworkEntityInactive {
        entity_type: String, // "Network", "Branch", "Terminal"
        entity_id: Uuid,
        status: String,
    },

    #[error("Hierarchical validation failed: {validation_errors:?}")]
    HierarchicalValidationFailed {
        validation_errors: Vec<String>,
    },

    // Calendar and Business Day Validation
    #[error("Invalid weekend days configuration: {invalid_days:?} - days must be between 1 (Monday) and 7 (Sunday)")]
    InvalidWeekendDays { invalid_days: Vec<i32> },

    #[error("Weekend configuration validation failed: {validation_errors:?}")]
    WeekendConfigValidationFailed {
        validation_errors: Vec<String>,
    },

    // Product and system errors
    #[error("Invalid product id: {0}")]
    InvalidProductId(Uuid),

    #[error("Product not found: {0}")]
    ProductNotFound(Uuid),

    #[error("Product catalog unavailable for {product_id}, fallback used: {fallback_used}")]
    ProductCatalogUnavailable {
        product_id: Uuid,
        fallback_used: bool,
    },

    #[error("Business day calculation error for date {date} in jurisdiction {jurisdiction}")]
    BusinessDayCalculationError {
        date: NaiveDate,
        jurisdiction: String,
    },

    // Network and infrastructure
    #[error("Network error: {error_details}, retry possible: {retry_possible}")]
    NetworkError {
        error_details: String,
        retry_possible: bool,
    },

    #[error("Database constraint violation: {constraint} - {details}")]
    DatabaseConstraintViolation {
        constraint: String,
        details: String,
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

    // Not implemented features
    #[error("Feature not implemented: {0}")]
    NotImplemented(String),
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
            }
            _ => BankingError::Internal(format!("Database error: {err}")),
        }
    }
}
impl From<Box<dyn std::error::Error + Send + Sync>> for BankingError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        BankingError::Internal(err.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LimitType {
    Daily,
    Monthly,
    PerTransaction,
    Terminal,
    Branch,
    Network,
}
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Database model for Transaction table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct TransactionModel {
    pub transaction_id: Uuid,
    pub account_id: Uuid,
    pub transaction_code: String,
    pub transaction_type: String,
    pub amount: Decimal,
    pub currency: String,
    pub description: String,
    pub channel_id: String,
    pub terminal_id: Option<Uuid>,
    pub agent_user_id: Option<Uuid>,
    pub transaction_date: DateTime<Utc>,
    pub value_date: NaiveDate,
    pub status: String,
    pub reference_number: String,
    pub external_reference: Option<String>,
    pub gl_code: String,
    pub requires_approval: bool,
    pub approval_status: Option<String>,
    pub risk_score: Option<Decimal>,
    pub created_at: DateTime<Utc>,
}

/// Database model for Transaction Approvals
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct TransactionApprovalModel {
    pub approval_id: Uuid,
    pub transaction_id: Uuid,
    pub approver_id: Uuid,
    pub approval_status: String,
    pub approved_at: DateTime<Utc>,
    pub notes: Option<String>,
}

/// Database model for Approval Workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct TransactionApprovalWorkflowModel {
    pub workflow_id: Uuid,
    pub transaction_id: Uuid,
    pub required_approvers: String, // JSON array
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub timeout_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Database model for GL Entries
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct GlEntryModel {
    pub entry_id: Uuid,
    pub transaction_id: Uuid,
    pub account_code: String,
    pub debit_amount: Option<Decimal>,
    pub credit_amount: Option<Decimal>,
    pub currency: String,
    pub description: String,
    pub reference_number: String,
    pub value_date: NaiveDate,
    pub posting_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Database model for Transaction Audit Trail
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct TransactionAuditModel {
    pub audit_id: Uuid,
    pub transaction_id: Uuid,
    pub action: String,
    pub performed_by: String,
    pub performed_at: DateTime<Utc>,
    pub details: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}
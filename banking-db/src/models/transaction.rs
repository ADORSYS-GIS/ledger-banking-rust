use blake3::Hash;
use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;
use banking_api::domain::{TransactionType, TransactionStatus, TransactionApprovalStatus};

/// Database model for Transaction table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct TransactionModel {
    pub transaction_id: Uuid,
    pub account_id: Uuid,
    pub transaction_code: HeaplessString<8>,
    #[serde(
        serialize_with = "serialize_transaction_type",
        deserialize_with = "deserialize_transaction_type"
    )]
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    pub currency: HeaplessString<3>,
    pub description: HeaplessString<500>,
    pub channel_id: HeaplessString<50>,
    pub terminal_id: Option<Uuid>,
    pub agent_user_id: Option<Uuid>,
    pub transaction_date: DateTime<Utc>,
    pub value_date: NaiveDate,
    #[serde(
        serialize_with = "serialize_transaction_status",
        deserialize_with = "deserialize_transaction_status"
    )]
    pub status: TransactionStatus,
    pub reference_number: HeaplessString<100>,
    pub external_reference: Option<HeaplessString<100>>,
    pub gl_code: HeaplessString<10>,
    pub requires_approval: bool,
    #[serde(
        serialize_with = "serialize_approval_status_option",
        deserialize_with = "deserialize_approval_status_option"
    )]
    pub approval_status: Option<TransactionApprovalStatus>,
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
    pub currency: HeaplessString<3>,
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
    /// References ReferencedPerson.person_id
    pub performed_by: Uuid,
    pub performed_at: DateTime<Utc>,
    pub details: Option<Hash>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

// Transaction enum serialization helpers for database compatibility
fn serialize_transaction_type<S>(transaction_type: &TransactionType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let type_str = match transaction_type {
        TransactionType::Credit => "Credit",
        TransactionType::Debit => "Debit",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_transaction_type<'de, D>(deserializer: D) -> Result<TransactionType, D::Error>
where
    D: Deserializer<'de>,
{
    let type_str = String::deserialize(deserializer)?;
    match type_str.as_str() {
        "Credit" => Ok(TransactionType::Credit),
        "Debit" => Ok(TransactionType::Debit),
        _ => Err(serde::de::Error::custom(format!("Invalid transaction type: {type_str}"))),
    }
}

fn serialize_transaction_status<S>(status: &TransactionStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let status_str = match status {
        TransactionStatus::Pending => "Pending",
        TransactionStatus::Posted => "Posted",
        TransactionStatus::Reversed => "Reversed",
        TransactionStatus::Failed => "Failed",
        TransactionStatus::AwaitingApproval => "AwaitingApproval",
        TransactionStatus::ApprovalRejected => "ApprovalRejected",
    };
    serializer.serialize_str(status_str)
}

fn deserialize_transaction_status<'de, D>(deserializer: D) -> Result<TransactionStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let status_str = String::deserialize(deserializer)?;
    match status_str.as_str() {
        "Pending" => Ok(TransactionStatus::Pending),
        "Posted" => Ok(TransactionStatus::Posted),
        "Reversed" => Ok(TransactionStatus::Reversed),
        "Failed" => Ok(TransactionStatus::Failed),
        "AwaitingApproval" => Ok(TransactionStatus::AwaitingApproval),
        "ApprovalRejected" => Ok(TransactionStatus::ApprovalRejected),
        _ => Err(serde::de::Error::custom(format!("Invalid transaction status: {status_str}"))),
    }
}

fn serialize_approval_status_option<S>(approval_status: &Option<TransactionApprovalStatus>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match approval_status {
        Some(status) => {
            let status_str = match status {
                TransactionApprovalStatus::Pending => "Pending",
                TransactionApprovalStatus::Approved => "Approved",
                TransactionApprovalStatus::Rejected => "Rejected",
                TransactionApprovalStatus::PartiallyApproved => "PartiallyApproved",
            };
            serializer.serialize_some(status_str)
        }
        None => serializer.serialize_none(),
    }
}

fn deserialize_approval_status_option<'de, D>(deserializer: D) -> Result<Option<TransactionApprovalStatus>, D::Error>
where
    D: Deserializer<'de>,
{
    let status_opt: Option<String> = Option::deserialize(deserializer)?;
    match status_opt {
        Some(status_str) => {
            let status = match status_str.as_str() {
                "Pending" => TransactionApprovalStatus::Pending,
                "Approved" => TransactionApprovalStatus::Approved,
                "Rejected" => TransactionApprovalStatus::Rejected,
                "PartiallyApproved" => TransactionApprovalStatus::PartiallyApproved,
                _ => return Err(serde::de::Error::custom(format!("Invalid approval status: {status_str}"))),
            };
            Ok(Some(status))
        }
        None => Ok(None),
    }
}




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
    #[serde(
        serialize_with = "serialize_transaction_code",
        deserialize_with = "deserialize_transaction_code"
    )]
    pub transaction_code: [u8; 8],
    #[serde(
        serialize_with = "serialize_transaction_type",
        deserialize_with = "deserialize_transaction_type"
    )]
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    #[serde(
        serialize_with = "serialize_currency",
        deserialize_with = "deserialize_currency"
    )]
    pub currency: [u8; 3],
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
    #[serde(
        serialize_with = "serialize_gl_code",
        deserialize_with = "deserialize_gl_code"
    )]
    pub gl_code: [u8; 10],
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
    #[serde(
        serialize_with = "serialize_currency",
        deserialize_with = "deserialize_currency"
    )]
    pub currency: [u8; 3],
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

// Currency serialization helpers for ISO 4217 compliance
fn serialize_currency<S>(currency: &[u8; 3], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let currency_str = std::str::from_utf8(currency)
        .map_err(|_| serde::ser::Error::custom("Invalid UTF-8 in currency code"))?;
    serializer.serialize_str(currency_str)
}

fn deserialize_currency<'de, D>(deserializer: D) -> Result<[u8; 3], D::Error>
where
    D: Deserializer<'de>,
{
    let currency_str = String::deserialize(deserializer)?;
    if currency_str.len() != 3 {
        return Err(serde::de::Error::custom(format!(
            "Currency code must be exactly 3 characters, got {}",
            currency_str.len()
        )));
    }
    
    let currency_bytes = currency_str.as_bytes();
    if !currency_bytes.iter().all(|&b| b.is_ascii_alphabetic() && b.is_ascii_uppercase()) {
        return Err(serde::de::Error::custom(
            "Currency code must contain only uppercase ASCII letters"
        ));
    }
    
    Ok([currency_bytes[0], currency_bytes[1], currency_bytes[2]])
}

// Transaction code serialization helpers for banking transaction codes (up to 8 chars)
fn serialize_transaction_code<S>(transaction_code: &[u8; 8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let end = transaction_code.iter().position(|&b| b == 0).unwrap_or(8);
    let code_str = std::str::from_utf8(&transaction_code[..end])
        .map_err(|_| serde::ser::Error::custom("Invalid UTF-8 in transaction code"))?;
    serializer.serialize_str(code_str)
}

fn deserialize_transaction_code<'de, D>(deserializer: D) -> Result<[u8; 8], D::Error>
where
    D: Deserializer<'de>,
{
    let code_str = String::deserialize(deserializer)?;
    if code_str.len() > 8 {
        return Err(serde::de::Error::custom(format!(
            "Transaction code cannot exceed 8 characters, got {}",
            code_str.len()
        )));
    }
    
    if code_str.is_empty() {
        return Err(serde::de::Error::custom(
            "Transaction code cannot be empty"
        ));
    }
    
    let code_bytes = code_str.as_bytes();
    if !code_bytes.iter().all(|&b| b.is_ascii_alphanumeric() || b == b'_') {
        return Err(serde::de::Error::custom(
            "Transaction code must contain only alphanumeric characters or underscores"
        ));
    }
    
    let mut array = [0u8; 8];
    array[..code_bytes.len()].copy_from_slice(code_bytes);
    Ok(array)
}

// GL code serialization helpers for banking GL codes (up to 10 chars)
fn serialize_gl_code<S>(gl_code: &[u8; 10], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let end = gl_code.iter().position(|&b| b == 0).unwrap_or(10);
    let code_str = std::str::from_utf8(&gl_code[..end])
        .map_err(|_| serde::ser::Error::custom("Invalid UTF-8 in GL code"))?;
    serializer.serialize_str(code_str)
}

fn deserialize_gl_code<'de, D>(deserializer: D) -> Result<[u8; 10], D::Error>
where
    D: Deserializer<'de>,
{
    let code_str = String::deserialize(deserializer)?;
    if code_str.len() > 10 {
        return Err(serde::de::Error::custom(format!(
            "GL code cannot exceed 10 characters, got {}",
            code_str.len()
        )));
    }
    
    if code_str.is_empty() {
        return Err(serde::de::Error::custom(
            "GL code cannot be empty"
        ));
    }
    
    let code_bytes = code_str.as_bytes();
    if !code_bytes.iter().all(|&b| b.is_ascii_alphanumeric()) {
        return Err(serde::de::Error::custom(
            "GL code must contain only alphanumeric characters"
        ));
    }
    
    let mut array = [0u8; 10];
    array[..code_bytes.len()].copy_from_slice(code_bytes);
    Ok(array)
}
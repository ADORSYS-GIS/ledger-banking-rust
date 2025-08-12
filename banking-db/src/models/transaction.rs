use blake3::Hash;
use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;


/// Database model for Transaction table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct TransactionModel {
    pub id: Uuid,
    pub account_id: Uuid,
    pub transaction_code: HeaplessString<8>,
    #[cfg_attr(feature = "sqlx", sqlx(rename = "transaction_type"))]
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    pub currency: HeaplessString<3>,
    pub description: HeaplessString<200>,
    pub channel_id: HeaplessString<50>,
    pub terminal_id: Option<Uuid>,
    pub agent_person_id: Option<Uuid>,
    pub transaction_date: DateTime<Utc>,
    pub value_date: NaiveDate,
    #[cfg_attr(feature = "sqlx", sqlx(rename = "status"))]
    pub status: TransactionStatus,
    pub reference_number: HeaplessString<100>,
    pub external_reference: Option<HeaplessString<100>>,
    pub gl_code: HeaplessString<10>,
    pub requires_approval: bool,
    #[cfg_attr(feature = "sqlx", sqlx(rename = "approval_status"))]
    pub approval_status: Option<TransactionApprovalStatus>,
    pub risk_score: Option<Decimal>,
    pub created_at: DateTime<Utc>,
}

/// Database model for Transaction Approvals
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct TransactionApprovalModel {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub required: bool,
    pub approver_person_id: Uuid,
    #[cfg_attr(feature = "sqlx", sqlx(rename = "approval_status"))]
    pub approval_status: TransactionApprovalStatus,
    pub approved_at: DateTime<Utc>,
    pub notes: Option<HeaplessString<500>>,
    pub created_at: DateTime<Utc>,
}

/// Database model for Approval Workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct TransactionApprovalWorkflowModel {
    pub id: Uuid,
    pub transaction_id: Uuid,
    #[cfg_attr(feature = "sqlx", sqlx(rename = "status"))]
    pub status: TransactionWorkflowStatus,
    pub timeout_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Database model for GL Entries
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct GlEntryModel {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub account_code: Uuid,
    pub debit_amount: Option<Decimal>,
    pub credit_amount: Option<Decimal>,
    pub currency: HeaplessString<3>,
    pub description: HeaplessString<200>,
    pub reference_number: HeaplessString<50>,
    pub value_date: NaiveDate,
    pub posting_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Database model for Transaction Audit Trail
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct TransactionAuditModel {
    pub id: Uuid,
    pub transaction_id: Uuid,
    #[cfg_attr(feature = "sqlx", sqlx(rename = "action_type"))]
    pub action_type: TransactionAuditAction,
    /// References Person.person_id
    pub performed_by_person_id: Uuid,
    pub performed_at: DateTime<Utc>,
    #[cfg_attr(feature = "sqlx", sqlx(rename = "old_status"))]
    pub old_status: Option<TransactionStatus>,
    #[cfg_attr(feature = "sqlx", sqlx(rename = "new_status"))]
    pub new_status: Option<TransactionStatus>,
    /// References ReasonAndPurpose.id for audit reason
    pub reason_id: Option<Uuid>,
    /// Blake3 hash of additional details for tamper detection
    #[serde(
        serialize_with = "serialize_hash_option",
        deserialize_with = "deserialize_hash_option"
    )]
    pub details: Option<Hash>,
}

/// Database model for Transaction Request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct TransactionRequestModel {
    pub id: Uuid,
    pub account_id: Uuid,
    #[cfg_attr(feature = "sqlx", sqlx(rename = "transaction_type"))]
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    pub currency: HeaplessString<3>,
    pub description: HeaplessString<200>,
    #[cfg_attr(feature = "sqlx", sqlx(rename = "channel"))]
    pub channel: ChannelType,
    pub terminal_id: Option<Uuid>,
    pub initiator_person_id: Uuid,
    pub external_reference: Option<HeaplessString<100>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct TransactionRequestMetadataModel {
    pub id: Uuid,
    pub transaction_request_id: Uuid,
    pub key: HeaplessString<50>,
    pub value: HeaplessString<500>,
}

/// Database model for Transaction Result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct TransactionResultModel {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub reference_number: HeaplessString<50>,
    pub timestamp: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Database model for Validation Result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct TransactionValidationResultModel {
    pub id: Uuid,
    pub is_valid: bool,
    pub transaction_id: Option<Uuid>,
    pub validation_error_01_field: Option<HeaplessString<50>>,
    pub validation_error_01_message: Option<HeaplessString<200>>,
    pub validation_error_01_error_code: Option<HeaplessString<50>>,
    pub validation_error_02_field: Option<HeaplessString<50>>,
    pub validation_error_02_message: Option<HeaplessString<200>>,
    pub validation_error_02_error_code: Option<HeaplessString<50>>,
    pub validation_error_03_field: Option<HeaplessString<50>>,
    pub validation_error_03_message: Option<HeaplessString<200>>,
    pub validation_error_03_error_code: Option<HeaplessString<50>>,
    pub warning_01: Option<HeaplessString<200>>,
    pub warning_02: Option<HeaplessString<200>>,
    pub warning_03: Option<HeaplessString<200>>,
    pub created_at: DateTime<Utc>,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "transaction_type", rename_all = "PascalCase"))]
pub enum TransactionType {
    Credit,
    Debit,
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Credit => write!(f, "Credit"),
            TransactionType::Debit => write!(f, "Debit"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "transaction_status", rename_all = "PascalCase"))]
pub enum TransactionStatus {
    Pending,
    Posted,
    Reversed,
    Failed,
    AwaitingApproval,
    ApprovalRejected,
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionStatus::Pending => write!(f, "Pending"),
            TransactionStatus::Posted => write!(f, "Posted"),
            TransactionStatus::Reversed => write!(f, "Reversed"),
            TransactionStatus::Failed => write!(f, "Failed"),
            TransactionStatus::AwaitingApproval => write!(f, "AwaitingApproval"),
            TransactionStatus::ApprovalRejected => write!(f, "ApprovalRejected"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "transaction_approval_status", rename_all = "PascalCase"))]
pub enum TransactionApprovalStatus {
    Pending,
    Approved,
    Rejected,
    PartiallyApproved,
}

impl std::fmt::Display for TransactionApprovalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionApprovalStatus::Pending => write!(f, "Pending"),
            TransactionApprovalStatus::Approved => write!(f, "Approved"),
            TransactionApprovalStatus::Rejected => write!(f, "Rejected"),
            TransactionApprovalStatus::PartiallyApproved => write!(f, "PartiallyApproved"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "transaction_workflow_status", rename_all = "PascalCase"))]
pub enum TransactionWorkflowStatus {
    Pending,
    Approved,
    Rejected,
    TimedOut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "transaction_audit_action", rename_all = "PascalCase"))]
pub enum TransactionAuditAction {
    Created,
    StatusChanged,
    Posted,
    Reversed,
    Failed,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "channel_type", rename_all = "PascalCase"))]
pub enum ChannelType {
    MobileApp,
    AgentTerminal,
    ATM,
    InternetBanking,
    BranchTeller,
    USSD,
    ApiGateway,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "permitted_operation", rename_all = "PascalCase"))]
pub enum PermittedOperation {
    Credit,
    Debit,
    InterestPosting,
    FeeApplication,
    ClosureSettlement,
    None,
}

// Blake3 Hash serialization helpers
fn serialize_hash_option<S>(hash: &Option<Hash>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match hash {
        Some(hash) => {
            let hash_bytes = hash.as_bytes();
            serializer.serialize_some(hash_bytes)
        }
        None => serializer.serialize_none(),
    }
}

fn deserialize_hash_option<'de, D>(deserializer: D) -> Result<Option<Hash>, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes_opt: Option<Vec<u8>> = Option::deserialize(deserializer)?;
    match bytes_opt {
        Some(bytes) => {
            if bytes.len() != 32 {
                return Err(serde::de::Error::custom(format!("Invalid hash length: expected 32 bytes, got {}", bytes.len())));
            }
            let mut hash_array = [0u8; 32];
            hash_array.copy_from_slice(&bytes);
            Ok(Some(Hash::from(hash_array)))
        }
        None => Ok(None),
    }
}


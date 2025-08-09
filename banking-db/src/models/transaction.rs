use blake3::Hash;
use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;
pub use banking_api::domain::{
    TransactionType, TransactionStatus, TransactionApprovalStatus, TransactionWorkflowStatus, 
    TransactionAuditAction, ChannelType, PermittedOperation
};


/// Database model for Transaction table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct TransactionModel {
    pub id: Uuid,
    pub account_id: Uuid,
    pub transaction_code: HeaplessString<8>,
    #[serde(
        serialize_with = "serialize_transaction_type",
        deserialize_with = "deserialize_transaction_type"
    )]
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    pub currency: HeaplessString<3>,
    pub description: HeaplessString<200>,
    pub channel_id: HeaplessString<50>,
    pub terminal_id: Option<Uuid>,
    pub agent_person_id: Option<Uuid>,
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
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub approver_person_id: Uuid,
    #[serde(
        serialize_with = "serialize_transaction_approval_status",
        deserialize_with = "deserialize_transaction_approval_status"
    )]
    pub approval_status: TransactionApprovalStatus,
    pub approved_at: DateTime<Utc>,
    pub notes: Option<String>,
}

/// Database model for Approval Workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct TransactionApprovalWorkflowModel {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub required_approvers: String, // JSON array
    #[serde(
        serialize_with = "serialize_transaction_workflow_status",
        deserialize_with = "deserialize_transaction_workflow_status"
    )]
    pub status: TransactionWorkflowStatus,
    pub created_at: DateTime<Utc>,
    pub timeout_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
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
    pub reference_number: HeaplessString<200>,
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
    #[serde(
        serialize_with = "serialize_transaction_audit_action",
        deserialize_with = "deserialize_transaction_audit_action"
    )]
    pub action_type: TransactionAuditAction,
    /// References Person.person_id
    pub performed_by_person_id: Uuid,
    pub performed_at: DateTime<Utc>,
    #[serde(
        serialize_with = "serialize_transaction_status_option",
        deserialize_with = "deserialize_transaction_status_option"
    )]
    pub old_status: Option<TransactionStatus>,
    #[serde(
        serialize_with = "serialize_transaction_status_option",
        deserialize_with = "deserialize_transaction_status_option"
    )]
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
    #[serde(
        serialize_with = "serialize_transaction_type",
        deserialize_with = "deserialize_transaction_type"
    )]
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    pub currency: HeaplessString<3>,
    pub description: HeaplessString<200>,
    #[serde(
        serialize_with = "serialize_channel_type",
        deserialize_with = "deserialize_channel_type"
    )]
    pub channel: ChannelType,
    pub terminal_id: Option<Uuid>,
    pub initiator_person_id: Uuid, // References Person.person_id
    pub external_reference: Option<HeaplessString<100>>,
    pub metadata: String, // JSON-serialized HashMap
    pub created_at: DateTime<Utc>,
}

/// Database model for Transaction Result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct TransactionResultModel {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub reference_number: HeaplessString<200>,
    pub timestamp: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Database model for Validation Result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct ValidationResultModel {
    pub id: Uuid,
    pub transaction_id: Option<Uuid>,
    pub is_valid: bool,
    pub errors: String, // JSON array
    pub warnings: String, // JSON array
    pub created_at: DateTime<Utc>,
}

/// Database model for Approval
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct ApprovalModel {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub approver_person_id: Uuid,
    pub approved_at: DateTime<Utc>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
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

// TransactionAuditAction serialization helpers
fn serialize_transaction_audit_action<S>(action: &TransactionAuditAction, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let action_str = match action {
        TransactionAuditAction::Created => "Created",
        TransactionAuditAction::StatusChanged => "StatusChanged",
        TransactionAuditAction::Posted => "Posted",
        TransactionAuditAction::Reversed => "Reversed",
        TransactionAuditAction::Failed => "Failed",
        TransactionAuditAction::Approved => "Approved",
        TransactionAuditAction::Rejected => "Rejected",
    };
    serializer.serialize_str(action_str)
}

fn deserialize_transaction_audit_action<'de, D>(deserializer: D) -> Result<TransactionAuditAction, D::Error>
where
    D: Deserializer<'de>,
{
    let action_str = String::deserialize(deserializer)?;
    match action_str.as_str() {
        "Created" => Ok(TransactionAuditAction::Created),
        "StatusChanged" => Ok(TransactionAuditAction::StatusChanged),
        "Posted" => Ok(TransactionAuditAction::Posted),
        "Reversed" => Ok(TransactionAuditAction::Reversed),
        "Failed" => Ok(TransactionAuditAction::Failed),
        "Approved" => Ok(TransactionAuditAction::Approved),
        "Rejected" => Ok(TransactionAuditAction::Rejected),
        _ => Err(serde::de::Error::custom(format!("Invalid transaction audit action: {action_str}"))),
    }
}

// TransactionStatus Option serialization helpers  
fn serialize_transaction_status_option<S>(status: &Option<TransactionStatus>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match status {
        Some(status) => {
            let status_str = match status {
                TransactionStatus::Pending => "Pending",
                TransactionStatus::Posted => "Posted",
                TransactionStatus::Reversed => "Reversed",
                TransactionStatus::Failed => "Failed",
                TransactionStatus::AwaitingApproval => "AwaitingApproval",
                TransactionStatus::ApprovalRejected => "ApprovalRejected",
            };
            serializer.serialize_some(status_str)
        }
        None => serializer.serialize_none(),
    }
}

fn deserialize_transaction_status_option<'de, D>(deserializer: D) -> Result<Option<TransactionStatus>, D::Error>
where
    D: Deserializer<'de>,
{
    let status_opt: Option<String> = Option::deserialize(deserializer)?;
    match status_opt {
        Some(status_str) => {
            let status = match status_str.as_str() {
                "Pending" => TransactionStatus::Pending,
                "Posted" => TransactionStatus::Posted,
                "Reversed" => TransactionStatus::Reversed,
                "Failed" => TransactionStatus::Failed,
                "AwaitingApproval" => TransactionStatus::AwaitingApproval,
                "ApprovalRejected" => TransactionStatus::ApprovalRejected,
                _ => return Err(serde::de::Error::custom(format!("Invalid transaction status: {status_str}"))),
            };
            Ok(Some(status))
        }
        None => Ok(None),
    }
}

// TransactionApprovalStatus serialization helpers
fn serialize_transaction_approval_status<S>(status: &TransactionApprovalStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let status_str = match status {
        TransactionApprovalStatus::Pending => "Pending",
        TransactionApprovalStatus::Approved => "Approved",
        TransactionApprovalStatus::Rejected => "Rejected",
        TransactionApprovalStatus::PartiallyApproved => "PartiallyApproved",
    };
    serializer.serialize_str(status_str)
}

fn deserialize_transaction_approval_status<'de, D>(deserializer: D) -> Result<TransactionApprovalStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let status_str = String::deserialize(deserializer)?;
    match status_str.as_str() {
        "Pending" => Ok(TransactionApprovalStatus::Pending),
        "Approved" => Ok(TransactionApprovalStatus::Approved),
        "Rejected" => Ok(TransactionApprovalStatus::Rejected),
        "PartiallyApproved" => Ok(TransactionApprovalStatus::PartiallyApproved),
        _ => Err(serde::de::Error::custom(format!("Invalid transaction approval status: {status_str}"))),
    }
}

// TransactionWorkflowStatus serialization helpers
fn serialize_transaction_workflow_status<S>(status: &TransactionWorkflowStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let status_str = match status {
        TransactionWorkflowStatus::Pending => "Pending",
        TransactionWorkflowStatus::Approved => "Approved",
        TransactionWorkflowStatus::Rejected => "Rejected",
        TransactionWorkflowStatus::TimedOut => "TimedOut",
    };
    serializer.serialize_str(status_str)
}

fn deserialize_transaction_workflow_status<'de, D>(deserializer: D) -> Result<TransactionWorkflowStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let status_str = String::deserialize(deserializer)?;
    match status_str.as_str() {
        "Pending" => Ok(TransactionWorkflowStatus::Pending),
        "Approved" => Ok(TransactionWorkflowStatus::Approved),
        "Rejected" => Ok(TransactionWorkflowStatus::Rejected),
        "TimedOut" => Ok(TransactionWorkflowStatus::TimedOut),
        _ => Err(serde::de::Error::custom(format!("Invalid transaction workflow status: {status_str}"))),
    }
}

// ChannelType serialization helpers
fn serialize_channel_type<S>(channel: &ChannelType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let channel_str = match channel {
        ChannelType::MobileApp => "MobileApp",
        ChannelType::AgentTerminal => "AgentTerminal",
        ChannelType::ATM => "ATM",
        ChannelType::InternetBanking => "InternetBanking",
        ChannelType::BranchTeller => "BranchTeller",
        ChannelType::USSD => "USSD",
        ChannelType::ApiGateway => "ApiGateway",
    };
    serializer.serialize_str(channel_str)
}

fn deserialize_channel_type<'de, D>(deserializer: D) -> Result<ChannelType, D::Error>
where
    D: Deserializer<'de>,
{
    let channel_str = String::deserialize(deserializer)?;
    match channel_str.as_str() {
        "MobileApp" => Ok(ChannelType::MobileApp),
        "AgentTerminal" => Ok(ChannelType::AgentTerminal),
        "ATM" => Ok(ChannelType::ATM),
        "InternetBanking" => Ok(ChannelType::InternetBanking),
        "BranchTeller" => Ok(ChannelType::BranchTeller),
        "USSD" => Ok(ChannelType::USSD),
        "ApiGateway" => Ok(ChannelType::ApiGateway),
        _ => Err(serde::de::Error::custom(format!("Invalid channel type: {channel_str}"))),
    }
}

// PermittedOperation serialization helpers (unused but kept for future use)
#[allow(dead_code)]
fn serialize_permitted_operation<S>(operation: &PermittedOperation, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let operation_str = match operation {
        PermittedOperation::Credit => "Credit",
        PermittedOperation::Debit => "Debit",
        PermittedOperation::InterestPosting => "InterestPosting",
        PermittedOperation::FeeApplication => "FeeApplication",
        PermittedOperation::ClosureSettlement => "ClosureSettlement",
        PermittedOperation::None => "None",
    };
    serializer.serialize_str(operation_str)
}

#[allow(dead_code)]
fn deserialize_permitted_operation<'de, D>(deserializer: D) -> Result<PermittedOperation, D::Error>
where
    D: Deserializer<'de>,
{
    let operation_str = String::deserialize(deserializer)?;
    match operation_str.as_str() {
        "Credit" => Ok(PermittedOperation::Credit),
        "Debit" => Ok(PermittedOperation::Debit),
        "InterestPosting" => Ok(PermittedOperation::InterestPosting),
        "FeeApplication" => Ok(PermittedOperation::FeeApplication),
        "ClosureSettlement" => Ok(PermittedOperation::ClosureSettlement),
        "None" => Ok(PermittedOperation::None),
        _ => Err(serde::de::Error::custom(format!("Invalid permitted operation: {operation_str}"))),
    }
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


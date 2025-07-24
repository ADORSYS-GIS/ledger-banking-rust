use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

/// Agent Network database model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentNetworkModel {
    pub network_id: Uuid,
    pub network_name: String,
    pub network_type: String, // Internal, Partner, ThirdParty
    pub status: String,       // Active, Suspended, Terminated
    pub contract_id: Option<Uuid>,
    pub aggregate_daily_limit: Decimal,
    pub current_daily_volume: Decimal,
    #[serde(
        serialize_with = "serialize_gl_code",
        deserialize_with = "deserialize_gl_code"
    )]
    pub settlement_gl_code: [u8; 10],
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: String,
}

/// Agency Branch database model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgencyBranchModel {
    pub branch_id: Uuid,
    pub network_id: Uuid,
    pub parent_branch_id: Option<Uuid>,
    pub branch_name: String,
    #[serde(
        serialize_with = "serialize_branch_code",
        deserialize_with = "deserialize_branch_code"
    )]
    pub branch_code: [u8; 8],
    pub branch_level: i32,
    #[serde(
        serialize_with = "serialize_gl_code_prefix",
        deserialize_with = "deserialize_gl_code_prefix"
    )]
    pub gl_code_prefix: [u8; 6],
    pub geolocation: Option<String>,
    pub status: String, // Active, Suspended, Closed
    pub daily_transaction_limit: Decimal,
    pub current_daily_volume: Decimal,
    pub max_cash_limit: Decimal,
    pub current_cash_balance: Decimal,
    pub minimum_cash_balance: Decimal,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: String,
}

/// Agent Terminal database model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTerminalModel {
    pub terminal_id: Uuid,
    pub branch_id: Uuid,
    pub agent_user_id: Uuid,
    pub terminal_type: String, // Pos, Mobile, Atm, WebPortal
    pub terminal_name: String,
    pub daily_transaction_limit: Decimal,
    pub current_daily_volume: Decimal,
    pub max_cash_limit: Decimal,
    pub current_cash_balance: Decimal,
    pub minimum_cash_balance: Decimal,
    pub status: String, // Active, Maintenance, Suspended, Decommissioned
    pub last_sync_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: String,
}

/// Cash Limit Check database model
#[derive(Debug, Clone)]
pub struct CashLimitCheckModel {
    pub check_id: Uuid,
    pub entity_id: Uuid,
    pub entity_type: String, // Branch, Terminal
    pub requested_amount: Decimal,
    pub operation_type: String, // Withdrawal, Deposit, CashOut, CashIn
    pub validation_result: String, // Approved, InsufficientCash, ExceedsMaxLimit, BelowMinimum
    pub available_amount: Option<Decimal>,
    pub max_limit: Option<Decimal>,
    pub minimum_required: Option<Decimal>,
    pub checked_at: DateTime<Utc>,
    pub checked_by: String,
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

// Branch code serialization helpers for banking branch codes (up to 8 chars)
fn serialize_branch_code<S>(branch_code: &[u8; 8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let end = branch_code.iter().position(|&b| b == 0).unwrap_or(8);
    let code_str = std::str::from_utf8(&branch_code[..end])
        .map_err(|_| serde::ser::Error::custom("Invalid UTF-8 in branch code"))?;
    serializer.serialize_str(code_str)
}

fn deserialize_branch_code<'de, D>(deserializer: D) -> Result<[u8; 8], D::Error>
where
    D: Deserializer<'de>,
{
    let code_str = String::deserialize(deserializer)?;
    if code_str.len() > 8 {
        return Err(serde::de::Error::custom(format!(
            "Branch code cannot exceed 8 characters, got {}",
            code_str.len()
        )));
    }
    
    if code_str.is_empty() {
        return Err(serde::de::Error::custom(
            "Branch code cannot be empty"
        ));
    }
    
    let code_bytes = code_str.as_bytes();
    if !code_bytes.iter().all(|&b| b.is_ascii_alphanumeric() || b == b'_') {
        return Err(serde::de::Error::custom(
            "Branch code must contain only alphanumeric characters or underscores"
        ));
    }
    
    let mut array = [0u8; 8];
    array[..code_bytes.len()].copy_from_slice(code_bytes);
    Ok(array)
}

// GL code prefix serialization helpers for short GL prefixes (up to 6 chars)
fn serialize_gl_code_prefix<S>(gl_prefix: &[u8; 6], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let end = gl_prefix.iter().position(|&b| b == 0).unwrap_or(6);
    let prefix_str = std::str::from_utf8(&gl_prefix[..end])
        .map_err(|_| serde::ser::Error::custom("Invalid UTF-8 in GL prefix"))?;
    serializer.serialize_str(prefix_str)
}

fn deserialize_gl_code_prefix<'de, D>(deserializer: D) -> Result<[u8; 6], D::Error>
where
    D: Deserializer<'de>,
{
    let prefix_str = String::deserialize(deserializer)?;
    if prefix_str.len() > 6 {
        return Err(serde::de::Error::custom(format!(
            "GL prefix cannot exceed 6 characters, got {}",
            prefix_str.len()
        )));
    }
    
    if prefix_str.is_empty() {
        return Err(serde::de::Error::custom(
            "GL prefix cannot be empty"
        ));
    }
    
    let prefix_bytes = prefix_str.as_bytes();
    if !prefix_bytes.iter().all(|&b| b.is_ascii_alphanumeric()) {
        return Err(serde::de::Error::custom(
            "GL prefix must contain only alphanumeric characters"
        ));
    }
    
    let mut array = [0u8; 6];
    array[..prefix_bytes.len()].copy_from_slice(prefix_bytes);
    Ok(array)
}
use chrono::{DateTime, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentNetwork {
    pub network_id: Uuid,
    pub network_name: HeaplessString<255>,
    pub network_type: NetworkType,
    pub status: NetworkStatus,
    pub contract_id: Option<Uuid>,
    pub aggregate_daily_limit: Decimal,
    pub current_daily_volume: Decimal,
    #[serde(
        serialize_with = "serialize_gl_code",
        deserialize_with = "deserialize_gl_code"
    )]
    pub settlement_gl_code: [u8; 10],
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgencyBranch {
    pub branch_id: Uuid,
    pub network_id: Uuid,
    pub parent_branch_id: Option<Uuid>,
    pub branch_name: HeaplessString<255>,
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
    pub status: BranchStatus,
    pub daily_transaction_limit: Decimal,
    pub current_daily_volume: Decimal,
    pub max_cash_limit: Decimal,
    pub current_cash_balance: Decimal,
    pub minimum_cash_balance: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTerminal {
    pub terminal_id: Uuid,
    pub branch_id: Uuid,
    pub agent_user_id: Uuid,
    pub terminal_type: TerminalType,
    pub terminal_name: HeaplessString<255>,
    pub daily_transaction_limit: Decimal,
    pub current_daily_volume: Decimal,
    pub max_cash_limit: Decimal,
    pub current_cash_balance: Decimal,
    pub minimum_cash_balance: Decimal,
    pub status: TerminalStatus,
    pub last_sync_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NetworkType { 
    Internal, 
    Partner, 
    ThirdParty 
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NetworkStatus { 
    Active, 
    Suspended, 
    Terminated 
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BranchStatus { 
    Active, 
    Suspended, 
    Closed 
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TerminalType { 
    Pos, 
    Mobile, 
    Atm, 
    WebPortal 
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TerminalStatus { 
    Active, 
    Maintenance, 
    Suspended, 
    Decommissioned 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalLimits {
    pub daily_limit: Decimal,
    pub per_transaction_limit: Decimal,
    pub monthly_limit: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerminalLimitResult {
    Approved,
    Denied { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CashLimitValidation {
    Approved,
    InsufficientCash { available: Decimal, required: Decimal },
    ExceedsMaxLimit { current: Decimal, max_limit: Decimal },
    BelowMinimum { current: Decimal, minimum: Decimal },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashLimitCheck {
    pub entity_id: Uuid,
    pub entity_type: CashLimitEntityType,
    pub requested_amount: Decimal,
    pub operation_type: CashOperationType,
    pub validation_result: CashLimitValidation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CashLimitEntityType {
    Branch,
    Terminal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CashOperationType {
    Withdrawal,
    Deposit,
    CashOut,
    CashIn,
}

// GL code serialization helpers for banking GL codes (up to 10 chars)
fn serialize_gl_code<S>(gl_code: &[u8; 10], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let end = gl_code.iter().position(|&b| b == 0).unwrap_or(10);
    let code_str = std::str::from_utf8(&gl_code[..end])
        .map_err(|_| serde::ser::Error::custom("Invalid UTF-8 in GL code"))?;
    serializer.serialize_str(code_str)
}

fn deserialize_gl_code<'de, D>(deserializer: D) -> Result<[u8; 10], D::Error>
where
    D: serde::Deserializer<'de>,
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
    S: serde::Serializer,
{
    let end = branch_code.iter().position(|&b| b == 0).unwrap_or(8);
    let code_str = std::str::from_utf8(&branch_code[..end])
        .map_err(|_| serde::ser::Error::custom("Invalid UTF-8 in branch code"))?;
    serializer.serialize_str(code_str)
}

fn deserialize_branch_code<'de, D>(deserializer: D) -> Result<[u8; 8], D::Error>
where
    D: serde::Deserializer<'de>,
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
    S: serde::Serializer,
{
    let end = gl_prefix.iter().position(|&b| b == 0).unwrap_or(6);
    let prefix_str = std::str::from_utf8(&gl_prefix[..end])
        .map_err(|_| serde::ser::Error::custom("Invalid UTF-8 in GL prefix"))?;
    serializer.serialize_str(prefix_str)
}

fn deserialize_gl_code_prefix<'de, D>(deserializer: D) -> Result<[u8; 6], D::Error>
where
    D: serde::Deserializer<'de>,
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

impl AgentNetwork {
    /// Convert settlement_gl_code array to string for use in APIs
    pub fn settlement_gl_code_as_str(&self) -> &str {
        let end = self.settlement_gl_code.iter().position(|&b| b == 0).unwrap_or(10);
        std::str::from_utf8(&self.settlement_gl_code[..end]).unwrap_or("")
    }
}

impl AgencyBranch {
    /// Convert branch_code array to string for use in APIs
    pub fn branch_code_as_str(&self) -> &str {
        let end = self.branch_code.iter().position(|&b| b == 0).unwrap_or(8);
        std::str::from_utf8(&self.branch_code[..end]).unwrap_or("")
    }
    
    /// Convert gl_code_prefix array to string for use in APIs
    pub fn gl_code_prefix_as_str(&self) -> &str {
        let end = self.gl_code_prefix.iter().position(|&b| b == 0).unwrap_or(6);
        std::str::from_utf8(&self.gl_code_prefix[..end]).unwrap_or("")
    }
}
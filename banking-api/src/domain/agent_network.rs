use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AgentNetwork {
    pub network_id: Uuid,
    #[validate(length(min = 1, max = 255))]
    pub network_name: String,
    pub network_type: NetworkType,
    pub status: NetworkStatus,
    pub contract_id: Option<Uuid>,
    pub aggregate_daily_limit: Decimal,
    pub current_daily_volume: Decimal,
    #[validate(length(min = 1, max = 50))]
    pub settlement_gl_code: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AgencyBranch {
    pub branch_id: Uuid,
    pub network_id: Uuid,
    pub parent_branch_id: Option<Uuid>,
    #[validate(length(min = 1, max = 255))]
    pub branch_name: String,
    #[validate(length(min = 1, max = 50))]
    pub branch_code: String,
    pub branch_level: i32,
    #[validate(length(min = 1, max = 20))]
    pub gl_code_prefix: String,
    #[validate(length(max = 255))]
    pub geolocation: Option<String>,
    pub status: BranchStatus,
    pub daily_transaction_limit: Decimal,
    pub current_daily_volume: Decimal,
    pub max_cash_limit: Decimal,
    pub current_cash_balance: Decimal,
    pub minimum_cash_balance: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AgentTerminal {
    pub terminal_id: Uuid,
    pub branch_id: Uuid,
    pub agent_user_id: Uuid,
    pub terminal_type: TerminalType,
    #[validate(length(min = 1, max = 255))]
    pub terminal_name: String,
    pub daily_transaction_limit: Decimal,
    pub current_daily_volume: Decimal,
    pub max_cash_limit: Decimal,
    pub current_cash_balance: Decimal,
    pub minimum_cash_balance: Decimal,
    pub status: TerminalStatus,
    pub last_sync_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkType { 
    Internal, 
    Partner, 
    ThirdParty 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkStatus { 
    Active, 
    Suspended, 
    Terminated 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BranchStatus { 
    Active, 
    Suspended, 
    Closed 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerminalType { 
    Pos, 
    Mobile, 
    Atm, 
    WebPortal 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
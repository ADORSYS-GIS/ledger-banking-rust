use chrono::{DateTime, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
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
    pub settlement_gl_code: HeaplessString<10>,
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
    pub branch_code: HeaplessString<8>,
    pub branch_level: i32,
    pub gl_code_prefix: HeaplessString<6>,
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




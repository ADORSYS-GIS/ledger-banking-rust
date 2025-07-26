// Temporary simplified models that match the current database schema
// These should eventually be moved to the proper models crate

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleBranchModel {
    pub branch_id: Uuid,
    pub network_id: Uuid,
    pub branch_code: String,
    pub branch_name: String,
    pub contact_person: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub physical_address: Option<String>,
    pub status: String,
    pub max_transaction_limit: Decimal,
    pub max_daily_limit: Decimal,
    pub settlement_account_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleTerminalModel {
    pub terminal_id: Uuid,
    pub branch_id: Uuid,
    pub terminal_code: String,
    pub terminal_type: String,
    pub agent_user_id: Uuid,
    pub status: String,
    pub max_transaction_limit: Decimal,
    pub max_daily_limit: Decimal,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleNetworkModel {
    pub network_id: Uuid,
    pub network_code: String,
    pub network_name: String,
    pub network_type: String,
    pub status: String,
    pub max_transaction_limit: Decimal,
    pub max_daily_limit: Decimal,
    pub commission_rate: Decimal,
    pub settlement_gl_code: String,
    pub created_at: DateTime<Utc>,
}
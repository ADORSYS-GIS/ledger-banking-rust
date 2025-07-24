use async_trait::async_trait;
use banking_api::BankingResult;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, NaiveDate};

use crate::models::{AgentNetworkModel, AgencyBranchModel, AgentTerminalModel, CashLimitCheckModel};

#[async_trait]
pub trait AgentNetworkRepository: Send + Sync {
    /// Agent Network Operations
    async fn create_network(&self, network: AgentNetworkModel) -> BankingResult<AgentNetworkModel>;
    async fn update_network(&self, network: AgentNetworkModel) -> BankingResult<AgentNetworkModel>;
    async fn find_network_by_id(&self, network_id: Uuid) -> BankingResult<Option<AgentNetworkModel>>;
    async fn find_networks_by_status(&self, status: &str) -> BankingResult<Vec<AgentNetworkModel>>;
    async fn find_networks_by_type(&self, network_type: &str) -> BankingResult<Vec<AgentNetworkModel>>;
    async fn update_network_daily_volume(&self, network_id: Uuid, amount: Decimal) -> BankingResult<()>;
    async fn reset_network_daily_counters(&self) -> BankingResult<()>;
    async fn list_networks(&self, offset: i64, limit: i64) -> BankingResult<Vec<AgentNetworkModel>>;
    
    /// Agency Branch Operations  
    async fn create_branch(&self, branch: AgencyBranchModel) -> BankingResult<AgencyBranchModel>;
    async fn update_branch(&self, branch: AgencyBranchModel) -> BankingResult<AgencyBranchModel>;
    async fn find_branch_by_id(&self, branch_id: Uuid) -> BankingResult<Option<AgencyBranchModel>>;
    async fn find_branches_by_network(&self, network_id: Uuid) -> BankingResult<Vec<AgencyBranchModel>>;
    async fn find_branches_by_parent(&self, parent_branch_id: Uuid) -> BankingResult<Vec<AgencyBranchModel>>;
    async fn find_branches_by_status(&self, status: &str) -> BankingResult<Vec<AgencyBranchModel>>;
    async fn find_root_branches(&self, network_id: Uuid) -> BankingResult<Vec<AgencyBranchModel>>;
    async fn find_branch_hierarchy(&self, branch_id: Uuid) -> BankingResult<Vec<AgencyBranchModel>>;
    async fn update_branch_daily_volume(&self, branch_id: Uuid, amount: Decimal) -> BankingResult<()>;
    async fn reset_branch_daily_counters(&self) -> BankingResult<()>;
    async fn get_branch_gl_prefix(&self, branch_id: Uuid) -> BankingResult<Option<String>>;
    async fn list_branches(&self, offset: i64, limit: i64) -> BankingResult<Vec<AgencyBranchModel>>;
    
    /// Agent Terminal Operations
    async fn create_terminal(&self, terminal: AgentTerminalModel) -> BankingResult<AgentTerminalModel>;
    async fn update_terminal(&self, terminal: AgentTerminalModel) -> BankingResult<AgentTerminalModel>;
    async fn find_terminal_by_id(&self, terminal_id: Uuid) -> BankingResult<Option<AgentTerminalModel>>;
    async fn find_terminals_by_branch(&self, branch_id: Uuid) -> BankingResult<Vec<AgentTerminalModel>>;
    async fn find_terminals_by_agent(&self, agent_user_id: Uuid) -> BankingResult<Vec<AgentTerminalModel>>;
    async fn find_terminals_by_type(&self, terminal_type: &str) -> BankingResult<Vec<AgentTerminalModel>>;
    async fn find_terminals_by_status(&self, status: &str) -> BankingResult<Vec<AgentTerminalModel>>;
    async fn update_terminal_daily_volume(&self, terminal_id: Uuid, amount: Decimal) -> BankingResult<()>;
    async fn reset_terminal_daily_counters(&self) -> BankingResult<()>;
    async fn update_terminal_sync(&self, terminal_id: Uuid, sync_time: DateTime<Utc>) -> BankingResult<()>;
    async fn find_terminals_needing_sync(&self, threshold: DateTime<Utc>) -> BankingResult<Vec<AgentTerminalModel>>;
    async fn list_terminals(&self, offset: i64, limit: i64) -> BankingResult<Vec<AgentTerminalModel>>;
    
    /// Hierarchical Limit Validation
    async fn get_terminal_limits(&self, terminal_id: Uuid) -> BankingResult<Option<TerminalLimits>>;
    async fn get_branch_limits(&self, branch_id: Uuid) -> BankingResult<Option<BranchLimits>>;
    async fn get_network_limits(&self, network_id: Uuid) -> BankingResult<Option<NetworkLimits>>;
    async fn validate_hierarchical_limits(&self, terminal_id: Uuid, amount: Decimal) -> BankingResult<LimitValidationResult>;
    
    /// Daily Volume Tracking
    async fn get_current_daily_volume_terminal(&self, terminal_id: Uuid) -> BankingResult<Decimal>;
    async fn get_current_daily_volume_branch(&self, branch_id: Uuid) -> BankingResult<Decimal>;
    async fn get_current_daily_volume_network(&self, network_id: Uuid) -> BankingResult<Decimal>;
    
    /// Reporting and Analytics
    async fn get_network_performance(&self, network_id: Uuid, from_date: NaiveDate, to_date: NaiveDate) -> BankingResult<NetworkPerformanceReport>;
    async fn get_branch_performance(&self, branch_id: Uuid, from_date: NaiveDate, to_date: NaiveDate) -> BankingResult<BranchPerformanceReport>;
    async fn get_terminal_performance(&self, terminal_id: Uuid, from_date: NaiveDate, to_date: NaiveDate) -> BankingResult<TerminalPerformanceReport>;
    
    /// Cash Limit Operations
    async fn update_branch_cash_balance(&self, branch_id: Uuid, new_balance: Decimal) -> BankingResult<()>;
    async fn update_terminal_cash_balance(&self, terminal_id: Uuid, new_balance: Decimal) -> BankingResult<()>;
    async fn validate_cash_limit(&self, entity_id: Uuid, entity_type: &str, requested_amount: Decimal, operation_type: &str) -> BankingResult<CashLimitValidationResult>;
    async fn record_cash_limit_check(&self, check: CashLimitCheckModel) -> BankingResult<CashLimitCheckModel>;
    async fn get_cash_limit_history(&self, entity_id: Uuid, from_date: DateTime<Utc>, to_date: DateTime<Utc>) -> BankingResult<Vec<CashLimitCheckModel>>;
    async fn get_branch_cash_status(&self, branch_id: Uuid) -> BankingResult<Option<CashStatus>>;
    async fn get_terminal_cash_status(&self, terminal_id: Uuid) -> BankingResult<Option<CashStatus>>;
    async fn get_low_cash_alerts(&self, threshold_percentage: f64) -> BankingResult<Vec<CashAlert>>;
    
    /// Utility Operations
    async fn network_exists(&self, network_id: Uuid) -> BankingResult<bool>;
    async fn branch_exists(&self, branch_id: Uuid) -> BankingResult<bool>;
    async fn terminal_exists(&self, terminal_id: Uuid) -> BankingResult<bool>;
    async fn count_networks(&self) -> BankingResult<i64>;
    async fn count_branches(&self) -> BankingResult<i64>;
    async fn count_terminals(&self) -> BankingResult<i64>;
}

/// Supporting structures for limit validation
pub struct TerminalLimits {
    pub daily_limit: Decimal,
    pub current_volume: Decimal,
    pub status: String,
}

pub struct BranchLimits {
    pub daily_limit: Decimal,
    pub current_volume: Decimal,
    pub status: String,
}

pub struct NetworkLimits {
    pub daily_limit: Decimal,
    pub current_volume: Decimal,
    pub status: String,
}

pub struct LimitValidationResult {
    pub terminal_approved: bool,
    pub branch_approved: bool,
    pub network_approved: bool,
    pub overall_approved: bool,
    pub rejection_reason: Option<String>,
}

/// Performance reporting structures
pub struct NetworkPerformanceReport {
    pub network_id: Uuid,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_volume: Decimal,
    pub total_transactions: i64,
    pub active_branches: i64,
    pub active_terminals: i64,
}

pub struct BranchPerformanceReport {
    pub branch_id: Uuid,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_volume: Decimal,
    pub total_transactions: i64,
    pub active_terminals: i64,
}

pub struct TerminalPerformanceReport {
    pub terminal_id: Uuid,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_volume: Decimal,
    pub total_transactions: i64,
    pub uptime_percentage: f64,
}

/// Cash limit validation and monitoring structures
pub struct CashLimitValidationResult {
    pub is_valid: bool,
    pub validation_type: String, // Approved, InsufficientCash, ExceedsMaxLimit, BelowMinimum
    pub current_cash: Decimal,
    pub requested_amount: Decimal,
    pub max_limit: Decimal,
    pub minimum_required: Decimal,
    pub available_for_operation: Decimal,
}

pub struct CashStatus {
    pub entity_id: Uuid,
    pub entity_type: String, // Branch, Terminal
    pub current_cash_balance: Decimal,
    pub max_cash_limit: Decimal,
    pub minimum_cash_balance: Decimal,
    pub utilization_percentage: f64,
    pub status: String, // Normal, Low, Critical, Overstocked
    pub last_updated: DateTime<Utc>,
}

pub struct CashAlert {
    pub entity_id: Uuid,
    pub entity_type: String, // Branch, Terminal
    pub entity_name: String,
    pub alert_type: String, // LowCash, CriticalCash, Overstocked
    pub current_balance: Decimal,
    pub threshold_amount: Decimal,
    pub utilization_percentage: f64,
    pub created_at: DateTime<Utc>,
}
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
    domain::{Transaction, Channel, Fee, ReconciliationReport, ChannelType},
    error::BankingResult,
};

#[async_trait]
pub trait ChannelProcessor: Send + Sync {
    /// Validate channel-specific limits
    async fn validate_channel_limits(&self, transaction: &Transaction, channel: &Channel) -> BankingResult<bool>;
    
    /// Apply channel-specific fees
    async fn apply_channel_fees(&self, transaction: &Transaction, channel: &Channel) -> BankingResult<Vec<Fee>>;
    
    /// Log channel activity for audit
    async fn log_channel_activity(&self, transaction: &Transaction, channel: &Channel) -> BankingResult<()>;
    
    /// Handle channel reconciliation
    async fn handle_channel_reconciliation(&self, channel_id: String, date: NaiveDate) -> BankingResult<ReconciliationReport>;
    
    /// Channel-specific authorization workflows
    async fn requires_additional_auth(&self, transaction: &Transaction, channel: &Channel) -> BankingResult<bool>;

    /// Create a new channel
    async fn create_channel(&self, channel: Channel) -> BankingResult<Channel>;

    /// Find channel by ID
    async fn find_channel_by_id(&self, channel_id: Uuid) -> BankingResult<Option<Channel>>;

    /// Find channel by code
    async fn find_channel_by_code(&self, channel_code: &str) -> BankingResult<Option<Channel>>;

    /// Update channel status
    async fn update_channel_status(&self, channel_id: Uuid, status: crate::domain::ChannelStatus) -> BankingResult<()>;

    /// Get all active channels
    async fn get_active_channels(&self) -> BankingResult<Vec<Channel>>;

    /// Get channels by type
    async fn get_channels_by_type(&self, channel_type: ChannelType) -> BankingResult<Vec<Channel>>;

    /// Validate channel configuration
    async fn validate_channel_config(&self, channel: &Channel) -> BankingResult<ChannelValidationResult>;

    /// Process channel maintenance
    async fn process_channel_maintenance(&self, channel_id: Uuid) -> BankingResult<MaintenanceResult>;

    /// Get channel performance metrics
    async fn get_channel_metrics(&self, channel_id: Uuid, from_date: NaiveDate, to_date: NaiveDate) -> BankingResult<ChannelMetrics>;

    /// Handle channel timeout
    async fn handle_channel_timeout(&self, channel_id: Uuid, transaction_id: Uuid) -> BankingResult<()>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChannelValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MaintenanceResult {
    pub channel_id: Uuid,
    pub maintenance_type: String,
    pub performed_at: chrono::DateTime<chrono::Utc>,
    pub result: String,
    pub next_maintenance_due: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChannelMetrics {
    pub channel_id: Uuid,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_transactions: i64,
    pub successful_transactions: i64,
    pub failed_transactions: i64,
    pub total_volume: rust_decimal::Decimal,
    pub average_response_time_ms: f64,
    pub uptime_percentage: f64,
}
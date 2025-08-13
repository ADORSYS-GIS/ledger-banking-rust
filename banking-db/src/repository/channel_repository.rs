use async_trait::async_trait;
use banking_api::BankingResult;
use uuid::Uuid;

use crate::{models::channel::{ChannelModel, ChannelStatus}, ChannelType};

#[async_trait]
pub trait ChannelRepository: Send + Sync {
    /// Create a new channel record
    async fn create(&self, channel: ChannelModel) -> BankingResult<ChannelModel>;
    
    /// Update existing channel record
    async fn update(&self, channel: ChannelModel) -> BankingResult<ChannelModel>;
    
    /// Find channel by ID
    async fn find_by_id(&self, channel_id: Uuid) -> BankingResult<Option<ChannelModel>>;
    
    /// Find channel by code
    async fn find_by_code(&self, channel_code: &str) -> BankingResult<Option<ChannelModel>>;
    
    /// Find channels by type
    async fn find_by_type(&self, channel_type: ChannelType) -> BankingResult<Vec<ChannelModel>>;
    
    /// Find all active channels
    async fn find_active(&self) -> BankingResult<Vec<ChannelModel>>;
    
    /// Update channel status
    async fn update_status(&self, channel_id: Uuid, status: ChannelStatus) -> BankingResult<()>;
    
    /// Check if channel exists
    async fn exists(&self, channel_id: Uuid) -> BankingResult<bool>;
    
    /// Find channels with specific currency support
    async fn find_by_currency(&self, currency: &str) -> BankingResult<Vec<ChannelModel>>;
    
    /// Get channel statistics
    async fn get_channel_stats(&self, channel_id: Uuid) -> BankingResult<ChannelStats>;
    
    /// Soft delete channel (mark as inactive)
    async fn soft_delete(&self, channel_id: Uuid) -> BankingResult<()>;
    
    /// Get all channels with pagination
    async fn find_all_paginated(&self, limit: i64, offset: i64) -> BankingResult<Vec<ChannelModel>>;
    
    /// Count total channels
    async fn count_all(&self) -> BankingResult<i64>;
}

/// Channel statistics structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChannelStats {
    pub channel_id: Uuid,
    pub total_transactions: i64,
    pub daily_volume: rust_decimal::Decimal,
    pub monthly_volume: rust_decimal::Decimal,
    pub last_transaction_date: Option<chrono::DateTime<chrono::Utc>>,
    pub success_rate: f64,
}
use async_trait::async_trait;
use banking_api::{BankingResult, BankingError};
use banking_db::models::channel::{ChannelModel, ChannelStatus};
use banking_db::repository::{ChannelRepository, ChannelStats};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use heapless::String as HeaplessString;

pub struct ChannelRepositoryImpl {
    pool: PgPool,
}

impl ChannelRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

trait TryFromRow<R> {
    fn try_from_row(row: &R) -> BankingResult<Self>
    where
        Self: Sized;
}

impl TryFromRow<sqlx::postgres::PgRow> for ChannelModel {
    fn try_from_row(row: &sqlx::postgres::PgRow) -> BankingResult<Self> {
        Ok(ChannelModel {
            id: row.get("id"),
            channel_code: HeaplessString::try_from(
                row.get::<String, _>("channel_code").as_str()
            ).map_err(|_| BankingError::ValidationError {
                field: "channel_code".to_string(),
                message: "Channel code too long".to_string(),
            })?,
            channel_name: HeaplessString::try_from(
                row.get::<String, _>("channel_name").as_str()
            ).map_err(|_| BankingError::ValidationError {
                field: "channel_name".to_string(),
                message: "Channel name too long".to_string(),
            })?,
            channel_type: row.get("channel_type"),
            status: row.get::<String, _>("status").parse().map_err(|_| 
                BankingError::ValidationError {
                    field: "status".to_string(),
                    message: "Invalid channel status".to_string(),
                }
            )?,
            daily_limit: row.get("daily_limit"),
            per_transaction_limit: row.get("per_transaction_limit"),
            supported_currency01: row.get::<Option<String>, _>("supported_currency01")
                .map(|s| HeaplessString::try_from(s.as_str()))
                .transpose()
                .map_err(|_| BankingError::ValidationError {
                    field: "supported_currency01".to_string(),
                    message: "Currency code too long".to_string(),
                })?,
            supported_currency02: row.get::<Option<String>, _>("supported_currency02")
                .map(|s| HeaplessString::try_from(s.as_str()))
                .transpose()
                .map_err(|_| BankingError::ValidationError {
                    field: "supported_currency02".to_string(),
                    message: "Currency code too long".to_string(),
                })?,
            supported_currency03: row.get::<Option<String>, _>("supported_currency03")
                .map(|s| HeaplessString::try_from(s.as_str()))
                .transpose()
                .map_err(|_| BankingError::ValidationError {
                    field: "supported_currency03".to_string(),
                    message: "Currency code too long".to_string(),
                })?,
            requires_additional_auth: row.get("requires_additional_auth"),
            fee_schedule_id: row.get("fee_schedule_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

impl TryFromRow<sqlx::postgres::PgRow> for ChannelStats {
    fn try_from_row(row: &sqlx::postgres::PgRow) -> BankingResult<Self> {
        Ok(ChannelStats {
            channel_id: row.get("channel_id"),
            total_transactions: row.get("total_transactions"),
            daily_volume: row.get("daily_volume"),
            monthly_volume: row.get("monthly_volume"), 
            last_transaction_date: row.get("last_transaction_date"),
            success_rate: row.get("success_rate"),
        })
    }
}

#[async_trait]
impl ChannelRepository for ChannelRepositoryImpl {
    async fn create(&self, channel: ChannelModel) -> BankingResult<ChannelModel> {
        let row = sqlx::query(
            "INSERT INTO channels (
                id, channel_code, channel_name, channel_type, status, 
                daily_limit, per_transaction_limit, supported_currency01, 
                supported_currency02, supported_currency03,
                requires_additional_auth, fee_schedule_id, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5::channel_status, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING id, channel_code, channel_name, channel_type, status::text, daily_limit, per_transaction_limit, supported_currency01, supported_currency02, supported_currency03, requires_additional_auth, fee_schedule_id, created_at, updated_at"
        )
        .bind(channel.id)
        .bind(channel.channel_code.as_str())
        .bind(channel.channel_name.as_str())
        .bind(&channel.channel_type)
        .bind(channel.status.to_string())
        .bind(channel.daily_limit)
        .bind(channel.per_transaction_limit)
        .bind(channel.supported_currency01.as_ref().map(|c| c.as_str()))
        .bind(channel.supported_currency02.as_ref().map(|c| c.as_str()))
        .bind(channel.supported_currency03.as_ref().map(|c| c.as_str()))
        .bind(channel.requires_additional_auth)
        .bind(channel.fee_schedule_id)
        .bind(channel.created_at)
        .bind(channel.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(BankingError::from)?;
        
        ChannelModel::try_from_row(&row)
    }
    
    async fn update(&self, channel: ChannelModel) -> BankingResult<ChannelModel> {
        let row = sqlx::query(
            "UPDATE channels SET 
                channel_code = $2, channel_name = $3, channel_type = $4, 
                status = $5::channel_status, daily_limit = $6, per_transaction_limit = $7, 
                supported_currency01 = $8, supported_currency02 = $9, supported_currency03 = $10,
                requires_additional_auth = $11, fee_schedule_id = $12, updated_at = $13
            WHERE id = $1
            RETURNING id, channel_code, channel_name, channel_type, status::text, daily_limit, per_transaction_limit, supported_currency01, supported_currency02, supported_currency03, requires_additional_auth, fee_schedule_id, created_at, updated_at"
        )
        .bind(channel.id)
        .bind(channel.channel_code.as_str())
        .bind(channel.channel_name.as_str())
        .bind(&channel.channel_type)
        .bind(channel.status.to_string())
        .bind(channel.daily_limit)
        .bind(channel.per_transaction_limit)
        .bind(channel.supported_currency01.as_ref().map(|c| c.as_str()))
        .bind(channel.supported_currency02.as_ref().map(|c| c.as_str()))
        .bind(channel.supported_currency03.as_ref().map(|c| c.as_str()))
        .bind(channel.requires_additional_auth)
        .bind(channel.fee_schedule_id)
        .bind(channel.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(BankingError::from)?;
        
        ChannelModel::try_from_row(&row)
    }
    
    async fn find_by_id(&self, channel_id: Uuid) -> BankingResult<Option<ChannelModel>> {
        let row = sqlx::query("SELECT id, channel_code, channel_name, channel_type, status::text, daily_limit, per_transaction_limit, supported_currency01, supported_currency02, supported_currency03, requires_additional_auth, fee_schedule_id, created_at, updated_at FROM channels WHERE id = $1")
            .bind(channel_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(BankingError::from)?;
        
        match row {
            Some(row) => Ok(Some(ChannelModel::try_from_row(&row)?)),
            None => Ok(None),
        }
    }
    
    async fn find_by_code(&self, channel_code: &str) -> BankingResult<Option<ChannelModel>> {
        let row = sqlx::query("SELECT id, channel_code, channel_name, channel_type, status::text, daily_limit, per_transaction_limit, supported_currency01, supported_currency02, supported_currency03, requires_additional_auth, fee_schedule_id, created_at, updated_at FROM channels WHERE channel_code = $1")
            .bind(channel_code)
            .fetch_optional(&self.pool)
            .await
            .map_err(BankingError::from)?;
        
        match row {
            Some(row) => Ok(Some(ChannelModel::try_from_row(&row)?)),
            None => Ok(None),
        }
    }
    
    async fn find_by_type(&self, channel_type: &str) -> BankingResult<Vec<ChannelModel>> {
        let rows = sqlx::query("SELECT id, channel_code, channel_name, channel_type, status::text, daily_limit, per_transaction_limit, supported_currency01, supported_currency02, supported_currency03, requires_additional_auth, fee_schedule_id, created_at, updated_at FROM channels WHERE channel_type = $1 ORDER BY channel_name")
            .bind(channel_type)
            .fetch_all(&self.pool)
            .await
            .map_err(BankingError::from)?;
        
        let mut channels = Vec::new();
        for row in rows {
            channels.push(ChannelModel::try_from_row(&row)?);
        }
        Ok(channels)
    }
    
    async fn find_active(&self) -> BankingResult<Vec<ChannelModel>> {
        let rows = sqlx::query("SELECT id, channel_code, channel_name, channel_type, status::text, daily_limit, per_transaction_limit, supported_currency01, supported_currency02, supported_currency03, requires_additional_auth, fee_schedule_id, created_at, updated_at FROM channels WHERE status = 'Active' ORDER BY channel_name")
            .fetch_all(&self.pool)
            .await
            .map_err(BankingError::from)?;
        
        let mut channels = Vec::new();
        for row in rows {
            channels.push(ChannelModel::try_from_row(&row)?);
        }
        Ok(channels)
    }
    
    async fn update_status(&self, channel_id: Uuid, status: ChannelStatus) -> BankingResult<()> {
        let rows_affected = sqlx::query(
            "UPDATE channels SET status = $1::channel_status, updated_at = NOW() WHERE id = $2"
        )
        .bind(status.to_string())
        .bind(channel_id)
        .execute(&self.pool)
        .await
        .map_err(BankingError::from)?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(BankingError::Internal(format!("Channel not found: {channel_id}")));
        }
        
        Ok(())
    }
    
    async fn exists(&self, channel_id: Uuid) -> BankingResult<bool> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM channels WHERE id = $1")
            .bind(channel_id)
            .fetch_one(&self.pool)
            .await
            .map_err(BankingError::from)?;
        
        Ok(count > 0)
    }
    
    async fn find_by_currency(&self, currency: &str) -> BankingResult<Vec<ChannelModel>> {
        let rows = sqlx::query(
            "SELECT id, channel_code, channel_name, channel_type, status::text, daily_limit, per_transaction_limit, supported_currency01, supported_currency02, supported_currency03, requires_additional_auth, fee_schedule_id, created_at, updated_at FROM channels WHERE supported_currency01 = $1 OR supported_currency02 = $1 OR supported_currency03 = $1 ORDER BY channel_name"
        )
        .bind(currency)
        .fetch_all(&self.pool)
        .await
        .map_err(BankingError::from)?;
        
        let mut channels = Vec::new();
        for row in rows {
            channels.push(ChannelModel::try_from_row(&row)?);
        }
        Ok(channels)
    }
    
    async fn get_channel_stats(&self, channel_id: Uuid) -> BankingResult<ChannelStats> {
        // Simplified stats query without transactions table dependency
        // In a real implementation, this would join with the actual transactions table
        let row = sqlx::query(
            "SELECT 
                $1 as channel_id,
                0::bigint as total_transactions,
                0::decimal(15,2) as daily_volume,
                0::decimal(15,2) as monthly_volume,
                NULL::timestamp with time zone as last_transaction_date,
                0.0::float as success_rate"
        )
        .bind(channel_id)
        .fetch_one(&self.pool)
        .await
        .map_err(BankingError::from)?;
        
        ChannelStats::try_from_row(&row)
    }
    
    async fn soft_delete(&self, channel_id: Uuid) -> BankingResult<()> {
        let rows_affected = sqlx::query(
            "UPDATE channels SET status = 'Inactive', updated_at = NOW() WHERE id = $1"
        )
        .bind(channel_id)
        .execute(&self.pool)
        .await
        .map_err(BankingError::from)?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(BankingError::Internal(format!("Channel not found: {channel_id}")));
        }
        
        Ok(())
    }
    
    async fn find_all_paginated(&self, limit: i64, offset: i64) -> BankingResult<Vec<ChannelModel>> {
        let rows = sqlx::query(
            "SELECT id, channel_code, channel_name, channel_type, status::text, daily_limit, per_transaction_limit, supported_currency01, supported_currency02, supported_currency03, requires_additional_auth, fee_schedule_id, created_at, updated_at FROM channels ORDER BY channel_name LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(BankingError::from)?;
        
        let mut channels = Vec::new();
        for row in rows {
            channels.push(ChannelModel::try_from_row(&row)?);
        }
        Ok(channels)
    }
    
    async fn count_all(&self) -> BankingResult<i64> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM channels")
            .fetch_one(&self.pool)
            .await
            .map_err(BankingError::from)?;
        
        Ok(count)
    }
}
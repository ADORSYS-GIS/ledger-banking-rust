use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Database model for channels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelModel {
    pub channel_id: Uuid,
    pub channel_code: HeaplessString<50>,
    pub channel_name: HeaplessString<255>,
    pub channel_type: String,
    pub status: String,
    pub daily_limit: Option<Decimal>,
    pub per_transaction_limit: Option<Decimal>,
    pub supported_currencies: Vec<HeaplessString<3>>,
    pub requires_additional_auth: bool,
    pub fee_schedule_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for fee schedules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeScheduleModel {
    pub schedule_id: Uuid,
    pub schedule_name: HeaplessString<100>,
    pub channel_id: Option<Uuid>,
    pub effective_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub currency: HeaplessString<3>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for fee items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeItemModel {
    pub fee_item_id: Uuid,
    pub schedule_id: Uuid,
    pub fee_code: HeaplessString<20>,
    pub fee_name: HeaplessString<100>,
    pub fee_type: String,
    pub calculation_method: String,
    pub fee_amount: Option<Decimal>,
    pub fee_percentage: Option<Decimal>,
    pub minimum_fee: Option<Decimal>,
    pub maximum_fee: Option<Decimal>,
    pub applies_to_transaction_types: Vec<HeaplessString<20>>,
    pub is_waivable: bool,
    pub requires_approval_for_waiver: bool,
    pub created_at: DateTime<Utc>,
}

/// Database model for fee tiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeTierModel {
    pub tier_id: Uuid,
    pub fee_item_id: Uuid,
    pub tier_name: HeaplessString<50>,
    pub min_amount: Decimal,
    pub max_amount: Option<Decimal>,
    pub fee_amount: Option<Decimal>,
    pub fee_percentage: Option<Decimal>,
    pub tier_order: i32,
    pub created_at: DateTime<Utc>,
}

/// Database model for channel reconciliation reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelReconciliationReportModel {
    pub report_id: Uuid,
    pub channel_id: Uuid,
    pub reconciliation_date: NaiveDate,
    pub total_transactions: i64,
    pub total_amount: Decimal,
    pub status: String,
    pub generated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Database model for reconciliation discrepancies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciliationDiscrepancyModel {
    pub discrepancy_id: Uuid,
    pub report_id: Uuid,
    pub transaction_id: Uuid,
    pub description: HeaplessString<500>,
    pub expected_amount: Decimal,
    pub actual_amount: Decimal,
    pub difference: Decimal,
    pub resolved: bool,
    pub resolution_notes: Option<HeaplessString<1000>>,
    pub created_at: DateTime<Utc>,
}

/// Conversion from domain Channel to database ChannelModel
impl From<banking_api::domain::channel::Channel> for ChannelModel {
    fn from(channel: banking_api::domain::channel::Channel) -> Self {
        Self {
            channel_id: channel.channel_id,
            channel_code: channel.channel_code,
            channel_name: channel.channel_name,
            channel_type: format!("{:?}", channel.channel_type),
            status: format!("{:?}", channel.status),
            daily_limit: channel.daily_limit,
            per_transaction_limit: channel.per_transaction_limit,
            supported_currencies: channel.supported_currencies,
            requires_additional_auth: channel.requires_additional_auth,
            fee_schedule_id: channel.fee_schedule_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

/// Conversion from database ChannelModel to domain Channel
impl TryFrom<ChannelModel> for banking_api::domain::channel::Channel {
    type Error = String;

    fn try_from(model: ChannelModel) -> Result<Self, Self::Error> {
        use banking_api::domain::channel::{Channel, ChannelStatus};
        use banking_api::domain::transaction::ChannelType;

        let channel_type = match model.channel_type.as_str() {
            "BranchTeller" => ChannelType::BranchTeller,
            "ATM" => ChannelType::ATM,
            "InternetBanking" => ChannelType::InternetBanking,
            "MobileApp" => ChannelType::MobileApp,
            "AgentTerminal" => ChannelType::AgentTerminal,
            "USSD" => ChannelType::USSD,
            "ApiGateway" => ChannelType::ApiGateway,
            _ => return Err(format!("Unknown channel type: {}", model.channel_type)),
        };

        let status = match model.status.as_str() {
            "Active" => ChannelStatus::Active,
            "Inactive" => ChannelStatus::Inactive,
            "Maintenance" => ChannelStatus::Maintenance,
            "Suspended" => ChannelStatus::Suspended,
            _ => return Err(format!("Unknown channel status: {}", model.status)),
        };

        Ok(Channel {
            channel_id: model.channel_id,
            channel_code: model.channel_code,
            channel_name: model.channel_name,
            channel_type,
            status,
            daily_limit: model.daily_limit,
            per_transaction_limit: model.per_transaction_limit,
            supported_currencies: model.supported_currencies,
            requires_additional_auth: model.requires_additional_auth,
            fee_schedule_id: model.fee_schedule_id,
        })
    }
}

/// Conversion from domain FeeSchedule to database FeeScheduleModel
impl From<banking_api::domain::channel::FeeSchedule> for FeeScheduleModel {
    fn from(schedule: banking_api::domain::channel::FeeSchedule) -> Self {
        Self {
            schedule_id: schedule.schedule_id,
            schedule_name: schedule.schedule_name,
            channel_id: Some(schedule.channel_id),
            effective_date: schedule.effective_date,
            expiry_date: schedule.expiry_date,
            currency: schedule.currency,
            is_active: schedule.is_active,
            created_at: schedule.created_at,
            updated_at: schedule.updated_at,
        }
    }
}

/// Conversion from database FeeScheduleModel to domain FeeSchedule
impl TryFrom<FeeScheduleModel> for banking_api::domain::channel::FeeSchedule {
    type Error = String;

    fn try_from(model: FeeScheduleModel) -> Result<Self, Self::Error> {
        use banking_api::domain::channel::FeeSchedule;

        Ok(FeeSchedule {
            schedule_id: model.schedule_id,
            schedule_name: model.schedule_name,
            channel_id: model.channel_id.ok_or("Channel ID is required for fee schedule")?,
            effective_date: model.effective_date,
            expiry_date: model.expiry_date,
            currency: model.currency,
            fee_items: Vec::new(), // To be populated separately via repository
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        })
    }
}
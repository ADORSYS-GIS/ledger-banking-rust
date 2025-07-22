use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Channel {
    pub channel_id: Uuid,
    #[validate(length(min = 1, max = 50))]
    pub channel_code: String,
    #[validate(length(min = 1, max = 255))]
    pub channel_name: String,
    pub channel_type: super::transaction::ChannelType,
    pub status: ChannelStatus,
    pub daily_limit: Option<Decimal>,
    pub per_transaction_limit: Option<Decimal>,
    pub supported_currencies: Vec<String>,
    pub requires_additional_auth: bool,
    pub fee_schedule_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelStatus {
    Active,
    Inactive,
    Maintenance,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fee {
    pub fee_id: Uuid,
    pub fee_type: ChannelFeeType,
    pub amount: Decimal,
    pub currency: String,
    pub description: String,
    pub applies_to_transaction: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelFeeType {
    TransactionFee,
    MaintenanceFee,
    PenaltyFee,
    ServiceFee,
    ComplianceFee,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciliationReport {
    pub report_id: Uuid,
    pub channel_id: String,
    pub reconciliation_date: NaiveDate,
    pub total_transactions: i64,
    pub total_amount: Decimal,
    pub discrepancies: Vec<Discrepancy>,
    pub status: ReconciliationStatus,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Discrepancy {
    pub transaction_id: Uuid,
    #[validate(length(min = 1, max = 500))]
    pub description: String,
    pub expected_amount: Decimal,
    pub actual_amount: Decimal,
    pub difference: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReconciliationStatus {
    InProgress,
    Completed,
    Failed,
    RequiresManualReview,
}
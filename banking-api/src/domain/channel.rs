use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Channel {
    pub id: Uuid,
    pub channel_code: HeaplessString<50>,
    pub channel_name: HeaplessString<100>,
    pub channel_type: super::transaction::ChannelType,
    pub status: ChannelStatus,
    pub daily_limit: Option<Decimal>,
    pub per_transaction_limit: Option<Decimal>,
    pub supported_currencies: Vec<HeaplessString<3>>,
    pub requires_additional_auth: bool,
    pub fee_schedule_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelStatus {
    Active,
    Inactive,
    Maintenance,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelFee {
    pub id: Uuid,
    pub fee_type: ChannelFeeType,
    pub amount: Decimal,
    pub currency: HeaplessString<3>,
    pub description: HeaplessString<200>,
    pub applies_to_transaction_id: Uuid,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciliationReport {
    pub id: Uuid,
    pub channel_id: Uuid,
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
    pub description: HeaplessString<200>,
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

/// Fee Schedule structure for comprehensive channel fee management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeSchedule {
    pub id: Uuid,
    pub schedule_name: HeaplessString<100>,
    pub channel_id: Uuid,
    pub effective_date: chrono::NaiveDate,
    pub expiry_date: Option<chrono::NaiveDate>,
    pub currency: HeaplessString<3>,
    pub fee_items: Vec<FeeItem>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Individual fee item within a fee schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeItem {
    pub id: Uuid,
    pub fee_code: HeaplessString<20>,
    pub fee_name: HeaplessString<100>,
    pub fee_type: ChannelFeeType,
    pub calculation_method: ChannelFeeCalculationMethod,
    pub fee_amount: Option<Decimal>, // For fixed fees
    pub fee_percentage: Option<Decimal>, // For percentage-based fees
    pub minimum_fee: Option<Decimal>,
    pub maximum_fee: Option<Decimal>,
    pub fee_tiers: Vec<ChannelFeeTier>,
    pub applies_to_transaction_types: Vec<HeaplessString<20>>,
    pub is_waivable: bool,
    pub requires_approval_for_waiver: bool,
}

/// Channel fee calculation methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelFeeCalculationMethod {
    Fixed,           // Fixed amount regardless of transaction
    Percentage,      // Percentage of transaction amount
    Tiered,         // Different rates based on amount tiers
    BalanceBased,   // Based on account balance
    RuleBased,      // Complex rule-based calculation
    Hybrid,         // Combination of methods
}

/// Channel fee types for categorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelFeeType {
    TransactionFee,     // Per-transaction fees
    MaintenanceFee,     // Account maintenance fees
    ServiceFee,         // Service-related fees
    PenaltyFee,        // Penalty and fine fees
    ProcessingFee,     // Processing and handling fees
    ComplianceFee,     // Regulatory compliance fees
    InterchangeFee,    // Card interchange fees
    NetworkFee,        // Network usage fees
}

/// Channel fee tier for tiered pricing structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelFeeTier {
    pub id: Uuid,
    pub tier_name: HeaplessString<50>,
    pub min_amount: Decimal,
    pub max_amount: Option<Decimal>,
    pub fee_amount: Option<Decimal>,
    pub fee_percentage: Option<Decimal>,
    pub tier_order: i32,
}
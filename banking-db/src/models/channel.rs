use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

/// Channel status enum for database compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelStatus {
    Active,
    Inactive,
    Maintenance,
    Suspended,
}

/// Channel fee types for categorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelFeeType {
    TransactionFee,
    MaintenanceFee,
    ServiceFee,
    PenaltyFee,
    ProcessingFee,
    ComplianceFee,
    InterchangeFee,
    NetworkFee,
}

/// Channel fee calculation methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelFeeCalculationMethod {
    Fixed,
    Percentage,
    Tiered,
    BalanceBased,
    RuleBased,
    Hybrid,
}

/// Reconciliation status enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReconciliationStatus {
    InProgress,
    Completed,
    Failed,
    RequiresManualReview,
}

// Custom serialization functions for database compatibility
fn serialize_channel_status<S>(value: &ChannelStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let value_str = match value {
        ChannelStatus::Active => "Active",
        ChannelStatus::Inactive => "Inactive",
        ChannelStatus::Maintenance => "Maintenance",
        ChannelStatus::Suspended => "Suspended",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_channel_status<'de, D>(deserializer: D) -> Result<ChannelStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Active" => Ok(ChannelStatus::Active),
        "Inactive" => Ok(ChannelStatus::Inactive),
        "Maintenance" => Ok(ChannelStatus::Maintenance),
        "Suspended" => Ok(ChannelStatus::Suspended),
        _ => Err(serde::de::Error::custom(format!("Unknown channel status: {s}"))),
    }
}

fn serialize_channel_fee_type<S>(value: &ChannelFeeType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let value_str = match value {
        ChannelFeeType::TransactionFee => "TransactionFee",
        ChannelFeeType::MaintenanceFee => "MaintenanceFee",
        ChannelFeeType::ServiceFee => "ServiceFee",
        ChannelFeeType::PenaltyFee => "PenaltyFee",
        ChannelFeeType::ProcessingFee => "ProcessingFee",
        ChannelFeeType::ComplianceFee => "ComplianceFee",
        ChannelFeeType::InterchangeFee => "InterchangeFee",
        ChannelFeeType::NetworkFee => "NetworkFee",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_channel_fee_type<'de, D>(deserializer: D) -> Result<ChannelFeeType, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "TransactionFee" => Ok(ChannelFeeType::TransactionFee),
        "MaintenanceFee" => Ok(ChannelFeeType::MaintenanceFee),
        "ServiceFee" => Ok(ChannelFeeType::ServiceFee),
        "PenaltyFee" => Ok(ChannelFeeType::PenaltyFee),
        "ProcessingFee" => Ok(ChannelFeeType::ProcessingFee),
        "ComplianceFee" => Ok(ChannelFeeType::ComplianceFee),
        "InterchangeFee" => Ok(ChannelFeeType::InterchangeFee),
        "NetworkFee" => Ok(ChannelFeeType::NetworkFee),
        _ => Err(serde::de::Error::custom(format!("Unknown channel fee type: {s}"))),
    }
}

fn serialize_channel_fee_calculation_method<S>(value: &ChannelFeeCalculationMethod, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let value_str = match value {
        ChannelFeeCalculationMethod::Fixed => "Fixed",
        ChannelFeeCalculationMethod::Percentage => "Percentage",
        ChannelFeeCalculationMethod::Tiered => "Tiered",
        ChannelFeeCalculationMethod::BalanceBased => "BalanceBased",
        ChannelFeeCalculationMethod::RuleBased => "RuleBased",
        ChannelFeeCalculationMethod::Hybrid => "Hybrid",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_channel_fee_calculation_method<'de, D>(deserializer: D) -> Result<ChannelFeeCalculationMethod, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Fixed" => Ok(ChannelFeeCalculationMethod::Fixed),
        "Percentage" => Ok(ChannelFeeCalculationMethod::Percentage),
        "Tiered" => Ok(ChannelFeeCalculationMethod::Tiered),
        "BalanceBased" => Ok(ChannelFeeCalculationMethod::BalanceBased),
        "RuleBased" => Ok(ChannelFeeCalculationMethod::RuleBased),
        "Hybrid" => Ok(ChannelFeeCalculationMethod::Hybrid),
        _ => Err(serde::de::Error::custom(format!("Unknown channel fee calculation method: {s}"))),
    }
}

fn serialize_reconciliation_status<S>(value: &ReconciliationStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let value_str = match value {
        ReconciliationStatus::InProgress => "InProgress",
        ReconciliationStatus::Completed => "Completed",
        ReconciliationStatus::Failed => "Failed",
        ReconciliationStatus::RequiresManualReview => "RequiresManualReview",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_reconciliation_status<'de, D>(deserializer: D) -> Result<ReconciliationStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "InProgress" => Ok(ReconciliationStatus::InProgress),
        "Completed" => Ok(ReconciliationStatus::Completed),
        "Failed" => Ok(ReconciliationStatus::Failed),
        "RequiresManualReview" => Ok(ReconciliationStatus::RequiresManualReview),
        _ => Err(serde::de::Error::custom(format!("Unknown reconciliation status: {s}"))),
    }
}

// Display and FromStr implementations for Channel enums
impl std::fmt::Display for ChannelStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelStatus::Active => write!(f, "Active"),
            ChannelStatus::Inactive => write!(f, "Inactive"),
            ChannelStatus::Maintenance => write!(f, "Maintenance"),
            ChannelStatus::Suspended => write!(f, "Suspended"),
        }
    }
}

impl std::str::FromStr for ChannelStatus {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(ChannelStatus::Active),
            "Inactive" => Ok(ChannelStatus::Inactive),
            "Maintenance" => Ok(ChannelStatus::Maintenance),
            "Suspended" => Ok(ChannelStatus::Suspended),
            _ => Err(format!("Unknown channel status: {s}")),
        }
    }
}

impl std::fmt::Display for ChannelFeeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelFeeType::TransactionFee => write!(f, "TransactionFee"),
            ChannelFeeType::MaintenanceFee => write!(f, "MaintenanceFee"),
            ChannelFeeType::ServiceFee => write!(f, "ServiceFee"),
            ChannelFeeType::PenaltyFee => write!(f, "PenaltyFee"),
            ChannelFeeType::ProcessingFee => write!(f, "ProcessingFee"),
            ChannelFeeType::ComplianceFee => write!(f, "ComplianceFee"),
            ChannelFeeType::InterchangeFee => write!(f, "InterchangeFee"),
            ChannelFeeType::NetworkFee => write!(f, "NetworkFee"),
        }
    }
}

impl std::str::FromStr for ChannelFeeType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TransactionFee" => Ok(ChannelFeeType::TransactionFee),
            "MaintenanceFee" => Ok(ChannelFeeType::MaintenanceFee),
            "ServiceFee" => Ok(ChannelFeeType::ServiceFee),
            "PenaltyFee" => Ok(ChannelFeeType::PenaltyFee),
            "ProcessingFee" => Ok(ChannelFeeType::ProcessingFee),
            "ComplianceFee" => Ok(ChannelFeeType::ComplianceFee),
            "InterchangeFee" => Ok(ChannelFeeType::InterchangeFee),
            "NetworkFee" => Ok(ChannelFeeType::NetworkFee),
            _ => Err(format!("Unknown channel fee type: {s}")),
        }
    }
}

impl std::fmt::Display for ChannelFeeCalculationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelFeeCalculationMethod::Fixed => write!(f, "Fixed"),
            ChannelFeeCalculationMethod::Percentage => write!(f, "Percentage"),
            ChannelFeeCalculationMethod::Tiered => write!(f, "Tiered"),
            ChannelFeeCalculationMethod::BalanceBased => write!(f, "BalanceBased"),
            ChannelFeeCalculationMethod::RuleBased => write!(f, "RuleBased"),
            ChannelFeeCalculationMethod::Hybrid => write!(f, "Hybrid"),
        }
    }
}

impl std::str::FromStr for ChannelFeeCalculationMethod {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Fixed" => Ok(ChannelFeeCalculationMethod::Fixed),
            "Percentage" => Ok(ChannelFeeCalculationMethod::Percentage),
            "Tiered" => Ok(ChannelFeeCalculationMethod::Tiered),
            "BalanceBased" => Ok(ChannelFeeCalculationMethod::BalanceBased),
            "RuleBased" => Ok(ChannelFeeCalculationMethod::RuleBased),
            "Hybrid" => Ok(ChannelFeeCalculationMethod::Hybrid),
            _ => Err(format!("Unknown channel fee calculation method: {s}")),
        }
    }
}

impl std::fmt::Display for ReconciliationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReconciliationStatus::InProgress => write!(f, "InProgress"),
            ReconciliationStatus::Completed => write!(f, "Completed"),
            ReconciliationStatus::Failed => write!(f, "Failed"),
            ReconciliationStatus::RequiresManualReview => write!(f, "RequiresManualReview"),
        }
    }
}

impl std::str::FromStr for ReconciliationStatus {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "InProgress" => Ok(ReconciliationStatus::InProgress),
            "Completed" => Ok(ReconciliationStatus::Completed),
            "Failed" => Ok(ReconciliationStatus::Failed),
            "RequiresManualReview" => Ok(ReconciliationStatus::RequiresManualReview),
            _ => Err(format!("Unknown reconciliation status: {s}")),
        }
    }
}

/// Database model for channels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelModel {
    pub id: Uuid,
    pub channel_code: HeaplessString<50>,
    pub channel_name: HeaplessString<100>,
    pub channel_type: String,
    #[serde(serialize_with = "serialize_channel_status", deserialize_with = "deserialize_channel_status")]
    pub status: ChannelStatus,
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
    pub id: Uuid,
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
    pub id: Uuid,
    pub schedule_id: Uuid,
    pub fee_code: HeaplessString<20>,
    pub fee_name: HeaplessString<100>,
    #[serde(serialize_with = "serialize_channel_fee_type", deserialize_with = "deserialize_channel_fee_type")]
    pub fee_type: ChannelFeeType,
    #[serde(serialize_with = "serialize_channel_fee_calculation_method", deserialize_with = "deserialize_channel_fee_calculation_method")]
    pub calculation_method: ChannelFeeCalculationMethod,
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
    pub id: Uuid,
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
    pub id: Uuid,
    pub channel_id: Uuid,
    pub reconciliation_date: NaiveDate,
    pub total_transactions: i64,
    pub total_amount: Decimal,
    #[serde(serialize_with = "serialize_reconciliation_status", deserialize_with = "deserialize_reconciliation_status")]
    pub status: ReconciliationStatus,
    pub generated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Database model for reconciliation discrepancies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciliationDiscrepancyModel {
    pub id: Uuid,
    pub report_id: Uuid,
    pub transaction_id: Uuid,
    pub description: HeaplessString<200>,
    pub expected_amount: Decimal,
    pub actual_amount: Decimal,
    pub difference: Decimal,
    pub resolved: bool,
    pub resolution_notes: Option<HeaplessString<1000>>,
    pub created_at: DateTime<Utc>,
}

/// Database model for channel fees
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelFeeModel {
    pub id: Uuid,
    #[serde(serialize_with = "serialize_channel_fee_type", deserialize_with = "deserialize_channel_fee_type")]
    pub fee_type: ChannelFeeType,
    pub amount: Decimal,
    pub currency: HeaplessString<3>,
    pub description: HeaplessString<200>,
    pub applies_to_transaction_id: Uuid,
    pub created_at: DateTime<Utc>,
}



use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Database model enums
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "network_type", rename_all = "lowercase")]
pub enum NetworkType {
    Internal,
    Partner,
    ThirdParty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "network_status", rename_all = "lowercase")]
pub enum NetworkStatus {
    Active,
    Suspended,
    Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "branch_status", rename_all = "lowercase")]
pub enum BranchStatus {
    Active,
    Suspended,
    Closed,
    TemporarilyClosed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "terminal_type", rename_all = "lowercase")]
pub enum TerminalType {
    Pos,
    Mobile,
    Atm,
    WebPortal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "terminal_status", rename_all = "lowercase")]
pub enum TerminalStatus {
    Active,
    Maintenance,
    Suspended,
    Decommissioned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "branch_type", rename_all = "snake_case")]
pub enum BranchType {
    MainBranch,
    SubBranch,
    AgentOutlet,
    StandaloneKiosk,
    PartnerAgent,
    AtmLocation,
    MobileUnit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "branch_risk_rating", rename_all = "lowercase")]
pub enum BranchRiskRating {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "cash_limit_entity_type", rename_all = "lowercase")]
pub enum CashLimitEntityType {
    Branch,
    Terminal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "cash_operation_type", rename_all = "snake_case")]
pub enum CashOperationType {
    Withdrawal,
    Deposit,
    CashOut,
    CashIn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "cash_limit_validation_result", rename_all = "snake_case")]
pub enum CashLimitValidationResult {
    Approved,
    InsufficientCash,
    ExceedsMaxLimit,
    BelowMinimum,
}

/// Agent Network database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AgentNetworkModel {
    pub network_id: Uuid,
    pub network_name: HeaplessString<255>,
    pub network_type: NetworkType,
    pub status: NetworkStatus,
    pub contract_id: Option<Uuid>,
    pub aggregate_daily_limit: Decimal,
    pub current_daily_volume: Decimal,
    pub settlement_gl_code: HeaplessString<10>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: Uuid,
}

/// Agency Branch database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AgencyBranchModel {
    // === EXISTING FIELDS ===
    pub branch_id: Uuid,
    pub network_id: Uuid,
    pub parent_branch_id: Option<Uuid>,
    pub branch_name: HeaplessString<255>,
    pub branch_code: HeaplessString<8>,
    pub branch_level: i32,
    pub gl_code_prefix: HeaplessString<6>,
    pub status: BranchStatus,
    pub daily_transaction_limit: Decimal,
    pub current_daily_volume: Decimal,
    pub max_cash_limit: Decimal,
    pub current_cash_balance: Decimal,
    pub minimum_cash_balance: Decimal,
    pub created_at: DateTime<Utc>,
    
    // === NEW LOCATION FIELDS ===
    // Physical address (serialized as JSON)
    pub address_json: HeaplessString<500>,
    pub gps_latitude: Option<f64>,
    pub gps_longitude: Option<f64>,
    pub gps_accuracy_meters: Option<f32>,
    pub landmark_description: Option<HeaplessString<200>>,
    
    // Operational details (serialized as JSON)
    pub operating_hours_json: HeaplessString<1000>,
    pub holiday_schedule_json: HeaplessString<2000>,
    pub temporary_closure_json: Option<HeaplessString<500>>,
    
    // Contact information
    pub primary_phone: HeaplessString<20>,
    pub secondary_phone: Option<HeaplessString<20>>,
    pub email: Option<HeaplessString<100>>,
    pub branch_manager_id: Option<Uuid>,
    
    // Services and capabilities
    pub branch_type: BranchType,
    pub supported_services_json: HeaplessString<500>, // JSON array
    pub supported_currencies_json: HeaplessString<100>, // JSON array
    pub languages_spoken_json: HeaplessString<50>, // JSON array
    
    // Security and access (serialized as JSON)
    pub security_features_json: HeaplessString<500>,
    pub accessibility_features_json: HeaplessString<500>,
    pub required_documents_json: HeaplessString<1000>,
    
    // Customer capacity
    pub max_daily_customers: Option<u32>,
    pub average_wait_time_minutes: Option<u16>,
    
    // Transaction limits (enhanced from existing)
    pub per_transaction_limit: Decimal,
    pub monthly_transaction_limit: Option<Decimal>,
    
    // Compliance and risk
    pub risk_rating: BranchRiskRating,
    pub last_audit_date: Option<NaiveDate>,
    pub compliance_certifications_json: HeaplessString<1000>,
    
    // Metadata
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: Uuid,
}

/// Agent Terminal database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AgentTerminalModel {
    pub terminal_id: Uuid,
    pub branch_id: Uuid,
    pub agent_user_id: Uuid,
    pub terminal_type: TerminalType,
    pub terminal_name: HeaplessString<255>,
    pub daily_transaction_limit: Decimal,
    pub current_daily_volume: Decimal,
    pub max_cash_limit: Decimal,
    pub current_cash_balance: Decimal,
    pub minimum_cash_balance: Decimal,
    pub status: TerminalStatus,
    pub last_sync_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: Uuid,
}

/// Cash Limit Check database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CashLimitCheckModel {
    pub check_id: Uuid,
    pub entity_id: Uuid,
    pub entity_type: CashLimitEntityType,
    pub requested_amount: Decimal,
    pub operation_type: CashOperationType,
    pub validation_result: CashLimitValidationResult,
    pub available_amount: Option<Decimal>,
    pub max_limit: Option<Decimal>,
    pub minimum_required: Option<Decimal>,
    pub checked_at: DateTime<Utc>,
    pub checked_by: Uuid,
}




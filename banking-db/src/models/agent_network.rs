use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Agent Network database model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentNetworkModel {
    pub network_id: Uuid,
    pub network_name: String,
    pub network_type: String, // Internal, Partner, ThirdParty
    pub status: String,       // Active, Suspended, Terminated
    pub contract_id: Option<Uuid>,
    pub aggregate_daily_limit: Decimal,
    pub current_daily_volume: Decimal,
    pub settlement_gl_code: HeaplessString<10>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: Uuid,
}

/// Agency Branch database model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgencyBranchModel {
    // === EXISTING FIELDS ===
    pub branch_id: Uuid,
    pub network_id: Uuid,
    pub parent_branch_id: Option<Uuid>,
    pub branch_name: String,
    pub branch_code: HeaplessString<8>,
    pub branch_level: i32,
    pub gl_code_prefix: HeaplessString<6>,
    pub status: String, // Active, Suspended, Closed, TemporarilyClosed
    pub daily_transaction_limit: Decimal,
    pub current_daily_volume: Decimal,
    pub max_cash_limit: Decimal,
    pub current_cash_balance: Decimal,
    pub minimum_cash_balance: Decimal,
    pub created_at: DateTime<Utc>,
    
    // === NEW LOCATION FIELDS ===
    // Physical address (serialized as JSON)
    pub address_json: String,
    pub gps_latitude: Option<f64>,
    pub gps_longitude: Option<f64>,
    pub gps_accuracy_meters: Option<f32>,
    pub landmark_description: Option<String>,
    
    // Operational details (serialized as JSON)
    pub operating_hours_json: String,
    pub holiday_schedule_json: String,
    pub temporary_closure_json: Option<String>,
    
    // Contact information
    pub primary_phone: String,
    pub secondary_phone: Option<String>,
    pub email: Option<String>,
    pub branch_manager_id: Option<Uuid>,
    
    // Services and capabilities
    pub branch_type: String, // MainBranch, SubBranch, AgentOutlet, etc.
    pub supported_services_json: String, // JSON array
    pub supported_currencies_json: String, // JSON array
    pub languages_spoken_json: String, // JSON array
    
    // Security and access (serialized as JSON)
    pub security_features_json: String,
    pub accessibility_features_json: String,
    pub required_documents_json: String,
    
    // Customer capacity
    pub max_daily_customers: Option<u32>,
    pub average_wait_time_minutes: Option<u16>,
    
    // Transaction limits (enhanced from existing)
    pub per_transaction_limit: Decimal,
    pub monthly_transaction_limit: Option<Decimal>,
    
    // Compliance and risk
    pub risk_rating: String, // Low, Medium, High, Critical (BranchRiskRating)
    pub last_audit_date: Option<NaiveDate>,
    pub compliance_certifications_json: String,
    
    // Metadata
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: Uuid,
}

/// Agent Terminal database model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTerminalModel {
    pub terminal_id: Uuid,
    pub branch_id: Uuid,
    pub agent_user_id: Uuid,
    pub terminal_type: String, // Pos, Mobile, Atm, WebPortal
    pub terminal_name: String,
    pub daily_transaction_limit: Decimal,
    pub current_daily_volume: Decimal,
    pub max_cash_limit: Decimal,
    pub current_cash_balance: Decimal,
    pub minimum_cash_balance: Decimal,
    pub status: String, // Active, Maintenance, Suspended, Decommissioned
    pub last_sync_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: Uuid,
}

/// Cash Limit Check database model
#[derive(Debug, Clone)]
pub struct CashLimitCheckModel {
    pub check_id: Uuid,
    pub entity_id: Uuid,
    pub entity_type: String, // Branch, Terminal
    pub requested_amount: Decimal,
    pub operation_type: String, // Withdrawal, Deposit, CashOut, CashIn
    pub validation_result: String, // Approved, InsufficientCash, ExceedsMaxLimit, BelowMinimum
    pub available_amount: Option<Decimal>,
    pub max_limit: Option<Decimal>,
    pub minimum_required: Option<Decimal>,
    pub checked_at: DateTime<Utc>,
    pub checked_by: Uuid,
}




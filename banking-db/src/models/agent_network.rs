use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Import MessagingType from person models
use super::person::MessagingType;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "service_type", rename_all = "snake_case")]
pub enum ServiceType {
    CashWithdrawal,
    CashDeposit,
    CashTransfer,
    BillPayment,
    AccountOpening,
    CardServices,
    CheckDeposit,
    ForeignExchange,
    RemittanceCollection,
    AgentBanking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "certification_status", rename_all = "lowercase")]
pub enum CertificationStatus {
    Active,
    Expired,
    Suspended,
    Revoked,
}

/// Agent Network database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AgentNetworkModel {
    pub id: Uuid,
    pub network_name: HeaplessString<100>,
    pub network_type: NetworkType,
    pub status: NetworkStatus,
    pub contract_id: Option<Uuid>,
    pub aggregate_daily_limit: Decimal,
    pub current_daily_volume: Decimal,
    pub settlement_gl_code: HeaplessString<8>,
    pub created_at: DateTime<Utc>,
    // Note: Added missing fields from domain (though domain doesn't have these, keeping for DB audit trail)
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: Uuid,
}

/// Agency Branch database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AgencyBranchModel {
    // === EXISTING FIELDS ===
    pub id: Uuid,
    pub network_id: Uuid,
    pub parent_branch_id: Option<Uuid>,
    pub branch_name: HeaplessString<100>,
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
    
    // === LOCATION FIELDS ===
    // Physical address reference
    pub address: Uuid,  // Changed from address_id to match domain
    pub landmark_description: Option<HeaplessString<200>>,
    
    // Operational details
    pub operating_hours: Uuid,  // Changed from operating_hours_id to match domain
    pub holiday_plan: Uuid,     // Changed from holiday_schedule_json to match domain
    pub temporary_closure_id: Option<Uuid>,  // Changed from temporary_closure_json to match domain
    
    // Contact information - individual messaging fields (up to 5 entries)
    pub messaging1_id: Option<Uuid>,
    pub messaging1_type: Option<MessagingType>,
    pub messaging2_id: Option<Uuid>,
    pub messaging2_type: Option<MessagingType>,
    pub messaging3_id: Option<Uuid>,
    pub messaging3_type: Option<MessagingType>,
    pub messaging4_id: Option<Uuid>,
    pub messaging4_type: Option<MessagingType>,
    pub messaging5_id: Option<Uuid>,
    pub messaging5_type: Option<MessagingType>,
    pub branch_manager_id: Option<Uuid>,
    
    // Services and capabilities
    pub branch_type: BranchType,
    pub branch_capabilities: Uuid,  // Changed from branch_capabilities_id to match domain
    
    // Security and access
    pub security_access: Uuid,      // Changed from security_access_id to match domain
    
    // Customer capacity
    pub max_daily_customers: Option<u32>,
    pub average_wait_time_minutes: Option<u16>,
    
    // Transaction limits (enhanced from existing)
    pub per_transaction_limit: Decimal,
    pub monthly_transaction_limit: Option<Decimal>,
    
    // Compliance and risk
    pub risk_rating: BranchRiskRating,
    pub last_audit_date: Option<NaiveDate>,
    pub last_compliance_certification_id: Option<Uuid>,  // Changed from compliance_certifications_json to match domain
    
    // Metadata
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: Uuid,
}

/// Agent Terminal database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AgentTerminalModel {
    pub id: Uuid,
    pub branch_id: Uuid,
    pub agent_user_id: Uuid,
    pub terminal_type: TerminalType,
    pub terminal_name: HeaplessString<100>,
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
    pub id: Uuid,
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

/// Operating Hours database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OperatingHoursModel {
    pub id: Uuid,
    // Multi-language name support
    pub name_l1: HeaplessString<100>,
    pub name_l2: HeaplessString<100>,
    pub name_l3: HeaplessString<100>,
    pub monday_open: Option<NaiveTime>,
    pub monday_close: Option<NaiveTime>,
    pub monday_break_start: Option<NaiveTime>,
    pub monday_break_end: Option<NaiveTime>,
    pub tuesday_open: Option<NaiveTime>,
    pub tuesday_close: Option<NaiveTime>,
    pub tuesday_break_start: Option<NaiveTime>,
    pub tuesday_break_end: Option<NaiveTime>,
    pub wednesday_open: Option<NaiveTime>,
    pub wednesday_close: Option<NaiveTime>,
    pub wednesday_break_start: Option<NaiveTime>,
    pub wednesday_break_end: Option<NaiveTime>,
    pub thursday_open: Option<NaiveTime>,
    pub thursday_close: Option<NaiveTime>,
    pub thursday_break_start: Option<NaiveTime>,
    pub thursday_break_end: Option<NaiveTime>,
    pub friday_open: Option<NaiveTime>,
    pub friday_close: Option<NaiveTime>,
    pub friday_break_start: Option<NaiveTime>,
    pub friday_break_end: Option<NaiveTime>,
    pub saturday_open: Option<NaiveTime>,
    pub saturday_close: Option<NaiveTime>,
    pub saturday_break_start: Option<NaiveTime>,
    pub saturday_break_end: Option<NaiveTime>,
    pub sunday_open: Option<NaiveTime>,
    pub sunday_close: Option<NaiveTime>,
    pub sunday_break_start: Option<NaiveTime>,
    pub sunday_break_end: Option<NaiveTime>,
    pub timezone: HeaplessString<50>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

/// Branch Capabilities database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BranchCapabilitiesModel {
    pub id: Uuid,
    // Multi-language name support
    pub name_l1: HeaplessString<100>,
    pub name_l2: HeaplessString<100>,
    pub name_l3: HeaplessString<100>,
    // Supported services (up to 10 individual fields)
    pub supported_service1: Option<ServiceType>,
    pub supported_service2: Option<ServiceType>,
    pub supported_service3: Option<ServiceType>,
    pub supported_service4: Option<ServiceType>,
    pub supported_service5: Option<ServiceType>,
    pub supported_service6: Option<ServiceType>,
    pub supported_service7: Option<ServiceType>,
    pub supported_service8: Option<ServiceType>,
    pub supported_service9: Option<ServiceType>,
    pub supported_service10: Option<ServiceType>,
    // Supported currencies (up to 3 individual fields)
    pub supported_currency1: Option<HeaplessString<3>>,
    pub supported_currency2: Option<HeaplessString<3>>,
    pub supported_currency3: Option<HeaplessString<3>>,
    // Languages spoken (up to 3 individual fields)
    pub language_spoken1: Option<HeaplessString<3>>,
    pub language_spoken2: Option<HeaplessString<3>>,
    pub language_spoken3: Option<HeaplessString<3>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

/// Security Access database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SecurityAccessModel {
    pub id: Uuid,
    // Multi-language name support
    pub name_l1: HeaplessString<100>,
    pub name_l2: HeaplessString<100>,
    pub name_l3: HeaplessString<100>,
    // Security features (flattened)
    pub has_security_guard: bool,
    pub has_cctv: bool,
    pub has_panic_button: bool,
    pub has_safe: bool,
    pub has_biometric_verification: bool,
    pub police_station_distance_km: Option<f32>,
    // Accessibility features (flattened)
    pub wheelchair_accessible: bool,
    pub has_ramp: bool,
    pub has_braille_signage: bool,
    pub has_audio_assistance: bool,
    pub has_sign_language_support: bool,
    pub parking_available: bool,
    pub public_transport_nearby: bool,
    // Required documents (up to 20 individual references)
    pub required_document1: Option<Uuid>,
    pub required_document2: Option<Uuid>,
    pub required_document3: Option<Uuid>,
    pub required_document4: Option<Uuid>,
    pub required_document5: Option<Uuid>,
    pub required_document6: Option<Uuid>,
    pub required_document7: Option<Uuid>,
    pub required_document8: Option<Uuid>,
    pub required_document9: Option<Uuid>,
    pub required_document10: Option<Uuid>,
    pub required_document11: Option<Uuid>,
    pub required_document12: Option<Uuid>,
    pub required_document13: Option<Uuid>,
    pub required_document14: Option<Uuid>,
    pub required_document15: Option<Uuid>,
    pub required_document16: Option<Uuid>,
    pub required_document17: Option<Uuid>,
    pub required_document18: Option<Uuid>,
    pub required_document19: Option<Uuid>,
    pub required_document20: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

/// Holiday Plan database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HollidayPlanModel {
    pub id: Uuid,
    pub name_l1: HeaplessString<100>,
    pub name_l2: HeaplessString<100>,
    pub name_l3: HeaplessString<100>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

/// Holiday Schedule database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HolidayScheduleModel {
    pub id: Uuid,
    pub holiday_plan_id: Uuid,
    pub date: NaiveDate,
    // Multi-language name support
    pub name_l1: HeaplessString<100>,
    pub name_l2: HeaplessString<100>,
    pub name_l3: HeaplessString<100>,
    pub is_closed: bool,
    // Special hours (flattened DayHours)
    pub special_open_time: Option<NaiveTime>,
    pub special_close_time: Option<NaiveTime>,
    pub special_break_start: Option<NaiveTime>,
    pub special_break_end: Option<NaiveTime>,
}

/// Temporary Closure database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TemporaryClosureModel {
    pub id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    /// References ReasonAndPurpose.id for closure reason
    pub reason_id: Uuid,
    /// Additional context for closure (multi-language support)
    pub additional_details_l1: Option<HeaplessString<100>>,
    pub additional_details_l2: Option<HeaplessString<100>>,
    pub additional_details_l3: Option<HeaplessString<100>>,
    pub alternative_branch_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

/// Required Document database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RequiredDocumentModel {
    pub id: Uuid,
    // Multi-language document type support
    pub document_type_l1: HeaplessString<50>,
    pub document_type_l2: HeaplessString<50>,
    pub document_type_l3: HeaplessString<50>,
    pub is_mandatory: bool,
    // Alternative document references (up to 3)
    pub alternative1_id: Option<Uuid>,
    pub alternative2_id: Option<Uuid>,
    pub alternative3_id: Option<Uuid>,
}

/// Compliance Certification database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ComplianceCertModel {
    pub id: Uuid,
    // Multi-language certification name support
    pub certification_name_l1: HeaplessString<100>,
    pub certification_name_l2: HeaplessString<100>,
    pub certification_name_l3: HeaplessString<100>,
    pub issuer: Uuid,
    pub issue_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub status: CertificationStatus,
}

/// Terminal Limits database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TerminalLimitsModel {
    pub terminal_id: Uuid,
    pub daily_limit: Decimal,
    pub per_transaction_limit: Decimal,
    pub monthly_limit: Decimal,
}



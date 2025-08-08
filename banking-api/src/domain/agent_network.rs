use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use heapless::{String as HeaplessString};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Import MessagingType from person domain
use super::person::MessagingType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentNetwork {
    pub id: Uuid,
    pub network_name: HeaplessString<100>,
    pub network_type: NetworkType,
    pub status: NetworkStatus,
    pub contract_external_id: Option<HeaplessString<50>>,
    pub aggregate_daily_limit: Decimal,
    pub current_daily_volume: Decimal,
    pub settlement_gl_code: HeaplessString<8>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgencyBranch {

    pub id: Uuid,
    pub agent_network_id: Uuid,
    pub parent_agency_branch_id: Option<Uuid>,
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
    
    // Physical address
    pub address_id: Uuid,
    pub landmark_description: Option<HeaplessString<200>>,
    
    // Operational details
    pub operating_hours_id: Uuid,
    pub holiday_plan_id: Uuid,
    pub temporary_closure_id: Option<Uuid>,
    
    // Contact information - individual messaging fields (up to 5 entries)
    /// References to Messaging.messaging_id from person.rs
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
    pub branch_manager_person_id: Option<Uuid>,
    
    // Services and capabilities - references to separate entities
    pub branch_type: BranchType,  // Replaces LocationType
    pub branch_capabilities_id: Uuid,
    
    // Security and access - reference to separate entity
    pub security_access_id: Uuid,
    
    // Customer capacity
    pub max_daily_customers: Option<u32>,
    pub average_wait_time_minutes: Option<u16>,
    
    // Transaction limits (enhanced from existing)
    pub per_transaction_limit: Decimal,
    pub monthly_transaction_limit: Option<Decimal>,
    
    // Compliance and risk
    pub risk_rating: BranchRiskRating,
    pub last_audit_date: Option<NaiveDate>,
    pub last_compliance_certification_id: Option<Uuid>,
    
    // Metadata
    pub last_updated_at: DateTime<Utc>,
    pub updated_by_person_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTerminal {
    pub id: Uuid,
    pub agency_branch_id: Uuid,
    pub agent_person_id: Uuid,
    pub terminal_type: TerminalType,
    pub terminal_name: HeaplessString<100>,
    pub daily_transaction_limit: Decimal,
    pub current_daily_volume: Decimal,
    pub max_cash_limit: Decimal,
    pub current_cash_balance: Decimal,
    pub minimum_cash_balance: Decimal,
    pub status: TerminalStatus,
    pub last_sync_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NetworkType { 
    Internal, 
    Partner, 
    ThirdParty 
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NetworkStatus { 
    Active, 
    Suspended, 
    Terminated 
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BranchStatus { 
    Active, 
    Suspended, 
    Closed,
    TemporarilyClosed,  // New status
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TerminalType { 
    Pos, 
    Mobile, 
    Atm, 
    WebPortal 
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TerminalStatus { 
    Active, 
    Maintenance, 
    Suspended, 
    Decommissioned 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalLimits {
    pub daily_limit: Decimal,
    pub per_transaction_limit: Decimal,
    pub monthly_limit: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerminalLimitResult {
    Approved,
    Denied { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CashLimitValidation {
    Approved,
    InsufficientCash { available: Decimal, required: Decimal },
    ExceedsMaxLimit { current: Decimal, max_limit: Decimal },
    BelowMinimum { current: Decimal, minimum: Decimal },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashLimitCheck {
    pub entity_id: Uuid,
    pub entity_type: CashLimitEntityType,
    pub requested_amount: Decimal,
    pub operation_type: CashOperationType,
    pub validation_result: CashLimitValidation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CashLimitEntityType {
    Branch,
    Terminal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CashOperationType {
    Withdrawal,
    Deposit,
    CashOut,
    CashIn,
}

// New enum to replace LocationType
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BranchType {
    MainBranch,
    SubBranch,
    AgentOutlet,
    StandaloneKiosk,
    PartnerAgent,
    ATMLocation,
    MobileUnit,
}

// Supporting structs - AgentAddress and GpsCoordinates removed, using person::Address instead

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchCapabilities {
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
    pub created_by_person_id: Uuid,
    pub updated_by_person_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAccess {
    pub id: Uuid,
    // Multi-language name support
    pub name_l1: HeaplessString<100>,
    pub name_l2: HeaplessString<100>,
    pub name_l3: HeaplessString<100>,
    pub security_features: SecurityFeatures,
    pub accessibility_features: AccessibilityFeatures,
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
    pub created_by_person_id: Uuid,
    pub updated_by_person_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatingHours {
    pub id: Uuid,
    // Multi-language name support
    pub name_l1: HeaplessString<100>,
    pub name_l2: HeaplessString<100>,
    pub name_l3: HeaplessString<100>,
    pub monday: Option<DayHours>,
    pub tuesday: Option<DayHours>,
    pub wednesday: Option<DayHours>,
    pub thursday: Option<DayHours>,
    pub friday: Option<DayHours>,
    pub saturday: Option<DayHours>,
    pub sunday: Option<DayHours>,
    pub timezone: HeaplessString<50>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by_person_id: Uuid,
    pub updated_by_person_id: Uuid,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DayHours {
    pub open_time: NaiveTime,
    pub close_time: NaiveTime,
    pub break_start: Option<NaiveTime>,
    pub break_end: Option<NaiveTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HollidayPlan {
    pub id: Uuid,
    pub name_l1: HeaplessString<100>,
    pub name_l2: HeaplessString<100>,
    pub name_l3: HeaplessString<100>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by_person_id: Uuid,
    pub updated_by_person_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolidaySchedule {
    pub id: Uuid,
    pub holiday_plan_id: Uuid,
    pub date: NaiveDate,
    // Multi-language name support
    pub name_l1: HeaplessString<100>,
    pub name_l2: HeaplessString<100>,
    pub name_l3: HeaplessString<100>,
    pub is_closed: bool,
    pub special_hours: Option<DayHours>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporaryClosure {
    pub id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    /// References ReasonAndPurpose.id for closure reason
    pub closure_reason_id: Uuid,
    /// Additional context for closure (multi-language support)
    pub additional_details_l1: Option<HeaplessString<100>>,
    pub additional_details_l2: Option<HeaplessString<100>>,
    pub additional_details_l3: Option<HeaplessString<100>>,
    pub alternative_agency_branch_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by_person_id: Uuid,
    pub updated_by_person_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityFeatures {
    pub has_security_guard: bool,
    pub has_cctv: bool,
    pub has_panic_button: bool,
    pub has_safe: bool,
    pub has_biometric_verification: bool,
    pub police_station_distance_km: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityFeatures {
    pub wheelchair_accessible: bool,
    pub has_ramp: bool,
    pub has_braille_signage: bool,
    pub has_audio_assistance: bool,
    pub has_sign_language_support: bool,
    pub parking_available: bool,
    pub public_transport_nearby: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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
    AgentBanking,  // Additional service
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BranchRiskRating {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequiredDocument {
    pub id: Uuid,
    // Multi-language document type support
    pub document_type_l1: HeaplessString<50>,
    pub document_type_l2: HeaplessString<50>,
    pub document_type_l3: HeaplessString<50>,
    pub is_mandatory: bool,
    // Alternative document references (up to 3)
    pub alternative1_document_id: Option<Uuid>,
    pub alternative2_document_id: Option<Uuid>,
    pub alternative3_document_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCert {
    pub id: Uuid,
    // Multi-language certification name support
    pub certification_name_l1: HeaplessString<100>,
    pub certification_name_l2: HeaplessString<100>,
    pub certification_name_l3: HeaplessString<100>,
    pub issuer_person_id: Uuid,
    pub issue_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub status: CertificationStatus,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum CertificationStatus {
    Active,
    Expired,
    Suspended,
    Revoked,
}

impl AgentNetwork {
    /// Set settlement GL code from string with validation
    pub fn set_settlement_gl_code(&mut self, gl_code: &str) -> Result<(), &'static str> {
        self.settlement_gl_code = HeaplessString::try_from(gl_code).map_err(|_| "Settlement GL code too long")?;
        Ok(())
    }
}

impl AgencyBranch {
    // === EXISTING METHODS ===
    /// Set branch code from string with validation
    pub fn set_branch_code(&mut self, code: &str) -> Result<(), &'static str> {
        self.branch_code = HeaplessString::try_from(code).map_err(|_| "Branch code too long")?;
        Ok(())
    }
    
    /// Set GL code prefix from string with validation
    pub fn set_gl_code_prefix(&mut self, prefix: &str) -> Result<(), &'static str> {
        self.gl_code_prefix = HeaplessString::try_from(prefix).map_err(|_| "GL code prefix too long")?;
        Ok(())
    }
    
    // === NEW METHODS ===
    /// Check if branch can be used for cash pickup (requires capabilities lookup)
    pub fn is_cash_pickup_enabled_basic(&self) -> bool {
        self.status == BranchStatus::Active
            && self.current_cash_balance > self.minimum_cash_balance
    }
    
    /// Check if branch is currently open
    pub fn is_open_now(&self, _current_time: DateTime<Utc>) -> bool {
        // Convert to branch timezone and check operating hours
        // Implementation depends on timezone handling library
        true // Placeholder
    }
    
    /// Validate cash operation against limits
    pub fn validate_cash_operation(
        &self, 
        amount: Decimal, 
        operation: CashOperationType
    ) -> CashLimitValidation {
        match operation {
            CashOperationType::Withdrawal | CashOperationType::CashOut => {
                if amount > self.current_cash_balance {
                    CashLimitValidation::InsufficientCash {
                        available: self.current_cash_balance,
                        required: amount,
                    }
                } else if self.current_cash_balance - amount < self.minimum_cash_balance {
                    CashLimitValidation::BelowMinimum {
                        current: self.current_cash_balance - amount,
                        minimum: self.minimum_cash_balance,
                    }
                } else if amount > self.per_transaction_limit {
                    CashLimitValidation::ExceedsMaxLimit {
                        current: amount,
                        max_limit: self.per_transaction_limit,
                    }
                } else {
                    CashLimitValidation::Approved
                }
            }
            CashOperationType::Deposit | CashOperationType::CashIn => {
                if self.current_cash_balance + amount > self.max_cash_limit {
                    CashLimitValidation::ExceedsMaxLimit {
                        current: self.current_cash_balance + amount,
                        max_limit: self.max_cash_limit,
                    }
                } else {
                    CashLimitValidation::Approved
                }
            }
        }
    }
    
    /// Get address reference ID
    pub fn get_address_id(&self) -> Uuid {
        self.address_id
    }
    
    /// Get capabilities reference ID
    pub fn get_capabilities_id(&self) -> Uuid {
        self.branch_capabilities_id
    }
    
    /// Create a minimal AgencyBranch for backward compatibility with existing mappers
    /// This provides default values for all new fields
    #[allow(clippy::too_many_arguments)]
    pub fn create_minimal(
        id: Uuid,
        agent_network_id: Uuid,
        parent_agency_branch_id: Option<Uuid>,
        branch_name: HeaplessString<100>,
        branch_code: HeaplessString<8>,
        branch_level: i32,
        gl_code_prefix: HeaplessString<6>,
        status: BranchStatus,
        daily_transaction_limit: Decimal,
        current_daily_volume: Decimal,
        max_cash_limit: Decimal,
        current_cash_balance: Decimal,
        minimum_cash_balance: Decimal,
        created_at: DateTime<Utc>,
        default_address_id: Uuid,
        default_operating_hours_id: Uuid,
        default_capabilities_id: Uuid,
        default_security_access_id: Uuid,
    ) -> Self {
        AgencyBranch {
            id,
            agent_network_id,
            parent_agency_branch_id,
            branch_name,
            branch_code,
            branch_level,
            gl_code_prefix,
            status,
            daily_transaction_limit,
            current_daily_volume,
            max_cash_limit,
            current_cash_balance,
            minimum_cash_balance,
            created_at,
            
            // Default values for new fields
            address_id: default_address_id,
            landmark_description: None,
            operating_hours_id: default_operating_hours_id,
            holiday_plan_id: Uuid::nil(), // Default to nil UUID
            temporary_closure_id: None,
            messaging1_id: None,
            messaging1_type: None,
            messaging2_id: None,
            messaging2_type: None,
            messaging3_id: None,
            messaging3_type: None,
            messaging4_id: None,
            messaging4_type: None,
            messaging5_id: None,
            messaging5_type: None,
            branch_manager_person_id: None,
            branch_type: BranchType::SubBranch,
            branch_capabilities_id: default_capabilities_id,
            security_access_id: default_security_access_id,
            max_daily_customers: None,
            average_wait_time_minutes: None,
            per_transaction_limit: daily_transaction_limit,
            monthly_transaction_limit: None,
            risk_rating: BranchRiskRating::Low,
            last_audit_date: None,
            last_compliance_certification_id: None,
            last_updated_at: created_at,
            updated_by_person_id: Uuid::nil(), // Default to nil UUID for system-generated records
        }
    }
}

// Helper function for calculating distance between GPS coordinates
pub fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    // Haversine formula for calculating distance between two points on Earth
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lon = (lon2 - lon1).to_radians();
    
    let a = (delta_lat / 2.0).sin().powi(2) + 
            lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    
    // Earth's radius in kilometers
    const EARTH_RADIUS_KM: f64 = 6371.0;
    EARTH_RADIUS_KM * c
}
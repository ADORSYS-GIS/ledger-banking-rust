use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use heapless::{String as HeaplessString, Vec as HeaplessVec};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentNetwork {
    pub network_id: Uuid,
    pub network_name: HeaplessString<255>,
    pub network_type: NetworkType,
    pub status: NetworkStatus,
    pub contract_id: Option<Uuid>,
    pub aggregate_daily_limit: Decimal,
    pub current_daily_volume: Decimal,
    pub settlement_gl_code: HeaplessString<10>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgencyBranch {
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
    // Physical address
    pub address: AgentAddress,
    pub gps_coordinates: Option<GpsCoordinates>,
    pub landmark_description: Option<HeaplessString<200>>,
    
    // Operational details
    pub operating_hours: OperatingHours,
    pub holiday_schedule: HeaplessVec<HolidaySchedule, 20>,
    pub temporary_closure: Option<TemporaryClosure>,
    
    // Contact information
    pub primary_phone: HeaplessString<20>,
    pub secondary_phone: Option<HeaplessString<20>>,
    pub email: Option<HeaplessString<100>>,
    pub branch_manager_id: Option<Uuid>,
    
    // Services and capabilities
    pub branch_type: BranchType,  // Replaces LocationType
    pub supported_services: HeaplessVec<ServiceType, 20>,
    pub supported_currencies: HeaplessVec<[u8; 3], 10>,
    pub languages_spoken: HeaplessVec<[u8; 3], 5>,
    
    // Security and access
    pub security_features: SecurityFeatures,
    pub accessibility_features: AccessibilityFeatures,
    pub required_documents: HeaplessVec<RequiredDocument, 10>,
    
    // Customer capacity
    pub max_daily_customers: Option<u32>,
    pub average_wait_time_minutes: Option<u16>,
    
    // Transaction limits (enhanced from existing)
    pub per_transaction_limit: Decimal,
    pub monthly_transaction_limit: Option<Decimal>,
    
    // Compliance and risk
    pub risk_rating: BranchRiskRating,
    pub last_audit_date: Option<NaiveDate>,
    pub compliance_certifications: HeaplessVec<ComplianceCert, 5>,
    
    // Metadata
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTerminal {
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

// Supporting structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAddress {
    pub street_line1: HeaplessString<100>,
    pub street_line2: Option<HeaplessString<100>>,
    pub city: HeaplessString<50>,
    pub state_province: HeaplessString<50>,
    pub postal_code: HeaplessString<20>,
    pub country_code: [u8; 2],
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GpsCoordinates {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy_meters: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatingHours {
    pub monday: Option<DayHours>,
    pub tuesday: Option<DayHours>,
    pub wednesday: Option<DayHours>,
    pub thursday: Option<DayHours>,
    pub friday: Option<DayHours>,
    pub saturday: Option<DayHours>,
    pub sunday: Option<DayHours>,
    pub timezone: HeaplessString<50>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DayHours {
    pub open_time: NaiveTime,
    pub close_time: NaiveTime,
    pub break_start: Option<NaiveTime>,
    pub break_end: Option<NaiveTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolidaySchedule {
    pub date: NaiveDate,
    pub name: HeaplessString<100>,
    pub is_closed: bool,
    pub special_hours: Option<DayHours>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporaryClosure {
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    /// References ReasonAndPurpose.id for closure reason
    pub reason_id: Uuid,
    /// Additional context for closure
    pub additional_details: Option<HeaplessString<200>>,
    pub alternative_branch_id: Option<Uuid>,
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
    pub document_type: HeaplessString<50>,
    pub is_mandatory: bool,
    pub alternatives: HeaplessVec<HeaplessString<50>, 3>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCert {
    pub certification_name: HeaplessString<100>,
    pub issuer: Uuid,
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
    /// Check if branch can be used for cash pickup
    pub fn is_cash_pickup_enabled(&self) -> bool {
        self.status == BranchStatus::Active
            && self.supported_services.contains(&ServiceType::CashWithdrawal)
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
    
    /// Get formatted address
    pub fn get_formatted_address(&self) -> String {
        format!(
            "{}, {}, {} {}, {}",
            self.address.street_line1,
            self.address.city,
            self.address.state_province,
            self.address.postal_code,
            std::str::from_utf8(&self.address.country_code).unwrap_or("??")
        )
    }
    
    /// Check if branch supports a specific service
    pub fn supports_service(&self, service: ServiceType) -> bool {
        self.supported_services.contains(&service)
    }
    
    /// Create a minimal AgencyBranch for backward compatibility with existing mappers
    /// This provides default values for all new fields
    #[allow(clippy::too_many_arguments)]
    pub fn create_minimal(
        branch_id: Uuid,
        network_id: Uuid,
        parent_branch_id: Option<Uuid>,
        branch_name: HeaplessString<255>,
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
    ) -> Self {
        AgencyBranch {
            branch_id,
            network_id,
            parent_branch_id,
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
            address: AgentAddress {
                street_line1: HeaplessString::new(),
                street_line2: None,
                city: HeaplessString::new(),
                state_province: HeaplessString::new(),
                postal_code: HeaplessString::new(),
                country_code: [0, 0],
            },
            gps_coordinates: None,
            landmark_description: None,
            operating_hours: OperatingHours {
                monday: None,
                tuesday: None,
                wednesday: None,
                thursday: None,
                friday: None,
                saturday: None,
                sunday: None,
                timezone: HeaplessString::try_from("UTC").unwrap_or_default(),
            },
            holiday_schedule: HeaplessVec::new(),
            temporary_closure: None,
            primary_phone: HeaplessString::new(),
            secondary_phone: None,
            email: None,
            branch_manager_id: None,
            branch_type: BranchType::SubBranch,
            supported_services: HeaplessVec::new(),
            supported_currencies: HeaplessVec::new(),
            languages_spoken: HeaplessVec::new(),
            security_features: SecurityFeatures {
                has_security_guard: false,
                has_cctv: false,
                has_panic_button: false,
                has_safe: false,
                has_biometric_verification: false,
                police_station_distance_km: None,
            },
            accessibility_features: AccessibilityFeatures {
                wheelchair_accessible: false,
                has_ramp: false,
                has_braille_signage: false,
                has_audio_assistance: false,
                has_sign_language_support: false,
                parking_available: false,
                public_transport_nearby: false,
            },
            required_documents: HeaplessVec::new(),
            max_daily_customers: None,
            average_wait_time_minutes: None,
            per_transaction_limit: daily_transaction_limit,
            monthly_transaction_limit: None,
            risk_rating: BranchRiskRating::Low,
            last_audit_date: None,
            compliance_certifications: HeaplessVec::new(),
            last_updated_at: created_at,
            updated_by: Uuid::nil(), // Default to nil UUID for system-generated records
        }
    }
}

// Helper function for calculating distance between GPS coordinates
pub fn calculate_distance(coord1: GpsCoordinates, coord2: GpsCoordinates) -> f64 {
    // Haversine formula for calculating distance between two points on Earth
    let lat1_rad = coord1.latitude.to_radians();
    let lat2_rad = coord2.latitude.to_radians();
    let delta_lat = (coord2.latitude - coord1.latitude).to_radians();
    let delta_lon = (coord2.longitude - coord1.longitude).to_radians();
    
    let a = (delta_lat / 2.0).sin().powi(2) + 
            lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    
    // Earth's radius in kilometers
    const EARTH_RADIUS_KM: f64 = 6371.0;
    EARTH_RADIUS_KM * c
}
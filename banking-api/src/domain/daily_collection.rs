use chrono::{DateTime, Duration, NaiveDate, NaiveTime, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use super::collateral::AlertSeverity;


// ======== Collection Agent Models ========

/// Collection Agent representing an agent responsible for daily collection operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionAgent {
    pub id: Uuid,
    pub person_id: Uuid,
    pub license_number: HeaplessString<50>,
    pub license_expiry: NaiveDate,
    pub status: AgentStatus,
    pub assigned_territory_id: Uuid,
    pub agent_performance_metrics_id: Uuid,
    pub cash_limit: Decimal,
    pub device_information_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentStatus {
    Active,
    Suspended,
    Training,
    OnLeave,
    Terminated,
}

impl fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentStatus::Active => write!(f, "Active"),
            AgentStatus::Suspended => write!(f, "Suspended"),
            AgentStatus::Training => write!(f, "Training"),
            AgentStatus::OnLeave => write!(f, "OnLeave"),
            AgentStatus::Terminated => write!(f, "Terminated"),
        }
    }
}

impl std::str::FromStr for AgentStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(AgentStatus::Active),
            "Suspended" => Ok(AgentStatus::Suspended),
            "Training" => Ok(AgentStatus::Training),
            "OnLeave" => Ok(AgentStatus::OnLeave),
            "Terminated" => Ok(AgentStatus::Terminated),
            _ => Err(format!("Invalid AgentStatus: {s}")),
        }
    }
}


/// Territory assignment for collection agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Territory {
    pub id: Uuid,
    pub territory_name: HeaplessString<100>,
    pub coverage_area_id: Uuid,
    pub customer_count: i32,
    pub route_optimization_enabled: bool,
    pub territory_manager_person_id: Option<Uuid>,
}

/// Coverage area within a territory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageArea {
    pub id: Uuid,
    pub area_name: HeaplessString<100>,
    pub area_type: AreaType,
    pub boundary_coordinates_long_1: Option<Decimal>,
    pub boundary_coordinates_lat_1: Option<Decimal>,
    pub boundary_coordinates_long_2: Option<Decimal>,
    pub boundary_coordinates_lat_2: Option<Decimal>,
    pub boundary_coordinates_long_3: Option<Decimal>,
    pub boundary_coordinates_lat_3: Option<Decimal>,
    pub boundary_coordinates_long_4: Option<Decimal>,
    pub boundary_coordinates_lat_4: Option<Decimal>,
    pub boundary_coordinates_long_5: Option<Decimal>,
    pub boundary_coordinates_lat_5: Option<Decimal>,
    pub customer_density: CustomerDensity,
    pub transport_mode: TransportMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AreaType {
    Urban,
    Suburban,
    Rural,
    Commercial,
    Industrial,
    Mixed,
}

impl fmt::Display for AreaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AreaType::Urban => write!(f, "Urban"),
            AreaType::Suburban => write!(f, "Suburban"),
            AreaType::Rural => write!(f, "Rural"),
            AreaType::Commercial => write!(f, "Commercial"),
            AreaType::Industrial => write!(f, "Industrial"),
            AreaType::Mixed => write!(f, "Mixed"),
        }
    }
}

impl std::str::FromStr for AreaType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Urban" => Ok(AreaType::Urban),
            "Suburban" => Ok(AreaType::Suburban),
            "Rural" => Ok(AreaType::Rural),
            "Commercial" => Ok(AreaType::Commercial),
            "Industrial" => Ok(AreaType::Industrial),
            "Mixed" => Ok(AreaType::Mixed),
            _ => Err(format!("Invalid AreaType: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CustomerDensity {
    High,
    Medium,
    Low,
}

impl fmt::Display for CustomerDensity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomerDensity::High => write!(f, "High"),
            CustomerDensity::Medium => write!(f, "Medium"),
            CustomerDensity::Low => write!(f, "Low"),
        }
    }
}

impl std::str::FromStr for CustomerDensity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "High" => Ok(CustomerDensity::High),
            "Medium" => Ok(CustomerDensity::Medium),
            "Low" => Ok(CustomerDensity::Low),
            _ => Err(format!("Invalid CustomerDensity: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TransportMode {
    Walking,
    Bicycle,
    Motorcycle,
    Car,
    PublicTransport,
    Mixed,
}

impl fmt::Display for TransportMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransportMode::Walking => write!(f, "Walking"),
            TransportMode::Bicycle => write!(f, "Bicycle"),
            TransportMode::Motorcycle => write!(f, "Motorcycle"),
            TransportMode::Car => write!(f, "Car"),
            TransportMode::PublicTransport => write!(f, "PublicTransport"),
            TransportMode::Mixed => write!(f, "Mixed"),
        }
    }
}

impl std::str::FromStr for TransportMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Walking" => Ok(TransportMode::Walking),
            "Bicycle" => Ok(TransportMode::Bicycle),
            "Motorcycle" => Ok(TransportMode::Motorcycle),
            "Car" => Ok(TransportMode::Car),
            "PublicTransport" => Ok(TransportMode::PublicTransport),
            "Mixed" => Ok(TransportMode::Mixed),
            _ => Err(format!("Invalid TransportMode: {s}")),
        }
    }
}


/// Agent performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPerformanceMetrics {
    pub id: Uuid,
    pub collection_rate: Decimal,
    pub customer_satisfaction_score: Decimal,
    pub punctuality_score: Decimal,
    pub cash_handling_accuracy: Decimal,
    pub compliance_score: Decimal,
    pub total_collections: i64,
    pub total_amount_collected: Decimal,
    pub average_collection_time: Duration,
    pub customer_retention_rate: Decimal,
    pub route_efficiency: Decimal,
    pub monthly_targets_id: Uuid,
    pub performance_alert_1_id: Option<Uuid>,
    pub performance_alert_2_id: Option<Uuid>,
    pub performance_alert_3_id: Option<Uuid>,
    pub performance_alert_4_id: Option<Uuid>,
    pub performance_alert_5_id: Option<Uuid>,
}

/// Monthly targets for agent performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyTargets {
    pub id: Uuid,
    pub collection_target: Decimal,
    pub customer_target: i32,
    pub satisfaction_target: Decimal,
    pub punctuality_target: Decimal,
    pub accuracy_target: Decimal,
}

/// Performance alert for agent monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub id: Uuid,
    pub agent_performance_metrics_id: Uuid,
    pub alert_type: CollectionAlertType,
    pub severity: AlertSeverity,
    pub message: HeaplessString<200>,
    pub created_at: DateTime<Utc>,
    pub acknowledged: bool,
    pub resolution_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CollectionAlertType {
    LowCollectionRate,
    CustomerComplaint,
    CashDiscrepancy,
    MissedSchedule,
    ComplianceViolation,
    SafetyConcern,
    DeviceIssue,
}

impl fmt::Display for CollectionAlertType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectionAlertType::LowCollectionRate => write!(f, "LowCollectionRate"),
            CollectionAlertType::CustomerComplaint => write!(f, "CustomerComplaint"),
            CollectionAlertType::CashDiscrepancy => write!(f, "CashDiscrepancy"),
            CollectionAlertType::MissedSchedule => write!(f, "MissedSchedule"),
            CollectionAlertType::ComplianceViolation => write!(f, "ComplianceViolation"),
            CollectionAlertType::SafetyConcern => write!(f, "SafetyConcern"),
            CollectionAlertType::DeviceIssue => write!(f, "DeviceIssue"),
        }
    }
}

impl std::str::FromStr for CollectionAlertType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LowCollectionRate" => Ok(CollectionAlertType::LowCollectionRate),
            "CustomerComplaint" => Ok(CollectionAlertType::CustomerComplaint),
            "CashDiscrepancy" => Ok(CollectionAlertType::CashDiscrepancy),
            "MissedSchedule" => Ok(CollectionAlertType::MissedSchedule),
            "ComplianceViolation" => Ok(CollectionAlertType::ComplianceViolation),
            "SafetyConcern" => Ok(CollectionAlertType::SafetyConcern),
            "DeviceIssue" => Ok(CollectionAlertType::DeviceIssue),
            _ => Err(format!("Invalid CollectionAlertType: {s}")),
        }
    }
}


/// Device information for collection agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInformation {
    pub id: Uuid,
    pub external_id: HeaplessString<100>,
    pub device_type: DeviceType,
    pub model: HeaplessString<50>,
    pub os_version: HeaplessString<50>,
    pub app_version: HeaplessString<20>,
    pub last_sync: Option<DateTime<Utc>>,
    pub battery_level: Option<f32>,
    pub connectivity_status: ConnectivityStatus,
    pub security_features_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DeviceType {
    Smartphone,
    Tablet,
    PortableTerminal,
    SmartWatch,
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceType::Smartphone => write!(f, "Smartphone"),
            DeviceType::Tablet => write!(f, "Tablet"),
            DeviceType::PortableTerminal => write!(f, "PortableTerminal"),
            DeviceType::SmartWatch => write!(f, "SmartWatch"),
        }
    }
}

impl std::str::FromStr for DeviceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Smartphone" => Ok(DeviceType::Smartphone),
            "Tablet" => Ok(DeviceType::Tablet),
            "PortableTerminal" => Ok(DeviceType::PortableTerminal),
            "SmartWatch" => Ok(DeviceType::SmartWatch),
            _ => Err(format!("Invalid DeviceType: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConnectivityStatus {
    Online,
    Offline,
    LimitedConnectivity,
    SyncPending,
}

impl fmt::Display for ConnectivityStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectivityStatus::Online => write!(f, "Online"),
            ConnectivityStatus::Offline => write!(f, "Offline"),
            ConnectivityStatus::LimitedConnectivity => write!(f, "LimitedConnectivity"),
            ConnectivityStatus::SyncPending => write!(f, "SyncPending"),
        }
    }
}

impl std::str::FromStr for ConnectivityStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Online" => Ok(ConnectivityStatus::Online),
            "Offline" => Ok(ConnectivityStatus::Offline),
            "LimitedConnectivity" => Ok(ConnectivityStatus::LimitedConnectivity),
            "SyncPending" => Ok(ConnectivityStatus::SyncPending),
            _ => Err(format!("Invalid ConnectivityStatus: {s}")),
        }
    }
}

/// Security features for agent devices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionSecurityFeatures {
    pub id: Uuid,
    pub biometric_enabled: bool,
    pub pin_protection: bool,
    pub encryption_enabled: bool,
    pub remote_wipe_enabled: bool,
    pub certificate_installed: bool,
    pub last_security_scan: Option<DateTime<Utc>>,
}

// ======== Collection Program Models ========

/// Daily Collection Program entity representing a structured savings program
/// managed through agent-mediated collection operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionProgram {
    pub id: Uuid,
    pub name: HeaplessString<100>,
    pub description: HeaplessString<500>,
    pub program_type: CollectionProgramType,
    pub status: ProgramStatus,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub collection_frequency: CollectionFrequency,
    pub operating_hours_id: Option<Uuid>,
    pub minimum_amount: Decimal,
    pub maximum_amount: Decimal,
    pub target_amount: Option<Decimal>,
    pub program_duration_days: i32,
    pub graduation_criteria_id: Uuid,
    pub fee_structure_id: Uuid,
    pub interest_rate: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by_person_id: Uuid,
    pub reason_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CollectionProgramType {
    FixedAmount,
    VariableAmount,
    TargetBased,
    DurationBased,
}

impl fmt::Display for CollectionProgramType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectionProgramType::FixedAmount => write!(f, "FixedAmount"),
            CollectionProgramType::VariableAmount => write!(f, "VariableAmount"),
            CollectionProgramType::TargetBased => write!(f, "TargetBased"),
            CollectionProgramType::DurationBased => write!(f, "DurationBased"),
        }
    }
}

impl std::str::FromStr for CollectionProgramType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FixedAmount" => Ok(CollectionProgramType::FixedAmount),
            "VariableAmount" => Ok(CollectionProgramType::VariableAmount),
            "TargetBased" => Ok(CollectionProgramType::TargetBased),
            "DurationBased" => Ok(CollectionProgramType::DurationBased),
            _ => Err(format!("Invalid CollectionProgramType: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProgramStatus {
    Active,
    Suspended,
    Closed,
    UnderReview,
}

impl fmt::Display for ProgramStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramStatus::Active => write!(f, "Active"),
            ProgramStatus::Suspended => write!(f, "Suspended"),
            ProgramStatus::Closed => write!(f, "Closed"),
            ProgramStatus::UnderReview => write!(f, "UnderReview"),
        }
    }
}

impl std::str::FromStr for ProgramStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(ProgramStatus::Active),
            "Suspended" => Ok(ProgramStatus::Suspended),
            "Closed" => Ok(ProgramStatus::Closed),
            "UnderReview" => Ok(ProgramStatus::UnderReview),
            _ => Err(format!("Invalid ProgramStatus: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CollectionFrequency {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
}

impl fmt::Display for CollectionFrequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectionFrequency::Daily => write!(f, "Daily"),
            CollectionFrequency::Weekly => write!(f, "Weekly"),
            CollectionFrequency::Monthly => write!(f, "Monthly"),
            CollectionFrequency::Quarterly => write!(f, "Quarterly"),
            CollectionFrequency::Yearly => write!(f, "Yearly"),
        }
    }
}

impl std::str::FromStr for CollectionFrequency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Daily" => Ok(CollectionFrequency::Daily),
            "Weekly" => Ok(CollectionFrequency::Weekly),
            "Monthly" => Ok(CollectionFrequency::Monthly),
            "Quarterly" => Ok(CollectionFrequency::Quarterly),
            "Yearly" => Ok(CollectionFrequency::Yearly),
            _ => Err(format!("Invalid CollectionFrequency: {s}")),
        }
    }
}


/// Graduation criteria for transitioning customers to regular banking products
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraduationCriteria {
    pub id: Uuid,
    pub minimum_balance: Option<Decimal>,
    pub minimum_collection_rate: Option<Decimal>,
    pub minimum_duration_days: Option<i32>,
    pub consecutive_collections_required: Option<i32>,
    pub target_achievement_required: bool,
    pub auto_graduation_enabled: bool,
}

/// Fee structure for collection programs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeStructure {
    pub id: Uuid,
    pub setup_fee: Option<Decimal>,
    pub collection_fee: Option<Decimal>,
    pub maintenance_fee: Option<Decimal>,
    pub graduation_fee: Option<Decimal>,
    pub early_termination_fee: Option<Decimal>,
    pub fee_frequency: CollectionFeeFrequency,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CollectionFeeFrequency {
    PerCollection,
    Daily,
    Weekly,
    Monthly,
    OneTime,
}

impl fmt::Display for CollectionFeeFrequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectionFeeFrequency::PerCollection => write!(f, "PerCollection"),
            CollectionFeeFrequency::Daily => write!(f, "Daily"),
            CollectionFeeFrequency::Weekly => write!(f, "Weekly"),
            CollectionFeeFrequency::Monthly => write!(f, "Monthly"),
            CollectionFeeFrequency::OneTime => write!(f, "OneTime"),
        }
    }
}

impl std::str::FromStr for CollectionFeeFrequency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PerCollection" => Ok(CollectionFeeFrequency::PerCollection),
            "Daily" => Ok(CollectionFeeFrequency::Daily),
            "Weekly" => Ok(CollectionFeeFrequency::Weekly),
            "Monthly" => Ok(CollectionFeeFrequency::Monthly),
            "OneTime" => Ok(CollectionFeeFrequency::OneTime),
            _ => Err(format!("Invalid FeeFrequency: {s}")),
        }
    }
}

// ======== Customer Collection Profile Models ========

/// Customer Collection Profile representing an individual customer's participation
/// in a daily collection program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerCollectionProfile {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub collection_program_id: Uuid,
    pub account_id: Uuid,
    pub enrollment_date: NaiveDate,
    pub status: CollectionStatus,
    pub daily_amount: Decimal,
    pub collection_schedule: CollectionSchedule,
    pub assigned_collection_agent_id: Uuid,
    pub collection_location_address_id: Uuid,
    pub collection_performance_metrics: CollectionPerformanceMetrics,
    pub graduation_progress: GraduationProgress,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub reason_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CollectionStatus {
    Active,
    Suspended,
    Defaulted,
    Graduated,
    Terminated,
}

impl fmt::Display for CollectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectionStatus::Active => write!(f, "Active"),
            CollectionStatus::Suspended => write!(f, "Suspended"),
            CollectionStatus::Defaulted => write!(f, "Defaulted"),
            CollectionStatus::Graduated => write!(f, "Graduated"),
            CollectionStatus::Terminated => write!(f, "Terminated"),
        }
    }
}

impl std::str::FromStr for CollectionStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(CollectionStatus::Active),
            "Suspended" => Ok(CollectionStatus::Suspended),
            "Defaulted" => Ok(CollectionStatus::Defaulted),
            "Graduated" => Ok(CollectionStatus::Graduated),
            "Terminated" => Ok(CollectionStatus::Terminated),
            _ => Err(format!("Invalid CollectionStatus: {s}")),
        }
    }
}

/// Collection schedule configuration for a customer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionSchedule {
    pub id: Uuid,
    pub frequency: CollectionFrequency,
    pub collection_time: NaiveTime,
    pub timezone: HeaplessString<50>,
    pub holiday_handling: HolidayHandling,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum HolidayHandling {
    Skip,
    NextBusinessDay,
    PreviousBusinessDay,
    CollectDouble,
}

impl fmt::Display for HolidayHandling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HolidayHandling::Skip => write!(f, "Skip"),
            HolidayHandling::NextBusinessDay => write!(f, "NextBusinessDay"),
            HolidayHandling::PreviousBusinessDay => write!(f, "PreviousBusinessDay"),
            HolidayHandling::CollectDouble => write!(f, "CollectDouble"),
        }
    }
}

impl std::str::FromStr for HolidayHandling {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Skip" => Ok(HolidayHandling::Skip),
            "NextBusinessDay" => Ok(HolidayHandling::NextBusinessDay),
            "PreviousBusinessDay" => Ok(HolidayHandling::PreviousBusinessDay),
            "CollectDouble" => Ok(HolidayHandling::CollectDouble),
            _ => Err(format!("Invalid HolidayHandling: {s}")),
        }
    }
}





/// Customer performance metrics in the collection program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionPerformanceMetrics {
    pub id: Uuid,
    pub collection_rate: Decimal,
    pub total_collections: i64,
    pub total_amount_collected: Decimal,
    pub average_collection_amount: Decimal,
    pub consecutive_collections: i32,
    pub missed_collections: i32,
    pub last_collection_date: Option<NaiveDate>,
    pub performance_score: Decimal,
    pub reliability_rating: ReliabilityRating,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ReliabilityRating {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}

impl fmt::Display for ReliabilityRating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReliabilityRating::Excellent => write!(f, "Excellent"),
            ReliabilityRating::Good => write!(f, "Good"),
            ReliabilityRating::Fair => write!(f, "Fair"),
            ReliabilityRating::Poor => write!(f, "Poor"),
            ReliabilityRating::Critical => write!(f, "Critical"),
        }
    }
}

impl std::str::FromStr for ReliabilityRating {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Excellent" => Ok(ReliabilityRating::Excellent),
            "Good" => Ok(ReliabilityRating::Good),
            "Fair" => Ok(ReliabilityRating::Fair),
            "Poor" => Ok(ReliabilityRating::Poor),
            "Critical" => Ok(ReliabilityRating::Critical),
            _ => Err(format!("Invalid ReliabilityRating: {s}")),
        }
    }
}

/// Progress tracking for graduation from collection program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraduationProgress {
    pub id: Uuid,
    pub customer_collection_profile_id: Uuid,
    pub current_balance: Decimal,
    pub target_balance: Option<Decimal>,
    pub days_in_program: i32,
    pub minimum_days_required: Option<i32>,
    pub collection_consistency_rate: Decimal,
    pub minimum_consistency_required: Option<Decimal>,
    pub graduation_eligible: bool,
    pub graduation_date: Option<NaiveDate>,
    pub next_review_date: NaiveDate,
}

// ======== Collection Record Models ========

/// Collection Record representing a single collection transaction
/// in the daily collection program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionRecord {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub collection_agent_id: Uuid,
    pub collection_program_id: Uuid,
    pub account_id: Uuid,
    pub collection_date: NaiveDate,
    pub collection_time: DateTime<Utc>,
    pub amount: Decimal,
    pub currency: HeaplessString<3>,
    pub collection_method: CollectionMethod,
    pub location_address_id: Option<Uuid>,
    pub receipt_number: HeaplessString<50>,
    pub status: CollectionRecordStatus,
    pub notes: Option<HeaplessString<500>>,
    pub collection_verification_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
    pub reason_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CollectionMethod {
    Cash,
    MobilePayment,
    BankTransfer,
    DigitalWallet,
}

impl fmt::Display for CollectionMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectionMethod::Cash => write!(f, "Cash"),
            CollectionMethod::MobilePayment => write!(f, "MobilePayment"),
            CollectionMethod::BankTransfer => write!(f, "BankTransfer"),
            CollectionMethod::DigitalWallet => write!(f, "DigitalWallet"),
        }
    }
}

impl std::str::FromStr for CollectionMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Cash" => Ok(CollectionMethod::Cash),
            "MobilePayment" => Ok(CollectionMethod::MobilePayment),
            "BankTransfer" => Ok(CollectionMethod::BankTransfer),
            "DigitalWallet" => Ok(CollectionMethod::DigitalWallet),
            _ => Err(format!("Invalid CollectionMethod: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CollectionRecordStatus {
    Pending,
    Processed,
    Failed,
    Reversed,
    UnderReview,
}

impl fmt::Display for CollectionRecordStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectionRecordStatus::Pending => write!(f, "Pending"),
            CollectionRecordStatus::Processed => write!(f, "Processed"),
            CollectionRecordStatus::Failed => write!(f, "Failed"),
            CollectionRecordStatus::Reversed => write!(f, "Reversed"),
            CollectionRecordStatus::UnderReview => write!(f, "UnderReview"),
        }
    }
}

impl std::str::FromStr for CollectionRecordStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(CollectionRecordStatus::Pending),
            "Processed" => Ok(CollectionRecordStatus::Processed),
            "Failed" => Ok(CollectionRecordStatus::Failed),
            "Reversed" => Ok(CollectionRecordStatus::Reversed),
            "UnderReview" => Ok(CollectionRecordStatus::UnderReview),
            _ => Err(format!("Invalid CollectionRecordStatus: {s}")),
        }
    }
}

/// Verification data for collection transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionVerification {
    pub id: Uuid,
    pub collection_record_id: Uuid,
    pub customer_signature: Option<HeaplessString<200>>,
    pub agent_verification_code: Option<HeaplessString<50>>,
    pub biometric_data_id: Option<Uuid>,
    pub photo_evidence_id: Option<Uuid>,
    pub witness_person_id: Option<Uuid>,
    pub verification_timestamp: DateTime<Utc>,
}

/// Biometric verification data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricData {
    pub id: Uuid,
    pub collection_verification_id: Uuid,
    pub fingerprint_hash: Option<HeaplessString<100>>,
    pub face_recognition_score: Option<f64>,
    pub verification_method: BiometricMethod,
    pub confidence_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BiometricMethod {
    Fingerprint,
    FaceRecognition,
    VoicePrint,
    Combined,
}

impl fmt::Display for BiometricMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BiometricMethod::Fingerprint => write!(f, "Fingerprint"),
            BiometricMethod::FaceRecognition => write!(f, "FaceRecognition"),
            BiometricMethod::VoicePrint => write!(f, "VoicePrint"),
            BiometricMethod::Combined => write!(f, "Combined"),
        }
    }
}

impl std::str::FromStr for BiometricMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Fingerprint" => Ok(BiometricMethod::Fingerprint),
            "FaceRecognition" => Ok(BiometricMethod::FaceRecognition),
            "VoicePrint" => Ok(BiometricMethod::VoicePrint),
            "Combined" => Ok(BiometricMethod::Combined),
            _ => Err(format!("Invalid BiometricMethod: {s}")),
        }
    }
}

/// Photo evidence for collection verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoEvidence {
    pub id: Uuid,
    pub collection_verification_id: Uuid,
    pub customer_photo_hash: Option<HeaplessString<100>>,
    pub receipt_photo_hash: Option<HeaplessString<100>>,
    pub location_photo_hash: Option<HeaplessString<100>>,
    pub photo_timestamp: DateTime<Utc>,
}

/// Witness information for collection verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessInformation {
    pub id: Uuid,
    pub collection_verification_id: Uuid,
    pub witness_name: HeaplessString<100>,
    pub witness_contact: HeaplessString<50>,
    pub witness_relationship: HeaplessString<50>,
    pub witness_signature: Option<HeaplessString<200>>,
}

/// Collection batch information for bulk processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionBatch {
    pub id: Uuid,
    pub collection_agent_id: Uuid,
    pub collection_date: NaiveDate,
    pub total_collections: i32,
    pub total_amount: Decimal,
    pub currency: HeaplessString<3>,
    pub status: BatchStatus,
    pub collection_records: Vec<Uuid>,
    pub reconciliation_data_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BatchStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    PartiallyProcessed,
    RequiresReconciliation,
}

impl fmt::Display for BatchStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BatchStatus::Pending => write!(f, "Pending"),
            BatchStatus::Processing => write!(f, "Processing"),
            BatchStatus::Completed => write!(f, "Completed"),
            BatchStatus::Failed => write!(f, "Failed"),
            BatchStatus::PartiallyProcessed => write!(f, "PartiallyProcessed"),
            BatchStatus::RequiresReconciliation => write!(f, "RequiresReconciliation"),
        }
    }
}

impl std::str::FromStr for BatchStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(BatchStatus::Pending),
            "Processing" => Ok(BatchStatus::Processing),
            "Completed" => Ok(BatchStatus::Completed),
            "Failed" => Ok(BatchStatus::Failed),
            "PartiallyProcessed" => Ok(BatchStatus::PartiallyProcessed),
            "RequiresReconciliation" => Ok(BatchStatus::RequiresReconciliation),
            _ => Err(format!("Invalid BatchStatus: {s}")),
        }
    }
}

/// Reconciliation data for batch processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciliationData {
    pub id: Uuid,
    pub collection_batch_id: Uuid,
    pub expected_amount: Decimal,
    pub actual_amount: Decimal,
    pub variance: Decimal,
    pub variance_reason: Option<HeaplessString<500>>,
    pub reconciled_by_person_id: Uuid,
    pub reconciliation_timestamp: DateTime<Utc>,
    pub adjustment_required: bool,
}

// ======== Builder Patterns ========

/// Builder for CollectionAgent
pub struct CollectionAgentBuilder {
    id: Uuid,
    person_id: Option<Uuid>,
    license_number: Option<HeaplessString<50>>,
    license_expiry: Option<NaiveDate>,
    status: AgentStatus,
    assigned_territory_id: Option<Uuid>,
    agent_performance_metrics_id: Option<Uuid>,
    cash_limit: Option<Decimal>,
    device_information_id: Option<Uuid>,
}

impl CollectionAgent {
    pub fn builder(id: Uuid) -> CollectionAgentBuilder {
        CollectionAgentBuilder {
            id,
            person_id: None,
            license_number: None,
            license_expiry: None,
            status: AgentStatus::Training,
            assigned_territory_id: None,
            agent_performance_metrics_id: None,
            cash_limit: None,
            device_information_id: None,
        }
    }
}

impl CollectionAgentBuilder {
    pub fn person_id(mut self, person_id: Uuid) -> Self {
        self.person_id = Some(person_id);
        self
    }


    pub fn license_number(mut self, license: &str) -> Result<Self, String> {
        self.license_number = Some(HeaplessString::try_from(license)
            .map_err(|_| format!("License number too long: {license}"))?);
        Ok(self)
    }

    pub fn license_expiry(mut self, expiry: NaiveDate) -> Self {
        self.license_expiry = Some(expiry);
        self
    }

    pub fn status(mut self, status: AgentStatus) -> Self {
        self.status = status;
        self
    }

    pub fn assigned_territory_id(mut self, id: Uuid) -> Self {
        self.assigned_territory_id = Some(id);
        self
    }

    pub fn agent_performance_metrics_id(mut self, id: Uuid) -> Self {
        self.agent_performance_metrics_id = Some(id);
        self
    }

    pub fn cash_limit(mut self, limit: Decimal) -> Self {
        self.cash_limit = Some(limit);
        self
    }

    pub fn device_information_id(mut self, id: Uuid) -> Self {
        self.device_information_id = Some(id);
        self
    }

    pub fn build(self) -> Result<CollectionAgent, String> {
        let now = Utc::now();

        // Generate default performance metrics ID
        let default_metrics_id = Uuid::new_v4();

        Ok(CollectionAgent {
            id: self.id,
            person_id: self.person_id.ok_or("Person reference is required")?,
            license_number: self.license_number.ok_or("License number is required")?,
            license_expiry: self.license_expiry.ok_or("License expiry is required")?,
            status: self.status,
            assigned_territory_id: self.assigned_territory_id.ok_or("Territory assignment is required")?,
            agent_performance_metrics_id: self.agent_performance_metrics_id.unwrap_or(default_metrics_id),
            cash_limit: self.cash_limit.ok_or("Cash limit is required")?,
            device_information_id: self.device_information_id.ok_or("Device information ID is required")?,
            created_at: now,
            updated_at: now,
        })
    }
}

/// Builder for CollectionProgram
pub struct CollectionProgramBuilder {
    id: Uuid,
    name: Option<HeaplessString<100>>,
    description: Option<HeaplessString<500>>,
    program_type: CollectionProgramType,
    status: ProgramStatus,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    collection_frequency: Option<CollectionFrequency>,
    operating_hours_id: Option<Uuid>,
    minimum_amount: Option<Decimal>,
    maximum_amount: Option<Decimal>,
    target_amount: Option<Decimal>,
    program_duration_days: Option<i32>,
    graduation_criteria_id: Option<Uuid>,
    fee_structure_id: Option<Uuid>,
    interest_rate: Option<Decimal>,
    created_by_person_id: Uuid,
    reason_id: Option<Uuid>,
}

impl CollectionProgram {
    pub fn builder(id: Uuid, program_type: CollectionProgramType, created_by_person_id: Uuid) -> CollectionProgramBuilder {
        CollectionProgramBuilder {
            id,
            name: None,
            description: None,
            program_type,
            status: ProgramStatus::Active,
            start_date: None,
            end_date: None,
            collection_frequency: None,
            operating_hours_id: None,
            minimum_amount: None,
            maximum_amount: None,
            target_amount: None,
            program_duration_days: None,
            graduation_criteria_id: None,
            fee_structure_id: None,
            interest_rate: None,
            created_by_person_id,
            reason_id: None,
        }
    }
}

impl CollectionProgramBuilder {
    pub fn name(mut self, name: &str) -> Result<Self, String> {
        self.name = Some(HeaplessString::try_from(name)
            .map_err(|_| format!("Program name too long: {name}"))?);
        Ok(self)
    }

    pub fn description(mut self, description: &str) -> Result<Self, String> {
        self.description = Some(HeaplessString::try_from(description)
            .map_err(|_| format!("Program description too long: {description}"))?);
        Ok(self)
    }

    pub fn status(mut self, status: ProgramStatus) -> Self {
        self.status = status;
        self
    }

    pub fn start_date(mut self, start_date: NaiveDate) -> Self {
        self.start_date = Some(start_date);
        self
    }

    pub fn end_date(mut self, end_date: NaiveDate) -> Self {
        self.end_date = Some(end_date);
        self
    }

    pub fn collection_frequency(mut self, frequency: CollectionFrequency) -> Self {
        self.collection_frequency = Some(frequency);
        self
    }

    pub fn operating_hours_id(mut self, operating_hours_id: Uuid) -> Self {
        self.operating_hours_id = Some(operating_hours_id);
        self
    }

    pub fn amounts(mut self, minimum: Decimal, maximum: Decimal) -> Self {
        self.minimum_amount = Some(minimum);
        self.maximum_amount = Some(maximum);
        self
    }

    pub fn target_amount(mut self, target: Decimal) -> Self {
        self.target_amount = Some(target);
        self
    }

    pub fn program_duration_days(mut self, days: i32) -> Self {
        self.program_duration_days = Some(days);
        self
    }

    pub fn graduation_criteria_id(mut self, id: Uuid) -> Self {
        self.graduation_criteria_id = Some(id);
        self
    }

    pub fn fee_structure_id(mut self, id: Uuid) -> Self {
        self.fee_structure_id = Some(id);
        self
    }

    pub fn interest_rate(mut self, rate: Decimal) -> Self {
        self.interest_rate = Some(rate);
        self
    }

    pub fn reason_id(mut self, reason_id: Uuid) -> Self {
        self.reason_id = Some(reason_id);
        self
    }

    pub fn build(self) -> Result<CollectionProgram, String> {
        let now = Utc::now();
        
        Ok(CollectionProgram {
            id: self.id,
            name: self.name.ok_or("Program name is required")?,
            description: self.description.unwrap_or_default(),
            program_type: self.program_type,
            status: self.status,
            start_date: self.start_date.ok_or("Start date is required")?,
            end_date: self.end_date,
            collection_frequency: self.collection_frequency.ok_or("Collection frequency is required")?,
            operating_hours_id: self.operating_hours_id,
            minimum_amount: self.minimum_amount.ok_or("Minimum amount is required")?,
            maximum_amount: self.maximum_amount.ok_or("Maximum amount is required")?,
            target_amount: self.target_amount,
            program_duration_days: self.program_duration_days.ok_or("Program duration is required")?,
            graduation_criteria_id: self.graduation_criteria_id.ok_or("Graduation criteria ID is required")?,
            fee_structure_id: self.fee_structure_id.ok_or("Fee structure ID is required")?,
            interest_rate: self.interest_rate,
            created_at: now,
            updated_at: now,
            created_by_person_id: self.created_by_person_id,
            reason_id: self.reason_id,
        })
    }
}

/// Builder for CustomerCollectionProfile
pub struct CustomerCollectionProfileBuilder {
    id: Uuid,
    customer_id: Uuid,
    collection_program_id: Uuid,
    account_id: Uuid,
    enrollment_date: Option<NaiveDate>,
    status: CollectionStatus,
    daily_amount: Option<Decimal>,
    collection_schedule: Option<CollectionSchedule>,
    assigned_collection_agent_id: Option<Uuid>,
    collection_location_address_id: Option<Uuid>,
    collection_performance_metrics: Option<CollectionPerformanceMetrics>,
    graduation_progress: Option<GraduationProgress>,
    reason_id: Option<Uuid>,
}

impl CustomerCollectionProfile {
    pub fn builder(
        id: Uuid,
        customer_id: Uuid,
        collection_program_id: Uuid,
        account_id: Uuid,
    ) -> CustomerCollectionProfileBuilder {
        CustomerCollectionProfileBuilder {
            id,
            customer_id,
            collection_program_id,
            account_id,
            enrollment_date: None,
            status: CollectionStatus::Active,
            daily_amount: None,
            collection_schedule: None,
            assigned_collection_agent_id: None,
            collection_location_address_id: None,
            collection_performance_metrics: None,
            graduation_progress: None,
            reason_id: None,
        }
    }
}

impl CustomerCollectionProfileBuilder {
    pub fn enrollment_date(mut self, date: NaiveDate) -> Self {
        self.enrollment_date = Some(date);
        self
    }

    pub fn status(mut self, status: CollectionStatus) -> Self {
        self.status = status;
        self
    }

    pub fn daily_amount(mut self, amount: Decimal) -> Self {
        self.daily_amount = Some(amount);
        self
    }

    pub fn collection_schedule(mut self, schedule: CollectionSchedule) -> Self {
        self.collection_schedule = Some(schedule);
        self
    }

    pub fn assigned_collection_agent_id(mut self, id: Uuid) -> Self {
        self.assigned_collection_agent_id = Some(id);
        self
    }

    pub fn collection_location_address_id(mut self, id: Uuid) -> Self {
        self.collection_location_address_id = Some(id);
        self
    }

    pub fn collection_performance_metrics(mut self, metrics: CollectionPerformanceMetrics) -> Self {
        self.collection_performance_metrics = Some(metrics);
        self
    }

    pub fn graduation_progress(mut self, progress: GraduationProgress) -> Self {
        self.graduation_progress = Some(progress);
        self
    }

    pub fn reason_id(mut self, reason_id: Uuid) -> Self {
        self.reason_id = Some(reason_id);
        self
    }

    pub fn build(self) -> Result<CustomerCollectionProfile, String> {
        let now = Utc::now();
        let enrollment_date = self.enrollment_date.unwrap_or_else(|| now.date_naive());

        Ok(CustomerCollectionProfile {
            id: self.id,
            customer_id: self.customer_id,
            collection_program_id: self.collection_program_id,
            account_id: self.account_id,
            enrollment_date,
            status: self.status,
            daily_amount: self.daily_amount.ok_or("Daily amount is required")?,
            collection_schedule: self.collection_schedule.ok_or("Collection schedule is required")?,
            assigned_collection_agent_id: self.assigned_collection_agent_id.ok_or("Assigned agent ID is required")?,
            collection_location_address_id: self.collection_location_address_id.ok_or("Collection location ID is required")?,
            collection_performance_metrics: self.collection_performance_metrics.ok_or("Collection performance metrics are required")?,
            graduation_progress: self.graduation_progress.ok_or("Graduation progress is required")?,
            created_at: now,
            updated_at: now,
            reason_id: self.reason_id,
        })
    }
}

/// Builder for CollectionRecord
pub struct CollectionRecordBuilder {
    id: Uuid,
    customer_id: Uuid,
    collection_agent_id: Uuid,
    collection_program_id: Uuid,
    account_id: Uuid,
    collection_date: Option<NaiveDate>,
    collection_time: Option<DateTime<Utc>>,
    amount: Option<Decimal>,
    currency: Option<HeaplessString<3>>,
    collection_method: Option<CollectionMethod>,
    location_address_id: Option<Uuid>,
    receipt_number: Option<HeaplessString<50>>,
    status: CollectionRecordStatus,
    notes: Option<HeaplessString<500>>,
    collection_verification_id: Option<Uuid>,
    reason_id: Option<Uuid>,
}

impl CollectionRecord {
    pub fn builder(
        id: Uuid,
        customer_id: Uuid,
        collection_agent_id: Uuid,
        collection_program_id: Uuid,
        account_id: Uuid,
    ) -> CollectionRecordBuilder {
        CollectionRecordBuilder {
            id,
            customer_id,
            collection_agent_id,
            collection_program_id,
            account_id,
            collection_date: None,
            collection_time: None,
            amount: None,
            currency: None,
            collection_method: None,
            location_address_id: None,
            receipt_number: None,
            status: CollectionRecordStatus::Pending,
            notes: None,
            collection_verification_id: None,
            reason_id: None,
        }
    }
}

impl CollectionRecordBuilder {
    pub fn collection_date(mut self, date: NaiveDate) -> Self {
        self.collection_date = Some(date);
        self
    }

    pub fn collection_time(mut self, time: DateTime<Utc>) -> Self {
        self.collection_time = Some(time);
        self
    }

    pub fn amount(mut self, amount: Decimal) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn currency(mut self, currency: &str) -> Result<Self, String> {
        self.currency = Some(HeaplessString::try_from(currency)
            .map_err(|_| format!("Currency code too long: {currency}"))?);
        Ok(self)
    }

    pub fn collection_method(mut self, method: CollectionMethod) -> Self {
        self.collection_method = Some(method);
        self
    }

    pub fn location_address_id(mut self, id: Uuid) -> Self {
        self.location_address_id = Some(id);
        self
    }

    pub fn receipt_number(mut self, receipt: &str) -> Result<Self, String> {
        self.receipt_number = Some(HeaplessString::try_from(receipt)
            .map_err(|_| format!("Receipt number too long: {receipt}"))?);
        Ok(self)
    }

    pub fn status(mut self, status: CollectionRecordStatus) -> Self {
        self.status = status;
        self
    }

    pub fn notes(mut self, notes: &str) -> Result<Self, String> {
        self.notes = Some(HeaplessString::try_from(notes)
            .map_err(|_| format!("Notes too long: {notes}"))?);
        Ok(self)
    }

    pub fn collection_verification_id(mut self, id: Uuid) -> Self {
        self.collection_verification_id = Some(id);
        self
    }

    pub fn reason_id(mut self, reason_id: Uuid) -> Self {
        self.reason_id = Some(reason_id);
        self
    }

    pub fn build(self) -> Result<CollectionRecord, String> {
        let now = Utc::now();

        Ok(CollectionRecord {
            id: self.id,
            customer_id: self.customer_id,
            collection_agent_id: self.collection_agent_id,
            collection_program_id: self.collection_program_id,
            account_id: self.account_id,
            collection_date: self.collection_date.unwrap_or_else(|| now.date_naive()),
            collection_time: self.collection_time.unwrap_or(now),
            amount: self.amount.ok_or("Collection amount is required")?,
            currency: self.currency.ok_or("Currency is required")?,
            collection_method: self.collection_method.ok_or("Collection method is required")?,
            location_address_id: self.location_address_id,
            receipt_number: self.receipt_number.ok_or("Receipt number is required")?,
            status: self.status,
            notes: self.notes,
            collection_verification_id: self.collection_verification_id,
            created_at: now,
            processed_at: None,
            reason_id: self.reason_id,
        })
    }
}
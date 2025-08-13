use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::collateral::AlertSeverity;

// Re-define enums with sqlx::Type support
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "agent_status", rename_all = "PascalCase")]
pub enum AgentStatus {
    Active,
    Suspended,
    Training,
    OnLeave,
    Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "area_type", rename_all = "PascalCase")]
pub enum AreaType {
    Urban,
    Suburban,
    Rural,
    Commercial,
    Industrial,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "customer_density", rename_all = "PascalCase")]
pub enum CustomerDensity {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transport_mode", rename_all = "PascalCase")]
pub enum TransportMode {
    Walking,
    Bicycle,
    Motorcycle,
    Car,
    PublicTransport,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "device_type", rename_all = "PascalCase")]
pub enum DeviceType {
    Smartphone,
    Tablet,
    PortableTerminal,
    SmartWatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "connectivity_status", rename_all = "PascalCase")]
pub enum ConnectivityStatus {
    Online,
    Offline,
    LimitedConnectivity,
    SyncPending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "collection_program_type", rename_all = "PascalCase")]
pub enum CollectionProgramType {
    FixedAmount,
    VariableAmount,
    TargetBased,
    DurationBased,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "program_status", rename_all = "PascalCase")]
pub enum ProgramStatus {
    Active,
    Suspended,
    Closed,
    UnderReview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "collection_frequency", rename_all = "PascalCase")]
pub enum CollectionFrequency {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "collection_status", rename_all = "PascalCase")]
pub enum CollectionStatus {
    Active,
    Suspended,
    Defaulted,
    Graduated,
    Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "holiday_handling", rename_all = "PascalCase")]
pub enum HolidayHandling {
    Skip,
    NextBusinessDay,
    PreviousBusinessDay,
    CollectDouble,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "reliability_rating", rename_all = "PascalCase")]
pub enum ReliabilityRating {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "collection_method", rename_all = "PascalCase")]
pub enum CollectionMethod {
    Cash,
    MobilePayment,
    BankTransfer,
    DigitalWallet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "collection_record_status", rename_all = "PascalCase")]
pub enum CollectionRecordStatus {
    Pending,
    Processed,
    Failed,
    Reversed,
    UnderReview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "biometric_method", rename_all = "PascalCase")]
pub enum BiometricMethod {
    Fingerprint,
    FaceRecognition,
    VoicePrint,
    Combined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "batch_status", rename_all = "PascalCase")]
pub enum BatchStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    PartiallyProcessed,
    RequiresReconciliation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "alert_type", rename_all = "PascalCase")]
pub enum AlertType {
    LowCollectionRate,
    CustomerComplaint,
    CashDiscrepancy,
    MissedSchedule,
    ComplianceViolation,
    SafetyConcern,
    DeviceIssue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "fee_frequency", rename_all = "PascalCase")]
pub enum FeeFrequency {
    PerCollection,
    Daily,
    Weekly,
    Monthly,
    OneTime,
}

// ======== Collection Agent Database Models ========

/// Database model for Collection Agent
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CollectionAgentModel {
    pub id: Uuid,
    pub person_id: Uuid,
    pub license_number: String,
    pub license_expiry: NaiveDate,
    pub status: AgentStatus,
    pub assigned_territory_id: Uuid,
    pub agent_performance_metrics_id: Uuid,
    pub cash_limit: Decimal,
    pub device_information_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for Territory
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TerritoryModel {
    pub id: Uuid,
    pub territory_name: String,
    pub coverage_area_id: Uuid,
    pub customer_count: i32,
    pub route_optimization_enabled: bool,
    pub territory_manager_person_id: Option<Uuid>,
}

/// Database model for Agent Performance Metrics
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AgentPerformanceMetricsModel {
    pub id: Uuid,
    pub collection_rate: Decimal,
    pub customer_satisfaction_score: Decimal,
    pub punctuality_score: Decimal,
    pub cash_handling_accuracy: Decimal,
    pub compliance_score: Decimal,
    pub total_collections: i64,
    pub total_amount_collected: Decimal,
    pub average_collection_time_minutes: i64,
    pub customer_retention_rate: Decimal,
    pub route_efficiency: Decimal,
    pub monthly_targets_id: Uuid,
}

/// Database model for Monthly Targets
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MonthlyTargetsModel {
    pub id: Uuid,
    pub collection_target: Decimal,
    pub customer_target: i32,
    pub satisfaction_target: Decimal,
    pub punctuality_target: Decimal,
    pub accuracy_target: Decimal,
}

/// Database model for Device Information
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DeviceInformationModel {
    pub id: Uuid,
    pub external_id: String,
    pub device_type: DeviceType,
    pub model: String,
    pub os_version: String,
    pub app_version: String,
    pub last_sync: Option<DateTime<Utc>>,
    pub battery_level: Option<f32>,
    pub connectivity_status: ConnectivityStatus,
    pub security_features_id: Uuid,
}

/// Database model for Security Features
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CollectionSecurityFeaturesModel {
    pub id: Uuid,
    pub biometric_enabled: bool,
    pub pin_protection: bool,
    pub encryption_enabled: bool,
    pub remote_wipe_enabled: bool,
    pub certificate_installed: bool,
    pub last_security_scan: Option<DateTime<Utc>>,
}

/// Database model for Coverage Areas
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CoverageAreaModel {
    pub id: Uuid,
    pub area_name: String,
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

/// Database model for Collection Operating Hours
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(sqlx::FromRow)]
pub struct CollectionOperatingHoursModel {
    pub id: Uuid,
    pub program_id: Uuid,
    pub monday_open: Option<NaiveTime>,
    pub monday_close: Option<NaiveTime>,
    pub tuesday_open: Option<NaiveTime>,
    pub tuesday_close: Option<NaiveTime>,
    pub wednesday_open: Option<NaiveTime>,
    pub wednesday_close: Option<NaiveTime>,
    pub thursday_open: Option<NaiveTime>,
    pub thursday_close: Option<NaiveTime>,
    pub friday_open: Option<NaiveTime>,
    pub friday_close: Option<NaiveTime>,
    pub saturday_open: Option<NaiveTime>,
    pub saturday_close: Option<NaiveTime>,
    pub sunday_open: Option<NaiveTime>,
    pub sunday_close: Option<NaiveTime>,
    pub timezone: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for Performance Alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(sqlx::FromRow)]
pub struct PerformanceAlertModel {
    pub id: Uuid,
    pub agent_performance_metrics_id: Uuid,
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: HeaplessString<200>,
    pub acknowledged: bool,
    pub resolution_required: bool,
    pub created_at: DateTime<Utc>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
}

// ======== Collection Program Database Models ========

/// Database model for Collection Program
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(sqlx::FromRow)]
pub struct CollectionProgramModel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
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
    
    // Graduation criteria fields (flattened)
    pub graduation_minimum_balance: Option<Decimal>,
    pub graduation_minimum_collection_rate: Option<Decimal>,
    pub graduation_minimum_duration_days: Option<i32>,
    pub graduation_consecutive_collections_required: Option<i32>,
    pub graduation_target_achievement_required: bool,
    pub graduation_auto_graduation_enabled: bool,
    
    // Fee structure fields (flattened)
    pub fee_setup_fee: Option<Decimal>,
    pub fee_collection_fee: Option<Decimal>,
    pub fee_maintenance_fee: Option<Decimal>,
    pub fee_graduation_fee: Option<Decimal>,
    pub fee_early_termination_fee: Option<Decimal>,
    pub fee_frequency: FeeFrequency,
    
    pub interest_rate: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by_person_id: Uuid,
    pub reason_id: Option<Uuid>,
}

// ======== Customer Collection Profile Database Models ========

/// Database model for Customer Collection Profile
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(sqlx::FromRow)]
pub struct CustomerCollectionProfileModel {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub collection_program_id: Uuid,
    pub account_id: Uuid,
    pub enrollment_date: NaiveDate,
    pub status: CollectionStatus,
    pub daily_amount: Decimal,
    
    // Collection schedule fields (flattened)
    pub schedule_frequency: CollectionFrequency,
    pub schedule_collection_time: NaiveTime,
    pub schedule_timezone: String,
    pub schedule_holiday_handling: HolidayHandling,
    
    pub assigned_collection_agent_id: Uuid,
    pub collection_location_address_id: Uuid,
    
    // Performance metrics fields (flattened)
    pub performance_collection_rate: Decimal,
    pub performance_total_collections: i64,
    pub performance_total_amount_collected: Decimal,
    pub performance_average_collection_amount: Decimal,
    pub performance_consecutive_collections: i32,
    pub performance_missed_collections: i32,
    pub performance_last_collection_date: Option<NaiveDate>,
    pub performance_score: Decimal,
    pub performance_reliability_rating: ReliabilityRating,
    
    // Graduation progress fields (flattened)
    pub graduation_current_balance: Decimal,
    pub graduation_target_balance: Option<Decimal>,
    pub graduation_days_in_program: i32,
    pub graduation_minimum_days_required: Option<i32>,
    pub graduation_collection_consistency_rate: Decimal,
    pub graduation_minimum_consistency_required: Option<Decimal>,
    pub graduation_eligible: bool,
    pub graduation_date: Option<NaiveDate>,
    pub graduation_next_review_date: NaiveDate,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub reason_id: Option<Uuid>,
}

// ======== Collection Record Database Models ========

/// Database model for Collection Record
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(sqlx::FromRow)]
pub struct CollectionRecordModel {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub collection_agent_id: Uuid,
    pub collection_program_id: Uuid,
    pub account_id: Uuid,
    pub collection_date: NaiveDate,
    pub collection_time: DateTime<Utc>,
    pub amount: Decimal,
    pub currency: String,
    pub collection_method: CollectionMethod,
    pub location_address_id: Option<Uuid>,
    pub receipt_number: String,
    pub status: CollectionRecordStatus,
    pub notes: Option<String>,
    
    // Verification data fields (flattened)
    pub verification_customer_signature: Option<String>,
    pub verification_agent_verification_code: Option<String>,
    pub verification_fingerprint_hash: Option<String>,
    pub verification_face_recognition_score: Option<f64>,
    pub verification_biometric_method: Option<BiometricMethod>,
    pub verification_confidence_level: Option<f64>,
    pub verification_customer_photo_hash: Option<String>,
    pub verification_receipt_photo_hash: Option<String>,
    pub verification_location_photo_hash: Option<String>,
    pub verification_photo_timestamp: Option<DateTime<Utc>>,
    pub verification_witness_name: Option<String>,
    pub verification_witness_contact: Option<String>,
    pub verification_witness_relationship: Option<String>,
    pub verification_witness_signature: Option<String>,
    pub verification_timestamp: Option<DateTime<Utc>>,
    
    pub created_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
    pub reason_id: Option<Uuid>,
}

/// Database model for Collection Batch
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(sqlx::FromRow)]
pub struct CollectionBatchModel {
    pub id: Uuid,
    pub collection_agent_id: Uuid,
    pub collection_date: NaiveDate,
    pub total_collections: i32,
    pub total_amount: Decimal,
    pub currency: String,
    pub status: BatchStatus,
    pub collection_records: Vec<Uuid>,
    
    // Reconciliation data fields (flattened)
    pub reconciliation_expected_amount: Option<Decimal>,
    pub reconciliation_actual_amount: Option<Decimal>,
    pub reconciliation_variance: Option<Decimal>,
    pub reconciliation_variance_reason: Option<String>,
    pub reconciled_by_person_id: Option<Uuid>,
    pub reconciliation_timestamp: Option<DateTime<Utc>>,
    pub reconciliation_adjustment_required: Option<bool>,
    
    pub created_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
}

/// Database model for Collection Batch Records (junction table)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(sqlx::FromRow)]
pub struct CollectionBatchRecordModel {
    pub batch_id: Uuid,
    pub collection_record_id: Uuid,
    pub sequence_number: i32,
    pub created_at: DateTime<Utc>,
}


use blake3::Hash;
use chrono::{DateTime, Utc, NaiveDate};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize, Deserializer, Serializer};
use uuid::Uuid;

// Re-export enums from API domain for backward compatibility in mappers
pub use banking_api::domain::KycStatus;

// Domain-aligned enums with custom serialization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlType {
    DirectOwnership,
    IndirectOwnership,
    SignificantInfluence,
    SeniorManagement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationStatus {
    Pending,
    Verified,
    Rejected,
    RequiresUpdate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckResult {
    Pass,
    Fail,
    Warning,
    Manual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreeningType {
    Sanctions,
    PoliticallyExposed,
    AdverseMedia,
    Watchlist,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskLevel::Low => write!(f, "Low"),
            RiskLevel::Medium => write!(f, "Medium"),
            RiskLevel::High => write!(f, "High"),
            RiskLevel::Critical => write!(f, "Critical"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertType {
    StructuringDetection,
    VelocityCheck,
    LargeCashTransaction,
    SuspiciousPattern,
    GeographicAnomaly,
    CrossBorderTransaction,
}

impl std::fmt::Display for AlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertType::StructuringDetection => write!(f, "StructuringDetection"),
            AlertType::VelocityCheck => write!(f, "VelocityCheck"),
            AlertType::LargeCashTransaction => write!(f, "LargeCashTransaction"),
            AlertType::SuspiciousPattern => write!(f, "SuspiciousPattern"),
            AlertType::GeographicAnomaly => write!(f, "GeographicAnomaly"),
            AlertType::CrossBorderTransaction => write!(f, "CrossBorderTransaction"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertStatus {
    New,
    InReview,
    Investigated,
    Cleared,
    Escalated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SarStatus {
    Draft,
    Filed,
    Acknowledged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Passed,
    Failed,
    RequiresReview,
    Pending,
}

// Re-export KycStatus from customer model for local use
pub use banking_api::domain::KycStatus as ComplianceKycStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckType {
    Kyc,
    Aml,
    Cdd,
    Edd,
    SanctionsScreening,
    PepScreening,
    AdverseMediaScreening,
    WatchlistScreening,
    UboVerification,
    DocumentVerification,
    AddressVerification,
    SourceOfFundsVerification,
    SourceOfWealthVerification,
    RiskAssessment,
    OngoingMonitoring,
    PeriodicReview,
}

// Custom serialization for database compatibility
fn serialize_check_type<S>(check_type: &CheckType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let check_type_str = match check_type {
        CheckType::Kyc => "Kyc",
        CheckType::Aml => "Aml",
        CheckType::Cdd => "Cdd",
        CheckType::Edd => "Edd",
        CheckType::SanctionsScreening => "SanctionsScreening",
        CheckType::PepScreening => "PepScreening",
        CheckType::AdverseMediaScreening => "AdverseMediaScreening",
        CheckType::WatchlistScreening => "WatchlistScreening",
        CheckType::UboVerification => "UboVerification",
        CheckType::DocumentVerification => "DocumentVerification",
        CheckType::AddressVerification => "AddressVerification",
        CheckType::SourceOfFundsVerification => "SourceOfFundsVerification",
        CheckType::SourceOfWealthVerification => "SourceOfWealthVerification",
        CheckType::RiskAssessment => "RiskAssessment",
        CheckType::OngoingMonitoring => "OngoingMonitoring",
        CheckType::PeriodicReview => "PeriodicReview",
    };
    serializer.serialize_str(check_type_str)
}

fn deserialize_check_type<'de, D>(deserializer: D) -> Result<CheckType, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Kyc" => Ok(CheckType::Kyc),
        "Aml" => Ok(CheckType::Aml),
        "Cdd" => Ok(CheckType::Cdd),
        "Edd" => Ok(CheckType::Edd),
        "SanctionsScreening" => Ok(CheckType::SanctionsScreening),
        "PepScreening" => Ok(CheckType::PepScreening),
        "AdverseMediaScreening" => Ok(CheckType::AdverseMediaScreening),
        "WatchlistScreening" => Ok(CheckType::WatchlistScreening),
        "UboVerification" => Ok(CheckType::UboVerification),
        "DocumentVerification" => Ok(CheckType::DocumentVerification),
        "AddressVerification" => Ok(CheckType::AddressVerification),
        "SourceOfFundsVerification" => Ok(CheckType::SourceOfFundsVerification),
        "SourceOfWealthVerification" => Ok(CheckType::SourceOfWealthVerification),
        "RiskAssessment" => Ok(CheckType::RiskAssessment),
        "OngoingMonitoring" => Ok(CheckType::OngoingMonitoring),
        "PeriodicReview" => Ok(CheckType::PeriodicReview),
        _ => Err(serde::de::Error::custom(format!("Unknown check type: {s}"))),
    }
}

/// KYC Result database model - aligned with domain KycResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KycResultModel {
    pub customer_id: Uuid,
    #[serde(serialize_with = "serialize_kyc_status", deserialize_with = "deserialize_kyc_status")]
    pub status: KycStatus,
    pub completed_checks: Vec<KycCheckModel>,
    pub missing_documents: Vec<HeaplessString<100>>,
    pub risk_score: Option<Decimal>,
    pub verified_at: Option<DateTime<Utc>>,
}

/// KYC Check database model - aligned with domain KycCheck
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KycCheckModel {
    pub check_type: Hash,
    #[serde(serialize_with = "serialize_check_result", deserialize_with = "deserialize_check_result")]
    pub result: CheckResult,
    pub details: Option<HeaplessString<500>>,
    pub performed_at: DateTime<Utc>,
}

/// KYC Record database model (legacy - kept for repository compatibility)
#[derive(Debug, Clone)]
pub struct KycRecordModel {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub status: KycStatus,
    pub risk_assessment: HeaplessString<100>,
    pub verification_level: HeaplessString<50>, // Basic, Enhanced, Simplified
    pub documents_verified: HeaplessString<500>, // JSON array of document types
    pub last_review_date: Option<NaiveDate>,
    pub next_review_date: Option<NaiveDate>,
    pub reviewed_by: Option<HeaplessString<100>>,
    pub verification_notes: Option<HeaplessString<500>>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by_person_id: HeaplessString<100>,
}

/// Screening Result database model - aligned with domain ScreeningResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreeningResultModel {
    pub customer_id: Uuid,
    #[serde(serialize_with = "serialize_screening_type", deserialize_with = "deserialize_screening_type")]
    pub screening_type: ScreeningType,
    pub matches_found: Vec<SanctionsMatchModel>,
    #[serde(serialize_with = "serialize_risk_level", deserialize_with = "deserialize_risk_level")]
    pub risk_level: RiskLevel,
    pub screened_at: DateTime<Utc>,
    pub requires_manual_review: bool,
}

/// Sanctions Match database model - aligned with domain SanctionsMatch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanctionsMatchModel {
    pub matched_name: HeaplessString<100>,
    pub confidence_score: Decimal,
    pub details: Option<HeaplessString<500>>,
    pub list_source: HeaplessString<50>,
}

/// Sanctions Screening database model (legacy - kept for repository compatibility)
#[derive(Debug, Clone)]
pub struct SanctionsScreeningModel {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub screening_date: DateTime<Utc>,
    pub screening_result: HeaplessString<50>, // Clear, Match, PotentialMatch
    pub match_details: Option<HeaplessString<500>>, // JSON with match information
    pub risk_score: Option<Decimal>,
    pub screening_provider: HeaplessString<50>,
    pub status: HeaplessString<50>, // Pending, Cleared, UnderReview
    pub reviewed_by: Option<HeaplessString<100>>,
    pub review_notes: Option<HeaplessString<500>>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
}

/// Compliance Alert database model - aligned with domain ComplianceAlert
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ComplianceAlertModel {
    pub id: Uuid,
    #[serde(serialize_with = "serialize_alert_type", deserialize_with = "deserialize_alert_type")]
    pub alert_type: AlertType,
    pub description: HeaplessString<500>,
    #[serde(serialize_with = "serialize_severity", deserialize_with = "deserialize_severity")]
    pub severity: Severity,
    pub triggered_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_alert_status", deserialize_with = "deserialize_alert_status")]
    pub status: AlertStatus,
}

/// Extended Compliance Alert database model (for repository use)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedComplianceAlertModel {
    pub id: Uuid,
    pub customer_id: Option<Uuid>,
    pub transaction_id: Option<Uuid>,
    #[serde(serialize_with = "serialize_alert_type", deserialize_with = "deserialize_alert_type")]
    pub alert_type: AlertType,
    #[serde(serialize_with = "serialize_severity", deserialize_with = "deserialize_severity")]
    pub severity: Severity,
    pub description: HeaplessString<500>,
    pub generated_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_alert_status", deserialize_with = "deserialize_alert_status")]
    pub status: AlertStatus,
    pub assigned_to: Option<HeaplessString<100>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<HeaplessString<100>>,
    pub resolution_notes: Option<HeaplessString<500>>,
    pub metadata: Option<HeaplessString<1000>>, // JSON with additional alert data
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
}

/// UBO Verification Result database model - aligned with domain UboVerificationResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UboVerificationResultModel {
    pub corporate_customer_id: Uuid,
    pub ubo_chain: Vec<UboLinkModel>,
    pub verification_complete: bool,
    pub requires_update: Vec<HeaplessString<100>>,
}

/// UBO Link database model - aligned with domain UboLink
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UboLinkModel {
    pub id: Uuid,
    pub beneficiary_customer_id: Uuid,
    pub ownership_percentage: Option<Decimal>,
    #[serde(serialize_with = "serialize_control_type", deserialize_with = "deserialize_control_type")]
    pub control_type: ControlType,
    #[serde(serialize_with = "serialize_verification_status", deserialize_with = "deserialize_verification_status")]
    pub verification_status: VerificationStatus,
}

// UltimateBeneficiaryModel has been moved to banking-db/src/models/account.rs
// Use the account::UltimateBeneficiaryModel instead of this legacy version

/// Compliance Result database model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceResultModel {
    pub id: Uuid,
    pub account_id: Uuid,
    #[serde(serialize_with = "serialize_check_type", deserialize_with = "deserialize_check_type")]
    pub check_type: CheckType,
    #[serde(serialize_with = "serialize_compliance_status", deserialize_with = "deserialize_compliance_status")]
    pub status: ComplianceStatus,
    pub risk_score: Option<Decimal>,
    pub findings: Vec<HeaplessString<300>>,
    pub recommendations: Vec<HeaplessString<300>>,
    pub checked_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Compliance Risk Score database model
#[derive(Debug, Clone)]
pub struct ComplianceRiskScoreModel {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub risk_score: Decimal,
    pub risk_category: HeaplessString<20>, // Low, Medium, High, Critical
    pub calculation_method: HeaplessString<50>,
    pub factors_considered: HeaplessString<1000>, // JSON array of risk factors
    pub calculated_at: DateTime<Utc>,
    pub calculated_by: HeaplessString<100>,
    pub valid_until: Option<NaiveDate>,
    pub notes: Option<HeaplessString<500>>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
}

/// SAR Data database model - aligned with domain SarData
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarDataModel {
    pub id: Uuid,
    pub customer_id: Uuid,
    /// References ReasonAndPurpose.id for SAR reason
    pub reason_id: Uuid,
    /// Additional context for SAR
    pub additional_details: Option<HeaplessString<500>>,
    pub supporting_transactions: Vec<Uuid>,
    pub generated_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_sar_status", deserialize_with = "deserialize_sar_status")]
    pub status: SarStatus,
}

/// Extended SAR Data database model (for repository use)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedSarDataModel {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub reason_id: Uuid,
    pub additional_details: Option<HeaplessString<500>>,
    pub supporting_transactions: Vec<Uuid>,
    pub generated_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_sar_status", deserialize_with = "deserialize_sar_status")]
    pub status: SarStatus,
    pub related_transactions: HeaplessString<1000>, // JSON array of transaction IDs
    pub suspicious_activity_type: HeaplessString<100>,
    pub description: HeaplessString<1000>,
    pub amount_involved: Option<Decimal>,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub prepared_by: HeaplessString<100>,
    pub approved_by: Option<HeaplessString<100>>,
    pub filed_date: Option<NaiveDate>,
    pub reference_number: Option<HeaplessString<100>>,
    pub regulatory_response: Option<HeaplessString<500>>,
    pub supporting_documents: Option<HeaplessString<1000>>, // JSON array
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by_person_id: HeaplessString<100>,
}

/// Customer Document database model (for KYC)
#[derive(Debug, Clone)]
pub struct ComplianceDocumentModel {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub document_type: HeaplessString<50>,
    pub document_path: Hash,
    pub status: HeaplessString<20>, // Uploaded, Verified, Rejected, Expired
    pub uploaded_at: DateTime<Utc>,
    pub uploaded_by: HeaplessString<100>,
    pub verified_at: Option<DateTime<Utc>>,
    pub verified_by: Option<HeaplessString<100>>,
    pub verification_notes: Option<HeaplessString<500>>,
    pub expiry_date: Option<NaiveDate>,
}

/// Customer Audit Trail database model
#[derive(Debug, Clone)]
pub struct ComplianceCustomerAuditModel {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub field_name: HeaplessString<50>,
    pub old_value: Option<HeaplessString<500>>,
    pub new_value: HeaplessString<500>,
    pub changed_at: DateTime<Utc>,
    pub changed_by: HeaplessString<100>,
    pub reason: Option<HeaplessString<200>>,
    pub ip_address: Option<HeaplessString<45>>, // IPv6 max length
    pub user_agent: Option<HeaplessString<200>>,
}

/// Monitoring Result database model - aligned with domain MonitoringResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringResultModel {
    pub transaction_id: Uuid,
    pub alerts_triggered: Vec<ComplianceAlertModel>,
    pub risk_score: Decimal,
    pub requires_investigation: bool,
    pub auto_approved: bool,
}

/// Monitoring Rules database model - aligned with domain MonitoringRules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringRulesModel {
    pub structuring_detection: bool,
    pub velocity_checks: bool,
    pub geographic_risk_assessment: bool,
    pub large_cash_threshold: Decimal,
    pub suspicious_pattern_detection: bool,
    pub cross_border_transaction_monitoring: bool,
}

/// Customer Portfolio View database model (for 360-degree customer view)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCustomerPortfolioModel {
    pub customer_id: Uuid,
    pub total_accounts: i64,
    pub total_balance: Decimal,
    pub last_activity_date: Option<DateTime<Utc>>,
    pub risk_score: Option<Decimal>,
    #[serde(serialize_with = "serialize_kyc_status", deserialize_with = "deserialize_kyc_status")]
    pub kyc_status: KycStatus,
    pub sanctions_checked: bool,
    pub last_screening_date: Option<DateTime<Utc>>,
}

// ============================================================================
// CUSTOM SERIALIZATION FUNCTIONS FOR DATABASE COMPATIBILITY
// ============================================================================

// KycStatus serialization
fn serialize_kyc_status<S>(status: &KycStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let status_str = match status {
        KycStatus::NotStarted => "NotStarted",
        KycStatus::InProgress => "InProgress",
        KycStatus::Pending => "Pending",
        KycStatus::Complete => "Complete",
        KycStatus::Approved => "Approved",
        KycStatus::Rejected => "Rejected",
        KycStatus::RequiresUpdate => "RequiresUpdate",
        KycStatus::Failed => "Failed",
    };
    serializer.serialize_str(status_str)
}

fn deserialize_kyc_status<'de, D>(deserializer: D) -> Result<KycStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "NotStarted" => Ok(KycStatus::NotStarted),
        "InProgress" => Ok(KycStatus::InProgress),
        "Pending" => Ok(KycStatus::Pending),
        "Complete" => Ok(KycStatus::Complete),
        "Approved" => Ok(KycStatus::Approved),
        "Rejected" => Ok(KycStatus::Rejected),
        "RequiresUpdate" => Ok(KycStatus::RequiresUpdate),
        "Failed" => Ok(KycStatus::Failed),
        _ => Err(serde::de::Error::custom(format!("Unknown KYC status: {s}"))),
    }
}

// CheckResult serialization
fn serialize_check_result<S>(result: &CheckResult, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let result_str = match result {
        CheckResult::Pass => "Pass",
        CheckResult::Fail => "Fail",
        CheckResult::Warning => "Warning",
        CheckResult::Manual => "Manual",
    };
    serializer.serialize_str(result_str)
}

fn deserialize_check_result<'de, D>(deserializer: D) -> Result<CheckResult, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Pass" => Ok(CheckResult::Pass),
        "Fail" => Ok(CheckResult::Fail),
        "Warning" => Ok(CheckResult::Warning),
        "Manual" => Ok(CheckResult::Manual),
        _ => Err(serde::de::Error::custom(format!("Unknown check result: {s}"))),
    }
}

// ScreeningType serialization
fn serialize_screening_type<S>(screening_type: &ScreeningType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let type_str = match screening_type {
        ScreeningType::Sanctions => "Sanctions",
        ScreeningType::PoliticallyExposed => "PoliticallyExposed",
        ScreeningType::AdverseMedia => "AdverseMedia",
        ScreeningType::Watchlist => "Watchlist",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_screening_type<'de, D>(deserializer: D) -> Result<ScreeningType, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Sanctions" => Ok(ScreeningType::Sanctions),
        "PoliticallyExposed" => Ok(ScreeningType::PoliticallyExposed),
        "AdverseMedia" => Ok(ScreeningType::AdverseMedia),
        "Watchlist" => Ok(ScreeningType::Watchlist),
        _ => Err(serde::de::Error::custom(format!("Unknown screening type: {s}"))),
    }
}

// RiskLevel serialization
fn serialize_risk_level<S>(risk_level: &RiskLevel, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let level_str = match risk_level {
        RiskLevel::Low => "Low",
        RiskLevel::Medium => "Medium",
        RiskLevel::High => "High",
        RiskLevel::Critical => "Critical",
    };
    serializer.serialize_str(level_str)
}

fn deserialize_risk_level<'de, D>(deserializer: D) -> Result<RiskLevel, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Low" => Ok(RiskLevel::Low),
        "Medium" => Ok(RiskLevel::Medium),
        "High" => Ok(RiskLevel::High),
        "Critical" => Ok(RiskLevel::Critical),
        _ => Err(serde::de::Error::custom(format!("Unknown risk level: {s}"))),
    }
}

// AlertType serialization
fn serialize_alert_type<S>(alert_type: &AlertType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let type_str = match alert_type {
        AlertType::StructuringDetection => "StructuringDetection",
        AlertType::VelocityCheck => "VelocityCheck",
        AlertType::LargeCashTransaction => "LargeCashTransaction",
        AlertType::SuspiciousPattern => "SuspiciousPattern",
        AlertType::GeographicAnomaly => "GeographicAnomaly",
        AlertType::CrossBorderTransaction => "CrossBorderTransaction",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_alert_type<'de, D>(deserializer: D) -> Result<AlertType, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "StructuringDetection" => Ok(AlertType::StructuringDetection),
        "VelocityCheck" => Ok(AlertType::VelocityCheck),
        "LargeCashTransaction" => Ok(AlertType::LargeCashTransaction),
        "SuspiciousPattern" => Ok(AlertType::SuspiciousPattern),
        "GeographicAnomaly" => Ok(AlertType::GeographicAnomaly),
        "CrossBorderTransaction" => Ok(AlertType::CrossBorderTransaction),
        _ => Err(serde::de::Error::custom(format!("Unknown alert type: {s}"))),
    }
}

// Severity serialization
fn serialize_severity<S>(severity: &Severity, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let severity_str = match severity {
        Severity::Low => "Low",
        Severity::Medium => "Medium",
        Severity::High => "High",
        Severity::Critical => "Critical",
    };
    serializer.serialize_str(severity_str)
}

fn deserialize_severity<'de, D>(deserializer: D) -> Result<Severity, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Low" => Ok(Severity::Low),
        "Medium" => Ok(Severity::Medium),
        "High" => Ok(Severity::High),
        "Critical" => Ok(Severity::Critical),
        _ => Err(serde::de::Error::custom(format!("Unknown severity: {s}"))),
    }
}

// AlertStatus serialization
fn serialize_alert_status<S>(status: &AlertStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let status_str = match status {
        AlertStatus::New => "New",
        AlertStatus::InReview => "InReview",
        AlertStatus::Investigated => "Investigated",
        AlertStatus::Cleared => "Cleared",
        AlertStatus::Escalated => "Escalated",
    };
    serializer.serialize_str(status_str)
}

fn deserialize_alert_status<'de, D>(deserializer: D) -> Result<AlertStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "New" => Ok(AlertStatus::New),
        "InReview" => Ok(AlertStatus::InReview),
        "Investigated" => Ok(AlertStatus::Investigated),
        "Cleared" => Ok(AlertStatus::Cleared),
        "Escalated" => Ok(AlertStatus::Escalated),
        _ => Err(serde::de::Error::custom(format!("Unknown alert status: {s}"))),
    }
}

// ControlType serialization
fn serialize_control_type<S>(control_type: &ControlType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let type_str = match control_type {
        ControlType::DirectOwnership => "DirectOwnership",
        ControlType::IndirectOwnership => "IndirectOwnership",
        ControlType::SignificantInfluence => "SignificantInfluence",
        ControlType::SeniorManagement => "SeniorManagement",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_control_type<'de, D>(deserializer: D) -> Result<ControlType, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "DirectOwnership" => Ok(ControlType::DirectOwnership),
        "IndirectOwnership" => Ok(ControlType::IndirectOwnership),
        "SignificantInfluence" => Ok(ControlType::SignificantInfluence),
        "SeniorManagement" => Ok(ControlType::SeniorManagement),
        _ => Err(serde::de::Error::custom(format!("Unknown control type: {s}"))),
    }
}

// VerificationStatus serialization
fn serialize_verification_status<S>(status: &VerificationStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let status_str = match status {
        VerificationStatus::Pending => "Pending",
        VerificationStatus::Verified => "Verified",
        VerificationStatus::Rejected => "Rejected",
        VerificationStatus::RequiresUpdate => "RequiresUpdate",
    };
    serializer.serialize_str(status_str)
}

fn deserialize_verification_status<'de, D>(deserializer: D) -> Result<VerificationStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Pending" => Ok(VerificationStatus::Pending),
        "Verified" => Ok(VerificationStatus::Verified),
        "Rejected" => Ok(VerificationStatus::Rejected),
        "RequiresUpdate" => Ok(VerificationStatus::RequiresUpdate),
        _ => Err(serde::de::Error::custom(format!("Unknown verification status: {s}"))),
    }
}

// ComplianceStatus serialization
fn serialize_compliance_status<S>(status: &ComplianceStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let status_str = match status {
        ComplianceStatus::Passed => "Passed",
        ComplianceStatus::Failed => "Failed",
        ComplianceStatus::RequiresReview => "RequiresReview",
        ComplianceStatus::Pending => "Pending",
    };
    serializer.serialize_str(status_str)
}

fn deserialize_compliance_status<'de, D>(deserializer: D) -> Result<ComplianceStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Passed" => Ok(ComplianceStatus::Passed),
        "Failed" => Ok(ComplianceStatus::Failed),
        "RequiresReview" => Ok(ComplianceStatus::RequiresReview),
        "Pending" => Ok(ComplianceStatus::Pending),
        _ => Err(serde::de::Error::custom(format!("Unknown compliance status: {s}"))),
    }
}

// SarStatus serialization
fn serialize_sar_status<S>(status: &SarStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let status_str = match status {
        SarStatus::Draft => "Draft",
        SarStatus::Filed => "Filed",
        SarStatus::Acknowledged => "Acknowledged",
    };
    serializer.serialize_str(status_str)
}

fn deserialize_sar_status<'de, D>(deserializer: D) -> Result<SarStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Draft" => Ok(SarStatus::Draft),
        "Filed" => Ok(SarStatus::Filed),
        "Acknowledged" => Ok(SarStatus::Acknowledged),
        _ => Err(serde::de::Error::custom(format!("Unknown SAR status: {s}"))),
    }
}
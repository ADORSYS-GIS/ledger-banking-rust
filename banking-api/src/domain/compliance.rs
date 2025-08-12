use blake3::Hash;
use chrono::{DateTime, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KycResult {
    pub customer_id: Uuid,
    pub status: super::customer::KycStatus,
    pub completed_check_01: Option<KycCheck>,
    pub completed_check_02: Option<KycCheck>,
    pub completed_check_03: Option<KycCheck>,
    pub completed_check_04: Option<KycCheck>,
    pub completed_check_05: Option<KycCheck>,
    pub completed_check_06: Option<KycCheck>,
    pub completed_check_07: Option<KycCheck>,
    pub missing_required_document_id_01: Option<Uuid>,
    pub missing_required_document_id_02: Option<Uuid>,
    pub missing_required_document_id_03: Option<Uuid>,
    pub missing_required_document_id_04: Option<Uuid>,
    pub missing_required_document_id_05: Option<Uuid>,
    pub missing_required_document_id_06: Option<Uuid>,
    pub missing_required_document_id_07: Option<Uuid>,
    pub risk_score: Option<Decimal>,
    pub verified_at: Option<DateTime<Utc>>,
}

impl KycResult {
    /// Count the number of missing required documents
    pub fn missing_documents_count(&self) -> usize {
        [
            &self.missing_required_document_id_01,
            &self.missing_required_document_id_02,
            &self.missing_required_document_id_03,
            &self.missing_required_document_id_04,
            &self.missing_required_document_id_05,
            &self.missing_required_document_id_06,
            &self.missing_required_document_id_07,
        ]
        .iter()
        .filter(|id| id.is_some())
        .count()
    }

    /// Count the number of completed checks
    pub fn completed_checks_count(&self) -> usize {
        [
            &self.completed_check_01,
            &self.completed_check_02,
            &self.completed_check_03,
            &self.completed_check_04,
            &self.completed_check_05,
            &self.completed_check_06,
            &self.completed_check_07,
        ]
        .iter()
        .filter(|check| check.is_some())
        .count()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KycCheck {
    pub check_type: Hash,
    pub result: CheckResult,
    pub details: Option<HeaplessString<500>>,
    pub performed_at: DateTime<Utc>,
}

impl KycCheck {
    /// Create new KYC check with hash-based type identifier
    pub fn new(check_type_name: &str, result: CheckResult, details: Option<HeaplessString<500>>) -> Self {
        Self {
            check_type: blake3::hash(check_type_name.as_bytes()),
            result,
            details,
            performed_at: Utc::now(),
        }
    }
    
    /// Get check type as hex string for display/logging
    pub fn check_type_hex(&self) -> String {
        self.check_type.to_hex().to_string()
    }
    
    /// Create hash from check type name for comparison
    pub fn hash_from_type_name(check_type_name: &str) -> Hash {
        blake3::hash(check_type_name.as_bytes())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheckResult {
    Pass,
    Fail,
    Warning,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreeningResult {
    pub customer_id: Uuid,
    pub screening_type: ScreeningType,
    pub found_sanctions_match_01: Option<SanctionsMatch>,
    pub found_sanctions_match_02: Option<SanctionsMatch>,
    pub found_sanctions_match_03: Option<SanctionsMatch>,
    pub risk_level: RiskLevel,
    pub screened_at: DateTime<Utc>,
    pub requires_manual_review: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScreeningType {
    Sanctions,
    PoliticallyExposed,
    AdverseMedia,
    Watchlist,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanctionsMatch {
    pub matched_name: HeaplessString<100>,
    pub confidence_score: Decimal,
    pub details: Option<HeaplessString<500>>,
    pub list_source: HeaplessString<50>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringResult {
    pub transaction_id: Uuid,
    pub triggered_compliance_alert_id_01: Option<Uuid>,
    pub triggered_compliance_alert_id_02: Option<Uuid>,
    pub triggered_compliance_alert_id_03: Option<Uuid>,
    pub risk_score: Decimal,
    pub requires_investigation: bool,
    pub auto_approved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceAlert {
    pub id: Uuid,
    pub customer_id: Option<Uuid>,
    pub account_id: Option<Uuid>,
    pub transaction_id: Option<Uuid>,
    pub alert_type: ComplianceAlertType,
    pub description: HeaplessString<500>,
    pub severity: Severity,
    pub triggered_at: DateTime<Utc>,
    pub status: AlertStatus,
    pub assigned_to_person_id: Option<Uuid>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by_person_id: Option<Uuid>,
    pub resolution_notes: Option<HeaplessString<500>>,
    pub metadata: Option<HeaplessString<1000>>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceAlertType {
    StructuringDetection,
    VelocityCheck,
    LargeCashTransaction,
    SuspiciousPattern,
    GeographicAnomaly,
    CrossBorderTransaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertStatus {
    New,
    InReview,
    Investigated,
    Cleared,
    Escalated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarData {
    pub id: Uuid,
    pub customer_id: Uuid,
    /// References ReasonAndPurpose.id for SAR reason
    pub reason_id: Uuid,
    /// Additional context for SAR
    pub additional_details: Option<HeaplessString<500>>,
    /// References Transaction.id
    pub supporting_transaction_id_01: Option<Uuid>,
    pub supporting_transaction_id_02: Option<Uuid>,
    pub supporting_transaction_id_03: Option<Uuid>,
    pub supporting_transaction_id_04: Option<Uuid>,
    pub supporting_transaction_id_05: Option<Uuid>,
    pub supporting_transaction_id_06: Option<Uuid>,
    pub supporting_transaction_id_07: Option<Uuid>,
    pub supporting_transaction_id_08: Option<Uuid>,
    pub supporting_transaction_id_09: Option<Uuid>,
    pub supporting_transaction_id_10: Option<Uuid>,
    pub supporting_transaction_id_11: Option<Uuid>,
    pub supporting_transaction_id_12: Option<Uuid>,
    pub supporting_transaction_id_13: Option<Uuid>,
    pub supporting_transaction_id_14: Option<Uuid>,
    pub supporting_transaction_id_15: Option<Uuid>,
    pub supporting_transaction_id_16: Option<Uuid>,
    pub supporting_transaction_id_17: Option<Uuid>,
    pub supporting_transaction_id_18: Option<Uuid>,
    pub supporting_transaction_id_19: Option<Uuid>,
    pub generated_at: DateTime<Utc>,
    pub status: SarStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SarStatus {
    Draft,
    Filed,
    Acknowledged,
    UnderReview,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UboVerificationResult {
    pub corporate_customer_id: Uuid,
    pub ubo_chain_link_id_01: Option<Uuid>,
    pub ubo_chain_link_id_02: Option<Uuid>,
    pub ubo_chain_link_id_03: Option<Uuid>,
    pub ubo_chain_link_id_04: Option<Uuid>,
    pub ubo_chain_link_id_05: Option<Uuid>,
    pub verification_complete: bool,
    pub requires_update_01: Option<HeaplessString<100>>,
    pub requires_update_02: Option<HeaplessString<100>>,
    pub requires_update_03: Option<HeaplessString<100>>,
    pub requires_update_04: Option<HeaplessString<100>>,
    pub requires_update_05: Option<HeaplessString<100>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UboLink {
    pub id: Uuid,
    pub beneficiary_customer_id: Uuid,
    pub ownership_percentage: Option<Decimal>,
    pub control_type: super::account::ControlType,
    pub verification_status: super::account::VerificationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringRules {
    pub structuring_detection: bool,
    pub velocity_checks: bool,
    pub geographic_risk_assessment: bool,
    pub large_cash_threshold: Decimal,
    pub suspicious_pattern_detection: bool,
    pub cross_border_transaction_monitoring: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceResult {
    pub id: Uuid,
    pub account_id: Uuid,
    pub check_type: CheckType,
    pub status: ComplianceStatus,
    pub risk_score: Option<Decimal>,
    pub findings_01: Option<HeaplessString<300>>,
    pub findings_02: Option<HeaplessString<300>>,
    pub findings_03: Option<HeaplessString<300>>,
    pub findings_04: Option<HeaplessString<300>>,
    pub findings_05: Option<HeaplessString<300>>,
    pub findings_06: Option<HeaplessString<300>>,
    pub findings_07: Option<HeaplessString<300>>,
    pub recommendations_01: Option<HeaplessString<300>>,
    pub recommendations_02: Option<HeaplessString<300>>,
    pub recommendations_03: Option<HeaplessString<300>>,
    pub recommendations_04: Option<HeaplessString<300>>,
    pub recommendations_05: Option<HeaplessString<300>>,
    pub recommendations_06: Option<HeaplessString<300>>,
    pub recommendations_07: Option<HeaplessString<300>>,
    pub checked_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Passed,
    Failed,
    RequiresReview,
    Pending,
}
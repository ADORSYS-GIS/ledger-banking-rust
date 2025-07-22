use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KycResult {
    pub customer_id: Uuid,
    pub status: super::customer::KycStatus,
    pub completed_checks: Vec<KycCheck>,
    pub missing_documents: Vec<String>,
    pub risk_score: Option<Decimal>,
    pub verified_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KycCheck {
    pub check_type: String,
    pub result: CheckResult,
    pub details: Option<String>,
    pub performed_at: DateTime<Utc>,
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
    pub matches_found: Vec<SanctionsMatch>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SanctionsMatch {
    #[validate(length(min = 1, max = 255))]
    pub matched_name: String,
    pub confidence_score: Decimal,
    #[validate(length(max = 500))]
    pub details: Option<String>,
    pub list_source: String,
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
    pub alerts_triggered: Vec<ComplianceAlert>,
    pub risk_score: Decimal,
    pub requires_investigation: bool,
    pub auto_approved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ComplianceAlert {
    pub alert_id: Uuid,
    pub alert_type: AlertType,
    #[validate(length(min = 1, max = 500))]
    pub description: String,
    pub severity: Severity,
    pub triggered_at: DateTime<Utc>,
    pub status: AlertStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
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
    pub sar_id: Uuid,
    pub customer_id: Uuid,
    pub reason: String,
    pub supporting_transactions: Vec<Uuid>,
    pub generated_at: DateTime<Utc>,
    pub status: SarStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SarStatus {
    Draft,
    Filed,
    Acknowledged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UboVerificationResult {
    pub corporate_customer_id: Uuid,
    pub ubo_chain: Vec<UboLink>,
    pub verification_complete: bool,
    pub requires_update: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UboLink {
    pub ubo_id: Uuid,
    pub beneficiary_customer_id: Uuid,
    pub ownership_percentage: Option<Decimal>,
    pub control_type: super::account_relations::ControlType,
    pub verification_status: super::account_relations::VerificationStatus,
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
    pub result_id: Uuid,
    pub account_id: Uuid,
    pub check_type: String,
    pub status: ComplianceStatus,
    pub risk_score: Option<Decimal>,
    pub findings: Vec<String>,
    pub recommendations: Vec<String>,
    pub checked_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Passed,
    Failed,
    RequiresReview,
    Pending,
}
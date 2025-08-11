use async_trait::async_trait;
use banking_api::BankingResult;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};

use crate::models::{SanctionsScreeningModel, ComplianceAlertModel, ComplianceRiskScoreModel, ComplianceResultModel, SarDataModel};
use crate::models::account::UltimateBeneficiaryModel;
use crate::AlertType;

#[async_trait]
pub trait ComplianceRepository: Send + Sync {
    /// Sanctions Screening Operations
    async fn create_sanctions_screening(&self, screening: SanctionsScreeningModel) -> BankingResult<SanctionsScreeningModel>;
    async fn find_screening_by_id(&self, screening_id: Uuid) -> BankingResult<Option<SanctionsScreeningModel>>;
    async fn find_screening_by_customer(&self, customer_id: Uuid) -> BankingResult<Vec<SanctionsScreeningModel>>;
    async fn find_latest_screening(&self, customer_id: Uuid) -> BankingResult<Option<SanctionsScreeningModel>>;
    async fn find_positive_screenings(&self) -> BankingResult<Vec<SanctionsScreeningModel>>;
    async fn find_screenings_requiring_review(&self) -> BankingResult<Vec<SanctionsScreeningModel>>;
    async fn update_screening_status(&self, screening_id: Uuid, status: &str, reviewed_by: &str) -> BankingResult<()>;
    async fn find_customers_needing_screening(&self, days_threshold: i32) -> BankingResult<Vec<Uuid>>;
    
    /// Compliance Alert Operations
    async fn create_alert(&self, alert: ComplianceAlertModel) -> BankingResult<ComplianceAlertModel>;
    async fn find_alert_by_id(&self, alert_id: Uuid) -> BankingResult<Option<ComplianceAlertModel>>;
    async fn find_alerts_by_customer(&self, customer_id: Uuid) -> BankingResult<Vec<ComplianceAlertModel>>;
    async fn find_alerts_by_transaction(&self, transaction_id: Uuid) -> BankingResult<Vec<ComplianceAlertModel>>;
    async fn find_alerts_by_type(&self, alert_type: AlertType) -> BankingResult<Vec<ComplianceAlertModel>>;
    async fn find_alerts_by_status(&self, status: &str) -> BankingResult<Vec<ComplianceAlertModel>>;
    async fn find_open_alerts(&self) -> BankingResult<Vec<ComplianceAlertModel>>;
    async fn update_alert_status(&self, alert_id: Uuid, status: &str, resolved_by_person_id: Option<Uuid>) -> BankingResult<()>;
    async fn find_alerts_by_severity(&self, severity: &str) -> BankingResult<Vec<ComplianceAlertModel>>;
    
    /// Ultimate Beneficial Owner Operations
    async fn create_ubo_link(&self, ubo: UltimateBeneficiaryModel) -> BankingResult<UltimateBeneficiaryModel>;
    async fn update_ubo_link(&self, ubo: UltimateBeneficiaryModel) -> BankingResult<UltimateBeneficiaryModel>;
    async fn find_ubo_by_id(&self, ubo_id: Uuid) -> BankingResult<Option<UltimateBeneficiaryModel>>;
    async fn find_ubo_by_corporate(&self, corporate_customer_id: Uuid) -> BankingResult<Vec<UltimateBeneficiaryModel>>;
    async fn find_ubo_by_beneficiary(&self, beneficiary_customer_id: Uuid) -> BankingResult<Vec<UltimateBeneficiaryModel>>;
    async fn find_ubo_by_verification_status(&self, status: &str) -> BankingResult<Vec<UltimateBeneficiaryModel>>;
    async fn update_ubo_verification_status(&self, ubo_id: Uuid, status: &str, verified_by: &str) -> BankingResult<()>;
    async fn find_ubo_requiring_verification(&self) -> BankingResult<Vec<UltimateBeneficiaryModel>>;
    async fn delete_ubo_link(&self, ubo_id: Uuid) -> BankingResult<()>;
    
    /// Risk Score Operations
    async fn create_risk_score(&self, risk_score: ComplianceRiskScoreModel) -> BankingResult<ComplianceRiskScoreModel>;
    async fn update_risk_score(&self, risk_score: ComplianceRiskScoreModel) -> BankingResult<ComplianceRiskScoreModel>;
    async fn find_risk_score_by_customer(&self, customer_id: Uuid) -> BankingResult<Option<ComplianceRiskScoreModel>>;
    async fn find_high_risk_customers(&self, threshold_score: f64) -> BankingResult<Vec<ComplianceRiskScoreModel>>;
    async fn find_risk_scores_requiring_review(&self, days_threshold: i32) -> BankingResult<Vec<ComplianceRiskScoreModel>>;
    
    /// Compliance Result Operations
    async fn create_compliance_result(&self, result: ComplianceResultModel) -> BankingResult<ComplianceResultModel>;
    async fn find_compliance_result_by_id(&self, result_id: Uuid) -> BankingResult<Option<ComplianceResultModel>>;
    async fn find_compliance_results_by_account(&self, account_id: Uuid) -> BankingResult<Vec<ComplianceResultModel>>;
    async fn find_compliance_results_by_check_type(&self, check_type: &str) -> BankingResult<Vec<ComplianceResultModel>>;
    async fn find_failed_compliance_results(&self) -> BankingResult<Vec<ComplianceResultModel>>;
    
    /// SAR (Suspicious Activity Report) Operations
    async fn create_sar_data(&self, sar: SarDataModel) -> BankingResult<SarDataModel>;
    async fn find_sar_by_id(&self, sar_id: Uuid) -> BankingResult<Option<SarDataModel>>;
    async fn find_sar_by_customer(&self, customer_id: Uuid) -> BankingResult<Vec<SarDataModel>>;
    async fn find_sar_by_status(&self, status: &str) -> BankingResult<Vec<SarDataModel>>;
    async fn update_sar_status(&self, sar_id: Uuid, status: &str, updated_by_person_id: &str) -> BankingResult<()>;
    async fn find_pending_sar_filings(&self) -> BankingResult<Vec<SarDataModel>>;
    
    /// Transaction Monitoring Operations
    async fn record_transaction_monitoring(&self, transaction_id: Uuid, monitoring_result: TransactionMonitoringResult) -> BankingResult<()>;
    async fn find_flagged_transactions(&self, from_date: NaiveDate, to_date: NaiveDate) -> BankingResult<Vec<TransactionMonitoringRecord>>;
    async fn find_transactions_by_pattern(&self, pattern_type: &str) -> BankingResult<Vec<TransactionMonitoringRecord>>;
    
    /// Reporting Operations
    async fn generate_compliance_summary(&self, from_date: NaiveDate, to_date: NaiveDate) -> BankingResult<ComplianceSummaryReport>;
    async fn generate_sanctions_report(&self, from_date: NaiveDate, to_date: NaiveDate) -> BankingResult<SanctionsComplianceReport>;
    async fn generate_alert_summary(&self, from_date: NaiveDate, to_date: NaiveDate) -> BankingResult<AlertSummaryReport>;
    
    /// Utility Operations
    async fn count_sanctions_screenings(&self) -> BankingResult<i64>;
    async fn count_compliance_alerts(&self) -> BankingResult<i64>;
    async fn count_ubo_links(&self) -> BankingResult<i64>;
    async fn count_open_alerts(&self) -> BankingResult<i64>;
    async fn count_pending_reviews(&self) -> BankingResult<i64>;
}

/// Supporting structures for compliance operations
pub struct TransactionMonitoringResult {
    pub risk_score: f64,
    pub patterns_detected: Vec<String>,
    pub requires_investigation: bool,
    pub alert_generated: bool,
}

pub struct TransactionMonitoringRecord {
    pub transaction_id: Uuid,
    pub customer_id: Uuid,
    pub monitoring_date: DateTime<Utc>,
    pub risk_score: f64,
    pub patterns_detected: String,
    pub status: String,
    pub investigated_by: Option<String>,
}

pub struct ComplianceSummaryReport {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_kyc_checks: i64,
    pub total_sanctions_screenings: i64,
    pub total_alerts_generated: i64,
    pub total_alerts_resolved: i64,
    pub total_sar_filings: i64,
    pub high_risk_customers: i64,
}

pub struct KycComplianceReport {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_verifications: i64,
    pub approved_verifications: i64,
    pub rejected_verifications: i64,
    pub pending_verifications: i64,
    pub expired_verifications: i64,
}

pub struct SanctionsComplianceReport {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_screenings: i64,
    pub positive_matches: i64,
    pub false_positives: i64,
    pub pending_reviews: i64,
}

pub struct AlertSummaryReport {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_alerts: i64,
    pub high_severity_alerts: i64,
    pub resolved_alerts: i64,
    pub pending_alerts: i64,
    pub average_resolution_time_hours: f64,
}
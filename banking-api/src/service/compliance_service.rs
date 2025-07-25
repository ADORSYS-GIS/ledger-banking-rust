use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::{
        Customer, Transaction, KycResult, ScreeningResult, MonitoringResult, 
        SarData, UboVerificationResult, VerificationStatus
    },
    error::BankingResult,
};

#[async_trait]
pub trait ComplianceService: Send + Sync {
    /// Perform KYC check on customer
    async fn perform_kyc_check(&self, customer: &Customer) -> BankingResult<KycResult>;
    
    /// Screen customer against sanctions lists
    async fn screen_against_sanctions(&self, customer: &Customer) -> BankingResult<ScreeningResult>;
    
    /// Monitor transaction for compliance violations
    async fn monitor_transaction(&self, transaction: &Transaction) -> BankingResult<MonitoringResult>;
    
    /// Generate SAR (Suspicious Activity Report) data with reason ID validation
    async fn generate_sar_data(&self, customer_id: Uuid, reason_id: Uuid, additional_details: Option<&str>) -> BankingResult<SarData>;
    
    /// Legacy method - deprecated, use generate_sar_data with reason_id instead
    #[deprecated(note = "Use generate_sar_data with reason_id instead")]
    async fn generate_sar_data_legacy(&self, customer_id: Uuid, reason: String) -> BankingResult<SarData>;
    
    /// Ultimate Beneficial Owner verification
    async fn verify_ubo_chain(&self, corporate_customer_id: Uuid) -> BankingResult<UboVerificationResult>;
    async fn update_ubo_status(&self, ubo_link_id: Uuid, status: VerificationStatus) -> BankingResult<()>;

    /// Batch compliance screening for efficiency
    async fn batch_screen_customers(&self, customer_ids: Vec<Uuid>) -> BankingResult<Vec<ScreeningResult>>;

    /// Get compliance alerts for review
    async fn get_pending_compliance_alerts(&self) -> BankingResult<Vec<crate::domain::ComplianceAlert>>;

    /// Update compliance alert status
    async fn update_alert_status(&self, alert_id: Uuid, status: crate::domain::AlertStatus, updated_by: String) -> BankingResult<()>;

    /// Generate compliance report
    async fn generate_compliance_report(&self, from_date: chrono::NaiveDate, to_date: chrono::NaiveDate) -> BankingResult<ComplianceReport>;

    /// Check if customer requires enhanced due diligence
    async fn requires_enhanced_due_diligence(&self, customer_id: Uuid) -> BankingResult<bool>;

    /// Perform enhanced due diligence
    async fn perform_enhanced_due_diligence(&self, customer_id: Uuid) -> BankingResult<EnhancedDueDiligenceResult>;

    /// Update customer risk profile
    async fn update_risk_profile(&self, customer_id: Uuid, risk_factors: Vec<String>) -> BankingResult<()>;

    /// Get transaction monitoring rules
    async fn get_monitoring_rules(&self) -> BankingResult<crate::domain::MonitoringRules>;

    /// Update transaction monitoring rules
    async fn update_monitoring_rules(&self, rules: crate::domain::MonitoringRules) -> BankingResult<()>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComplianceReport {
    pub report_id: Uuid,
    pub from_date: chrono::NaiveDate,
    pub to_date: chrono::NaiveDate,
    pub total_customers_screened: i64,
    pub total_transactions_monitored: i64,
    pub alerts_generated: i64,
    pub sars_filed: i64,
    pub kyc_completions: i64,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnhancedDueDiligenceResult {
    pub customer_id: Uuid,
    pub performed_at: chrono::DateTime<chrono::Utc>,
    pub risk_assessment: crate::domain::RiskLevel,
    pub findings: Vec<String>,
    pub recommendations: Vec<String>,
    pub requires_ongoing_monitoring: bool,
}
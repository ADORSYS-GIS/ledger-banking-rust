use async_trait::async_trait;
use chrono::NaiveDate;
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    error::BankingResult,
    service::{AccrualReport, CapitalizationReport}
};

#[async_trait]
pub trait EodService: Send + Sync {
    /// Interest processing with business day rules
    async fn process_daily_interest_accrual(&self) -> BankingResult<EodReport>;
    async fn post_periodic_interest(&self, processing_date: NaiveDate) -> BankingResult<EodReport>;
    
    /// Fee application with product-specific schedules
    async fn apply_periodic_fees(&self, processing_date: NaiveDate) -> BankingResult<EodReport>;
    
    /// Loan management with grace period calculations
    async fn update_delinquent_loans(&self, processing_date: NaiveDate) -> BankingResult<EodReport>;
    
    /// Regulatory reporting
    async fn generate_regulatory_reports(&self, processing_date: NaiveDate) -> BankingResult<Vec<RegulatoryReport>>;
    
    /// System maintenance
    async fn reset_daily_counters(&self) -> BankingResult<()>;
    async fn archive_completed_workflows(&self) -> BankingResult<()>;
    
    /// Business calendar operations
    async fn determine_next_processing_date(&self, current_date: NaiveDate) -> BankingResult<NaiveDate>;

    /// Complete EOD processing workflow
    async fn run_eod_processing(&self, processing_date: NaiveDate) -> BankingResult<EodProcessingResult>;

    /// Dormancy management from enhancements
    async fn process_dormancy_candidates(&self, processing_date: NaiveDate) -> BankingResult<DormancyReport>;

    /// Account maintenance from enhancements
    async fn process_pending_closures(&self, processing_date: NaiveDate) -> BankingResult<MaintenanceReport>;

    /// Cleanup expired workflows
    async fn cleanup_expired_workflows(&self, processing_date: NaiveDate) -> BankingResult<()>;

    /// Generate regulatory notifications
    async fn generate_regulatory_notifications(&self, processing_date: NaiveDate) -> BankingResult<Vec<RegulatoryNotification>>;

    /// Account maintenance job
    async fn run_account_maintenance(&self, processing_date: NaiveDate) -> BankingResult<MaintenanceReport>;
    
    async fn get_dormancy_threshold(&self, product_id: Uuid) -> BankingResult<i32>;
    
    async fn calculate_inactivity_period(&self, account_id: Uuid, reference_date: NaiveDate) -> BankingResult<i32>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EodReport {
    pub processing_date: NaiveDate,
    pub report_type: String,
    pub status: EodReportStatus,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub records_processed: i64,
    pub records_successful: i64,
    pub records_failed: i64,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EodReportStatus {
    InProgress,
    Completed,
    Failed,
    CompletedWithWarnings,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegulatoryReport {
    pub report_id: uuid::Uuid,
    pub report_type: String,
    pub jurisdiction: String,
    pub reporting_period: NaiveDate,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub status: ReportStatus,
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ReportStatus {
    Generated,
    Submitted,
    Acknowledged,
    Rejected,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EodProcessingResult {
    pub processing_date: NaiveDate,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub interest_accrual: AccrualReport,
    pub interest_capitalization: CapitalizationReport,
    pub fee_processing: EodReport,
    pub loan_updates: EodReport,
    pub dormancy_processing: DormancyReport,
    pub maintenance_processing: MaintenanceReport,
    pub regulatory_reports: Vec<RegulatoryReport>,
    pub overall_status: EodReportStatus,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DormancyReport {
    pub processing_date: NaiveDate,
    pub accounts_evaluated: i32,
    pub accounts_marked_dormant: i32,
    pub accounts_by_product: HashMap<String, i32>,
    pub notifications_generated: i32,
    pub errors_encountered: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MaintenanceReport {
    pub processing_date: NaiveDate,
    pub pending_closures_processed: i32,
    pub closures_completed: i32,
    pub workflows_cleaned: i32,
    pub notifications_sent: i32,
    pub errors_encountered: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegulatoryNotification {
    pub notification_id: uuid::Uuid,
    pub notification_type: String,
    pub recipient: String,
    pub subject: String,
    pub content: String,
    pub status: NotificationStatus,
    pub sent_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum NotificationStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
}
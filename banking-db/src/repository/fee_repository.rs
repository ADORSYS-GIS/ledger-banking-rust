use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;
use banking_api::BankingResult;

use crate::models::{
    FeeApplicationModel, FeeWaiverModel, FeeProcessingJobModel, 
    ProductFeeScheduleModel, FeeCalculationCacheModel
};

/// Repository for fee application data access
#[async_trait]
pub trait FeeRepository: Send + Sync {
    
    // ============================================================================
    // FEE APPLICATION CRUD OPERATIONS
    // ============================================================================
    
    /// Create a new fee application record
    async fn create_fee_application(
        &self,
        fee_application: FeeApplicationModel,
    ) -> BankingResult<FeeApplicationModel>;
    
    /// Update fee application status and details
    async fn update_fee_application(
        &self,
        fee_application: FeeApplicationModel,
    ) -> BankingResult<FeeApplicationModel>;
    
    /// Get fee application by ID
    async fn get_fee_application_by_id(
        &self,
        fee_application_id: Uuid,
    ) -> BankingResult<Option<FeeApplicationModel>>;
    
    /// Get fee applications for an account
    async fn get_fee_applications_for_account(
        &self,
        account_id: Uuid,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
        status_filter: Option<String>,
    ) -> BankingResult<Vec<FeeApplicationModel>>;
    
    /// Get fee applications by status across all accounts
    async fn get_fee_applications_by_status(
        &self,
        status: String,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
        limit: Option<i32>,
    ) -> BankingResult<Vec<FeeApplicationModel>>;
    
    /// Bulk create fee applications (for batch processing)
    async fn bulk_create_fee_applications(
        &self,
        fee_applications: Vec<FeeApplicationModel>,
    ) -> BankingResult<Vec<FeeApplicationModel>>;
    
    // ============================================================================
    // FEE WAIVER OPERATIONS
    // ============================================================================
    
    /// Create a fee waiver record
    async fn create_fee_waiver(
        &self,
        fee_waiver: FeeWaiverModel,
    ) -> BankingResult<FeeWaiverModel>;
    
    /// Update fee waiver approval status
    async fn update_fee_waiver_approval(
        &self,
        waiver_id: Uuid,
        approved_by: String,
        approved_at: DateTime<Utc>,
    ) -> BankingResult<FeeWaiverModel>;
    
    /// Get fee waivers for an account
    async fn get_fee_waivers_for_account(
        &self,
        account_id: Uuid,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> BankingResult<Vec<FeeWaiverModel>>;
    
    /// Get pending fee waiver approvals
    async fn get_pending_fee_waivers(
        &self,
        limit: Option<i32>,
    ) -> BankingResult<Vec<FeeWaiverModel>>;
    
    // ============================================================================
    // BATCH PROCESSING OPERATIONS
    // ============================================================================
    
    /// Create a fee processing job record
    async fn create_fee_processing_job(
        &self,
        job: FeeProcessingJobModel,
    ) -> BankingResult<FeeProcessingJobModel>;
    
    /// Update fee processing job status and results
    async fn update_fee_processing_job(
        &self,
        job: FeeProcessingJobModel,
    ) -> BankingResult<FeeProcessingJobModel>;
    
    /// Get fee processing job by ID
    async fn get_fee_processing_job_by_id(
        &self,
        job_id: Uuid,
    ) -> BankingResult<Option<FeeProcessingJobModel>>;
    
    /// Get fee processing jobs by status and date range
    async fn get_fee_processing_jobs(
        &self,
        status: Option<String>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> BankingResult<Vec<FeeProcessingJobModel>>;
    
    /// Get accounts eligible for periodic fee processing
    async fn get_accounts_eligible_for_fees(
        &self,
        product_ids: Option<Vec<Uuid>>,
        fee_categories: Vec<String>,
        processing_date: NaiveDate,
        offset: i32,
        limit: i32,
    ) -> BankingResult<Vec<Uuid>>;
    
    // ============================================================================
    // PRODUCT CATALOG INTEGRATION
    // ============================================================================
    
    /// Cache product fee schedule from external catalog
    async fn cache_product_fee_schedule(
        &self,
        schedule: ProductFeeScheduleModel,
    ) -> BankingResult<ProductFeeScheduleModel>;
    
    /// Get cached product fee schedule
    async fn get_cached_product_fee_schedule(
        &self,
        product_id: Uuid,
        effective_date: NaiveDate,
    ) -> BankingResult<Option<ProductFeeScheduleModel>>;
    
    /// Invalidate cached fee schedule for product
    async fn invalidate_fee_schedule_cache(
        &self,
        product_id: Uuid,
    ) -> BankingResult<()>;
    
    // ============================================================================
    // FEE CALCULATION CACHE
    // ============================================================================
    
    /// Cache fee calculation result
    async fn cache_fee_calculation(
        &self,
        cache_entry: FeeCalculationCacheModel,
    ) -> BankingResult<FeeCalculationCacheModel>;
    
    /// Get cached fee calculation
    async fn get_cached_fee_calculation(
        &self,
        calculation_key: String,
    ) -> BankingResult<Option<FeeCalculationCacheModel>>;
    
    /// Clean expired fee calculation cache entries
    async fn clean_expired_fee_cache(
        &self,
        cutoff_date: DateTime<Utc>,
    ) -> BankingResult<u32>;
    
    // ============================================================================
    // REPORTING AND ANALYTICS
    // ============================================================================
    
    /// Get fee revenue summary for reporting
    async fn get_fee_revenue_summary(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
        product_ids: Option<Vec<Uuid>>,
        fee_categories: Option<Vec<String>>,
    ) -> BankingResult<FeeRevenueSummary>;
    
    /// Get top fee-generating accounts
    async fn get_top_fee_accounts(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
        limit: i32,
    ) -> BankingResult<Vec<TopFeeAccount>>;
    
    /// Get fee application counts by type and period
    async fn get_fee_application_statistics(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
        group_by: String, // "day", "week", "month"
    ) -> BankingResult<Vec<FeeStatistic>>;
    
    // ============================================================================
    // REVERSAL AND CORRECTION OPERATIONS
    // ============================================================================
    
    /// Mark fee application as reversed
    async fn reverse_fee_application(
        &self,
        fee_application_id: Uuid,
        reversal_reason: String,
        reversed_by: String,
        reversed_at: DateTime<Utc>,
    ) -> BankingResult<FeeApplicationModel>;
    
    /// Bulk reverse fee applications for an account
    async fn bulk_reverse_account_fees(
        &self,
        account_id: Uuid,
        fee_application_ids: Vec<Uuid>,
        reversal_reason: String,
        reversed_by: String,
    ) -> BankingResult<Vec<FeeApplicationModel>>;
}

/// Fee revenue summary for reporting
#[derive(Debug, Clone)]
pub struct FeeRevenueSummary {
    pub total_revenue: Decimal,
    pub fee_count: u32,
    pub waived_amount: Decimal,
    pub reversed_amount: Decimal,
    pub revenue_by_category: std::collections::HashMap<String, Decimal>,
    pub revenue_by_product: std::collections::HashMap<Uuid, Decimal>,
}

/// Top fee account for reporting
#[derive(Debug, Clone)]
pub struct TopFeeAccount {
    pub account_id: Uuid,
    pub total_fees: Decimal,
    pub fee_count: u32,
    pub avg_fee_amount: Decimal,
    pub product_id: Uuid,
}

/// Fee statistics for analytics
#[derive(Debug, Clone)]
pub struct FeeStatistic {
    pub period: String,
    pub fee_category: String,
    pub application_count: u32,
    pub total_amount: Decimal,
    pub waived_count: u32,
    pub waived_amount: Decimal,
}
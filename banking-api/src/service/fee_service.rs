use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    BankingResult,
    domain::{
        FeeApplication, FeeApplicationStatus, FeeTriggerEvent,
        FeeProcessingJob, FeeJobType, ProductFeeSchedule, ProductFee,
        FeeWaiver, FeeCategory,
        fee::FeeType,
    },
};

/// Fee Management Service - handles all fee application logic
/// 
/// This service is responsible for:
/// 1. Event-based (real-time) fee application during transactions
/// 2. Batch-based (periodic) fee processing via EOD/EOM jobs  
/// 3. Fee waiver management and approval workflows
/// 4. Integration with Product Catalog for fee rules
#[async_trait]
pub trait FeeService: Send + Sync {
    
    // ============================================================================
    // EVENT-BASED FEE PROCESSING (Real-time)
    // ============================================================================
    
    /// Apply fees for a specific transaction event in real-time
    /// This is called during transaction processing for atomic fee application
    async fn apply_event_based_fees(
        &self,
        account_id: Uuid,
        transaction_id: Uuid,
        trigger_event: FeeTriggerEvent,
        transaction_amount: Option<Decimal>,
        channel: Option<String>,
    ) -> BankingResult<Vec<FeeApplication>>;
    
    /// Check if an event has associated fees without applying them
    /// Used for pre-transaction validation and user notification
    async fn preview_event_fees(
        &self,
        account_id: Uuid,
        trigger_event: FeeTriggerEvent,
        transaction_amount: Option<Decimal>,
        channel: Option<String>,
    ) -> BankingResult<Vec<FeeApplication>>;
    
    /// Validate if a transaction can proceed given fee requirements
    /// Checks available balance after potential fee deduction
    async fn validate_transaction_with_fees(
        &self,
        account_id: Uuid,
        transaction_amount: Decimal,
        trigger_event: FeeTriggerEvent,
    ) -> BankingResult<bool>;
    
    // ============================================================================
    // BATCH-BASED FEE PROCESSING (Periodic)
    // ============================================================================
    
    /// Schedule a batch fee processing job
    async fn schedule_batch_fee_job(
        &self,
        job_type: FeeJobType,
        processing_date: NaiveDate,
        target_products: Option<Vec<Uuid>>,
        target_categories: Vec<FeeCategory>,
    ) -> BankingResult<FeeProcessingJob>;
    
    /// Execute a batch fee processing job
    /// Processes all eligible accounts for periodic fees
    async fn execute_batch_fee_job(
        &self,
        job_id: Uuid,
    ) -> BankingResult<FeeProcessingJob>;
    
    /// Get accounts eligible for batch fee processing
    async fn get_eligible_accounts_for_fees(
        &self,
        fee_categories: Vec<FeeCategory>,
        processing_date: NaiveDate,
        product_ids: Option<Vec<Uuid>>,
    ) -> BankingResult<Vec<Uuid>>;
    
    /// Process periodic fees for a single account
    async fn apply_periodic_fees_for_account(
        &self,
        account_id: Uuid,
        processing_date: NaiveDate,
        fee_categories: Vec<FeeCategory>,
    ) -> BankingResult<Vec<FeeApplication>>;
    
    // ============================================================================
    // FEE WAIVER MANAGEMENT
    // ============================================================================
    
    /// Request a fee waiver
    async fn request_fee_waiver(
        &self,
        fee_application_id: Uuid,
        reason: String,
        requested_by: String,
    ) -> BankingResult<FeeWaiver>;
    
    /// Approve or reject a fee waiver
    async fn process_fee_waiver(
        &self,
        waiver_id: Uuid,
        approved: bool,
        approved_by: String,
        notes: Option<String>,
    ) -> BankingResult<FeeWaiver>;
    
    /// Automatically waive fees based on business rules
    async fn apply_automatic_waivers(
        &self,
        account_id: Uuid,
        fee_applications: Vec<FeeApplication>,
    ) -> BankingResult<Vec<FeeApplication>>;
    
    // ============================================================================
    // PRODUCT CATALOG INTEGRATION
    // ============================================================================
    
    /// Get fee schedule from Product Catalog for a product
    async fn get_product_fee_schedule(
        &self,
        product_id: Uuid,
    ) -> BankingResult<ProductFeeSchedule>;
    
    /// Refresh fee rules cache from Product Catalog
    async fn refresh_fee_rules_cache(
        &self,
        product_id: Option<Uuid>,
    ) -> BankingResult<()>;
    
    /// Get applicable fees for a specific trigger event
    async fn get_applicable_fees(
        &self,
        product_id: Uuid,
        trigger_event: FeeTriggerEvent,
    ) -> BankingResult<Vec<ProductFee>>;
    
    // ============================================================================
    // FEE CALCULATION ENGINE
    // ============================================================================
    
    /// Calculate fee amount based on product rules
    async fn calculate_fee_amount(
        &self,
        product_fee: &ProductFee,
        base_amount: Option<Decimal>,
        account_balance: Option<Decimal>,
        additional_context: Option<&str>,
    ) -> BankingResult<Decimal>;
    
    /// Apply tiered fee calculation
    async fn calculate_tiered_fee(
        &self,
        product_fee: &ProductFee,
        base_amount: Decimal,
    ) -> BankingResult<Decimal>;
    
    /// Check fee conditions and eligibility
    async fn check_fee_conditions(
        &self,
        account_id: Uuid,
        product_fee: &ProductFee,
        transaction_context: Option<&str>,
    ) -> BankingResult<bool>;
    
    // ============================================================================
    // QUERY AND REPORTING
    // ============================================================================
    
    /// Get fee applications for an account
    async fn get_account_fee_history(
        &self,
        account_id: Uuid,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
        fee_types: Option<Vec<FeeType>>,
    ) -> BankingResult<Vec<FeeApplication>>;
    
    /// Get fee applications by status
    async fn get_fee_applications_by_status(
        &self,
        status: FeeApplicationStatus,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> BankingResult<Vec<FeeApplication>>;
    
    /// Get fee processing job status
    async fn get_fee_job_status(
        &self,
        job_id: Uuid,
    ) -> BankingResult<FeeProcessingJob>;
    
    /// Get fee revenue summary for reporting
    async fn get_fee_revenue_summary(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
        fee_categories: Option<Vec<FeeCategory>>,
        product_ids: Option<Vec<Uuid>>,
    ) -> BankingResult<FeeRevenueSummary>;
    
    // ============================================================================
    // REVERSAL AND CORRECTION
    // ============================================================================
    
    /// Reverse a fee application
    async fn reverse_fee_application(
        &self,
        fee_application_id: Uuid,
        reversal_reason: String,
        reversed_by: String,
    ) -> BankingResult<FeeApplication>;
    
    /// Bulk reverse fees for an account (e.g., account closure)
    async fn bulk_reverse_account_fees(
        &self,
        account_id: Uuid,
        reason: String,
        reversed_by: String,
        fee_types: Option<Vec<FeeType>>,
    ) -> BankingResult<Vec<FeeApplication>>;
}

/// Fee revenue summary for reporting
#[derive(Debug, Clone)]
pub struct FeeRevenueSummary {
    pub total_revenue: Decimal,
    pub fee_count: u32,
    pub revenue_by_category: std::collections::HashMap<FeeCategory, Decimal>,
    pub revenue_by_product: std::collections::HashMap<Uuid, Decimal>,
    pub waived_amount: Decimal,
    pub reversed_amount: Decimal,
    pub period_from: NaiveDate,
    pub period_to: NaiveDate,
}
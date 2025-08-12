use async_trait::async_trait;
use banking_api::BankingResult;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, NaiveDate};

use crate::models::{
    AccountHoldModel, HoldReleaseRecordModel, AccountHoldExpiryJobModel, AccountBalanceCalculationModel,
    AccountHoldSummaryModel, AccountHoldReleaseRequestModel, 
    HoldPrioritySummary, HoldOverrideRecord, HoldAnalyticsSummary, HighHoldRatioAccount,
    JudicialHoldReportData, HoldAgingBucket, HoldValidationError
};


#[async_trait]
pub trait AccountHoldRepository: Send + Sync {
    /// Account Hold Operations
    async fn create_hold(&self, hold: AccountHoldModel) -> BankingResult<AccountHoldModel>;
    async fn find_holds_by_account(&self, account_id: Uuid) -> BankingResult<Vec<AccountHoldModel>>;
    async fn find_active_holds(&self, account_id: Uuid) -> BankingResult<Vec<AccountHoldModel>>;
    /// Release a hold
    /// @param released_by - References Person.person_id
    async fn release_hold(&self, hold_id: Uuid, released_by: Uuid) -> BankingResult<()>;
    async fn release_expired_holds(&self, reference_date: DateTime<Utc>) -> BankingResult<i64>;

    // ============================================================================
    // ENHANCED HOLD OPERATIONS (integrated from HoldRepository)
    // ============================================================================
    
    /// Update an existing hold
    async fn update_hold(
        &self,
        hold: AccountHoldModel,
    ) -> BankingResult<AccountHoldModel>;
    
    /// Get hold by ID
    async fn get_hold_by_id(
        &self,
        hold_id: Uuid,
    ) -> BankingResult<Option<AccountHoldModel>>;
    
    /// Get all active holds for an account with type filtering
    async fn get_active_holds_for_account(
        &self,
        account_id: Uuid,
        hold_types: Option<Vec<String>>,
    ) -> BankingResult<Vec<AccountHoldModel>>;
    
    /// Get holds by status for an account
    async fn get_holds_by_status(
        &self,
        account_id: Option<Uuid>,
        status: String,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> BankingResult<Vec<AccountHoldModel>>;
    
    /// Get holds by type across accounts
    async fn get_holds_by_type(
        &self,
        hold_type: String,
        status: Option<String>,
        account_ids: Option<Vec<Uuid>>,
        limit: Option<i32>,
    ) -> BankingResult<Vec<AccountHoldModel>>;
    
    /// Get hold history for an account
    async fn get_hold_history(
        &self,
        account_id: Uuid,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
        include_released: bool,
    ) -> BankingResult<Vec<AccountHoldModel>>;
    
    // ============================================================================
    // BALANCE CALCULATION OPERATIONS
    // ============================================================================
    
    /// Calculate total active holds for an account
    async fn calculate_total_holds(
        &self,
        account_id: Uuid,
        exclude_hold_types: Option<Vec<String>>,
    ) -> BankingResult<Decimal>;
    
    /// Get hold amounts grouped by priority
    async fn get_hold_amounts_by_priority(
        &self,
        account_id: Uuid,
    ) -> BankingResult<Vec<HoldPrioritySummary>>;
    
    
    /// Cache balance calculation result
    async fn cache_balance_calculation(
        &self,
        calculation: AccountBalanceCalculationModel,
    ) -> BankingResult<AccountBalanceCalculationModel>;
    
    /// Get cached balance calculation if still valid
    async fn get_cached_balance_calculation(
        &self,
        account_id: Uuid,
        max_age_seconds: u64,
    ) -> BankingResult<Option<AccountBalanceCalculationModel>>;
    
    // ============================================================================
    // ENHANCED HOLD RELEASE OPERATIONS
    // ============================================================================
    
    /// Release a hold (completely or partially) - enhanced version
    async fn release_hold_detailed(
        &self,
        hold_id: Uuid,
        release_amount: Option<Decimal>,
        release_reason_id: Uuid, // References ReasonAndPurpose.id
        released_by: Uuid, // References Person.person_id
        released_at: DateTime<Utc>,
    ) -> BankingResult<AccountHoldModel>;
    
    /// Record hold release transaction
    async fn create_hold_release_record(
        &self,
        release_record: HoldReleaseRecordModel,
    ) -> BankingResult<HoldReleaseRecordModel>;
    
    /// Get hold release history
    async fn get_hold_release_records(
        &self,
        hold_id: Uuid,
    ) -> BankingResult<Vec<HoldReleaseRecordModel>>;
    
    /// Bulk release holds
    async fn bulk_release_holds(
        &self,
        hold_ids: Vec<Uuid>,
        release_reason_id: Uuid, // References ReasonAndPurpose.id
        released_by: Uuid, // References Person.person_id
    ) -> BankingResult<Vec<AccountHoldModel>>;
    
    // ============================================================================
    // BATCH PROCESSING OPERATIONS
    // ============================================================================
    
    /// Get expired holds for batch processing
    async fn get_expired_holds(
        &self,
        cutoff_date: DateTime<Utc>,
        hold_types: Option<Vec<String>>,
        limit: Option<i32>,
    ) -> BankingResult<Vec<AccountHoldModel>>;
    
    /// Get holds eligible for automatic release
    async fn get_auto_release_eligible_holds(
        &self,
        processing_date: NaiveDate,
        hold_types: Option<Vec<String>>,
    ) -> BankingResult<Vec<AccountHoldModel>>;
    
    /// Create hold expiry job record
    async fn create_hold_expiry_job(
        &self,
        job: AccountHoldExpiryJobModel,
    ) -> BankingResult<AccountHoldExpiryJobModel>;
    
    /// Update hold expiry job with results
    async fn update_hold_expiry_job(
        &self,
        job: AccountHoldExpiryJobModel,
    ) -> BankingResult<AccountHoldExpiryJobModel>;
    
    /// Bulk place holds (for regulatory/compliance scenarios)
    async fn bulk_place_holds(
        &self,
        holds: Vec<AccountHoldModel>,
    ) -> BankingResult<Vec<AccountHoldModel>>;
    
    // ============================================================================
    // PRIORITY AND AUTHORIZATION MANAGEMENT
    // ============================================================================
    
    /// Update hold priorities for an account
    async fn update_hold_priorities(
        &self,
        account_id: Uuid,
        hold_priority_updates: Vec<(Uuid, String)>, // (hold_id, new_priority)
        updated_by_person_id: Uuid, // References Person.person_id
    ) -> BankingResult<Vec<AccountHoldModel>>;
    
    /// Get holds that can be overridden by priority
    async fn get_overrideable_holds(
        &self,
        account_id: Uuid,
        required_amount: Decimal,
        override_priority: String,
    ) -> BankingResult<Vec<AccountHoldModel>>;
    
    /// Temporarily override holds for transaction authorization
    async fn create_hold_override(
        &self,
        account_id: Uuid,
        overridden_holds: Vec<Uuid>,
        override_amount: Decimal,
        authorized_by: Uuid, // References Person.person_id
        override_reason_id: Uuid, // References ReasonAndPurpose.id
    ) -> BankingResult<HoldOverrideRecord>;
    
    // ============================================================================
    // EXTERNAL SYSTEM INTEGRATION
    // ============================================================================
    
    /// Get judicial holds by court reference
    async fn get_judicial_holds_by_reference(
        &self,
        court_reference: String,
    ) -> BankingResult<Vec<AccountHoldModel>>;
    
    /// Update loan pledge holds when loan status changes
    async fn update_loan_pledge_holds(
        &self,
        loan_account_id: Uuid,
        collateral_account_ids: Vec<Uuid>,
        new_pledge_amount: Decimal,
        updated_by_person_id: Uuid, // References Person.person_id
    ) -> BankingResult<Vec<AccountHoldModel>>;
    
    /// Get compliance holds by alert reference
    async fn get_compliance_holds_by_alert(
        &self,
        compliance_alert_id: Uuid,
    ) -> BankingResult<Vec<AccountHoldModel>>;
    
    // ============================================================================
    // REPORTING AND ANALYTICS
    // ============================================================================
    
    /// Generate hold analytics summary
    async fn get_hold_analytics(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
        hold_types: Option<Vec<String>>,
    ) -> BankingResult<HoldAnalyticsSummary>;
    
    /// Get accounts with high hold ratios
    async fn get_high_hold_ratio_accounts(
        &self,
        minimum_ratio: Decimal,
        exclude_hold_types: Option<Vec<String>>,
        limit: i32,
    ) -> BankingResult<Vec<HighHoldRatioAccount>>;
    
    /// Generate judicial hold report
    async fn generate_judicial_hold_report(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> BankingResult<JudicialHoldReportData>;
    
    /// Get hold aging report
    async fn get_hold_aging_report(
        &self,
        hold_types: Option<Vec<String>>,
        aging_buckets: Vec<i32>, // Days: [30, 60, 90, 365]
    ) -> BankingResult<Vec<HoldAgingBucket>>;
    
    // ============================================================================
    // VALIDATION AND CONSISTENCY
    // ============================================================================
    
    /// Check for hold amount consistency
    async fn validate_hold_amounts(
        &self,
        account_id: Uuid,
    ) -> BankingResult<Vec<HoldValidationError>>;
    
    /// Find orphaned holds (account doesn't exist)
    async fn find_orphaned_holds(
        &self,
        limit: Option<i32>,
    ) -> BankingResult<Vec<AccountHoldModel>>;
    
    /// Clean up old released/expired holds
    async fn cleanup_old_holds(
        &self,
        cutoff_date: NaiveDate,
        hold_statuses: Vec<String>,
    ) -> BankingResult<u32>;

    // AccountBalanceCalculation operations
    async fn create_balance_calculation(&self, calc: AccountBalanceCalculationModel) -> BankingResult<AccountBalanceCalculationModel>;
    async fn find_balance_calculation_by_id(&self, id: Uuid) -> BankingResult<Option<AccountBalanceCalculationModel>>;

    // AccountHoldSummary operations
    async fn create_hold_summary(&self, summary: AccountHoldSummaryModel) -> BankingResult<AccountHoldSummaryModel>;
    async fn find_hold_summaries_by_calc_id(&self, calc_id: Uuid) -> BankingResult<Vec<AccountHoldSummaryModel>>;

    // AccountHoldReleaseRequest operations
    async fn create_hold_release_request(&self, request: AccountHoldReleaseRequestModel) -> BankingResult<AccountHoldReleaseRequestModel>;

    // PlaceHoldRequest operations
    async fn create_place_hold_request(&self, request: AccountHoldModel) -> BankingResult<AccountHoldModel>;
}
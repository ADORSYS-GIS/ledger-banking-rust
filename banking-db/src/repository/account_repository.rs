use async_trait::async_trait;
use banking_api::BankingResult;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, NaiveDate};

use crate::models::{
    AccountModel, AccountOwnershipModel, AccountRelationshipModel, AccountMandateModel, 
    AccountHoldModel, StatusChangeModel, AccountFinalSettlementModel,
    HoldReleaseRecordModel, AccountHoldExpiryJobModel, AccountBalanceCalculationModel
};

#[async_trait]
pub trait AccountRepository: Send + Sync {
    /// Create a new account record
    async fn create(&self, account: AccountModel) -> BankingResult<AccountModel>;
    
    /// Update existing account record
    async fn update(&self, account: AccountModel) -> BankingResult<AccountModel>;
    
    /// Find account by ID
    async fn find_by_id(&self, account_id: Uuid) -> BankingResult<Option<AccountModel>>;
    
    /// Find accounts by customer ID
    async fn find_by_customer_id(&self, customer_id: Uuid) -> BankingResult<Vec<AccountModel>>;
    
    /// Find accounts by product code
    async fn find_by_product_code(&self, product_code: &str) -> BankingResult<Vec<AccountModel>>;
    
    /// Find accounts by status
    async fn find_by_status(&self, status: &str) -> BankingResult<Vec<AccountModel>>;
    
    /// Find accounts eligible for dormancy
    async fn find_dormancy_candidates(&self, reference_date: NaiveDate, threshold_days: i32) -> BankingResult<Vec<AccountModel>>;
    
    /// Find accounts pending closure
    async fn find_pending_closure(&self) -> BankingResult<Vec<AccountModel>>;
    
    /// Find interest-bearing accounts
    async fn find_interest_bearing_accounts(&self) -> BankingResult<Vec<AccountModel>>;
    
    /// Update account status with audit trail
    /// @param changed_by - References Person.person_id
    async fn update_status(&self, account_id: Uuid, status: &str, reason: &str, changed_by: Uuid) -> BankingResult<()>;
    
    /// Update account balance
    async fn update_balance(&self, account_id: Uuid, current_balance: Decimal, available_balance: Decimal) -> BankingResult<()>;
    
    /// Update accrued interest
    async fn update_accrued_interest(&self, account_id: Uuid, accrued_interest: Decimal) -> BankingResult<()>;
    
    /// Reset accrued interest to zero (after capitalization)
    async fn reset_accrued_interest(&self, account_id: Uuid) -> BankingResult<()>;
    
    /// Account Ownership Operations
    async fn create_ownership(&self, ownership: AccountOwnershipModel) -> BankingResult<AccountOwnershipModel>;
    async fn find_ownership_by_account(&self, account_id: Uuid) -> BankingResult<Vec<AccountOwnershipModel>>;
    async fn find_accounts_by_owner(&self, customer_id: Uuid) -> BankingResult<Vec<AccountOwnershipModel>>;
    async fn delete_ownership(&self, ownership_id: Uuid) -> BankingResult<()>;
    
    /// Account Relationship Operations
    async fn create_relationship(&self, relationship: AccountRelationshipModel) -> BankingResult<AccountRelationshipModel>;
    async fn find_relationships_by_account(&self, account_id: Uuid) -> BankingResult<Vec<AccountRelationshipModel>>;
    async fn find_relationships_by_entity(&self, entity_id: Uuid, entity_type: &str) -> BankingResult<Vec<AccountRelationshipModel>>;
    async fn update_relationship(&self, relationship: AccountRelationshipModel) -> BankingResult<AccountRelationshipModel>;
    async fn delete_relationship(&self, relationship_id: Uuid) -> BankingResult<()>;
    
    /// Account Mandate Operations
    async fn create_mandate(&self, mandate: AccountMandateModel) -> BankingResult<AccountMandateModel>;
    async fn find_mandates_by_account(&self, account_id: Uuid) -> BankingResult<Vec<AccountMandateModel>>;
    async fn find_mandates_by_grantee(&self, grantee_customer_id: Uuid) -> BankingResult<Vec<AccountMandateModel>>;
    async fn update_mandate_status(&self, mandate_id: Uuid, status: &str) -> BankingResult<()>;
    async fn find_active_mandates(&self, account_id: Uuid) -> BankingResult<Vec<AccountMandateModel>>;
    
    /// Account Hold Operations
    async fn create_hold(&self, hold: AccountHoldModel) -> BankingResult<AccountHoldModel>;
    async fn find_holds_by_account(&self, account_id: Uuid) -> BankingResult<Vec<AccountHoldModel>>;
    async fn find_active_holds(&self, account_id: Uuid) -> BankingResult<Vec<AccountHoldModel>>;
    /// Release a hold
    /// @param released_by - References Person.person_id
    async fn release_hold(&self, hold_id: Uuid, released_by: Uuid) -> BankingResult<()>;
    async fn release_expired_holds(&self, reference_date: DateTime<Utc>) -> BankingResult<i64>;
    
    /// Final Settlement Operations
    async fn create_final_settlement(&self, settlement: AccountFinalSettlementModel) -> BankingResult<AccountFinalSettlementModel>;
    async fn find_settlement_by_account(&self, account_id: Uuid) -> BankingResult<Option<AccountFinalSettlementModel>>;
    async fn update_settlement_status(&self, settlement_id: Uuid, status: &str) -> BankingResult<()>;
    
    /// Status History Operations
    async fn get_status_history(&self, account_id: Uuid) -> BankingResult<Vec<StatusChangeModel>>;
    async fn add_status_change(&self, status_change: StatusChangeModel) -> BankingResult<StatusChangeModel>;
    
    /// Utility Operations
    async fn exists(&self, account_id: Uuid) -> BankingResult<bool>;
    async fn count_by_customer(&self, customer_id: Uuid) -> BankingResult<i64>;
    async fn count_by_product(&self, product_code: &str) -> BankingResult<i64>;
    async fn list(&self, offset: i64, limit: i64) -> BankingResult<Vec<AccountModel>>;
    async fn count(&self) -> BankingResult<i64>;

    /// Update last activity date for account
    async fn update_last_activity_date(&self, account_id: Uuid, activity_date: chrono::NaiveDate) -> BankingResult<()>;

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
    
    /// Get hold breakdown for balance calculation
    async fn get_hold_breakdown(
        &self,
        account_id: Uuid,
    ) -> BankingResult<Vec<HoldTypeSummary>>;
    
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
}

/// Hold priority summary for balance calculations
#[derive(Debug, Clone)]
pub struct HoldPrioritySummary {
    pub priority: String,
    pub total_amount: Decimal,
    pub hold_count: u32,
}

/// Hold type summary for balance calculations  
#[derive(Debug, Clone)]
pub struct HoldTypeSummary {
    pub hold_type: String,
    pub total_amount: Decimal,
    pub hold_count: u32,
    pub priority: String,
}

/// Hold override record for audit trail
#[derive(Debug, Clone)]
pub struct HoldOverrideRecord {
    pub override_id: Uuid,
    pub account_id: Uuid,
    pub overridden_holds: Vec<Uuid>,
    pub override_amount: Decimal,
    pub authorized_by: Uuid, // References Person.person_id
    pub override_reason_id: Uuid, // References ReasonAndPurpose.id
    pub created_at: DateTime<Utc>,
}

/// Hold analytics summary for reporting
#[derive(Debug, Clone)]
pub struct HoldAnalyticsSummary {
    pub total_hold_amount: Decimal,
    pub active_hold_count: u32,
    pub expired_hold_count: u32,
    pub released_hold_count: u32,
    pub average_hold_duration_days: f64,
    pub hold_by_type: std::collections::HashMap<String, (u32, Decimal)>,
    pub hold_by_priority: std::collections::HashMap<String, (u32, Decimal)>,
}

/// Account with high hold ratio for monitoring
#[derive(Debug, Clone)]
pub struct HighHoldRatioAccount {
    pub account_id: Uuid,
    pub current_balance: Decimal,
    pub total_holds: Decimal,
    pub hold_ratio: Decimal,
    pub active_hold_count: u32,
    pub critical_priority_holds: u32,
}

/// Judicial hold report data
#[derive(Debug, Clone)]  
pub struct JudicialHoldReportData {
    pub total_judicial_holds: u32,
    pub total_amount: Decimal,
    pub active_holds: Vec<AccountHoldModel>,
    pub released_holds: Vec<AccountHoldModel>,
    pub expired_holds: Vec<AccountHoldModel>,
}

/// Hold aging bucket for aging report
#[derive(Debug, Clone)]
pub struct HoldAgingBucket {
    pub bucket_name: String, // "0-30 days", "31-60 days", etc.
    pub min_days: i32,
    pub max_days: Option<i32>,
    pub hold_count: u32,
    pub total_amount: Decimal,
}

/// Hold validation error
#[derive(Debug, Clone)]
pub struct HoldValidationError {
    pub hold_id: Uuid,
    pub error_type: String,
    pub error_message: String,
    pub detected_at: DateTime<Utc>,
}
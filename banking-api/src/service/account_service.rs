use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    BankingResult,
    domain::{
        Account, AccountStatus, AccountHold, HoldType, HoldStatus, HoldPriority, 
        HoldReleaseRequest, BalanceCalculation, HoldSummary, HoldExpiryJob, PlaceHoldRequest
    },
};

#[async_trait]
pub trait AccountService: Send + Sync {
    /// Create a new account
    async fn create_account(&self, account: Account) -> BankingResult<Account>;
    
    /// Find account by ID
    async fn find_account_by_id(&self, account_id: Uuid) -> BankingResult<Option<Account>>;
    
    /// Status updates with immediate enforcement
    /// @param authorized_by_person_id - References Person.person_id
    async fn update_account_status(&self, account_id: Uuid, status: AccountStatus, authorized_by_person_id: Uuid) -> BankingResult<()>;
    
    /// Balance operations with product rule integration
    async fn calculate_balance(&self, account_id: Uuid) -> BankingResult<Decimal>;
    async fn calculate_available_balance(&self, account_id: Uuid) -> BankingResult<Decimal>;
    /// Apply hold with reason ID validation
    async fn apply_hold(&self, account_id: Uuid, amount: Decimal, reason_id: Uuid, additional_details: Option<&str>) -> BankingResult<()>;
    
    /// Legacy method - deprecated, use apply_hold with reason_id instead
    #[deprecated(note = "Use apply_hold with reason_id instead")]
    async fn apply_hold_legacy(&self, account_id: Uuid, amount: Decimal, reason: String) -> BankingResult<()>;
    
    /// Product catalog integration
    async fn refresh_product_rules(&self, account_id: Uuid) -> BankingResult<()>;

    /// Find accounts by customer
    async fn find_accounts_by_customer(&self, customer_id: Uuid) -> BankingResult<Vec<Account>>;

    /// Find accounts by status
    async fn find_accounts_by_status(&self, status: AccountStatus) -> BankingResult<Vec<Account>>;

    /// Find interest bearing accounts
    async fn find_interest_bearing_accounts(&self) -> BankingResult<Vec<Account>>;

    /// Update account balance
    /// @param updated_by_person_id - References Person.person_id
    async fn update_balance(&self, account_id: Uuid, new_balance: Decimal, updated_by_person_id: Uuid) -> BankingResult<()>;

    /// Reset accrued interest to zero
    async fn reset_accrued_interest(&self, account_id: Uuid) -> BankingResult<()>;

    /// Update accrued interest
    async fn update_accrued_interest(&self, account_id: Uuid, amount: Decimal) -> BankingResult<()>;

    /// Get account status (for caching)
    async fn get_account_status(&self, account_id: Uuid) -> BankingResult<AccountStatus>;

    /// Get active holds for an account
    async fn get_active_holds(&self, account_id: Uuid) -> BankingResult<Vec<AccountHold>>;

    /// Release a hold
    /// @param released_by_person_id - References Person.person_id
    async fn release_hold(&self, hold_id: Uuid, released_by_person_id: Uuid) -> BankingResult<()>;

    /// Find accounts eligible for dormancy check
    async fn find_dormancy_candidates(&self, threshold_days: i32) -> BankingResult<Vec<Account>>;

    /// Update last activity date
    async fn update_last_activity_date(&self, account_id: Uuid, activity_date: chrono::NaiveDate) -> BankingResult<()>;

    // ============================================================================
    // HOLD PLACEMENT AND MANAGEMENT (integrated from HoldService)
    // ============================================================================
    
    /// Place a hold on an account with specified amount and type
    /// This immediately impacts available balance but not current balance
    async fn place_hold(
        &self,
        request: PlaceHoldRequest,
    ) -> BankingResult<AccountHold>;
    
    /// Release a hold completely or partially
    async fn release_hold_with_request(
        &self,
        release_request: HoldReleaseRequest,
    ) -> BankingResult<AccountHold>;
    
    /// Modify an existing hold (amount, expiry, etc.)
    async fn modify_hold(
        &self,
        hold_id: Uuid,
        new_amount: Option<Decimal>,
        new_expiry: Option<DateTime<Utc>>,
        new_reason_id: Option<Uuid>, // References ReasonAndPurpose.id
        modified_by_person_id: Uuid, // References Person.person_id
    ) -> BankingResult<AccountHold>;
    
    /// Cancel a hold (mark as cancelled, not released)
    async fn cancel_hold(
        &self,
        hold_id: Uuid,
        cancellation_reason_id: Uuid, // References ReasonAndPurpose.id
        cancelled_by_person_id: Uuid, // References Person.person_id
    ) -> BankingResult<AccountHold>;
    
    // ============================================================================
    // BALANCE CALCULATION ENGINE (enhanced)
    // ============================================================================
    
    /// Calculate real-time available balance for an account
    /// Implements: availableBalance = currentBalance + overdraftLimit - SUM(active holds)
    async fn calculate_available_balance_detailed(
        &self,
        account_id: Uuid,
    ) -> BankingResult<BalanceCalculation>;
    
    /// Validate if a transaction can proceed given hold constraints
    async fn validate_transaction_against_holds(
        &self,
        account_id: Uuid,
        transaction_amount: Decimal,
        ignore_hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<bool>;
    
    /// Get total hold amount by priority for authorization override scenarios
    async fn get_hold_amounts_by_priority(
        &self,
        account_id: Uuid,
    ) -> BankingResult<Vec<HoldSummary>>;
    
    /// Check if sufficient available balance exists after placing a new hold
    async fn validate_hold_placement(
        &self,
        account_id: Uuid,
        additional_hold_amount: Decimal,
        hold_priority: HoldPriority,
    ) -> BankingResult<bool>;
    
    // ============================================================================
    // HOLD QUERIES AND REPORTING
    // ============================================================================
    
    /// Get all active holds for an account with optional type filtering
    async fn get_active_holds_with_types(
        &self,
        account_id: Uuid,
        hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<Vec<AccountHold>>;
    
    /// Get hold by ID
    async fn get_hold_by_id(
        &self,
        hold_id: Uuid,
    ) -> BankingResult<Option<AccountHold>>;
    
    /// Get holds by status and date range
    async fn get_holds_by_status(
        &self,
        account_id: Option<Uuid>,
        status: HoldStatus,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> BankingResult<Vec<AccountHold>>;
    
    /// Get holds by type across multiple accounts (for reporting)
    async fn get_holds_by_type(
        &self,
        hold_type: HoldType,
        status: Option<HoldStatus>,
        account_ids: Option<Vec<Uuid>>,
    ) -> BankingResult<Vec<AccountHold>>;
    
    /// Get hold history for an account including released/expired holds
    async fn get_hold_history(
        &self,
        account_id: Uuid,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> BankingResult<Vec<AccountHold>>;
    
    // ============================================================================
    // BATCH PROCESSING AND AUTOMATION
    // ============================================================================
    
    /// Process expired holds in batch (typically EOD job)
    async fn process_expired_holds(
        &self,
        processing_date: NaiveDate,
        hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<HoldExpiryJob>;
    
    /// Auto-release holds based on business rules
    /// E.g., release uncleared funds holds after clearance period
    async fn process_automatic_releases(
        &self,
        processing_date: NaiveDate,
    ) -> BankingResult<Vec<AccountHold>>;
    
    /// Bulk place holds (e.g., regulatory compliance sweep)
    async fn bulk_place_holds(
        &self,
        account_ids: Vec<Uuid>,
        hold_type: HoldType,
        amount_per_account: Decimal,
        reason_id: Uuid, // References ReasonAndPurpose.id
        placed_by_person_id: Uuid, // References Person.person_id
        expires_at: Option<DateTime<Utc>>,
    ) -> BankingResult<Vec<AccountHold>>;
    
    /// Bulk release holds (e.g., court order resolution)
    async fn bulk_release_holds(
        &self,
        hold_ids: Vec<Uuid>,
        release_reason_id: Uuid, // References ReasonAndPurpose.id
        released_by_person_id: Uuid, // References Person.person_id
    ) -> BankingResult<Vec<AccountHold>>;
    
    // ============================================================================
    // PRIORITY AND AUTHORIZATION MANAGEMENT
    // ============================================================================
    
    /// Override lower priority holds to allow high-priority transactions
    async fn override_holds_for_transaction(
        &self,
        account_id: Uuid,
        transaction_amount: Decimal,
        override_priority: HoldPriority,
        authorized_by_person_id: Uuid, // References Person.person_id
        override_reason_id: Uuid, // References ReasonAndPurpose.id
    ) -> BankingResult<Vec<AccountHold>>;
    
    /// Reorder hold priorities (e.g., judicial lien takes precedence)
    async fn reorder_hold_priorities(
        &self,
        account_id: Uuid,
        hold_priority_map: Vec<(Uuid, HoldPriority)>,
        authorized_by_person_id: Uuid, // References Person.person_id
    ) -> BankingResult<Vec<AccountHold>>;
    
    /// Check authorization level required for hold operations
    async fn get_required_authorization_level(
        &self,
        hold_type: HoldType,
        amount: Decimal,
    ) -> BankingResult<HoldAuthorizationLevel>;
    
    // ============================================================================
    // INTEGRATION WITH EXTERNAL SYSTEMS
    // ============================================================================
    
    /// Sync judicial holds with court system
    async fn sync_judicial_holds(
        &self,
        court_reference: String,
    ) -> BankingResult<Vec<AccountHold>>;
    
    /// Update loan pledge holds when loan status changes
    async fn update_loan_pledge_holds(
        &self,
        loan_account_id: Uuid,
        collateral_account_ids: Vec<Uuid>,
        new_pledge_amount: Decimal,
    ) -> BankingResult<Vec<AccountHold>>;
    
    /// Process regulatory compliance holds (e.g., sanctions screening)
    async fn process_compliance_holds(
        &self,
        compliance_alert_id: Uuid,
        affected_accounts: Vec<Uuid>,
        hold_amount_per_account: Decimal,
    ) -> BankingResult<Vec<AccountHold>>;
    
    // ============================================================================
    // REPORTING AND ANALYTICS
    // ============================================================================
    
    /// Generate hold analytics summary
    async fn get_hold_analytics(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
        hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<HoldAnalytics>;
    
    /// Get accounts with high hold ratios (holds vs balance)
    async fn get_high_hold_ratio_accounts(
        &self,
        minimum_ratio: Decimal, // e.g., 0.8 for 80%
        exclude_hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<Vec<HighHoldAccount>>;
    
    /// Generate regulatory reporting for judicial holds
    async fn generate_judicial_hold_report(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> BankingResult<JudicialHoldReport>;
}

/// Authorization levels for hold operations
#[derive(Debug, Clone)]
pub enum HoldAuthorizationLevel {
    /// Teller or front-line staff
    Standard,
    /// Supervisor approval required
    Supervisor,
    /// Manager approval required
    Manager,
    /// Executive approval required
    Executive,
    /// External authorization required (court order, regulatory)
    External,
}

/// Hold analytics summary for reporting
#[derive(Debug, Clone)]
pub struct HoldAnalytics {
    pub total_hold_amount: Decimal,
    pub active_hold_count: u32,
    pub expired_hold_count: u32,
    pub released_hold_count: u32,
    pub hold_by_type: std::collections::HashMap<HoldType, (u32, Decimal)>,
    pub hold_by_priority: std::collections::HashMap<HoldPriority, (u32, Decimal)>,
    pub average_hold_duration_days: f64,
    pub top_hold_accounts: Vec<Uuid>,
}

/// Account with high hold ratio for risk monitoring
#[derive(Debug, Clone)]
pub struct HighHoldAccount {
    pub account_id: Uuid,
    pub current_balance: Decimal,
    pub total_holds: Decimal,
    pub hold_ratio: Decimal,
    pub active_hold_count: u32,
    pub high_priority_holds: u32,
    pub last_assessment_date: DateTime<Utc>,
}

/// Judicial hold reporting structure
#[derive(Debug, Clone)]
pub struct JudicialHoldReport {
    pub total_judicial_holds: u32,
    pub total_amount: Decimal,
    pub active_holds: Vec<AccountHold>,
    pub released_holds: Vec<AccountHold>,
    pub expired_holds: Vec<AccountHold>,
    pub report_period: (NaiveDate, NaiveDate),
    pub generated_at: DateTime<Utc>,
}
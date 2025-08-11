use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    BankingResult,
    domain::{
        OverdraftFacility, OverdraftUtilization, OverdraftInterestCalculation,
        CasaAccountSummary, OverdraftProcessingJob, OverdraftLimitAdjustment,
        CasaTransactionValidation, InterestPostingRecord, InterestType,
        CompoundingFrequency, ReviewFrequency, CreateOverdraftFacilityRequest,
        transaction::TransactionType
    },
};

/// CASA (Current & Savings Account) Service
/// 
/// Provides specialized functionality for Current and Savings accounts including:
/// - Overdraft facility management
/// - Transaction validation with overdraft consideration
/// - Daily interest calculation on overdrawn balances
/// - EOD processing for overdraft interest accrual and capitalization
#[async_trait]
pub trait CasaService: Send + Sync {
    
    // ============================================================================
    // OVERDRAFT FACILITY MANAGEMENT
    // ============================================================================
    
    /// Create or setup an overdraft facility for a current account
    async fn create_overdraft_facility(
        &self,
        request: CreateOverdraftFacilityRequest,
    ) -> BankingResult<OverdraftFacility>;
    
    /// Update overdraft facility terms (limit, rate, expiry)
    async fn update_overdraft_facility(
        &self,
        facility_id: Uuid,
        new_limit: Option<Decimal>,
        new_rate: Option<Decimal>,
        new_expiry_date: Option<NaiveDate>,
        updated_by_person_id: Uuid, // References Person.person_id
    ) -> BankingResult<OverdraftFacility>;
    
    /// Suspend or activate overdraft facility
    async fn update_overdraft_status(
        &self,
        facility_id: Uuid,
        new_status: crate::domain::OverdraftStatus,
        reason: String,
        updated_by_person_id: Uuid, // References Person.person_id
    ) -> BankingResult<OverdraftFacility>;
    
    /// Get overdraft facility details for an account
    async fn get_overdraft_facility(
        &self,
        account_id: Uuid,
    ) -> BankingResult<Option<OverdraftFacility>>;
    
    /// Request overdraft limit adjustment
    async fn request_overdraft_limit_adjustment(
        &self,
        account_id: Uuid,
        requested_limit: Decimal,
        adjustment_reason: String,
        supporting_documents: Vec<String>,
        requested_by_person_id: Uuid, // References Person.person_id
    ) -> BankingResult<OverdraftLimitAdjustment>;
    
    /// Process overdraft limit adjustment approval
    async fn process_overdraft_adjustment(
        &self,
        adjustment_id: Uuid,
        approved: bool,
        approved_by_person_id: Uuid, // References Person.person_id
        approval_notes: Option<HeaplessString<512>>,
        effective_date: Option<NaiveDate>,
    ) -> BankingResult<OverdraftLimitAdjustment>;
    
    // ============================================================================
    // TRANSACTION VALIDATION WITH OVERDRAFT
    // ============================================================================
    
    /// Validate transaction considering overdraft limits and holds
    /// Integrates with Section 3.1 transaction validation framework
    async fn validate_casa_transaction(
        &self,
        account_id: Uuid,
        transaction_amount: Decimal,
        transaction_type: TransactionType,
        channel: Option<String>,
    ) -> BankingResult<CasaTransactionValidation>;
    
    /// Check if transaction would trigger overdraft usage
    async fn check_overdraft_impact(
        &self,
        account_id: Uuid,
        transaction_amount: Decimal,
    ) -> BankingResult<OverdraftImpactAnalysis>;
    
    /// Pre-authorize overdraft usage for large transactions
    async fn preauthorize_overdraft_usage(
        &self,
        account_id: Uuid,
        requested_amount: Decimal,
        authorization_reason: String,
        authorized_by_person_id: Uuid,
        validity_period: chrono::Duration,
    ) -> BankingResult<OverdraftPreauthorization>;
    
    // ============================================================================
    // OVERDRAFT INTEREST CALCULATION AND ACCRUAL
    // ============================================================================
    
    /// Calculate daily overdraft interest for an account
    async fn calculate_daily_overdraft_interest(
        &self,
        account_id: Uuid,
        calculation_date: NaiveDate,
    ) -> BankingResult<OverdraftInterestCalculation>;
    
    /// Get overdraft utilization history for interest calculation
    async fn get_overdraft_utilization_history(
        &self,
        account_id: Uuid,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> BankingResult<Vec<OverdraftUtilization>>;
    
    /// Record daily overdraft utilization for interest calculation
    async fn record_overdraft_utilization(
        &self,
        account_id: Uuid,
        utilization_date: NaiveDate,
        opening_balance: Decimal,
        closing_balance: Decimal,
        average_daily_balance: Decimal,
    ) -> BankingResult<OverdraftUtilization>;
    
    /// Calculate compound interest on overdrawn amounts
    async fn calculate_compound_overdraft_interest(
        &self,
        account_id: Uuid,
        principal_amount: Decimal,
        annual_rate: Decimal,
        days: u32,
        compounding_frequency: CompoundingFrequency,
    ) -> BankingResult<Decimal>;
    
    // ============================================================================
    // INTEREST POSTING AND CAPITALIZATION
    // ============================================================================
    
    /// Post accrued overdraft interest to account
    async fn post_overdraft_interest(
        &self,
        account_id: Uuid,
        interest_amount: Decimal,
        calculation_period_start: NaiveDate,
        calculation_period_end: NaiveDate,
        posting_date: NaiveDate,
        posted_by_person_id: Uuid,
    ) -> BankingResult<InterestPostingRecord>;
    
    /// Capitalize accrued interest (add to principal)
    async fn capitalize_overdraft_interest(
        &self,
        account_id: Uuid,
        interest_amount: Decimal,
        capitalization_date: NaiveDate,
        authorized_by_person_id: Uuid,
    ) -> BankingResult<InterestPostingRecord>;
    
    /// Get interest posting history for an account
    async fn get_interest_posting_history(
        &self,
        account_id: Uuid,
        interest_type: Option<InterestType>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> BankingResult<Vec<InterestPostingRecord>>;
    
    // ============================================================================
    // EOD BATCH PROCESSING
    // ============================================================================
    
    /// Execute daily overdraft processing job
    /// Called during EOD process to calculate interest on all overdrawn accounts
    async fn execute_daily_overdraft_processing(
        &self,
        processing_date: NaiveDate,
        account_filter: Option<Vec<Uuid>>, // Process specific accounts if provided
    ) -> BankingResult<OverdraftProcessingJob>;
    
    /// Get overdrawn accounts requiring interest calculation
    async fn get_overdrawn_accounts(
        &self,
        as_of_date: NaiveDate,
    ) -> BankingResult<Vec<Uuid>>;
    
    /// Process interest capitalization for accounts due for capitalization
    async fn process_interest_capitalization(
        &self,
        processing_date: NaiveDate,
        capitalization_frequency: CompoundingFrequency,
    ) -> BankingResult<Vec<InterestPostingRecord>>;
    
    /// Generate overdraft processing report
    async fn generate_overdraft_processing_report(
        &self,
        job_id: Uuid,
    ) -> BankingResult<OverdraftProcessingReport>;
    
    // ============================================================================
    // CASA ACCOUNT MANAGEMENT
    // ============================================================================
    
    /// Get comprehensive CASA account summary
    async fn get_casa_account_summary(
        &self,
        account_id: Uuid,
    ) -> BankingResult<CasaAccountSummary>;
    
    /// Update account with overdraft utilization
    async fn update_overdraft_utilization(
        &self,
        account_id: Uuid,
        utilized_amount: Decimal,
    ) -> BankingResult<()>;
    
    /// Check account dormancy risk based on transaction activity
    async fn assess_dormancy_risk(
        &self,
        account_id: Uuid,
        assessment_date: NaiveDate,
    ) -> BankingResult<crate::domain::DormancyRisk>;
    
    /// Get accounts requiring overdraft facility review
    async fn get_accounts_for_overdraft_review(
        &self,
        review_date: NaiveDate,
        review_frequency: ReviewFrequency,
    ) -> BankingResult<Vec<OverdraftFacility>>;
    
    // ============================================================================
    // REPORTING AND ANALYTICS
    // ============================================================================
    
    /// Generate overdraft portfolio analytics
    async fn generate_overdraft_portfolio_analytics(
        &self,
        as_of_date: NaiveDate,
        product_ids: Option<Vec<Uuid>>,
    ) -> BankingResult<OverdraftPortfolioAnalytics>;
    
    /// Get overdraft revenue summary
    async fn get_overdraft_revenue_summary(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
        account_ids: Option<Vec<Uuid>>,
    ) -> BankingResult<OverdraftRevenueSummary>;
    
    /// Generate regulatory reporting for overdraft facilities
    async fn generate_overdraft_regulatory_report(
        &self,
        reporting_date: NaiveDate,
        report_type: OverdraftReportType,
    ) -> BankingResult<OverdraftRegulatoryReport>;
    
    /// Get high-risk overdraft accounts (high utilization, frequent overdrafts)
    async fn get_high_risk_overdraft_accounts(
        &self,
        risk_threshold: Decimal, // Utilization ratio threshold
        assessment_date: NaiveDate,
    ) -> BankingResult<Vec<HighRiskOverdraftAccount>>;
}

/// Overdraft impact analysis for transaction validation
#[derive(Debug, Clone)]
pub struct OverdraftImpactAnalysis {
    pub account_id: Uuid,
    pub current_balance: Decimal,
    pub transaction_amount: Decimal,
    pub post_transaction_balance: Decimal,
    pub overdraft_limit: Option<Decimal>,
    pub will_trigger_overdraft: bool,
    pub overdraft_amount: Option<Decimal>,
    pub available_overdraft_limit: Option<Decimal>,
    pub estimated_daily_interest_cost: Option<Decimal>,
    pub authorization_required: bool,
}

/// Overdraft preauthorization for large transactions
#[derive(Debug, Clone)]
pub struct OverdraftPreauthorization {
    pub preauth_id: Uuid,
    pub account_id: Uuid,
    pub authorized_amount: Decimal,
    pub authorization_reason: String,
    pub authorized_by: String,
    pub valid_from: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub utilized_amount: Decimal,
    pub remaining_amount: Decimal,
    pub status: PreauthorizationStatus,
}

#[derive(Debug, Clone)]
pub enum PreauthorizationStatus {
    Active,
    PartiallyUtilized,
    FullyUtilized,
    Expired,
    Cancelled,
}

/// Overdraft processing report
#[derive(Debug, Clone)]
pub struct OverdraftProcessingReport {
    pub job_id: Uuid,
    pub processing_date: NaiveDate,
    pub total_accounts_processed: u32,
    pub total_overdrawn_accounts: u32,
    pub total_interest_calculated: Decimal,
    pub total_interest_posted: Decimal,
    pub accounts_capitalized: u32,
    pub total_capitalized_amount: Decimal,
    pub processing_duration: chrono::Duration,
    pub errors_encountered: Vec<String>,
    pub summary_by_product: Vec<ProductOverdraftSummary>,
}

/// Product-wise overdraft summary
#[derive(Debug, Clone)]
pub struct ProductOverdraftSummary {
    pub product_id: Uuid,
    pub account_count: u32,
    pub total_overdraft_amount: Decimal,
    pub total_interest_calculated: Decimal,
    pub average_overdraft_utilization: Decimal,
}

/// Overdraft portfolio analytics
#[derive(Debug, Clone)]
pub struct OverdraftPortfolioAnalytics {
    pub as_of_date: NaiveDate,
    pub total_facilities: u32,
    pub active_facilities: u32,
    pub total_approved_limit: Decimal,
    pub total_utilized_amount: Decimal,
    pub utilization_ratio: Decimal,
    pub average_facility_size: Decimal,
    pub average_utilization_per_account: Decimal,
    pub revenue_generated: Decimal,
    pub risk_distribution: OverdraftRiskDistribution,
    pub aging_analysis: OverdraftAgingAnalysis,
}

/// Risk distribution of overdraft facilities
#[derive(Debug, Clone)]
pub struct OverdraftRiskDistribution {
    pub low_risk_facilities: (u32, Decimal),    // Count, Amount
    pub medium_risk_facilities: (u32, Decimal),
    pub high_risk_facilities: (u32, Decimal),
    pub facilities_requiring_review: u32,
}

/// Aging analysis of overdraft usage
#[derive(Debug, Clone)]
pub struct OverdraftAgingAnalysis {
    pub current_overdrafts: (u32, Decimal),     // 0-30 days
    pub aged_30_60_days: (u32, Decimal),
    pub aged_60_90_days: (u32, Decimal),
    pub aged_over_90_days: (u32, Decimal),
    pub chronic_overdraft_accounts: u32, // Overdrawn > 180 days
}

/// Overdraft revenue summary
#[derive(Debug, Clone)]
pub struct OverdraftRevenueSummary {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_interest_income: Decimal,
    pub average_daily_overdraft_balance: Decimal,
    pub peak_overdraft_balance: Decimal,
    pub number_of_overdraft_days: u32,
    pub interest_income_by_month: Vec<(NaiveDate, Decimal)>,
    pub top_revenue_generating_accounts: Vec<(Uuid, Decimal)>,
}

/// High-risk overdraft account
#[derive(Debug, Clone)]
pub struct HighRiskOverdraftAccount {
    pub account_id: Uuid,
    pub current_utilization: Decimal,
    pub utilization_ratio: Decimal,
    pub days_continuously_overdrawn: u32,
    pub total_overdraft_interest_ytd: Decimal,
    pub risk_score: Decimal,
    pub recommended_action: String,
    pub last_review_date: Option<NaiveDate>,
}

/// Types of overdraft regulatory reports
#[derive(Debug, Clone)]
pub enum OverdraftReportType {
    MonthlyOverdraftReport,
    QuarterlyRiskAssessment,
    AnnualOverdraftSummary,
    RegulatoryCompliance,
    StressTestData,
}

/// Overdraft regulatory report
#[derive(Debug, Clone)]
pub struct OverdraftRegulatoryReport {
    pub report_type: OverdraftReportType,
    pub reporting_date: NaiveDate,
    pub report_data: String, // JSON or XML formatted report data
    pub total_facilities: u32,
    pub total_exposure: Decimal,
    pub compliance_status: String,
    pub generated_at: DateTime<Utc>,
}
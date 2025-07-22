use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    BankingResult,
    domain::{
        AmortizationSchedule, AmortizationEntry, LoanDelinquency, LoanPayment,
        PaymentAllocation, PrepaymentHandling, LoanRestructuring, LoanDelinquencyJob,
        LoanPortfolioSummary, CollectionAction, PaymentType, PrepaymentType,
        DelinquencyStage, RestructuringType, CollectionActionType, PaymentMethod,
        InstallmentStatus
    },
};

/// Loan Servicing Service
/// 
/// Comprehensive loan lifecycle management including:
/// - Amortization schedule generation and management
/// - Payment processing and allocation
/// - Delinquency management and collection actions
/// - Prepayment handling with flexible rules
/// - Loan restructuring and modification
/// - Portfolio analytics and reporting
#[async_trait]
pub trait LoanService: Send + Sync {
    
    // ============================================================================
    // AMORTIZATION SCHEDULE MANAGEMENT
    // ============================================================================
    
    /// Generate amortization schedule upon loan disbursement
    /// Called automatically after successful loan disbursement
    async fn generate_amortization_schedule(
        &self,
        loan_account_id: Uuid,
        principal_amount: Decimal,
        annual_interest_rate: Decimal,
        term_months: u32,
        first_payment_date: NaiveDate,
        payment_frequency: PaymentFrequency,
        calculation_method: AmortizationMethod,
    ) -> BankingResult<AmortizationSchedule>;
    
    /// Regenerate amortization schedule after loan modifications
    async fn regenerate_amortization_schedule(
        &self,
        loan_account_id: Uuid,
        remaining_principal: Decimal,
        new_interest_rate: Option<Decimal>,
        new_term_months: Option<u32>,
        effective_date: NaiveDate,
        modification_reason: String,
    ) -> BankingResult<AmortizationSchedule>;
    
    /// Get current amortization schedule for a loan
    async fn get_amortization_schedule(
        &self,
        loan_account_id: Uuid,
        include_paid_installments: bool,
    ) -> BankingResult<AmortizationSchedule>;
    
    /// Get specific installment details
    async fn get_installment_details(
        &self,
        loan_account_id: Uuid,
        installment_number: u32,
    ) -> BankingResult<AmortizationEntry>;
    
    /// Update installment status (paid, overdue, etc.)
    async fn update_installment_status(
        &self,
        entry_id: Uuid,
        new_status: InstallmentStatus,
        paid_date: Option<NaiveDate>,
        paid_amount: Option<Decimal>,
        updated_by: String,
    ) -> BankingResult<AmortizationEntry>;
    
    /// Get upcoming installments for a loan
    async fn get_upcoming_installments(
        &self,
        loan_account_id: Uuid,
        number_of_installments: u32,
    ) -> BankingResult<Vec<AmortizationEntry>>;
    
    // ============================================================================
    // PAYMENT PROCESSING AND ALLOCATION
    // ============================================================================
    
    /// Process loan payment with intelligent allocation
    async fn process_loan_payment(
        &self,
        loan_account_id: Uuid,
        payment_amount: Decimal,
        payment_date: NaiveDate,
        payment_method: PaymentMethod,
        external_reference: Option<String>,
        processed_by: String,
    ) -> BankingResult<LoanPayment>;
    
    /// Calculate payment allocation across loan components
    async fn calculate_payment_allocation(
        &self,
        loan_account_id: Uuid,
        payment_amount: Decimal,
        payment_date: NaiveDate,
    ) -> BankingResult<PaymentAllocation>;
    
    /// Process prepayment with customer choice handling
    async fn process_prepayment(
        &self,
        loan_account_id: Uuid,
        prepayment_amount: Decimal,
        prepayment_type: PrepaymentType,
        customer_choice: bool,
        processed_by: String,
    ) -> BankingResult<PrepaymentHandling>;
    
    /// Apply prepayment to reduce loan term
    async fn apply_prepayment_term_reduction(
        &self,
        loan_account_id: Uuid,
        excess_amount: Decimal,
    ) -> BankingResult<AmortizationSchedule>;
    
    /// Apply prepayment to reduce installment amount
    async fn apply_prepayment_installment_reduction(
        &self,
        loan_account_id: Uuid,
        excess_amount: Decimal,
    ) -> BankingResult<AmortizationSchedule>;
    
    /// Reverse a loan payment
    async fn reverse_loan_payment(
        &self,
        payment_id: Uuid,
        reversal_reason: String,
        reversed_by: String,
    ) -> BankingResult<LoanPayment>;
    
    /// Get payment history for a loan
    async fn get_payment_history(
        &self,
        loan_account_id: Uuid,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
        payment_types: Option<Vec<PaymentType>>,
    ) -> BankingResult<Vec<LoanPayment>>;
    
    // ============================================================================
    // DELINQUENCY MANAGEMENT
    // ============================================================================
    
    /// Identify overdue loans during EOD processing
    /// Called as part of the EOD process to detect delinquent loans
    async fn identify_overdue_loans(
        &self,
        assessment_date: NaiveDate,
        product_codes: Option<Vec<String>>,
    ) -> BankingResult<Vec<Uuid>>;
    
    /// Create or update loan delinquency record
    async fn update_loan_delinquency(
        &self,
        loan_account_id: Uuid,
        assessment_date: NaiveDate,
    ) -> BankingResult<LoanDelinquency>;
    
    /// Calculate days past due for a loan
    async fn calculate_days_past_due(
        &self,
        loan_account_id: Uuid,
        as_of_date: NaiveDate,
    ) -> BankingResult<u32>;
    
    /// Update loan internal status based on delinquency
    async fn update_loan_delinquency_status(
        &self,
        loan_account_id: Uuid,
        delinquency_stage: DelinquencyStage,
        status_change_reason: String,
        updated_by: String,
    ) -> BankingResult<()>;
    
    /// Apply penalty interest on overdue amounts
    async fn apply_penalty_interest(
        &self,
        loan_account_id: Uuid,
        overdue_principal: Decimal,
        overdue_interest: Decimal,
        penalty_rate: Decimal,
        calculation_date: NaiveDate,
    ) -> BankingResult<Decimal>;
    
    /// Get delinquency details for a loan
    async fn get_loan_delinquency(
        &self,
        loan_account_id: Uuid,
    ) -> BankingResult<Option<LoanDelinquency>>;
    
    /// Execute daily delinquency processing job
    async fn execute_daily_delinquency_processing(
        &self,
        processing_date: NaiveDate,
        loan_filter: Option<Vec<Uuid>>,
    ) -> BankingResult<LoanDelinquencyJob>;
    
    // ============================================================================
    // COLLECTION ACTIONS AND NOTIFICATIONS
    // ============================================================================
    
    /// Create collection action for delinquent loan
    async fn create_collection_action(
        &self,
        loan_account_id: Uuid,
        action_type: CollectionActionType,
        description: String,
        amount_demanded: Option<Decimal>,
        due_date: Option<NaiveDate>,
        assigned_to: String,
        created_by: String,
    ) -> BankingResult<CollectionAction>;
    
    /// Update collection action status and response
    async fn update_collection_action(
        &self,
        action_id: Uuid,
        response_received: bool,
        response_details: Option<String>,
        follow_up_required: bool,
        follow_up_date: Option<NaiveDate>,
        updated_by: String,
    ) -> BankingResult<CollectionAction>;
    
    /// Get collection actions for a loan
    async fn get_collection_actions(
        &self,
        loan_account_id: Uuid,
        action_types: Option<Vec<CollectionActionType>>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> BankingResult<Vec<CollectionAction>>;
    
    /// Trigger automated notifications for delinquent loans
    async fn trigger_delinquency_notifications(
        &self,
        loan_account_id: Uuid,
        delinquency_stage: DelinquencyStage,
        notification_channels: Vec<NotificationChannel>,
    ) -> BankingResult<Vec<CollectionAction>>;
    
    /// Generate collection action recommendations
    async fn generate_collection_recommendations(
        &self,
        loan_account_id: Uuid,
        current_dpd: u32,
        payment_history_score: Option<Decimal>,
    ) -> BankingResult<Vec<CollectionRecommendation>>;
    
    // ============================================================================
    // LOAN RESTRUCTURING AND MODIFICATION
    // ============================================================================
    
    /// Request loan restructuring
    async fn request_loan_restructuring(
        &self,
        loan_account_id: Uuid,
        restructuring_type: RestructuringType,
        restructuring_reason: String,
        proposed_terms: RestructuringTerms,
        requested_by: String,
    ) -> BankingResult<LoanRestructuring>;
    
    /// Process restructuring approval
    async fn process_restructuring_approval(
        &self,
        restructuring_id: Uuid,
        approved: bool,
        approved_by: String,
        conditions: Vec<String>,
        effective_date: Option<NaiveDate>,
    ) -> BankingResult<LoanRestructuring>;
    
    /// Apply approved restructuring to loan
    async fn apply_loan_restructuring(
        &self,
        restructuring_id: Uuid,
        effective_date: NaiveDate,
    ) -> BankingResult<AmortizationSchedule>;
    
    /// Check restructuring eligibility for a loan
    async fn check_restructuring_eligibility(
        &self,
        loan_account_id: Uuid,
        restructuring_type: RestructuringType,
    ) -> BankingResult<RestructuringEligibility>;
    
    /// Get restructuring history for a loan
    async fn get_restructuring_history(
        &self,
        loan_account_id: Uuid,
    ) -> BankingResult<Vec<LoanRestructuring>>;
    
    // ============================================================================
    // LOAN PORTFOLIO ANALYTICS AND REPORTING
    // ============================================================================
    
    /// Generate loan portfolio summary
    async fn generate_loan_portfolio_summary(
        &self,
        as_of_date: NaiveDate,
        product_codes: Option<Vec<String>>,
        branch_ids: Option<Vec<Uuid>>,
    ) -> BankingResult<LoanPortfolioSummary>;
    
    /// Get loans requiring attention (high risk, overdue, etc.)
    async fn get_loans_requiring_attention(
        &self,
        attention_types: Vec<AttentionType>,
        priority_threshold: Option<Decimal>,
    ) -> BankingResult<Vec<LoanAttentionItem>>;
    
    /// Generate delinquency aging report
    async fn generate_delinquency_aging_report(
        &self,
        as_of_date: NaiveDate,
        aging_buckets: Vec<u32>, // Days: [30, 60, 90, 180]
    ) -> BankingResult<DelinquencyAgingReport>;
    
    /// Calculate portfolio risk metrics
    async fn calculate_portfolio_risk_metrics(
        &self,
        as_of_date: NaiveDate,
        lookback_months: u32,
    ) -> BankingResult<PortfolioRiskMetrics>;
    
    /// Generate collection effectiveness report
    async fn generate_collection_effectiveness_report(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
        collection_teams: Option<Vec<String>>,
    ) -> BankingResult<CollectionEffectivenessReport>;
    
    // ============================================================================
    // LOAN ACCOUNT STATUS AND LIFECYCLE
    // ============================================================================
    
    /// Update loan account status
    async fn update_loan_account_status(
        &self,
        loan_account_id: Uuid,
        new_status: LoanAccountStatus,
        reason: String,
        updated_by: String,
    ) -> BankingResult<()>;
    
    /// Close loan account after full payment
    async fn close_loan_account(
        &self,
        loan_account_id: Uuid,
        closure_reason: String,
        final_settlement_amount: Option<Decimal>,
        closed_by: String,
    ) -> BankingResult<()>;
    
    /// Calculate early settlement amount
    async fn calculate_early_settlement_amount(
        &self,
        loan_account_id: Uuid,
        settlement_date: NaiveDate,
        include_penalties: bool,
    ) -> BankingResult<EarlySettlementCalculation>;
    
    /// Process loan write-off
    async fn process_loan_write_off(
        &self,
        loan_account_id: Uuid,
        write_off_amount: Decimal,
        write_off_reason: String,
        authorized_by: String,
    ) -> BankingResult<LoanWriteOff>;
}

/// Payment frequency for loan installments
#[derive(Debug, Clone)]
pub enum PaymentFrequency {
    Weekly,
    BiWeekly,
    Monthly,
    Quarterly,
    SemiAnnually,
    Annually,
}

/// Amortization calculation methods
#[derive(Debug, Clone)]
pub enum AmortizationMethod {
    EqualInstallments,      // Equal monthly payments
    EqualPrincipal,         // Equal principal, declining interest
    InterestOnly,           // Interest-only with balloon payment
    BulletPayment,          // Single payment at maturity
}

/// Notification channels for collection actions
#[derive(Debug, Clone)]
pub enum NotificationChannel {
    Email,
    Sms,
    Phone,
    Mail,
    InApp,
    WhatsApp,
}

/// Collection recommendation based on loan profile
#[derive(Debug, Clone)]
pub struct CollectionRecommendation {
    pub recommendation_id: Uuid,
    pub loan_account_id: Uuid,
    pub recommended_action: CollectionActionType,
    pub priority_score: Decimal,
    pub expected_success_rate: Decimal,
    pub estimated_recovery_amount: Decimal,
    pub recommendation_reason: String,
    pub timing_recommendation: String,
}

/// Proposed terms for loan restructuring
#[derive(Debug, Clone)]
pub struct RestructuringTerms {
    pub new_principal: Option<Decimal>,
    pub new_interest_rate: Option<Decimal>,
    pub new_term_months: Option<u32>,
    pub moratorium_period: Option<u32>,
    pub capitalized_interest: Option<Decimal>,
    pub waived_penalty_amount: Option<Decimal>,
    pub new_payment_amount: Option<Decimal>,
}

/// Restructuring eligibility assessment
#[derive(Debug, Clone)]
pub struct RestructuringEligibility {
    pub loan_account_id: Uuid,
    pub is_eligible: bool,
    pub eligibility_reasons: Vec<String>,
    pub available_options: Vec<RestructuringType>,
    pub restrictions: Vec<String>,
    pub required_approvals: Vec<String>,
}

/// Types of loans requiring attention
#[derive(Debug, Clone)]
pub enum AttentionType {
    HighDelinquency,
    LargeExposure,
    FrequentRestructuring,
    PaymentIrregularity,
    CollateralDeterioration,
    CustomerFinancialStress,
}

/// Loan requiring management attention
#[derive(Debug, Clone)]
pub struct LoanAttentionItem {
    pub loan_account_id: Uuid,
    pub attention_type: AttentionType,
    pub severity_score: Decimal,
    pub description: String,
    pub recommended_action: String,
    pub last_review_date: Option<NaiveDate>,
    pub assigned_officer: Option<String>,
}

/// Delinquency aging report structure
#[derive(Debug, Clone)]
pub struct DelinquencyAgingReport {
    pub report_date: NaiveDate,
    pub total_delinquent_loans: u32,
    pub total_delinquent_amount: Decimal,
    pub aging_buckets: Vec<AgingBucket>,
    pub par_ratios: ParRatios,
    pub trend_analysis: TrendAnalysis,
}

/// Aging bucket for delinquency analysis
#[derive(Debug, Clone)]
pub struct AgingBucket {
    pub bucket_name: String,
    pub min_dpd: u32,
    pub max_dpd: Option<u32>,
    pub loan_count: u32,
    pub outstanding_amount: Decimal,
    pub percentage_of_portfolio: Decimal,
}

/// Portfolio at Risk ratios
#[derive(Debug, Clone)]
pub struct ParRatios {
    pub par_30: Decimal,
    pub par_60: Decimal,
    pub par_90: Decimal,
    pub npl_ratio: Decimal,
}

/// Trend analysis for portfolio performance
#[derive(Debug, Clone)]
pub struct TrendAnalysis {
    pub three_month_trend: TrendDirection,
    pub six_month_trend: TrendDirection,
    pub year_over_year_change: Decimal,
    pub seasonal_adjustments: Vec<(String, Decimal)>,
}

#[derive(Debug, Clone)]
pub enum TrendDirection {
    Improving,
    Stable,
    Deteriorating,
}

/// Portfolio risk metrics
#[derive(Debug, Clone)]
pub struct PortfolioRiskMetrics {
    pub value_at_risk: Decimal,
    pub expected_loss: Decimal,
    pub stress_test_results: StressTestResults,
    pub concentration_risk: ConcentrationRisk,
    pub credit_migration: CreditMigration,
}

/// Stress test scenario results
#[derive(Debug, Clone)]
pub struct StressTestResults {
    pub base_case_loss: Decimal,
    pub adverse_scenario_loss: Decimal,
    pub severely_adverse_loss: Decimal,
    pub survival_probability: Decimal,
}

/// Concentration risk analysis
#[derive(Debug, Clone)]
pub struct ConcentrationRisk {
    pub top_10_borrowers_exposure: Decimal,
    pub geographic_concentration: Vec<(String, Decimal)>,
    pub sector_concentration: Vec<(String, Decimal)>,
    pub herfindahl_index: Decimal,
}

/// Credit migration patterns
#[derive(Debug, Clone)]
pub struct CreditMigration {
    pub upgrade_rate: Decimal,
    pub downgrade_rate: Decimal,
    pub default_rate: Decimal,
    pub recovery_rate: Decimal,
}

/// Collection effectiveness metrics
#[derive(Debug, Clone)]
pub struct CollectionEffectivenessReport {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_collections: Decimal,
    pub collection_target: Decimal,
    pub effectiveness_ratio: Decimal,
    pub collection_by_stage: Vec<(DelinquencyStage, Decimal)>,
    pub collection_by_action: Vec<(CollectionActionType, Decimal)>,
    pub cost_of_collection: Decimal,
    pub return_on_collection_investment: Decimal,
}

/// Loan account status for lifecycle management
#[derive(Debug, Clone)]
pub enum LoanAccountStatus {
    Active,
    Current,
    Overdue,
    Restructured,
    NonPerforming,
    WriteOff,
    Closed,
    Settled,
}

/// Early settlement calculation
#[derive(Debug, Clone)]
pub struct EarlySettlementCalculation {
    pub loan_account_id: Uuid,
    pub settlement_date: NaiveDate,
    pub outstanding_principal: Decimal,
    pub accrued_interest: Decimal,
    pub penalty_amount: Decimal,
    pub early_settlement_discount: Option<Decimal>,
    pub total_settlement_amount: Decimal,
    pub savings_to_customer: Decimal,
}

/// Loan write-off record
#[derive(Debug, Clone)]
pub struct LoanWriteOff {
    pub write_off_id: Uuid,
    pub loan_account_id: Uuid,
    pub write_off_date: NaiveDate,
    pub written_off_amount: Decimal,
    pub write_off_reason: String,
    pub recovery_prospects: String,
    pub authorized_by: String,
    pub regulatory_reported: bool,
    pub created_at: DateTime<Utc>,
}
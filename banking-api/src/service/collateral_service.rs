use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::domain::{
    Collateral, CollateralAlert, CollateralEnforcement, CollateralPledge, CollateralPortfolioSummary,
    CollateralValuation, ConcentrationAnalysis, RiskDistribution, ValuationStatusSummary,
    ComplianceSummary, CovenantCompliance, AlertSeverity, EnforcementMethod, CollateralType, 
    CollateralRiskRating,
};

/// Service for managing collateral assets including pledges, valuations, monitoring, and enforcement
#[async_trait]
pub trait CollateralService: Send + Sync {
    // === CORE COLLATERAL MANAGEMENT ===
    
    /// Create a new collateral asset
    async fn create_collateral(&self, collateral: Collateral) -> Result<Uuid, String>;
    
    /// Retrieve a collateral by ID
    async fn get_collateral(&self, collateral_id: Uuid) -> Result<Option<Collateral>, String>;
    
    /// Update an existing collateral
    async fn update_collateral(&self, collateral: Collateral) -> Result<(), String>;
    
    /// Delete a collateral (soft delete - mark as released)
    async fn release_collateral(&self, collateral_id: Uuid, released_by: Uuid) -> Result<(), String>;
    
    /// Get all collaterals for a specific customer
    async fn get_collaterals_by_customer(&self, customer_id: Uuid) -> Result<Vec<Collateral>, String>;
    
    /// Search collaterals by type, status, or other criteria
    async fn search_collaterals(
        &self,
        collateral_type: Option<CollateralType>,
        risk_rating: Option<CollateralRiskRating>,
        limit: Option<u32>,
        offset: Option<u32>
    ) -> Result<Vec<Collateral>, String>;

    // === VALUATION MANAGEMENT ===
    
    /// Create a new collateral valuation
    async fn create_valuation(&self, valuation: CollateralValuation) -> Result<Uuid, String>;
    
    /// Get all valuations for a collateral
    async fn get_valuations_by_collateral(&self, collateral_id: Uuid) -> Result<Vec<CollateralValuation>, String>;
    
    /// Get the latest valuation for a collateral
    async fn get_latest_valuation(&self, collateral_id: Uuid) -> Result<Option<CollateralValuation>, String>;
    
    /// Find collaterals with valuations due by a specific date
    async fn get_valuations_due(&self, reference_date: NaiveDate) -> Result<Vec<Collateral>, String>;
    
    /// Find collaterals with overdue valuations
    async fn get_overdue_valuations(&self, reference_date: NaiveDate) -> Result<Vec<Collateral>, String>;
    
    /// Update collateral market value based on latest valuation
    async fn update_market_value(&self, collateral_id: Uuid, new_value: Decimal, valuation_date: NaiveDate, updated_by: Uuid) -> Result<(), String>;

    // === PLEDGE MANAGEMENT ===
    
    /// Create a new collateral pledge to secure a loan
    async fn create_pledge(&self, pledge: CollateralPledge) -> Result<Uuid, String>;
    
    /// Get all pledges for a collateral
    async fn get_pledges_by_collateral(&self, collateral_id: Uuid) -> Result<Vec<CollateralPledge>, String>;
    
    /// Get all pledges for a loan account
    async fn get_pledges_by_loan(&self, loan_account_id: Uuid) -> Result<Vec<CollateralPledge>, String>;
    
    /// Release a specific pledge
    async fn release_pledge(&self, pledge_id: Uuid, released_by: Uuid) -> Result<(), String>;
    
    /// Partially release a pledge (reduce pledged amount)
    async fn partial_release_pledge(&self, pledge_id: Uuid, release_amount: Decimal, released_by: Uuid) -> Result<(), String>;
    
    /// Substitute one collateral for another in an existing pledge
    async fn substitute_collateral(&self, pledge_id: Uuid, new_collateral_id: Uuid, substituted_by: Uuid) -> Result<(), String>;

    // === RISK AND COMPLIANCE MONITORING ===
    
    /// Calculate loan-to-value ratio for a specific loan account
    async fn calculate_portfolio_ltv(&self, loan_account_id: Uuid) -> Result<Decimal, String>;
    
    /// Calculate loan-to-value ratio for a specific collateral and loan amount
    async fn calculate_collateral_ltv(&self, collateral_id: Uuid, loan_amount: Decimal) -> Result<Decimal, String>;
    
    /// Check covenant compliance for all pledges
    async fn check_covenant_compliance(&self, reference_date: NaiveDate) -> Result<Vec<CovenantCompliance>, String>;
    
    /// Update covenant compliance status for a pledge
    async fn update_covenant_compliance(&self, pledge_id: Uuid, compliance: CovenantCompliance) -> Result<(), String>;
    
    /// Calculate available collateral value for additional pledging
    async fn calculate_available_value(&self, collateral_id: Uuid) -> Result<Decimal, String>;

    // === ALERT AND MONITORING SYSTEM ===
    
    /// Generate collateral alerts based on various criteria
    async fn generate_alerts(&self, reference_date: NaiveDate) -> Result<Vec<CollateralAlert>, String>;
    
    /// Create a manual collateral alert
    async fn create_alert(&self, alert: CollateralAlert) -> Result<Uuid, String>;
    
    /// Get all active alerts for a collateral
    async fn get_alerts_by_collateral(&self, collateral_id: Uuid) -> Result<Vec<CollateralAlert>, String>;
    
    /// Get alerts by severity level
    async fn get_alerts_by_severity(&self, severity: AlertSeverity) -> Result<Vec<CollateralAlert>, String>;
    
    /// Resolve an alert
    async fn resolve_alert(&self, alert_id: Uuid, resolution_notes: String, resolved_by: Uuid) -> Result<(), String>;
    
    /// Dismiss an alert
    async fn dismiss_alert(&self, alert_id: Uuid, dismissed_by: Uuid) -> Result<(), String>;

    // === PORTFOLIO ANALYSIS AND REPORTING ===
    
    /// Get comprehensive portfolio summary
    async fn get_portfolio_summary(&self, portfolio_id: Uuid, as_of_date: NaiveDate) -> Result<CollateralPortfolioSummary, String>;
    
    /// Get concentration analysis by category
    async fn get_concentration_analysis(&self, portfolio_id: Uuid) -> Result<Vec<ConcentrationAnalysis>, String>;
    
    /// Get risk distribution analysis
    async fn get_risk_distribution(&self, portfolio_id: Uuid) -> Result<Vec<RiskDistribution>, String>;
    
    /// Get valuation status summary
    async fn get_valuation_status_summary(&self, portfolio_id: Uuid) -> Result<ValuationStatusSummary, String>;
    
    /// Get compliance summary
    async fn get_compliance_summary(&self, portfolio_id: Uuid) -> Result<ComplianceSummary, String>;
    
    /// Calculate total portfolio exposure
    async fn calculate_total_exposure(&self, portfolio_id: Uuid) -> Result<Decimal, String>;

    // === ENFORCEMENT AND LIQUIDATION ===
    
    /// Initiate collateral enforcement process
    async fn initiate_enforcement(&self, enforcement: CollateralEnforcement) -> Result<Uuid, String>;
    
    /// Get all enforcement actions for a collateral
    async fn get_enforcements_by_collateral(&self, collateral_id: Uuid) -> Result<Vec<CollateralEnforcement>, String>;
    
    /// Update enforcement status
    async fn update_enforcement_status(&self, enforcement_id: Uuid, status: crate::domain::EnforcementStatus, updated_by: Uuid) -> Result<(), String>;
    
    /// Record enforcement completion and recovery amounts
    async fn complete_enforcement(
        &self, 
        enforcement_id: Uuid, 
        recovery_amount: Decimal, 
        enforcement_costs: Decimal, 
        completed_by: Uuid
    ) -> Result<(), String>;
    
    /// Calculate estimated recovery value for enforcement
    async fn estimate_recovery_value(&self, collateral_id: Uuid, enforcement_method: EnforcementMethod) -> Result<Decimal, String>;

    // === BULK OPERATIONS ===
    
    /// Bulk update collateral market values (for end-of-day processing)
    async fn bulk_update_market_values(&self, valuations: Vec<(Uuid, Decimal, NaiveDate)>, updated_by: Uuid) -> Result<u32, String>;
    
    /// Bulk generate alerts for multiple collaterals
    async fn bulk_generate_alerts(&self, collateral_ids: Vec<Uuid>, reference_date: NaiveDate) -> Result<Vec<CollateralAlert>, String>;
    
    /// Process scheduled valuation requirements
    async fn process_valuation_schedule(&self, reference_date: NaiveDate) -> Result<Vec<CollateralAlert>, String>;

    // === ADVANCED ANALYTICS ===
    
    /// Calculate value-at-risk for collateral portfolio
    async fn calculate_portfolio_var(&self, portfolio_id: Uuid, confidence_level: Decimal, time_horizon_days: i32) -> Result<Decimal, String>;
    
    /// Stress test collateral portfolio against market scenarios
    async fn stress_test_portfolio(&self, portfolio_id: Uuid, market_decline_percentage: Decimal) -> Result<Decimal, String>;
    
    /// Get collateral performance metrics over time
    async fn get_performance_metrics(&self, collateral_id: Uuid, from_date: NaiveDate, to_date: NaiveDate) -> Result<Vec<(NaiveDate, Decimal)>, String>;
    
    /// Recommend optimal collateral mix for risk management
    async fn recommend_collateral_optimization(&self, portfolio_id: Uuid) -> Result<Vec<String>, String>;
}
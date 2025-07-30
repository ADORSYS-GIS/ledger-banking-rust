use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::models::{
    CollateralModel, CollateralEnforcementModel
};

/// Repository trait for collateral data persistence operations
#[async_trait]
pub trait CollateralRepository: Send + Sync {
    // === CORE COLLATERAL CRUD ===
    
    /// Save a new collateral or update existing one
    async fn save_collateral(&self, collateral: &CollateralModel) -> Result<(), String>;
    
    /// Find collateral by ID
    async fn find_collateral_by_id(&self, collateral_id: Uuid) -> Result<Option<CollateralModel>, String>;
    
    /// Find all collaterals for a customer
    async fn find_collaterals_by_customer(&self, customer_id: Uuid) -> Result<Vec<CollateralModel>, String>;
    
    /// Find collaterals by type
    async fn find_collaterals_by_type(&self, collateral_type: String) -> Result<Vec<CollateralModel>, String>;
    
    /// Find collaterals by status
    async fn find_collaterals_by_status(&self, status: String) -> Result<Vec<CollateralModel>, String>;
    
    /// Search collaterals with pagination
    async fn search_collaterals(
        &self,
        collateral_type: Option<String>,
        risk_rating: Option<String>,
        status: Option<String>,
        limit: u32,
        offset: u32
    ) -> Result<Vec<CollateralModel>, String>;
    
    /// Count total collaterals matching criteria
    async fn count_collaterals(
        &self,
        collateral_type: Option<String>,
        risk_rating: Option<String>,
        status: Option<String>
    ) -> Result<u64, String>;
    
    /// Update collateral status
    async fn update_collateral_status(&self, collateral_id: Uuid, status: String, updated_by: Uuid) -> Result<(), String>;
    
    /// Update collateral market value
    async fn update_market_value(&self, collateral_id: Uuid, new_value: Decimal, valuation_date: NaiveDate, updated_by: Uuid) -> Result<(), String>;

    // === VALUATION OPERATIONS ===
    
    /// Save a collateral valuation (using JSON data for now)
    async fn save_valuation(&self, collateral_id: Uuid, valuation_data: String) -> Result<(), String>;
    
    /// Find all valuations for a collateral (returns JSON data)
    async fn find_valuations_by_collateral(&self, collateral_id: Uuid) -> Result<Vec<String>, String>;
    
    /// Find the latest valuation for a collateral (returns JSON data)
    async fn find_latest_valuation(&self, collateral_id: Uuid) -> Result<Option<String>, String>;
    
    /// Find collaterals with valuations due by date
    async fn find_valuations_due(&self, reference_date: NaiveDate) -> Result<Vec<CollateralModel>, String>;
    
    /// Find collaterals with overdue valuations
    async fn find_overdue_valuations(&self, reference_date: NaiveDate) -> Result<Vec<CollateralModel>, String>;
    
    /// Find valuations by date range (returns JSON data)
    async fn find_valuations_by_date_range(&self, from_date: NaiveDate, to_date: NaiveDate) -> Result<Vec<String>, String>;

    // === PLEDGE OPERATIONS ===
    
    /// Save a collateral pledge (using JSON data for now)
    async fn save_pledge(&self, collateral_id: Uuid, pledge_data: String) -> Result<(), String>;
    
    /// Find pledge by ID (returns JSON data)
    async fn find_pledge_by_id(&self, pledge_id: Uuid) -> Result<Option<String>, String>;
    
    /// Find all pledges for a collateral (returns JSON data)
    async fn find_pledges_by_collateral(&self, collateral_id: Uuid) -> Result<Vec<String>, String>;
    
    /// Find all pledges for a loan account (returns JSON data)
    async fn find_pledges_by_loan_account(&self, loan_account_id: Uuid) -> Result<Vec<String>, String>;
    
    /// Find active pledges only (returns JSON data)
    async fn find_active_pledges_by_collateral(&self, collateral_id: Uuid) -> Result<Vec<String>, String>;
    
    /// Update pledge status
    async fn update_pledge_status(&self, pledge_id: Uuid, status: String, updated_by: Uuid) -> Result<(), String>;
    
    /// Update pledged amount (for partial releases)
    async fn update_pledged_amount(&self, pledge_id: Uuid, new_amount: Decimal, updated_by: Uuid) -> Result<(), String>;
    
    /// Find pledges by priority level (returns JSON data)
    async fn find_pledges_by_priority(&self, priority: String) -> Result<Vec<String>, String>;

    // === ALERT OPERATIONS ===
    
    /// Save a collateral alert (using JSON data for now)
    async fn save_alert(&self, collateral_id: Uuid, alert_data: String) -> Result<(), String>;
    
    /// Find alert by ID (returns JSON data)
    async fn find_alert_by_id(&self, alert_id: Uuid) -> Result<Option<String>, String>;
    
    /// Find all alerts for a collateral (returns JSON data)
    async fn find_alerts_by_collateral(&self, collateral_id: Uuid) -> Result<Vec<String>, String>;
    
    /// Find active alerts only (returns JSON data)
    async fn find_active_alerts(&self) -> Result<Vec<String>, String>;
    
    /// Find alerts by severity (returns JSON data)
    async fn find_alerts_by_severity(&self, severity: String) -> Result<Vec<String>, String>;
    
    /// Find alerts by type (returns JSON data)
    async fn find_alerts_by_type(&self, alert_type: String) -> Result<Vec<String>, String>;
    
    /// Find alerts assigned to a person (returns JSON data)
    async fn find_alerts_by_assignee(&self, assigned_to: Uuid) -> Result<Vec<String>, String>;
    
    /// Update alert status
    async fn update_alert_status(&self, alert_id: Uuid, status: String, updated_by: Uuid) -> Result<(), String>;
    
    /// Resolve alert with notes
    async fn resolve_alert(&self, alert_id: Uuid, resolution_notes: String, resolved_by: Uuid) -> Result<(), String>;

    // === ENFORCEMENT OPERATIONS ===
    
    /// Save a collateral enforcement action
    async fn save_enforcement(&self, enforcement: &CollateralEnforcementModel) -> Result<(), String>;
    
    /// Find enforcement by ID
    async fn find_enforcement_by_id(&self, enforcement_id: Uuid) -> Result<Option<CollateralEnforcementModel>, String>;
    
    /// Find all enforcement actions for a collateral
    async fn find_enforcements_by_collateral(&self, collateral_id: Uuid) -> Result<Vec<CollateralEnforcementModel>, String>;
    
    /// Find enforcement actions by loan account
    async fn find_enforcements_by_loan_account(&self, loan_account_id: Uuid) -> Result<Vec<CollateralEnforcementModel>, String>;
    
    /// Find enforcement actions by status
    async fn find_enforcements_by_status(&self, status: String) -> Result<Vec<CollateralEnforcementModel>, String>;
    
    /// Update enforcement status
    async fn update_enforcement_status(&self, enforcement_id: Uuid, status: String, updated_by: Uuid) -> Result<(), String>;
    
    /// Complete enforcement with recovery details
    async fn complete_enforcement(
        &self,
        enforcement_id: Uuid,
        recovery_amount: Decimal,
        enforcement_costs: Decimal,
        net_recovery: Decimal,
        completed_by: Uuid
    ) -> Result<(), String>;

    // === PORTFOLIO AND ANALYTICS ===
    
    /// Calculate total portfolio value
    async fn calculate_total_portfolio_value(&self, portfolio_id: Uuid) -> Result<Decimal, String>;
    
    /// Calculate total pledged value for portfolio
    async fn calculate_total_pledged_value(&self, portfolio_id: Uuid) -> Result<Decimal, String>;
    
    /// Calculate weighted average LTV for portfolio
    async fn calculate_weighted_average_ltv(&self, portfolio_id: Uuid) -> Result<Decimal, String>;
    
    /// Get collateral concentration by type
    async fn get_concentration_by_type(&self, portfolio_id: Uuid) -> Result<Vec<(String, u32, Decimal)>, String>;
    
    /// Get collateral concentration by location
    async fn get_concentration_by_location(&self, portfolio_id: Uuid) -> Result<Vec<(String, u32, Decimal)>, String>;
    
    /// Get risk distribution
    async fn get_risk_distribution(&self, portfolio_id: Uuid) -> Result<Vec<(String, u32, Decimal)>, String>;
    
    /// Get valuation status summary
    async fn get_valuation_status_summary(&self, portfolio_id: Uuid) -> Result<(u32, u32, u32, i32), String>;
    
    /// Get compliance summary
    async fn get_compliance_summary(&self, portfolio_id: Uuid) -> Result<(u32, u32, u32, u32), String>;
    
    /// Find collaterals by LTV threshold
    async fn find_collaterals_by_ltv_threshold(&self, min_ltv: Decimal, max_ltv: Option<Decimal>) -> Result<Vec<CollateralModel>, String>;

    // === BATCH OPERATIONS ===
    
    /// Batch update market values
    async fn batch_update_market_values(&self, updates: Vec<(Uuid, Decimal, NaiveDate)>, updated_by: Uuid) -> Result<u32, String>;
    
    /// Batch create alerts (using JSON data)
    async fn batch_create_alerts(&self, alert_data: Vec<String>) -> Result<u32, String>;
    
    /// Batch update pledge statuses
    async fn batch_update_pledge_statuses(&self, updates: Vec<(Uuid, String)>, updated_by: Uuid) -> Result<u32, String>;

    // === REPORTING QUERIES ===
    
    /// Get collaterals by custody location
    async fn find_collaterals_by_custody_location(&self, custody_location: String) -> Result<Vec<CollateralModel>, String>;
    
    /// Get collaterals requiring insurance review
    async fn find_collaterals_requiring_insurance_review(&self, reference_date: NaiveDate) -> Result<Vec<CollateralModel>, String>;
    
    /// Get collaterals with perfection expiring
    async fn find_collaterals_with_expiring_perfection(&self, days_ahead: i32) -> Result<Vec<CollateralModel>, String>;
    
    /// Get collateral performance history
    async fn get_collateral_performance_history(&self, collateral_id: Uuid, from_date: NaiveDate, to_date: NaiveDate) -> Result<Vec<(NaiveDate, Decimal)>, String>;
    
    /// Find collaterals by environmental risk level
    async fn find_collaterals_by_environmental_risk(&self, risk_level: String) -> Result<Vec<CollateralModel>, String>;

    // === COVENANT MONITORING ===
    
    /// Find pledges with covenant breaches (returns JSON data)
    async fn find_covenant_breaches(&self, reference_date: NaiveDate) -> Result<Vec<String>, String>;
    
    /// Update covenant compliance status
    async fn update_covenant_compliance(&self, pledge_id: Uuid, compliance_data: String, updated_by: Uuid) -> Result<(), String>;
    
    /// Find pledges requiring covenant review (returns JSON data)
    async fn find_pledges_requiring_covenant_review(&self, reference_date: NaiveDate) -> Result<Vec<String>, String>;

    // === AUDIT AND HISTORY ===
    
    /// Get collateral audit trail
    async fn get_collateral_audit_trail(&self, collateral_id: Uuid) -> Result<Vec<String>, String>;
    
    /// Get pledge audit trail
    async fn get_pledge_audit_trail(&self, pledge_id: Uuid) -> Result<Vec<String>, String>;
    
    /// Get valuation history for collateral (returns JSON data)
    async fn get_valuation_history(&self, collateral_id: Uuid) -> Result<Vec<String>, String>;

    // === CLEANUP AND MAINTENANCE ===
    
    /// Archive old alerts (move to historical table)
    async fn archive_old_alerts(&self, cutoff_date: NaiveDate) -> Result<u32, String>;
    
    /// Archive completed enforcement actions
    async fn archive_completed_enforcements(&self, cutoff_date: NaiveDate) -> Result<u32, String>;
    
    /// Clean up temporary valuation records
    async fn cleanup_temporary_valuations(&self, cutoff_date: NaiveDate) -> Result<u32, String>;
}
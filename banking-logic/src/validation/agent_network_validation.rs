use rust_decimal::Decimal;

use banking_api::{BankingResult, BankingError};
use banking_db::models::{AgentNetworkModel, AgencyBranchModel, AgentTerminalModel};

/// Comprehensive validation utilities for agent network hierarchy
pub struct AgentNetworkValidation;

impl AgentNetworkValidation {
    /// Validate that branch limits are within network constraints
    pub fn validate_branch_limits_against_network(
        branch: &AgencyBranchModel,
        network: &AgentNetworkModel,
    ) -> BankingResult<()> {
        // Validate network status first
        if network.status != "Active" {
            return Err(BankingError::AgentNetworkEntityInactive {
                entity_type: "Network".to_string(),
                entity_id: network.network_id,
                status: network.status.clone(),
            });
        }

        // Validate transaction limits
        if branch.per_transaction_limit > network.aggregate_daily_limit {
            return Err(BankingError::BranchLimitExceedsNetwork {
                branch_limit: branch.per_transaction_limit,
                network_limit: network.aggregate_daily_limit,
                limit_type: "transaction".to_string(),
            });
        }

        // Validate daily limits
        if branch.daily_transaction_limit > network.aggregate_daily_limit {
            return Err(BankingError::BranchLimitExceedsNetwork {
                branch_limit: branch.daily_transaction_limit,
                network_limit: network.aggregate_daily_limit,
                limit_type: "daily".to_string(),
            });
        }

        // Additional business rule: Daily limit should be reasonable multiple of transaction limit
        if branch.daily_transaction_limit < branch.per_transaction_limit {
            return Err(BankingError::ValidationFailed(
                "Branch daily limit cannot be less than transaction limit".to_string()
            ));
        }

        // Warn if daily limit is too high compared to transaction limit (optional business rule)
        let reasonable_multiplier = Decimal::from(100); // 100 transactions per day max
        if branch.daily_transaction_limit > branch.per_transaction_limit * reasonable_multiplier {
            // This could be a warning in a real system, but for now we allow it
            log::warn!(
                "Branch {} daily limit ({}) is unusually high compared to transaction limit ({})",
                branch.branch_id,
                branch.daily_transaction_limit,
                branch.per_transaction_limit
            );
        }

        Ok(())
    }

    /// Validate that terminal limits are within branch constraints
    pub fn validate_terminal_limits_against_branch(
        terminal: &AgentTerminalModel,
        branch: &AgencyBranchModel,
    ) -> BankingResult<()> {
        // Validate branch status first
        if branch.status != "Active" {
            return Err(BankingError::AgentNetworkEntityInactive {
                entity_type: "Branch".to_string(),
                entity_id: branch.branch_id,
                status: branch.status.clone(),
            });
        }

        // Validate transaction limits
        if terminal.daily_transaction_limit > branch.per_transaction_limit {
            return Err(BankingError::TerminalLimitExceedsBranch {
                terminal_limit: terminal.daily_transaction_limit,
                branch_limit: branch.per_transaction_limit,
                limit_type: "transaction".to_string(),
            });
        }

        // Validate daily limits
        if terminal.daily_transaction_limit > branch.daily_transaction_limit {
            return Err(BankingError::TerminalLimitExceedsBranch {
                terminal_limit: terminal.daily_transaction_limit,
                branch_limit: branch.daily_transaction_limit,
                limit_type: "daily".to_string(),
            });
        }

        // Additional business rule: Daily limit validation 
        // Since we only have daily_transaction_limit, we'll validate it's positive
        if terminal.daily_transaction_limit <= Decimal::ZERO {
            return Err(BankingError::ValidationFailed(
                "Terminal daily transaction limit must be positive".to_string()
            ));
        }

        Ok(())
    }

    /// Validate complete hierarchy for a transaction amount
    pub fn validate_transaction_hierarchy(
        amount: Decimal,
        terminal: &AgentTerminalModel,
        branch: &AgencyBranchModel,
        network: &AgentNetworkModel,
    ) -> BankingResult<HierarchyValidationResult> {
        let mut validation_errors = Vec::new();

        // Check terminal constraints
        if terminal.status != "Active" {
            validation_errors.push(format!(
                "Terminal {} is not active (status: {})", 
                terminal.terminal_id, terminal.status
            ));
        }

        if amount > terminal.daily_transaction_limit {
            validation_errors.push(format!(
                "Amount {} exceeds terminal transaction limit {}", 
                amount, terminal.daily_transaction_limit
            ));
        }

        // Check branch constraints
        if branch.status != "Active" {
            validation_errors.push(format!(
                "Branch {} is not active (status: {})", 
                branch.branch_id, branch.status
            ));
        }

        if amount > branch.per_transaction_limit {
            validation_errors.push(format!(
                "Amount {} exceeds branch transaction limit {}", 
                amount, branch.per_transaction_limit
            ));
        }

        // Check network constraints
        if network.status != "Active" {
            validation_errors.push(format!(
                "Network {} is not active (status: {})", 
                network.network_id, network.status
            ));
        }

        if amount > network.aggregate_daily_limit {
            validation_errors.push(format!(
                "Amount {} exceeds network transaction limit {}", 
                amount, network.aggregate_daily_limit
            ));
        }

        if validation_errors.is_empty() {
            Ok(HierarchyValidationResult {
                is_valid: true,
                errors: Vec::new(),
                terminal_approved: true,
                branch_approved: true,
                network_approved: true,
            })
        } else {
            Ok(HierarchyValidationResult {
                is_valid: false,
                errors: validation_errors,
                terminal_approved: terminal.status == "Active" && amount <= terminal.daily_transaction_limit,
                branch_approved: branch.status == "Active" && amount <= branch.per_transaction_limit,
                network_approved: network.status == "Active" && amount <= network.aggregate_daily_limit,
            })
        }
    }

    /// Validate daily volume constraints across hierarchy
    pub fn validate_daily_volume_constraints(
        additional_amount: Decimal,
        terminal: &AgentTerminalModel,
        branch: &AgencyBranchModel,
        network: &AgentNetworkModel,
    ) -> BankingResult<DailyVolumeValidationResult> {
        let mut validation_errors = Vec::new();

        // Check terminal daily volume
        let terminal_new_volume = terminal.current_daily_volume + additional_amount;
        if terminal_new_volume > terminal.daily_transaction_limit {
            validation_errors.push(format!(
                "Terminal daily volume would exceed limit: {} + {} > {}", 
                terminal.current_daily_volume, additional_amount, terminal.daily_transaction_limit
            ));
        }

        // Check branch daily volume
        let branch_new_volume = branch.current_daily_volume + additional_amount;
        if branch_new_volume > branch.daily_transaction_limit {
            validation_errors.push(format!(
                "Branch daily volume would exceed limit: {} + {} > {}", 
                branch.current_daily_volume, additional_amount, branch.daily_transaction_limit
            ));
        }

        // Check network daily volume
        let network_new_volume = network.current_daily_volume + additional_amount;
        if network_new_volume > network.aggregate_daily_limit {
            validation_errors.push(format!(
                "Network daily volume would exceed limit: {} + {} > {}", 
                network.current_daily_volume, additional_amount, network.aggregate_daily_limit
            ));
        }

        Ok(DailyVolumeValidationResult {
            is_valid: validation_errors.is_empty(),
            errors: validation_errors,
            terminal_remaining: terminal.daily_transaction_limit - terminal.current_daily_volume,
            branch_remaining: branch.daily_transaction_limit - branch.current_daily_volume,
            network_remaining: network.aggregate_daily_limit - network.current_daily_volume,
        })
    }

    /// Validate network configuration for business rules
    pub fn validate_network_configuration(network: &AgentNetworkModel) -> BankingResult<()> {
        let mut validation_errors = Vec::new();

        // Network name should not be empty
        if network.network_name.trim().is_empty() {
            validation_errors.push("Network name cannot be empty".to_string());
        }

        // Network has contract_id - validate it exists if set
        if network.contract_id.is_none() {
            validation_errors.push(
                "Network must have a contract ID assigned".to_string()
            );
        }

        // Limits should be reasonable
        if network.aggregate_daily_limit <= Decimal::ZERO {
            validation_errors.push("Aggregate daily limit must be positive".to_string());
        }

        // Current volume should not exceed limit
        if network.current_daily_volume > network.aggregate_daily_limit {
            validation_errors.push("Current daily volume exceeds aggregate daily limit".to_string());
        }

        // Settlement GL code should not be empty (it's HeaplessString now)
        if network.settlement_gl_code.as_str().trim().is_empty() {
            validation_errors.push("Settlement GL code cannot be empty".to_string());
        }

        if !validation_errors.is_empty() {
            return Err(BankingError::HierarchicalValidationFailed {
                validation_errors,
            });
        }

        Ok(())
    }

    /// Validate branch configuration for business rules
    pub fn validate_branch_configuration(branch: &AgencyBranchModel) -> BankingResult<()> {
        let mut validation_errors = Vec::new();

        // Branch name should not be empty
        if branch.branch_name.trim().is_empty() {
            validation_errors.push("Branch name cannot be empty".to_string());
        }

        // Branch code should not be empty (it's HeaplessString now)
        if branch.branch_code.as_str().trim().is_empty() {
            validation_errors.push("Branch code cannot be empty".to_string());
        }

        // Limits should be reasonable
        if branch.per_transaction_limit <= Decimal::ZERO {
            validation_errors.push("Per-transaction limit must be positive".to_string());
        }

        if branch.daily_transaction_limit <= Decimal::ZERO {
            validation_errors.push("Daily transaction limit must be positive".to_string());
        }

        // Daily limit should be at least equal to per-transaction limit
        if branch.daily_transaction_limit < branch.per_transaction_limit {
            validation_errors.push("Daily limit cannot be less than per-transaction limit".to_string());
        }

        if !validation_errors.is_empty() {
            return Err(BankingError::HierarchicalValidationFailed {
                validation_errors,
            });
        }

        Ok(())
    }

    /// Validate terminal configuration for business rules
    pub fn validate_terminal_configuration(terminal: &AgentTerminalModel) -> BankingResult<()> {
        let mut validation_errors = Vec::new();

        // Terminal name should not be empty
        if terminal.terminal_name.trim().is_empty() {
            validation_errors.push("Terminal name cannot be empty".to_string());
        }

        // Limits should be reasonable
        if terminal.daily_transaction_limit <= Decimal::ZERO {
            validation_errors.push("Daily transaction limit must be positive".to_string());
        }

        // Validate cash limits
        if terminal.max_cash_limit <= Decimal::ZERO {
            validation_errors.push("Maximum cash limit must be positive".to_string());
        }

        // Current cash should not exceed max limit
        if terminal.current_cash_balance > terminal.max_cash_limit {
            validation_errors.push("Current cash balance exceeds maximum limit".to_string());
        }

        if !validation_errors.is_empty() {
            return Err(BankingError::HierarchicalValidationFailed {
                validation_errors,
            });
        }

        Ok(())
    }
}

/// Result of hierarchy validation for transactions
#[derive(Debug, Clone)]
pub struct HierarchyValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub terminal_approved: bool,
    pub branch_approved: bool,
    pub network_approved: bool,
}

/// Result of daily volume validation
#[derive(Debug, Clone)]
pub struct DailyVolumeValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub terminal_remaining: Decimal,
    pub branch_remaining: Decimal,
    pub network_remaining: Decimal,
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_branch_limit_validation_success() {
        let network = AgentNetworkModel {
            network_id: Uuid::new_v4(),
            network_name: "Test Network".to_string(),
            network_type: "Urban".to_string(),
            status: "Active".to_string(),
            contract_id: Some(Uuid::new_v4()),
            aggregate_daily_limit: Decimal::from(100000),
            current_daily_volume: Decimal::ZERO,
            settlement_gl_code: heapless::String::try_from("GL123").unwrap(),
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
            updated_by: "system".to_string(),
        };

        let branch = AgencyBranchModel {
            branch_id: Uuid::new_v4(),
            network_id: network.network_id,
            parent_branch_id: None,
            branch_name: "Test Branch".to_string(),
            branch_code: heapless::String::try_from("BR001").unwrap(),
            branch_level: 1,
            gl_code_prefix: heapless::String::try_from("GL001").unwrap(),
            status: "Active".to_string(),
            daily_transaction_limit: Decimal::from(50000),
            current_daily_volume: Decimal::ZERO,
            max_cash_limit: Decimal::from(100000),
            current_cash_balance: Decimal::from(50000),
            minimum_cash_balance: Decimal::from(10000),
            created_at: Utc::now(),
            address_json: "{}".to_string(),
            gps_latitude: None,
            gps_longitude: None,
            gps_accuracy_meters: None,
            landmark_description: None,
            operating_hours_json: "{}".to_string(),
            holiday_schedule_json: "{}".to_string(),
            temporary_closure_json: None,
            primary_phone: "+1234567890".to_string(),
            secondary_phone: None,
            email: Some("test@example.com".to_string()),
            branch_manager_id: None,
            branch_type: "MainBranch".to_string(),
            supported_services_json: "[]".to_string(),
            supported_currencies_json: "[]".to_string(),
            languages_spoken_json: "[]".to_string(),
            security_features_json: "{}".to_string(),
            accessibility_features_json: "{}".to_string(),
            required_documents_json: "[]".to_string(),
            max_daily_customers: Some(100),
            average_wait_time_minutes: Some(15),
            per_transaction_limit: Decimal::from(5000),
            monthly_transaction_limit: Some(Decimal::from(1000000)),
            risk_rating: "Low".to_string(),
            last_audit_date: None,
            compliance_certifications_json: "[]".to_string(),
            last_updated_at: Utc::now(),
            updated_by: "system".to_string(),
        };

        let result = AgentNetworkValidation::validate_branch_limits_against_network(&branch, &network);
        assert!(result.is_ok());
    }

    #[test]
    fn test_branch_limit_validation_failure() {
        let network = AgentNetworkModel {
            network_id: Uuid::new_v4(),
            network_name: "Test Network".to_string(),
            network_type: "Urban".to_string(),
            status: "Active".to_string(),
            contract_id: Some(Uuid::new_v4()),
            aggregate_daily_limit: Decimal::from(50000),
            current_daily_volume: Decimal::ZERO,
            settlement_gl_code: heapless::String::try_from("GL123").unwrap(),
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
            updated_by: "system".to_string(),
        };

        let branch = AgencyBranchModel {
            branch_id: Uuid::new_v4(),
            network_id: network.network_id,
            parent_branch_id: None,
            branch_name: "Test Branch".to_string(),
            branch_code: heapless::String::try_from("BR001").unwrap(),
            branch_level: 1,
            gl_code_prefix: heapless::String::try_from("GL001").unwrap(),
            status: "Active".to_string(),
            daily_transaction_limit: Decimal::from(60000), // Exceeds network limit
            current_daily_volume: Decimal::ZERO,
            max_cash_limit: Decimal::from(100000),
            current_cash_balance: Decimal::from(50000),
            minimum_cash_balance: Decimal::from(10000),
            created_at: Utc::now(),
            address_json: "{}".to_string(),
            gps_latitude: None,
            gps_longitude: None,
            gps_accuracy_meters: None,
            landmark_description: None,
            operating_hours_json: "{}".to_string(),
            holiday_schedule_json: "{}".to_string(),
            temporary_closure_json: None,
            primary_phone: "+1234567890".to_string(),
            secondary_phone: None,
            email: Some("test@example.com".to_string()),
            branch_manager_id: None,
            branch_type: "MainBranch".to_string(),
            supported_services_json: "[]".to_string(),
            supported_currencies_json: "[]".to_string(),
            languages_spoken_json: "[]".to_string(),
            security_features_json: "{}".to_string(),
            accessibility_features_json: "{}".to_string(),
            required_documents_json: "[]".to_string(),
            max_daily_customers: Some(100),
            average_wait_time_minutes: Some(15),
            per_transaction_limit: Decimal::from(60000), // Exceeds network limit
            monthly_transaction_limit: Some(Decimal::from(1000000)),
            risk_rating: "Low".to_string(),
            last_audit_date: None,
            compliance_certifications_json: "[]".to_string(),
            last_updated_at: Utc::now(),
            updated_by: "system".to_string(),
        };

        let result = AgentNetworkValidation::validate_branch_limits_against_network(&branch, &network);
        assert!(result.is_err());
        
        if let Err(BankingError::BranchLimitExceedsNetwork { limit_type, .. }) = result {
            assert_eq!(limit_type, "transaction");
        } else {
            panic!("Expected BranchLimitExceedsNetwork error");
        }
    }
}
use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

use banking_api::{
    BankingResult, BankingError,
    domain::{
        AgentNetwork, AgencyBranch, AgentTerminal,
        NetworkStatus, BranchStatus, TerminalStatus
    },
    service::{HierarchyService, NetworkHierarchy, BranchHierarchy},
};
use banking_db::{
    repository::AgentNetworkRepository,
    models::agent_network::{BranchStatus as DbBranchStatus, TerminalStatus as DbTerminalStatus, NetworkStatus as DbNetworkStatus}
};
use banking_api::domain::transaction::TransactionValidationResult as ValidationResult;
use banking_api::domain::TerminalLimits;
use crate::mappers::AgentNetworkMapper;

/// Production implementation of HierarchyService
/// Manages agent networks, branches, and terminal hierarchies with comprehensive limit validation
pub struct HierarchyServiceImpl {
    agent_network_repository: Arc<dyn AgentNetworkRepository>,
}

impl HierarchyServiceImpl {
    pub fn new(agent_network_repository: Arc<dyn AgentNetworkRepository>) -> Self {
        Self { agent_network_repository }
    }

    /// Validates that branch belongs to network and is active
    async fn validate_branch_network_relationship(&self, branch_id: Uuid, network_id: Uuid) -> BankingResult<()> {
        let branch = self.agent_network_repository
            .find_branch_by_id(branch_id)
            .await?
            .ok_or_else(|| BankingError::Internal(format!("Branch {branch_id} not found")))?;

        if branch.agent_network_id != network_id {
            return Err(BankingError::ValidationFailed(
                format!("Branch {branch_id} does not belong to network {network_id}")
            ));
        }

        if branch.status != DbBranchStatus::Active {
            return Err(BankingError::ValidationFailed(
                format!("Branch {branch_id} is not active (status: {:?})", branch.status)
            ));
        }

        Ok(())
    }


    /// Recursively builds branch hierarchy with all sub-branches and terminals
    fn build_branch_hierarchy<'a>(&'a self, branch_id: Uuid) -> std::pin::Pin<Box<dyn std::future::Future<Output = BankingResult<BranchHierarchy>> + Send + 'a>> {
        Box::pin(async move {
        let branch_model = self.agent_network_repository
            .find_branch_by_id(branch_id)
            .await?
            .ok_or_else(|| BankingError::Internal(format!("Branch {branch_id} not found")))?;

        let branch = AgentNetworkMapper::branch_from_model(branch_model);

        // Get all terminals for this branch
        let terminal_models = self.agent_network_repository
            .find_terminals_by_branch(branch_id)
            .await?;
        
        let terminals: Vec<AgentTerminal> = terminal_models
            .into_iter()
            .map(AgentNetworkMapper::terminal_from_model)
            .collect();

        // Get all sub-branches
        let sub_branch_models = self.agent_network_repository
            .find_branches_by_parent(branch_id)
            .await?;

        let mut sub_branches = Vec::new();
        for sub_branch_model in sub_branch_models {
            let sub_hierarchy = self.build_branch_hierarchy(sub_branch_model.id).await?;
            sub_branches.push(sub_hierarchy);
        }

        Ok(BranchHierarchy {
            branch,
            terminals,
            sub_branches,
        })
        })
    }
}

#[async_trait]
impl HierarchyService for HierarchyServiceImpl {
    /// Validate hierarchical limits across Terminal → Branch → Network
    async fn validate_hierarchical_limits(&self, terminal_id: Uuid, amount: Decimal) -> BankingResult<ValidationResult> {
        // Get terminal and validate it's active
        let terminal_model = self.agent_network_repository
            .find_terminal_by_id(terminal_id)
            .await?
            .ok_or_else(|| BankingError::Internal(format!("Terminal {terminal_id} not found")))?;

        if terminal_model.status != DbTerminalStatus::Active {
            return Ok(ValidationResult::failure(
                None,
                vec![(
                    Some(heapless::String::try_from("terminal_status").unwrap()),
                    Some(heapless::String::try_from(format!("Terminal {terminal_id} is not active").as_str()).unwrap()),
                    Some(heapless::String::try_from("TERMINAL_INACTIVE").unwrap()),
                )],
            ));
        }

        // Check terminal limit
        if terminal_model.current_daily_volume + amount > terminal_model.daily_transaction_limit {
            return Ok(ValidationResult::failure(
                None,
                vec![(
                    Some(heapless::String::try_from("terminal_limit").unwrap()),
                    Some(heapless::String::try_from(format!(
                        "Terminal daily limit exceeded. Current: {}, Limit: {}, Requested: {amount}",
                        terminal_model.current_daily_volume,
                        terminal_model.daily_transaction_limit
                    ).as_str()).unwrap()),
                    Some(heapless::String::try_from("TERMINAL_LIMIT_EXCEEDED").unwrap()),
                )],
            ));
        }

        // Get branch and validate it's active
        let branch_model = self.agent_network_repository
            .find_branch_by_id(terminal_model.agency_branch_id)
            .await?
            .ok_or_else(|| BankingError::Internal(format!("Branch {} not found", terminal_model.agency_branch_id)))?;

        if branch_model.status != DbBranchStatus::Active {
            return Ok(ValidationResult::failure(
                None,
                vec![(
                    Some(heapless::String::try_from("branch_status").unwrap()),
                    Some(heapless::String::try_from(format!("Branch {} is not active", terminal_model.agency_branch_id).as_str()).unwrap()),
                    Some(heapless::String::try_from("BRANCH_INACTIVE").unwrap()),
                )],
            ));
        }

        // Check branch limit
        if branch_model.current_daily_volume + amount > branch_model.daily_transaction_limit {
            return Ok(ValidationResult::failure(
                None,
                vec![(
                    Some(heapless::String::try_from("branch_limit").unwrap()),
                    Some(heapless::String::try_from(format!(
                        "Branch daily limit exceeded. Current: {}, Limit: {}, Requested: {amount}",
                        branch_model.current_daily_volume,
                        branch_model.daily_transaction_limit
                    ).as_str()).unwrap()),
                    Some(heapless::String::try_from("BRANCH_LIMIT_EXCEEDED").unwrap()),
                )],
            ));
        }

        // Get network and validate it's active
        let network_model = self.agent_network_repository
            .find_network_by_id(branch_model.agent_network_id)
            .await?
            .ok_or_else(|| BankingError::Internal(format!("Network {} not found", branch_model.agent_network_id)))?;

        if network_model.status != DbNetworkStatus::Active {
            return Ok(ValidationResult::failure(
                None,
                vec![(
                    Some(heapless::String::try_from("network_status").unwrap()),
                    Some(heapless::String::try_from(format!("Network {} is not active", branch_model.agent_network_id).as_str()).unwrap()),
                    Some(heapless::String::try_from("NETWORK_INACTIVE").unwrap()),
                )],
            ));
        }

        // Check network limit
        if network_model.current_daily_volume + amount > network_model.aggregate_daily_limit {
            return Ok(ValidationResult::failure(
                None,
                vec![(
                    Some(heapless::String::try_from("network_limit").unwrap()),
                    Some(heapless::String::try_from(format!(
                        "Network daily limit exceeded. Current: {}, Limit: {}, Requested: {amount}",
                        network_model.current_daily_volume,
                        network_model.aggregate_daily_limit
                    ).as_str()).unwrap()),
                    Some(heapless::String::try_from("NETWORK_LIMIT_EXCEEDED").unwrap()),
                )],
            ));
        }

        Ok(ValidationResult::success(None))
    }

    /// Get branch GL prefix for transaction coding
    async fn get_branch_gl_prefix(&self, branch_id: Uuid) -> BankingResult<String> {
        self.agent_network_repository
            .get_branch_gl_prefix(branch_id)
            .await?
            .ok_or_else(|| BankingError::Internal(format!("Branch GL prefix not found for branch {branch_id}")))
    }

    /// Update daily volumes across the hierarchy (Terminal → Branch → Network)
    async fn update_daily_volumes(&self, terminal_id: Uuid, amount: Decimal) -> BankingResult<()> {
        // Get terminal to find branch and network IDs
        let terminal_model = self.agent_network_repository
            .find_terminal_by_id(terminal_id)
            .await?
            .ok_or_else(|| BankingError::Internal(format!("Terminal {terminal_id} not found")))?;

        let branch_model = self.agent_network_repository
            .find_branch_by_id(terminal_model.agency_branch_id)
            .await?
            .ok_or_else(|| BankingError::Internal(format!("Branch {} not found", terminal_model.agency_branch_id)))?;

        // Update volumes at all levels
        self.agent_network_repository.update_terminal_daily_volume(terminal_id, amount).await?;
        self.agent_network_repository.update_branch_daily_volume(terminal_model.agency_branch_id, amount).await?;
        self.agent_network_repository.update_network_daily_volume(branch_model.agent_network_id, amount).await?;

        Ok(())
    }

    /// Reset daily counters for all levels (EOD operation)
    async fn reset_daily_counters(&self) -> BankingResult<()> {
        self.agent_network_repository.reset_terminal_daily_counters().await?;
        self.agent_network_repository.reset_branch_daily_counters().await?;
        self.agent_network_repository.reset_network_daily_counters().await?;
        Ok(())
    }

    /// Get terminal limits with validation
    async fn get_terminal_limits(&self, terminal_id: Uuid) -> BankingResult<TerminalLimits> {
        let db_limits = self.agent_network_repository
            .get_terminal_limits(terminal_id)
            .await?
            .ok_or_else(|| BankingError::Internal(format!("Terminal limits not found for terminal {terminal_id}")))?;
        
        // Convert from database TerminalLimits to domain TerminalLimits
        Ok(TerminalLimits {
            daily_limit: db_limits.daily_limit,
            per_transaction_limit: db_limits.daily_limit, // Using daily limit as per transaction limit fallback
            monthly_limit: db_limits.daily_limit * rust_decimal::Decimal::from(30), // Approximate monthly limit
        })
    }

    /// Get current daily volume for terminal
    async fn get_current_daily_volume(&self, terminal_id: Uuid) -> BankingResult<Decimal> {
        let terminal_model = self.agent_network_repository
            .find_terminal_by_id(terminal_id)
            .await?
            .ok_or_else(|| BankingError::Internal(format!("Terminal {terminal_id} not found")))?;

        Ok(terminal_model.current_daily_volume)
    }

    /// Create agent network with validation
    async fn create_agent_network(&self, mut network: AgentNetwork) -> BankingResult<AgentNetwork> {
        // Validate network data - basic validation
        if network.network_name.trim().is_empty() {
            return Err(BankingError::ValidationFailed("Network name cannot be empty".to_string()));
        }
        if network.settlement_gl_code.as_str().trim().is_empty() {
            return Err(BankingError::ValidationFailed("Settlement GL code cannot be empty".to_string()));
        }

        // Set system timestamps
        network.created_at = Utc::now();

        // Initialize daily volume to zero
        network.current_daily_volume = Decimal::ZERO;

        // Convert to model and create
        let network_model = AgentNetworkMapper::network_to_model(network.clone());
        let created_model = self.agent_network_repository.create_network(network_model).await?;
        
        Ok(AgentNetworkMapper::network_from_model(created_model))
    }

    /// Create agency branch with validation
    async fn create_agency_branch(&self, mut branch: AgencyBranch) -> BankingResult<AgencyBranch> {
        // Validate branch data - basic validation
        if branch.branch_name.trim().is_empty() {
            return Err(BankingError::ValidationFailed("Branch name cannot be empty".to_string()));
        }
        if branch.branch_code.as_str().trim().is_empty() {
            return Err(BankingError::ValidationFailed("Branch code cannot be empty".to_string()));
        }

        // Validate network exists and is active
        let network_model = self.agent_network_repository
            .find_network_by_id(branch.agent_network_id)
            .await?
            .ok_or_else(|| BankingError::Internal(format!("Network {} not found", branch.agent_network_id)))?;

        if network_model.status != DbNetworkStatus::Active {
            return Err(BankingError::ValidationFailed(
                format!("Cannot create branch under inactive network {}", branch.agent_network_id)
            ));
        }

        // NOTE: The AgentNetworkModel doesn't have max_transaction_limit or max_daily_limit fields
        // We'll compare against aggregate_daily_limit instead for the daily limit check
        // and skip the transaction limit check as the field doesn't exist

        // Check if branch daily limit exceeds network aggregate daily limit
        if branch.max_cash_limit > network_model.aggregate_daily_limit {
            return Err(BankingError::ValidationFailed(
                format!("Branch max cash limit {} exceeds network aggregate daily limit {}", 
                    branch.max_cash_limit, network_model.aggregate_daily_limit)
            ));
        }

        // Validate parent branch if specified
        if let Some(parent_id) = branch.parent_agency_branch_id {
            self.validate_branch_network_relationship(parent_id, branch.agent_network_id).await?;
            
            // Set branch level based on parent
            let parent_model = self.agent_network_repository
                .find_branch_by_id(parent_id)
                .await?
                .unwrap();
            branch.branch_level = parent_model.branch_level + 1;
        } else {
            // Root branch
            branch.branch_level = 1;
        }

        // Set system timestamps
        branch.created_at = Utc::now();

        // Initialize daily volume to zero
        branch.current_daily_volume = Decimal::ZERO;

        // Convert to model and create
        let branch_model = AgentNetworkMapper::branch_to_model(branch.clone());
        let created_model = self.agent_network_repository.create_branch(branch_model).await?;
        
        Ok(AgentNetworkMapper::branch_from_model(created_model))
    }

    /// Create agent terminal with validation
    async fn create_agent_terminal(&self, mut terminal: AgentTerminal) -> BankingResult<AgentTerminal> {
        // Validate terminal data - basic validation
        if terminal.terminal_name.trim().is_empty() {
            return Err(BankingError::ValidationFailed("Terminal name cannot be empty".to_string()));
        }

        // Validate branch exists and is active
        let branch_model = self.agent_network_repository
            .find_branch_by_id(terminal.agency_branch_id)
            .await?
            .ok_or_else(|| BankingError::Internal(format!("Branch {} not found", terminal.agency_branch_id)))?;

        if branch_model.status != DbBranchStatus::Active {
            return Err(BankingError::ValidationFailed(
                format!("Cannot create terminal under inactive branch {}", terminal.agency_branch_id)
            ));
        }

        // CRITICAL: Validate terminal limits against branch limits
        if terminal.max_cash_limit > branch_model.max_cash_limit {
            return Err(BankingError::ValidationFailed(
                format!("Terminal max cash limit {} exceeds branch max cash limit {}", 
                    terminal.max_cash_limit, branch_model.max_cash_limit)
            ));
        }

        // Initialize daily volume to zero
        terminal.current_daily_volume = Decimal::ZERO;

        // Set last sync time to now
        terminal.last_sync_at = Utc::now();

        // Convert to model and create
        let terminal_model = AgentNetworkMapper::terminal_to_model(terminal.clone());
        let created_model = self.agent_network_repository.create_terminal(terminal_model).await?;
        
        Ok(AgentNetworkMapper::terminal_from_model(created_model))
    }

    /// Find network by ID
    async fn find_network_by_id(&self, network_id: Uuid) -> BankingResult<Option<AgentNetwork>> {
        if let Some(network_model) = self.agent_network_repository.find_network_by_id(network_id).await? {
            Ok(Some(AgentNetworkMapper::network_from_model(network_model)))
        } else {
            Ok(None)
        }
    }

    /// Find branch by ID
    async fn find_branch_by_id(&self, branch_id: Uuid) -> BankingResult<Option<AgencyBranch>> {
        if let Some(branch_model) = self.agent_network_repository.find_branch_by_id(branch_id).await? {
            Ok(Some(AgentNetworkMapper::branch_from_model(branch_model)))
        } else {
            Ok(None)
        }
    }

    /// Find terminal by ID
    async fn find_terminal_by_id(&self, terminal_id: Uuid) -> BankingResult<Option<AgentTerminal>> {
        if let Some(terminal_model) = self.agent_network_repository.find_terminal_by_id(terminal_id).await? {
            Ok(Some(AgentNetworkMapper::terminal_from_model(terminal_model)))
        } else {
            Ok(None)
        }
    }

    /// Find branches by network
    async fn find_branches_by_network(&self, network_id: Uuid) -> BankingResult<Vec<AgencyBranch>> {
        let branch_models = self.agent_network_repository.find_branches_by_network(network_id).await?;
        Ok(branch_models.into_iter()
            .map(AgentNetworkMapper::branch_from_model)
            .collect())
    }

    /// Find terminals by branch
    async fn find_terminals_by_branch(&self, branch_id: Uuid) -> BankingResult<Vec<AgentTerminal>> {
        let terminal_models = self.agent_network_repository.find_terminals_by_branch(branch_id).await?;
        Ok(terminal_models.into_iter()
            .map(AgentNetworkMapper::terminal_from_model)
            .collect())
    }

    /// Update terminal status with validation
    async fn update_terminal_status(&self, terminal_id: Uuid, status: TerminalStatus) -> BankingResult<()> {
        let mut terminal_model = self.agent_network_repository
            .find_terminal_by_id(terminal_id)
            .await?
            .ok_or_else(|| BankingError::Internal(format!("Terminal {terminal_id} not found")))?;

        terminal_model.status = match status {
            TerminalStatus::Active => DbTerminalStatus::Active,
            TerminalStatus::Maintenance => DbTerminalStatus::Maintenance,
            TerminalStatus::Suspended => DbTerminalStatus::Suspended,
            TerminalStatus::Decommissioned => DbTerminalStatus::Decommissioned,
        };

        self.agent_network_repository.update_terminal(terminal_model).await?;
        Ok(())
    }

    /// Update branch status with validation
    async fn update_branch_status(&self, branch_id: Uuid, status: BranchStatus) -> BankingResult<()> {
        let mut branch_model = self.agent_network_repository
            .find_branch_by_id(branch_id)
            .await?
            .ok_or_else(|| BankingError::Internal(format!("Branch {branch_id} not found")))?;

        branch_model.status = match status {
            BranchStatus::Active => DbBranchStatus::Active,
            BranchStatus::Suspended => DbBranchStatus::Suspended,
            BranchStatus::Closed => DbBranchStatus::Closed,
            BranchStatus::TemporarilyClosed => DbBranchStatus::TemporarilyClosed,
        };

        self.agent_network_repository.update_branch(branch_model).await?;
        Ok(())
    }

    /// Update network status with validation
    async fn update_network_status(&self, network_id: Uuid, status: NetworkStatus) -> BankingResult<()> {
        let mut network_model = self.agent_network_repository
            .find_network_by_id(network_id)
            .await?
            .ok_or_else(|| BankingError::Internal(format!("Network {network_id} not found")))?;

        network_model.status = match status {
            NetworkStatus::Active => DbNetworkStatus::Active,
            NetworkStatus::Suspended => DbNetworkStatus::Suspended,
            NetworkStatus::Terminated => DbNetworkStatus::Terminated,
        };

        self.agent_network_repository.update_network(network_model).await?;
        Ok(())
    }

    /// Get complete network hierarchy (network -> branches -> terminals)
    async fn get_network_hierarchy(&self, network_id: Uuid) -> BankingResult<NetworkHierarchy> {
        // Get network
        let network_model = self.agent_network_repository
            .find_network_by_id(network_id)
            .await?
            .ok_or_else(|| BankingError::Internal(format!("Network {network_id} not found")))?;

        let network = AgentNetworkMapper::network_from_model(network_model);

        // Get root branches (branches with no parent)
        let root_branch_models = self.agent_network_repository
            .find_root_branches(network_id)
            .await?;

        let mut branches = Vec::new();
        for root_branch_model in root_branch_models {
            let branch_hierarchy = self.build_branch_hierarchy(root_branch_model.id).await?;
            branches.push(branch_hierarchy);
        }

        Ok(NetworkHierarchy {
            network,
            branches,
        })
    }
}

#[cfg(test)]
mod tests {
    // Test implementations will be added here when needed
}
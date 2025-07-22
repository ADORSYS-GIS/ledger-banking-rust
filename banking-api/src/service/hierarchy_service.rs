use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    domain::{ValidationResult, TerminalLimits, AgentNetwork, AgencyBranch, AgentTerminal},
    error::BankingResult,
};

#[async_trait]
pub trait HierarchyService: Send + Sync {
    /// Validate hierarchical limits (Terminal → Branch → Network)
    async fn validate_hierarchical_limits(&self, terminal_id: Uuid, amount: Decimal) -> BankingResult<ValidationResult>;
    
    /// Get branch GL prefix for transaction coding
    async fn get_branch_gl_prefix(&self, branch_id: Uuid) -> BankingResult<String>;
    
    /// Update daily volumes for limit tracking
    async fn update_daily_volumes(&self, terminal_id: Uuid, amount: Decimal) -> BankingResult<()>;
    
    /// Reset daily counters (EOD operation)
    async fn reset_daily_counters(&self) -> BankingResult<()>;

    /// Get terminal limits
    async fn get_terminal_limits(&self, terminal_id: Uuid) -> BankingResult<TerminalLimits>;

    /// Get current daily volume for terminal
    async fn get_current_daily_volume(&self, terminal_id: Uuid) -> BankingResult<Decimal>;

    /// Create agent network
    async fn create_agent_network(&self, network: AgentNetwork) -> BankingResult<AgentNetwork>;

    /// Create agency branch
    async fn create_agency_branch(&self, branch: AgencyBranch) -> BankingResult<AgencyBranch>;

    /// Create agent terminal
    async fn create_agent_terminal(&self, terminal: AgentTerminal) -> BankingResult<AgentTerminal>;

    /// Find network by ID
    async fn find_network_by_id(&self, network_id: Uuid) -> BankingResult<Option<AgentNetwork>>;

    /// Find branch by ID
    async fn find_branch_by_id(&self, branch_id: Uuid) -> BankingResult<Option<AgencyBranch>>;

    /// Find terminal by ID
    async fn find_terminal_by_id(&self, terminal_id: Uuid) -> BankingResult<Option<AgentTerminal>>;

    /// Find branches by network
    async fn find_branches_by_network(&self, network_id: Uuid) -> BankingResult<Vec<AgencyBranch>>;

    /// Find terminals by branch
    async fn find_terminals_by_branch(&self, branch_id: Uuid) -> BankingResult<Vec<AgentTerminal>>;

    /// Update terminal status
    async fn update_terminal_status(&self, terminal_id: Uuid, status: crate::domain::TerminalStatus) -> BankingResult<()>;

    /// Update branch status
    async fn update_branch_status(&self, branch_id: Uuid, status: crate::domain::BranchStatus) -> BankingResult<()>;

    /// Update network status
    async fn update_network_status(&self, network_id: Uuid, status: crate::domain::NetworkStatus) -> BankingResult<()>;

    /// Get network hierarchy (network -> branches -> terminals)
    async fn get_network_hierarchy(&self, network_id: Uuid) -> BankingResult<NetworkHierarchy>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NetworkHierarchy {
    pub network: AgentNetwork,
    pub branches: Vec<BranchHierarchy>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BranchHierarchy {
    pub branch: AgencyBranch,
    pub terminals: Vec<AgentTerminal>,
    pub sub_branches: Vec<BranchHierarchy>,
}
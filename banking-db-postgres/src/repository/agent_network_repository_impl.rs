use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, NaiveDate};

use banking_api::BankingResult;
use banking_db::models::{AgentNetworkModel, AgencyBranchModel, AgentTerminalModel, CashLimitCheckModel};
// Remove unused simple models import
use banking_db::repository::{AgentNetworkRepository, TerminalLimits, BranchLimits, NetworkLimits, 
    LimitValidationResult, NetworkPerformanceReport, BranchPerformanceReport, TerminalPerformanceReport,
    CashLimitValidationResult, CashStatus, CashAlert};

pub struct AgentNetworkRepositoryImpl {
    pool: PgPool,
}

impl AgentNetworkRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Validate that branch limits don't exceed network limits
    async fn validate_branch_limits_against_network(&self, branch: &AgencyBranchModel) -> BankingResult<()> {
        // Fetch parent network limits
        let network = sqlx::query!(
            r#"
            SELECT aggregate_daily_limit, status::text
            FROM agent_networks 
            WHERE network_id = $1
            "#,
            branch.network_id
        )
        .fetch_optional(&self.pool)
        .await?;

        let network = network.ok_or_else(|| {
            banking_api::error::BankingError::ValidationError {
                field: "network_id".to_string(),
                message: format!("Parent network {} not found", branch.network_id),
            }
        })?;

        // Check if network is active
        if network.status.as_deref() != Some("active") {
            return Err(banking_api::error::BankingError::ValidationError {
                field: "network_status".to_string(),
                message: format!("Cannot create branch under inactive network (status: {:?})", network.status),
            });
        }

        // Convert BigDecimal to Decimal for comparison
        let network_aggregate_limit = network.aggregate_daily_limit.to_string().parse::<Decimal>()
            .map_err(|_| banking_api::error::BankingError::ValidationError {
                field: "aggregate_daily_limit".to_string(),
                message: "Invalid network aggregate daily limit".to_string(),
            })?;
        
        // Validate daily limit against network aggregate limit
        if branch.daily_transaction_limit > network_aggregate_limit {
            return Err(banking_api::error::BankingError::ValidationError {
                field: "daily_transaction_limit".to_string(),
                message: format!(
                    "Branch daily transaction limit ({}) exceeds network aggregate limit ({})",
                    branch.daily_transaction_limit, network_aggregate_limit
                ),
            });
        }

        Ok(())
    }

    /// Validate that terminal limits don't exceed branch limits
    async fn validate_terminal_limits_against_branch(&self, terminal: &AgentTerminalModel) -> BankingResult<()> {
        // Fetch parent branch limits
        let branch = sqlx::query!(
            r#"
            SELECT daily_transaction_limit, status::text
            FROM agent_branches 
            WHERE branch_id = $1
            "#,
            terminal.branch_id
        )
        .fetch_optional(&self.pool)
        .await?;

        let branch = branch.ok_or_else(|| {
            banking_api::error::BankingError::ValidationError {
                field: "branch_id".to_string(),
                message: format!("Parent branch {} not found", terminal.branch_id),
            }
        })?;

        // Check if branch is active
        if branch.status.as_deref() != Some("active") {
            return Err(banking_api::error::BankingError::ValidationError {
                field: "branch_status".to_string(),
                message: format!("Cannot create terminal under inactive branch (status: {:?})", branch.status),
            });
        }

        // Convert BigDecimal to Decimal for comparison
        let branch_daily_limit = branch.daily_transaction_limit.to_string().parse::<Decimal>()
            .map_err(|_| banking_api::error::BankingError::ValidationError {
                field: "daily_transaction_limit".to_string(),
                message: "Invalid branch daily transaction limit".to_string(),
            })?;

        // Validate terminal daily limit against branch daily limit
        if terminal.daily_transaction_limit > branch_daily_limit {
            return Err(banking_api::error::BankingError::ValidationError {
                field: "daily_transaction_limit".to_string(),
                message: format!(
                    "Terminal daily transaction limit ({}) exceeds branch daily limit ({})",
                    terminal.daily_transaction_limit, branch_daily_limit
                ),
            });
        }

        Ok(())
    }
}

#[async_trait]
impl AgentNetworkRepository for AgentNetworkRepositoryImpl {
    /// Agent Network Operations
    async fn create_network(&self, network: AgentNetworkModel) -> BankingResult<AgentNetworkModel> {
        // TODO: AgentNetworkModel doesn't have network_code field, temporarily disable
        // For now, return the input network to avoid compilation errors
        Ok(network)
    }

    async fn update_network(&self, network: AgentNetworkModel) -> BankingResult<AgentNetworkModel> {
        // TODO: AgentNetworkModel field mismatch with database schema, temporarily disable
        Ok(network)
    }

    async fn find_network_by_id(&self, _network_id: Uuid) -> BankingResult<Option<AgentNetworkModel>> {
        // TODO: AgentNetworkModel field mismatch with database schema, temporarily disable
        Ok(None)
    }

    async fn find_networks_by_status(&self, _status: &str) -> BankingResult<Vec<AgentNetworkModel>> {
        // TODO: AgentNetworkModel field mismatch with database schema, temporarily disable
        Ok(vec![])
    }

    async fn find_networks_by_type(&self, _network_type: &str) -> BankingResult<Vec<AgentNetworkModel>> {
        // TODO: AgentNetworkModel field mismatch with database schema, temporarily disable
        Ok(vec![])
    }

    /// Agency Branch Operations with Validation
    async fn create_branch(&self, branch: AgencyBranchModel) -> BankingResult<AgencyBranchModel> {
        // CRITICAL: Validate branch limits against parent network
        self.validate_branch_limits_against_network(&branch).await?;

        // TODO: Implement branch creation - model mismatch with database schema
        // For now, return the input branch to avoid compilation errors
        Ok(branch)
    }

    async fn update_branch(&self, branch: AgencyBranchModel) -> BankingResult<AgencyBranchModel> {
        // CRITICAL: Validate updated limits against parent network
        self.validate_branch_limits_against_network(&branch).await?;

        // TODO: Implement branch update - model mismatch with database schema
        Ok(branch)
    }

    async fn find_branch_by_id(&self, _branch_id: Uuid) -> BankingResult<Option<AgencyBranchModel>> {
        // TODO: Implement branch lookup - model mismatch with database schema
        Ok(None)
    }

    async fn find_branches_by_network(&self, _network_id: Uuid) -> BankingResult<Vec<AgencyBranchModel>> {
        // TODO: Implement branches lookup - model mismatch with database schema
        Ok(vec![])
    }

    async fn find_branches_by_status(&self, _status: &str) -> BankingResult<Vec<AgencyBranchModel>> {
        // TODO: Implement branches by status lookup - model mismatch with database schema
        Ok(vec![])
    }

    /// Agent Terminal Operations with Validation
    async fn create_terminal(&self, terminal: AgentTerminalModel) -> BankingResult<AgentTerminalModel> {
        // CRITICAL: Validate terminal limits against parent branch
        self.validate_terminal_limits_against_branch(&terminal).await?;

        // TODO: Implement terminal creation - model mismatch with database schema
        Ok(terminal)
    }

    async fn update_terminal(&self, terminal: AgentTerminalModel) -> BankingResult<AgentTerminalModel> {
        // CRITICAL: Validate updated limits against parent branch
        self.validate_terminal_limits_against_branch(&terminal).await?;

        // TODO: Implement terminal update - model mismatch with database schema
        Ok(terminal)
    }

    async fn find_terminal_by_id(&self, _terminal_id: Uuid) -> BankingResult<Option<AgentTerminalModel>> {
        // TODO: Implement terminal lookup - model mismatch with database schema
        Ok(None)
    }

    async fn find_terminals_by_branch(&self, _branch_id: Uuid) -> BankingResult<Vec<AgentTerminalModel>> {
        // TODO: Implement terminals by branch lookup - model mismatch with database schema
        Ok(vec![])
    }

    /// Hierarchical Limit Validation - The core validation logic
    async fn validate_hierarchical_limits(&self, terminal_id: Uuid, amount: Decimal) -> BankingResult<LimitValidationResult> {
        // Get terminal with branch and network info in one query
        let hierarchy = sqlx::query!(
            r#"
            SELECT 
                t.daily_transaction_limit as terminal_daily_limit,
                t.status::text as terminal_status,
                b.daily_transaction_limit as branch_daily_limit,
                b.status::text as branch_status,
                n.aggregate_daily_limit as network_daily_limit,
                n.status::text as network_status
            FROM agent_terminals t
            JOIN agent_branches b ON t.branch_id = b.branch_id
            JOIN agent_networks n ON b.network_id = n.network_id
            WHERE t.terminal_id = $1
            "#,
            terminal_id
        )
        .fetch_optional(&self.pool)
        .await?;

        let hierarchy = hierarchy.ok_or_else(|| {
            banking_api::error::BankingError::ValidationError {
                field: "terminal_id".to_string(),
                message: format!("Terminal {terminal_id} not found in hierarchy"),
            }
        })?;

        // Convert BigDecimal to Decimal via string conversion
        let terminal_daily_limit = hierarchy.terminal_daily_limit.to_string().parse::<Decimal>().unwrap_or(Decimal::ZERO);
        let branch_daily_limit = hierarchy.branch_daily_limit.to_string().parse::<Decimal>().unwrap_or(Decimal::ZERO);
        let network_daily_limit = hierarchy.network_daily_limit.to_string().parse::<Decimal>().unwrap_or(Decimal::ZERO);

        // Check status at each level
        let terminal_approved = hierarchy.terminal_status.as_deref() == Some("active") && amount <= terminal_daily_limit;
        let branch_approved = hierarchy.branch_status.as_deref() == Some("active") && amount <= branch_daily_limit;
        let network_approved = hierarchy.network_status.as_deref() == Some("active") && amount <= network_daily_limit;

        let overall_approved = terminal_approved && branch_approved && network_approved;

        let rejection_reason = if !overall_approved {
            Some(format!(
                "Transaction rejected: Terminal({}/{}), Branch({}/{}), Network({}/{})",
                if hierarchy.terminal_status.as_deref() == Some("active") { "Active" } else { hierarchy.terminal_status.as_deref().unwrap_or("Unknown") },
                if amount <= terminal_daily_limit { "Limit OK" } else { "Limit Exceeded" },
                if hierarchy.branch_status.as_deref() == Some("active") { "Active" } else { hierarchy.branch_status.as_deref().unwrap_or("Unknown") },
                if amount <= branch_daily_limit { "Limit OK" } else { "Limit Exceeded" },
                if hierarchy.network_status.as_deref() == Some("active") { "Active" } else { hierarchy.network_status.as_deref().unwrap_or("Unknown") },
                if amount <= network_daily_limit { "Limit OK" } else { "Limit Exceeded" }
            ))
        } else {
            None
        };

        Ok(LimitValidationResult {
            terminal_approved,
            branch_approved,
            network_approved,
            overall_approved,
            rejection_reason,
        })
    }

    // Placeholder implementations for remaining methods
    async fn update_network_daily_volume(&self, _network_id: Uuid, _amount: Decimal) -> BankingResult<()> {
        // TODO: Implement daily volume tracking
        Ok(())
    }

    async fn reset_network_daily_counters(&self) -> BankingResult<()> {
        // TODO: Implement daily counter reset
        Ok(())
    }

    async fn list_networks(&self, _offset: i64, _limit: i64) -> BankingResult<Vec<AgentNetworkModel>> {
        // TODO: AgentNetworkModel field mismatch with database schema, temporarily disable
        Ok(vec![])
    }

    // Additional placeholder implementations would continue here...
    // For brevity, implementing key validation methods only

    async fn find_branches_by_parent(&self, _parent_branch_id: Uuid) -> BankingResult<Vec<AgencyBranchModel>> {
        todo!("Implement hierarchical branch structure")
    }

    async fn find_root_branches(&self, network_id: Uuid) -> BankingResult<Vec<AgencyBranchModel>> {
        // For now, return all branches in network (assuming flat structure)
        self.find_branches_by_network(network_id).await
    }

    async fn find_branch_hierarchy(&self, _branch_id: Uuid) -> BankingResult<Vec<AgencyBranchModel>> {
        todo!("Implement branch hierarchy traversal")
    }

    async fn update_branch_daily_volume(&self, _branch_id: Uuid, _amount: Decimal) -> BankingResult<()> {
        Ok(())
    }

    async fn reset_branch_daily_counters(&self) -> BankingResult<()> {
        Ok(())
    }

    async fn get_branch_gl_prefix(&self, _branch_id: Uuid) -> BankingResult<Option<String>> {
        todo!("Implement GL prefix logic")
    }

    async fn list_branches(&self, _offset: i64, _limit: i64) -> BankingResult<Vec<AgencyBranchModel>> {
        // TODO: Implement branches list - model mismatch with database schema
        Ok(vec![])
    }

    async fn find_terminals_by_agent(&self, _agent_user_id: Uuid) -> BankingResult<Vec<AgentTerminalModel>> {
        // TODO: Implement terminals by agent lookup - model mismatch with database schema
        Ok(vec![])
    }

    async fn find_terminals_by_type(&self, _terminal_type: &str) -> BankingResult<Vec<AgentTerminalModel>> {
        // TODO: Implement terminals by type lookup - model mismatch with database schema
        Ok(vec![])
    }

    async fn find_terminals_by_status(&self, _status: &str) -> BankingResult<Vec<AgentTerminalModel>> {
        // TODO: Implement terminals by status lookup - model mismatch with database schema
        Ok(vec![])
    }

    async fn update_terminal_daily_volume(&self, _terminal_id: Uuid, _amount: Decimal) -> BankingResult<()> {
        Ok(())
    }

    async fn reset_terminal_daily_counters(&self) -> BankingResult<()> {
        Ok(())
    }

    async fn update_terminal_sync(&self, terminal_id: Uuid, sync_time: DateTime<Utc>) -> BankingResult<()> {
        sqlx::query!(
            "UPDATE agent_terminals SET last_sync_at = $1 WHERE terminal_id = $2",
            sync_time,
            terminal_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_terminals_needing_sync(&self, _threshold: DateTime<Utc>) -> BankingResult<Vec<AgentTerminalModel>> {
        // TODO: Implement terminals needing sync lookup - model mismatch with database schema
        Ok(vec![])
    }

    async fn list_terminals(&self, _offset: i64, _limit: i64) -> BankingResult<Vec<AgentTerminalModel>> {
        // TODO: Implement terminals list - model mismatch with database schema
        Ok(vec![])
    }

    async fn get_terminal_limits(&self, terminal_id: Uuid) -> BankingResult<Option<TerminalLimits>> {
        let limits = sqlx::query!(
            r#"
            SELECT daily_transaction_limit, status::text
            FROM agent_terminals 
            WHERE terminal_id = $1
            "#,
            terminal_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(limits.map(|l| TerminalLimits {
            daily_limit: l.daily_transaction_limit.to_string().parse::<Decimal>().unwrap_or(Decimal::ZERO),
            current_volume: Decimal::ZERO, // TODO: Calculate from transactions
            status: l.status.unwrap_or_else(|| "unknown".to_string()),
        }))
    }

    async fn get_branch_limits(&self, branch_id: Uuid) -> BankingResult<Option<BranchLimits>> {
        let limits = sqlx::query!(
            r#"
            SELECT daily_transaction_limit, status::text
            FROM agent_branches 
            WHERE branch_id = $1
            "#,
            branch_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(limits.map(|l| BranchLimits {
            daily_limit: l.daily_transaction_limit.to_string().parse::<Decimal>().unwrap_or(Decimal::ZERO),
            current_volume: Decimal::ZERO, // TODO: Calculate from transactions
            status: l.status.unwrap_or_else(|| "unknown".to_string()),
        }))
    }

    async fn get_network_limits(&self, network_id: Uuid) -> BankingResult<Option<NetworkLimits>> {
        let limits = sqlx::query!(
            r#"
            SELECT aggregate_daily_limit, status::text
            FROM agent_networks 
            WHERE network_id = $1
            "#,
            network_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(limits.map(|l| NetworkLimits {
            daily_limit: l.aggregate_daily_limit.to_string().parse::<Decimal>().unwrap_or(Decimal::ZERO),
            current_volume: Decimal::ZERO, // TODO: Calculate from transactions
            status: l.status.unwrap_or_else(|| "unknown".to_string()),
        }))
    }

    async fn get_current_daily_volume_terminal(&self, _terminal_id: Uuid) -> BankingResult<Decimal> {
        // TODO: Calculate from transactions table
        Ok(Decimal::ZERO)
    }

    async fn get_current_daily_volume_branch(&self, _branch_id: Uuid) -> BankingResult<Decimal> {
        // TODO: Calculate from transactions table
        Ok(Decimal::ZERO)
    }

    async fn get_current_daily_volume_network(&self, _network_id: Uuid) -> BankingResult<Decimal> {
        // TODO: Calculate from transactions table
        Ok(Decimal::ZERO)
    }

    async fn get_network_performance(&self, _network_id: Uuid, _from_date: NaiveDate, _to_date: NaiveDate) -> BankingResult<NetworkPerformanceReport> {
        todo!("Implement network performance reporting")
    }

    async fn get_branch_performance(&self, _branch_id: Uuid, _from_date: NaiveDate, _to_date: NaiveDate) -> BankingResult<BranchPerformanceReport> {
        todo!("Implement branch performance reporting")
    }

    async fn get_terminal_performance(&self, _terminal_id: Uuid, _from_date: NaiveDate, _to_date: NaiveDate) -> BankingResult<TerminalPerformanceReport> {
        todo!("Implement terminal performance reporting")
    }

    async fn update_branch_cash_balance(&self, _branch_id: Uuid, _new_balance: Decimal) -> BankingResult<()> {
        todo!("Implement cash balance management")
    }

    async fn update_terminal_cash_balance(&self, _terminal_id: Uuid, _new_balance: Decimal) -> BankingResult<()> {
        todo!("Implement cash balance management")
    }

    async fn validate_cash_limit(&self, _entity_id: Uuid, _entity_type: &str, _requested_amount: Decimal, _operation_type: &str) -> BankingResult<CashLimitValidationResult> {
        todo!("Implement cash limit validation")
    }

    async fn record_cash_limit_check(&self, _check: CashLimitCheckModel) -> BankingResult<CashLimitCheckModel> {
        todo!("Implement cash limit check recording")
    }

    async fn get_cash_limit_history(&self, _entity_id: Uuid, _from_date: DateTime<Utc>, _to_date: DateTime<Utc>) -> BankingResult<Vec<CashLimitCheckModel>> {
        todo!("Implement cash limit history")
    }

    async fn get_branch_cash_status(&self, _branch_id: Uuid) -> BankingResult<Option<CashStatus>> {
        todo!("Implement branch cash status")
    }

    async fn get_terminal_cash_status(&self, _terminal_id: Uuid) -> BankingResult<Option<CashStatus>> {
        todo!("Implement terminal cash status")
    }

    async fn get_low_cash_alerts(&self, _threshold_percentage: f64) -> BankingResult<Vec<CashAlert>> {
        todo!("Implement low cash alerts")
    }

    async fn network_exists(&self, network_id: Uuid) -> BankingResult<bool> {
        let exists = sqlx::query!(
            "SELECT 1 as exists FROM agent_networks WHERE network_id = $1",
            network_id
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some();

        Ok(exists)
    }

    async fn branch_exists(&self, branch_id: Uuid) -> BankingResult<bool> {
        let exists = sqlx::query!(
            "SELECT 1 as exists FROM agent_branches WHERE branch_id = $1",
            branch_id
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some();

        Ok(exists)
    }

    async fn terminal_exists(&self, terminal_id: Uuid) -> BankingResult<bool> {
        let exists = sqlx::query!(
            "SELECT 1 as exists FROM agent_terminals WHERE terminal_id = $1",
            terminal_id
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some();

        Ok(exists)
    }

    async fn count_networks(&self) -> BankingResult<i64> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM agent_networks"
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(0);

        Ok(count)
    }

    async fn count_branches(&self) -> BankingResult<i64> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM agent_branches"
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(0);

        Ok(count)
    }

    async fn count_terminals(&self) -> BankingResult<i64> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM agent_terminals"
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(0);

        Ok(count)
    }
}
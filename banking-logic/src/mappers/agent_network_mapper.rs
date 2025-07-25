use banking_api::domain::{
    AgentNetwork, AgencyBranch, AgentTerminal,
    NetworkType, NetworkStatus, BranchStatus, TerminalType, TerminalStatus
};
use banking_db::models::{AgentNetworkModel, AgencyBranchModel, AgentTerminalModel};

pub struct AgentNetworkMapper;

impl AgentNetworkMapper {
    /// Map from domain AgentNetwork to database AgentNetworkModel
    pub fn network_to_model(network: AgentNetwork) -> AgentNetworkModel {
        AgentNetworkModel {
            network_id: network.network_id,
            network_name: network.network_name.to_string(),
            network_type: Self::network_type_to_string(network.network_type),
            status: Self::network_status_to_string(network.status),
            contract_id: network.contract_id,
            aggregate_daily_limit: network.aggregate_daily_limit,
            current_daily_volume: network.current_daily_volume,
            settlement_gl_code: network.settlement_gl_code,
            created_at: network.created_at,
            last_updated_at: network.created_at,
            updated_by: "system".to_string(),
        }
    }

    /// Map from database AgentNetworkModel to domain AgentNetwork
    pub fn network_from_model(model: AgentNetworkModel) -> AgentNetwork {
        AgentNetwork {
            network_id: model.network_id,
            network_name: heapless::String::try_from(model.network_name.as_str()).unwrap_or_default(),
            network_type: Self::string_to_network_type(&model.network_type),
            status: Self::string_to_network_status(&model.status),
            contract_id: model.contract_id,
            aggregate_daily_limit: model.aggregate_daily_limit,
            current_daily_volume: model.current_daily_volume,
            settlement_gl_code: model.settlement_gl_code,
            created_at: model.created_at,
        }
    }

    /// Map from domain AgencyBranch to database AgencyBranchModel
    pub fn branch_to_model(branch: AgencyBranch) -> AgencyBranchModel {
        AgencyBranchModel {
            branch_id: branch.branch_id,
            network_id: branch.network_id,
            parent_branch_id: branch.parent_branch_id,
            branch_name: branch.branch_name.to_string(),
            branch_code: branch.branch_code,
            branch_level: branch.branch_level,
            gl_code_prefix: branch.gl_code_prefix,
            geolocation: branch.geolocation,
            status: Self::branch_status_to_string(branch.status),
            daily_transaction_limit: branch.daily_transaction_limit,
            current_daily_volume: branch.current_daily_volume,
            max_cash_limit: branch.max_cash_limit,
            current_cash_balance: branch.current_cash_balance,
            minimum_cash_balance: branch.minimum_cash_balance,
            created_at: branch.created_at,
            last_updated_at: branch.created_at,
            updated_by: "system".to_string(),
        }
    }

    /// Map from database AgencyBranchModel to domain AgencyBranch
    pub fn branch_from_model(model: AgencyBranchModel) -> AgencyBranch {
        AgencyBranch {
            branch_id: model.branch_id,
            network_id: model.network_id,
            parent_branch_id: model.parent_branch_id,
            branch_name: heapless::String::try_from(model.branch_name.as_str()).unwrap_or_default(),
            branch_code: model.branch_code,
            branch_level: model.branch_level,
            gl_code_prefix: model.gl_code_prefix,
            geolocation: model.geolocation,
            status: Self::string_to_branch_status(&model.status),
            daily_transaction_limit: model.daily_transaction_limit,
            current_daily_volume: model.current_daily_volume,
            max_cash_limit: model.max_cash_limit,
            current_cash_balance: model.current_cash_balance,
            minimum_cash_balance: model.minimum_cash_balance,
            created_at: model.created_at,
        }
    }

    /// Map from domain AgentTerminal to database AgentTerminalModel
    pub fn terminal_to_model(terminal: AgentTerminal) -> AgentTerminalModel {
        AgentTerminalModel {
            terminal_id: terminal.terminal_id,
            branch_id: terminal.branch_id,
            agent_user_id: terminal.agent_user_id,
            terminal_type: Self::terminal_type_to_string(terminal.terminal_type),
            terminal_name: terminal.terminal_name.to_string(),
            daily_transaction_limit: terminal.daily_transaction_limit,
            current_daily_volume: terminal.current_daily_volume,
            max_cash_limit: terminal.max_cash_limit,
            current_cash_balance: terminal.current_cash_balance,
            minimum_cash_balance: terminal.minimum_cash_balance,
            status: Self::terminal_status_to_string(terminal.status),
            last_sync_at: terminal.last_sync_at,
            created_at: terminal.last_sync_at,
            last_updated_at: terminal.last_sync_at,
            updated_by: "system".to_string(),
        }
    }

    /// Map from database AgentTerminalModel to domain AgentTerminal
    pub fn terminal_from_model(model: AgentTerminalModel) -> AgentTerminal {
        AgentTerminal {
            terminal_id: model.terminal_id,
            branch_id: model.branch_id,
            agent_user_id: model.agent_user_id,
            terminal_type: Self::string_to_terminal_type(&model.terminal_type),
            terminal_name: heapless::String::try_from(model.terminal_name.as_str()).unwrap_or_default(),
            daily_transaction_limit: model.daily_transaction_limit,
            current_daily_volume: model.current_daily_volume,
            max_cash_limit: model.max_cash_limit,
            current_cash_balance: model.current_cash_balance,
            minimum_cash_balance: model.minimum_cash_balance,
            status: Self::string_to_terminal_status(&model.status),
            last_sync_at: model.last_sync_at,
        }
    }

    // Network Type conversions
    fn network_type_to_string(network_type: NetworkType) -> String {
        match network_type {
            NetworkType::Internal => "Internal".to_string(),
            NetworkType::Partner => "Partner".to_string(),
            NetworkType::ThirdParty => "ThirdParty".to_string(),
        }
    }

    fn string_to_network_type(s: &str) -> NetworkType {
        match s {
            "Internal" => NetworkType::Internal,
            "Partner" => NetworkType::Partner,
            "ThirdParty" => NetworkType::ThirdParty,
            _ => NetworkType::Internal, // Default fallback
        }
    }

    // Network Status conversions
    fn network_status_to_string(status: NetworkStatus) -> String {
        match status {
            NetworkStatus::Active => "Active".to_string(),
            NetworkStatus::Suspended => "Suspended".to_string(),
            NetworkStatus::Terminated => "Terminated".to_string(),
        }
    }

    fn string_to_network_status(s: &str) -> NetworkStatus {
        match s {
            "Active" => NetworkStatus::Active,
            "Suspended" => NetworkStatus::Suspended,
            "Terminated" => NetworkStatus::Terminated,
            _ => NetworkStatus::Active, // Default fallback
        }
    }

    // Branch Status conversions
    fn branch_status_to_string(status: BranchStatus) -> String {
        match status {
            BranchStatus::Active => "Active".to_string(),
            BranchStatus::Suspended => "Suspended".to_string(),
            BranchStatus::Closed => "Closed".to_string(),
        }
    }

    fn string_to_branch_status(s: &str) -> BranchStatus {
        match s {
            "Active" => BranchStatus::Active,
            "Suspended" => BranchStatus::Suspended,
            "Closed" => BranchStatus::Closed,
            _ => BranchStatus::Active, // Default fallback
        }
    }

    // Terminal Type conversions
    fn terminal_type_to_string(terminal_type: TerminalType) -> String {
        match terminal_type {
            TerminalType::Pos => "Pos".to_string(),
            TerminalType::Mobile => "Mobile".to_string(),
            TerminalType::Atm => "Atm".to_string(),
            TerminalType::WebPortal => "WebPortal".to_string(),
        }
    }

    fn string_to_terminal_type(s: &str) -> TerminalType {
        match s {
            "Pos" => TerminalType::Pos,
            "Mobile" => TerminalType::Mobile,
            "Atm" => TerminalType::Atm,
            "WebPortal" => TerminalType::WebPortal,
            _ => TerminalType::Mobile, // Default fallback
        }
    }

    // Terminal Status conversions
    fn terminal_status_to_string(status: TerminalStatus) -> String {
        match status {
            TerminalStatus::Active => "Active".to_string(),
            TerminalStatus::Maintenance => "Maintenance".to_string(),
            TerminalStatus::Suspended => "Suspended".to_string(),
            TerminalStatus::Decommissioned => "Decommissioned".to_string(),
        }
    }

    fn string_to_terminal_status(s: &str) -> TerminalStatus {
        match s {
            "Active" => TerminalStatus::Active,
            "Maintenance" => TerminalStatus::Maintenance,
            "Suspended" => TerminalStatus::Suspended,
            "Decommissioned" => TerminalStatus::Decommissioned,
            _ => TerminalStatus::Active, // Default fallback
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use rust_decimal::Decimal;
    use uuid::Uuid;

    #[test]
    fn test_network_type_conversion() {
        assert_eq!(
            AgentNetworkMapper::network_type_to_string(NetworkType::Internal),
            "Internal"
        );
        assert_eq!(
            AgentNetworkMapper::string_to_network_type("Partner"),
            NetworkType::Partner
        );
    }

    #[test]
    fn test_network_status_conversion() {
        assert_eq!(
            AgentNetworkMapper::network_status_to_string(NetworkStatus::Active),
            "Active"
        );
        assert_eq!(
            AgentNetworkMapper::string_to_network_status("Suspended"),
            NetworkStatus::Suspended
        );
    }

    #[test]
    fn test_branch_status_conversion() {
        assert_eq!(
            AgentNetworkMapper::branch_status_to_string(BranchStatus::Closed),
            "Closed"
        );
        assert_eq!(
            AgentNetworkMapper::string_to_branch_status("Active"),
            BranchStatus::Active
        );
    }

    #[test]
    fn test_terminal_type_conversion() {
        assert_eq!(
            AgentNetworkMapper::terminal_type_to_string(TerminalType::Atm),
            "Atm"
        );
        assert_eq!(
            AgentNetworkMapper::string_to_terminal_type("WebPortal"),
            TerminalType::WebPortal
        );
    }

    #[test]
    fn test_terminal_status_conversion() {
        assert_eq!(
            AgentNetworkMapper::terminal_status_to_string(TerminalStatus::Maintenance),
            "Maintenance"
        );
        assert_eq!(
            AgentNetworkMapper::string_to_terminal_status("Decommissioned"),
            TerminalStatus::Decommissioned
        );
    }

    #[test]
    fn test_network_model_conversion() {
        let network = AgentNetwork {
            network_id: Uuid::new_v4(),
            network_name: heapless::String::try_from("Test Network").unwrap(),
            network_type: NetworkType::Partner,
            status: NetworkStatus::Active,
            contract_id: Some(Uuid::new_v4()),
            aggregate_daily_limit: Decimal::new(1000000, 2),
            current_daily_volume: Decimal::ZERO,
            settlement_gl_code: heapless::String::try_from("GL001").unwrap(),
            created_at: Utc::now(),
        };

        let model = AgentNetworkMapper::network_to_model(network.clone());
        let converted_back = AgentNetworkMapper::network_from_model(model);

        assert_eq!(network.network_id, converted_back.network_id);
        assert_eq!(network.network_name, converted_back.network_name);
        assert_eq!(network.network_type, converted_back.network_type);
        assert_eq!(network.status, converted_back.status);
    }

    #[test]
    fn test_branch_model_conversion() {
        let branch = AgencyBranch {
            branch_id: Uuid::new_v4(),
            network_id: Uuid::new_v4(),
            parent_branch_id: Some(Uuid::new_v4()),
            branch_name: heapless::String::try_from("Test Branch").unwrap(),
            branch_code: heapless::String::try_from("BR001").unwrap(),
            branch_level: 2,
            gl_code_prefix: heapless::String::try_from("GL").unwrap(),
            geolocation: Some("Location".to_string()),
            status: BranchStatus::Active,
            daily_transaction_limit: Decimal::new(500000, 2),
            current_daily_volume: Decimal::ZERO,
            max_cash_limit: Decimal::new(1000000, 2),
            current_cash_balance: Decimal::new(500000, 2),
            minimum_cash_balance: Decimal::new(100000, 2),
            created_at: Utc::now(),
        };

        let model = AgentNetworkMapper::branch_to_model(branch.clone());
        let converted_back = AgentNetworkMapper::branch_from_model(model);

        assert_eq!(branch.branch_id, converted_back.branch_id);
        assert_eq!(branch.branch_name, converted_back.branch_name);
        assert_eq!(branch.branch_level, converted_back.branch_level);
        assert_eq!(branch.status, converted_back.status);
    }

    #[test]
    fn test_terminal_model_conversion() {
        let terminal = AgentTerminal {
            terminal_id: Uuid::new_v4(),
            branch_id: Uuid::new_v4(),
            agent_user_id: Uuid::new_v4(),
            terminal_type: TerminalType::Mobile,
            terminal_name: heapless::String::try_from("Test Terminal").unwrap(),
            daily_transaction_limit: Decimal::new(100000, 2),
            current_daily_volume: Decimal::ZERO,
            max_cash_limit: Decimal::new(200000, 2),
            current_cash_balance: Decimal::new(100000, 2),
            minimum_cash_balance: Decimal::new(20000, 2),
            status: TerminalStatus::Active,
            last_sync_at: Utc::now(),
        };

        let model = AgentNetworkMapper::terminal_to_model(terminal.clone());
        let converted_back = AgentNetworkMapper::terminal_from_model(model);

        assert_eq!(terminal.terminal_id, converted_back.terminal_id);
        assert_eq!(terminal.terminal_name, converted_back.terminal_name);
        assert_eq!(terminal.terminal_type, converted_back.terminal_type);
        assert_eq!(terminal.status, converted_back.status);
    }
}
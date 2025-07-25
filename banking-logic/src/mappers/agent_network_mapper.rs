use banking_api::domain::{
    AgentNetwork, AgencyBranch, AgentTerminal,
    NetworkType, NetworkStatus, BranchStatus, TerminalType, TerminalStatus, BranchType, BranchRiskRating
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
            status: Self::branch_status_to_string(branch.status),
            daily_transaction_limit: branch.daily_transaction_limit,
            current_daily_volume: branch.current_daily_volume,
            max_cash_limit: branch.max_cash_limit,
            current_cash_balance: branch.current_cash_balance,
            minimum_cash_balance: branch.minimum_cash_balance,
            created_at: branch.created_at,
            
            // New location fields - serialize as JSON or extract coordinates
            address_json: serde_json::to_string(&branch.address).unwrap_or_default(),
            gps_latitude: branch.gps_coordinates.map(|c| c.latitude),
            gps_longitude: branch.gps_coordinates.map(|c| c.longitude),
            gps_accuracy_meters: branch.gps_coordinates.and_then(|c| c.accuracy_meters),
            landmark_description: branch.landmark_description.map(|s| s.to_string()),
            
            // Operational details
            operating_hours_json: serde_json::to_string(&branch.operating_hours).unwrap_or_default(),
            holiday_schedule_json: serde_json::to_string(&branch.holiday_schedule).unwrap_or_default(),
            temporary_closure_json: branch.temporary_closure.as_ref().map(|tc| serde_json::to_string(tc).unwrap_or_default()),
            
            // Contact information
            primary_phone: branch.primary_phone.to_string(),
            secondary_phone: branch.secondary_phone.map(|s| s.to_string()),
            email: branch.email.map(|s| s.to_string()),
            branch_manager_id: branch.branch_manager_id,
            
            // Services and capabilities
            branch_type: format!("{:?}", branch.branch_type),
            supported_services_json: serde_json::to_string(&branch.supported_services).unwrap_or_default(),
            supported_currencies_json: serde_json::to_string(&branch.supported_currencies).unwrap_or_default(),
            languages_spoken_json: serde_json::to_string(&branch.languages_spoken).unwrap_or_default(),
            
            // Security and access
            security_features_json: serde_json::to_string(&branch.security_features).unwrap_or_default(),
            accessibility_features_json: serde_json::to_string(&branch.accessibility_features).unwrap_or_default(),
            required_documents_json: serde_json::to_string(&branch.required_documents).unwrap_or_default(),
            
            // Customer capacity
            max_daily_customers: branch.max_daily_customers,
            average_wait_time_minutes: branch.average_wait_time_minutes,
            
            // Transaction limits
            per_transaction_limit: branch.per_transaction_limit,
            monthly_transaction_limit: branch.monthly_transaction_limit,
            
            // Compliance and risk
            risk_rating: format!("{:?}", branch.risk_rating),
            last_audit_date: branch.last_audit_date,
            compliance_certifications_json: serde_json::to_string(&branch.compliance_certifications).unwrap_or_default(),
            
            // Metadata
            last_updated_at: branch.last_updated_at,
            updated_by: branch.updated_by.to_string(),
        }
    }

    /// Map from database AgencyBranchModel to domain AgencyBranch
    /// Uses create_minimal for backward compatibility with old database schema
    pub fn branch_from_model(model: AgencyBranchModel) -> AgencyBranch {
        let mut branch = AgencyBranch::create_minimal(
            model.branch_id,
            model.network_id,
            model.parent_branch_id,
            heapless::String::try_from(model.branch_name.as_str()).unwrap_or_default(),
            model.branch_code,
            model.branch_level,
            model.gl_code_prefix,
            Self::string_to_branch_status(&model.status),
            model.daily_transaction_limit,
            model.current_daily_volume,
            model.max_cash_limit,
            model.current_cash_balance,
            model.minimum_cash_balance,
            model.created_at,
        );
        
        // Set parsed branch type and risk rating from database
        branch.branch_type = Self::string_to_branch_type(&model.branch_type);
        branch.risk_rating = Self::string_to_branch_risk_rating(&model.risk_rating);
        branch.last_updated_at = model.last_updated_at;
        branch.updated_by = heapless::String::try_from(model.updated_by.as_str()).unwrap_or_default();
        
        branch
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

    // Helper methods for enum conversions
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
            _ => NetworkType::Internal, // Default
        }
    }

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
            _ => NetworkStatus::Active, // Default
        }
    }

    fn branch_status_to_string(status: BranchStatus) -> String {
        match status {
            BranchStatus::Active => "Active".to_string(),
            BranchStatus::Suspended => "Suspended".to_string(),
            BranchStatus::Closed => "Closed".to_string(),
            BranchStatus::TemporarilyClosed => "TemporarilyClosed".to_string(),
        }
    }

    fn string_to_branch_status(s: &str) -> BranchStatus {
        match s {
            "Active" => BranchStatus::Active,
            "Suspended" => BranchStatus::Suspended,
            "Closed" => BranchStatus::Closed,
            "TemporarilyClosed" => BranchStatus::TemporarilyClosed,
            _ => BranchStatus::Active, // Default
        }
    }

    fn string_to_branch_type(s: &str) -> BranchType {
        match s {
            "MainBranch" => BranchType::MainBranch,
            "SubBranch" => BranchType::SubBranch,
            "AgentOutlet" => BranchType::AgentOutlet,
            "StandaloneKiosk" => BranchType::StandaloneKiosk,
            "PartnerAgent" => BranchType::PartnerAgent,
            "ATMLocation" => BranchType::ATMLocation,
            "MobileUnit" => BranchType::MobileUnit,
            _ => BranchType::SubBranch, // Default
        }
    }

    fn string_to_branch_risk_rating(s: &str) -> BranchRiskRating {
        match s {
            "Low" => BranchRiskRating::Low,
            "Medium" => BranchRiskRating::Medium,
            "High" => BranchRiskRating::High,
            "Critical" => BranchRiskRating::Critical,
            _ => BranchRiskRating::Low, // Default
        }
    }

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
            _ => TerminalType::Pos, // Default
        }
    }

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
            _ => TerminalStatus::Active, // Default
        }
    }
}
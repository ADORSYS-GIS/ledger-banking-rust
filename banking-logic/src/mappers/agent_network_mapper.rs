use banking_api::domain::{
    AgentNetwork, AgencyBranch, AgentTerminal,
    NetworkType, NetworkStatus, BranchStatus, TerminalType, TerminalStatus, BranchType, BranchRiskRating
};
use banking_db::models::agent_network::{
    AgentNetworkModel, AgencyBranchModel, AgentTerminalModel,
    NetworkType as DbNetworkType, NetworkStatus as DbNetworkStatus,
    BranchStatus as DbBranchStatus, TerminalType as DbTerminalType,
    TerminalStatus as DbTerminalStatus, BranchType as DbBranchType,
    BranchRiskRating as DbBranchRiskRating
};
use uuid::Uuid;

pub struct AgentNetworkMapper;

impl AgentNetworkMapper {
    /// Map from domain AgentNetwork to database AgentNetworkModel
    pub fn network_to_model(network: AgentNetwork) -> AgentNetworkModel {
        AgentNetworkModel {
            network_id: network.network_id,
            network_name: network.network_name,
            network_type: Self::network_type_to_db(network.network_type),
            status: Self::network_status_to_db(network.status),
            contract_id: network.contract_id,
            aggregate_daily_limit: network.aggregate_daily_limit,
            current_daily_volume: network.current_daily_volume,
            settlement_gl_code: network.settlement_gl_code,
            created_at: network.created_at,
            last_updated_at: network.created_at,
            updated_by: Uuid::nil(), // System UUID
        }
    }

    /// Map from database AgentNetworkModel to domain AgentNetwork
    pub fn network_from_model(model: AgentNetworkModel) -> AgentNetwork {
        AgentNetwork {
            network_id: model.network_id,
            network_name: model.network_name,
            network_type: Self::network_type_from_db(model.network_type),
            status: Self::network_status_from_db(model.status),
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
            branch_name: branch.branch_name,
            branch_code: branch.branch_code,
            branch_level: branch.branch_level,
            gl_code_prefix: branch.gl_code_prefix,
            status: Self::branch_status_to_db(branch.status),
            daily_transaction_limit: branch.daily_transaction_limit,
            current_daily_volume: branch.current_daily_volume,
            max_cash_limit: branch.max_cash_limit,
            current_cash_balance: branch.current_cash_balance,
            minimum_cash_balance: branch.minimum_cash_balance,
            created_at: branch.created_at,
            
            // Location fields - normalized to UUID references
            address_id: branch.address, // Now a UUID reference
            landmark_description: branch.landmark_description,
            
            // Operational details - normalized to UUID references
            operating_hours_id: branch.operating_hours, // Now a UUID reference
            holiday_schedule_json: heapless::String::try_from("{}").unwrap_or_default(), // Placeholder for holiday schedule
            temporary_closure_json: None, // Simplified for now
            
            // Contact information - now serialized as JSON array of messaging UUIDs
            messaging_json: heapless::String::try_from("[]").unwrap_or_default(), // Placeholder for messaging array
            branch_manager_id: branch.branch_manager_id,
            
            // Services and capabilities - normalized to UUID reference
            branch_type: Self::branch_type_to_db(branch.branch_type),
            branch_capabilities_id: branch.branch_capabilities, // Now a UUID reference
            
            // Security and access - normalized to UUID reference
            security_access_id: branch.security_access, // Now a UUID reference
            
            // Customer capacity
            max_daily_customers: branch.max_daily_customers,
            average_wait_time_minutes: branch.average_wait_time_minutes,
            
            // Transaction limits (enhanced from existing)
            per_transaction_limit: branch.per_transaction_limit,
            monthly_transaction_limit: branch.monthly_transaction_limit,
            
            // Compliance and risk
            risk_rating: Self::branch_risk_rating_to_db(branch.risk_rating),
            last_audit_date: branch.last_audit_date,
            compliance_certifications_json: heapless::String::try_from("[]").unwrap_or_default(), // Placeholder
            
            // Metadata
            last_updated_at: branch.last_updated_at,
            updated_by: branch.updated_by,
        }
    }

    /// Map from database AgencyBranchModel to domain AgencyBranch
    /// Uses create_minimal for backward compatibility with old database schema
    pub fn branch_from_model(model: AgencyBranchModel) -> AgencyBranch {
        let mut branch = AgencyBranch::create_minimal(
            model.branch_id,
            model.network_id,
            model.parent_branch_id,
            model.branch_name,
            model.branch_code,
            model.branch_level,
            model.gl_code_prefix,
            Self::branch_status_from_db(model.status),
            model.daily_transaction_limit,
            model.current_daily_volume,
            model.max_cash_limit,
            model.current_cash_balance,
            model.minimum_cash_balance,
            model.created_at,
            model.address_id,
            model.operating_hours_id,
            model.branch_capabilities_id,
            model.security_access_id,
        );
        
        // Set parsed branch type and risk rating from database
        branch.branch_type = Self::branch_type_from_db(model.branch_type);
        branch.risk_rating = Self::branch_risk_rating_from_db(model.risk_rating);
        branch.last_updated_at = model.last_updated_at;
        branch.updated_by = model.updated_by;
        
        branch
    }

    /// Map from domain AgentTerminal to database AgentTerminalModel
    pub fn terminal_to_model(terminal: AgentTerminal) -> AgentTerminalModel {
        AgentTerminalModel {
            terminal_id: terminal.terminal_id,
            branch_id: terminal.branch_id,
            agent_user_id: terminal.agent_user_id,
            terminal_type: Self::terminal_type_to_db(terminal.terminal_type),
            terminal_name: terminal.terminal_name,
            daily_transaction_limit: terminal.daily_transaction_limit,
            current_daily_volume: terminal.current_daily_volume,
            max_cash_limit: terminal.max_cash_limit,
            current_cash_balance: terminal.current_cash_balance,
            minimum_cash_balance: terminal.minimum_cash_balance,
            status: Self::terminal_status_to_db(terminal.status),
            last_sync_at: terminal.last_sync_at,
            created_at: terminal.last_sync_at,
            last_updated_at: terminal.last_sync_at,
            updated_by: Uuid::nil(), // System UUID
        }
    }

    /// Map from database AgentTerminalModel to domain AgentTerminal
    pub fn terminal_from_model(model: AgentTerminalModel) -> AgentTerminal {
        AgentTerminal {
            terminal_id: model.terminal_id,
            branch_id: model.branch_id,
            agent_user_id: model.agent_user_id,
            terminal_type: Self::terminal_type_from_db(model.terminal_type),
            terminal_name: model.terminal_name,
            daily_transaction_limit: model.daily_transaction_limit,
            current_daily_volume: model.current_daily_volume,
            max_cash_limit: model.max_cash_limit,
            current_cash_balance: model.current_cash_balance,
            minimum_cash_balance: model.minimum_cash_balance,
            status: Self::terminal_status_from_db(model.status),
            last_sync_at: model.last_sync_at,
        }
    }

    // Helper methods for enum conversions
    fn network_type_to_db(network_type: NetworkType) -> DbNetworkType {
        match network_type {
            NetworkType::Internal => DbNetworkType::Internal,
            NetworkType::Partner => DbNetworkType::Partner,
            NetworkType::ThirdParty => DbNetworkType::ThirdParty,
        }
    }

    fn network_type_from_db(db_type: DbNetworkType) -> NetworkType {
        match db_type {
            DbNetworkType::Internal => NetworkType::Internal,
            DbNetworkType::Partner => NetworkType::Partner,
            DbNetworkType::ThirdParty => NetworkType::ThirdParty,
        }
    }

    fn network_status_to_db(status: NetworkStatus) -> DbNetworkStatus {
        match status {
            NetworkStatus::Active => DbNetworkStatus::Active,
            NetworkStatus::Suspended => DbNetworkStatus::Suspended,
            NetworkStatus::Terminated => DbNetworkStatus::Terminated,
        }
    }

    fn network_status_from_db(db_status: DbNetworkStatus) -> NetworkStatus {
        match db_status {
            DbNetworkStatus::Active => NetworkStatus::Active,
            DbNetworkStatus::Suspended => NetworkStatus::Suspended,
            DbNetworkStatus::Terminated => NetworkStatus::Terminated,
        }
    }

    fn branch_status_to_db(status: BranchStatus) -> DbBranchStatus {
        match status {
            BranchStatus::Active => DbBranchStatus::Active,
            BranchStatus::Suspended => DbBranchStatus::Suspended,
            BranchStatus::Closed => DbBranchStatus::Closed,
            BranchStatus::TemporarilyClosed => DbBranchStatus::TemporarilyClosed,
        }
    }

    fn branch_status_from_db(db_status: DbBranchStatus) -> BranchStatus {
        match db_status {
            DbBranchStatus::Active => BranchStatus::Active,
            DbBranchStatus::Suspended => BranchStatus::Suspended,
            DbBranchStatus::Closed => BranchStatus::Closed,
            DbBranchStatus::TemporarilyClosed => BranchStatus::TemporarilyClosed,
        }
    }

    fn branch_type_to_db(branch_type: BranchType) -> DbBranchType {
        match branch_type {
            BranchType::MainBranch => DbBranchType::MainBranch,
            BranchType::SubBranch => DbBranchType::SubBranch,
            BranchType::AgentOutlet => DbBranchType::AgentOutlet,
            BranchType::StandaloneKiosk => DbBranchType::StandaloneKiosk,
            BranchType::PartnerAgent => DbBranchType::PartnerAgent,
            BranchType::ATMLocation => DbBranchType::AtmLocation,
            BranchType::MobileUnit => DbBranchType::MobileUnit,
        }
    }

    fn branch_type_from_db(db_type: DbBranchType) -> BranchType {
        match db_type {
            DbBranchType::MainBranch => BranchType::MainBranch,
            DbBranchType::SubBranch => BranchType::SubBranch,
            DbBranchType::AgentOutlet => BranchType::AgentOutlet,
            DbBranchType::StandaloneKiosk => BranchType::StandaloneKiosk,
            DbBranchType::PartnerAgent => BranchType::PartnerAgent,
            DbBranchType::AtmLocation => BranchType::ATMLocation,
            DbBranchType::MobileUnit => BranchType::MobileUnit,
        }
    }

    fn branch_risk_rating_to_db(rating: BranchRiskRating) -> DbBranchRiskRating {
        match rating {
            BranchRiskRating::Low => DbBranchRiskRating::Low,
            BranchRiskRating::Medium => DbBranchRiskRating::Medium,
            BranchRiskRating::High => DbBranchRiskRating::High,
            BranchRiskRating::Critical => DbBranchRiskRating::Critical,
        }
    }

    fn branch_risk_rating_from_db(db_rating: DbBranchRiskRating) -> BranchRiskRating {
        match db_rating {
            DbBranchRiskRating::Low => BranchRiskRating::Low,
            DbBranchRiskRating::Medium => BranchRiskRating::Medium,
            DbBranchRiskRating::High => BranchRiskRating::High,
            DbBranchRiskRating::Critical => BranchRiskRating::Critical,
        }
    }

    fn terminal_type_to_db(terminal_type: TerminalType) -> DbTerminalType {
        match terminal_type {
            TerminalType::Pos => DbTerminalType::Pos,
            TerminalType::Mobile => DbTerminalType::Mobile,
            TerminalType::Atm => DbTerminalType::Atm,
            TerminalType::WebPortal => DbTerminalType::WebPortal,
        }
    }

    fn terminal_type_from_db(db_type: DbTerminalType) -> TerminalType {
        match db_type {
            DbTerminalType::Pos => TerminalType::Pos,
            DbTerminalType::Mobile => TerminalType::Mobile,
            DbTerminalType::Atm => TerminalType::Atm,
            DbTerminalType::WebPortal => TerminalType::WebPortal,
        }
    }

    fn terminal_status_to_db(status: TerminalStatus) -> DbTerminalStatus {
        match status {
            TerminalStatus::Active => DbTerminalStatus::Active,
            TerminalStatus::Maintenance => DbTerminalStatus::Maintenance,
            TerminalStatus::Suspended => DbTerminalStatus::Suspended,
            TerminalStatus::Decommissioned => DbTerminalStatus::Decommissioned,
        }
    }

    fn terminal_status_from_db(db_status: DbTerminalStatus) -> TerminalStatus {
        match db_status {
            DbTerminalStatus::Active => TerminalStatus::Active,
            DbTerminalStatus::Maintenance => TerminalStatus::Maintenance,
            DbTerminalStatus::Suspended => TerminalStatus::Suspended,
            DbTerminalStatus::Decommissioned => TerminalStatus::Decommissioned,
        }
    }
}
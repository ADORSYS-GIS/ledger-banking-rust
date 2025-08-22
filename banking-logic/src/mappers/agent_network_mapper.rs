use banking_api::domain::{
    AgentNetwork, AgencyBranch, AgentTerminal,
    NetworkType, NetworkStatus, BranchStatus, TerminalType, TerminalStatus, BranchType, BranchRiskRating
};
use banking_api::domain::person::MessagingType as DomainMessagingType;
use banking_db::models::person::MessagingType as DbMessagingType;
use banking_db::models::agent_network::{
    AgentNetworkModel, AgencyBranchModel, AgentTerminalModel,
    NetworkType as DbNetworkType, NetworkStatus as DbNetworkStatus,
    BranchStatus as DbBranchStatus, TerminalType as DbTerminalType,
    TerminalStatus as DbTerminalStatus, BranchType as DbBranchType,
    BranchRiskRating as DbBranchRiskRating,
    // New models for agent network related structures
    HollidayPlanModel, TemporaryClosureModel,
    OperatingHoursModel, BranchCapabilitiesModel, SecurityAccessModel,
    RequiredDocumentModel, ComplianceCertModel
};
use uuid::Uuid;

pub struct AgentNetworkMapper;

impl AgentNetworkMapper {
    /// Map from domain AgentNetwork to database AgentNetworkModel
    pub fn network_to_model(network: AgentNetwork) -> AgentNetworkModel {
        AgentNetworkModel {
            id: network.id,
            network_name: network.network_name,
            network_type: Self::network_type_to_db(network.network_type),
            status: Self::network_status_to_db(network.status),
            contract_external_id: network.contract_external_id,
            aggregate_daily_limit: network.aggregate_daily_limit,
            current_daily_volume: network.current_daily_volume,
            settlement_gl_code: network.settlement_gl_code,
            created_at: network.created_at,
            last_updated_at: network.created_at,
            updated_by_person_id: Uuid::nil(), // System UUID
        }
    }

    /// Map from database AgentNetworkModel to domain AgentNetwork
    pub fn network_from_model(model: AgentNetworkModel) -> AgentNetwork {
        AgentNetwork {
            id: model.id,
            network_name: model.network_name,
            network_type: Self::network_type_from_db(model.network_type),
            status: Self::network_status_from_db(model.status),
            contract_external_id: model.contract_external_id,
            aggregate_daily_limit: model.aggregate_daily_limit,
            current_daily_volume: model.current_daily_volume,
            settlement_gl_code: model.settlement_gl_code,
            created_at: model.created_at,
        }
    }

    /// Map from domain AgencyBranch to database AgencyBranchModel
    pub fn branch_to_model(branch: AgencyBranch) -> AgencyBranchModel {
        AgencyBranchModel {
            id: branch.id,
            agent_network_id: branch.agent_network_id,
            parent_agency_branch_id: branch.parent_agency_branch_id,
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
            location_id: branch.location_id,
            landmark_description: branch.landmark_description,
            
            // Operational details - normalized to UUID references
            operating_hours_id: branch.operating_hours_id,
            holiday_plan_id: branch.holiday_plan_id,
            temporary_closure_id: branch.temporary_closure_id, // UUID reference to temporary closure
            
            // Contact information - individual messaging fields
            messaging1_id: branch.messaging1_id,
            messaging1_type: branch.messaging1_type.map(Self::messaging_type_to_db),
            messaging2_id: branch.messaging2_id,
            messaging2_type: branch.messaging2_type.map(Self::messaging_type_to_db),
            messaging3_id: branch.messaging3_id,
            messaging3_type: branch.messaging3_type.map(Self::messaging_type_to_db),
            messaging4_id: branch.messaging4_id,
            messaging4_type: branch.messaging4_type.map(Self::messaging_type_to_db),
            messaging5_id: branch.messaging5_id,
            messaging5_type: branch.messaging5_type.map(Self::messaging_type_to_db),
            branch_manager_person_id: branch.branch_manager_person_id,
            
            // Services and capabilities - normalized to UUID reference
            branch_type: Self::branch_type_to_db(branch.branch_type),
            branch_capabilities_id: branch.branch_capabilities_id,
            
            // Security and access - normalized to UUID reference
            security_access_id: branch.security_access_id,
            
            // Customer capacity
            max_daily_customers: branch.max_daily_customers,
            average_wait_time_minutes: branch.average_wait_time_minutes,
            
            // Transaction limits (enhanced from existing)
            per_transaction_limit: branch.per_transaction_limit,
            monthly_transaction_limit: branch.monthly_transaction_limit,
            
            // Compliance and risk
            risk_rating: Self::branch_risk_rating_to_db(branch.risk_rating),
            last_audit_date: branch.last_audit_date,
            last_compliance_certification_id: branch.last_compliance_certification_id, // UUID reference to compliance cert
            
            // Metadata
            last_updated_at: branch.last_updated_at,
            updated_by_person_id: branch.updated_by_person_id,
        }
    }

    /// Map from database AgencyBranchModel to domain AgencyBranch
    pub fn branch_from_model(model: AgencyBranchModel) -> AgencyBranch {
        AgencyBranch {
            id: model.id,
            agent_network_id: model.agent_network_id,
            parent_agency_branch_id: model.parent_agency_branch_id,
            branch_name: model.branch_name,
            branch_code: model.branch_code,
            branch_level: model.branch_level,
            gl_code_prefix: model.gl_code_prefix,
            status: Self::branch_status_from_db(model.status),
            daily_transaction_limit: model.daily_transaction_limit,
            current_daily_volume: model.current_daily_volume,
            max_cash_limit: model.max_cash_limit,
            current_cash_balance: model.current_cash_balance,
            minimum_cash_balance: model.minimum_cash_balance,
            created_at: model.created_at,
            
            // Location fields
            location_id: model.location_id,
            landmark_description: model.landmark_description,
            
            // Operational details
            operating_hours_id: model.operating_hours_id,
            holiday_plan_id: model.holiday_plan_id,
            temporary_closure_id: model.temporary_closure_id,
            
            // Contact information - individual messaging fields
            messaging1_id: model.messaging1_id,
            messaging1_type: model.messaging1_type.map(Self::messaging_type_from_db),
            messaging2_id: model.messaging2_id,
            messaging2_type: model.messaging2_type.map(Self::messaging_type_from_db),
            messaging3_id: model.messaging3_id,
            messaging3_type: model.messaging3_type.map(Self::messaging_type_from_db),
            messaging4_id: model.messaging4_id,
            messaging4_type: model.messaging4_type.map(Self::messaging_type_from_db),
            messaging5_id: model.messaging5_id,
            messaging5_type: model.messaging5_type.map(Self::messaging_type_from_db),
            branch_manager_person_id: model.branch_manager_person_id,
            
            // Services and capabilities
            branch_type: Self::branch_type_from_db(model.branch_type),
            branch_capabilities_id: model.branch_capabilities_id,
            
            // Security and access
            security_access_id: model.security_access_id,
            
            // Customer capacity
            max_daily_customers: model.max_daily_customers,
            average_wait_time_minutes: model.average_wait_time_minutes,
            
            // Transaction limits
            per_transaction_limit: model.per_transaction_limit,
            monthly_transaction_limit: model.monthly_transaction_limit,
            
            // Compliance and risk
            risk_rating: Self::branch_risk_rating_from_db(model.risk_rating),
            last_audit_date: model.last_audit_date,
            last_compliance_certification_id: model.last_compliance_certification_id,
            
            // Metadata
            last_updated_at: model.last_updated_at,
            updated_by_person_id: model.updated_by_person_id,
        }
    }

    /// Map from domain AgentTerminal to database AgentTerminalModel
    pub fn terminal_to_model(terminal: AgentTerminal) -> AgentTerminalModel {
        AgentTerminalModel {
            id: terminal.id,
            agency_branch_id: terminal.agency_branch_id,
            agent_person_id: terminal.agent_person_id,
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
            updated_by_person_id: Uuid::nil(), // System UUID
        }
    }

    /// Map from database AgentTerminalModel to domain AgentTerminal
    pub fn terminal_from_model(model: AgentTerminalModel) -> AgentTerminal {
        AgentTerminal {
            id: model.id,
            agency_branch_id: model.agency_branch_id,
            agent_person_id: model.agent_person_id,
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
            BranchType::AtmLocation => DbBranchType::AtmLocation,
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
            DbBranchType::AtmLocation => BranchType::AtmLocation,
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

    fn messaging_type_to_db(messaging_type: DomainMessagingType) -> DbMessagingType {
        match messaging_type {
            DomainMessagingType::Email => DbMessagingType::Email,
            DomainMessagingType::Phone => DbMessagingType::Phone,
            DomainMessagingType::Sms => DbMessagingType::Sms,
            DomainMessagingType::WhatsApp => DbMessagingType::WhatsApp,
            DomainMessagingType::Telegram => DbMessagingType::Telegram,
            DomainMessagingType::Skype => DbMessagingType::Skype,
            DomainMessagingType::Teams => DbMessagingType::Teams,
            DomainMessagingType::Signal => DbMessagingType::Signal,
            DomainMessagingType::WeChat => DbMessagingType::WeChat,
            DomainMessagingType::Viber => DbMessagingType::Viber,
            DomainMessagingType::Messenger => DbMessagingType::Messenger,
            DomainMessagingType::LinkedIn => DbMessagingType::LinkedIn,
            DomainMessagingType::Slack => DbMessagingType::Slack,
            DomainMessagingType::Discord => DbMessagingType::Discord,
            DomainMessagingType::Other => DbMessagingType::Other,
        }
    }

    fn messaging_type_from_db(db_type: DbMessagingType) -> DomainMessagingType {
        match db_type {
            DbMessagingType::Email => DomainMessagingType::Email,
            DbMessagingType::Phone => DomainMessagingType::Phone,
            DbMessagingType::Sms => DomainMessagingType::Sms,
            DbMessagingType::WhatsApp => DomainMessagingType::WhatsApp,
            DbMessagingType::Telegram => DomainMessagingType::Telegram,
            DbMessagingType::Skype => DomainMessagingType::Skype,
            DbMessagingType::Teams => DomainMessagingType::Teams,
            DbMessagingType::Signal => DomainMessagingType::Signal,
            DbMessagingType::WeChat => DomainMessagingType::WeChat,
            DbMessagingType::Viber => DomainMessagingType::Viber,
            DbMessagingType::Messenger => DomainMessagingType::Messenger,
            DbMessagingType::LinkedIn => DomainMessagingType::LinkedIn,
            DbMessagingType::Slack => DomainMessagingType::Slack,
            DbMessagingType::Discord => DomainMessagingType::Discord,
            DbMessagingType::Other => DomainMessagingType::Other,
        }
    }

    // New mappers for additional agent network structures

    /// Map from domain HollidayPlan to database HollidayPlanModel
    /// Note: This is a placeholder as domain models for these don't exist yet
    pub fn holiday_plan_to_model(
        id: Uuid,
        name_l1: &str,
        name_l2: Option<&str>,
        name_l3: Option<&str>,
        created_by_person_id: Uuid,
    ) -> HollidayPlanModel {
        use heapless::String as HeaplessString;
        use chrono::Utc;
        
        HollidayPlanModel {
            id,
            name_l1: HeaplessString::try_from(name_l1).unwrap_or_default(),
            name_l2: name_l2.map(|n| HeaplessString::try_from(n).unwrap_or_default()).unwrap_or_default(),
            name_l3: name_l3.map(|n| HeaplessString::try_from(n).unwrap_or_default()).unwrap_or_default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by_person_id,
            updated_by_person_id: created_by_person_id,
        }
    }

    /// Map from database HollidayPlanModel to tuple (for now, until domain model exists)
    pub fn holiday_plan_from_model(model: HollidayPlanModel) -> (Uuid, String, String, String) {
        (
            model.id,
            model.name_l1.to_string(),
            model.name_l2.to_string(),
            model.name_l3.to_string(),
        )
    }

    /// Placeholder mapper for OperatingHoursModel
    pub fn operating_hours_from_model(model: OperatingHoursModel) -> (Uuid, String) {
        (model.id, model.name_l1.to_string())
    }

    /// Placeholder mapper for BranchCapabilitiesModel
    pub fn branch_capabilities_from_model(model: BranchCapabilitiesModel) -> (Uuid, String) {
        (model.id, model.name_l1.to_string())
    }

    /// Placeholder mapper for SecurityAccessModel
    pub fn security_access_from_model(model: SecurityAccessModel) -> (Uuid, String) {
        (model.id, model.name_l1.to_string())
    }

    /// Placeholder mapper for TemporaryClosureModel
    pub fn temporary_closure_from_model(model: TemporaryClosureModel) -> (Uuid, String) {
        (
            model.id,
            model.additional_details_l1.map(|s| s.to_string()).unwrap_or_default(),
        )
    }

    /// Placeholder mapper for RequiredDocumentModel
    pub fn required_document_from_model(model: RequiredDocumentModel) -> (Uuid, String) {
        (model.id, model.document_type_l1.to_string())
    }

    /// Placeholder mapper for ComplianceCertModel
    pub fn compliance_cert_from_model(model: ComplianceCertModel) -> (Uuid, String) {
        (model.id, model.certification_name_l1.to_string())
    }
}
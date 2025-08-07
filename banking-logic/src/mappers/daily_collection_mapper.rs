use banking_api::domain::{
    collateral::AlertSeverity as DomainAlertSeverity,
    daily_collection::{
    CollectionAgent, CollectionProgram, CustomerCollectionProfile, CollectionRecord, CollectionBatch,
    AgentStatus as DomainAgentStatus, AreaType as DomainAreaType, CustomerDensity as DomainCustomerDensity,
    TransportMode as DomainTransportMode, DeviceType as DomainDeviceType, ConnectivityStatus as DomainConnectivityStatus,
    CollectionProgramType as DomainCollectionProgramType, ProgramStatus as DomainProgramStatus,
    CollectionFrequency as DomainCollectionFrequency, CollectionStatus as DomainCollectionStatus,
    HolidayHandling as DomainHolidayHandling, ReliabilityRating as DomainReliabilityRating,
    CollectionMethod as DomainCollectionMethod, CollectionRecordStatus as DomainCollectionRecordStatus,
    BiometricMethod as DomainBiometricMethod, BatchStatus as DomainBatchStatus,
    AlertType as DomainAlertType, FeeFrequency as DomainFeeFrequency,
    CoverageArea, PerformanceAlert,
    },
};

use banking_db::models::{
    collateral::AlertSeverity as DbAlertSeverity,
    daily_collection::{
    CollectionAgentModel, CollectionProgramModel, CustomerCollectionProfileModel, 
    CollectionRecordModel, CollectionBatchModel, CoverageAreaModel, PerformanceAlertModel,
    AgentStatus as DbAgentStatus, AreaType as DbAreaType, CustomerDensity as DbCustomerDensity,
    TransportMode as DbTransportMode, DeviceType as DbDeviceType, ConnectivityStatus as DbConnectivityStatus,
    CollectionProgramType as DbCollectionProgramType, ProgramStatus as DbProgramStatus,
    CollectionFrequency as DbCollectionFrequency, CollectionStatus as DbCollectionStatus,
    HolidayHandling as DbHolidayHandling, ReliabilityRating as DbReliabilityRating,
    CollectionMethod as DbCollectionMethod, CollectionRecordStatus as DbCollectionRecordStatus,
    BiometricMethod as DbBiometricMethod, BatchStatus as DbBatchStatus,
    AlertType as DbAlertType, FeeFrequency as DbFeeFrequency,
    },
};

/// Mapper for converting between domain and database models for Daily Collection entities
pub struct DailyCollectionMapper;

impl DailyCollectionMapper {
    // ======== Enum Mappers ========

    /// Convert domain AgentStatus to database AgentStatus
    pub fn agent_status_to_model(status: DomainAgentStatus) -> DbAgentStatus {
        match status {
            DomainAgentStatus::Active => DbAgentStatus::Active,
            DomainAgentStatus::Suspended => DbAgentStatus::Suspended,
            DomainAgentStatus::Training => DbAgentStatus::Training,
            DomainAgentStatus::OnLeave => DbAgentStatus::OnLeave,
            DomainAgentStatus::Terminated => DbAgentStatus::Terminated,
        }
    }

    /// Convert database AgentStatus to domain AgentStatus
    pub fn agent_status_from_model(model: DbAgentStatus) -> DomainAgentStatus {
        match model {
            DbAgentStatus::Active => DomainAgentStatus::Active,
            DbAgentStatus::Suspended => DomainAgentStatus::Suspended,
            DbAgentStatus::Training => DomainAgentStatus::Training,
            DbAgentStatus::OnLeave => DomainAgentStatus::OnLeave,
            DbAgentStatus::Terminated => DomainAgentStatus::Terminated,
        }
    }

    /// Convert domain AreaType to database AreaType
    pub fn area_type_to_model(area_type: DomainAreaType) -> DbAreaType {
        match area_type {
            DomainAreaType::Urban => DbAreaType::Urban,
            DomainAreaType::Suburban => DbAreaType::Suburban,
            DomainAreaType::Rural => DbAreaType::Rural,
            DomainAreaType::Commercial => DbAreaType::Commercial,
            DomainAreaType::Industrial => DbAreaType::Industrial,
            DomainAreaType::Mixed => DbAreaType::Mixed,
        }
    }

    /// Convert database AreaType to domain AreaType
    pub fn area_type_from_model(model: DbAreaType) -> DomainAreaType {
        match model {
            DbAreaType::Urban => DomainAreaType::Urban,
            DbAreaType::Suburban => DomainAreaType::Suburban,
            DbAreaType::Rural => DomainAreaType::Rural,
            DbAreaType::Commercial => DomainAreaType::Commercial,
            DbAreaType::Industrial => DomainAreaType::Industrial,
            DbAreaType::Mixed => DomainAreaType::Mixed,
        }
    }

    /// Convert domain CustomerDensity to database CustomerDensity
    pub fn customer_density_to_model(density: DomainCustomerDensity) -> DbCustomerDensity {
        match density {
            DomainCustomerDensity::High => DbCustomerDensity::High,
            DomainCustomerDensity::Medium => DbCustomerDensity::Medium,
            DomainCustomerDensity::Low => DbCustomerDensity::Low,
        }
    }

    /// Convert database CustomerDensity to domain CustomerDensity
    pub fn customer_density_from_model(model: DbCustomerDensity) -> DomainCustomerDensity {
        match model {
            DbCustomerDensity::High => DomainCustomerDensity::High,
            DbCustomerDensity::Medium => DomainCustomerDensity::Medium,
            DbCustomerDensity::Low => DomainCustomerDensity::Low,
        }
    }

    /// Convert domain TransportMode to database TransportMode
    pub fn transport_mode_to_model(mode: DomainTransportMode) -> DbTransportMode {
        match mode {
            DomainTransportMode::Walking => DbTransportMode::Walking,
            DomainTransportMode::Bicycle => DbTransportMode::Bicycle,
            DomainTransportMode::Motorcycle => DbTransportMode::Motorcycle,
            DomainTransportMode::Car => DbTransportMode::Car,
            DomainTransportMode::PublicTransport => DbTransportMode::PublicTransport,
            DomainTransportMode::Mixed => DbTransportMode::Mixed,
        }
    }

    /// Convert database TransportMode to domain TransportMode
    pub fn transport_mode_from_model(model: DbTransportMode) -> DomainTransportMode {
        match model {
            DbTransportMode::Walking => DomainTransportMode::Walking,
            DbTransportMode::Bicycle => DomainTransportMode::Bicycle,
            DbTransportMode::Motorcycle => DomainTransportMode::Motorcycle,
            DbTransportMode::Car => DomainTransportMode::Car,
            DbTransportMode::PublicTransport => DomainTransportMode::PublicTransport,
            DbTransportMode::Mixed => DomainTransportMode::Mixed,
        }
    }

    /// Convert domain DeviceType to database DeviceType
    pub fn device_type_to_model(device_type: DomainDeviceType) -> DbDeviceType {
        match device_type {
            DomainDeviceType::Smartphone => DbDeviceType::Smartphone,
            DomainDeviceType::Tablet => DbDeviceType::Tablet,
            DomainDeviceType::PortableTerminal => DbDeviceType::PortableTerminal,
            DomainDeviceType::SmartWatch => DbDeviceType::SmartWatch,
        }
    }

    /// Convert database DeviceType to domain DeviceType
    pub fn device_type_from_model(model: DbDeviceType) -> DomainDeviceType {
        match model {
            DbDeviceType::Smartphone => DomainDeviceType::Smartphone,
            DbDeviceType::Tablet => DomainDeviceType::Tablet,
            DbDeviceType::PortableTerminal => DomainDeviceType::PortableTerminal,
            DbDeviceType::SmartWatch => DomainDeviceType::SmartWatch,
        }
    }

    /// Convert domain ConnectivityStatus to database ConnectivityStatus
    pub fn connectivity_status_to_model(status: DomainConnectivityStatus) -> DbConnectivityStatus {
        match status {
            DomainConnectivityStatus::Online => DbConnectivityStatus::Online,
            DomainConnectivityStatus::Offline => DbConnectivityStatus::Offline,
            DomainConnectivityStatus::LimitedConnectivity => DbConnectivityStatus::LimitedConnectivity,
            DomainConnectivityStatus::SyncPending => DbConnectivityStatus::SyncPending,
        }
    }

    /// Convert database ConnectivityStatus to domain ConnectivityStatus
    pub fn connectivity_status_from_model(model: DbConnectivityStatus) -> DomainConnectivityStatus {
        match model {
            DbConnectivityStatus::Online => DomainConnectivityStatus::Online,
            DbConnectivityStatus::Offline => DomainConnectivityStatus::Offline,
            DbConnectivityStatus::LimitedConnectivity => DomainConnectivityStatus::LimitedConnectivity,
            DbConnectivityStatus::SyncPending => DomainConnectivityStatus::SyncPending,
        }
    }

    /// Convert domain CollectionProgramType to database CollectionProgramType
    pub fn collection_program_type_to_model(program_type: DomainCollectionProgramType) -> DbCollectionProgramType {
        match program_type {
            DomainCollectionProgramType::FixedAmount => DbCollectionProgramType::FixedAmount,
            DomainCollectionProgramType::VariableAmount => DbCollectionProgramType::VariableAmount,
            DomainCollectionProgramType::TargetBased => DbCollectionProgramType::TargetBased,
            DomainCollectionProgramType::DurationBased => DbCollectionProgramType::DurationBased,
        }
    }

    /// Convert database CollectionProgramType to domain CollectionProgramType
    pub fn collection_program_type_from_model(model: DbCollectionProgramType) -> DomainCollectionProgramType {
        match model {
            DbCollectionProgramType::FixedAmount => DomainCollectionProgramType::FixedAmount,
            DbCollectionProgramType::VariableAmount => DomainCollectionProgramType::VariableAmount,
            DbCollectionProgramType::TargetBased => DomainCollectionProgramType::TargetBased,
            DbCollectionProgramType::DurationBased => DomainCollectionProgramType::DurationBased,
        }
    }

    /// Convert domain ProgramStatus to database ProgramStatus
    pub fn program_status_to_model(status: DomainProgramStatus) -> DbProgramStatus {
        match status {
            DomainProgramStatus::Active => DbProgramStatus::Active,
            DomainProgramStatus::Suspended => DbProgramStatus::Suspended,
            DomainProgramStatus::Closed => DbProgramStatus::Closed,
            DomainProgramStatus::UnderReview => DbProgramStatus::UnderReview,
        }
    }

    /// Convert database ProgramStatus to domain ProgramStatus
    pub fn program_status_from_model(model: DbProgramStatus) -> DomainProgramStatus {
        match model {
            DbProgramStatus::Active => DomainProgramStatus::Active,
            DbProgramStatus::Suspended => DomainProgramStatus::Suspended,
            DbProgramStatus::Closed => DomainProgramStatus::Closed,
            DbProgramStatus::UnderReview => DomainProgramStatus::UnderReview,
        }
    }

    /// Convert domain CollectionFrequency to database CollectionFrequency
    pub fn collection_frequency_to_model(frequency: DomainCollectionFrequency) -> DbCollectionFrequency {
        match frequency {
            DomainCollectionFrequency::Daily => DbCollectionFrequency::Daily,
            DomainCollectionFrequency::Weekly => DbCollectionFrequency::Weekly,
            DomainCollectionFrequency::Monthly => DbCollectionFrequency::Monthly,
            DomainCollectionFrequency::Quarterly => DbCollectionFrequency::Quarterly,
            DomainCollectionFrequency::Yearly => DbCollectionFrequency::Yearly,
        }
    }

    /// Convert database CollectionFrequency to domain CollectionFrequency
    pub fn collection_frequency_from_model(model: DbCollectionFrequency) -> DomainCollectionFrequency {
        match model {
            DbCollectionFrequency::Daily => DomainCollectionFrequency::Daily,
            DbCollectionFrequency::Weekly => DomainCollectionFrequency::Weekly,
            DbCollectionFrequency::Monthly => DomainCollectionFrequency::Monthly,
            DbCollectionFrequency::Quarterly => DomainCollectionFrequency::Quarterly,
            DbCollectionFrequency::Yearly => DomainCollectionFrequency::Yearly,
        }
    }

    /// Convert domain CollectionStatus to database CollectionStatus
    pub fn collection_status_to_model(status: DomainCollectionStatus) -> DbCollectionStatus {
        match status {
            DomainCollectionStatus::Active => DbCollectionStatus::Active,
            DomainCollectionStatus::Suspended => DbCollectionStatus::Suspended,
            DomainCollectionStatus::Defaulted => DbCollectionStatus::Defaulted,
            DomainCollectionStatus::Graduated => DbCollectionStatus::Graduated,
            DomainCollectionStatus::Terminated => DbCollectionStatus::Terminated,
        }
    }

    /// Convert database CollectionStatus to domain CollectionStatus
    pub fn collection_status_from_model(model: DbCollectionStatus) -> DomainCollectionStatus {
        match model {
            DbCollectionStatus::Active => DomainCollectionStatus::Active,
            DbCollectionStatus::Suspended => DomainCollectionStatus::Suspended,
            DbCollectionStatus::Defaulted => DomainCollectionStatus::Defaulted,
            DbCollectionStatus::Graduated => DomainCollectionStatus::Graduated,
            DbCollectionStatus::Terminated => DomainCollectionStatus::Terminated,
        }
    }

    /// Convert domain HolidayHandling to database HolidayHandling
    pub fn holiday_handling_to_model(handling: DomainHolidayHandling) -> DbHolidayHandling {
        match handling {
            DomainHolidayHandling::Skip => DbHolidayHandling::Skip,
            DomainHolidayHandling::NextBusinessDay => DbHolidayHandling::NextBusinessDay,
            DomainHolidayHandling::PreviousBusinessDay => DbHolidayHandling::PreviousBusinessDay,
            DomainHolidayHandling::CollectDouble => DbHolidayHandling::CollectDouble,
        }
    }

    /// Convert database HolidayHandling to domain HolidayHandling
    pub fn holiday_handling_from_model(model: DbHolidayHandling) -> DomainHolidayHandling {
        match model {
            DbHolidayHandling::Skip => DomainHolidayHandling::Skip,
            DbHolidayHandling::NextBusinessDay => DomainHolidayHandling::NextBusinessDay,
            DbHolidayHandling::PreviousBusinessDay => DomainHolidayHandling::PreviousBusinessDay,
            DbHolidayHandling::CollectDouble => DomainHolidayHandling::CollectDouble,
        }
    }

    /// Convert domain ReliabilityRating to database ReliabilityRating
    pub fn reliability_rating_to_model(rating: DomainReliabilityRating) -> DbReliabilityRating {
        match rating {
            DomainReliabilityRating::Excellent => DbReliabilityRating::Excellent,
            DomainReliabilityRating::Good => DbReliabilityRating::Good,
            DomainReliabilityRating::Fair => DbReliabilityRating::Fair,
            DomainReliabilityRating::Poor => DbReliabilityRating::Poor,
            DomainReliabilityRating::Critical => DbReliabilityRating::Critical,
        }
    }

    /// Convert database ReliabilityRating to domain ReliabilityRating
    pub fn reliability_rating_from_model(model: DbReliabilityRating) -> DomainReliabilityRating {
        match model {
            DbReliabilityRating::Excellent => DomainReliabilityRating::Excellent,
            DbReliabilityRating::Good => DomainReliabilityRating::Good,
            DbReliabilityRating::Fair => DomainReliabilityRating::Fair,
            DbReliabilityRating::Poor => DomainReliabilityRating::Poor,
            DbReliabilityRating::Critical => DomainReliabilityRating::Critical,
        }
    }

    /// Convert domain CollectionMethod to database CollectionMethod
    pub fn collection_method_to_model(method: DomainCollectionMethod) -> DbCollectionMethod {
        match method {
            DomainCollectionMethod::Cash => DbCollectionMethod::Cash,
            DomainCollectionMethod::MobilePayment => DbCollectionMethod::MobilePayment,
            DomainCollectionMethod::BankTransfer => DbCollectionMethod::BankTransfer,
            DomainCollectionMethod::DigitalWallet => DbCollectionMethod::DigitalWallet,
        }
    }

    /// Convert database CollectionMethod to domain CollectionMethod
    pub fn collection_method_from_model(model: DbCollectionMethod) -> DomainCollectionMethod {
        match model {
            DbCollectionMethod::Cash => DomainCollectionMethod::Cash,
            DbCollectionMethod::MobilePayment => DomainCollectionMethod::MobilePayment,
            DbCollectionMethod::BankTransfer => DomainCollectionMethod::BankTransfer,
            DbCollectionMethod::DigitalWallet => DomainCollectionMethod::DigitalWallet,
        }
    }

    /// Convert domain CollectionRecordStatus to database CollectionRecordStatus
    pub fn collection_record_status_to_model(status: DomainCollectionRecordStatus) -> DbCollectionRecordStatus {
        match status {
            DomainCollectionRecordStatus::Pending => DbCollectionRecordStatus::Pending,
            DomainCollectionRecordStatus::Processed => DbCollectionRecordStatus::Processed,
            DomainCollectionRecordStatus::Failed => DbCollectionRecordStatus::Failed,
            DomainCollectionRecordStatus::Reversed => DbCollectionRecordStatus::Reversed,
            DomainCollectionRecordStatus::UnderReview => DbCollectionRecordStatus::UnderReview,
        }
    }

    /// Convert database CollectionRecordStatus to domain CollectionRecordStatus
    pub fn collection_record_status_from_model(model: DbCollectionRecordStatus) -> DomainCollectionRecordStatus {
        match model {
            DbCollectionRecordStatus::Pending => DomainCollectionRecordStatus::Pending,
            DbCollectionRecordStatus::Processed => DomainCollectionRecordStatus::Processed,
            DbCollectionRecordStatus::Failed => DomainCollectionRecordStatus::Failed,
            DbCollectionRecordStatus::Reversed => DomainCollectionRecordStatus::Reversed,
            DbCollectionRecordStatus::UnderReview => DomainCollectionRecordStatus::UnderReview,
        }
    }

    /// Convert domain BiometricMethod to database BiometricMethod
    pub fn biometric_method_to_model(method: DomainBiometricMethod) -> DbBiometricMethod {
        match method {
            DomainBiometricMethod::Fingerprint => DbBiometricMethod::Fingerprint,
            DomainBiometricMethod::FaceRecognition => DbBiometricMethod::FaceRecognition,
            DomainBiometricMethod::VoicePrint => DbBiometricMethod::VoicePrint,
            DomainBiometricMethod::Combined => DbBiometricMethod::Combined,
        }
    }

    /// Convert database BiometricMethod to domain BiometricMethod
    pub fn biometric_method_from_model(model: DbBiometricMethod) -> DomainBiometricMethod {
        match model {
            DbBiometricMethod::Fingerprint => DomainBiometricMethod::Fingerprint,
            DbBiometricMethod::FaceRecognition => DomainBiometricMethod::FaceRecognition,
            DbBiometricMethod::VoicePrint => DomainBiometricMethod::VoicePrint,
            DbBiometricMethod::Combined => DomainBiometricMethod::Combined,
        }
    }

    /// Convert domain BatchStatus to database BatchStatus
    pub fn batch_status_to_model(status: DomainBatchStatus) -> DbBatchStatus {
        match status {
            DomainBatchStatus::Pending => DbBatchStatus::Pending,
            DomainBatchStatus::Processing => DbBatchStatus::Processing,
            DomainBatchStatus::Completed => DbBatchStatus::Completed,
            DomainBatchStatus::Failed => DbBatchStatus::Failed,
            DomainBatchStatus::PartiallyProcessed => DbBatchStatus::PartiallyProcessed,
            DomainBatchStatus::RequiresReconciliation => DbBatchStatus::RequiresReconciliation,
        }
    }

    /// Convert database BatchStatus to domain BatchStatus
    pub fn batch_status_from_model(model: DbBatchStatus) -> DomainBatchStatus {
        match model {
            DbBatchStatus::Pending => DomainBatchStatus::Pending,
            DbBatchStatus::Processing => DomainBatchStatus::Processing,
            DbBatchStatus::Completed => DomainBatchStatus::Completed,
            DbBatchStatus::Failed => DomainBatchStatus::Failed,
            DbBatchStatus::PartiallyProcessed => DomainBatchStatus::PartiallyProcessed,
            DbBatchStatus::RequiresReconciliation => DomainBatchStatus::RequiresReconciliation,
        }
    }

    /// Convert domain AlertType to database AlertType
    pub fn alert_type_to_model(alert_type: DomainAlertType) -> DbAlertType {
        match alert_type {
            DomainAlertType::LowCollectionRate => DbAlertType::LowCollectionRate,
            DomainAlertType::CustomerComplaint => DbAlertType::CustomerComplaint,
            DomainAlertType::CashDiscrepancy => DbAlertType::CashDiscrepancy,
            DomainAlertType::MissedSchedule => DbAlertType::MissedSchedule,
            DomainAlertType::ComplianceViolation => DbAlertType::ComplianceViolation,
            DomainAlertType::SafetyConcern => DbAlertType::SafetyConcern,
            DomainAlertType::DeviceIssue => DbAlertType::DeviceIssue,
        }
    }

    /// Convert database AlertType to domain AlertType
    pub fn alert_type_from_model(model: DbAlertType) -> DomainAlertType {
        match model {
            DbAlertType::LowCollectionRate => DomainAlertType::LowCollectionRate,
            DbAlertType::CustomerComplaint => DomainAlertType::CustomerComplaint,
            DbAlertType::CashDiscrepancy => DomainAlertType::CashDiscrepancy,
            DbAlertType::MissedSchedule => DomainAlertType::MissedSchedule,
            DbAlertType::ComplianceViolation => DomainAlertType::ComplianceViolation,
            DbAlertType::SafetyConcern => DomainAlertType::SafetyConcern,
            DbAlertType::DeviceIssue => DomainAlertType::DeviceIssue,
        }
    }

    /// Convert domain FeeFrequency to database FeeFrequency
    pub fn fee_frequency_to_model(frequency: DomainFeeFrequency) -> DbFeeFrequency {
        match frequency {
            DomainFeeFrequency::PerCollection => DbFeeFrequency::PerCollection,
            DomainFeeFrequency::Daily => DbFeeFrequency::Daily,
            DomainFeeFrequency::Weekly => DbFeeFrequency::Weekly,
            DomainFeeFrequency::Monthly => DbFeeFrequency::Monthly,
            DomainFeeFrequency::OneTime => DbFeeFrequency::OneTime,
        }
    }

    /// Convert database FeeFrequency to domain FeeFrequency
    pub fn fee_frequency_from_model(model: DbFeeFrequency) -> DomainFeeFrequency {
        match model {
            DbFeeFrequency::PerCollection => DomainFeeFrequency::PerCollection,
            DbFeeFrequency::Daily => DomainFeeFrequency::Daily,
            DbFeeFrequency::Weekly => DomainFeeFrequency::Weekly,
            DbFeeFrequency::Monthly => DomainFeeFrequency::Monthly,
            DbFeeFrequency::OneTime => DomainFeeFrequency::OneTime,
        }
    }

    /// Convert domain AlertSeverity to database AlertSeverity
    pub fn alert_severity_to_model(severity: DomainAlertSeverity) -> DbAlertSeverity {
        match severity {
            DomainAlertSeverity::Low => DbAlertSeverity::Low,
            DomainAlertSeverity::Medium => DbAlertSeverity::Medium,
            DomainAlertSeverity::High => DbAlertSeverity::High,
            DomainAlertSeverity::Critical => DbAlertSeverity::Critical,
        }
    }

    /// Convert database AlertSeverity to domain AlertSeverity
    pub fn alert_severity_from_model(model: DbAlertSeverity) -> DomainAlertSeverity {
        match model {
            DbAlertSeverity::Low => DomainAlertSeverity::Low,
            DbAlertSeverity::Medium => DomainAlertSeverity::Medium,
            DbAlertSeverity::High => DomainAlertSeverity::High,
            DbAlertSeverity::Critical => DomainAlertSeverity::Critical,
        }
    }

    // ======== Model Mappers ========

    /// Convert a vector of domain models to database models
    pub fn collection_agents_to_models(agents: Vec<CollectionAgent>) -> Vec<CollectionAgentModel> {
        agents.into_iter().map(Self::collection_agent_to_model).collect()
    }

    /// Convert a vector of database models to domain models
    pub fn collection_agents_from_models(models: Vec<CollectionAgentModel>) -> Vec<CollectionAgent> {
        models.into_iter().map(Self::collection_agent_from_model).collect()
    }

    /// Convert a vector of domain models to database models
    pub fn collection_programs_to_models(programs: Vec<CollectionProgram>) -> Vec<CollectionProgramModel> {
        programs.into_iter().map(Self::collection_program_to_model).collect()
    }

    /// Convert a vector of database models to domain models
    pub fn collection_programs_from_models(models: Vec<CollectionProgramModel>) -> Vec<CollectionProgram> {
        models.into_iter().map(Self::collection_program_from_model).collect()
    }

    /// Convert a vector of domain models to database models
    pub fn customer_collection_profiles_to_models(profiles: Vec<CustomerCollectionProfile>) -> Vec<CustomerCollectionProfileModel> {
        profiles.into_iter().map(Self::customer_collection_profile_to_model).collect()
    }

    /// Convert a vector of database models to domain models
    pub fn customer_collection_profiles_from_models(models: Vec<CustomerCollectionProfileModel>) -> Vec<CustomerCollectionProfile> {
        models.into_iter().map(Self::customer_collection_profile_from_model).collect()
    }

    /// Convert a vector of domain models to database models
    pub fn collection_records_to_models(records: Vec<CollectionRecord>) -> Vec<CollectionRecordModel> {
        records.into_iter().map(Self::collection_record_to_model).collect()
    }

    /// Convert a vector of database models to domain models
    pub fn collection_records_from_models(models: Vec<CollectionRecordModel>) -> Vec<CollectionRecord> {
        models.into_iter().map(Self::collection_record_from_model).collect()
    }

    /// Convert a vector of domain models to database models
    pub fn collection_batches_to_models(batches: Vec<CollectionBatch>) -> Vec<CollectionBatchModel> {
        batches.into_iter().map(Self::collection_batch_to_model).collect()
    }

    /// Convert a vector of database models to domain models
    pub fn collection_batches_from_models(models: Vec<CollectionBatchModel>) -> Vec<CollectionBatch> {
        models.into_iter().map(Self::collection_batch_from_model).collect()
    }

    // ======== Individual Model Mappers (TODO: Implement) ========
    
    /// Convert domain CollectionAgent to database CollectionAgentModel
    pub fn collection_agent_to_model(_agent: CollectionAgent) -> CollectionAgentModel {
        todo!("Implement collection_agent_to_model")
    }

    /// Convert database CollectionAgentModel to domain CollectionAgent
    pub fn collection_agent_from_model(_model: CollectionAgentModel) -> CollectionAgent {
        todo!("Implement collection_agent_from_model")
    }

    /// Convert domain CollectionProgram to database CollectionProgramModel
    pub fn collection_program_to_model(_program: CollectionProgram) -> CollectionProgramModel {
        todo!("Implement collection_program_to_model")
    }

    /// Convert database CollectionProgramModel to domain CollectionProgram
    pub fn collection_program_from_model(_model: CollectionProgramModel) -> CollectionProgram {
        todo!("Implement collection_program_from_model")
    }

    /// Convert domain CustomerCollectionProfile to database CustomerCollectionProfileModel
    pub fn customer_collection_profile_to_model(_profile: CustomerCollectionProfile) -> CustomerCollectionProfileModel {
        todo!("Implement customer_collection_profile_to_model")
    }

    /// Convert database CustomerCollectionProfileModel to domain CustomerCollectionProfile
    pub fn customer_collection_profile_from_model(_model: CustomerCollectionProfileModel) -> CustomerCollectionProfile {
        todo!("Implement customer_collection_profile_from_model")
    }

    /// Convert domain CollectionRecord to database CollectionRecordModel
    pub fn collection_record_to_model(_record: CollectionRecord) -> CollectionRecordModel {
        todo!("Implement collection_record_to_model")
    }

    /// Convert database CollectionRecordModel to domain CollectionRecord
    pub fn collection_record_from_model(_model: CollectionRecordModel) -> CollectionRecord {
        todo!("Implement collection_record_from_model")
    }

    /// Convert domain CollectionBatch to database CollectionBatchModel
    pub fn collection_batch_to_model(_batch: CollectionBatch) -> CollectionBatchModel {
        todo!("Implement collection_batch_to_model")
    }

    /// Convert database CollectionBatchModel to domain CollectionBatch
    pub fn collection_batch_from_model(_model: CollectionBatchModel) -> CollectionBatch {
        todo!("Implement collection_batch_from_model")
    }

    /// Convert domain CoverageArea to database CoverageAreaModel
    pub fn coverage_area_to_model(_area: CoverageArea) -> CoverageAreaModel {
        todo!("Implement coverage_area_to_model")
    }

    /// Convert database CoverageAreaModel to domain CoverageArea
    pub fn coverage_area_from_model(_model: CoverageAreaModel) -> CoverageArea {
        todo!("Implement coverage_area_from_model")
    }

    /// Convert domain PerformanceAlert to database PerformanceAlertModel
    pub fn performance_alert_to_model(_alert: PerformanceAlert) -> PerformanceAlertModel {
        todo!("Implement performance_alert_to_model")
    }

    /// Convert database PerformanceAlertModel to domain PerformanceAlert
    pub fn performance_alert_from_model(_model: PerformanceAlertModel) -> PerformanceAlert {
        todo!("Implement performance_alert_from_model")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_status_mapping() {
        let domain_statuses = vec![
            DomainAgentStatus::Active,
            DomainAgentStatus::Suspended,
            DomainAgentStatus::Training,
            DomainAgentStatus::OnLeave,
            DomainAgentStatus::Terminated,
        ];

        for domain_status in domain_statuses {
            let model_status = DailyCollectionMapper::agent_status_to_model(domain_status.clone());
            let back_to_domain = DailyCollectionMapper::agent_status_from_model(model_status);
            assert_eq!(domain_status, back_to_domain);
        }
    }

    #[test]
    fn test_area_type_mapping() {
        let domain_types = vec![
            DomainAreaType::Urban,
            DomainAreaType::Suburban,
            DomainAreaType::Rural,
            DomainAreaType::Commercial,
            DomainAreaType::Industrial,
            DomainAreaType::Mixed,
        ];

        for domain_type in domain_types {
            let model_type = DailyCollectionMapper::area_type_to_model(domain_type.clone());
            let back_to_domain = DailyCollectionMapper::area_type_from_model(model_type);
            assert_eq!(domain_type, back_to_domain);
        }
    }

    #[test]
    fn test_collection_status_mapping() {
        let domain_statuses = vec![
            DomainCollectionStatus::Active,
            DomainCollectionStatus::Suspended,
            DomainCollectionStatus::Defaulted,
            DomainCollectionStatus::Graduated,
            DomainCollectionStatus::Terminated,
        ];

        for domain_status in domain_statuses {
            let model_status = DailyCollectionMapper::collection_status_to_model(domain_status.clone());
            let back_to_domain = DailyCollectionMapper::collection_status_from_model(model_status);
            assert_eq!(domain_status, back_to_domain);
        }
    }
}
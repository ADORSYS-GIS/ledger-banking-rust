use banking_api::domain::{
    collateral::AlertSeverity as DomainAlertSeverity,
    daily_collection::{
        AgentStatus as DomainAgentStatus, CollectionAlertType as DomainAlertType,
        AreaType as DomainAreaType, BatchStatus as DomainBatchStatus, BiometricData,
        BiometricMethod as DomainBiometricMethod, CollectionAgent, CollectionBatch,
        CollectionFrequency as DomainCollectionFrequency, CollectionMethod as DomainCollectionMethod,
        CollectionProgram, CollectionProgramType as DomainCollectionProgramType,
        CollectionRecord, CollectionRecordStatus as DomainCollectionRecordStatus,
        CollectionSchedule, CollectionStatus as DomainCollectionStatus,
        CollectionVerification, ConnectivityStatus as DomainConnectivityStatus, CoverageArea,
        CustomerCollectionProfile, CustomerDensity as DomainCustomerDensity,
        DeviceType as DomainDeviceType, CollectionFeeFrequency as DomainFeeFrequency,
        FeeStructure, GraduationCriteria, GraduationProgress,
        HolidayHandling as DomainHolidayHandling, PerformanceAlert,
        CollectionPerformanceMetrics, PhotoEvidence, ProgramStatus as DomainProgramStatus,
        ReliabilityRating as DomainReliabilityRating,
        TransportMode as DomainTransportMode, WitnessInformation,
    },
};
use banking_db::models::{
    collateral::AlertSeverity as DbAlertSeverity,
    daily_collection::{
        AgentStatus as DbAgentStatus, AreaType as DbAreaType,
        BatchStatus as DbBatchStatus, BiometricMethod as DbBiometricMethod,
        CollectionAgentModel, CollectionBatchModel, CollectionFrequency as DbCollectionFrequency,
        CollectionMethod as DbCollectionMethod, CollectionProgramModel,
        CollectionProgramType as DbCollectionProgramType, CollectionRecordModel,
        CollectionRecordStatus as DbCollectionRecordStatus, CollectionStatus as DbCollectionStatus,
        ConnectivityStatus as DbConnectivityStatus, CoverageAreaModel,
        CustomerCollectionProfileModel, CustomerDensity as DbCustomerDensity,
        DeviceType as DbDeviceType, HolidayHandling as DbHolidayHandling,
        PerformanceAlertModel, ProgramStatus as DbProgramStatus,
        ReliabilityRating as DbReliabilityRating,
        TransportMode as DbTransportMode, AlertType as DbAlertType,
        FeeFrequency as DbFeeFrequency,
    },
};
use heapless::String as HeaplessString;

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

    /// Convert a vector of database models to domain models
    pub fn collection_agents_from_models(models: Vec<CollectionAgentModel>) -> Vec<CollectionAgent> {
        models
            .into_iter()
            .map(Self::collection_agent_from_model)
            .collect()
    }

    /// Convert a vector of database models to domain models
    pub fn collection_programs_from_models(
        models: Vec<CollectionProgramModel>,
    ) -> Vec<CollectionProgram> {
        models
            .into_iter()
            .map(Self::collection_program_from_model)
            .collect()
    }

    /// Convert a vector of database models to domain models
    pub fn customer_collection_profiles_from_models(
        models: Vec<CustomerCollectionProfileModel>,
    ) -> Vec<CustomerCollectionProfile> {
        models
            .into_iter()
            .map(Self::customer_collection_profile_from_model)
            .collect()
    }

    /// Convert a vector of database models to domain models
    pub fn collection_records_from_models(
        models: Vec<CollectionRecordModel>,
    ) -> Vec<CollectionRecord> {
        models
            .into_iter()
            .map(Self::collection_record_from_model)
            .collect()
    }

    /// Convert a vector of database models to domain models
    pub fn collection_batches_from_models(
        models: Vec<CollectionBatchModel>,
    ) -> Vec<CollectionBatch> {
        models
            .into_iter()
            .map(Self::collection_batch_from_model)
            .collect()
    }

    // ======== Individual Model Mappers ========

    /// Convert domain CollectionAgent to database CollectionAgentModel
    pub fn collection_agent_to_model(agent: &CollectionAgent) -> CollectionAgentModel {
        CollectionAgentModel {
            id: agent.id,
            person_id: agent.person_id,
            license_number: agent.license_number.to_string(),
            license_expiry: agent.license_expiry,
            status: Self::agent_status_to_model(agent.status.clone()),
            assigned_territory_id: agent.assigned_territory_id,
            agent_performance_metrics_id: agent.agent_performance_metrics_id,
            cash_limit: agent.cash_limit,
            device_information_id: agent.device_information_id,
            created_at: agent.created_at,
            updated_at: agent.updated_at,
        }
    }

   /// Convert database CollectionAgentModel to domain CollectionAgent
   pub fn collection_agent_from_model(model: CollectionAgentModel) -> CollectionAgent {
       CollectionAgent {
           id: model.id,
           person_id: model.person_id,
           license_number: HeaplessString::try_from(model.license_number.as_str()).unwrap_or_default(),
           license_expiry: model.license_expiry,
           status: Self::agent_status_from_model(model.status),
           assigned_territory_id: model.assigned_territory_id,
           agent_performance_metrics_id: model.agent_performance_metrics_id,
           cash_limit: model.cash_limit,
           device_information_id: model.device_information_id,
           created_at: model.created_at,
           updated_at: model.updated_at,
       }
   }

   /// Convert domain CollectionProgram to database CollectionProgramModel
   pub fn collection_program_to_model(
       program: &CollectionProgram,
       graduation_criteria: &GraduationCriteria,
       fee_structure: &FeeStructure,
   ) -> CollectionProgramModel {
       CollectionProgramModel {
           id: program.id,
           name: program.name.to_string(),
           description: program.description.to_string(),
           program_type: Self::collection_program_type_to_model(program.program_type.clone()),
           status: Self::program_status_to_model(program.status.clone()),
           start_date: program.start_date,
           end_date: program.end_date,
           collection_frequency: Self::collection_frequency_to_model(
               program.collection_frequency.clone(),
           ),
           operating_hours_id: program.operating_hours_id,
           minimum_amount: program.minimum_amount,
           maximum_amount: program.maximum_amount,
           target_amount: program.target_amount,
           program_duration_days: program.program_duration_days,
           graduation_minimum_balance: graduation_criteria.minimum_balance,
           graduation_minimum_collection_rate: graduation_criteria.minimum_collection_rate,
           graduation_minimum_duration_days: graduation_criteria.minimum_duration_days,
           graduation_consecutive_collections_required: graduation_criteria
               .consecutive_collections_required,
           graduation_target_achievement_required: graduation_criteria
               .target_achievement_required,
           graduation_auto_graduation_enabled: graduation_criteria.auto_graduation_enabled,
           fee_setup_fee: fee_structure.setup_fee,
           fee_collection_fee: fee_structure.collection_fee,
           fee_maintenance_fee: fee_structure.maintenance_fee,
           fee_graduation_fee: fee_structure.graduation_fee,
           fee_early_termination_fee: fee_structure.early_termination_fee,
           fee_frequency: Self::fee_frequency_to_model(fee_structure.fee_frequency.clone()),
           interest_rate: program.interest_rate,
           created_at: program.created_at,
           updated_at: program.updated_at,
           created_by_person_id: program.created_by_person_id,
           reason_id: program.reason_id,
       }
   }

   /// Convert database CollectionProgramModel to domain CollectionProgram
   pub fn collection_program_from_model(model: CollectionProgramModel) -> CollectionProgram {
       CollectionProgram {
           id: model.id,
           name: HeaplessString::try_from(model.name.as_str()).unwrap_or_default(),
           description: HeaplessString::try_from(model.description.as_str()).unwrap_or_default(),
           program_type: Self::collection_program_type_from_model(model.program_type),
           status: Self::program_status_from_model(model.status),
           start_date: model.start_date,
           end_date: model.end_date,
           collection_frequency: Self::collection_frequency_from_model(model.collection_frequency),
           operating_hours_id: model.operating_hours_id,
           minimum_amount: model.minimum_amount,
           maximum_amount: model.maximum_amount,
           target_amount: model.target_amount,
           program_duration_days: model.program_duration_days,
           graduation_criteria_id: model.id,
           fee_structure_id: model.id,
           interest_rate: model.interest_rate,
           created_at: model.created_at,
           updated_at: model.updated_at,
           created_by_person_id: model.created_by_person_id,
           reason_id: model.reason_id,
       }
   }

   /// Convert domain CustomerCollectionProfile to database CustomerCollectionProfileModel
   pub fn customer_collection_profile_to_model(profile: &CustomerCollectionProfile) -> CustomerCollectionProfileModel {
       CustomerCollectionProfileModel {
           id: profile.id,
           customer_id: profile.customer_id,
           collection_program_id: profile.collection_program_id,
           account_id: profile.account_id,
           enrollment_date: profile.enrollment_date,
           status: Self::collection_status_to_model(profile.status.clone()),
           daily_amount: profile.daily_amount,
           assigned_collection_agent_id: profile.assigned_collection_agent_id,
           collection_location_address_id: profile.collection_location_address_id,
           schedule_frequency: Self::collection_frequency_to_model(profile.collection_schedule.frequency.clone()),
            schedule_collection_time: profile.collection_schedule.collection_time,
            schedule_timezone: profile.collection_schedule.timezone.to_string(),
            schedule_holiday_handling: Self::holiday_handling_to_model(profile.collection_schedule.holiday_handling.clone()),
            performance_collection_rate: profile.collection_performance_metrics.collection_rate,
            performance_total_collections: profile.collection_performance_metrics.total_collections,
            performance_total_amount_collected: profile.collection_performance_metrics.total_amount_collected,
            performance_average_collection_amount: profile.collection_performance_metrics.average_collection_amount,
            performance_consecutive_collections: profile.collection_performance_metrics.consecutive_collections,
            performance_missed_collections: profile.collection_performance_metrics.missed_collections,
            performance_last_collection_date: profile.collection_performance_metrics.last_collection_date,
            performance_score: profile.collection_performance_metrics.performance_score,
            performance_reliability_rating: Self::reliability_rating_to_model(profile.collection_performance_metrics.reliability_rating.clone()),
            graduation_current_balance: profile.graduation_progress.current_balance,
            graduation_target_balance: profile.graduation_progress.target_balance,
            graduation_days_in_program: profile.graduation_progress.days_in_program,
            graduation_minimum_days_required: profile.graduation_progress.minimum_days_required,
            graduation_collection_consistency_rate: profile.graduation_progress.collection_consistency_rate,
            graduation_minimum_consistency_required: profile.graduation_progress.minimum_consistency_required,
            graduation_eligible: profile.graduation_progress.graduation_eligible,
            graduation_date: profile.graduation_progress.graduation_date,
            graduation_next_review_date: profile.graduation_progress.next_review_date,
            created_at: profile.created_at,
           updated_at: profile.updated_at,
           reason_id: profile.reason_id,
       }
   }

   /// Convert database CustomerCollectionProfileModel to domain CustomerCollectionProfile
   pub fn customer_collection_profile_from_model(model: CustomerCollectionProfileModel) -> CustomerCollectionProfile {
        CustomerCollectionProfile {
            id: model.id,
            customer_id: model.customer_id,
            collection_program_id: model.collection_program_id,
            account_id: model.account_id,
            enrollment_date: model.enrollment_date,
            status: Self::collection_status_from_model(model.status),
            daily_amount: model.daily_amount,
            collection_schedule: CollectionSchedule {
                id: model.id,
                frequency: Self::collection_frequency_from_model(model.schedule_frequency),
                collection_time: model.schedule_collection_time,
                timezone: HeaplessString::try_from(model.schedule_timezone.as_str()).unwrap_or_default(),
                holiday_handling: Self::holiday_handling_from_model(model.schedule_holiday_handling),
            },
            assigned_collection_agent_id: model.assigned_collection_agent_id,
            collection_location_address_id: model.collection_location_address_id,
            collection_performance_metrics: CollectionPerformanceMetrics {
                id: model.id,
                collection_rate: model.performance_collection_rate,
                total_collections: model.performance_total_collections,
                total_amount_collected: model.performance_total_amount_collected,
                average_collection_amount: model.performance_average_collection_amount,
                consecutive_collections: model.performance_consecutive_collections,
                missed_collections: model.performance_missed_collections,
                last_collection_date: model.performance_last_collection_date,
                performance_score: model.performance_score,
                reliability_rating: Self::reliability_rating_from_model(model.performance_reliability_rating),
            },
            graduation_progress: GraduationProgress {
                id: model.id,
                customer_collection_profile_id: model.id,
                current_balance: model.graduation_current_balance,
                target_balance: model.graduation_target_balance,
                days_in_program: model.graduation_days_in_program,
                minimum_days_required: model.graduation_minimum_days_required,
                collection_consistency_rate: model.graduation_collection_consistency_rate,
                minimum_consistency_required: model.graduation_minimum_consistency_required,
                graduation_eligible: model.graduation_eligible,
                graduation_date: model.graduation_date,
                next_review_date: model.graduation_next_review_date,
            },
            created_at: model.created_at,
            updated_at: model.updated_at,
            reason_id: model.reason_id,
        }
    }

   /// Convert domain CollectionRecord to database CollectionRecordModel
   pub fn collection_record_to_model(record: &CollectionRecord, verification: Option<&CollectionVerification>, biometric: Option<&BiometricData>, photo: Option<&PhotoEvidence>, witness: Option<&WitnessInformation>) -> CollectionRecordModel {
       CollectionRecordModel {
           id: record.id,
           customer_id: record.customer_id,
           collection_agent_id: record.collection_agent_id,
           collection_program_id: record.collection_program_id,
           account_id: record.account_id,
           collection_date: record.collection_date,
           collection_time: record.collection_time,
           amount: record.amount,
           currency: record.currency.to_string(),
           collection_method: Self::collection_method_to_model(record.collection_method.clone()),
           location_address_id: record.location_address_id,
           receipt_number: record.receipt_number.to_string(),
           status: Self::collection_record_status_to_model(record.status.clone()),
           notes: record.notes.as_ref().map(|s| s.to_string()),
           verification_customer_signature: verification.and_then(|v| v.customer_signature.as_ref().map(|s| s.to_string())),
           verification_agent_verification_code: verification.and_then(|v| v.agent_verification_code.as_ref().map(|s| s.to_string())),
           verification_fingerprint_hash: biometric.and_then(|b| b.fingerprint_hash.as_ref().map(|s| s.to_string())),
           verification_face_recognition_score: biometric.and_then(|b| b.face_recognition_score),
           verification_biometric_method: biometric.map(|b| Self::biometric_method_to_model(b.verification_method.clone())),
           verification_confidence_level: biometric.map(|b| b.confidence_level),
           verification_customer_photo_hash: photo.and_then(|p| p.customer_photo_hash.as_ref().map(|s| s.to_string())),
           verification_receipt_photo_hash: photo.and_then(|p| p.receipt_photo_hash.as_ref().map(|s| s.to_string())),
           verification_location_photo_hash: photo.and_then(|p| p.location_photo_hash.as_ref().map(|s| s.to_string())),
           verification_photo_timestamp: photo.map(|p| p.photo_timestamp),
           verification_witness_name: witness.map(|w| w.witness_name.to_string()),
           verification_witness_contact: witness.map(|w| w.witness_contact.to_string()),
           verification_witness_relationship: witness.map(|w| w.witness_relationship.to_string()),
           verification_witness_signature: witness.and_then(|w| w.witness_signature.as_ref().map(|s| s.to_string())),
           verification_timestamp: verification.map(|v| v.verification_timestamp),
           created_at: record.created_at,
           processed_at: record.processed_at,
           reason_id: record.reason_id,
       }
   }

   /// Convert database CollectionRecordModel to domain CollectionRecord
   pub fn collection_record_from_model(model: CollectionRecordModel) -> CollectionRecord {
       CollectionRecord {
           id: model.id,
           customer_id: model.customer_id,
           collection_agent_id: model.collection_agent_id,
           collection_program_id: model.collection_program_id,
           account_id: model.account_id,
           collection_date: model.collection_date,
           collection_time: model.collection_time,
           amount: model.amount,
           currency: HeaplessString::try_from(model.currency.as_str()).unwrap_or_default(),
           collection_method: Self::collection_method_from_model(model.collection_method),
           location_address_id: model.location_address_id,
           receipt_number: HeaplessString::try_from(model.receipt_number.as_str()).unwrap_or_default(),
           status: Self::collection_record_status_from_model(model.status),
           notes: model.notes.map(|s| HeaplessString::try_from(s.as_str()).unwrap_or_default()),
           collection_verification_id: None, // Needs to be handled separately
           created_at: model.created_at,
           processed_at: model.processed_at,
           reason_id: model.reason_id,
       }
   }

   /// Convert domain CollectionBatch to database CollectionBatchModel
   pub fn collection_batch_to_model(batch: &CollectionBatch) -> CollectionBatchModel {
       CollectionBatchModel {
           id: batch.id,
           collection_agent_id: batch.collection_agent_id,
           collection_date: batch.collection_date,
           total_collections: batch.total_collections,
           total_amount: batch.total_amount,
           currency: batch.currency.to_string(),
           status: Self::batch_status_to_model(batch.status.clone()),
           collection_records: batch.collection_records.clone(),
           reconciliation_expected_amount: None,
           reconciliation_actual_amount: None,
           reconciliation_variance: None,
           reconciliation_variance_reason: None,
           reconciled_by_person_id: None,
           reconciliation_timestamp: None,
           reconciliation_adjustment_required: None,
           created_at: batch.created_at,
           processed_at: batch.processed_at,
       }
   }

   /// Convert database CollectionBatchModel to domain CollectionBatch
   pub fn collection_batch_from_model(model: CollectionBatchModel) -> CollectionBatch {
       CollectionBatch {
           id: model.id,
           collection_agent_id: model.collection_agent_id,
           collection_date: model.collection_date,
           total_collections: model.total_collections,
           total_amount: model.total_amount,
           currency: HeaplessString::try_from(model.currency.as_str()).unwrap_or_default(),
           status: Self::batch_status_from_model(model.status),
           collection_records: model.collection_records,
           reconciliation_data_id: if model.reconciliation_expected_amount.is_some() {
               Some(model.id)
           } else {
               None
           },
           created_at: model.created_at,
           processed_at: model.processed_at,
       }
   }

   /// Convert domain CoverageArea to database CoverageAreaModel
   pub fn coverage_area_to_model(area: &CoverageArea) -> CoverageAreaModel {
       CoverageAreaModel {
           id: area.id,
           area_name: area.area_name.to_string(),
           area_type: Self::area_type_to_model(area.area_type.clone()),
           boundary_coordinates_long_1: area.boundary_coordinates_long_1,
           boundary_coordinates_lat_1: area.boundary_coordinates_lat_1,
           boundary_coordinates_long_2: area.boundary_coordinates_long_2,
           boundary_coordinates_lat_2: area.boundary_coordinates_lat_2,
           boundary_coordinates_long_3: area.boundary_coordinates_long_3,
           boundary_coordinates_lat_3: area.boundary_coordinates_lat_3,
           boundary_coordinates_long_4: area.boundary_coordinates_long_4,
           boundary_coordinates_lat_4: area.boundary_coordinates_lat_4,
           boundary_coordinates_long_5: area.boundary_coordinates_long_5,
           boundary_coordinates_lat_5: area.boundary_coordinates_lat_5,
           customer_density: Self::customer_density_to_model(area.customer_density.clone()),
           transport_mode: Self::transport_mode_to_model(area.transport_mode.clone()),
       }
   }

   /// Convert database CoverageAreaModel to domain CoverageArea
   pub fn coverage_area_from_model(model: CoverageAreaModel) -> CoverageArea {
       CoverageArea {
           id: model.id,
           area_name: HeaplessString::try_from(model.area_name.as_str()).unwrap_or_default(),
           area_type: Self::area_type_from_model(model.area_type),
           boundary_coordinates_long_1: model.boundary_coordinates_long_1,
           boundary_coordinates_lat_1: model.boundary_coordinates_lat_1,
           boundary_coordinates_long_2: model.boundary_coordinates_long_2,
           boundary_coordinates_lat_2: model.boundary_coordinates_lat_2,
           boundary_coordinates_long_3: model.boundary_coordinates_long_3,
           boundary_coordinates_lat_3: model.boundary_coordinates_lat_3,
           boundary_coordinates_long_4: model.boundary_coordinates_long_4,
           boundary_coordinates_lat_4: model.boundary_coordinates_lat_4,
           boundary_coordinates_long_5: model.boundary_coordinates_long_5,
           boundary_coordinates_lat_5: model.boundary_coordinates_lat_5,
           customer_density: Self::customer_density_from_model(model.customer_density),
           transport_mode: Self::transport_mode_from_model(model.transport_mode),
       }
   }

   /// Convert domain PerformanceAlert to database PerformanceAlertModel
   pub fn performance_alert_to_model(alert: PerformanceAlert) -> PerformanceAlertModel {
       PerformanceAlertModel {
           id: alert.id,
           agent_performance_metrics_id: alert.agent_performance_metrics_id,
           alert_type: Self::alert_type_to_model(alert.alert_type),
           severity: Self::alert_severity_to_model(alert.severity),
           message: alert.message,
           acknowledged: alert.acknowledged,
           resolution_required: alert.resolution_required,
           created_at: alert.created_at,
           acknowledged_at: None,
           resolved_at: None,
       }
   }

   /// Convert database PerformanceAlertModel to domain PerformanceAlert
   pub fn performance_alert_from_model(model: PerformanceAlertModel) -> PerformanceAlert {
       PerformanceAlert {
           id: model.id,
           agent_performance_metrics_id: model.agent_performance_metrics_id,
           alert_type: Self::alert_type_from_model(model.alert_type),
           severity: Self::alert_severity_from_model(model.severity),
           message: model.message,
           created_at: model.created_at,
           acknowledged: model.acknowledged,
           resolution_required: model.resolution_required,
       }
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
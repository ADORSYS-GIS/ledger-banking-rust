use banking_api::domain::daily_collection as domain;
use banking_db::models::daily_collection as db_models;

pub struct DailyCollectionMapper;

impl DailyCollectionMapper {
    // AgentStatus
    pub fn agent_status_to_db(status: domain::AgentStatus) -> db_models::AgentStatus {
        match status {
            domain::AgentStatus::Active => db_models::AgentStatus::Active,
            domain::AgentStatus::Suspended => db_models::AgentStatus::Suspended,
            domain::AgentStatus::Training => db_models::AgentStatus::Training,
            domain::AgentStatus::OnLeave => db_models::AgentStatus::OnLeave,
            domain::AgentStatus::Terminated => db_models::AgentStatus::Terminated,
        }
    }

    pub fn agent_status_from_db(status: db_models::AgentStatus) -> domain::AgentStatus {
        match status {
            db_models::AgentStatus::Active => domain::AgentStatus::Active,
            db_models::AgentStatus::Suspended => domain::AgentStatus::Suspended,
            db_models::AgentStatus::Training => domain::AgentStatus::Training,
            db_models::AgentStatus::OnLeave => domain::AgentStatus::OnLeave,
            db_models::AgentStatus::Terminated => domain::AgentStatus::Terminated,
        }
    }

    // AreaType
    pub fn area_type_to_db(area_type: domain::AreaType) -> db_models::AreaType {
        match area_type {
            domain::AreaType::Urban => db_models::AreaType::Urban,
            domain::AreaType::Suburban => db_models::AreaType::Suburban,
            domain::AreaType::Rural => db_models::AreaType::Rural,
            domain::AreaType::Commercial => db_models::AreaType::Commercial,
            domain::AreaType::Industrial => db_models::AreaType::Industrial,
            domain::AreaType::Mixed => db_models::AreaType::Mixed,
        }
    }

    pub fn area_type_from_db(area_type: db_models::AreaType) -> domain::AreaType {
        match area_type {
            db_models::AreaType::Urban => domain::AreaType::Urban,
            db_models::AreaType::Suburban => domain::AreaType::Suburban,
            db_models::AreaType::Rural => domain::AreaType::Rural,
            db_models::AreaType::Commercial => domain::AreaType::Commercial,
            db_models::AreaType::Industrial => domain::AreaType::Industrial,
            db_models::AreaType::Mixed => domain::AreaType::Mixed,
        }
    }

    // CustomerDensity
    pub fn customer_density_to_db(density: domain::CustomerDensity) -> db_models::CustomerDensity {
        match density {
            domain::CustomerDensity::High => db_models::CustomerDensity::High,
            domain::CustomerDensity::Medium => db_models::CustomerDensity::Medium,
            domain::CustomerDensity::Low => db_models::CustomerDensity::Low,
        }
    }

    pub fn customer_density_from_db(density: db_models::CustomerDensity) -> domain::CustomerDensity {
        match density {
            db_models::CustomerDensity::High => domain::CustomerDensity::High,
            db_models::CustomerDensity::Medium => domain::CustomerDensity::Medium,
            db_models::CustomerDensity::Low => domain::CustomerDensity::Low,
        }
    }

    // TransportMode
    pub fn transport_mode_to_db(mode: domain::TransportMode) -> db_models::TransportMode {
        match mode {
            domain::TransportMode::Walking => db_models::TransportMode::Walking,
            domain::TransportMode::Bicycle => db_models::TransportMode::Bicycle,
            domain::TransportMode::Motorcycle => db_models::TransportMode::Motorcycle,
            domain::TransportMode::Car => db_models::TransportMode::Car,
            domain::TransportMode::PublicTransport => db_models::TransportMode::PublicTransport,
            domain::TransportMode::Mixed => db_models::TransportMode::Mixed,
        }
    }

    pub fn transport_mode_from_db(mode: db_models::TransportMode) -> domain::TransportMode {
        match mode {
            db_models::TransportMode::Walking => domain::TransportMode::Walking,
            db_models::TransportMode::Bicycle => domain::TransportMode::Bicycle,
            db_models::TransportMode::Motorcycle => domain::TransportMode::Motorcycle,
            db_models::TransportMode::Car => domain::TransportMode::Car,
            db_models::TransportMode::PublicTransport => domain::TransportMode::PublicTransport,
            db_models::TransportMode::Mixed => domain::TransportMode::Mixed,
        }
    }

    // CollectionAlertType
    pub fn collection_alert_type_to_db(alert_type: domain::CollectionAlertType) -> db_models::CollectionAlertType {
        match alert_type {
            domain::CollectionAlertType::LowCollectionRate => db_models::CollectionAlertType::LowCollectionRate,
            domain::CollectionAlertType::CustomerComplaint => db_models::CollectionAlertType::CustomerComplaint,
            domain::CollectionAlertType::CashDiscrepancy => db_models::CollectionAlertType::CashDiscrepancy,
            domain::CollectionAlertType::MissedSchedule => db_models::CollectionAlertType::MissedSchedule,
            domain::CollectionAlertType::ComplianceViolation => db_models::CollectionAlertType::ComplianceViolation,
            domain::CollectionAlertType::SafetyConcern => db_models::CollectionAlertType::SafetyConcern,
            domain::CollectionAlertType::DeviceIssue => db_models::CollectionAlertType::DeviceIssue,
        }
    }

    pub fn collection_alert_type_from_db(alert_type: db_models::CollectionAlertType) -> domain::CollectionAlertType {
        match alert_type {
            db_models::CollectionAlertType::LowCollectionRate => domain::CollectionAlertType::LowCollectionRate,
            db_models::CollectionAlertType::CustomerComplaint => domain::CollectionAlertType::CustomerComplaint,
            db_models::CollectionAlertType::CashDiscrepancy => domain::CollectionAlertType::CashDiscrepancy,
            db_models::CollectionAlertType::MissedSchedule => domain::CollectionAlertType::MissedSchedule,
            db_models::CollectionAlertType::ComplianceViolation => domain::CollectionAlertType::ComplianceViolation,
            db_models::CollectionAlertType::SafetyConcern => domain::CollectionAlertType::SafetyConcern,
            db_models::CollectionAlertType::DeviceIssue => domain::CollectionAlertType::DeviceIssue,
        }
    }

    // DeviceType
    pub fn device_type_to_db(device_type: domain::DeviceType) -> db_models::DeviceType {
        match device_type {
            domain::DeviceType::Smartphone => db_models::DeviceType::Smartphone,
            domain::DeviceType::Tablet => db_models::DeviceType::Tablet,
            domain::DeviceType::PortableTerminal => db_models::DeviceType::PortableTerminal,
            domain::DeviceType::SmartWatch => db_models::DeviceType::SmartWatch,
        }
    }

    pub fn device_type_from_db(device_type: db_models::DeviceType) -> domain::DeviceType {
        match device_type {
            db_models::DeviceType::Smartphone => domain::DeviceType::Smartphone,
            db_models::DeviceType::Tablet => domain::DeviceType::Tablet,
            db_models::DeviceType::PortableTerminal => domain::DeviceType::PortableTerminal,
            db_models::DeviceType::SmartWatch => domain::DeviceType::SmartWatch,
        }
    }

    // ConnectivityStatus
    pub fn connectivity_status_to_db(status: domain::ConnectivityStatus) -> db_models::ConnectivityStatus {
        match status {
            domain::ConnectivityStatus::Online => db_models::ConnectivityStatus::Online,
            domain::ConnectivityStatus::Offline => db_models::ConnectivityStatus::Offline,
            domain::ConnectivityStatus::LimitedConnectivity => db_models::ConnectivityStatus::LimitedConnectivity,
            domain::ConnectivityStatus::SyncPending => db_models::ConnectivityStatus::SyncPending,
        }
    }

    pub fn connectivity_status_from_db(status: db_models::ConnectivityStatus) -> domain::ConnectivityStatus {
        match status {
            db_models::ConnectivityStatus::Online => domain::ConnectivityStatus::Online,
            db_models::ConnectivityStatus::Offline => domain::ConnectivityStatus::Offline,
            db_models::ConnectivityStatus::LimitedConnectivity => domain::ConnectivityStatus::LimitedConnectivity,
            db_models::ConnectivityStatus::SyncPending => domain::ConnectivityStatus::SyncPending,
        }
    }

    // CollectionProgramType
    pub fn collection_program_type_to_db(program_type: domain::CollectionProgramType) -> db_models::CollectionProgramType {
        match program_type {
            domain::CollectionProgramType::FixedAmount => db_models::CollectionProgramType::FixedAmount,
            domain::CollectionProgramType::VariableAmount => db_models::CollectionProgramType::VariableAmount,
            domain::CollectionProgramType::TargetBased => db_models::CollectionProgramType::TargetBased,
            domain::CollectionProgramType::DurationBased => db_models::CollectionProgramType::DurationBased,
        }
    }

    pub fn collection_program_type_from_db(program_type: db_models::CollectionProgramType) -> domain::CollectionProgramType {
        match program_type {
            db_models::CollectionProgramType::FixedAmount => domain::CollectionProgramType::FixedAmount,
            db_models::CollectionProgramType::VariableAmount => domain::CollectionProgramType::VariableAmount,
            db_models::CollectionProgramType::TargetBased => domain::CollectionProgramType::TargetBased,
            db_models::CollectionProgramType::DurationBased => domain::CollectionProgramType::DurationBased,
        }
    }

    // ProgramStatus
    pub fn program_status_to_db(status: domain::ProgramStatus) -> db_models::ProgramStatus {
        match status {
            domain::ProgramStatus::Active => db_models::ProgramStatus::Active,
            domain::ProgramStatus::Suspended => db_models::ProgramStatus::Suspended,
            domain::ProgramStatus::Closed => db_models::ProgramStatus::Closed,
            domain::ProgramStatus::UnderReview => db_models::ProgramStatus::UnderReview,
        }
    }

    pub fn program_status_from_db(status: db_models::ProgramStatus) -> domain::ProgramStatus {
        match status {
            db_models::ProgramStatus::Active => domain::ProgramStatus::Active,
            db_models::ProgramStatus::Suspended => domain::ProgramStatus::Suspended,
            db_models::ProgramStatus::Closed => domain::ProgramStatus::Closed,
            db_models::ProgramStatus::UnderReview => domain::ProgramStatus::UnderReview,
        }
    }

    // CollectionFrequency
    pub fn collection_frequency_to_db(frequency: domain::CollectionFrequency) -> db_models::CollectionFrequency {
        match frequency {
            domain::CollectionFrequency::Daily => db_models::CollectionFrequency::Daily,
            domain::CollectionFrequency::Weekly => db_models::CollectionFrequency::Weekly,
            domain::CollectionFrequency::Monthly => db_models::CollectionFrequency::Monthly,
            domain::CollectionFrequency::Quarterly => db_models::CollectionFrequency::Quarterly,
            domain::CollectionFrequency::Yearly => db_models::CollectionFrequency::Yearly,
        }
    }

    pub fn collection_frequency_from_db(frequency: db_models::CollectionFrequency) -> domain::CollectionFrequency {
        match frequency {
            db_models::CollectionFrequency::Daily => domain::CollectionFrequency::Daily,
            db_models::CollectionFrequency::Weekly => domain::CollectionFrequency::Weekly,
            db_models::CollectionFrequency::Monthly => domain::CollectionFrequency::Monthly,
            db_models::CollectionFrequency::Quarterly => domain::CollectionFrequency::Quarterly,
            db_models::CollectionFrequency::Yearly => domain::CollectionFrequency::Yearly,
        }
    }

    // CollectionFeeFrequency
    pub fn collection_fee_frequency_to_db(frequency: domain::CollectionFeeFrequency) -> db_models::CollectionFeeFrequency {
        match frequency {
            domain::CollectionFeeFrequency::PerCollection => db_models::CollectionFeeFrequency::PerCollection,
            domain::CollectionFeeFrequency::Daily => db_models::CollectionFeeFrequency::Daily,
            domain::CollectionFeeFrequency::Weekly => db_models::CollectionFeeFrequency::Weekly,
            domain::CollectionFeeFrequency::Monthly => db_models::CollectionFeeFrequency::Monthly,
            domain::CollectionFeeFrequency::OneTime => db_models::CollectionFeeFrequency::OneTime,
        }
    }

    pub fn collection_fee_frequency_from_db(frequency: db_models::CollectionFeeFrequency) -> domain::CollectionFeeFrequency {
        match frequency {
            db_models::CollectionFeeFrequency::PerCollection => domain::CollectionFeeFrequency::PerCollection,
            db_models::CollectionFeeFrequency::Daily => domain::CollectionFeeFrequency::Daily,
            db_models::CollectionFeeFrequency::Weekly => domain::CollectionFeeFrequency::Weekly,
            db_models::CollectionFeeFrequency::Monthly => domain::CollectionFeeFrequency::Monthly,
            db_models::CollectionFeeFrequency::OneTime => domain::CollectionFeeFrequency::OneTime,
        }
    }

    // CollectionStatus
    pub fn collection_status_to_db(status: domain::CollectionStatus) -> db_models::CollectionStatus {
        match status {
            domain::CollectionStatus::Active => db_models::CollectionStatus::Active,
            domain::CollectionStatus::Suspended => db_models::CollectionStatus::Suspended,
            domain::CollectionStatus::Defaulted => db_models::CollectionStatus::Defaulted,
            domain::CollectionStatus::Graduated => db_models::CollectionStatus::Graduated,
            domain::CollectionStatus::Terminated => db_models::CollectionStatus::Terminated,
        }
    }

    pub fn collection_status_from_db(status: db_models::CollectionStatus) -> domain::CollectionStatus {
        match status {
            db_models::CollectionStatus::Active => domain::CollectionStatus::Active,
            db_models::CollectionStatus::Suspended => domain::CollectionStatus::Suspended,
            db_models::CollectionStatus::Defaulted => domain::CollectionStatus::Defaulted,
            db_models::CollectionStatus::Graduated => domain::CollectionStatus::Graduated,
            db_models::CollectionStatus::Terminated => domain::CollectionStatus::Terminated,
        }
    }

    // HolidayHandling
    pub fn holiday_handling_to_db(handling: domain::HolidayHandling) -> db_models::HolidayHandling {
        match handling {
            domain::HolidayHandling::Skip => db_models::HolidayHandling::Skip,
            domain::HolidayHandling::NextBusinessDay => db_models::HolidayHandling::NextBusinessDay,
            domain::HolidayHandling::PreviousBusinessDay => db_models::HolidayHandling::PreviousBusinessDay,
            domain::HolidayHandling::CollectDouble => db_models::HolidayHandling::CollectDouble,
        }
    }

    pub fn holiday_handling_from_db(handling: db_models::HolidayHandling) -> domain::HolidayHandling {
        match handling {
            db_models::HolidayHandling::Skip => domain::HolidayHandling::Skip,
            db_models::HolidayHandling::NextBusinessDay => domain::HolidayHandling::NextBusinessDay,
            db_models::HolidayHandling::PreviousBusinessDay => domain::HolidayHandling::PreviousBusinessDay,
            db_models::HolidayHandling::CollectDouble => domain::HolidayHandling::CollectDouble,
        }
    }

    // ReliabilityRating
    pub fn reliability_rating_to_db(rating: domain::ReliabilityRating) -> db_models::ReliabilityRating {
        match rating {
            domain::ReliabilityRating::Excellent => db_models::ReliabilityRating::Excellent,
            domain::ReliabilityRating::Good => db_models::ReliabilityRating::Good,
            domain::ReliabilityRating::Fair => db_models::ReliabilityRating::Fair,
            domain::ReliabilityRating::Poor => db_models::ReliabilityRating::Poor,
            domain::ReliabilityRating::Critical => db_models::ReliabilityRating::Critical,
        }
    }

    pub fn reliability_rating_from_db(rating: db_models::ReliabilityRating) -> domain::ReliabilityRating {
        match rating {
            db_models::ReliabilityRating::Excellent => domain::ReliabilityRating::Excellent,
            db_models::ReliabilityRating::Good => domain::ReliabilityRating::Good,
            db_models::ReliabilityRating::Fair => domain::ReliabilityRating::Fair,
            db_models::ReliabilityRating::Poor => domain::ReliabilityRating::Poor,
            db_models::ReliabilityRating::Critical => domain::ReliabilityRating::Critical,
        }
    }

    // CollectionMethod
    pub fn collection_method_to_db(method: domain::CollectionMethod) -> db_models::CollectionMethod {
        match method {
            domain::CollectionMethod::Cash => db_models::CollectionMethod::Cash,
            domain::CollectionMethod::MobilePayment => db_models::CollectionMethod::MobilePayment,
            domain::CollectionMethod::BankTransfer => db_models::CollectionMethod::BankTransfer,
            domain::CollectionMethod::DigitalWallet => db_models::CollectionMethod::DigitalWallet,
        }
    }

    pub fn collection_method_from_db(method: db_models::CollectionMethod) -> domain::CollectionMethod {
        match method {
            db_models::CollectionMethod::Cash => domain::CollectionMethod::Cash,
            db_models::CollectionMethod::MobilePayment => domain::CollectionMethod::MobilePayment,
            db_models::CollectionMethod::BankTransfer => domain::CollectionMethod::BankTransfer,
            db_models::CollectionMethod::DigitalWallet => domain::CollectionMethod::DigitalWallet,
        }
    }

    // CollectionRecordStatus
    pub fn collection_record_status_to_db(status: domain::CollectionRecordStatus) -> db_models::CollectionRecordStatus {
        match status {
            domain::CollectionRecordStatus::Pending => db_models::CollectionRecordStatus::Pending,
            domain::CollectionRecordStatus::Processed => db_models::CollectionRecordStatus::Processed,
            domain::CollectionRecordStatus::Failed => db_models::CollectionRecordStatus::Failed,
            domain::CollectionRecordStatus::Reversed => db_models::CollectionRecordStatus::Reversed,
            domain::CollectionRecordStatus::UnderReview => db_models::CollectionRecordStatus::UnderReview,
        }
    }

    pub fn collection_record_status_from_db(status: db_models::CollectionRecordStatus) -> domain::CollectionRecordStatus {
        match status {
            db_models::CollectionRecordStatus::Pending => domain::CollectionRecordStatus::Pending,
            db_models::CollectionRecordStatus::Processed => domain::CollectionRecordStatus::Processed,
            db_models::CollectionRecordStatus::Failed => domain::CollectionRecordStatus::Failed,
            db_models::CollectionRecordStatus::Reversed => domain::CollectionRecordStatus::Reversed,
            db_models::CollectionRecordStatus::UnderReview => domain::CollectionRecordStatus::UnderReview,
        }
    }

    // BiometricMethod
    pub fn biometric_method_to_db(method: domain::BiometricMethod) -> db_models::BiometricMethod {
        match method {
            domain::BiometricMethod::Fingerprint => db_models::BiometricMethod::Fingerprint,
            domain::BiometricMethod::FaceRecognition => db_models::BiometricMethod::FaceRecognition,
            domain::BiometricMethod::VoicePrint => db_models::BiometricMethod::VoicePrint,
            domain::BiometricMethod::Combined => db_models::BiometricMethod::Combined,
        }
    }

    pub fn biometric_method_from_db(method: db_models::BiometricMethod) -> domain::BiometricMethod {
        match method {
            db_models::BiometricMethod::Fingerprint => domain::BiometricMethod::Fingerprint,
            db_models::BiometricMethod::FaceRecognition => domain::BiometricMethod::FaceRecognition,
            db_models::BiometricMethod::VoicePrint => domain::BiometricMethod::VoicePrint,
            db_models::BiometricMethod::Combined => domain::BiometricMethod::Combined,
        }
    }

    // BatchStatus
    pub fn batch_status_to_db(status: domain::BatchStatus) -> db_models::BatchStatus {
        match status {
            domain::BatchStatus::Pending => db_models::BatchStatus::Pending,
            domain::BatchStatus::Processing => db_models::BatchStatus::Processing,
            domain::BatchStatus::Completed => db_models::BatchStatus::Completed,
            domain::BatchStatus::Failed => db_models::BatchStatus::Failed,
            domain::BatchStatus::PartiallyProcessed => db_models::BatchStatus::PartiallyProcessed,
            domain::BatchStatus::RequiresReconciliation => db_models::BatchStatus::RequiresReconciliation,
        }
    }

    pub fn batch_status_from_db(status: db_models::BatchStatus) -> domain::BatchStatus {
        match status {
            db_models::BatchStatus::Pending => domain::BatchStatus::Pending,
            db_models::BatchStatus::Processing => domain::BatchStatus::Processing,
            db_models::BatchStatus::Completed => domain::BatchStatus::Completed,
            db_models::BatchStatus::Failed => domain::BatchStatus::Failed,
            db_models::BatchStatus::PartiallyProcessed => domain::BatchStatus::PartiallyProcessed,
            db_models::BatchStatus::RequiresReconciliation => domain::BatchStatus::RequiresReconciliation,
        }
    }
}
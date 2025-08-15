use banking_api::domain::daily_collection as domain;
use banking_db::models::daily_collection as db_models;
use uuid::Uuid;

use crate::mappers::collateral_mapper::CollateralMapper;

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
    pub fn collection_alert_type_to_db(
        alert_type: domain::CollectionAlertType,
    ) -> db_models::CollectionAlertType {
        match alert_type {
            domain::CollectionAlertType::LowCollectionRate => {
                db_models::CollectionAlertType::LowCollectionRate
            }
            domain::CollectionAlertType::CustomerComplaint => {
                db_models::CollectionAlertType::CustomerComplaint
            }
            domain::CollectionAlertType::CashDiscrepancy => {
                db_models::CollectionAlertType::CashDiscrepancy
            }
            domain::CollectionAlertType::MissedSchedule => {
                db_models::CollectionAlertType::MissedSchedule
            }
            domain::CollectionAlertType::ComplianceViolation => {
                db_models::CollectionAlertType::ComplianceViolation
            }
            domain::CollectionAlertType::SafetyConcern => {
                db_models::CollectionAlertType::SafetyConcern
            }
            domain::CollectionAlertType::DeviceIssue => db_models::CollectionAlertType::DeviceIssue,
        }
    }

    pub fn collection_alert_type_from_db(
        alert_type: db_models::CollectionAlertType,
    ) -> domain::CollectionAlertType {
        match alert_type {
            db_models::CollectionAlertType::LowCollectionRate => {
                domain::CollectionAlertType::LowCollectionRate
            }
            db_models::CollectionAlertType::CustomerComplaint => {
                domain::CollectionAlertType::CustomerComplaint
            }
            db_models::CollectionAlertType::CashDiscrepancy => {
                domain::CollectionAlertType::CashDiscrepancy
            }
            db_models::CollectionAlertType::MissedSchedule => {
                domain::CollectionAlertType::MissedSchedule
            }
            db_models::CollectionAlertType::ComplianceViolation => {
                domain::CollectionAlertType::ComplianceViolation
            }
            db_models::CollectionAlertType::SafetyConcern => {
                domain::CollectionAlertType::SafetyConcern
            }
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
    pub fn connectivity_status_to_db(
        status: domain::ConnectivityStatus,
    ) -> db_models::ConnectivityStatus {
        match status {
            domain::ConnectivityStatus::Online => db_models::ConnectivityStatus::Online,
            domain::ConnectivityStatus::Offline => db_models::ConnectivityStatus::Offline,
            domain::ConnectivityStatus::LimitedConnectivity => {
                db_models::ConnectivityStatus::LimitedConnectivity
            }
            domain::ConnectivityStatus::SyncPending => db_models::ConnectivityStatus::SyncPending,
        }
    }

    pub fn connectivity_status_from_db(
        status: db_models::ConnectivityStatus,
    ) -> domain::ConnectivityStatus {
        match status {
            db_models::ConnectivityStatus::Online => domain::ConnectivityStatus::Online,
            db_models::ConnectivityStatus::Offline => domain::ConnectivityStatus::Offline,
            db_models::ConnectivityStatus::LimitedConnectivity => {
                domain::ConnectivityStatus::LimitedConnectivity
            }
            db_models::ConnectivityStatus::SyncPending => domain::ConnectivityStatus::SyncPending,
        }
    }

    // CollectionProgramType
    pub fn collection_program_type_to_db(
        program_type: domain::CollectionProgramType,
    ) -> db_models::CollectionProgramType {
        match program_type {
            domain::CollectionProgramType::FixedAmount => {
                db_models::CollectionProgramType::FixedAmount
            }
            domain::CollectionProgramType::VariableAmount => {
                db_models::CollectionProgramType::VariableAmount
            }
            domain::CollectionProgramType::TargetBased => {
                db_models::CollectionProgramType::TargetBased
            }
            domain::CollectionProgramType::DurationBased => {
                db_models::CollectionProgramType::DurationBased
            }
        }
    }

    pub fn collection_program_type_from_db(
        program_type: db_models::CollectionProgramType,
    ) -> domain::CollectionProgramType {
        match program_type {
            db_models::CollectionProgramType::FixedAmount => {
                domain::CollectionProgramType::FixedAmount
            }
            db_models::CollectionProgramType::VariableAmount => {
                domain::CollectionProgramType::VariableAmount
            }
            db_models::CollectionProgramType::TargetBased => {
                domain::CollectionProgramType::TargetBased
            }
            db_models::CollectionProgramType::DurationBased => {
                domain::CollectionProgramType::DurationBased
            }
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
    pub fn collection_frequency_to_db(
        frequency: domain::CollectionFrequency,
    ) -> db_models::CollectionFrequency {
        match frequency {
            domain::CollectionFrequency::Daily => db_models::CollectionFrequency::Daily,
            domain::CollectionFrequency::Weekly => db_models::CollectionFrequency::Weekly,
            domain::CollectionFrequency::Monthly => db_models::CollectionFrequency::Monthly,
            domain::CollectionFrequency::Quarterly => db_models::CollectionFrequency::Quarterly,
            domain::CollectionFrequency::Yearly => db_models::CollectionFrequency::Yearly,
        }
    }

    pub fn collection_frequency_from_db(
        frequency: db_models::CollectionFrequency,
    ) -> domain::CollectionFrequency {
        match frequency {
            db_models::CollectionFrequency::Daily => domain::CollectionFrequency::Daily,
            db_models::CollectionFrequency::Weekly => domain::CollectionFrequency::Weekly,
            db_models::CollectionFrequency::Monthly => domain::CollectionFrequency::Monthly,
            db_models::CollectionFrequency::Quarterly => domain::CollectionFrequency::Quarterly,
            db_models::CollectionFrequency::Yearly => domain::CollectionFrequency::Yearly,
        }
    }

    // CollectionFeeFrequency
    pub fn collection_fee_frequency_to_db(
        frequency: domain::CollectionFeeFrequency,
    ) -> db_models::CollectionFeeFrequency {
        match frequency {
            domain::CollectionFeeFrequency::PerCollection => {
                db_models::CollectionFeeFrequency::PerCollection
            }
            domain::CollectionFeeFrequency::Daily => db_models::CollectionFeeFrequency::Daily,
            domain::CollectionFeeFrequency::Weekly => db_models::CollectionFeeFrequency::Weekly,
            domain::CollectionFeeFrequency::Monthly => db_models::CollectionFeeFrequency::Monthly,
            domain::CollectionFeeFrequency::OneTime => db_models::CollectionFeeFrequency::OneTime,
        }
    }

    pub fn collection_fee_frequency_from_db(
        frequency: db_models::CollectionFeeFrequency,
    ) -> domain::CollectionFeeFrequency {
        match frequency {
            db_models::CollectionFeeFrequency::PerCollection => {
                domain::CollectionFeeFrequency::PerCollection
            }
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
            domain::HolidayHandling::PreviousBusinessDay => {
                db_models::HolidayHandling::PreviousBusinessDay
            }
            domain::HolidayHandling::CollectDouble => db_models::HolidayHandling::CollectDouble,
        }
    }

    pub fn holiday_handling_from_db(handling: db_models::HolidayHandling) -> domain::HolidayHandling {
        match handling {
            db_models::HolidayHandling::Skip => domain::HolidayHandling::Skip,
            db_models::HolidayHandling::NextBusinessDay => domain::HolidayHandling::NextBusinessDay,
            db_models::HolidayHandling::PreviousBusinessDay => {
                domain::HolidayHandling::PreviousBusinessDay
            }
            db_models::HolidayHandling::CollectDouble => domain::HolidayHandling::CollectDouble,
        }
    }

    // ReliabilityRating
    pub fn reliability_rating_to_db(
        rating: domain::ReliabilityRating,
    ) -> db_models::ReliabilityRating {
        match rating {
            domain::ReliabilityRating::Excellent => db_models::ReliabilityRating::Excellent,
            domain::ReliabilityRating::Good => db_models::ReliabilityRating::Good,
            domain::ReliabilityRating::Fair => db_models::ReliabilityRating::Fair,
            domain::ReliabilityRating::Poor => db_models::ReliabilityRating::Poor,
            domain::ReliabilityRating::Critical => db_models::ReliabilityRating::Critical,
        }
    }

    pub fn reliability_rating_from_db(
        rating: db_models::ReliabilityRating,
    ) -> domain::ReliabilityRating {
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
    pub fn collection_record_status_to_db(
        status: domain::CollectionRecordStatus,
    ) -> db_models::CollectionRecordStatus {
        match status {
            domain::CollectionRecordStatus::Pending => db_models::CollectionRecordStatus::Pending,
            domain::CollectionRecordStatus::Processed => db_models::CollectionRecordStatus::Processed,
            domain::CollectionRecordStatus::Failed => db_models::CollectionRecordStatus::Failed,
            domain::CollectionRecordStatus::Reversed => db_models::CollectionRecordStatus::Reversed,
            domain::CollectionRecordStatus::UnderReview => {
                db_models::CollectionRecordStatus::UnderReview
            }
        }
    }

    pub fn collection_record_status_from_db(
        status: db_models::CollectionRecordStatus,
    ) -> domain::CollectionRecordStatus {
        match status {
            db_models::CollectionRecordStatus::Pending => domain::CollectionRecordStatus::Pending,
            db_models::CollectionRecordStatus::Processed => domain::CollectionRecordStatus::Processed,
            db_models::CollectionRecordStatus::Failed => domain::CollectionRecordStatus::Failed,
            db_models::CollectionRecordStatus::Reversed => domain::CollectionRecordStatus::Reversed,
            db_models::CollectionRecordStatus::UnderReview => {
                domain::CollectionRecordStatus::UnderReview
            }
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
            domain::BatchStatus::RequiresReconciliation => {
                db_models::BatchStatus::RequiresReconciliation
            }
        }
    }

    pub fn batch_status_from_db(status: db_models::BatchStatus) -> domain::BatchStatus {
        match status {
            db_models::BatchStatus::Pending => domain::BatchStatus::Pending,
            db_models::BatchStatus::Processing => domain::BatchStatus::Processing,
            db_models::BatchStatus::Completed => domain::BatchStatus::Completed,
            db_models::BatchStatus::Failed => domain::BatchStatus::Failed,
            db_models::BatchStatus::PartiallyProcessed => domain::BatchStatus::PartiallyProcessed,
            db_models::BatchStatus::RequiresReconciliation => {
                domain::BatchStatus::RequiresReconciliation
            }
        }
    }

    // CollectionAgent
    pub fn collection_agent_to_db(
        agent: domain::CollectionAgent,
    ) -> db_models::CollectionAgentModel {
        db_models::CollectionAgentModel {
            id: agent.id,
            person_id: agent.person_id,
            license_number: agent.license_number,
            license_expiry: agent.license_expiry,
            status: Self::agent_status_to_db(agent.status),
            assigned_territory_id: agent.assigned_territory_id,
            agent_performance_metrics_id: agent.agent_performance_metrics_id,
            cash_limit: agent.cash_limit,
            device_information_id: agent.device_information_id,
            created_at: agent.created_at,
            updated_at: agent.updated_at,
        }
    }

    pub fn collection_agent_from_db(
        model: db_models::CollectionAgentModel,
    ) -> domain::CollectionAgent {
        domain::CollectionAgent {
            id: model.id,
            person_id: model.person_id,
            license_number: model.license_number,
            license_expiry: model.license_expiry,
            status: Self::agent_status_from_db(model.status),
            assigned_territory_id: model.assigned_territory_id,
            agent_performance_metrics_id: model.agent_performance_metrics_id,
            cash_limit: model.cash_limit,
            device_information_id: model.device_information_id,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }

    // Territory
    pub fn territory_to_db(territory: domain::Territory) -> db_models::TerritoryModel {
        db_models::TerritoryModel {
            id: territory.id,
            territory_name: territory.territory_name,
            coverage_area_id: territory.coverage_area_id,
            customer_count: territory.customer_count,
            route_optimization_enabled: territory.route_optimization_enabled,
            territory_manager_person_id: territory.territory_manager_person_id,
        }
    }

    pub fn territory_from_db(model: db_models::TerritoryModel) -> domain::Territory {
        domain::Territory {
            id: model.id,
            territory_name: model.territory_name,
            coverage_area_id: model.coverage_area_id,
            customer_count: model.customer_count,
            route_optimization_enabled: model.route_optimization_enabled,
            territory_manager_person_id: model.territory_manager_person_id,
        }
    }

    // CoverageArea
    pub fn coverage_area_to_db(area: domain::CoverageArea) -> db_models::CoverageAreaModel {
        db_models::CoverageAreaModel {
            id: area.id,
            area_name: area.area_name,
            area_type: Self::area_type_to_db(area.area_type),
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
            customer_density: Self::customer_density_to_db(area.customer_density),
            transport_mode: Self::transport_mode_to_db(area.transport_mode),
        }
    }

    pub fn coverage_area_from_db(model: db_models::CoverageAreaModel) -> domain::CoverageArea {
        domain::CoverageArea {
            id: model.id,
            area_name: model.area_name,
            area_type: Self::area_type_from_db(model.area_type),
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
            customer_density: Self::customer_density_from_db(model.customer_density),
            transport_mode: Self::transport_mode_from_db(model.transport_mode),
        }
    }

    // AgentPerformanceMetrics
    pub fn agent_performance_metrics_to_db(
        metrics: domain::AgentPerformanceMetrics,
    ) -> db_models::AgentPerformanceMetricsModel {
        db_models::AgentPerformanceMetricsModel {
            id: metrics.id,
            collection_rate: metrics.collection_rate,
            customer_satisfaction_score: metrics.customer_satisfaction_score,
            punctuality_score: metrics.punctuality_score,
            cash_handling_accuracy: metrics.cash_handling_accuracy,
            compliance_score: metrics.compliance_score,
            total_collections: metrics.total_collections,
            total_amount_collected: metrics.total_amount_collected,
            average_collection_time_minutes: metrics.average_collection_time_minutes,
            customer_retention_rate: metrics.customer_retention_rate,
            route_efficiency: metrics.route_efficiency,
            monthly_targets_id: metrics.monthly_targets_id,
        }
    }

    pub fn agent_performance_metrics_from_db(
        model: db_models::AgentPerformanceMetricsModel,
        alerts: Vec<domain::PerformanceAlert>,
    ) -> domain::AgentPerformanceMetrics {
        domain::AgentPerformanceMetrics {
            id: model.id,
            collection_rate: model.collection_rate,
            customer_satisfaction_score: model.customer_satisfaction_score,
            punctuality_score: model.punctuality_score,
            cash_handling_accuracy: model.cash_handling_accuracy,
            compliance_score: model.compliance_score,
            total_collections: model.total_collections,
            total_amount_collected: model.total_amount_collected,
            average_collection_time_minutes: model.average_collection_time_minutes,
            customer_retention_rate: model.customer_retention_rate,
            route_efficiency: model.route_efficiency,
            monthly_targets_id: model.monthly_targets_id,
            performance_alert_1_id: alerts.get(0).map(|a| a.id),
            performance_alert_2_id: alerts.get(1).map(|a| a.id),
            performance_alert_3_id: alerts.get(2).map(|a| a.id),
            performance_alert_4_id: alerts.get(3).map(|a| a.id),
            performance_alert_5_id: alerts.get(4).map(|a| a.id),
        }
    }

    // MonthlyTargets
    pub fn monthly_targets_to_db(
        targets: domain::MonthlyTargets,
    ) -> db_models::MonthlyTargetsModel {
        db_models::MonthlyTargetsModel {
            id: targets.id,
            collection_target: targets.collection_target,
            customer_target: targets.customer_target,
            satisfaction_target: targets.satisfaction_target,
            punctuality_target: targets.punctuality_target,
            accuracy_target: targets.accuracy_target,
        }
    }

    pub fn monthly_targets_from_db(
        model: db_models::MonthlyTargetsModel,
    ) -> domain::MonthlyTargets {
        domain::MonthlyTargets {
            id: model.id,
            collection_target: model.collection_target,
            customer_target: model.customer_target,
            satisfaction_target: model.satisfaction_target,
            punctuality_target: model.punctuality_target,
            accuracy_target: model.accuracy_target,
        }
    }

    // PerformanceAlert
    pub fn performance_alert_to_db(
        alert: domain::PerformanceAlert,
    ) -> db_models::PerformanceAlertModel {
        db_models::PerformanceAlertModel {
            id: alert.id,
            agent_performance_metrics_id: alert.agent_performance_metrics_id,
            alert_type: Self::collection_alert_type_to_db(alert.alert_type),
            severity: CollateralMapper::alert_severity_to_db(alert.severity),
            message: alert.message,
            acknowledged: alert.acknowledged,
            resolution_required: alert.resolution_required,
            created_at: alert.created_at,
            acknowledged_at: alert.acknowledged_at,
            resolved_at: alert.resolved_at,
        }
    }

    pub fn performance_alert_from_db(
        model: db_models::PerformanceAlertModel,
    ) -> domain::PerformanceAlert {
        domain::PerformanceAlert {
            id: model.id,
            agent_performance_metrics_id: model.agent_performance_metrics_id,
            alert_type: Self::collection_alert_type_from_db(model.alert_type),
            severity: CollateralMapper::alert_severity_from_db(model.severity),
            message: model.message,
            acknowledged: model.acknowledged,
            resolution_required: model.resolution_required,
            created_at: model.created_at,
            acknowledged_at: model.acknowledged_at,
            resolved_at: model.resolved_at,
        }
    }

    // DeviceInformation
    pub fn device_information_to_db(
        info: domain::DeviceInformation,
    ) -> db_models::DeviceInformationModel {
        db_models::DeviceInformationModel {
            id: info.id,
            external_id: info.external_id,
            device_type: Self::device_type_to_db(info.device_type),
            model: info.model,
            os_version: info.os_version,
            app_version: info.app_version,
            last_sync: info.last_sync,
            battery_level: info.battery_level,
            connectivity_status: Self::connectivity_status_to_db(info.connectivity_status),
            security_features_id: info.security_features_id,
        }
    }

    pub fn device_information_from_db(
        model: db_models::DeviceInformationModel,
    ) -> domain::DeviceInformation {
        domain::DeviceInformation {
            id: model.id,
            external_id: model.external_id,
            device_type: Self::device_type_from_db(model.device_type),
            model: model.model,
            os_version: model.os_version,
            app_version: model.app_version,
            last_sync: model.last_sync,
            battery_level: model.battery_level,
            connectivity_status: Self::connectivity_status_from_db(model.connectivity_status),
            security_features_id: model.security_features_id,
        }
    }

    // CollectionSecurityFeatures
    pub fn collection_security_features_to_db(
        features: domain::CollectionSecurityFeatures,
    ) -> db_models::CollectionSecurityFeaturesModel {
        db_models::CollectionSecurityFeaturesModel {
            id: features.id,
            biometric_enabled: features.biometric_enabled,
            pin_protection: features.pin_protection,
            encryption_enabled: features.encryption_enabled,
            remote_wipe_enabled: features.remote_wipe_enabled,
            certificate_installed: features.certificate_installed,
            last_security_scan: features.last_security_scan,
        }
    }

    pub fn collection_security_features_from_db(
        model: db_models::CollectionSecurityFeaturesModel,
    ) -> domain::CollectionSecurityFeatures {
        domain::CollectionSecurityFeatures {
            id: model.id,
            biometric_enabled: model.biometric_enabled,
            pin_protection: model.pin_protection,
            encryption_enabled: model.encryption_enabled,
            remote_wipe_enabled: model.remote_wipe_enabled,
            certificate_installed: model.certificate_installed,
            last_security_scan: model.last_security_scan,
        }
    }

    // CollectionProgram
    pub fn collection_program_to_db(
        program: &domain::CollectionProgram,
        criteria: &domain::GraduationCriteria,
        fee: &domain::FeeStructure,
    ) -> db_models::CollectionProgramModel {
        db_models::CollectionProgramModel {
            id: program.id,
            name: program.name.clone(),
            description: program.description.clone(),
            program_type: Self::collection_program_type_to_db(program.program_type),
            status: Self::program_status_to_db(program.status),
            start_date: program.start_date,
            end_date: program.end_date,
            collection_frequency: Self::collection_frequency_to_db(program.collection_frequency),
            operating_hours_id: program.operating_hours_id,
            minimum_amount: program.minimum_amount,
            maximum_amount: program.maximum_amount,
            target_amount: program.target_amount,
            program_duration_days: program.program_duration_days,
            graduation_minimum_balance: criteria.minimum_balance,
            graduation_minimum_collection_rate: criteria.minimum_collection_rate,
            graduation_minimum_duration_days: criteria.minimum_duration_days,
            graduation_consecutive_collections_required: criteria.consecutive_collections_required,
            graduation_target_achievement_required: criteria.target_achievement_required,
            graduation_auto_graduation_enabled: criteria.auto_graduation_enabled,
            fee_setup_fee: fee.setup_fee,
            fee_collection_fee: fee.collection_fee,
            fee_maintenance_fee: fee.maintenance_fee,
            fee_graduation_fee: fee.graduation_fee,
            fee_early_termination_fee: fee.early_termination_fee,
            fee_frequency: Self::collection_fee_frequency_to_db(fee.fee_frequency),
            interest_rate: program.interest_rate,
            created_at: program.created_at,
            updated_at: program.updated_at,
            created_by_person_id: program.created_by_person_id,
            reason_id: program.reason_id,
        }
    }

    pub fn collection_program_from_db(
        model: db_models::CollectionProgramModel,
    ) -> (
        domain::CollectionProgram,
        domain::GraduationCriteria,
        domain::FeeStructure,
    ) {
        let graduation_criteria_id = Uuid::new_v4();
        let fee_structure_id = Uuid::new_v4();

        let graduation_criteria = domain::GraduationCriteria {
            id: graduation_criteria_id,
            minimum_balance: model.graduation_minimum_balance,
            minimum_collection_rate: model.graduation_minimum_collection_rate,
            minimum_duration_days: model.graduation_minimum_duration_days,
            consecutive_collections_required: model.graduation_consecutive_collections_required,
            target_achievement_required: model.graduation_target_achievement_required,
            auto_graduation_enabled: model.graduation_auto_graduation_enabled,
        };

        let fee_structure = domain::FeeStructure {
            id: fee_structure_id,
            setup_fee: model.fee_setup_fee,
            collection_fee: model.fee_collection_fee,
            maintenance_fee: model.fee_maintenance_fee,
            graduation_fee: model.fee_graduation_fee,
            early_termination_fee: model.fee_early_termination_fee,
            fee_frequency: Self::collection_fee_frequency_from_db(model.fee_frequency),
        };

        let program = domain::CollectionProgram {
            id: model.id,
            name: model.name,
            description: model.description,
            program_type: Self::collection_program_type_from_db(model.program_type),
            status: Self::program_status_from_db(model.status),
            start_date: model.start_date,
            end_date: model.end_date,
            collection_frequency: Self::collection_frequency_from_db(model.collection_frequency),
            operating_hours_id: model.operating_hours_id,
            minimum_amount: model.minimum_amount,
            maximum_amount: model.maximum_amount,
            target_amount: model.target_amount,
            program_duration_days: model.program_duration_days,
            graduation_criteria_id,
            fee_structure_id,
            interest_rate: model.interest_rate,
            created_at: model.created_at,
            updated_at: model.updated_at,
            created_by_person_id: model.created_by_person_id,
            reason_id: model.reason_id,
        };

        (program, graduation_criteria, fee_structure)
    }

    // CustomerCollectionProfile
    pub fn customer_collection_profile_to_db(
        profile: &domain::CustomerCollectionProfile,
    ) -> db_models::CustomerCollectionProfileModel {
        db_models::CustomerCollectionProfileModel {
            id: profile.id,
            customer_id: profile.customer_id,
            collection_program_id: profile.collection_program_id,
            account_id: profile.account_id,
            enrollment_date: profile.enrollment_date,
            status: Self::collection_status_to_db(profile.status),
            daily_amount: profile.daily_amount,
            schedule_frequency: Self::collection_frequency_to_db(
                profile.collection_schedule.frequency,
            ),
            schedule_collection_time: profile.collection_schedule.collection_time,
            schedule_timezone: profile.collection_schedule.timezone.clone(),
            schedule_holiday_handling: Self::holiday_handling_to_db(
                profile.collection_schedule.holiday_handling,
            ),
            assigned_collection_agent_id: profile.assigned_collection_agent_id,
            collection_location_address_id: profile.collection_location_address_id,
            performance_collection_rate: profile.collection_performance_metrics.collection_rate,
            performance_total_collections: profile.collection_performance_metrics.total_collections,
            performance_total_amount_collected: profile
                .collection_performance_metrics
                .total_amount_collected,
            performance_average_collection_amount: profile
                .collection_performance_metrics
                .average_collection_amount,
            performance_consecutive_collections: profile
                .collection_performance_metrics
                .consecutive_collections,
            performance_missed_collections: profile
                .collection_performance_metrics
                .missed_collections,
            performance_last_collection_date: profile
                .collection_performance_metrics
                .last_collection_date,
            performance_score: profile.collection_performance_metrics.performance_score,
            performance_reliability_rating: Self::reliability_rating_to_db(
                profile.collection_performance_metrics.reliability_rating,
            ),
            graduation_current_balance: profile.graduation_progress.current_balance,
            graduation_target_balance: profile.graduation_progress.target_balance,
            graduation_days_in_program: profile.graduation_progress.days_in_program,
            graduation_minimum_days_required: profile.graduation_progress.minimum_days_required,
            graduation_collection_consistency_rate: profile
                .graduation_progress
                .collection_consistency_rate,
            graduation_minimum_consistency_required: profile
                .graduation_progress
                .minimum_consistency_required,
            graduation_eligible: profile.graduation_progress.graduation_eligible,
            graduation_date: profile.graduation_progress.graduation_date,
            graduation_next_review_date: profile.graduation_progress.next_review_date,
            created_at: profile.created_at,
            updated_at: profile.updated_at,
            reason_id: profile.reason_id,
        }
    }

    pub fn customer_collection_profile_from_db(
        model: db_models::CustomerCollectionProfileModel,
    ) -> domain::CustomerCollectionProfile {
        let schedule = domain::CollectionSchedule {
            id: Uuid::new_v4(),
            frequency: Self::collection_frequency_from_db(model.schedule_frequency),
            collection_time: model.schedule_collection_time,
            timezone: model.schedule_timezone,
            holiday_handling: Self::holiday_handling_from_db(model.schedule_holiday_handling),
        };

        let metrics = domain::CollectionPerformanceMetrics {
            id: Uuid::new_v4(),
            collection_rate: model.performance_collection_rate,
            total_collections: model.performance_total_collections,
            total_amount_collected: model.performance_total_amount_collected,
            average_collection_amount: model.performance_average_collection_amount,
            consecutive_collections: model.performance_consecutive_collections,
            missed_collections: model.performance_missed_collections,
            last_collection_date: model.performance_last_collection_date,
            performance_score: model.performance_score,
            reliability_rating: Self::reliability_rating_from_db(
                model.performance_reliability_rating,
            ),
        };

        let progress = domain::GraduationProgress {
            id: Uuid::new_v4(),
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
        };

        domain::CustomerCollectionProfile {
            id: model.id,
            customer_id: model.customer_id,
            collection_program_id: model.collection_program_id,
            account_id: model.account_id,
            enrollment_date: model.enrollment_date,
            status: Self::collection_status_from_db(model.status),
            daily_amount: model.daily_amount,
            collection_schedule: schedule,
            assigned_collection_agent_id: model.assigned_collection_agent_id,
            collection_location_address_id: model.collection_location_address_id,
            collection_performance_metrics: metrics,
            graduation_progress: progress,
            created_at: model.created_at,
            updated_at: model.updated_at,
            reason_id: model.reason_id,
        }
    }

    // CollectionRecord
    pub fn collection_record_to_db(
        record: &domain::CollectionRecord,
        verification: Option<&domain::CollectionVerification>,
        biometric: Option<&domain::BiometricData>,
        photo: Option<&domain::PhotoEvidence>,
        witness: Option<&domain::WitnessInformation>,
    ) -> db_models::CollectionRecordModel {
        db_models::CollectionRecordModel {
            id: record.id,
            customer_id: record.customer_id,
            collection_agent_id: record.collection_agent_id,
            collection_program_id: record.collection_program_id,
            account_id: record.account_id,
            collection_date: record.collection_date,
            collection_time: record.collection_time,
            amount: record.amount,
            currency: record.currency.clone(),
            collection_method: Self::collection_method_to_db(record.collection_method),
            location_address_id: record.location_address_id,
            receipt_number: record.receipt_number.clone(),
            status: Self::collection_record_status_to_db(record.status),
            notes: record.notes.clone(),
            verification_customer_signature: verification.and_then(|v| v.customer_signature.clone()),
            verification_agent_verification_code: verification
                .and_then(|v| v.agent_verification_code.clone()),
            verification_fingerprint_hash: biometric.and_then(|b| b.fingerprint_hash.clone()),
            verification_face_recognition_score: biometric.and_then(|b| b.face_recognition_score),
            verification_biometric_method: biometric
                .map(|b| Self::biometric_method_to_db(b.verification_method)),
            verification_confidence_level: biometric.map(|b| b.confidence_level),
            verification_customer_photo_hash: photo.and_then(|p| p.customer_photo_hash.clone()),
            verification_receipt_photo_hash: photo.and_then(|p| p.receipt_photo_hash.clone()),
            verification_location_photo_hash: photo.and_then(|p| p.location_photo_hash.clone()),
            verification_photo_timestamp: photo.map(|p| p.photo_timestamp),
            verification_witness_name: witness.map(|w| w.witness_name.clone()),
            verification_witness_contact: witness.map(|w| w.witness_contact.clone()),
            verification_witness_relationship: witness.map(|w| w.witness_relationship.clone()),
            verification_witness_signature: witness.and_then(|w| w.witness_signature.clone()),
            verification_timestamp: verification.map(|v| v.verification_timestamp),
            created_at: record.created_at,
            processed_at: record.processed_at,
            reason_id: record.reason_id,
        }
    }

    pub fn collection_record_from_db(
        model: db_models::CollectionRecordModel,
    ) -> (
        domain::CollectionRecord,
        Option<domain::CollectionVerification>,
        Option<domain::BiometricData>,
        Option<domain::PhotoEvidence>,
        Option<domain::WitnessInformation>,
    ) {
        let verification_id = Uuid::new_v4();
        let biometric_id = Uuid::new_v4();
        let photo_id = Uuid::new_v4();
        let witness_id = Uuid::new_v4();

        let record = domain::CollectionRecord {
            id: model.id,
            customer_id: model.customer_id,
            collection_agent_id: model.collection_agent_id,
            collection_program_id: model.collection_program_id,
            account_id: model.account_id,
            collection_date: model.collection_date,
            collection_time: model.collection_time,
            amount: model.amount,
            currency: model.currency,
            collection_method: Self::collection_method_from_db(model.collection_method),
            location_address_id: model.location_address_id,
            receipt_number: model.receipt_number,
            status: Self::collection_record_status_from_db(model.status),
            notes: model.notes,
            collection_verification_id: if model.verification_timestamp.is_some() {
                Some(verification_id)
            } else {
                None
            },
            created_at: model.created_at,
            processed_at: model.processed_at,
            reason_id: model.reason_id,
        };

        let verification = if let Some(timestamp) = model.verification_timestamp {
            Some(domain::CollectionVerification {
                id: verification_id,
                collection_record_id: record.id,
                customer_signature: model.verification_customer_signature,
                agent_verification_code: model.verification_agent_verification_code,
                biometric_data_id: if model.verification_biometric_method.is_some() {
                    Some(biometric_id)
                } else {
                    None
                },
                photo_evidence_id: if model.verification_photo_timestamp.is_some() {
                    Some(photo_id)
                } else {
                    None
                },
                witness_person_id: if model.verification_witness_name.is_some() {
                    Some(witness_id)
                } else {
                    None
                },
                verification_timestamp: timestamp,
            })
        } else {
            None
        };

        let biometric = if let (Some(method), Some(confidence)) = (
            model.verification_biometric_method,
            model.verification_confidence_level,
        ) {
            Some(domain::BiometricData {
                id: biometric_id,
                collection_verification_id: verification_id,
                fingerprint_hash: model.verification_fingerprint_hash,
                face_recognition_score: model.verification_face_recognition_score,
                verification_method: Self::biometric_method_from_db(method),
                confidence_level: confidence,
            })
        } else {
            None
        };

        let photo = if let Some(timestamp) = model.verification_photo_timestamp {
            Some(domain::PhotoEvidence {
                id: photo_id,
                collection_verification_id: verification_id,
                customer_photo_hash: model.verification_customer_photo_hash,
                receipt_photo_hash: model.verification_receipt_photo_hash,
                location_photo_hash: model.verification_location_photo_hash,
                photo_timestamp: timestamp,
            })
        } else {
            None
        };

        let witness = if let (Some(name), Some(contact), Some(relationship)) = (
            model.verification_witness_name,
            model.verification_witness_contact,
            model.verification_witness_relationship,
        ) {
            Some(domain::WitnessInformation {
                id: witness_id,
                collection_verification_id: verification_id,
                witness_name: name,
                witness_contact: contact,
                witness_relationship: relationship,
                witness_signature: model.verification_witness_signature,
            })
        } else {
            None
        };

        (record, verification, biometric, photo, witness)
    }

    // CollectionBatch
    pub fn collection_batch_to_db(
        batch: &domain::CollectionBatch,
        reconciliation: Option<&domain::ReconciliationData>,
    ) -> db_models::CollectionBatchModel {
        db_models::CollectionBatchModel {
            id: batch.id,
            collection_agent_id: batch.collection_agent_id,
            collection_date: batch.collection_date,
            total_collections: batch.total_collections,
            total_amount: batch.total_amount,
            currency: batch.currency.clone(),
            status: Self::batch_status_to_db(batch.status),
            collection_records: batch.collection_records.clone(),
            reconciliation_expected_amount: reconciliation.map(|r| r.expected_amount),
            reconciliation_actual_amount: reconciliation.map(|r| r.actual_amount),
            reconciliation_variance: reconciliation.map(|r| r.variance),
            reconciliation_variance_reason: reconciliation.and_then(|r| r.variance_reason.clone()),
            reconciled_by_person_id: reconciliation.map(|r| r.reconciled_by_person_id),
            reconciliation_timestamp: reconciliation.map(|r| r.reconciliation_timestamp),
            reconciliation_adjustment_required: reconciliation.map(|r| r.adjustment_required),
            created_at: batch.created_at,
            processed_at: batch.processed_at,
        }
    }

    pub fn collection_batch_from_db(
        model: db_models::CollectionBatchModel,
    ) -> (domain::CollectionBatch, Option<domain::ReconciliationData>) {
        let reconciliation_id = Uuid::new_v4();

        let batch = domain::CollectionBatch {
            id: model.id,
            collection_agent_id: model.collection_agent_id,
            collection_date: model.collection_date,
            total_collections: model.total_collections,
            total_amount: model.total_amount,
            currency: model.currency,
            status: Self::batch_status_from_db(model.status),
            collection_records: model.collection_records,
            reconciliation_data_id: if model.reconciliation_timestamp.is_some() {
                Some(reconciliation_id)
            } else {
                None
            },
            created_at: model.created_at,
            processed_at: model.processed_at,
        };

        let reconciliation = if let (
            Some(expected),
            Some(actual),
            Some(variance),
            Some(person_id),
            Some(timestamp),
            Some(adjustment),
        ) = (
            model.reconciliation_expected_amount,
            model.reconciliation_actual_amount,
            model.reconciliation_variance,
            model.reconciled_by_person_id,
            model.reconciliation_timestamp,
            model.reconciliation_adjustment_required,
        ) {
            Some(domain::ReconciliationData {
                id: reconciliation_id,
                collection_batch_id: batch.id,
                expected_amount: expected,
                actual_amount: actual,
                variance,
                variance_reason: model.reconciliation_variance_reason,
                reconciled_by_person_id: person_id,
                reconciliation_timestamp: timestamp,
                adjustment_required: adjustment,
            })
        } else {
            None
        };

        (batch, reconciliation)
    }
}
use banking_api::domain::{
    KycResult, KycCheck, CheckResult, ScreeningResult, ScreeningType, SanctionsMatch,
    RiskLevel, MonitoringResult, ComplianceAlert, Severity, AlertStatus,
    compliance::AlertType,
    SarData, SarStatus, UboVerificationResult, UboLink, MonitoringRules
};
use banking_db::models::{
    // Domain-aligned models
    KycResultModel, KycCheckModel, ScreeningResultModel, SanctionsMatchModel,
    ComplianceAlertModel, SarDataModel, UboVerificationResultModel, UboLinkModel,
    MonitoringResultModel, MonitoringRulesModel, ComplianceResultModel,
    // Legacy models for repository compatibility
    KycRecordModel, SanctionsScreeningModel,
    // Enums
    CheckType as DbCheckType, CheckResult as DbCheckResult, ScreeningType as DbScreeningType,
    RiskLevel as DbRiskLevel, Severity as DbSeverity,
    compliance::AlertType as DbAlertType,
    AlertStatus as DbAlertStatus, SarStatus as DbSarStatus, KycStatus as DbKycStatus,
    ControlType as DbControlType, VerificationStatus as DbVerificationStatus,
    ComplianceStatus as DbComplianceStatus
};
use heapless::String as HeaplessString;
use chrono::Utc;
use uuid::Uuid;

pub struct ComplianceMapper;

impl ComplianceMapper {
    /// Map from domain KycResult to database KycResultModel (new aligned version)
    pub fn kyc_result_to_result_model(kyc_result: KycResult) -> KycResultModel {
        KycResultModel {
            customer_id: kyc_result.customer_id,
            status: Self::domain_kyc_status_to_db_kyc_status(kyc_result.status),
            completed_checks: kyc_result.completed_checks.into_iter()
                .map(Self::kyc_check_to_model)
                .collect(),
            missing_documents: kyc_result.missing_documents,
            risk_score: kyc_result.risk_score,
            verified_at: kyc_result.verified_at,
        }
    }

    /// Map from domain KycCheck to database KycCheckModel
    pub fn kyc_check_to_model(kyc_check: KycCheck) -> KycCheckModel {
        KycCheckModel {
            check_type: kyc_check.check_type,
            result: Self::domain_check_result_to_db_check_result(kyc_check.result),
            details: kyc_check.details,
            performed_at: kyc_check.performed_at,
        }
    }

    /// Map from domain ScreeningResult to database ScreeningResultModel (new aligned version)
    pub fn screening_result_to_result_model(screening_result: ScreeningResult) -> ScreeningResultModel {
        ScreeningResultModel {
            customer_id: screening_result.customer_id,
            screening_type: Self::domain_screening_type_to_db_screening_type(screening_result.screening_type),
            matches_found: screening_result.matches_found.into_iter()
                .map(Self::sanctions_match_to_model)
                .collect(),
            risk_level: Self::domain_risk_level_to_db_risk_level(screening_result.risk_level),
            screened_at: screening_result.screened_at,
            requires_manual_review: screening_result.requires_manual_review,
        }
    }

    /// Map from domain SanctionsMatch to database SanctionsMatchModel
    pub fn sanctions_match_to_model(sanctions_match: SanctionsMatch) -> SanctionsMatchModel {
        SanctionsMatchModel {
            matched_name: sanctions_match.matched_name,
            confidence_score: sanctions_match.confidence_score,
            details: sanctions_match.details,
            list_source: sanctions_match.list_source,
        }
    }

    /// Map from domain ComplianceAlert to database ComplianceAlertModel
    pub fn compliance_alert_to_model(alert: ComplianceAlert) -> ComplianceAlertModel {
        ComplianceAlertModel {
            alert_id: alert.alert_id,
            alert_type: Self::domain_alert_type_to_db_alert_type(alert.alert_type),
            description: alert.description,
            severity: Self::domain_severity_to_db_severity(alert.severity),
            triggered_at: alert.triggered_at,
            status: Self::domain_alert_status_to_db_alert_status(alert.status),
        }
    }

    /// Map from domain SarData to database SarDataModel
    pub fn sar_data_to_model(sar_data: SarData) -> SarDataModel {
        SarDataModel {
            sar_id: sar_data.sar_id,
            customer_id: sar_data.customer_id,
            reason_id: sar_data.reason_id,
            additional_details: sar_data.additional_details,
            supporting_transactions: sar_data.supporting_transactions,
            generated_at: sar_data.generated_at,
            status: Self::domain_sar_status_to_db_sar_status(sar_data.status),
        }
    }

    /// Map from domain UboVerificationResult to database UboVerificationResultModel
    pub fn ubo_verification_result_to_model(ubo_result: UboVerificationResult) -> UboVerificationResultModel {
        UboVerificationResultModel {
            corporate_customer_id: ubo_result.corporate_customer_id,
            ubo_chain: ubo_result.ubo_chain.into_iter()
                .map(Self::ubo_link_to_model)
                .collect(),
            verification_complete: ubo_result.verification_complete,
            requires_update: ubo_result.requires_update,
        }
    }

    /// Map from domain UboLink to database UboLinkModel
    pub fn ubo_link_to_model(ubo_link: UboLink) -> UboLinkModel {
        UboLinkModel {
            ubo_id: ubo_link.ubo_id,
            beneficiary_customer_id: ubo_link.beneficiary_customer_id,
            ownership_percentage: ubo_link.ownership_percentage,
            control_type: Self::domain_control_type_to_db_control_type(ubo_link.control_type),
            verification_status: Self::domain_verification_status_to_db_verification_status(ubo_link.verification_status),
        }
    }

    /// Map from domain MonitoringResult to database MonitoringResultModel
    pub fn monitoring_result_to_model(monitoring_result: MonitoringResult) -> MonitoringResultModel {
        MonitoringResultModel {
            transaction_id: monitoring_result.transaction_id,
            alerts_triggered: monitoring_result.alerts_triggered.into_iter()
                .map(Self::compliance_alert_to_model)
                .collect(),
            risk_score: monitoring_result.risk_score,
            requires_investigation: monitoring_result.requires_investigation,
            auto_approved: monitoring_result.auto_approved,
        }
    }

    /// Map from domain MonitoringRules to database MonitoringRulesModel
    pub fn monitoring_rules_to_model(monitoring_rules: MonitoringRules) -> MonitoringRulesModel {
        MonitoringRulesModel {
            structuring_detection: monitoring_rules.structuring_detection,
            velocity_checks: monitoring_rules.velocity_checks,
            geographic_risk_assessment: monitoring_rules.geographic_risk_assessment,
            large_cash_threshold: monitoring_rules.large_cash_threshold,
            suspicious_pattern_detection: monitoring_rules.suspicious_pattern_detection,
            cross_border_transaction_monitoring: monitoring_rules.cross_border_transaction_monitoring,
        }
    }

    /// Legacy compatibility - Map from domain KycResult to database KycRecordModel 
    pub fn kyc_result_to_record_model(kyc_result: KycResult) -> KycRecordModel {
        KycRecordModel {
            kyc_id: Uuid::new_v4(),
            customer_id: kyc_result.customer_id,
            status: Self::domain_kyc_status_to_db_kyc_status(kyc_result.status),
            risk_assessment: HeaplessString::try_from("Standard").unwrap_or_default(),
            verification_level: HeaplessString::try_from("Basic").unwrap_or_default(),
            documents_verified: HeaplessString::try_from("[]").unwrap_or_default(), // JSON array
            last_review_date: None,
            next_review_date: None,
            reviewed_by: None,
            verification_notes: None,
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
            updated_by: HeaplessString::try_from("system").unwrap_or_default(),
        }
    }

    /// Legacy compatibility - Map from domain ScreeningResult to database SanctionsScreeningModel
    pub fn screening_result_to_screening_model(screening_result: ScreeningResult) -> SanctionsScreeningModel {
        SanctionsScreeningModel {
            screening_id: Uuid::new_v4(),
            customer_id: screening_result.customer_id,
            screening_date: screening_result.screened_at,
            screening_result: Self::screening_type_to_heapless_string(screening_result.screening_type),
            match_details: None, // TODO: Convert matches_found to JSON
            risk_score: None,
            screening_provider: HeaplessString::try_from("DefaultProvider").unwrap_or_default(),
            status: HeaplessString::try_from("Completed").unwrap_or_default(),
            reviewed_by: None,
            review_notes: None,
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
        }
    }

    // Enum conversion helper methods
    fn domain_kyc_status_to_db_kyc_status(status: banking_api::domain::customer::KycStatus) -> DbKycStatus {
        match status {
            banking_api::domain::customer::KycStatus::NotStarted => DbKycStatus::NotStarted,
            banking_api::domain::customer::KycStatus::InProgress => DbKycStatus::InProgress,
            banking_api::domain::customer::KycStatus::Pending => DbKycStatus::Pending,
            banking_api::domain::customer::KycStatus::Complete => DbKycStatus::Complete,
            banking_api::domain::customer::KycStatus::Approved => DbKycStatus::Approved,
            banking_api::domain::customer::KycStatus::Rejected => DbKycStatus::Rejected,
            banking_api::domain::customer::KycStatus::RequiresUpdate => DbKycStatus::RequiresUpdate,
            banking_api::domain::customer::KycStatus::Failed => DbKycStatus::Failed,
        }
    }

    fn domain_check_result_to_db_check_result(result: CheckResult) -> DbCheckResult {
        match result {
            CheckResult::Pass => DbCheckResult::Pass,
            CheckResult::Fail => DbCheckResult::Fail,
            CheckResult::Warning => DbCheckResult::Warning,
            CheckResult::Manual => DbCheckResult::Manual,
        }
    }

    fn domain_screening_type_to_db_screening_type(screening_type: ScreeningType) -> DbScreeningType {
        match screening_type {
            ScreeningType::Sanctions => DbScreeningType::Sanctions,
            ScreeningType::PoliticallyExposed => DbScreeningType::PoliticallyExposed,
            ScreeningType::AdverseMedia => DbScreeningType::AdverseMedia,
            ScreeningType::Watchlist => DbScreeningType::Watchlist,
        }
    }

    fn domain_risk_level_to_db_risk_level(risk_level: RiskLevel) -> DbRiskLevel {
        match risk_level {
            RiskLevel::Low => DbRiskLevel::Low,
            RiskLevel::Medium => DbRiskLevel::Medium,
            RiskLevel::High => DbRiskLevel::High,
            RiskLevel::Critical => DbRiskLevel::Critical,
        }
    }

    fn domain_alert_type_to_db_alert_type(alert_type: AlertType) -> DbAlertType {
        match alert_type {
            AlertType::StructuringDetection => DbAlertType::StructuringDetection,
            AlertType::VelocityCheck => DbAlertType::VelocityCheck,
            AlertType::LargeCashTransaction => DbAlertType::LargeCashTransaction,
            AlertType::SuspiciousPattern => DbAlertType::SuspiciousPattern,
            AlertType::GeographicAnomaly => DbAlertType::GeographicAnomaly,
            AlertType::CrossBorderTransaction => DbAlertType::CrossBorderTransaction,
        }
    }

    fn domain_severity_to_db_severity(severity: Severity) -> DbSeverity {
        match severity {
            Severity::Low => DbSeverity::Low,
            Severity::Medium => DbSeverity::Medium,
            Severity::High => DbSeverity::High,
            Severity::Critical => DbSeverity::Critical,
        }
    }

    fn domain_alert_status_to_db_alert_status(status: AlertStatus) -> DbAlertStatus {
        match status {
            AlertStatus::New => DbAlertStatus::New,
            AlertStatus::InReview => DbAlertStatus::InReview,
            AlertStatus::Investigated => DbAlertStatus::Investigated,
            AlertStatus::Cleared => DbAlertStatus::Cleared,
            AlertStatus::Escalated => DbAlertStatus::Escalated,
        }
    }

    fn domain_sar_status_to_db_sar_status(status: SarStatus) -> DbSarStatus {
        match status {
            SarStatus::Draft => DbSarStatus::Draft,
            SarStatus::Filed => DbSarStatus::Filed,
            SarStatus::Acknowledged => DbSarStatus::Acknowledged,
        }
    }

    fn domain_control_type_to_db_control_type(control_type: banking_api::domain::account::ControlType) -> DbControlType {
        match control_type {
            banking_api::domain::account::ControlType::DirectOwnership => DbControlType::DirectOwnership,
            banking_api::domain::account::ControlType::IndirectOwnership => DbControlType::IndirectOwnership,
            banking_api::domain::account::ControlType::SignificantInfluence => DbControlType::SignificantInfluence,
            banking_api::domain::account::ControlType::SeniorManagement => DbControlType::SeniorManagement,
        }
    }

    fn domain_verification_status_to_db_verification_status(status: banking_api::domain::account::VerificationStatus) -> DbVerificationStatus {
        match status {
            banking_api::domain::account::VerificationStatus::Pending => DbVerificationStatus::Pending,
            banking_api::domain::account::VerificationStatus::Verified => DbVerificationStatus::Verified,
            banking_api::domain::account::VerificationStatus::Rejected => DbVerificationStatus::Rejected,
            banking_api::domain::account::VerificationStatus::RequiresUpdate => DbVerificationStatus::RequiresUpdate,
        }
    }

    /// Legacy helper methods for backward compatibility
    fn screening_type_to_heapless_string(screening_type: ScreeningType) -> HeaplessString<50> {
        let type_str = match screening_type {
            ScreeningType::Sanctions => "Sanctions",
            ScreeningType::PoliticallyExposed => "PoliticallyExposed",
            ScreeningType::AdverseMedia => "AdverseMedia",
            ScreeningType::Watchlist => "Watchlist",
        };
        HeaplessString::try_from(type_str).unwrap_or_default()
    }

    /// Map domain CheckType to database CheckType  
    pub fn domain_check_type_to_db_check_type(check_type: banking_api::domain::compliance::CheckType) -> DbCheckType {
        match check_type {
            banking_api::domain::compliance::CheckType::Kyc => DbCheckType::Kyc,
            banking_api::domain::compliance::CheckType::Aml => DbCheckType::Aml,
            banking_api::domain::compliance::CheckType::Cdd => DbCheckType::Cdd,
            banking_api::domain::compliance::CheckType::Edd => DbCheckType::Edd,
            banking_api::domain::compliance::CheckType::SanctionsScreening => DbCheckType::SanctionsScreening,
            banking_api::domain::compliance::CheckType::PepScreening => DbCheckType::PepScreening,
            banking_api::domain::compliance::CheckType::AdverseMediaScreening => DbCheckType::AdverseMediaScreening,
            banking_api::domain::compliance::CheckType::WatchlistScreening => DbCheckType::WatchlistScreening,
            banking_api::domain::compliance::CheckType::UboVerification => DbCheckType::UboVerification,
            banking_api::domain::compliance::CheckType::DocumentVerification => DbCheckType::DocumentVerification,
            banking_api::domain::compliance::CheckType::AddressVerification => DbCheckType::AddressVerification,
            banking_api::domain::compliance::CheckType::SourceOfFundsVerification => DbCheckType::SourceOfFundsVerification,
            banking_api::domain::compliance::CheckType::SourceOfWealthVerification => DbCheckType::SourceOfWealthVerification,
            banking_api::domain::compliance::CheckType::RiskAssessment => DbCheckType::RiskAssessment,
            banking_api::domain::compliance::CheckType::OngoingMonitoring => DbCheckType::OngoingMonitoring,
            banking_api::domain::compliance::CheckType::PeriodicReview => DbCheckType::PeriodicReview,
        }
    }

    /// Map database CheckType to domain CheckType
    pub fn db_check_type_to_domain_check_type(db_check_type: DbCheckType) -> banking_api::domain::compliance::CheckType {
        match db_check_type {
            DbCheckType::Kyc => banking_api::domain::compliance::CheckType::Kyc,
            DbCheckType::Aml => banking_api::domain::compliance::CheckType::Aml,
            DbCheckType::Cdd => banking_api::domain::compliance::CheckType::Cdd,
            DbCheckType::Edd => banking_api::domain::compliance::CheckType::Edd,
            DbCheckType::SanctionsScreening => banking_api::domain::compliance::CheckType::SanctionsScreening,
            DbCheckType::PepScreening => banking_api::domain::compliance::CheckType::PepScreening,
            DbCheckType::AdverseMediaScreening => banking_api::domain::compliance::CheckType::AdverseMediaScreening,
            DbCheckType::WatchlistScreening => banking_api::domain::compliance::CheckType::WatchlistScreening,
            DbCheckType::UboVerification => banking_api::domain::compliance::CheckType::UboVerification,
            DbCheckType::DocumentVerification => banking_api::domain::compliance::CheckType::DocumentVerification,
            DbCheckType::AddressVerification => banking_api::domain::compliance::CheckType::AddressVerification,
            DbCheckType::SourceOfFundsVerification => banking_api::domain::compliance::CheckType::SourceOfFundsVerification,
            DbCheckType::SourceOfWealthVerification => banking_api::domain::compliance::CheckType::SourceOfWealthVerification,
            DbCheckType::RiskAssessment => banking_api::domain::compliance::CheckType::RiskAssessment,
            DbCheckType::OngoingMonitoring => banking_api::domain::compliance::CheckType::OngoingMonitoring,
            DbCheckType::PeriodicReview => banking_api::domain::compliance::CheckType::PeriodicReview,
        }
    }

    /// Map domain ComplianceResult to database ComplianceResultModel
    pub fn compliance_result_to_model(result: banking_api::domain::compliance::ComplianceResult) -> ComplianceResultModel {
        ComplianceResultModel {
            result_id: result.result_id,
            account_id: result.account_id,
            check_type: Self::domain_check_type_to_db_check_type(result.check_type),
            status: Self::domain_compliance_status_to_db_compliance_status(result.status),
            risk_score: result.risk_score,
            findings: result.findings,
            recommendations: result.recommendations,
            checked_at: result.checked_at,
            expires_at: result.expires_at,
        }
    }

    /// Helper method to convert domain ComplianceStatus to database ComplianceStatus
    fn domain_compliance_status_to_db_compliance_status(status: banking_api::domain::compliance::ComplianceStatus) -> DbComplianceStatus {
        match status {
            banking_api::domain::compliance::ComplianceStatus::Passed => DbComplianceStatus::Passed,
            banking_api::domain::compliance::ComplianceStatus::Failed => DbComplianceStatus::Failed,
            banking_api::domain::compliance::ComplianceStatus::RequiresReview => DbComplianceStatus::RequiresReview,
            banking_api::domain::compliance::ComplianceStatus::Pending => DbComplianceStatus::Pending,
        }
    }
}
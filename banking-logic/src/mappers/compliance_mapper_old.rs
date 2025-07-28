use banking_api::domain::{
    KycResult, ScreeningResult, ScreeningType, 
    ComplianceAlert, AlertType, Severity, AlertStatus,
    SarData, SarStatus
};
use banking_db::models::{
    KycRecordModel, SanctionsScreeningModel, ComplianceAlertModel, SarDataModel,
    ComplianceResultModel, CheckType as DbCheckType
};
use heapless::String as HeaplessString;
use banking_api::BankingResult;
use chrono::Utc;
use uuid::Uuid;

pub struct ComplianceMapper;

impl ComplianceMapper {
    /// Map from domain KycResult to database KycRecordModel
    pub fn kyc_result_to_model(kyc_result: KycResult) -> KycRecordModel {
        KycRecordModel {
            kyc_id: Uuid::new_v4(),
            customer_id: kyc_result.customer_id,
            status: Self::kyc_status_to_heapless_string(kyc_result.status),
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

    /// Map from domain ScreeningResult to database SanctionsScreeningModel
    pub fn screening_result_to_model(screening_result: ScreeningResult) -> SanctionsScreeningModel {
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

    /// Map from domain ComplianceAlert to database ComplianceAlertModel
    pub fn compliance_alert_to_model(alert: ComplianceAlert) -> ComplianceAlertModel {
        ComplianceAlertModel {
            alert_id: alert.alert_id,
            customer_id: None, // Would need to be derived from context
            transaction_id: None, // Would need to be derived from context
            alert_type: Self::alert_type_to_heapless_string(alert.alert_type),
            severity: Self::severity_to_heapless_string(alert.severity),
            description: alert.description,
            generated_at: alert.triggered_at,
            status: Self::alert_status_to_heapless_string(alert.status),
            assigned_to: None,
            resolved_at: None,
            resolved_by: None,
            resolution_notes: None,
            metadata: None,
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
        }
    }

    /// Map from domain SarData to database SarDataModel
    pub fn sar_data_to_model(sar_data: SarData) -> SarDataModel {
        let supporting_transactions_json = serde_json::to_string(&sar_data.supporting_transactions)
            .unwrap_or_else(|_| "[]".to_string());
        
        SarDataModel {
            sar_id: sar_data.sar_id,
            customer_id: sar_data.customer_id,
            related_transactions: HeaplessString::try_from(supporting_transactions_json.as_str())
                .unwrap_or_default(),
            suspicious_activity_type: HeaplessString::try_from("Unknown").unwrap_or_default(),
            description: HeaplessString::try_from("SAR Generated").unwrap_or_default(),
            amount_involved: None,
            period_start: chrono::Utc::now().date_naive(),
            period_end: chrono::Utc::now().date_naive(),
            status: Self::sar_status_to_heapless_string(sar_data.status),
            prepared_by: HeaplessString::try_from("system").unwrap_or_default(),
            approved_by: None,
            filed_date: None,
            reference_number: None,
            regulatory_response: None,
            supporting_documents: None,
            created_at: sar_data.generated_at,
            last_updated_at: Utc::now(),
            updated_by: HeaplessString::try_from("system").unwrap_or_default(),
        }
    }

    /// Map from database KycRecordModel to domain KycResult
    pub fn kyc_model_to_result(model: KycRecordModel) -> BankingResult<KycResult> {
        Ok(KycResult {
            customer_id: model.customer_id,
            status: Self::heapless_string_to_kyc_status(&model.status)?,
            completed_checks: vec![], // Would need to be populated from separate checks
            missing_documents: vec![], // Would need to be parsed from JSON
            risk_score: None,
            verified_at: None,
        })
    }

    /// Helper methods for enum conversions
    fn kyc_status_to_heapless_string(status: banking_api::domain::customer::KycStatus) -> HeaplessString<50> {
        let status_str = match status {
            banking_api::domain::customer::KycStatus::NotStarted => "NotStarted",
            banking_api::domain::customer::KycStatus::InProgress => "InProgress",
            banking_api::domain::customer::KycStatus::Pending => "Pending",
            banking_api::domain::customer::KycStatus::Complete => "Complete",
            banking_api::domain::customer::KycStatus::Approved => "Approved",
            banking_api::domain::customer::KycStatus::Rejected => "Rejected",
            banking_api::domain::customer::KycStatus::RequiresUpdate => "RequiresUpdate",
            banking_api::domain::customer::KycStatus::Failed => "Failed",
        };
        HeaplessString::try_from(status_str).unwrap_or_default()
    }

    fn heapless_string_to_kyc_status(status: &HeaplessString<50>) -> BankingResult<banking_api::domain::customer::KycStatus> {
        match status.as_str() {
            "NotStarted" => Ok(banking_api::domain::customer::KycStatus::NotStarted),
            "InProgress" => Ok(banking_api::domain::customer::KycStatus::InProgress),
            "Pending" => Ok(banking_api::domain::customer::KycStatus::Pending),
            "Complete" => Ok(banking_api::domain::customer::KycStatus::Complete),
            "Approved" => Ok(banking_api::domain::customer::KycStatus::Approved),
            "Rejected" => Ok(banking_api::domain::customer::KycStatus::Rejected),
            "RequiresUpdate" => Ok(banking_api::domain::customer::KycStatus::RequiresUpdate),
            "Failed" => Ok(banking_api::domain::customer::KycStatus::Failed),
            _ => Err(banking_api::error::BankingError::ValidationError {
                field: "kyc_status".to_string(),
                message: format!("Invalid KYC status: {}", status.as_str()),
            }),
        }
    }

    fn screening_type_to_heapless_string(screening_type: ScreeningType) -> HeaplessString<50> {
        let type_str = match screening_type {
            ScreeningType::Sanctions => "Sanctions",
            ScreeningType::PoliticallyExposed => "PoliticallyExposed",
            ScreeningType::AdverseMedia => "AdverseMedia",
            ScreeningType::Watchlist => "Watchlist",
        };
        HeaplessString::try_from(type_str).unwrap_or_default()
    }

    fn alert_type_to_heapless_string(alert_type: AlertType) -> HeaplessString<50> {
        let type_str = match alert_type {
            AlertType::StructuringDetection => "StructuringDetection",
            AlertType::VelocityCheck => "VelocityCheck",
            AlertType::LargeCashTransaction => "LargeCashTransaction",
            AlertType::SuspiciousPattern => "SuspiciousPattern",
            AlertType::GeographicAnomaly => "GeographicAnomaly",
            AlertType::CrossBorderTransaction => "CrossBorderTransaction",
        };
        HeaplessString::try_from(type_str).unwrap_or_default()
    }

    fn severity_to_heapless_string(severity: Severity) -> HeaplessString<20> {
        let severity_str = match severity {
            Severity::Low => "Low",
            Severity::Medium => "Medium",
            Severity::High => "High",
            Severity::Critical => "Critical",
        };
        HeaplessString::try_from(severity_str).unwrap_or_default()
    }

    fn alert_status_to_heapless_string(status: AlertStatus) -> HeaplessString<50> {
        let status_str = match status {
            AlertStatus::New => "New",
            AlertStatus::InReview => "InReview",
            AlertStatus::Investigated => "Investigated",
            AlertStatus::Cleared => "Cleared",
            AlertStatus::Escalated => "Escalated",
        };
        HeaplessString::try_from(status_str).unwrap_or_default()
    }

    fn sar_status_to_heapless_string(status: SarStatus) -> HeaplessString<50> {
        let status_str = match status {
            SarStatus::Draft => "Draft",
            SarStatus::Filed => "Filed",
            SarStatus::Acknowledged => "Acknowledged",
        };
        HeaplessString::try_from(status_str).unwrap_or_default()
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
            status: HeaplessString::try_from(
                match result.status {
                    banking_api::domain::compliance::ComplianceStatus::Passed => "Passed",
                    banking_api::domain::compliance::ComplianceStatus::Failed => "Failed",
                    banking_api::domain::compliance::ComplianceStatus::RequiresReview => "RequiresReview",
                    banking_api::domain::compliance::ComplianceStatus::Pending => "Pending",
                }
            ).unwrap_or_default(),
            risk_score: result.risk_score,
            findings: result.findings,
            recommendations: result.recommendations,
            checked_at: result.checked_at,
            expires_at: result.expires_at,
        }
    }
}
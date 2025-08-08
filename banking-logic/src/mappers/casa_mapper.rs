use banking_api::domain::{
    OverdraftFacility, OverdraftStatus, ReviewFrequency, OverdraftUtilization,
    OverdraftInterestCalculation, CompoundingFrequency, CasaAccountSummary,
    DormancyRisk, CasaComplianceStatus, OverdraftProcessingJob, ProcessingJobStatus,
    OverdraftLimitAdjustment, CasaApprovalStatus,
    InterestPostingRecord,
    InterestType, PostingStatus, AccountType
};
use banking_api::domain::customer::{KycStatus, RiskRating};
use banking_db::models::{
    OverdraftFacility as DbOverdraftFacility, OverdraftStatus as DbOverdraftStatus,
    ReviewFrequency as DbReviewFrequency, OverdraftUtilization as DbOverdraftUtilization,
    OverdraftInterestCalculation as DbOverdraftInterestCalculation,
    CompoundingFrequency as DbCompoundingFrequency, CasaAccountSummary as DbCasaAccountSummary,
    DormancyRisk as DbDormancyRisk, CasaComplianceStatus as DbCasaComplianceStatus,
    OverdraftProcessingJob as DbOverdraftProcessingJob, ProcessingJobStatus as DbProcessingJobStatus,
    OverdraftLimitAdjustment as DbOverdraftLimitAdjustment, CasaApprovalStatus as DbCasaApprovalStatus,
    InterestPostingRecord as DbInterestPostingRecord,
    InterestType as DbInterestType, PostingStatus as DbPostingStatus
};
use banking_db::models::customer::{KycStatus as DbKycStatus, RiskRating as DbRiskRating};

pub struct CasaMapper;

impl CasaMapper {
    /// Map from domain OverdraftFacility to database model
    pub fn overdraft_facility_to_model(facility: OverdraftFacility) -> DbOverdraftFacility {
        DbOverdraftFacility {
            id: facility.id,
            account_id: facility.account_id,
            approved_limit: facility.approved_limit,
            current_utilized: facility.current_utilized,
            available_limit: facility.available_limit,
            interest_rate: facility.interest_rate,
            facility_status: Self::overdraft_status_to_db(facility.facility_status),
            approval_date: facility.approval_date,
            expiry_date: facility.expiry_date,
            approved_by_person_id: facility.approved_by_person_id,
            review_frequency: Self::review_frequency_to_db(facility.review_frequency),
            next_review_date: facility.next_review_date,
            security_required: facility.security_required,
            security_details: facility.security_details,
            created_at: facility.created_at,
            last_updated_at: facility.last_updated_at,
        }
    }

    /// Map from database OverdraftFacility to domain model
    pub fn overdraft_facility_from_model(model: DbOverdraftFacility) -> banking_api::BankingResult<OverdraftFacility> {
        Ok(OverdraftFacility {
            id: model.id,
            account_id: model.account_id,
            approved_limit: model.approved_limit,
            current_utilized: model.current_utilized,
            available_limit: model.available_limit,
            interest_rate: model.interest_rate,
            facility_status: Self::db_to_overdraft_status(model.facility_status),
            approval_date: model.approval_date,
            expiry_date: model.expiry_date,
            approved_by_person_id: model.approved_by_person_id,
            review_frequency: Self::db_to_review_frequency(model.review_frequency),
            next_review_date: model.next_review_date,
            security_required: model.security_required,
            security_details: model.security_details,
            created_at: model.created_at,
            last_updated_at: model.last_updated_at,
        })
    }

    /// Map from domain OverdraftUtilization to database model
    pub fn overdraft_utilization_to_model(utilization: OverdraftUtilization) -> DbOverdraftUtilization {
        DbOverdraftUtilization {
            id: utilization.id,
            account_id: utilization.account_id,
            utilization_date: utilization.utilization_date,
            opening_balance: utilization.opening_balance,
            closing_balance: utilization.closing_balance,
            average_daily_balance: utilization.average_daily_balance,
            days_overdrawn: utilization.days_overdrawn,
            interest_accrued: utilization.interest_accrued,
            interest_rate: utilization.interest_rate,
            capitalized: utilization.capitalized,
            capitalization_date: utilization.capitalization_date,
            created_at: utilization.created_at,
        }
    }

    /// Map from database OverdraftUtilization to domain model
    pub fn overdraft_utilization_from_model(model: DbOverdraftUtilization) -> OverdraftUtilization {
        OverdraftUtilization {
            id: model.id,
            account_id: model.account_id,
            utilization_date: model.utilization_date,
            opening_balance: model.opening_balance,
            closing_balance: model.closing_balance,
            average_daily_balance: model.average_daily_balance,
            days_overdrawn: model.days_overdrawn,
            interest_accrued: model.interest_accrued,
            interest_rate: model.interest_rate,
            capitalized: model.capitalized,
            capitalization_date: model.capitalization_date,
            created_at: model.created_at,
        }
    }

    /// Map from domain OverdraftInterestCalculation to database model
    pub fn interest_calculation_to_model(calc: OverdraftInterestCalculation) -> DbOverdraftInterestCalculation {
        DbOverdraftInterestCalculation {
            id: calc.id,
            account_id: calc.account_id,
            calculation_period_start: calc.calculation_period_start,
            calculation_period_end: calc.calculation_period_end,
            average_overdrawn_balance: calc.average_overdrawn_balance,
            applicable_rate: calc.applicable_rate,
            days_calculated: calc.days_calculated,
            interest_amount: calc.interest_amount,
            compounding_frequency: Self::compounding_frequency_to_db(calc.compounding_frequency),
            capitalization_due: calc.capitalization_due,
            calculated_at: calc.calculated_at,
            calculated_by_person_id: calc.calculated_by_person_id,
        }
    }

    /// Map from database OverdraftInterestCalculation to domain model
    pub fn interest_calculation_from_model(model: DbOverdraftInterestCalculation) -> OverdraftInterestCalculation {
        OverdraftInterestCalculation {
            id: model.id,
            account_id: model.account_id,
            calculation_period_start: model.calculation_period_start,
            calculation_period_end: model.calculation_period_end,
            average_overdrawn_balance: model.average_overdrawn_balance,
            applicable_rate: model.applicable_rate,
            days_calculated: model.days_calculated,
            interest_amount: model.interest_amount,
            compounding_frequency: Self::db_to_compounding_frequency(model.compounding_frequency),
            capitalization_due: model.capitalization_due,
            calculated_at: model.calculated_at,
            calculated_by_person_id: model.calculated_by_person_id,
        }
    }

    /// Map from domain OverdraftProcessingJob to database model
    pub fn processing_job_to_model(job: OverdraftProcessingJob) -> DbOverdraftProcessingJob {
        DbOverdraftProcessingJob {
            id: job.id,
            processing_date: job.processing_date,
            accounts_processed: job.accounts_processed,
            total_interest_accrued: job.total_interest_accrued,
            accounts_capitalized: job.accounts_capitalized,
            total_capitalized_amount: job.total_capitalized_amount,
            status: Self::processing_job_status_to_db(job.status),
            started_at: job.started_at,
            completed_at: job.completed_at,
            errors_01: job.errors_01,
            errors_02: job.errors_02,
            errors_03: job.errors_03,
            errors_04: job.errors_04,
            errors_05: job.errors_05,
        }
    }

    /// Map from database OverdraftProcessingJob to domain model
    pub fn processing_job_from_model(model: DbOverdraftProcessingJob) -> OverdraftProcessingJob {
        OverdraftProcessingJob {
            id: model.id,
            processing_date: model.processing_date,
            accounts_processed: model.accounts_processed,
            total_interest_accrued: model.total_interest_accrued,
            accounts_capitalized: model.accounts_capitalized,
            total_capitalized_amount: model.total_capitalized_amount,
            status: Self::db_to_processing_job_status(model.status),
            started_at: model.started_at,
            completed_at: model.completed_at,
            errors_01: model.errors_01,
            errors_02: model.errors_02,
            errors_03: model.errors_03,
            errors_04: model.errors_04,
            errors_05: model.errors_05,
        }
    }

    /// Map from domain OverdraftLimitAdjustment to database model
    pub fn limit_adjustment_to_model(adjustment: OverdraftLimitAdjustment) -> DbOverdraftLimitAdjustment {
        DbOverdraftLimitAdjustment {
            id: adjustment.id,
            account_id: adjustment.account_id,
            current_limit: adjustment.current_limit,
            requested_limit: adjustment.requested_limit,
            adjustment_reason_id: adjustment.adjustment_reason_id,
            additional_details: adjustment.additional_details,
            required_document01_id: adjustment.required_document01_id,
            required_document02_id: adjustment.required_document02_id,
            required_document03_id: adjustment.required_document03_id,
            required_document04_id: adjustment.required_document04_id,
            required_document05_id: adjustment.required_document05_id,
            required_document06_id: adjustment.required_document06_id,
            required_document07_id: adjustment.required_document07_id,
            requested_by_person_id: adjustment.requested_by_person_id,
            requested_at: adjustment.requested_at,
            approval_status: Self::casa_approval_status_to_db(adjustment.approval_status),
            approved_by_person_id: adjustment.approved_by_person_id,
            approved_at: adjustment.approved_at,
            approval_notes: adjustment.approval_notes,
            effective_date: adjustment.effective_date,
        }
    }

    /// Map from database OverdraftLimitAdjustment to domain model
    pub fn limit_adjustment_from_model(model: DbOverdraftLimitAdjustment) -> OverdraftLimitAdjustment {
        OverdraftLimitAdjustment {
            id: model.id,
            account_id: model.account_id,
            current_limit: model.current_limit,
            requested_limit: model.requested_limit,
            adjustment_reason_id: model.adjustment_reason_id,
            additional_details: model.additional_details,
            required_document01_id: model.required_document01_id,
            required_document02_id: model.required_document02_id,
            required_document03_id: model.required_document03_id,
            required_document04_id: model.required_document04_id,
            required_document05_id: model.required_document05_id,
            required_document06_id: model.required_document06_id,
            required_document07_id: model.required_document07_id,
            requested_by_person_id: model.requested_by_person_id,
            requested_at: model.requested_at,
            approval_status: Self::db_to_casa_approval_status(model.approval_status),
            approved_by_person_id: model.approved_by_person_id,
            approved_at: model.approved_at,
            approval_notes: model.approval_notes,
            effective_date: model.effective_date,
        }
    }

    /// Map from domain InterestPostingRecord to database model
    pub fn interest_posting_to_model(posting: InterestPostingRecord) -> DbInterestPostingRecord {
        DbInterestPostingRecord {
            id: posting.id,
            account_id: posting.account_id,
            posting_date: posting.posting_date,
            interest_type: Self::interest_type_to_db(posting.interest_type),
            period_start: posting.period_start,
            period_end: posting.period_end,
            principal_amount: posting.principal_amount,
            interest_rate: posting.interest_rate,
            days_calculated: posting.days_calculated,
            interest_amount: posting.interest_amount,
            tax_withheld: posting.tax_withheld,
            net_amount: posting.net_amount,
            posting_status: Self::posting_status_to_db(posting.posting_status),
            posted_by_person_id: posting.posted_by_person_id,
            posted_at: posting.posted_at,
        }
    }

    /// Map from database InterestPostingRecord to domain model
    pub fn interest_posting_from_model(model: DbInterestPostingRecord) -> InterestPostingRecord {
        InterestPostingRecord {
            id: model.id,
            account_id: model.account_id,
            posting_date: model.posting_date,
            interest_type: Self::db_to_interest_type(model.interest_type),
            period_start: model.period_start,
            period_end: model.period_end,
            principal_amount: model.principal_amount,
            interest_rate: model.interest_rate,
            days_calculated: model.days_calculated,
            interest_amount: model.interest_amount,
            tax_withheld: model.tax_withheld,
            net_amount: model.net_amount,
            posting_status: Self::db_to_posting_status(model.posting_status),
            posted_by_person_id: model.posted_by_person_id,
            posted_at: model.posted_at,
        }
    }

    /// Map from domain CasaAccountSummary to database model
    pub fn casa_account_summary_to_model(summary: CasaAccountSummary) -> DbCasaAccountSummary {
        DbCasaAccountSummary {
            account_id: summary.account_id,
            account_type: Self::account_type_to_db(summary.account_type),
            current_balance: summary.current_balance,
            available_balance: summary.available_balance,
            overdraft_facility: summary.overdraft_facility.map(Self::overdraft_facility_to_model),
            is_overdrawn: summary.is_overdrawn,
            overdrawn_amount: summary.overdrawn_amount,
            days_overdrawn: summary.days_overdrawn,
            credit_interest_accrued: summary.credit_interest_accrued,
            debit_interest_accrued: summary.debit_interest_accrued,
            last_interest_posting_date: summary.last_interest_posting_date,
            next_interest_posting_date: summary.next_interest_posting_date,
            last_transaction_date: summary.last_transaction_date,
            mtd_transaction_count: summary.mtd_transaction_count,
            mtd_debit_amount: summary.mtd_debit_amount,
            mtd_credit_amount: summary.mtd_credit_amount,
            dormancy_risk: Self::dormancy_risk_to_db(summary.dormancy_risk),
            compliance_status: Self::casa_compliance_status_to_model(summary.compliance_status),
        }
    }

    /// Map from database CasaAccountSummary to domain model
    pub fn casa_account_summary_from_model(model: DbCasaAccountSummary) -> banking_api::BankingResult<CasaAccountSummary> {
        Ok(CasaAccountSummary {
            account_id: model.account_id,
            account_type: Self::db_to_account_type(model.account_type),
            current_balance: model.current_balance,
            available_balance: model.available_balance,
            overdraft_facility: match model.overdraft_facility {
                Some(facility) => Some(Self::overdraft_facility_from_model(facility)?),
                None => None,
            },
            is_overdrawn: model.is_overdrawn,
            overdrawn_amount: model.overdrawn_amount,
            days_overdrawn: model.days_overdrawn,
            credit_interest_accrued: model.credit_interest_accrued,
            debit_interest_accrued: model.debit_interest_accrued,
            last_interest_posting_date: model.last_interest_posting_date,
            next_interest_posting_date: model.next_interest_posting_date,
            last_transaction_date: model.last_transaction_date,
            mtd_transaction_count: model.mtd_transaction_count,
            mtd_debit_amount: model.mtd_debit_amount,
            mtd_credit_amount: model.mtd_credit_amount,
            dormancy_risk: Self::db_to_dormancy_risk(model.dormancy_risk),
            compliance_status: Self::casa_compliance_status_from_model(model.compliance_status),
        })
    }

    // Enum conversion helper methods
    fn overdraft_status_to_db(status: OverdraftStatus) -> DbOverdraftStatus {
        match status {
            OverdraftStatus::Active => DbOverdraftStatus::Active,
            OverdraftStatus::Suspended => DbOverdraftStatus::Suspended,
            OverdraftStatus::Expired => DbOverdraftStatus::Expired,
            OverdraftStatus::UnderReview => DbOverdraftStatus::UnderReview,
            OverdraftStatus::Cancelled => DbOverdraftStatus::Cancelled,
        }
    }

    fn db_to_overdraft_status(status: DbOverdraftStatus) -> OverdraftStatus {
        match status {
            DbOverdraftStatus::Active => OverdraftStatus::Active,
            DbOverdraftStatus::Suspended => OverdraftStatus::Suspended,
            DbOverdraftStatus::Expired => OverdraftStatus::Expired,
            DbOverdraftStatus::UnderReview => OverdraftStatus::UnderReview,
            DbOverdraftStatus::Cancelled => OverdraftStatus::Cancelled,
        }
    }

    fn review_frequency_to_db(freq: ReviewFrequency) -> DbReviewFrequency {
        match freq {
            ReviewFrequency::Monthly => DbReviewFrequency::Monthly,
            ReviewFrequency::Quarterly => DbReviewFrequency::Quarterly,
            ReviewFrequency::SemiAnnually => DbReviewFrequency::SemiAnnually,
            ReviewFrequency::Annually => DbReviewFrequency::Annually,
        }
    }

    fn db_to_review_frequency(freq: DbReviewFrequency) -> ReviewFrequency {
        match freq {
            DbReviewFrequency::Monthly => ReviewFrequency::Monthly,
            DbReviewFrequency::Quarterly => ReviewFrequency::Quarterly,
            DbReviewFrequency::SemiAnnually => ReviewFrequency::SemiAnnually,
            DbReviewFrequency::Annually => ReviewFrequency::Annually,
        }
    }

    fn compounding_frequency_to_db(freq: CompoundingFrequency) -> DbCompoundingFrequency {
        match freq {
            CompoundingFrequency::Daily => DbCompoundingFrequency::Daily,
            CompoundingFrequency::Weekly => DbCompoundingFrequency::Weekly,
            CompoundingFrequency::Monthly => DbCompoundingFrequency::Monthly,
            CompoundingFrequency::Quarterly => DbCompoundingFrequency::Quarterly,
        }
    }

    fn db_to_compounding_frequency(freq: DbCompoundingFrequency) -> CompoundingFrequency {
        match freq {
            DbCompoundingFrequency::Daily => CompoundingFrequency::Daily,
            DbCompoundingFrequency::Weekly => CompoundingFrequency::Weekly,
            DbCompoundingFrequency::Monthly => CompoundingFrequency::Monthly,
            DbCompoundingFrequency::Quarterly => CompoundingFrequency::Quarterly,
        }
    }

    fn processing_job_status_to_db(status: ProcessingJobStatus) -> DbProcessingJobStatus {
        match status {
            ProcessingJobStatus::Scheduled => DbProcessingJobStatus::Scheduled,
            ProcessingJobStatus::Running => DbProcessingJobStatus::Running,
            ProcessingJobStatus::Completed => DbProcessingJobStatus::Completed,
            ProcessingJobStatus::Failed => DbProcessingJobStatus::Failed,
            ProcessingJobStatus::PartiallyCompleted => DbProcessingJobStatus::PartiallyCompleted,
        }
    }

    fn db_to_processing_job_status(status: DbProcessingJobStatus) -> ProcessingJobStatus {
        match status {
            DbProcessingJobStatus::Scheduled => ProcessingJobStatus::Scheduled,
            DbProcessingJobStatus::Running => ProcessingJobStatus::Running,
            DbProcessingJobStatus::Completed => ProcessingJobStatus::Completed,
            DbProcessingJobStatus::Failed => ProcessingJobStatus::Failed,
            DbProcessingJobStatus::PartiallyCompleted => ProcessingJobStatus::PartiallyCompleted,
        }
    }

    fn casa_approval_status_to_db(status: CasaApprovalStatus) -> DbCasaApprovalStatus {
        match status {
            CasaApprovalStatus::Pending => DbCasaApprovalStatus::Pending,
            CasaApprovalStatus::Approved => DbCasaApprovalStatus::Approved,
            CasaApprovalStatus::Rejected => DbCasaApprovalStatus::Rejected,
            CasaApprovalStatus::RequiresAdditionalDocuments => DbCasaApprovalStatus::RequiresAdditionalDocuments,
            CasaApprovalStatus::UnderReview => DbCasaApprovalStatus::UnderReview,
        }
    }

    fn db_to_casa_approval_status(status: DbCasaApprovalStatus) -> CasaApprovalStatus {
        match status {
            DbCasaApprovalStatus::Pending => CasaApprovalStatus::Pending,
            DbCasaApprovalStatus::Approved => CasaApprovalStatus::Approved,
            DbCasaApprovalStatus::Rejected => CasaApprovalStatus::Rejected,
            DbCasaApprovalStatus::RequiresAdditionalDocuments => CasaApprovalStatus::RequiresAdditionalDocuments,
            DbCasaApprovalStatus::UnderReview => CasaApprovalStatus::UnderReview,
        }
    }

    fn interest_type_to_db(interest_type: InterestType) -> DbInterestType {
        match interest_type {
            InterestType::CreditInterest => DbInterestType::CreditInterest,
            InterestType::DebitInterest => DbInterestType::DebitInterest,
            InterestType::PenaltyInterest => DbInterestType::PenaltyInterest,
        }
    }

    fn db_to_interest_type(interest_type: DbInterestType) -> InterestType {
        match interest_type {
            DbInterestType::CreditInterest => InterestType::CreditInterest,
            DbInterestType::DebitInterest => InterestType::DebitInterest,
            DbInterestType::PenaltyInterest => InterestType::PenaltyInterest,
        }
    }

    fn posting_status_to_db(status: PostingStatus) -> DbPostingStatus {
        match status {
            PostingStatus::Calculated => DbPostingStatus::Calculated,
            PostingStatus::Posted => DbPostingStatus::Posted,
            PostingStatus::Reversed => DbPostingStatus::Reversed,
            PostingStatus::Adjusted => DbPostingStatus::Adjusted,
        }
    }

    fn db_to_posting_status(status: DbPostingStatus) -> PostingStatus {
        match status {
            DbPostingStatus::Calculated => PostingStatus::Calculated,
            DbPostingStatus::Posted => PostingStatus::Posted,
            DbPostingStatus::Reversed => PostingStatus::Reversed,
            DbPostingStatus::Adjusted => PostingStatus::Adjusted,
        }
    }

    fn account_type_to_db(account_type: AccountType) -> AccountType {
        // AccountType is used directly from banking_api, no conversion needed
        account_type
    }

    fn db_to_account_type(account_type: AccountType) -> AccountType {
        // AccountType is used directly from banking_api, no conversion needed
        account_type
    }

    fn dormancy_risk_to_db(risk: DormancyRisk) -> DbDormancyRisk {
        match risk {
            DormancyRisk::Active => DbDormancyRisk::Active,
            DormancyRisk::AtRisk => DbDormancyRisk::AtRisk,
            DormancyRisk::Dormant => DbDormancyRisk::Dormant,
            DormancyRisk::Inactive => DbDormancyRisk::Inactive,
        }
    }

    fn db_to_dormancy_risk(risk: DbDormancyRisk) -> DormancyRisk {
        match risk {
            DbDormancyRisk::Active => DormancyRisk::Active,
            DbDormancyRisk::AtRisk => DormancyRisk::AtRisk,
            DbDormancyRisk::Dormant => DormancyRisk::Dormant,
            DbDormancyRisk::Inactive => DormancyRisk::Inactive,
        }
    }

    fn casa_compliance_status_to_model(status: CasaComplianceStatus) -> DbCasaComplianceStatus {
        DbCasaComplianceStatus {
            kyc_status: Self::kyc_status_to_db(status.kyc_status),
            last_kyc_update: status.last_kyc_update,
            aml_risk_rating: Self::risk_rating_to_db(status.aml_risk_rating),
            regulatory_alerts_01: status.regulatory_alerts_01,
            regulatory_alerts_02: status.regulatory_alerts_02,
            regulatory_alerts_03: status.regulatory_alerts_03,
            regulatory_alerts_04: status.regulatory_alerts_04,
            regulatory_alerts_05: status.regulatory_alerts_05,
        }
    }

    fn casa_compliance_status_from_model(status: DbCasaComplianceStatus) -> CasaComplianceStatus {
        CasaComplianceStatus {
            kyc_status: Self::kyc_status_from_db(status.kyc_status),
            last_kyc_update: status.last_kyc_update,
            aml_risk_rating: Self::risk_rating_from_db(status.aml_risk_rating),
            regulatory_alerts_01: status.regulatory_alerts_01,
            regulatory_alerts_02: status.regulatory_alerts_02,
            regulatory_alerts_03: status.regulatory_alerts_03,
            regulatory_alerts_04: status.regulatory_alerts_04,
            regulatory_alerts_05: status.regulatory_alerts_05,
        }
    }

    // Enum conversion helper methods
    fn kyc_status_to_db(status: KycStatus) -> DbKycStatus {
        match status {
            KycStatus::NotStarted => DbKycStatus::NotStarted,
            KycStatus::InProgress => DbKycStatus::InProgress,
            KycStatus::Pending => DbKycStatus::Pending,
            KycStatus::Complete => DbKycStatus::Complete,
            KycStatus::Approved => DbKycStatus::Approved,
            KycStatus::Rejected => DbKycStatus::Rejected,
            KycStatus::RequiresUpdate => DbKycStatus::RequiresUpdate,
            KycStatus::Failed => DbKycStatus::Failed,
        }
    }

    fn kyc_status_from_db(status: DbKycStatus) -> KycStatus {
        match status {
            DbKycStatus::NotStarted => KycStatus::NotStarted,
            DbKycStatus::InProgress => KycStatus::InProgress,
            DbKycStatus::Pending => KycStatus::Pending,
            DbKycStatus::Complete => KycStatus::Complete,
            DbKycStatus::Approved => KycStatus::Approved,
            DbKycStatus::Rejected => KycStatus::Rejected,
            DbKycStatus::RequiresUpdate => KycStatus::RequiresUpdate,
            DbKycStatus::Failed => KycStatus::Failed,
        }
    }

    fn risk_rating_to_db(rating: RiskRating) -> DbRiskRating {
        match rating {
            RiskRating::Low => DbRiskRating::Low,
            RiskRating::Medium => DbRiskRating::Medium,
            RiskRating::High => DbRiskRating::High,
            RiskRating::Blacklisted => DbRiskRating::Blacklisted,
        }
    }

    fn risk_rating_from_db(rating: DbRiskRating) -> RiskRating {
        match rating {
            DbRiskRating::Low => RiskRating::Low,
            DbRiskRating::Medium => RiskRating::Medium,
            DbRiskRating::High => RiskRating::High,
            DbRiskRating::Blacklisted => RiskRating::Blacklisted,
        }
    }
}
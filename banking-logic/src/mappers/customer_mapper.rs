use banking_api::domain::{
    Customer, CustomerAudit, CustomerComplianceStatus, CustomerDocument, CustomerPortfolio,
    CustomerStatus, CustomerType, DocumentStatus, IdentityType, KycStatus, RiskRating,
    RiskSummary,
};
use banking_db::models::{
    CustomerAuditModel, CustomerComplianceStatusModel, CustomerDocumentModel, CustomerModel,
    CustomerPortfolioModel, CustomerStatus as DbCustomerStatus, CustomerType as DbCustomerType,
    DocumentStatus as DbDocumentStatus, IdentityType as DbIdentityType, KycStatus as DbKycStatus,
    RiskRating as DbRiskRating, RiskSummaryModel,
};

pub struct CustomerMapper;

impl CustomerMapper {
    /// Map from domain Customer to database CustomerModel
    pub fn to_model(customer: Customer) -> CustomerModel {
        CustomerModel {
            id: customer.id,
            customer_type: Self::customer_type_to_db(customer.customer_type),
            full_name: customer.full_name,
            id_type: Self::identity_type_to_db(customer.id_type),
            id_number: customer.id_number,
            risk_rating: Self::risk_rating_to_db(customer.risk_rating),
            status: Self::customer_status_to_db(customer.status),
            created_at: customer.created_at,
            last_updated_at: customer.last_updated_at,
            updated_by_person_id: customer.updated_by_person_id,
        }
    }

    /// Map from database CustomerModel to domain Customer
    pub fn from_model(model: CustomerModel) -> banking_api::BankingResult<Customer> {
        Ok(Customer {
            id: model.id,
            customer_type: Self::customer_type_from_db(model.customer_type),
            full_name: model.full_name,
            id_type: Self::identity_type_from_db(model.id_type),
            id_number: model.id_number,
            risk_rating: Self::risk_rating_from_db(model.risk_rating),
            status: Self::customer_status_from_db(model.status),
            created_at: model.created_at,
            last_updated_at: model.last_updated_at,
            updated_by_person_id: model.updated_by_person_id,
        })
    }

    /// Map from database CustomerPortfolioModel to domain CustomerPortfolio
    pub fn portfolio_from_model(model: CustomerPortfolioModel) -> CustomerPortfolio {
        CustomerPortfolio {
            customer_id: model.customer_id,
            total_accounts: model.total_accounts,
            total_balance: model.total_balance,
            total_loan_outstanding: model.total_loan_outstanding,
            last_activity_date: model.last_activity_date,
            risk_score: model.risk_score,
            kyc_status: Self::kyc_status_from_db(model.kyc_status),
            sanctions_checked: model.sanctions_checked,
            last_screening_date: model.last_screening_date,
        }
    }

    // Helper methods for enum conversions between API domain and DB models
    pub fn customer_type_to_db(customer_type: CustomerType) -> DbCustomerType {
        match customer_type {
            CustomerType::Individual => DbCustomerType::Individual,
            CustomerType::Corporate => DbCustomerType::Corporate,
        }
    }

    pub fn customer_type_from_db(db_type: DbCustomerType) -> CustomerType {
        match db_type {
            DbCustomerType::Individual => CustomerType::Individual,
            DbCustomerType::Corporate => CustomerType::Corporate,
        }
    }

    pub fn identity_type_to_db(identity_type: IdentityType) -> DbIdentityType {
        match identity_type {
            IdentityType::NationalId => DbIdentityType::NationalId,
            IdentityType::Passport => DbIdentityType::Passport,
            IdentityType::CompanyRegistration => DbIdentityType::CompanyRegistration,
            IdentityType::PermanentResidentCard => DbIdentityType::PermanentResidentCard,
            IdentityType::AsylumCard => DbIdentityType::AsylumCard,  
            IdentityType::TemporaryResidentPermit => DbIdentityType::TemporaryResidentPermit,
            IdentityType::Unknown => DbIdentityType::Unknown,
        }
    }

    pub fn identity_type_from_db(db_type: DbIdentityType) -> IdentityType {
        match db_type {
            DbIdentityType::NationalId => IdentityType::NationalId,
            DbIdentityType::Passport => IdentityType::Passport,
            DbIdentityType::CompanyRegistration => IdentityType::CompanyRegistration,
            DbIdentityType::PermanentResidentCard => IdentityType::PermanentResidentCard,
            DbIdentityType::AsylumCard => IdentityType::AsylumCard,
            DbIdentityType::TemporaryResidentPermit => IdentityType::TemporaryResidentPermit,
            DbIdentityType::Unknown => IdentityType::Unknown,
        }
    }

    pub fn risk_rating_to_db(risk_rating: RiskRating) -> DbRiskRating {
        match risk_rating {
            RiskRating::Low => DbRiskRating::Low,
            RiskRating::Medium => DbRiskRating::Medium,
            RiskRating::High => DbRiskRating::High,
            RiskRating::Blacklisted => DbRiskRating::Blacklisted,
        }
    }

    pub fn risk_rating_from_db(db_rating: DbRiskRating) -> RiskRating {
        match db_rating {
            DbRiskRating::Low => RiskRating::Low,
            DbRiskRating::Medium => RiskRating::Medium,
            DbRiskRating::High => RiskRating::High,
            DbRiskRating::Blacklisted => RiskRating::Blacklisted,
        }
    }

    pub fn customer_status_to_db(status: CustomerStatus) -> DbCustomerStatus {
        match status {
            CustomerStatus::Active => DbCustomerStatus::Active,
            CustomerStatus::PendingVerification => DbCustomerStatus::PendingVerification,
            CustomerStatus::Deceased => DbCustomerStatus::Deceased,
            CustomerStatus::Dissolved => DbCustomerStatus::Dissolved,
            CustomerStatus::Blacklisted => DbCustomerStatus::Blacklisted,
        }
    }

    pub fn customer_status_from_db(db_status: DbCustomerStatus) -> CustomerStatus {
        match db_status {
            DbCustomerStatus::Active => CustomerStatus::Active,
            DbCustomerStatus::PendingVerification => CustomerStatus::PendingVerification,
            DbCustomerStatus::Deceased => CustomerStatus::Deceased,
            DbCustomerStatus::Dissolved => CustomerStatus::Dissolved,
            DbCustomerStatus::Blacklisted => CustomerStatus::Blacklisted,
        }
    }

    pub fn kyc_status_from_db(db_status: DbKycStatus) -> KycStatus {
        match db_status {
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

    pub fn kyc_status_to_db(status: KycStatus) -> DbKycStatus {
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

    /// Map from database RiskSummaryModel to domain RiskSummary
    pub fn risk_summary_from_model(model: RiskSummaryModel) -> RiskSummary {
        RiskSummary {
            current_rating: Self::risk_rating_from_db(model.current_rating),
            last_assessment_date: model.last_assessment_date,
            flags_01: model.flags_01,
            flags_02: model.flags_02,
            flags_03: model.flags_03,
            flags_04: model.flags_04,
            flags_05: model.flags_05,
        }
    }

    /// Map from domain RiskSummary to database RiskSummaryModel
    pub fn risk_summary_to_model(summary: RiskSummary) -> RiskSummaryModel {
        RiskSummaryModel {
            current_rating: Self::risk_rating_to_db(summary.current_rating),
            last_assessment_date: summary.last_assessment_date,
            flags_01: summary.flags_01,
            flags_02: summary.flags_02,
            flags_03: summary.flags_03,
            flags_04: summary.flags_04,
            flags_05: summary.flags_05,
        }
    }

    /// Map from database CustomerComplianceStatusModel to domain CustomerComplianceStatus
    pub fn compliance_status_from_model(model: CustomerComplianceStatusModel) -> CustomerComplianceStatus {
        CustomerComplianceStatus {
            kyc_status: Self::kyc_status_from_db(model.kyc_status),
            sanctions_checked: model.sanctions_checked,
            last_screening_date: model.last_screening_date,
        }
    }

    /// Map from domain CustomerComplianceStatus to database CustomerComplianceStatusModel
    pub fn compliance_status_to_model(status: CustomerComplianceStatus) -> CustomerComplianceStatusModel {
        CustomerComplianceStatusModel {
            kyc_status: Self::kyc_status_to_db(status.kyc_status),
            sanctions_checked: status.sanctions_checked,
            last_screening_date: status.last_screening_date,
        }
    }

    pub fn document_status_to_db(status: DocumentStatus) -> DbDocumentStatus {
        match status {
            DocumentStatus::Uploaded => DbDocumentStatus::Uploaded,
            DocumentStatus::Verified => DbDocumentStatus::Verified,
            DocumentStatus::Rejected => DbDocumentStatus::Rejected,
            DocumentStatus::Expired => DbDocumentStatus::Expired,
        }
    }

    pub fn document_status_from_db(db_status: DbDocumentStatus) -> DocumentStatus {
        match db_status {
            DbDocumentStatus::Uploaded => DocumentStatus::Uploaded,
            DbDocumentStatus::Verified => DocumentStatus::Verified,
            DbDocumentStatus::Rejected => DocumentStatus::Rejected,
            DbDocumentStatus::Expired => DocumentStatus::Expired,
        }
    }

    pub fn document_to_model(document: CustomerDocument) -> CustomerDocumentModel {
        CustomerDocumentModel {
            id: document.id,
            customer_id: document.customer_id,
            document_type: document.document_type,
            document_path: document.document_path,
            status: Self::document_status_to_db(document.status),
            uploaded_at: document.uploaded_at,
            uploaded_by: document.uploaded_by,
            verified_at: document.verified_at,
            verified_by: document.verified_by,
        }
    }

    pub fn document_from_model(model: CustomerDocumentModel) -> CustomerDocument {
        CustomerDocument {
            id: model.id,
            customer_id: model.customer_id,
            document_type: model.document_type,
            document_path: model.document_path,
            status: Self::document_status_from_db(model.status),
            uploaded_at: model.uploaded_at,
            uploaded_by: model.uploaded_by,
            verified_at: model.verified_at,
            verified_by: model.verified_by,
        }
    }

    pub fn audit_to_model(audit: CustomerAudit) -> CustomerAuditModel {
        CustomerAuditModel {
            id: audit.id,
            customer_id: audit.customer_id,
            field_name: audit.field_name,
            old_value: audit.old_value,
            new_value: audit.new_value,
            changed_at: audit.changed_at,
            changed_by: audit.changed_by,
            reason: audit.reason,
        }
    }

    pub fn audit_from_model(model: CustomerAuditModel) -> CustomerAudit {
        CustomerAudit {
            id: model.id,
            customer_id: model.customer_id,
            field_name: model.field_name,
            old_value: model.old_value,
            new_value: model.new_value,
            changed_at: model.changed_at,
            changed_by: model.changed_by,
            reason: model.reason,
        }
    }
}
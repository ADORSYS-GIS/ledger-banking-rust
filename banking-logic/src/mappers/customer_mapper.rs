use banking_api::domain::{Customer, CustomerType, IdentityType, RiskRating, CustomerStatus, CustomerPortfolio, KycStatus, RiskSummary, CustomerComplianceStatus};
use banking_db::models::{
    CustomerModel, CustomerPortfolioModel, RiskSummaryModel, CustomerComplianceStatusModel,
    CustomerType as DbCustomerType, IdentityType as DbIdentityType, 
    RiskRating as DbRiskRating, CustomerStatus as DbCustomerStatus,
    KycStatus as DbKycStatus
};

pub struct CustomerMapper;

impl CustomerMapper {
    /// Map from domain Customer to database CustomerModel
    pub fn to_model(customer: Customer) -> CustomerModel {
        CustomerModel {
            customer_id: customer.customer_id,
            customer_type: Self::customer_type_to_db(customer.customer_type),
            full_name: customer.full_name,
            id_type: Self::identity_type_to_db(customer.id_type),
            id_number: customer.id_number,
            risk_rating: Self::risk_rating_to_db(customer.risk_rating),
            status: Self::customer_status_to_db(customer.status),
            created_at: customer.created_at,
            last_updated_at: customer.last_updated_at,
            updated_by: customer.updated_by,
        }
    }

    /// Map from database CustomerModel to domain Customer
    pub fn from_model(model: CustomerModel) -> banking_api::BankingResult<Customer> {
        Ok(Customer {
            customer_id: model.customer_id,
            customer_type: Self::db_to_customer_type(model.customer_type),
            full_name: model.full_name,
            id_type: Self::db_to_identity_type(model.id_type),
            id_number: model.id_number,
            risk_rating: Self::db_to_risk_rating(model.risk_rating),
            status: Self::db_to_customer_status(model.status),
            created_at: model.created_at,
            last_updated_at: model.last_updated_at,
            updated_by: model.updated_by,
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
            kyc_status: Self::db_to_kyc_status(model.kyc_status),
            sanctions_checked: model.sanctions_checked,
            last_screening_date: model.last_screening_date,
        }
    }

    // Helper methods for enum conversions between API domain and DB models
    fn customer_type_to_db(customer_type: CustomerType) -> DbCustomerType {
        match customer_type {
            CustomerType::Individual => DbCustomerType::Individual,
            CustomerType::Corporate => DbCustomerType::Corporate,
        }
    }

    fn db_to_customer_type(db_type: DbCustomerType) -> CustomerType {
        match db_type {
            DbCustomerType::Individual => CustomerType::Individual,
            DbCustomerType::Corporate => CustomerType::Corporate,
        }
    }

    fn identity_type_to_db(identity_type: IdentityType) -> DbIdentityType {
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

    fn db_to_identity_type(db_type: DbIdentityType) -> IdentityType {
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

    fn risk_rating_to_db(risk_rating: RiskRating) -> DbRiskRating {
        match risk_rating {
            RiskRating::Low => DbRiskRating::Low,
            RiskRating::Medium => DbRiskRating::Medium,
            RiskRating::High => DbRiskRating::High,
            RiskRating::Blacklisted => DbRiskRating::Blacklisted,
        }
    }

    fn db_to_risk_rating(db_rating: DbRiskRating) -> RiskRating {
        match db_rating {
            DbRiskRating::Low => RiskRating::Low,
            DbRiskRating::Medium => RiskRating::Medium,
            DbRiskRating::High => RiskRating::High,
            DbRiskRating::Blacklisted => RiskRating::Blacklisted,
        }
    }

    fn customer_status_to_db(status: CustomerStatus) -> DbCustomerStatus {
        match status {
            CustomerStatus::Active => DbCustomerStatus::Active,
            CustomerStatus::PendingVerification => DbCustomerStatus::PendingVerification,
            CustomerStatus::Deceased => DbCustomerStatus::Deceased,
            CustomerStatus::Dissolved => DbCustomerStatus::Dissolved,
            CustomerStatus::Blacklisted => DbCustomerStatus::Blacklisted,
        }
    }

    fn db_to_customer_status(db_status: DbCustomerStatus) -> CustomerStatus {
        match db_status {
            DbCustomerStatus::Active => CustomerStatus::Active,
            DbCustomerStatus::PendingVerification => CustomerStatus::PendingVerification,
            DbCustomerStatus::Deceased => CustomerStatus::Deceased,
            DbCustomerStatus::Dissolved => CustomerStatus::Dissolved,
            DbCustomerStatus::Blacklisted => CustomerStatus::Blacklisted,
        }
    }

    fn db_to_kyc_status(db_status: DbKycStatus) -> KycStatus {
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

    /// Map from database RiskSummaryModel to domain RiskSummary
    pub fn risk_summary_from_model(model: RiskSummaryModel) -> RiskSummary {
        RiskSummary {
            current_rating: Self::db_to_risk_rating(model.current_rating),
            last_assessment_date: model.last_assessment_date,
            flags: model.flags,
        }
    }

    /// Map from domain RiskSummary to database RiskSummaryModel
    pub fn risk_summary_to_model(summary: RiskSummary) -> RiskSummaryModel {
        RiskSummaryModel {
            current_rating: Self::risk_rating_to_db(summary.current_rating),
            last_assessment_date: summary.last_assessment_date,
            flags: summary.flags,
        }
    }

    /// Map from database CustomerComplianceStatusModel to domain CustomerComplianceStatus
    pub fn compliance_status_from_model(model: CustomerComplianceStatusModel) -> CustomerComplianceStatus {
        CustomerComplianceStatus {
            kyc_status: Self::db_to_kyc_status(model.kyc_status),
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

    // Utility functions for external use (kept for backward compatibility)
    pub fn risk_rating_to_string(risk_rating: RiskRating) -> String {
        match risk_rating {
            RiskRating::Low => "Low".to_string(),
            RiskRating::Medium => "Medium".to_string(),
            RiskRating::High => "High".to_string(),
            RiskRating::Blacklisted => "Blacklisted".to_string(),
        }
    }

    pub fn customer_status_to_string(status: CustomerStatus) -> String {
        match status {
            CustomerStatus::Active => "Active".to_string(),
            CustomerStatus::PendingVerification => "PendingVerification".to_string(),
            CustomerStatus::Deceased => "Deceased".to_string(),
            CustomerStatus::Dissolved => "Dissolved".to_string(),
            CustomerStatus::Blacklisted => "Blacklisted".to_string(),
        }
    }
}
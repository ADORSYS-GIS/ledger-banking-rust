use banking_api::domain::{Customer, CustomerType, IdentityType, RiskRating, CustomerStatus, CustomerPortfolio};
use banking_db::models::{CustomerModel, CustomerPortfolioModel};

pub struct CustomerMapper;

impl CustomerMapper {
    /// Map from domain Customer to database CustomerModel
    pub fn to_model(customer: Customer) -> CustomerModel {
        CustomerModel {
            customer_id: customer.customer_id,
            customer_type: Self::customer_type_to_string(customer.customer_type),
            full_name: customer.full_name,
            id_type: Self::identity_type_to_string(customer.id_type),
            id_number: customer.id_number,
            risk_rating: Self::risk_rating_to_string(customer.risk_rating),
            status: Self::customer_status_to_string(customer.status),
            created_at: customer.created_at,
            last_updated_at: customer.last_updated_at,
            updated_by: customer.updated_by,
        }
    }

    /// Map from database CustomerModel to domain Customer
    pub fn from_model(model: CustomerModel) -> banking_api::BankingResult<Customer> {
        Ok(Customer {
            customer_id: model.customer_id,
            customer_type: Self::string_to_customer_type(&model.customer_type)?,
            full_name: model.full_name,
            id_type: Self::string_to_identity_type(&model.id_type)?,
            id_number: model.id_number,
            risk_rating: Self::string_to_risk_rating(&model.risk_rating)?,
            status: Self::string_to_customer_status(&model.status)?,
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
            total_loan_outstanding: None, // Would calculate from loan accounts if needed
            last_activity_date: model.last_activity_date,
            risk_score: model.risk_score,
            kyc_status: model.kyc_status,
            sanctions_checked: model.sanctions_checked,
            last_screening_date: model.last_screening_date,
        }
    }

    // Helper methods for enum conversions
    fn customer_type_to_string(customer_type: CustomerType) -> String {
        match customer_type {
            CustomerType::Individual => "Individual".to_string(),
            CustomerType::Corporate => "Corporate".to_string(),
        }
    }

    fn string_to_customer_type(s: &str) -> banking_api::BankingResult<CustomerType> {
        match s {
            "Individual" => Ok(CustomerType::Individual),
            "Corporate" => Ok(CustomerType::Corporate),
            _ => Err(banking_api::BankingError::InvalidEnumValue {
                field: "customer_type".to_string(),
                value: s.to_string(),
            }),
        }
    }

    fn identity_type_to_string(identity_type: IdentityType) -> String {
        match identity_type {
            IdentityType::NationalId => "NationalId".to_string(),
            IdentityType::Passport => "Passport".to_string(),
            IdentityType::CompanyRegistration => "CompanyRegistration".to_string(),
        }
    }

    fn string_to_identity_type(s: &str) -> banking_api::BankingResult<IdentityType> {
        match s {
            "NationalId" => Ok(IdentityType::NationalId),
            "Passport" => Ok(IdentityType::Passport),
            "CompanyRegistration" => Ok(IdentityType::CompanyRegistration),
            _ => Err(banking_api::BankingError::InvalidEnumValue {
                field: "id_type".to_string(),
                value: s.to_string(),
            }),
        }
    }

    pub fn risk_rating_to_string(risk_rating: RiskRating) -> String {
        match risk_rating {
            RiskRating::Low => "Low".to_string(),
            RiskRating::Medium => "Medium".to_string(),
            RiskRating::High => "High".to_string(),
            RiskRating::Blacklisted => "Blacklisted".to_string(),
        }
    }

    fn string_to_risk_rating(s: &str) -> banking_api::BankingResult<RiskRating> {
        match s {
            "Low" => Ok(RiskRating::Low),
            "Medium" => Ok(RiskRating::Medium),
            "High" => Ok(RiskRating::High),
            "Blacklisted" => Ok(RiskRating::Blacklisted),
            _ => Err(banking_api::BankingError::InvalidEnumValue {
                field: "risk_rating".to_string(),
                value: s.to_string(),
            }),
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

    fn string_to_customer_status(s: &str) -> banking_api::BankingResult<CustomerStatus> {
        match s {
            "Active" => Ok(CustomerStatus::Active),
            "PendingVerification" => Ok(CustomerStatus::PendingVerification),
            "Deceased" => Ok(CustomerStatus::Deceased),
            "Dissolved" => Ok(CustomerStatus::Dissolved),
            "Blacklisted" => Ok(CustomerStatus::Blacklisted),
            _ => Err(banking_api::BankingError::InvalidEnumValue {
                field: "status".to_string(),
                value: s.to_string(),
            }),
        }
    }
}
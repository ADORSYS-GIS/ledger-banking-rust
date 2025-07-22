use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Customer {
    pub customer_id: Uuid,
    pub customer_type: CustomerType,
    #[validate(length(min = 1, max = 255))]
    pub full_name: String,
    pub id_type: IdentityType,
    #[validate(length(min = 1, max = 50))]
    pub id_number: String,
    pub risk_rating: RiskRating,
    pub status: CustomerStatus,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    #[validate(length(min = 1, max = 100))]
    pub updated_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustomerType { 
    Individual, 
    Corporate 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityType { 
    NationalId, 
    Passport, 
    CompanyRegistration 
}

impl std::fmt::Display for IdentityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentityType::NationalId => write!(f, "NationalId"),
            IdentityType::Passport => write!(f, "Passport"),
            IdentityType::CompanyRegistration => write!(f, "CompanyRegistration"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum RiskRating { 
    Low, 
    Medium, 
    High, 
    Blacklisted
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CustomerStatus { 
    Active, 
    PendingVerification, 
    Deceased,
    Dissolved,
    Blacklisted
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerPortfolio {
    pub customer_id: Uuid,
    pub total_accounts: i64,
    pub total_balance: rust_decimal::Decimal,
    pub total_loan_outstanding: Option<rust_decimal::Decimal>,
    pub last_activity_date: Option<DateTime<Utc>>,
    pub risk_score: Option<rust_decimal::Decimal>,
    pub kyc_status: String,
    pub sanctions_checked: bool,
    pub last_screening_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSummary {
    pub current_rating: RiskRating,
    pub last_assessment_date: DateTime<Utc>,
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerComplianceStatus {
    pub kyc_status: KycStatus,
    pub sanctions_checked: bool,
    pub last_screening_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KycStatus {
    NotStarted,
    InProgress,
    Pending,
    Complete,
    Approved,
    Rejected,
    RequiresUpdate,
    Failed,
}
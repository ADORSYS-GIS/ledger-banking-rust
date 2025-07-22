use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::{Customer, CustomerPortfolio, RiskRating, CustomerStatus},
    error::BankingResult,
};

#[async_trait]
pub trait CustomerService: Send + Sync {
    /// Create a new customer record
    async fn create_customer(&self, customer: Customer) -> BankingResult<Customer>;
    
    /// Update existing customer information
    async fn update_customer(&self, customer: Customer) -> BankingResult<Customer>;
    
    /// Find customer by ID
    async fn find_customer_by_id(&self, customer_id: Uuid) -> BankingResult<Option<Customer>>;
    
    /// Risk rating updates - restricted to Risk & Compliance module only
    async fn update_risk_rating(&self, customer_id: Uuid, risk_rating: RiskRating, authorized_by: String) -> BankingResult<()>;
    
    /// Status changes with cascade effects
    async fn update_customer_status(&self, customer_id: Uuid, status: CustomerStatus, reason: String) -> BankingResult<()>;
    
    /// 360-degree customer view
    async fn get_customer_portfolio(&self, customer_id: Uuid) -> BankingResult<CustomerPortfolio>;

    /// Find customers by identity document
    async fn find_customer_by_identity(&self, id_type: crate::domain::IdentityType, id_number: &str) -> BankingResult<Option<Customer>>;

    /// Validate customer can open new account
    async fn validate_account_eligibility(&self, customer_id: Uuid, product_code: &str) -> BankingResult<bool>;

    /// Get all customers for a given risk rating
    async fn find_customers_by_risk_rating(&self, risk_rating: RiskRating) -> BankingResult<Vec<Customer>>;

    /// Get customers requiring compliance review
    async fn find_customers_requiring_review(&self) -> BankingResult<Vec<Customer>>;
}
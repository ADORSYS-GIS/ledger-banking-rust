use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::{
        Customer, CustomerAudit, CustomerDocument, CustomerPortfolio, CustomerStatus, RiskRating,
    },
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
    async fn update_risk_rating(&self, customer_id: Uuid, risk_rating: RiskRating, authorized_by: Uuid) -> BankingResult<()>;
    
    /// Status changes with cascade effects and reason ID validation
    async fn update_customer_status(&self, customer_id: Uuid, status: CustomerStatus, reason_id: Uuid, additional_details: Option<&str>) -> BankingResult<()>;
    
    /// Legacy method - deprecated, use update_customer_status with reason_id instead
    #[deprecated(note = "Use update_customer_status with reason_id instead")]
    async fn update_customer_status_legacy(&self, customer_id: Uuid, status: CustomerStatus, reason: String) -> BankingResult<()>;
    
    /// 360-degree customer view
    async fn get_customer_portfolio(&self, customer_id: Uuid) -> BankingResult<CustomerPortfolio>;

    /// Find customers by identity document
    async fn find_customer_by_identity(&self, id_type: crate::domain::IdentityType, id_number: &str) -> BankingResult<Option<Customer>>;

    /// Validate customer can open new account
    async fn validate_account_eligibility(&self, customer_id: Uuid, product_id: Uuid) -> BankingResult<bool>;

    /// Get all customers for a given risk rating
    async fn find_customers_by_risk_rating(&self, risk_rating: RiskRating) -> BankingResult<Vec<Customer>>;

    /// Get customers requiring compliance review
    async fn find_customers_requiring_review(&self) -> BankingResult<Vec<Customer>>;

    /// Add a document to a customer's profile
    async fn add_customer_document(
        &self,
        document: CustomerDocument,
    ) -> BankingResult<CustomerDocument>;

    /// Get all documents for a customer
    async fn get_customer_documents(&self, customer_id: Uuid) -> BankingResult<Vec<CustomerDocument>>;

    /// Add an audit trail entry for a customer
    async fn add_customer_audit_entry(&self, audit_entry: CustomerAudit)
        -> BankingResult<CustomerAudit>;

    /// Get the audit trail for a customer
    async fn get_customer_audit_trail(&self, customer_id: Uuid) -> BankingResult<Vec<CustomerAudit>>;
}
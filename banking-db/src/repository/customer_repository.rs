use async_trait::async_trait;
use banking_api::BankingResult;
use uuid::Uuid;

use crate::models::{CustomerModel, CustomerPortfolioModel, CustomerDocumentModel, CustomerAuditModel};

#[async_trait]
pub trait CustomerRepository: Send + Sync {
    /// Create a new customer record
    async fn create(&self, customer: CustomerModel) -> BankingResult<CustomerModel>;
    
    /// Update existing customer record
    async fn update(&self, customer: CustomerModel) -> BankingResult<CustomerModel>;
    
    /// Find customer by ID
    async fn find_by_id(&self, customer_id: Uuid) -> BankingResult<Option<CustomerModel>>;
    
    /// Check if customer exists
    async fn exists(&self, customer_id: Uuid) -> BankingResult<bool>;
    
    /// Find customer by identity document
    async fn find_by_identity(&self, id_type: &str, id_number: &str) -> BankingResult<Option<CustomerModel>>;
    
    /// Find customers by risk rating
    async fn find_by_risk_rating(&self, risk_rating: &str) -> BankingResult<Vec<CustomerModel>>;
    
    /// Find customers requiring compliance review
    async fn find_requiring_review(&self) -> BankingResult<Vec<CustomerModel>>;
    
    /// Get customer portfolio summary
    async fn get_portfolio(&self, customer_id: Uuid) -> BankingResult<Option<CustomerPortfolioModel>>;
    
    /// Update customer risk rating with audit trail
    /// @param authorized_by - References Person.person_id
    async fn update_risk_rating(&self, customer_id: Uuid, risk_rating: &str, authorized_by: Uuid) -> BankingResult<()>;
    
    /// Update customer status with audit trail
    async fn update_status(&self, customer_id: Uuid, status: &str, reason: &str) -> BankingResult<()>;
    
    /// Add customer document
    async fn add_document(&self, document: CustomerDocumentModel) -> BankingResult<CustomerDocumentModel>;
    
    /// Get customer documents
    async fn get_documents(&self, customer_id: Uuid) -> BankingResult<Vec<CustomerDocumentModel>>;
    
    /// Add audit trail entry
    async fn add_audit_entry(&self, audit: CustomerAuditModel) -> BankingResult<CustomerAuditModel>;
    
    /// Get customer audit trail
    async fn get_audit_trail(&self, customer_id: Uuid) -> BankingResult<Vec<CustomerAuditModel>>;
    
    /// Delete customer (soft delete)
    /// @param deleted_by - References Person.person_id
    async fn delete(&self, customer_id: Uuid, deleted_by: Uuid) -> BankingResult<()>;
    
    /// List customers with pagination
    async fn list(&self, offset: i64, limit: i64) -> BankingResult<Vec<CustomerModel>>;
    
    /// Count total customers
    async fn count(&self) -> BankingResult<i64>;
}
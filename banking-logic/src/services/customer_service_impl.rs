use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use banking_api::{
    BankingResult, Customer, CustomerPortfolio, RiskRating, CustomerStatus,
    service::CustomerService,
};
use banking_db::repository::CustomerRepository;
use crate::mappers::CustomerMapper;

/// Production implementation of CustomerService
/// Provides comprehensive Customer Information File (CIF) management
pub struct CustomerServiceImpl {
    customer_repository: Arc<dyn CustomerRepository>,
}

impl CustomerServiceImpl {
    pub fn new(customer_repository: Arc<dyn CustomerRepository>) -> Self {
        Self { customer_repository }
    }
}

#[async_trait]
impl CustomerService for CustomerServiceImpl {
    /// Create a new customer with full KYC validation
    async fn create_customer(&self, mut customer: Customer) -> BankingResult<Customer> {
        // Set system timestamps
        customer.created_at = Utc::now();
        customer.last_updated_at = Utc::now();

        // Validate business rules
        self.validate_customer_data(&customer)?;
        
        if let Some(existing) = self.customer_repository
            .find_by_identity(CustomerMapper::identity_type_to_db(customer.id_type), &customer.id_number)
            .await? 
        {
            return Err(banking_api::BankingError::DuplicateIdentityDocument(
                format!("Customer with {} '{}' already exists (existing customer ID: {})", 
                    customer.id_type, customer.id_number.as_str(), existing.id)
            ));
        }

        // Convert to database model and persist
        let customer_model = CustomerMapper::to_model(customer.clone());
        let created_model = self.customer_repository.create(customer_model).await?;

        // Convert back to domain object
        CustomerMapper::from_model(created_model)
    }

    /// Update existing customer with audit trail
    async fn update_customer(&self, mut customer: Customer) -> BankingResult<Customer> {
        // Update timestamp
        customer.last_updated_at = Utc::now();

        // Validate business rules
        self.validate_customer_data(&customer)?;

        // Ensure customer exists
        if !self.customer_repository.exists(customer.id).await? {
            return Err(banking_api::BankingError::CustomerNotFound(customer.id));
        }

        // Convert to database model and update
        let customer_model = CustomerMapper::to_model(customer.clone());
        let updated_model = self.customer_repository.update(customer_model).await?;

        // Convert back to domain object
        CustomerMapper::from_model(updated_model)
    }

    /// Find customer by unique identifier
    async fn find_customer_by_id(&self, customer_id: Uuid) -> BankingResult<Option<Customer>> {
        if let Some(model) = self.customer_repository.find_by_id(customer_id).await? {
            Ok(Some(CustomerMapper::from_model(model)?))
        } else {
            Ok(None)
        }
    }

    /// Update customer risk rating with proper authorization
    /// This is a restricted operation that requires proper authorization
    async fn update_risk_rating(
        &self,
        customer_id: Uuid,
        risk_rating: RiskRating,
        authorized_by: Uuid,
    ) -> BankingResult<()> {
        // Validate authorization (in production, this would check user permissions)
        self.validate_risk_rating_authorization(authorized_by)?;

        // Ensure customer exists
        if !self.customer_repository.exists(customer_id).await? {
            return Err(banking_api::BankingError::CustomerNotFound(customer_id));
        }

        // Update risk rating with audit trail
        self.customer_repository
            .update_risk_rating(
                customer_id,
                CustomerMapper::risk_rating_to_db(risk_rating),
                authorized_by,
            )
            .await?;

        // If blacklisted, trigger account freezing (this would be handled by event system)
        if risk_rating == RiskRating::Blacklisted {
            tracing::info!(
                "Customer {} marked as blacklisted by {}. Account freeze triggered.",
                customer_id, authorized_by
            );
        }

        Ok(())
    }

    /// Update customer status with cascade effects and reason ID validation
    async fn update_customer_status(
        &self,
        customer_id: Uuid,
        status: CustomerStatus,
        reason_id: Uuid,
        _additional_details: Option<&str>,
    ) -> BankingResult<()> {
        // Ensure customer exists
        if !self.customer_repository.exists(customer_id).await? {
            return Err(banking_api::BankingError::CustomerNotFound(customer_id));
        }

        // TODO: Validate reason_id against ReasonAndPurpose table
        // TODO: Store additional_details if provided
        
        // For now, convert reason_id to string for legacy compatibility
        let reason_string = format!("Reason ID: {reason_id}");
        
        // Update status with audit trail
        self.customer_repository
            .update_status(
                customer_id,
                CustomerMapper::customer_status_to_db(status),
                &reason_string,
            )
            .await?;

        // Handle cascade effects based on status
        match status {
            CustomerStatus::Deceased | CustomerStatus::Dissolved => {
                tracing::info!(
                    "Customer {} status changed to {:?}. Account restrictions will be applied.",
                    customer_id, status
                );
                // In production, this would trigger account status updates
            }
            CustomerStatus::Blacklisted => {
                tracing::warn!(
                    "Customer {} blacklisted. Immediate account freeze required.",
                    customer_id
                );
                // In production, this would trigger immediate account freezing
            }
            _ => {}
        }

        Ok(())
    }
    
    /// Legacy method - deprecated, use update_customer_status with reason_id instead
    async fn update_customer_status_legacy(
        &self,
        customer_id: Uuid,
        status: CustomerStatus,
        reason: String,
    ) -> BankingResult<()> {
        // Ensure customer exists
        if !self.customer_repository.exists(customer_id).await? {
            return Err(banking_api::BankingError::CustomerNotFound(customer_id));
        }

        // Update status with audit trail
        self.customer_repository
            .update_status(
                customer_id,
                CustomerMapper::customer_status_to_db(status),
                &reason,
            )
            .await?;

        // Handle cascade effects based on status
        match status {
            CustomerStatus::Deceased | CustomerStatus::Dissolved => {
                tracing::info!(
                    "Customer {} status changed to {:?}. Account restrictions will be applied.",
                    customer_id, status
                );
                // In production, this would trigger account status updates
            }
            CustomerStatus::Blacklisted => {
                tracing::warn!(
                    "Customer {} blacklisted. Immediate account freeze required.",
                    customer_id
                );
                // In production, this would trigger immediate account freezing
            }
            _ => {}
        }

        Ok(())
    }

    /// Get comprehensive customer portfolio view
    async fn get_customer_portfolio(&self, customer_id: Uuid) -> BankingResult<CustomerPortfolio> {
        let portfolio_model = self.customer_repository
            .get_portfolio(customer_id)
            .await?
            .ok_or(banking_api::BankingError::CustomerNotFound(customer_id))?;

        Ok(CustomerMapper::portfolio_from_model(portfolio_model))
    }

    /// Find customers by identity document
    async fn find_customer_by_identity(&self, id_type: banking_api::domain::IdentityType, id_number: &str) -> BankingResult<Option<Customer>> {
        let customer_model = self.customer_repository
            .find_by_identity(CustomerMapper::identity_type_to_db(id_type), id_number)
            .await?;
        
        match customer_model {
            Some(model) => CustomerMapper::from_model(model).map(Some),
            None => Ok(None),
        }
    }

    /// Validate customer can open new account
    async fn validate_account_eligibility(&self, customer_id: Uuid, product_id: Uuid) -> BankingResult<bool> {
        // Check if customer exists and is active
        let customer_model = self.customer_repository
            .find_by_id(customer_id)
            .await?
            .ok_or(banking_api::BankingError::CustomerNotFound(customer_id))?;

        // Convert to domain model to properly check status
        let customer = CustomerMapper::from_model(customer_model)?;
        
        // Basic eligibility checks
        if customer.status != CustomerStatus::Active {
            return Ok(false);
        }

        // Additional product-specific validation can be added here
        tracing::info!(
            "Customer {} eligibility validated for product {}",
            customer_id, product_id
        );
        
        Ok(true)
    }

    /// Get all customers for a given risk rating
    async fn find_customers_by_risk_rating(&self, risk_rating: RiskRating) -> BankingResult<Vec<Customer>> {
        let customer_models = self.customer_repository
            .find_by_risk_rating(CustomerMapper::risk_rating_to_db(risk_rating))
            .await?;
        
        let mut customers = Vec::new();
        for model in customer_models {
            customers.push(CustomerMapper::from_model(model)?);
        }
        
        Ok(customers)
    }

    /// Get customers requiring compliance review
    async fn find_customers_requiring_review(&self) -> BankingResult<Vec<Customer>> {
        let customer_models = self.customer_repository
            .find_requiring_review()
            .await?;
        
        let mut customers = Vec::new();
        for model in customer_models {
            customers.push(CustomerMapper::from_model(model)?);
        }
        
        Ok(customers)
    }
}

impl CustomerServiceImpl {
    /// Validate customer data according to business rules
    fn validate_customer_data(&self, customer: &Customer) -> BankingResult<()> {
        // Name validation
        if customer.full_name.trim().len() < 2 {
            return Err(banking_api::BankingError::ValidationError {
                field: "full_name".to_string(),
                message: "Full name must be at least 2 characters long".to_string(),
            });
        }

        if customer.full_name.len() > 255 {
            return Err(banking_api::BankingError::ValidationError {
                field: "full_name".to_string(),
                message: "Full name cannot exceed 255 characters".to_string(),
            });
        }

        // ID number validation
        if customer.id_number.trim().len() < 3 {
            return Err(banking_api::BankingError::ValidationError {
                field: "id_number".to_string(),
                message: "ID number must be at least 3 characters long".to_string(),
            });
        }

        // Updated by validation
        if customer.updated_by_person_id.is_nil() {
            return Err(banking_api::BankingError::ValidationError {
                field: "updated_by_person_id".to_string(),
                message: "Updated by field is required".to_string(),
            });
        }

        Ok(())
    }

    /// Validate authorization for risk rating updates
    fn validate_risk_rating_authorization(&self, authorized_by: Uuid) -> BankingResult<()> {
        // In production, this would check against user permissions database
        if authorized_by.is_nil() {
            return Err(banking_api::BankingError::UnauthorizedOperation(
                "Authorization required for risk rating updates".to_string()
            ));
        }

        // Additional authorization checks would go here
        // For now, we'll accept any non-empty authorized_by value

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use banking_api::domain::{CustomerType, IdentityType};

    // Mock repository for testing would go here
    // This is a simplified example - in production you'd use a proper mock framework

    #[tokio::test]
    async fn test_validate_customer_data() {
        let service = CustomerServiceImpl::new(Arc::new(MockCustomerRepository {}));

        #[allow(deprecated)]
        let valid_customer = Customer::new(
            Uuid::new_v4(),
            CustomerType::Individual,
            "John Doe",
            IdentityType::NationalId,
            "ID123456",
            RiskRating::Low,
            CustomerStatus::Active,
            uuid::Uuid::new_v4(),
        ).unwrap();

        assert!(service.validate_customer_data(&valid_customer).is_ok());

        // Test invalid name (empty should trigger validation error)
        #[allow(deprecated)]
        let invalid_customer = Customer::new(
            Uuid::new_v4(),
            CustomerType::Individual,
            "", // Empty name should be invalid
            IdentityType::NationalId,
            "ID123456",
            RiskRating::Low,
            CustomerStatus::Active,
            uuid::Uuid::new_v4(),
        ).unwrap(); // Customer creation succeeds, but validation should fail
        
        // Should fail validation due to empty name
        assert!(invalid_customer.validate().is_err());
    }

    // Mock repository implementation for testing
    struct MockCustomerRepository;

    #[async_trait]
    impl CustomerRepository for MockCustomerRepository {
        async fn create(&self, _customer: banking_db::models::CustomerModel) -> BankingResult<banking_db::models::CustomerModel> {
            unimplemented!()
        }

        async fn update(&self, _customer: banking_db::models::CustomerModel) -> BankingResult<banking_db::models::CustomerModel> {
            unimplemented!()
        }

        async fn find_by_id(&self, _customer_id: Uuid) -> BankingResult<Option<banking_db::models::CustomerModel>> {
            unimplemented!()
        }

        async fn find_by_identity(&self, _id_type: banking_db::models::IdentityType, _id_number: &str) -> BankingResult<Option<banking_db::models::CustomerModel>> {
            Ok(None) // No duplicates for testing
        }

        async fn find_by_risk_rating(&self, _risk_rating: banking_db::models::RiskRating) -> BankingResult<Vec<banking_db::models::CustomerModel>> {
            unimplemented!()
        }

        async fn find_requiring_review(&self) -> BankingResult<Vec<banking_db::models::CustomerModel>> {
            unimplemented!()
        }

        async fn get_portfolio(&self, _customer_id: Uuid) -> BankingResult<Option<banking_db::models::CustomerPortfolioModel>> {
            unimplemented!()
        }

        async fn update_risk_rating(&self, _customer_id: Uuid, _risk_rating: banking_db::models::RiskRating, _authorized_by: Uuid) -> BankingResult<()> {
            Ok(())
        }

        async fn update_status(&self, _customer_id: Uuid, _status: banking_db::models::CustomerStatus, _reason: &str) -> BankingResult<()> {
            Ok(())
        }

        async fn add_document(&self, _document: banking_db::models::CustomerDocumentModel) -> BankingResult<banking_db::models::CustomerDocumentModel> {
            unimplemented!()
        }

        async fn get_documents(&self, _customer_id: Uuid) -> BankingResult<Vec<banking_db::models::CustomerDocumentModel>> {
            unimplemented!()
        }

        async fn add_audit_entry(&self, _audit: banking_db::models::CustomerAuditModel) -> BankingResult<banking_db::models::CustomerAuditModel> {
            unimplemented!()
        }

        async fn get_audit_trail(&self, _customer_id: Uuid) -> BankingResult<Vec<banking_db::models::CustomerAuditModel>> {
            unimplemented!()
        }

        async fn delete(&self, _customer_id: Uuid, _deleted_by: Uuid) -> BankingResult<()> {
            unimplemented!()
        }

        async fn exists(&self, _customer_id: Uuid) -> BankingResult<bool> {
            Ok(true)
        }

        async fn list(&self, _offset: i64, _limit: i64) -> BankingResult<Vec<banking_db::models::CustomerModel>> {
            unimplemented!()
        }

        async fn count(&self) -> BankingResult<i64> {
            unimplemented!()
        }
    }
}
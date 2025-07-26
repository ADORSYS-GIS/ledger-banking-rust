use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

#[cfg(test)]
use heapless::String as HeaplessString;

use banking_api::{
    BankingResult, Account, AccountStatus,
    service::AccountService,
};
use banking_db::repository::AccountRepository;
use crate::{mappers::AccountMapper, integration::ProductCatalogClient};

/// Production implementation of AccountService
/// Provides Unified Account Model (UAM) operations with product integration
pub struct AccountServiceImpl {
    account_repository: Arc<dyn AccountRepository>,
    product_catalog_client: Arc<ProductCatalogClient>,
}

impl AccountServiceImpl {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
        product_catalog_client: Arc<ProductCatalogClient>,
    ) -> Self {
        Self {
            account_repository,
            product_catalog_client,
        }
    }
}

#[async_trait]
impl AccountService for AccountServiceImpl {
    /// Create a new account with product catalog integration
    async fn create_account(&self, mut account: Account) -> BankingResult<Account> {
        // Set system timestamps
        account.created_at = Utc::now();
        account.last_updated_at = Utc::now();

        // Validate account data
        self.validate_account_data(&account).await?;

        // Validate product code with catalog
        self.validate_product_code(account.product_code.as_str()).await?;

        // Apply product-specific defaults
        account = self.apply_product_defaults(account).await?;

        // Convert to database model and persist
        let account_model = AccountMapper::to_model(account.clone());
        let created_model = self.account_repository.create(account_model).await?;

        // Convert back to domain object
        AccountMapper::from_model(created_model)
    }

    /// Find account by unique identifier
    async fn find_account_by_id(&self, account_id: Uuid) -> BankingResult<Option<Account>> {
        if let Some(model) = self.account_repository.find_by_id(account_id).await? {
            Ok(Some(AccountMapper::from_model(model)?))
        } else {
            Ok(None)
        }
    }

    /// Update account status with immediate enforcement
    async fn update_account_status(
        &self,
        account_id: Uuid,
        status: AccountStatus,
        authorized_by: Uuid,
    ) -> BankingResult<()> {
        // Validate authorization for status changes (now using person ID)
        self.validate_status_change_authorization_by_id(&authorized_by, &status)?;

        // Ensure account exists
        if !self.account_repository.exists(account_id).await? {
            return Err(banking_api::BankingError::AccountNotFound(account_id));
        }

        // Update status with audit trail
        self.account_repository
            .update_status(
                account_id,
                &Self::account_status_to_string(status),
                "Status change authorized",
                authorized_by,
            )
            .await?;

        // Handle status-specific side effects
        self.handle_status_change_effects(account_id, status).await?;

        Ok(())
    }

    /// Calculate current account balance with real-time precision
    async fn calculate_balance(&self, account_id: Uuid) -> BankingResult<Decimal> {
        let account = self
            .find_account_by_id(account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(account_id))?;

        // For most account types, current balance is authoritative
        // For loan accounts, we might need to calculate outstanding principal
        match account.account_type {
            banking_api::domain::AccountType::Loan => {
                // Return outstanding principal for loan accounts
                Ok(account.outstanding_principal.unwrap_or(Decimal::ZERO))
            }
            _ => {
                // For deposit accounts, return current balance
                Ok(account.current_balance)
            }
        }
    }

    /// Calculate available balance considering holds and overdraft
    async fn calculate_available_balance(&self, account_id: Uuid) -> BankingResult<Decimal> {
        let account = self
            .find_account_by_id(account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(account_id))?;

        // Get active holds (this would typically come from a holds repository)
        let total_holds = self.get_total_active_holds(account_id).await?;

        // Calculate available balance
        let available = match account.account_type {
            banking_api::domain::AccountType::Current => {
                // Current accounts may have overdraft facilities
                account.current_balance - total_holds + account.overdraft_limit.unwrap_or(Decimal::ZERO)
            }
            banking_api::domain::AccountType::Savings => {
                // Savings accounts cannot go negative
                (account.current_balance - total_holds).max(Decimal::ZERO)
            }
            banking_api::domain::AccountType::Loan => {
                // Loan accounts represent debt, available balance is zero
                Decimal::ZERO
            }
        };

        Ok(available)
    }

    /// Apply hold with reason ID validation
    async fn apply_hold(&self, account_id: Uuid, amount: Decimal, reason_id: Uuid, _additional_details: Option<&str>) -> BankingResult<()> {
        // Validate account exists and is operational
        let account = self
            .find_account_by_id(account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(account_id))?;

        // Validate account can have holds applied
        self.validate_hold_eligibility(&account)?;

        // Validate hold amount
        if amount <= Decimal::ZERO {
            return Err(banking_api::BankingError::ValidationError {
                field: "amount".to_string(),
                message: "Hold amount must be positive".to_string(),
            });
        }

        // Check if applying hold would make available balance negative
        let current_available = self.calculate_available_balance(account_id).await?;
        if current_available < amount {
            return Err(banking_api::BankingError::InsufficientFunds {
                account_id,
                requested: amount,
                available: current_available,
            });
        }

        // TODO: Validate reason_id against ReasonAndPurpose table
        // TODO: Store additional_details if provided
        
        // For now, convert reason_id to string for legacy compatibility
        let reason_string = format!("Reason ID: {reason_id}");
        
        // Apply the hold (this would typically involve a holds repository)
        self.create_account_hold(account_id, amount, &reason_string).await?;

        tracing::info!(
            "Applied hold of {} on account {} for reason ID: {}",
            amount, account_id, reason_id
        );

        Ok(())
    }
    
    /// Legacy method - deprecated, use apply_hold with reason_id instead
    async fn apply_hold_legacy(&self, account_id: Uuid, amount: Decimal, reason: String) -> BankingResult<()> {
        // Validate account exists and is operational
        let account = self
            .find_account_by_id(account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(account_id))?;

        // Validate account can have holds applied
        self.validate_hold_eligibility(&account)?;

        // Validate hold amount
        if amount <= Decimal::ZERO {
            return Err(banking_api::BankingError::ValidationError {
                field: "amount".to_string(),
                message: "Hold amount must be positive".to_string(),
            });
        }

        // Check if applying hold would make available balance negative
        let current_available = self.calculate_available_balance(account_id).await?;
        if current_available < amount {
            return Err(banking_api::BankingError::InsufficientFunds {
                account_id,
                requested: amount,
                available: current_available,
            });
        }

        // Apply the hold (this would typically involve a holds repository)
        self.create_account_hold(account_id, amount, &reason).await?;

        tracing::info!(
            "Applied hold of {} on account {} for reason: {}",
            amount, account_id, reason
        );

        Ok(())
    }

    /// Refresh product rules for the account from catalog
    async fn refresh_product_rules(&self, account_id: Uuid) -> BankingResult<()> {
        let account = self
            .find_account_by_id(account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(account_id))?;

        // Fetch latest product rules
        let _product_rules = self
            .product_catalog_client
            .get_product_rules(account.product_code.as_str())
            .await?;

        // Update account with any product-specific changes
        // This might include dormancy thresholds, fee schedules, etc.
        tracing::info!(
            "Refreshed product rules for account {} with product code {}",
            account_id, account.product_code.as_str()
        );

        // In production, this would update specific fields based on product rules
        // For now, we'll just log the refresh

        Ok(())
    }

    /// Find all accounts for a customer
    async fn find_accounts_by_customer(&self, _customer_id: Uuid) -> BankingResult<Vec<Account>> {
        todo!("Implement find_accounts_by_customer")
    }

    /// Find accounts by status
    async fn find_accounts_by_status(&self, _status: AccountStatus) -> BankingResult<Vec<Account>> {
        todo!("Implement find_accounts_by_status")
    }

    /// Find interest bearing accounts
    async fn find_interest_bearing_accounts(&self) -> BankingResult<Vec<Account>> {
        todo!("Implement find_interest_bearing_accounts")
    }

    /// Update account balance
    async fn update_balance(&self, account_id: Uuid, new_balance: Decimal, updated_by: Uuid) -> BankingResult<()> {
        // In a real implementation, we'd calculate available balance based on holds
        self.account_repository.update_balance(account_id, new_balance, new_balance).await?;
        
        tracing::info!("Account {} balance updated to {} by {}", account_id, new_balance, updated_by);
        Ok(())
    }

    /// Reset accrued interest
    async fn reset_accrued_interest(&self, account_id: Uuid) -> BankingResult<()> {
        self.account_repository.reset_accrued_interest(account_id).await
    }

    /// Update accrued interest
    async fn update_accrued_interest(&self, account_id: Uuid, interest_amount: Decimal) -> BankingResult<()> {
        self.account_repository.update_accrued_interest(account_id, interest_amount).await
    }

    /// Get account status
    async fn get_account_status(&self, _account_id: Uuid) -> BankingResult<AccountStatus> {
        todo!("Implement get_account_status")
    }

    /// Get active holds
    async fn get_active_holds(&self, _account_id: Uuid) -> BankingResult<Vec<banking_api::domain::AccountHold>> {
        todo!("Implement get_active_holds")
    }

    /// Release hold
    async fn release_hold(&self, _hold_id: Uuid, _released_by: Uuid) -> BankingResult<()> {
        todo!("Implement release_hold")
    }

    /// Find dormancy candidates
    async fn find_dormancy_candidates(&self, _inactive_days: i32) -> BankingResult<Vec<Account>> {
        todo!("Implement find_dormancy_candidates")
    }

    /// Update last activity date
    async fn update_last_activity_date(&self, account_id: Uuid, activity_date: chrono::NaiveDate) -> BankingResult<()> {
        self.account_repository.update_last_activity_date(account_id, activity_date).await
    }
}

impl AccountServiceImpl {
    /// Validate account data according to business rules
    async fn validate_account_data(&self, account: &Account) -> BankingResult<()> {
        // Product code validation
        if account.product_code.as_str().trim().is_empty() {
            return Err(banking_api::BankingError::ValidationError {
                field: "product_code".to_string(),
                message: "Product code is required".to_string(),
            });
        }

        // Currency validation
        let currency_str = account.currency.as_str();
        
        // Check length and valid characters
        if currency_str.len() != 3 {
            return Err(banking_api::BankingError::ValidationError {
                field: "currency".to_string(),
                message: "Currency must be exactly 3 characters".to_string(),
            });
        }
        
        // Check for null bytes or invalid characters (not all uppercase letters)
        if currency_str.contains('\0') || !currency_str.chars().all(|c| c.is_ascii_alphabetic() && c.is_uppercase()) {
            return Err(banking_api::BankingError::ValidationError {
                field: "currency".to_string(),
                message: "Currency must be a 3-character ISO code".to_string(),
            });
        }

        // Balance validation
        if account.available_balance > account.current_balance + account.overdraft_limit.unwrap_or(Decimal::ZERO) {
            return Err(banking_api::BankingError::ValidationError {
                field: "available_balance".to_string(),
                message: "Available balance cannot exceed current balance plus overdraft limit".to_string(),
            });
        }

        // Loan-specific validation
        if account.account_type == banking_api::domain::AccountType::Loan {
            if account.original_principal.is_none() || account.outstanding_principal.is_none() {
                return Err(banking_api::BankingError::ValidationError {
                    field: "loan_principals".to_string(),
                    message: "Loan accounts must have original and outstanding principal amounts".to_string(),
                });
            }

            if let (Some(original), Some(outstanding)) = (account.original_principal, account.outstanding_principal) {
                if outstanding > original {
                    return Err(banking_api::BankingError::ValidationError {
                        field: "outstanding_principal".to_string(),
                        message: "Outstanding principal cannot exceed original principal".to_string(),
                    });
                }
            }
        }

        // Overdraft validation
        if let Some(overdraft_limit) = account.overdraft_limit {
            if overdraft_limit < Decimal::ZERO {
                return Err(banking_api::BankingError::ValidationError {
                    field: "overdraft_limit".to_string(),
                    message: "Overdraft limit cannot be negative".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Validate product code exists in catalog
    async fn validate_product_code(&self, product_code: &str) -> BankingResult<()> {
        // Check if product exists in catalog
        match self.product_catalog_client.get_product_rules(product_code).await {
            Ok(_) => Ok(()),
            Err(_) => Err(banking_api::BankingError::InvalidProductCode(product_code.to_string())),
        }
    }

    /// Apply product-specific defaults to new account
    async fn apply_product_defaults(&self, mut account: Account) -> BankingResult<Account> {
        let product_rules = self
            .product_catalog_client
            .get_product_rules(account.product_code.as_str())
            .await?;

        // Apply dormancy threshold if not set
        if account.dormancy_threshold_days.is_none() {
            if let Some(default_days) = product_rules.default_dormancy_days {
                account.dormancy_threshold_days = Some(default_days);
            }
        }

        // Apply default overdraft for current accounts if applicable
        if account.account_type == banking_api::domain::AccountType::Current && account.overdraft_limit.is_none() {
            account.overdraft_limit = product_rules.default_overdraft_limit;
        }

        Ok(account)
    }

    /// Validate authorization for status changes (using person ID)
    fn validate_status_change_authorization_by_id(
        &self,
        authorized_by: &Uuid,
        status: &AccountStatus,
    ) -> BankingResult<()> {
        // TODO: Verify the person ID exists in referenced_persons table
        // For now, just check it's not nil UUID
        if authorized_by.is_nil() {
            return Err(banking_api::BankingError::UnauthorizedOperation(
                "Valid person ID required for status changes".to_string()
            ));
        }

        // Validate based on status type
        match status {
            AccountStatus::Closed => {
                // TODO: Check if person has authority to close accounts
                Ok(())
            }
            AccountStatus::Frozen => {
                // TODO: Check if person has authority to freeze accounts
                Ok(())
            }
            _ => Ok(())
        }
    }

    /// Validate authorization for status changes (legacy string-based)
    #[allow(dead_code)]
    fn validate_status_change_authorization(
        &self,
        authorized_by: &str,
        status: &AccountStatus,
    ) -> BankingResult<()> {
        if authorized_by.trim().is_empty() {
            return Err(banking_api::BankingError::UnauthorizedOperation(
                "Authorization required for status changes".to_string()
            ));
        }

        // Frozen status requires special authorization
        if status == &AccountStatus::Frozen {
            // In production, this would check specific permissions
            tracing::warn!(
                "Account freeze operation requested by {}. Special authorization required.",
                authorized_by
            );
        }

        Ok(())
    }

    /// Handle side effects of status changes
    async fn handle_status_change_effects(&self, account_id: Uuid, status: AccountStatus) -> BankingResult<()> {
        match status {
            AccountStatus::Frozen => {
                tracing::warn!("Account {} frozen. All debits blocked.", account_id);
                // In production, this would trigger immediate transaction blocking
            }
            AccountStatus::Closed => {
                tracing::info!("Account {} closed. Final settlement may be required.", account_id);
                // In production, this would trigger closure workflow
            }
            AccountStatus::Dormant => {
                tracing::info!("Account {} marked dormant. Reactivation required for transactions.", account_id);
                // In production, this would update transaction rules
            }
            _ => {}
        }

        Ok(())
    }

    /// Get total amount of active holds on account
    async fn get_total_active_holds(&self, _account_id: Uuid) -> BankingResult<Decimal> {
        // In production, this would query a holds repository
        // For now, return zero
        Ok(Decimal::ZERO)
    }

    /// Validate account is eligible for holds
    fn validate_hold_eligibility(&self, account: &Account) -> BankingResult<()> {
        match account.account_status {
            AccountStatus::Active => Ok(()),
            AccountStatus::Dormant => {
                // Dormant accounts can have holds applied but require reactivation for transactions
                Ok(())
            }
            _status => Err(banking_api::BankingError::AccountNotTransactional {
                account_id: account.account_id,
            }),
        }
    }

    /// Create a hold on the account
    async fn create_account_hold(&self, _account_id: Uuid, _amount: Decimal, _reason: &str) -> BankingResult<()> {
        // In production, this would create a record in the holds repository
        // For now, we'll just return success
        Ok(())
    }
}

// Helper trait extension for AccountMapper enum conversions
impl AccountMapper {
}

#[cfg(test)]
mod tests {
    use super::*;
    use banking_api::domain::{AccountType, SigningCondition};
    use chrono::{NaiveDate, Utc};

    #[tokio::test]
    async fn test_validate_account_data_success() {
        let mock_repo = Arc::new(MockAccountRepository::new());
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let valid_account = create_valid_test_account();
        assert!(service.validate_account_data(&valid_account).await.is_ok());
    }

    #[tokio::test]
    async fn test_validate_account_data_empty_product_code() {
        let mock_repo = Arc::new(MockAccountRepository::new());
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let mut invalid_account = create_valid_test_account();
        invalid_account.product_code = HeaplessString::try_from("").unwrap(); // Empty product code

        let result = service.validate_account_data(&invalid_account).await;
        assert!(result.is_err());
        if let Err(banking_api::BankingError::ValidationError { field, message }) = result {
            assert_eq!(field, "product_code");
            assert_eq!(message, "Product code is required");
        } else {
            panic!("Expected ValidationError for empty product code");
        }
    }

    #[tokio::test]
    async fn test_validate_account_data_whitespace_only_product_code() {
        let mock_repo = Arc::new(MockAccountRepository::new());
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let mut invalid_account = create_valid_test_account();
        let _ = invalid_account.set_product_code("   "); // Should fail validation

        let result = service.validate_account_data(&invalid_account).await;
        assert!(result.is_err());
        if let Err(banking_api::BankingError::ValidationError { field, message }) = result {
            assert_eq!(field, "product_code");
            assert_eq!(message, "Product code is required");
        } else {
            panic!("Expected ValidationError for whitespace-only product code");
        }
    }

    #[tokio::test]
    async fn test_validate_account_data_invalid_currency_too_short() {
        let mock_repo = Arc::new(MockAccountRepository::new());
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let mut invalid_account = create_valid_test_account();
        invalid_account.currency = HeaplessString::try_from("US").unwrap(); // Invalid: only 2 characters

        let result = service.validate_account_data(&invalid_account).await;
        assert!(result.is_err());
        if let Err(banking_api::BankingError::ValidationError { field, message }) = result {
            assert_eq!(field, "currency");
            assert_eq!(message, "Currency must be exactly 3 characters");
        } else {
            panic!("Expected ValidationError for short currency code");
        }
    }

    #[tokio::test]
    async fn test_validate_account_data_invalid_currency_too_long() {
        let mock_repo = Arc::new(MockAccountRepository::new());
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let mut invalid_account = create_valid_test_account();
        invalid_account.currency = HeaplessString::try_from("usd").unwrap(); // Invalid: lowercase

        let result = service.validate_account_data(&invalid_account).await;
        assert!(result.is_err());
        if let Err(banking_api::BankingError::ValidationError { field, message }) = result {
            assert_eq!(field, "currency");
            assert_eq!(message, "Currency must be a 3-character ISO code");
        } else {
            panic!("Expected ValidationError for lowercase currency code");
        }
    }

    #[tokio::test]
    async fn test_validate_account_data_empty_currency() {
        let mock_repo = Arc::new(MockAccountRepository::new());
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let mut invalid_account = create_valid_test_account();
        invalid_account.currency = HeaplessString::try_from("").unwrap(); // Invalid: empty currency

        let result = service.validate_account_data(&invalid_account).await;
        assert!(result.is_err());
        if let Err(banking_api::BankingError::ValidationError { field, message }) = result {
            assert_eq!(field, "currency");
            assert_eq!(message, "Currency must be exactly 3 characters");
        } else {
            panic!("Expected ValidationError for empty currency");
        }
    }

    #[tokio::test]
    async fn test_validate_account_data_invalid_balance_relationship() {
        let mock_repo = Arc::new(MockAccountRepository::new());
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let mut invalid_account = create_valid_test_account();
        // Set available balance higher than current balance + overdraft
        invalid_account.current_balance = Decimal::new(1000, 2); // $10.00
        invalid_account.available_balance = Decimal::new(1200, 2); // $12.00
        invalid_account.overdraft_limit = Some(Decimal::new(100, 2)); // $1.00
        // Available ($12.00) > Current ($10.00) + Overdraft ($1.00) = $11.00

        let result = service.validate_account_data(&invalid_account).await;
        assert!(result.is_err());
        if let Err(banking_api::BankingError::ValidationError { field, message }) = result {
            assert_eq!(field, "available_balance");
            assert_eq!(message, "Available balance cannot exceed current balance plus overdraft limit");
        } else {
            panic!("Expected ValidationError for invalid balance relationship");
        }
    }

    #[tokio::test]
    async fn test_validate_account_data_valid_balance_relationship_with_overdraft() {
        let mock_repo = Arc::new(MockAccountRepository::new());
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let mut valid_account = create_valid_test_account();
        valid_account.current_balance = Decimal::new(1000, 2); // $10.00
        valid_account.available_balance = Decimal::new(1100, 2); // $11.00
        valid_account.overdraft_limit = Some(Decimal::new(100, 2)); // $1.00
        // Available ($11.00) = Current ($10.00) + Overdraft ($1.00)

        let result = service.validate_account_data(&valid_account).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_account_data_valid_balance_relationship_no_overdraft() {
        let mock_repo = Arc::new(MockAccountRepository::new());
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let mut valid_account = create_valid_test_account();
        valid_account.current_balance = Decimal::new(1000, 2); // $10.00
        valid_account.available_balance = Decimal::new(1000, 2); // $10.00
        valid_account.overdraft_limit = None;
        // Available ($10.00) = Current ($10.00) + Overdraft ($0.00)

        let result = service.validate_account_data(&valid_account).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_account_data_loan_missing_original_principal() {
        let mock_repo = Arc::new(MockAccountRepository::new());
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let mut invalid_account = create_valid_test_account();
        invalid_account.account_type = AccountType::Loan;
        invalid_account.original_principal = None; // Missing
        invalid_account.outstanding_principal = Some(Decimal::new(5000, 2));

        let result = service.validate_account_data(&invalid_account).await;
        assert!(result.is_err());
        if let Err(banking_api::BankingError::ValidationError { field, message }) = result {
            assert_eq!(field, "loan_principals");
            assert_eq!(message, "Loan accounts must have original and outstanding principal amounts");
        } else {
            panic!("Expected ValidationError for missing original principal");
        }
    }

    #[tokio::test]
    async fn test_validate_account_data_loan_missing_outstanding_principal() {
        let mock_repo = Arc::new(MockAccountRepository::new());
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let mut invalid_account = create_valid_test_account();
        invalid_account.account_type = AccountType::Loan;
        invalid_account.original_principal = Some(Decimal::new(10000, 2));
        invalid_account.outstanding_principal = None; // Missing

        let result = service.validate_account_data(&invalid_account).await;
        assert!(result.is_err());
        if let Err(banking_api::BankingError::ValidationError { field, message }) = result {
            assert_eq!(field, "loan_principals");
            assert_eq!(message, "Loan accounts must have original and outstanding principal amounts");
        } else {
            panic!("Expected ValidationError for missing outstanding principal");
        }
    }

    #[tokio::test]
    async fn test_validate_account_data_loan_missing_both_principals() {
        let mock_repo = Arc::new(MockAccountRepository::new());
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let mut invalid_account = create_valid_test_account();
        invalid_account.account_type = AccountType::Loan;
        invalid_account.original_principal = None;
        invalid_account.outstanding_principal = None;

        let result = service.validate_account_data(&invalid_account).await;
        assert!(result.is_err());
        if let Err(banking_api::BankingError::ValidationError { field, message }) = result {
            assert_eq!(field, "loan_principals");
            assert_eq!(message, "Loan accounts must have original and outstanding principal amounts");
        } else {
            panic!("Expected ValidationError for missing both principals");
        }
    }

    #[tokio::test]
    async fn test_validate_account_data_loan_outstanding_exceeds_original() {
        let mock_repo = Arc::new(MockAccountRepository::new());
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let mut invalid_account = create_valid_test_account();
        invalid_account.account_type = AccountType::Loan;
        invalid_account.original_principal = Some(Decimal::new(10000, 2)); // $100.00
        invalid_account.outstanding_principal = Some(Decimal::new(15000, 2)); // $150.00

        let result = service.validate_account_data(&invalid_account).await;
        assert!(result.is_err());
        if let Err(banking_api::BankingError::ValidationError { field, message }) = result {
            assert_eq!(field, "outstanding_principal");
            assert_eq!(message, "Outstanding principal cannot exceed original principal");
        } else {
            panic!("Expected ValidationError for outstanding exceeding original");
        }
    }

    #[tokio::test]
    async fn test_validate_account_data_loan_valid_principals() {
        let mock_repo = Arc::new(MockAccountRepository::new());
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let mut valid_account = create_valid_test_account();
        valid_account.account_type = AccountType::Loan;
        valid_account.original_principal = Some(Decimal::new(10000, 2)); // $100.00
        valid_account.outstanding_principal = Some(Decimal::new(7500, 2)); // $75.00

        let result = service.validate_account_data(&valid_account).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_account_data_loan_equal_principals() {
        let mock_repo = Arc::new(MockAccountRepository::new());
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let mut valid_account = create_valid_test_account();
        valid_account.account_type = AccountType::Loan;
        valid_account.original_principal = Some(Decimal::new(10000, 2)); // $100.00
        valid_account.outstanding_principal = Some(Decimal::new(10000, 2)); // $100.00

        let result = service.validate_account_data(&valid_account).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_account_data_negative_overdraft_limit() {
        let mock_repo = Arc::new(MockAccountRepository::new());
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let mut invalid_account = create_valid_test_account();
        // Set balances so that balance validation passes
        invalid_account.current_balance = Decimal::new(1000, 2);
        invalid_account.available_balance = Decimal::new(500, 2); // Less than current balance
        invalid_account.overdraft_limit = Some(Decimal::new(-500, 2)); // Negative overdraft

        let result = service.validate_account_data(&invalid_account).await;
        assert!(result.is_err());
        if let Err(banking_api::BankingError::ValidationError { field, message }) = result {
            assert_eq!(field, "overdraft_limit");
            assert_eq!(message, "Overdraft limit cannot be negative");
        } else {
            panic!("Expected ValidationError for negative overdraft limit");
        }
    }

    #[tokio::test]
    async fn test_validate_account_data_zero_overdraft_limit() {
        let mock_repo = Arc::new(MockAccountRepository::new());
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let mut valid_account = create_valid_test_account();
        valid_account.overdraft_limit = Some(Decimal::ZERO); // Zero is valid

        let result = service.validate_account_data(&valid_account).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_account_data_positive_overdraft_limit() {
        let mock_repo = Arc::new(MockAccountRepository::new());
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let mut valid_account = create_valid_test_account();
        valid_account.overdraft_limit = Some(Decimal::new(100000, 2)); // $1000.00

        let result = service.validate_account_data(&valid_account).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_account_data_savings_account_no_overdraft_needed() {
        let mock_repo = Arc::new(MockAccountRepository::new());
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let mut valid_account = create_valid_test_account();
        valid_account.account_type = AccountType::Savings;
        valid_account.overdraft_limit = None; // Savings typically don't have overdraft

        let result = service.validate_account_data(&valid_account).await;
        assert!(result.is_ok());
    }

    // Helper function to create valid test account
    fn create_valid_test_account() -> Account {
        Account {
            account_id: Uuid::new_v4(),
            product_code: HeaplessString::try_from("SAV001").unwrap(),
            account_type: AccountType::Savings,
            account_status: AccountStatus::Active,
            signing_condition: SigningCondition::None,
            currency: HeaplessString::try_from("USD").unwrap(),
            open_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            domicile_branch_id: Uuid::new_v4(),
            current_balance: Decimal::new(1000, 2),
            available_balance: Decimal::new(1000, 2),
            accrued_interest: Decimal::ZERO,
            overdraft_limit: None,
            original_principal: None,
            outstanding_principal: None,
            loan_interest_rate: None,
            loan_term_months: None,
            disbursement_date: None,
            maturity_date: None,
            installment_amount: None,
            next_due_date: None,
            penalty_rate: None,
            collateral_id: None,
            loan_purpose_id: None,
            close_date: None,
            last_activity_date: None,
            dormancy_threshold_days: None,
            reactivation_required: false,
            pending_closure_reason_id: None,
            disbursement_instructions: None,
            status_changed_by: None,
            status_change_reason_id: None,
            status_change_timestamp: None,
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
            updated_by: Uuid::new_v4(), // Changed to UUID for ReferencedPerson.person_id
        }
    }

    // Balance Calculation Tests
    
    #[tokio::test]
    async fn test_calculate_balance_savings_account() {
        let account_id = Uuid::new_v4();
        let mut account_model = create_test_account_model(AccountType::Savings, Decimal::new(50000, 2), None);
        account_model.account_id = account_id;
        
        let mock_repo = Arc::new(MockAccountRepository::new().with_account(account_model));
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let balance = service.calculate_balance(account_id).await.unwrap();
        
        assert_eq!(balance, Decimal::new(50000, 2)); // $500.00
    }

    #[tokio::test]
    async fn test_calculate_balance_current_account() {
        let account_id = Uuid::new_v4();
        let mut account_model = create_test_account_model(AccountType::Current, Decimal::new(75000, 2), None);
        account_model.account_id = account_id;
        
        let mock_repo = Arc::new(MockAccountRepository::new().with_account(account_model));
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let balance = service.calculate_balance(account_id).await.unwrap();
        
        assert_eq!(balance, Decimal::new(75000, 2)); // $750.00
    }

    #[tokio::test]
    async fn test_calculate_balance_loan_account_returns_outstanding_principal() {
        let account_id = Uuid::new_v4();
        let mut account_model = create_test_loan_account_model(
            Decimal::new(100000, 2), // original: $1000.00
            Decimal::new(60000, 2),  // outstanding: $600.00
            Decimal::new(25000, 2)   // current_balance: $250.00 (ignored for loans)
        );
        account_model.account_id = account_id;
        
        let mock_repo = Arc::new(MockAccountRepository::new().with_account(account_model));
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let balance = service.calculate_balance(account_id).await.unwrap();
        
        // For loan accounts, should return outstanding principal, not current balance
        assert_eq!(balance, Decimal::new(60000, 2)); // $600.00
    }

    #[tokio::test]
    async fn test_calculate_balance_loan_account_with_zero_outstanding() {
        let account_id = Uuid::new_v4();
        let mut account_model = create_test_loan_account_model(
            Decimal::new(100000, 2), // original: $1000.00
            Decimal::ZERO,           // outstanding: $0.00 (fully paid)
            Decimal::new(25000, 2)   // current_balance: $250.00 (ignored)
        );
        account_model.account_id = account_id;
        
        let mock_repo = Arc::new(MockAccountRepository::new().with_account(account_model));
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let balance = service.calculate_balance(account_id).await.unwrap();
        
        assert_eq!(balance, Decimal::ZERO); // Loan fully paid
    }

    #[tokio::test]
    async fn test_calculate_balance_account_not_found() {
        let mock_repo = Arc::new(MockAccountRepository::new()); // Empty repository
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let non_existent_account_id = Uuid::new_v4();
        let result = service.calculate_balance(non_existent_account_id).await;
        
        assert!(result.is_err());
        if let Err(banking_api::BankingError::AccountNotFound(account_id)) = result {
            assert_eq!(account_id, non_existent_account_id);
        } else {
            panic!("Expected AccountNotFound error");
        }
    }

    // Available Balance Calculation Tests
    
    #[tokio::test]
    async fn test_calculate_available_balance_current_account_with_overdraft() {
        let account_id = Uuid::new_v4();
        let mut account_model = create_test_account_model(AccountType::Current, Decimal::new(50000, 2), Some(Decimal::new(25000, 2))); // $500 balance, $250 overdraft
        account_model.account_id = account_id;
        
        let mock_repo = Arc::new(MockAccountRepository::new().with_account(account_model));
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let available = service.calculate_available_balance(account_id).await.unwrap();
        
        // Available = Current ($500) - Holds ($0 since mock returns zero) + Overdraft ($250) = $750
        assert_eq!(available, Decimal::new(75000, 2)); // $750.00
    }

    #[tokio::test]
    async fn test_calculate_available_balance_current_account_no_overdraft() {
        let account_id = Uuid::new_v4();
        let mut account_model = create_test_account_model(AccountType::Current, Decimal::new(50000, 2), None); // $500 balance, no overdraft
        account_model.account_id = account_id;
        
        let mock_repo = Arc::new(MockAccountRepository::new().with_account(account_model));
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let available = service.calculate_available_balance(account_id).await.unwrap();
        
        // Available = Current ($500) - Holds ($0) + Overdraft ($0) = $500
        assert_eq!(available, Decimal::new(50000, 2)); // $500.00
    }

    #[tokio::test]
    async fn test_calculate_available_balance_savings_account_positive() {
        let account_id = Uuid::new_v4();
        let mut account_model = create_test_account_model(AccountType::Savings, Decimal::new(50000, 2), None); // $500 balance
        account_model.account_id = account_id;
        
        let mock_repo = Arc::new(MockAccountRepository::new().with_account(account_model));
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let available = service.calculate_available_balance(account_id).await.unwrap();
        
        // Available = Current ($500) - Holds ($0) = $500 (no overdraft for savings)
        assert_eq!(available, Decimal::new(50000, 2)); // $500.00
    }

    #[tokio::test]
    async fn test_calculate_available_balance_savings_account_with_low_balance() {
        let account_id = Uuid::new_v4();
        let mut account_model = create_test_account_model(AccountType::Savings, Decimal::new(5000, 2), None); // $50 balance
        account_model.account_id = account_id;
        
        let mock_repo = Arc::new(MockAccountRepository::new().with_account(account_model));
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let available = service.calculate_available_balance(account_id).await.unwrap();
        
        // Available = max(Current ($50) - Holds ($0), 0) = $50
        assert_eq!(available, Decimal::new(5000, 2)); // $50.00
    }

    #[tokio::test]
    async fn test_calculate_available_balance_loan_account_always_zero() {
        let account_id = Uuid::new_v4();
        let mut account_model = create_test_loan_account_model(
            Decimal::new(100000, 2), // original: $1000.00
            Decimal::new(60000, 2),  // outstanding: $600.00
            Decimal::new(25000, 2)   // current_balance: $250.00
        );
        account_model.account_id = account_id;
        
        let mock_repo = Arc::new(MockAccountRepository::new().with_account(account_model));
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let available = service.calculate_available_balance(account_id).await.unwrap();
        
        // Loan accounts always have zero available balance
        assert_eq!(available, Decimal::ZERO);
    }

    #[tokio::test]
    async fn test_calculate_available_balance_current_account_with_large_overdraft() {
        let account_id = Uuid::new_v4();
        let mut account_model = create_test_account_model(AccountType::Current, Decimal::new(-5000, 2), Some(Decimal::new(100000, 2))); // -$50 balance, $1000 overdraft
        account_model.account_id = account_id;
        
        let mock_repo = Arc::new(MockAccountRepository::new().with_account(account_model));
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let available = service.calculate_available_balance(account_id).await.unwrap();
        
        // Available = Current (-$50) - Holds ($0) + Overdraft ($1000) = $950
        assert_eq!(available, Decimal::new(95000, 2)); // $950.00
    }

    #[tokio::test]
    async fn test_calculate_available_balance_account_not_found() {
        let mock_repo = Arc::new(MockAccountRepository::new()); // Empty repository
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let non_existent_account_id = Uuid::new_v4();
        let result = service.calculate_available_balance(non_existent_account_id).await;
        
        assert!(result.is_err());
        if let Err(banking_api::BankingError::AccountNotFound(account_id)) = result {
            assert_eq!(account_id, non_existent_account_id);
        } else {
            panic!("Expected AccountNotFound error");
        }
    }

    // Helper functions for creating test data
    
    fn create_test_account_model(account_type: AccountType, current_balance: Decimal, overdraft_limit: Option<Decimal>) -> banking_db::models::AccountModel {
        banking_db::models::AccountModel {
            account_id: Uuid::new_v4(),
            product_code: HeaplessString::try_from("TEST001").unwrap(),
            account_type,
            account_status: AccountStatus::Active,
            signing_condition: SigningCondition::None,
            currency: HeaplessString::try_from("USD").unwrap(),
            open_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            domicile_branch_id: Uuid::new_v4(),
            current_balance,
            available_balance: current_balance,
            accrued_interest: Decimal::ZERO,
            overdraft_limit,
            original_principal: None,
            outstanding_principal: None,
            loan_interest_rate: None,
            loan_term_months: None,
            disbursement_date: None,
            maturity_date: None,
            installment_amount: None,
            next_due_date: None,
            penalty_rate: None,
            collateral_id: None,
            loan_purpose_id: None,
            close_date: None,
            last_activity_date: None,
            dormancy_threshold_days: None,
            reactivation_required: false,
            pending_closure_reason_id: None,
            status_changed_by: None,
            status_change_reason_id: None,
            status_change_timestamp: None,
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
            updated_by: Uuid::new_v4(), // Changed to UUID for ReferencedPerson.person_id
        }
    }

    fn create_test_loan_account_model(original_principal: Decimal, outstanding_principal: Decimal, current_balance: Decimal) -> banking_db::models::AccountModel {
        banking_db::models::AccountModel {
            account_id: Uuid::new_v4(),
            product_code: HeaplessString::try_from("LOAN001").unwrap(),
            account_type: AccountType::Loan,
            account_status: AccountStatus::Active,
            signing_condition: SigningCondition::None,
            currency: HeaplessString::try_from("USD").unwrap(),
            open_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            domicile_branch_id: Uuid::new_v4(),
            current_balance,
            available_balance: Decimal::ZERO,
            accrued_interest: Decimal::ZERO,
            overdraft_limit: None,
            original_principal: Some(original_principal),
            outstanding_principal: Some(outstanding_principal),
            loan_interest_rate: Some(Decimal::new(750, 4)), // 7.5%
            loan_term_months: Some(36),
            disbursement_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            maturity_date: Some(NaiveDate::from_ymd_opt(2027, 1, 1).unwrap()),
            installment_amount: Some(Decimal::new(30000, 2)),
            next_due_date: Some(NaiveDate::from_ymd_opt(2024, 2, 1).unwrap()),
            penalty_rate: Some(Decimal::new(200, 4)), // 2%
            collateral_id: Some(heapless::String::try_from(Uuid::new_v4().to_string().as_str()).unwrap()),
            loan_purpose_id: None,  // Changed to UUID reference
            close_date: None,
            last_activity_date: Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
            dormancy_threshold_days: None,
            reactivation_required: false,
            pending_closure_reason_id: None,
            status_changed_by: None,
            status_change_reason_id: None,
            status_change_timestamp: None,
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
            updated_by: Uuid::new_v4(), // Changed to UUID for ReferencedPerson.person_id
        }
    }

    // Enhanced Mock repository for testing
    use std::collections::HashMap;
    use std::sync::Mutex;

    struct MockAccountRepository {
        accounts: Arc<Mutex<HashMap<Uuid, banking_db::models::AccountModel>>>,
        should_error: bool,
    }

    impl MockAccountRepository {
        fn new() -> Self {
            Self {
                accounts: Arc::new(Mutex::new(HashMap::new())),
                should_error: false,
            }
        }
        
        fn with_account(self, account: banking_db::models::AccountModel) -> Self {
            self.accounts.lock().unwrap().insert(account.account_id, account);
            self
        }
    }

    #[async_trait]
    impl AccountRepository for MockAccountRepository {
        async fn create(&self, _account: banking_db::models::AccountModel) -> BankingResult<banking_db::models::AccountModel> {
            unimplemented!()
        }

        async fn find_by_id(&self, account_id: Uuid) -> BankingResult<Option<banking_db::models::AccountModel>> {
            if self.should_error {
                return Err(banking_api::BankingError::Internal("Mock error".to_string()));
            }
            
            Ok(self.accounts.lock().unwrap().get(&account_id).cloned())
        }

        async fn update_status(&self, _account_id: Uuid, _status: &str, _reason: &str, _authorized_by: Uuid) -> BankingResult<()> {
            Ok(())
        }

        async fn exists(&self, account_id: Uuid) -> BankingResult<bool> {
            if self.should_error {
                return Err(banking_api::BankingError::Internal("Mock error".to_string()));
            }
            
            Ok(self.accounts.lock().unwrap().contains_key(&account_id))
        }

        async fn update(&self, _account: banking_db::models::AccountModel) -> BankingResult<banking_db::models::AccountModel> {
            todo!()
        }

        async fn find_by_customer_id(&self, _customer_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountModel>> {
            todo!()
        }

        async fn find_by_product_code(&self, _product_code: &str) -> BankingResult<Vec<banking_db::models::AccountModel>> {
            todo!()
        }

        async fn find_by_status(&self, _status: &str) -> BankingResult<Vec<banking_db::models::AccountModel>> {
            todo!()
        }

        async fn find_dormancy_candidates(&self, _reference_date: chrono::NaiveDate, _threshold_days: i32) -> BankingResult<Vec<banking_db::models::AccountModel>> {
            todo!()
        }

        async fn find_pending_closure(&self) -> BankingResult<Vec<banking_db::models::AccountModel>> {
            todo!()
        }

        async fn find_interest_bearing_accounts(&self) -> BankingResult<Vec<banking_db::models::AccountModel>> {
            todo!()
        }

        async fn update_balance(&self, _account_id: Uuid, _current_balance: rust_decimal::Decimal, _available_balance: rust_decimal::Decimal) -> BankingResult<()> {
            todo!()
        }

        async fn update_accrued_interest(&self, _account_id: Uuid, _accrued_interest: rust_decimal::Decimal) -> BankingResult<()> {
            todo!()
        }

        async fn reset_accrued_interest(&self, _account_id: Uuid) -> BankingResult<()> {
            todo!()
        }

        async fn create_ownership(&self, _ownership: banking_db::models::AccountOwnershipModel) -> BankingResult<banking_db::models::AccountOwnershipModel> {
            todo!()
        }

        async fn find_ownership_by_account(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountOwnershipModel>> {
            todo!()
        }

        async fn find_accounts_by_owner(&self, _customer_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountOwnershipModel>> {
            todo!()
        }

        async fn delete_ownership(&self, _ownership_id: Uuid) -> BankingResult<()> {
            todo!()
        }

        async fn create_relationship(&self, _relationship: banking_db::models::AccountRelationshipModel) -> BankingResult<banking_db::models::AccountRelationshipModel> {
            todo!()
        }

        async fn find_relationships_by_account(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountRelationshipModel>> {
            todo!()
        }

        async fn find_relationships_by_entity(&self, _entity_id: Uuid, _relationship_type: &str) -> BankingResult<Vec<banking_db::models::AccountRelationshipModel>> {
            todo!()
        }

        async fn update_relationship(&self, _relationship: banking_db::models::AccountRelationshipModel) -> BankingResult<banking_db::models::AccountRelationshipModel> {
            todo!()
        }

        async fn delete_relationship(&self, _relationship_id: Uuid) -> BankingResult<()> {
            todo!()
        }

        async fn create_mandate(&self, _mandate: banking_db::models::AccountMandateModel) -> BankingResult<banking_db::models::AccountMandateModel> {
            todo!()
        }

        async fn find_mandates_by_account(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountMandateModel>> {
            todo!()
        }

        async fn find_mandates_by_grantee(&self, _grantee_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountMandateModel>> {
            todo!()
        }

        async fn update_mandate_status(&self, _mandate_id: Uuid, _status: &str) -> BankingResult<()> {
            todo!()
        }

        async fn find_active_mandates(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountMandateModel>> {
            todo!()
        }

        async fn create_hold(&self, _hold: banking_db::models::AccountHoldModel) -> BankingResult<banking_db::models::AccountHoldModel> {
            todo!()
        }

        async fn find_holds_by_account(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> {
            todo!()
        }

        async fn find_active_holds(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> {
            todo!()
        }

        async fn release_hold(&self, _hold_id: Uuid, _released_by: Uuid) -> BankingResult<()> {
            todo!()
        }

        async fn release_expired_holds(&self, _expiry_cutoff: chrono::DateTime<chrono::Utc>) -> BankingResult<i64> {
            todo!()
        }

        async fn create_final_settlement(&self, _settlement: banking_db::models::AccountFinalSettlementModel) -> BankingResult<banking_db::models::AccountFinalSettlementModel> {
            todo!()
        }

        async fn find_settlement_by_account(&self, _account_id: Uuid) -> BankingResult<Option<banking_db::models::AccountFinalSettlementModel>> {
            todo!()
        }

        async fn update_settlement_status(&self, _settlement_id: Uuid, _status: &str) -> BankingResult<()> {
            todo!()
        }

        async fn get_status_history(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountStatusHistoryModel>> {
            todo!()
        }

        async fn add_status_change(&self, _status_change: banking_db::models::AccountStatusHistoryModel) -> BankingResult<banking_db::models::AccountStatusHistoryModel> {
            todo!()
        }

        async fn count_by_customer(&self, _customer_id: Uuid) -> BankingResult<i64> {
            todo!()
        }

        async fn count_by_product(&self, _product_code: &str) -> BankingResult<i64> {
            todo!()
        }

        async fn list(&self, _limit: i64, _offset: i64) -> BankingResult<Vec<banking_db::models::AccountModel>> {
            todo!()
        }

        async fn count(&self) -> BankingResult<i64> {
            todo!()
        }

        async fn update_last_activity_date(&self, _account_id: Uuid, _activity_date: chrono::NaiveDate) -> BankingResult<()> {
            todo!()
        }
    }
}

impl AccountServiceImpl {
    // Helper function for status conversion (temporary until repository is updated)
    fn account_status_to_string(status: AccountStatus) -> String {
        match status {
            AccountStatus::PendingApproval => "PendingApproval".to_string(),
            AccountStatus::Active => "Active".to_string(),
            AccountStatus::Dormant => "Dormant".to_string(),
            AccountStatus::Frozen => "Frozen".to_string(),
            AccountStatus::PendingClosure => "PendingClosure".to_string(),
            AccountStatus::Closed => "Closed".to_string(),
            AccountStatus::PendingReactivation => "PendingReactivation".to_string(),
        }
    }
}
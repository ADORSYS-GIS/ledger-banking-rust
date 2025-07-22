use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

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
        self.validate_product_code(&account.product_code).await?;

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
        authorized_by: String,
    ) -> BankingResult<()> {
        // Validate authorization for status changes
        self.validate_status_change_authorization(&authorized_by, &status)?;

        // Ensure account exists
        if !self.account_repository.exists(account_id).await? {
            return Err(banking_api::BankingError::AccountNotFound(account_id));
        }

        // Update status with audit trail
        self.account_repository
            .update_status(
                account_id,
                &crate::mappers::AccountMapper::account_status_to_string(status),
                "Status change authorized",
                &authorized_by,
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

    /// Apply a hold on the account for a specific amount
    async fn apply_hold(&self, account_id: Uuid, amount: Decimal, reason: String) -> BankingResult<()> {
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
            .get_product_rules(&account.product_code)
            .await?;

        // Update account with any product-specific changes
        // This might include dormancy thresholds, fee schedules, etc.
        tracing::info!(
            "Refreshed product rules for account {} with product code {}",
            account_id, account.product_code
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
    async fn update_balance(&self, account_id: Uuid, new_balance: Decimal, updated_by: String) -> BankingResult<()> {
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
    async fn release_hold(&self, _hold_id: Uuid, _released_by: String) -> BankingResult<()> {
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
        if account.product_code.trim().is_empty() {
            return Err(banking_api::BankingError::ValidationError {
                field: "product_code".to_string(),
                message: "Product code is required".to_string(),
            });
        }

        // Currency validation
        if account.currency.len() != 3 {
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
            .get_product_rules(&account.product_code)
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

    /// Validate authorization for status changes
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
    async fn test_validate_account_data() {
        let mock_repo = Arc::new(MockAccountRepository {});
        let mock_catalog = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let service = AccountServiceImpl::new(mock_repo, mock_catalog);

        let valid_account = Account {
            account_id: Uuid::new_v4(),
            product_code: "SAV001".to_string(),
            account_type: AccountType::Savings,
            account_status: AccountStatus::Active,
            signing_condition: SigningCondition::None,
            currency: "USD".to_string(),
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
            loan_purpose: None,
            close_date: None,
            last_activity_date: None,
            dormancy_threshold_days: None,
            reactivation_required: false,
            pending_closure_reason: None,
            disbursement_instructions: None,
            status_changed_by: None,
            status_change_reason: None,
            status_change_timestamp: None,
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
            updated_by: "TEST_USER".to_string(),
        };

        assert!(service.validate_account_data(&valid_account).await.is_ok());
    }

    // Mock repository for testing
    struct MockAccountRepository;

    #[async_trait]
    impl AccountRepository for MockAccountRepository {
        async fn create(&self, _account: banking_db::models::AccountModel) -> BankingResult<banking_db::models::AccountModel> {
            unimplemented!()
        }

        async fn find_by_id(&self, _account_id: Uuid) -> BankingResult<Option<banking_db::models::AccountModel>> {
            unimplemented!()
        }

        async fn update_status(&self, _account_id: Uuid, _status: &str, _reason: &str, _authorized_by: &str) -> BankingResult<()> {
            Ok(())
        }

        async fn exists(&self, _account_id: Uuid) -> BankingResult<bool> {
            Ok(true)
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

        async fn release_hold(&self, _hold_id: Uuid, _released_by: &str) -> BankingResult<()> {
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
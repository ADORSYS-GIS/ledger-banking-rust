use std::sync::Arc;
use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

#[cfg(test)]
use heapless::String as HeaplessString;

use banking_api::{
    BankingResult, BankingError, Account, AccountStatus,
    service::{
        AccountService, HoldAuthorizationLevel, HoldAnalytics, 
        HighHoldAccount, JudicialHoldReport
    },
    domain::{
        AccountHold, HoldType, HoldStatus, HoldPriority, HoldReleaseRequest,
        BalanceCalculation, HoldSummary, HoldExpiryJob, PlaceHoldRequest
    },
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
    async fn update_balance(&self, account_id: Uuid, new_balance: Decimal, updated_by_person_id: Uuid) -> BankingResult<()> {
        // In a real implementation, we'd calculate available balance based on holds
        self.account_repository.update_balance(account_id, new_balance, new_balance).await?;
        
        tracing::info!("Account {} balance updated to {} by {}", account_id, new_balance, updated_by_person_id);
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

    // ============================================================================
    // HOLD PLACEMENT AND MANAGEMENT (integrated from HoldServiceImpl)
    // ============================================================================
    
    /// Place a hold on an account with specified amount and type
    async fn place_hold(
        &self,
        request: PlaceHoldRequest,
    ) -> BankingResult<AccountHold> {
        tracing::info!("Placing hold of {} on account {} with type {:?}", 
                      request.amount, request.account_id, request.hold_type);

        // Validate account exists and is operational
        let account = self.account_repository
            .find_by_id(request.account_id)
            .await?
            .ok_or(BankingError::AccountNotFound(request.account_id))?;

        // Convert database model to domain for validation
        let account_domain = AccountMapper::from_model(account)?;

        // Validate account can have holds applied
        self.validate_hold_eligibility(&account_domain)?;

        // Validate hold amount
        if request.amount <= Decimal::ZERO {
            return Err(BankingError::ValidationError {
                field: "amount".to_string(),
                message: "Hold amount must be positive".to_string(),
            });
        }

        // Check if placing hold would exceed available balance constraints
        self.validate_hold_placement(
            request.account_id,
            request.amount,
            request.priority.clone(),
        ).await?;

        // Create the hold
        let hold = AccountHold {
            id: Uuid::new_v4(),
            account_id: request.account_id,
            amount: request.amount,
            hold_type: request.hold_type,
            reason_id: request.reason_id,
            additional_details: request.additional_details,
            placed_by_person_id: request.placed_by_person_id,
            placed_at: Utc::now(),
            expires_at: request.expires_at,
            status: HoldStatus::Active,
            released_at: None,
            released_by_person_id: None,
            priority: request.priority,
            source_reference: request.source_reference,
            automatic_release: request.expires_at.is_some(),
        };

        // Persist the hold
        let hold_model = AccountMapper::account_hold_to_model(hold.clone());
        let created_model = self.account_repository.create_hold(hold_model).await?;
        
        tracing::info!("Successfully placed hold {} for amount {} on account {}", 
                      created_model.id, created_model.amount, created_model.account_id);

        Ok(AccountMapper::account_hold_from_model(created_model))
    }

    async fn release_hold_with_request(
        &self,
        release_request: HoldReleaseRequest,
    ) -> BankingResult<AccountHold> {
        tracing::info!("Releasing hold {} by user {}", 
                      release_request.hold_id, release_request.released_by_person_id);

        // Find the hold
        let hold_model = self.account_repository
            .get_hold_by_id(release_request.hold_id)
            .await?
            .ok_or_else(|| BankingError::ValidationError {
                field: "hold_id".to_string(),
                message: "Hold not found".to_string(),
            })?;

        // Validate hold can be released
        if hold_model.status != HoldStatus::Active {
            return Err(BankingError::ValidationError {
                field: "status".to_string(),
                message: "Only active holds can be released".to_string(),
            });
        }

        // Determine release amount (full release if not specified)
        let release_amount = release_request.release_amount.unwrap_or(hold_model.amount);
        
        if release_amount > hold_model.amount {
            return Err(BankingError::ValidationError {
                field: "release_amount".to_string(),
                message: "Release amount cannot exceed hold amount".to_string(),
            });
        }

        // Release the hold using the enhanced repository method
        let released_model = self.account_repository
            .release_hold_detailed(
                release_request.hold_id,
                Some(release_amount),
                release_request.release_reason_id,
                release_request.released_by_person_id,
                Utc::now(),
            )
            .await?;
        
        tracing::info!("Successfully released hold {} with amount {}", 
                      released_model.id, release_amount);

        Ok(AccountMapper::account_hold_from_model(released_model))
    }

    async fn modify_hold(
        &self,
        hold_id: Uuid,
        new_amount: Option<Decimal>,
        new_expiry: Option<DateTime<Utc>>,
        new_reason_id: Option<Uuid>,
        modified_by: Uuid,
    ) -> BankingResult<AccountHold> {
        tracing::info!("Modifying hold {} by {}", hold_id, modified_by);

        // Find the hold
        let mut hold_model = self.account_repository
            .get_hold_by_id(hold_id)
            .await?
            .ok_or_else(|| BankingError::ValidationError {
                field: "hold_id".to_string(),
                message: "Hold not found".to_string(),
            })?;

        // Validate hold can be modified
        if hold_model.status != HoldStatus::Active {
            return Err(BankingError::ValidationError {
                field: "status".to_string(),
                message: "Only active holds can be modified".to_string(),
            });
        }

        // Apply modifications
        if let Some(amount) = new_amount {
            if amount <= Decimal::ZERO {
                return Err(BankingError::ValidationError {
                    field: "new_amount".to_string(),
                    message: "Hold amount must be positive".to_string(),
                });
            }
            hold_model.amount = amount;
        }
        
        if let Some(expiry) = new_expiry {
            hold_model.expires_at = Some(expiry);
            hold_model.automatic_release = true;
        }

        if let Some(reason_id) = new_reason_id {
            hold_model.reason_id = reason_id;
        }

        // Update the hold
        let updated_model = self.account_repository.update_hold(hold_model).await?;
        
        tracing::info!("Successfully modified hold {}", hold_id);

        Ok(AccountMapper::account_hold_from_model(updated_model))
    }

    async fn cancel_hold(
        &self,
        hold_id: Uuid,
        cancellation_reason_id: Uuid,
        cancelled_by: Uuid,
    ) -> BankingResult<AccountHold> {
        tracing::info!("Cancelling hold {} by {} for reason ID: {}", 
                      hold_id, cancelled_by, cancellation_reason_id);

        // Find and validate the hold
        let mut hold_model = self.account_repository
            .get_hold_by_id(hold_id)
            .await?
            .ok_or_else(|| BankingError::ValidationError {
                field: "hold_id".to_string(),
                message: "Hold not found".to_string(),
            })?;

        // Validate hold can be cancelled
        if hold_model.status != HoldStatus::Active {
            return Err(BankingError::ValidationError {
                field: "status".to_string(),
                message: "Only active holds can be cancelled".to_string(),
            });
        }

        // Cancel the hold by updating status
        hold_model.status = HoldStatus::Cancelled;
        hold_model.released_at = Some(Utc::now());
        hold_model.released_by_person_id = Some(cancelled_by);
        hold_model.reason_id = cancellation_reason_id;

        let cancelled_model = self.account_repository.update_hold(hold_model).await?;
        
        tracing::info!("Successfully cancelled hold {}", hold_id);

        Ok(AccountMapper::account_hold_from_model(cancelled_model))
    }

    // ============================================================================
    // BALANCE CALCULATION ENGINE (enhanced)
    // ============================================================================

    async fn calculate_available_balance_detailed(
        &self,
        account_id: Uuid,
    ) -> BankingResult<BalanceCalculation> {
        tracing::debug!("Calculating available balance for account {}", account_id);

        // Get account details
        let account = self.account_repository
            .find_by_id(account_id)
            .await?
            .ok_or(BankingError::AccountNotFound(account_id))?;

        // Get active holds
        let active_holds = self.account_repository
            .get_active_holds_for_account(account_id, None)
            .await?;

        // Calculate total holds
        let total_holds = active_holds.iter()
            .map(|h| h.amount)
            .fold(Decimal::ZERO, |acc, amount| acc + amount);

        // Calculate available balance based on account type
        let available_balance = match account.account_type {
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

        // Create hold breakdown
        let hold_breakdown = self.create_hold_breakdown(&active_holds);

        Ok(BalanceCalculation {
            account_id,
            current_balance: account.current_balance,
            available_balance,
            overdraft_limit: account.overdraft_limit,
            total_holds,
            active_hold_count: active_holds.len() as u32,
            calculation_timestamp: Utc::now(),
            hold_breakdown,
        })
    }

    async fn validate_transaction_against_holds(
        &self,
        account_id: Uuid,
        transaction_amount: Decimal,
        ignore_hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<bool> {
        tracing::debug!("Validating transaction of {} against holds for account {}", 
                       transaction_amount, account_id);

        // Calculate current available balance
        let balance_calc = self.calculate_available_balance_detailed(account_id).await?;
        
        // Adjust for ignored hold types
        let effective_available = if let Some(ignore_types) = ignore_hold_types {
            let ignored_amount = balance_calc.hold_breakdown.iter()
                .filter(|summary| ignore_types.contains(&summary.hold_type))
                .map(|summary| summary.total_amount)
                .fold(Decimal::ZERO, |acc, amount| acc + amount);
            
            balance_calc.available_balance + ignored_amount
        } else {
            balance_calc.available_balance
        };

        Ok(effective_available >= transaction_amount)
    }

    async fn get_hold_amounts_by_priority(
        &self,
        account_id: Uuid,
    ) -> BankingResult<Vec<HoldSummary>> {
        let active_holds = self.account_repository
            .get_active_holds_for_account(account_id, None)
            .await?;

        let summary = self.create_hold_breakdown(&active_holds);
        Ok(summary)
    }

    async fn validate_hold_placement(
        &self,
        account_id: Uuid,
        additional_hold_amount: Decimal,
        hold_priority: HoldPriority,
    ) -> BankingResult<bool> {
        // For high-priority holds, allow placement even if it would exceed balance
        match hold_priority {
            HoldPriority::Critical => Ok(true),
            _ => {
                let balance_calc = self.calculate_available_balance_detailed(account_id).await?;
                Ok(balance_calc.available_balance >= additional_hold_amount)
            }
        }
    }

    // ============================================================================
    // HOLD QUERIES AND REPORTING
    // ============================================================================

    async fn get_active_holds_with_types(
        &self,
        account_id: Uuid,
        hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<Vec<AccountHold>> {
        let holds = self.account_repository
            .get_active_holds_for_account(account_id, hold_types.map(|types| 
                types.into_iter().map(|t| t.to_string()).collect()
            ))
            .await?;

        Ok(holds.into_iter()
           .map(AccountMapper::account_hold_from_model)
           .collect())
    }

    async fn get_hold_by_id(
        &self,
        hold_id: Uuid,
    ) -> BankingResult<Option<AccountHold>> {
        if let Some(hold_model) = self.account_repository
            .get_hold_by_id(hold_id)
            .await? {
            Ok(Some(AccountMapper::account_hold_from_model(hold_model)))
        } else {
            Ok(None)
        }
    }

    async fn get_holds_by_status(
        &self,
        account_id: Option<Uuid>,
        status: HoldStatus,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> BankingResult<Vec<AccountHold>> {
        let holds = self.account_repository
            .get_holds_by_status(account_id, status.to_string(), from_date, to_date)
            .await?;

        Ok(holds.into_iter()
           .map(AccountMapper::account_hold_from_model)
           .collect())
    }

    async fn get_holds_by_type(
        &self,
        hold_type: HoldType,
        status: Option<HoldStatus>,
        account_ids: Option<Vec<Uuid>>,
    ) -> BankingResult<Vec<AccountHold>> {
        let holds = self.account_repository
            .get_holds_by_type(
                hold_type.to_string(),
                status.map(|s| s.to_string()),
                account_ids,
                None,
            )
            .await?;

        Ok(holds.into_iter()
           .map(AccountMapper::account_hold_from_model)
           .collect())
    }

    async fn get_hold_history(
        &self,
        account_id: Uuid,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> BankingResult<Vec<AccountHold>> {
        let holds = self.account_repository
            .get_hold_history(account_id, from_date, to_date, true)
            .await?;

        Ok(holds.into_iter()
           .map(AccountMapper::account_hold_from_model)
           .collect())
    }

    // ============================================================================
    // BATCH PROCESSING AND AUTOMATION
    // ============================================================================

    async fn process_expired_holds(
        &self,
        processing_date: NaiveDate,
        hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<HoldExpiryJob> {
        tracing::info!("Processing expired holds for date {}", processing_date);

        let job_id = Uuid::new_v4();
        let expired_holds = self.account_repository
            .get_expired_holds(
                processing_date.and_hms_opt(23, 59, 59).unwrap().and_utc(),
                hold_types.map(|types| types.into_iter().map(|t| t.to_string()).collect()),
                None,
            )
            .await?;

        let total_amount: Decimal = expired_holds.iter().map(|h| h.amount).sum();
        let processed_count = expired_holds.len() as u32;

        // Process each expired hold
        for hold in expired_holds {
            let _ = self.account_repository
                .release_hold_detailed(
                    hold.id,
                    None, // Full release
                    Uuid::new_v4(), // System reason ID - should be configurable
                    Uuid::new_v4(), // System user ID - should be configurable
                    Utc::now(),
                )
                .await;
        }

        Ok(HoldExpiryJob {
            id: job_id,
            processing_date,
            expired_holds_count: processed_count,
            total_released_amount: total_amount,
            processed_at: Utc::now(),
            errors: Vec::new(), // TODO: Collect actual errors during processing
        })
    }

    async fn process_automatic_releases(
        &self,
        processing_date: NaiveDate,
    ) -> BankingResult<Vec<AccountHold>> {
        tracing::info!("Processing automatic releases for date {}", processing_date);

        let eligible_holds = self.account_repository
            .get_auto_release_eligible_holds(processing_date, None)
            .await?;

        let mut released_holds = Vec::new();
        
        for hold in eligible_holds {
            match self.account_repository
                .release_hold_detailed(
                    hold.id,
                    None, // Full release
                    Uuid::new_v4(), // System reason ID
                    Uuid::new_v4(), // System user ID
                    Utc::now(),
                )
                .await 
            {
                Ok(released_hold) => {
                    released_holds.push(AccountMapper::account_hold_from_model(released_hold));
                }
                Err(e) => {
                    tracing::error!("Failed to auto-release hold {}: {}", hold.id, e);
                }
            }
        }

        Ok(released_holds)
    }

    async fn bulk_place_holds(
        &self,
        account_ids: Vec<Uuid>,
        hold_type: HoldType,
        amount_per_account: Decimal,
        reason_id: Uuid,
        placed_by_person_id: Uuid,
        expires_at: Option<DateTime<Utc>>,
    ) -> BankingResult<Vec<AccountHold>> {
        tracing::info!("Bulk placing {} holds of type {:?} for {} accounts", 
                      amount_per_account, hold_type, account_ids.len());

        let mut holds = Vec::new();
        
        for account_id in account_ids {
            let hold = banking_db::models::AccountHoldModel {
                id: Uuid::new_v4(),
                account_id,
                amount: amount_per_account,
                hold_type: hold_type.clone(),
                reason_id,
                additional_details: None,
                placed_by_person_id,
                placed_at: Utc::now(),
                expires_at,
                status: HoldStatus::Active,
                released_at: None,
                released_by_person_id: None,
                priority: HoldPriority::Standard,
                source_reference: None,
                automatic_release: expires_at.is_some(),
            };
            
            holds.push(hold);
        }

        let created_models = self.account_repository
            .bulk_place_holds(holds)
            .await?;

        Ok(created_models.into_iter()
           .map(AccountMapper::account_hold_from_model)
           .collect())
    }

    async fn bulk_release_holds(
        &self,
        hold_ids: Vec<Uuid>,
        release_reason_id: Uuid,
        released_by_person_id: Uuid,
    ) -> BankingResult<Vec<AccountHold>> {
        tracing::info!("Bulk releasing {} holds", hold_ids.len());

        let released_models = self.account_repository
            .bulk_release_holds(hold_ids, release_reason_id, released_by_person_id)
            .await?;

        Ok(released_models.into_iter()
           .map(AccountMapper::account_hold_from_model)
           .collect())
    }

    // ============================================================================
    // PRIORITY AND AUTHORIZATION MANAGEMENT
    // ============================================================================

    async fn override_holds_for_transaction(
        &self,
        account_id: Uuid,
        transaction_amount: Decimal,
        override_priority: HoldPriority,
        authorized_by: Uuid,
        override_reason_id: Uuid,
    ) -> BankingResult<Vec<AccountHold>> {
        tracing::warn!("Overriding holds for transaction of {} on account {} by {}", 
                      transaction_amount, account_id, authorized_by);

        let overridden_holds = self.account_repository
            .get_overrideable_holds(account_id, transaction_amount, override_priority.to_string())
            .await?;

        let hold_ids: Vec<Uuid> = overridden_holds.iter().map(|h| h.id).collect();

        let _override_record = self.account_repository
            .create_hold_override(
                account_id,
                hold_ids,
                transaction_amount,
                authorized_by,
                override_reason_id,
            )
            .await?;

        Ok(overridden_holds.into_iter()
           .map(AccountMapper::account_hold_from_model)
           .collect())
    }

    async fn reorder_hold_priorities(
        &self,
        account_id: Uuid,
        hold_priority_map: Vec<(Uuid, HoldPriority)>,
        authorized_by: Uuid,
    ) -> BankingResult<Vec<AccountHold>> {
        tracing::info!("Reordering hold priorities for account {} by {}", 
                      account_id, authorized_by);

        let priority_updates: Vec<(Uuid, String)> = hold_priority_map.into_iter()
            .map(|(hold_id, priority)| (hold_id, priority.to_string()))
            .collect();

        let reordered_models = self.account_repository
            .update_hold_priorities(account_id, priority_updates, authorized_by)
            .await?;

        Ok(reordered_models.into_iter()
           .map(AccountMapper::account_hold_from_model)
           .collect())
    }

    async fn get_required_authorization_level(
        &self,
        hold_type: HoldType,
        amount: Decimal,
    ) -> BankingResult<HoldAuthorizationLevel> {
        // Business rules for authorization levels
        match hold_type {
            HoldType::JudicialLien => Ok(HoldAuthorizationLevel::External),
            HoldType::ComplianceHold => {
                if amount > Decimal::from(1000000) { // $1M threshold
                    Ok(HoldAuthorizationLevel::Executive)
                } else {
                    Ok(HoldAuthorizationLevel::Manager)
                }
            }
            HoldType::FraudHold => Ok(HoldAuthorizationLevel::Manager),
            HoldType::AdministrativeHold => {
                if amount > Decimal::from(100000) { // $100K threshold
                    Ok(HoldAuthorizationLevel::Supervisor)
                } else {
                    Ok(HoldAuthorizationLevel::Standard)
                }
            }
            _ => Ok(HoldAuthorizationLevel::Standard),
        }
    }

    // ============================================================================
    // INTEGRATION WITH EXTERNAL SYSTEMS
    // ============================================================================

    async fn sync_judicial_holds(
        &self,
        court_reference: String,
    ) -> BankingResult<Vec<AccountHold>> {
        tracing::info!("Syncing judicial holds for court reference {}", court_reference);

        let synced_models = self.account_repository
            .get_judicial_holds_by_reference(court_reference)
            .await?;

        Ok(synced_models.into_iter()
           .map(AccountMapper::account_hold_from_model)
           .collect())
    }

    async fn update_loan_pledge_holds(
        &self,
        loan_account_id: Uuid,
        collateral_account_ids: Vec<Uuid>,
        new_pledge_amount: Decimal,
    ) -> BankingResult<Vec<AccountHold>> {
        tracing::info!("Updating loan pledge holds for loan {} with amount {}", 
                      loan_account_id, new_pledge_amount);

        let updated_models = self.account_repository
            .update_loan_pledge_holds(
                loan_account_id,
                collateral_account_ids,
                new_pledge_amount,
                Uuid::new_v4(), // System user ID - should be configurable
            )
            .await?;

        Ok(updated_models.into_iter()
           .map(AccountMapper::account_hold_from_model)
           .collect())
    }

    async fn process_compliance_holds(
        &self,
        compliance_alert_id: Uuid,
        affected_accounts: Vec<Uuid>,
        _hold_amount_per_account: Decimal,
    ) -> BankingResult<Vec<AccountHold>> {
        tracing::warn!("Processing compliance holds for alert {} affecting {} accounts", 
                      compliance_alert_id, affected_accounts.len());

        let created_models = self.account_repository
            .get_compliance_holds_by_alert(compliance_alert_id)
            .await?;

        Ok(created_models.into_iter()
           .map(AccountMapper::account_hold_from_model)
           .collect())
    }

    // ============================================================================
    // REPORTING AND ANALYTICS
    // ============================================================================

    async fn get_hold_analytics(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
        hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<HoldAnalytics> {
        let analytics_data = self.account_repository
            .get_hold_analytics(
                from_date,
                to_date,
                hold_types.map(|types| types.into_iter().map(|t| t.to_string()).collect()),
            )
            .await?;

        // Convert repository analytics to service analytics
        Ok(HoldAnalytics {
            total_hold_amount: analytics_data.total_hold_amount,
            active_hold_count: analytics_data.active_hold_count,
            expired_hold_count: analytics_data.expired_hold_count,
            released_hold_count: analytics_data.released_hold_count,
            average_hold_duration_days: analytics_data.average_hold_duration_days,
            hold_by_type: std::collections::HashMap::new(), // TODO: Convert from repository format
            hold_by_priority: std::collections::HashMap::new(), // TODO: Convert from repository format
            top_hold_accounts: Vec::new(), // TODO: Extract from repository data
        })
    }

    async fn get_high_hold_ratio_accounts(
        &self,
        minimum_ratio: Decimal,
        exclude_hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<Vec<HighHoldAccount>> {
        let high_ratio_accounts = self.account_repository
            .get_high_hold_ratio_accounts(
                minimum_ratio,
                exclude_hold_types.map(|types| types.into_iter().map(|t| t.to_string()).collect()),
                100
            )
            .await?;

        // Convert repository models to service models
        Ok(high_ratio_accounts.into_iter().map(|account| HighHoldAccount {
            account_id: account.account_id,
            current_balance: account.current_balance,
            total_holds: account.total_holds,
            hold_ratio: account.hold_ratio,
            active_hold_count: account.active_hold_count,
            high_priority_holds: account.critical_priority_holds,
            last_assessment_date: Utc::now(),
        }).collect())
    }

    async fn generate_judicial_hold_report(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> BankingResult<JudicialHoldReport> {
        let judicial_report_data = self.account_repository
            .generate_judicial_hold_report(from_date, to_date)
            .await?;

        Ok(JudicialHoldReport {
            total_judicial_holds: judicial_report_data.total_judicial_holds,
            total_amount: judicial_report_data.total_amount,
            active_holds: judicial_report_data.active_holds.into_iter()
                .map(AccountMapper::account_hold_from_model)
                .collect(),
            released_holds: judicial_report_data.released_holds.into_iter()
                .map(AccountMapper::account_hold_from_model)
                .collect(),
            expired_holds: judicial_report_data.expired_holds.into_iter()
                .map(AccountMapper::account_hold_from_model)
                .collect(),
            report_period: (from_date, to_date),
            generated_at: Utc::now(),
        })
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
        // TODO: Verify the person ID exists in persons table
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
                account_id: account.id,
            }),
        }
    }

    /// Create a hold on the account
    async fn create_account_hold(&self, _account_id: Uuid, _amount: Decimal, _reason: &str) -> BankingResult<()> {
        // In production, this would create a record in the holds repository
        // For now, we'll just return success
        Ok(())
    }

    /// Create hold breakdown summary by type and priority
    fn create_hold_breakdown(&self, holds: &[banking_db::models::AccountHoldModel]) -> Vec<HoldSummary> {
        use std::collections::HashMap;

        let mut breakdown: HashMap<(HoldType, HoldPriority), (u32, Decimal)> = HashMap::new();

        for hold in holds {
            let key = (hold.hold_type.clone(), hold.priority.clone());
            let entry = breakdown.entry(key).or_insert((0, Decimal::ZERO));
            entry.0 += 1;
            entry.1 += hold.amount;
        }

        breakdown.into_iter()
            .map(|((hold_type, priority), (count, amount))| HoldSummary {
                hold_type,
                total_amount: amount,
                hold_count: count,
                priority,
            })
            .collect()
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
            id: Uuid::new_v4(),
            product_code: HeaplessString::try_from("SAV001").unwrap(),
            account_type: AccountType::Savings,
            account_status: AccountStatus::Active,
            signing_condition: SigningCondition::None,
            currency: HeaplessString::try_from("USD").unwrap(),
            open_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            domicile_agency_branch_id: Uuid::new_v4(),
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
            last_disbursement_instruction_id: None,
            status_changed_by_person_id: None,
            status_change_reason_id: None,
            status_change_timestamp: None,
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
            updated_by_person_id: Uuid::new_v4(), // Changed to UUID for Person.person_id
        }
    }

    // Balance Calculation Tests
    
    #[tokio::test]
    async fn test_calculate_balance_savings_account() {
        let account_id = Uuid::new_v4();
        let mut account_model = create_test_account_model(AccountType::Savings, Decimal::new(50000, 2), None);
        account_model.id = account_id;
        
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
        account_model.id = account_id;
        
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
        account_model.id = account_id;
        
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
        account_model.id = account_id;
        
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
        account_model.id = account_id;
        
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
        account_model.id = account_id;
        
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
        account_model.id = account_id;
        
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
        account_model.id = account_id;
        
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
        account_model.id = account_id;
        
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
        account_model.id = account_id;
        
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
            id: Uuid::new_v4(),
            product_code: HeaplessString::try_from("TEST001").unwrap(),
            account_type,
            account_status: AccountStatus::Active,
            signing_condition: SigningCondition::None,
            currency: HeaplessString::try_from("USD").unwrap(),
            open_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            domicile_agency_branch_id: Uuid::new_v4(),
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
            last_disbursement_instruction_id: None,
            status_changed_by_person_id: None,
            status_change_reason_id: None,
            status_change_timestamp: None,
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
            updated_by_person_id: Uuid::new_v4(), // Changed to UUID for Person.person_id
        }
    }

    fn create_test_loan_account_model(original_principal: Decimal, outstanding_principal: Decimal, current_balance: Decimal) -> banking_db::models::AccountModel {
        banking_db::models::AccountModel {
            id: Uuid::new_v4(),
            product_code: HeaplessString::try_from("LOAN001").unwrap(),
            account_type: AccountType::Loan,
            account_status: AccountStatus::Active,
            signing_condition: SigningCondition::None,
            currency: HeaplessString::try_from("USD").unwrap(),
            open_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            domicile_agency_branch_id: Uuid::new_v4(),
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
            collateral_id: Some(Uuid::new_v4()),
            loan_purpose_id: None,  // Changed to UUID reference
            close_date: None,
            last_activity_date: Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
            dormancy_threshold_days: None,
            reactivation_required: false,
            pending_closure_reason_id: None,
            last_disbursement_instruction_id: None,
            status_changed_by_person_id: None,
            status_change_reason_id: None,
            status_change_timestamp: None,
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
            updated_by_person_id: Uuid::new_v4(), // Changed to UUID for Person.person_id
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
            self.accounts.lock().unwrap().insert(account.id, account);
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

        // Hold methods - Mock implementations
        async fn update_hold(&self, hold: banking_db::models::AccountHoldModel) -> BankingResult<banking_db::models::AccountHoldModel> {
            Ok(hold)
        }

        async fn get_hold_by_id(&self, _hold_id: Uuid) -> BankingResult<Option<banking_db::models::AccountHoldModel>> {
            Ok(None)
        }

        async fn get_active_holds_for_account(&self, _account_id: Uuid, _hold_types: Option<Vec<String>>) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> {
            Ok(vec![])
        }

        async fn get_holds_by_status(&self, _account_id: Option<Uuid>, _status: String, _from_date: Option<chrono::NaiveDate>, _to_date: Option<chrono::NaiveDate>) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> {
            Ok(vec![])
        }

        async fn get_holds_by_type(&self, _hold_type: String, _status: Option<String>, _account_ids: Option<Vec<Uuid>>, _limit: Option<i32>) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> {
            Ok(vec![])
        }

        async fn get_hold_history(&self, _account_id: Uuid, _from_date: Option<chrono::NaiveDate>, _to_date: Option<chrono::NaiveDate>, _include_released: bool) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> {
            Ok(vec![])
        }

        async fn calculate_total_holds(&self, _account_id: Uuid, _exclude_hold_types: Option<Vec<String>>) -> BankingResult<rust_decimal::Decimal> {
            Ok(rust_decimal::Decimal::ZERO)
        }

        async fn get_hold_amounts_by_priority(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::repository::HoldPrioritySummary>> {
            Ok(vec![])
        }

        async fn get_hold_breakdown(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::repository::HoldTypeSummary>> {
            Ok(vec![])
        }

        async fn cache_balance_calculation(&self, calculation: banking_db::models::AccountBalanceCalculationModel) -> BankingResult<banking_db::models::AccountBalanceCalculationModel> {
            Ok(calculation)
        }

        async fn get_cached_balance_calculation(&self, _account_id: Uuid, _max_age_seconds: u64) -> BankingResult<Option<banking_db::models::AccountBalanceCalculationModel>> {
            Ok(None)
        }

        async fn release_hold_detailed(&self, _hold_id: Uuid, _release_amount: Option<rust_decimal::Decimal>, _release_reason_id: Uuid, _released_by: Uuid, _released_at: chrono::DateTime<chrono::Utc>) -> BankingResult<banking_db::models::AccountHoldModel> {
            todo!()
        }

        async fn create_hold_release_record(&self, release_record: banking_db::models::HoldReleaseRecordModel) -> BankingResult<banking_db::models::HoldReleaseRecordModel> {
            Ok(release_record)
        }

        async fn get_hold_release_records(&self, _hold_id: Uuid) -> BankingResult<Vec<banking_db::models::HoldReleaseRecordModel>> {
            Ok(vec![])
        }

        async fn bulk_release_holds(&self, _hold_ids: Vec<Uuid>, _release_reason_id: Uuid, _released_by: Uuid) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> {
            Ok(vec![])
        }

        async fn get_expired_holds(&self, _cutoff_date: chrono::DateTime<chrono::Utc>, _hold_types: Option<Vec<String>>, _limit: Option<i32>) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> {
            Ok(vec![])
        }

        async fn get_auto_release_eligible_holds(&self, _processing_date: chrono::NaiveDate, _hold_types: Option<Vec<String>>) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> {
            Ok(vec![])
        }

        async fn create_hold_expiry_job(&self, job: banking_db::models::AccountHoldExpiryJobModel) -> BankingResult<banking_db::models::AccountHoldExpiryJobModel> {
            Ok(job)
        }

        async fn update_hold_expiry_job(&self, job: banking_db::models::AccountHoldExpiryJobModel) -> BankingResult<banking_db::models::AccountHoldExpiryJobModel> {
            Ok(job)
        }

        async fn bulk_place_holds(&self, holds: Vec<banking_db::models::AccountHoldModel>) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> {
            Ok(holds)
        }

        async fn update_hold_priorities(&self, _account_id: Uuid, _hold_priority_updates: Vec<(Uuid, String)>, _updated_by_person_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> {
            Ok(vec![])
        }

        async fn get_overrideable_holds(&self, _account_id: Uuid, _required_amount: rust_decimal::Decimal, _override_priority: String) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> {
            Ok(vec![])
        }

        async fn create_hold_override(&self, _account_id: Uuid, _overridden_holds: Vec<Uuid>, _override_amount: rust_decimal::Decimal, _authorized_by: Uuid, _override_reason_id: Uuid) -> BankingResult<banking_db::repository::HoldOverrideRecord> {
            todo!()
        }

        async fn get_judicial_holds_by_reference(&self, _court_reference: String) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> {
            Ok(vec![])
        }

        async fn update_loan_pledge_holds(&self, _loan_id: Uuid, _account_ids: Vec<Uuid>, _new_amount: rust_decimal::Decimal, _updated_by_person_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> {
            Ok(vec![])
        }

        async fn get_compliance_holds_by_alert(&self, _alert_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> {
            Ok(vec![])
        }

        async fn get_hold_analytics(&self, _from_date: chrono::NaiveDate, _to_date: chrono::NaiveDate, _hold_types: Option<Vec<String>>) -> BankingResult<banking_db::repository::HoldAnalyticsSummary> {
            todo!()
        }

        async fn get_high_hold_ratio_accounts(&self, _min_ratio: rust_decimal::Decimal, _exclude_hold_types: Option<Vec<String>>, _limit: i32) -> BankingResult<Vec<banking_db::repository::HighHoldRatioAccount>> {
            Ok(vec![])
        }

        async fn generate_judicial_hold_report(&self, _from_date: chrono::NaiveDate, _to_date: chrono::NaiveDate) -> BankingResult<banking_db::repository::JudicialHoldReportData> {
            todo!()
        }

        async fn get_hold_aging_report(&self, _hold_types: Option<Vec<String>>, _age_buckets: Vec<i32>) -> BankingResult<Vec<banking_db::repository::HoldAgingBucket>> {
            Ok(vec![])
        }

        async fn validate_hold_amounts(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::repository::HoldValidationError>> {
            Ok(vec![])
        }

        async fn find_orphaned_holds(&self, _limit: Option<i32>) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> {
            Ok(vec![])
        }

        async fn cleanup_old_holds(&self, _cutoff_date: chrono::NaiveDate, _hold_statuses: Vec<String>) -> BankingResult<u32> {
            Ok(0)
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
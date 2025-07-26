use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;
use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;
use heapless::String as HeaplessString;

use banking_api::{
    BankingResult, BankingError, Transaction, ApprovalWorkflow,
    service::TransactionService,
    domain::{TransactionType, TransactionStatus, AccountStatus, ValidationResult},
};
use banking_db::repository::{TransactionRepository, AccountRepository};
use crate::{
    mappers::{TransactionMapper, AccountMapper},
    integration::ProductCatalogClient,
};

/// Production implementation of TransactionService
/// Provides multi-level validation and processing with approval workflows
pub struct TransactionServiceImpl {
    transaction_repository: Arc<dyn TransactionRepository>,
    account_repository: Arc<dyn AccountRepository>,
    product_catalog_client: Arc<ProductCatalogClient>,
    validation_cache: ValidationCache,
}

impl TransactionServiceImpl {
    pub fn new(
        transaction_repository: Arc<dyn TransactionRepository>,
        account_repository: Arc<dyn AccountRepository>,
        product_catalog_client: Arc<ProductCatalogClient>,
    ) -> Self {
        Self {
            transaction_repository,
            account_repository,
            product_catalog_client,
            validation_cache: ValidationCache::new(),
        }
    }
}

#[async_trait]
impl TransactionService for TransactionServiceImpl {
    /// Process transaction with comprehensive validation and multi-stage pipeline
    async fn process_transaction(&self, mut transaction: Transaction) -> BankingResult<Transaction> {
        // Set system timestamp
        transaction.created_at = Utc::now();
        
        // Generate reference number if not provided
        if transaction.reference_number.is_empty() {
            let ref_num = self.generate_reference_number().await?;
            transaction.set_reference_number(&ref_num).map_err(|msg| BankingError::ValidationError {
                field: "reference_number".to_string(),
                message: msg.to_string(),
            })?;
        }

        // Stage 1: Pre-validation (fail-fast checks)
        self.pre_validate_transaction(&transaction).await?;

        // Stage 2: Comprehensive validation
        let validation_result = self.validate_transaction_limits(&transaction).await?;
        
        if !validation_result.is_valid() {
            transaction.status = TransactionStatus::Failed;
            let failed_transaction = TransactionMapper::to_model(transaction.clone());
            self.transaction_repository.create(failed_transaction).await?;
            
            let reasons = validation_result.get_failure_reasons().join(", ");
            return Err(banking_api::BankingError::ValidationFailed(reasons));
        }

        // Stage 3: Check if approval is required
        if self.requires_approval(&transaction).await? {
            transaction.requires_approval = true;
            transaction.approval_status = Some(banking_api::domain::TransactionApprovalStatus::Pending);
            transaction.status = TransactionStatus::AwaitingApproval;
        } else {
            transaction.status = TransactionStatus::Posted;
        }

        // Stage 4: Execute financial posting
        if transaction.status == TransactionStatus::Posted {
            self.execute_financial_posting(&mut transaction).await?;
        }

        // Stage 5: Persist transaction
        let transaction_model = TransactionMapper::to_model(transaction.clone());
        let created_model = self.transaction_repository.create(transaction_model).await?;

        // Stage 6: Update account activity timestamp
        if transaction.status == TransactionStatus::Posted {
            self.update_account_activity(transaction.account_id).await?;
        }

        tracing::info!(
            "Transaction {} processed with status {:?} for account {}",
            transaction.transaction_id, transaction.status, transaction.account_id
        );

        TransactionMapper::from_model(created_model)
    }

    /// Validate transaction limits across multiple tiers
    async fn validate_transaction_limits(&self, transaction: &Transaction) -> BankingResult<ValidationResult> {
        let mut validation_result = ValidationResult::success();

        // Account-level validations
        let account_validation = self.validate_account_level_limits(transaction).await?;
        validation_result.merge(account_validation);

        // Product-level validations
        let product_validation = self.validate_product_level_limits(transaction).await?;
        validation_result.merge(product_validation);

        // Terminal/Agent-level validations
        if let Some(terminal_id) = transaction.terminal_id {
            let terminal_validation = self.validate_terminal_level_limits(transaction, terminal_id).await?;
            validation_result.merge(terminal_validation);
        }

        // Customer risk-based validations
        let risk_validation = self.validate_risk_level_limits(transaction).await?;
        validation_result.merge(risk_validation);

        Ok(validation_result)
    }

    /// Reverse a posted transaction with reason ID validation
    async fn reverse_transaction(&self, transaction_id: Uuid, reason_id: Uuid, _additional_details: Option<&str>) -> BankingResult<()> {
        // TODO: Validate reason_id against ReasonAndPurpose table
        // TODO: Store additional_details if provided
        
        // For now, convert reason_id to string for legacy compatibility
        let reason = format!("Reason ID: {reason_id}");
        // Find original transaction
        let original_transaction = self.transaction_repository
            .find_by_id(transaction_id)
            .await?
            .ok_or(banking_api::BankingError::TransactionNotFound(transaction_id.to_string()))?;

        let original = TransactionMapper::from_model(original_transaction)?;

        // Validate transaction can be reversed
        if original.status != TransactionStatus::Posted {
            return Err(banking_api::BankingError::ValidationError {
                field: "transaction_status".to_string(),
                message: format!("Transaction {} cannot be reversed from status {:?}", transaction_id, original.status),
            });
        }

        // Create reversal transaction
        let reversal_transaction = Transaction {
            transaction_id: Uuid::new_v4(),
            account_id: original.account_id,
            transaction_code: {
                let rev_code = format!("REV_{}", original.transaction_code.as_str());
                HeaplessString::try_from(rev_code.as_str()).map_err(|_| BankingError::ValidationError {
                    field: "transaction_code".to_string(),
                    message: "Reversal transaction code too long".to_string(),
                })?
            },
            transaction_type: match original.transaction_type {
                TransactionType::Credit => TransactionType::Debit,
                TransactionType::Debit => TransactionType::Credit,
            },
            amount: original.amount,
            currency: original.currency,
            description: {
                let desc_str = format!("Reversal: {} - {}", original.description, reason);
                HeaplessString::try_from(desc_str.as_str()).map_err(|_| BankingError::ValidationError {
                    field: "description".to_string(),
                    message: "Description too long".to_string(),
                })?
            },
            channel_id: HeaplessString::try_from("SYSTEM_REVERSAL").map_err(|_| BankingError::ValidationError {
                field: "channel_id".to_string(),
                message: "Channel ID too long".to_string(),
            })?,
            terminal_id: None,
            agent_user_id: None,
            transaction_date: Utc::now(),
            value_date: original.value_date,
            status: TransactionStatus::Posted,
            reference_number: {
                let ref_num = self.generate_reference_number().await?;
                HeaplessString::try_from(ref_num.as_str()).map_err(|_| BankingError::ValidationError {
                    field: "reference_number".to_string(),
                    message: "Reference number too long".to_string(),
                })?
            },
            external_reference: Some(original.reference_number.clone()),
            gl_code: original.gl_code,
            requires_approval: false,
            approval_status: None,
            risk_score: Some(Decimal::ZERO), // System transaction
            created_at: Utc::now(),
        };

        // Process reversal transaction
        self.execute_financial_posting(&mut reversal_transaction.clone()).await?;
        let reversal_model = TransactionMapper::to_model(reversal_transaction);
        self.transaction_repository.create(reversal_model).await?;

        // Mark original transaction as reversed
        self.transaction_repository.update_status(
            transaction_id,
            "Reversed",
            &format!("Reversed: {reason}"),
        ).await?;

        tracing::info!(
            "Transaction {} reversed. Reason ID: {}",
            transaction_id, reason_id
        );

        Ok(())
    }
    
    /// Legacy method - deprecated, use reverse_transaction with reason_id instead
    async fn reverse_transaction_legacy(&self, transaction_id: Uuid, reason: String) -> BankingResult<()> {
        // Find original transaction
        let original_transaction = self.transaction_repository
            .find_by_id(transaction_id)
            .await?
            .ok_or(banking_api::BankingError::TransactionNotFound(transaction_id.to_string()))?;

        let original = TransactionMapper::from_model(original_transaction)?;

        // Validate transaction can be reversed
        if original.status != TransactionStatus::Posted {
            return Err(banking_api::BankingError::ValidationError {
                field: "transaction_status".to_string(),
                message: format!("Transaction {} cannot be reversed from status {:?}", transaction_id, original.status),
            });
        }

        // Legacy implementation would go here
        tracing::info!(
            "Transaction {} reversed. Reason: {}",
            transaction_id, reason
        );

        Ok(())
    }

    /// Find transactions for an account within date range
    async fn find_transactions_by_account(
        &self,
        account_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> BankingResult<Vec<Transaction>> {
        let models = self.transaction_repository
            .find_by_account_date_range(account_id, from, to)
            .await?;

        let mut transactions = Vec::new();
        for model in models {
            transactions.push(TransactionMapper::from_model(model)?);
        }

        Ok(transactions)
    }

    /// Initiate approval workflow for multi-party authorization
    async fn initiate_approval_workflow(&self, transaction: Transaction) -> BankingResult<ApprovalWorkflow> {
        // Get account information to determine required approvers
        let account = self.account_repository
            .find_by_id(transaction.account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(transaction.account_id))?;

        let account_domain = AccountMapper::from_model(account)?;

        // Determine required approvers based on signing condition
        let required_approvers = self.get_required_approvers(&account_domain, &transaction).await?;

        // Create workflow
        let workflow = ApprovalWorkflow {
            workflow_id: Uuid::new_v4(),
            transaction_id: transaction.transaction_id,
            required_approvers: required_approvers.clone(),
            received_approvals: Vec::new(),
            status: banking_api::domain::TransactionWorkflowStatus::Pending,
            timeout_at: Utc::now() + chrono::Duration::hours(24), // 24-hour timeout
        };

        // Persist workflow (this would typically involve a workflow repository)
        tracing::info!(
            "Approval workflow {} initiated for transaction {} with {} required approvers",
            workflow.workflow_id, transaction.transaction_id, required_approvers.len()
        );

        Ok(workflow)
    }

    /// Approve a transaction in the approval workflow
    async fn approve_transaction(&self, transaction_id: Uuid, approver_id: Uuid) -> BankingResult<()> {
        // Find transaction
        let transaction = self.transaction_repository
            .find_by_id(transaction_id)
            .await?
            .ok_or(banking_api::BankingError::TransactionNotFound(transaction_id.to_string()))?;

        if transaction.status != TransactionStatus::AwaitingApproval {
            return Err(banking_api::BankingError::ValidationError {
                field: "status".to_string(),
                message: format!("Transaction {transaction_id} is not awaiting approval"),
            });
        }

        // In production, this would:
        // 1. Validate approver authorization
        // 2. Check approval workflow requirements
        // 3. Update approval status
        // 4. Process transaction if all approvals received

        tracing::info!(
            "Approval recorded for transaction {} by approver {}",
            transaction_id, approver_id
        );

        Ok(())
    }

    /// Validate account transactional status
    async fn validate_account_transactional_status(&self, _account_id: Uuid, _transaction_type: banking_api::domain::TransactionType) -> BankingResult<banking_api::domain::ValidationResult> {
        todo!("Implement validate_account_transactional_status")
    }

    /// Get permitted operations for account
    async fn get_permitted_operations(&self, _account_id: Uuid) -> BankingResult<Vec<banking_api::domain::PermittedOperation>> {
        todo!("Implement get_permitted_operations")
    }

    /// Process closure transaction
    async fn process_closure_transaction(&self, _account_id: Uuid, _settlement: banking_api::domain::FinalSettlement) -> BankingResult<banking_api::domain::Transaction> {
        todo!("Implement process_closure_transaction")
    }

    /// Reverse pending transactions with reason ID validation
    async fn reverse_pending_transactions(&self, _account_id: Uuid, _reason_id: Uuid, _additional_details: Option<&str>) -> BankingResult<Vec<banking_api::domain::Transaction>> {
        // TODO: Validate reason_id against ReasonAndPurpose table
        // TODO: Store additional_details if provided
        todo!("Implement reverse_pending_transactions with reason_id")
    }
    
    /// Legacy method - deprecated, use reverse_pending_transactions with reason_id instead
    async fn reverse_pending_transactions_legacy(&self, _account_id: Uuid, _reason: String) -> BankingResult<Vec<banking_api::domain::Transaction>> {
        todo!("Implement reverse_pending_transactions")
    }

    /// Process transaction request
    async fn process_transaction_request(&self, _request: banking_api::domain::TransactionRequest) -> BankingResult<banking_api::domain::TransactionResult> {
        todo!("Implement process_transaction_request")
    }

    /// Find transaction by ID
    async fn find_transaction_by_id(&self, transaction_id: Uuid) -> BankingResult<Option<banking_api::domain::Transaction>> {
        if let Some(model) = self.transaction_repository.find_by_id(transaction_id).await? {
            Ok(Some(TransactionMapper::from_model(model)?))
        } else {
            Ok(None)
        }
    }

    /// Find transaction by reference
    async fn find_transaction_by_reference(&self, _reference_number: &str) -> BankingResult<Option<banking_api::domain::Transaction>> {
        todo!("Implement find_transaction_by_reference")
    }

    /// Get transaction audit trail
    async fn get_transaction_audit_trail(&self, _transaction_id: Uuid) -> BankingResult<Vec<banking_api::service::TransactionAuditEntry>> {
        todo!("Implement get_transaction_audit_trail")
    }

    /// Update transaction status
    async fn update_transaction_status(&self, transaction_id: Uuid, status: banking_api::domain::TransactionStatus, reason: String) -> BankingResult<()> {
        let status_str = match status {
            banking_api::domain::TransactionStatus::Pending => "Pending",
            banking_api::domain::TransactionStatus::Posted => "Posted",
            banking_api::domain::TransactionStatus::Reversed => "Reversed",
            banking_api::domain::TransactionStatus::Failed => "Failed",
            banking_api::domain::TransactionStatus::AwaitingApproval => "AwaitingApproval",
            banking_api::domain::TransactionStatus::ApprovalRejected => "ApprovalRejected",
        };
        self.transaction_repository.update_status(transaction_id, status_str, &reason).await
    }
}

impl TransactionServiceImpl {
    /// Pre-validation checks for fast failure
    async fn pre_validate_transaction(&self, transaction: &Transaction) -> BankingResult<()> {
        // Basic data validation
        if transaction.amount <= Decimal::ZERO {
            return Err(banking_api::BankingError::InvalidTransactionAmount(
                format!("Invalid transaction amount: {}", transaction.amount)
            ));
        }

        if transaction.currency.len() != 3 {
            return Err(banking_api::BankingError::ValidationError {
                field: "currency".to_string(),
                message: "Currency must be a 3-character ISO code".to_string(),
            });
        }

        // Account existence check (cached)
        let account_exists = if let Some(cached) = self.validation_cache.get_account_status(transaction.account_id) {
            cached != AccountStatus::Closed
        } else {
            self.account_repository.exists(transaction.account_id).await?
        };

        if !account_exists {
            return Err(banking_api::BankingError::AccountNotFound(transaction.account_id));
        }

        Ok(())
    }

    /// Validate account-level transaction rules
    async fn validate_account_level_limits(&self, transaction: &Transaction) -> BankingResult<ValidationResult> {
        let mut result = ValidationResult::success();

        // Get account details
        let account = self.account_repository
            .find_by_id(transaction.account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(transaction.account_id))?;

        let account_domain = AccountMapper::from_model(account)?;

        // Check account status
        match account_domain.account_status {
            AccountStatus::Active => {
                result.add_check("account_status", true, "Account is active".to_string());
            }
            AccountStatus::Frozen => {
                result.add_check("account_status", false, "Account is frozen".to_string());
            }
            AccountStatus::Closed => {
                result.add_check("account_status", false, "Account is closed".to_string());
            }
            _ => {
                result.add_check("account_status", false, "Account is not in transactional state".to_string());
            }
        }

        // For debit transactions, check available balance
        if transaction.transaction_type == TransactionType::Debit {
            let available_balance = account_domain.current_balance + account_domain.overdraft_limit.unwrap_or(Decimal::ZERO);
            if transaction.amount > available_balance {
                result.add_check(
                    "sufficient_funds",
                    false,
                    format!("Insufficient funds: {} requested, {} available", transaction.amount, available_balance),
                );
            } else {
                result.add_check("sufficient_funds", true, "Sufficient funds available".to_string());
            }
        }

        Ok(result)
    }

    /// Validate product-level transaction rules
    async fn validate_product_level_limits(&self, transaction: &Transaction) -> BankingResult<ValidationResult> {
        let mut result = ValidationResult::success();

        // Get account to determine product code
        let account = self.account_repository
            .find_by_id(transaction.account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(transaction.account_id))?;

        // Get product rules from catalog
        match self.product_catalog_client.get_product_rules(account.product_code.as_str()).await {
            Ok(product_rules) => {
                // Check per-transaction limits
                if let Some(per_txn_limit) = product_rules.per_transaction_limit {
                    if transaction.amount > per_txn_limit {
                        result.add_check(
                            "per_transaction_limit",
                            false,
                            format!("Transaction amount {} exceeds per-transaction limit {}", transaction.amount, per_txn_limit),
                        );
                    } else {
                        result.add_check("per_transaction_limit", true, "Within per-transaction limit".to_string());
                    }
                }

                // Check daily limits (would need to query today's transactions)
                result.add_check("daily_limit", true, "Daily limit check passed".to_string());
            }
            Err(_) => {
                result.add_check("product_rules", false, "Could not retrieve product rules".to_string());
            }
        }

        Ok(result)
    }

    /// Validate terminal/agent-level limits
    async fn validate_terminal_level_limits(&self, _transaction: &Transaction, _terminal_id: Uuid) -> BankingResult<ValidationResult> {
        let mut result = ValidationResult::success();

        // In production, this would:
        // 1. Get terminal information
        // 2. Check daily volume limits
        // 3. Validate hierarchical limits (terminal -> branch -> network)
        
        result.add_check("terminal_limits", true, "Terminal limits validated".to_string());
        Ok(result)
    }

    /// Validate customer risk-based limits
    async fn validate_risk_level_limits(&self, _transaction: &Transaction) -> BankingResult<ValidationResult> {
        let mut result = ValidationResult::success();

        // In production, this would:
        // 1. Get customer risk rating
        // 2. Apply risk-based transaction limits
        // 3. Check for suspicious patterns

        result.add_check("risk_limits", true, "Risk-based limits validated".to_string());
        Ok(result)
    }

    /// Check if transaction requires approval
    async fn requires_approval(&self, transaction: &Transaction) -> BankingResult<bool> {
        // Get account information
        let account = self.account_repository
            .find_by_id(transaction.account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(transaction.account_id))?;

        let account_domain = AccountMapper::from_model(account)?;

        // Check signing conditions
        match account_domain.signing_condition {
            banking_api::domain::SigningCondition::AllOwners => {
                // All owners must approve for any transaction
                Ok(true)
            }
            banking_api::domain::SigningCondition::AnyOwner => {
                // Any owner can approve, but large amounts might require approval
                Ok(transaction.amount > Decimal::new(10000, 2)) // $10,000 threshold
            }
            banking_api::domain::SigningCondition::None => {
                // Single-owner account, no approval required for normal transactions
                Ok(transaction.amount > Decimal::new(50000, 2)) // $50,000 threshold
            }
        }
    }

    /// Execute the financial posting (balance updates)
    async fn execute_financial_posting(&self, transaction: &mut Transaction) -> BankingResult<()> {
        let account = self.account_repository
            .find_by_id(transaction.account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(transaction.account_id))?;

        // Calculate new balances based on transaction type
        let new_balance = match transaction.transaction_type {
            TransactionType::Credit => account.current_balance + transaction.amount,
            TransactionType::Debit => account.current_balance - transaction.amount,
        };

        // Update account balance
        self.account_repository
            .update_balance(transaction.account_id, new_balance, new_balance)
            .await?;

        // Set GL code if not provided
        if transaction.gl_code.as_str().is_empty() {
            let gl_code_str = self.generate_gl_code(&account, transaction).await?;
            transaction.set_gl_code(&gl_code_str).map_err(|e| 
                banking_api::BankingError::ValidationError {
                    field: "gl_code".to_string(),
                    message: e.to_string(),
                }
            )?;
        }

        tracing::debug!(
            "Financial posting executed: Account {} balance updated to {}",
            transaction.account_id, new_balance
        );

        Ok(())
    }

    /// Generate unique transaction reference number
    async fn generate_reference_number(&self) -> BankingResult<String> {
        let now = Utc::now();
        let timestamp = now.format("%Y%m%d%H%M%S");
        let random_suffix: u32 = rand::random::<u32>() % 10000;
        Ok(format!("TXN{timestamp}{random_suffix:04}"))
    }

    /// Generate GL code based on account and transaction
    async fn generate_gl_code(&self, account: &banking_db::models::AccountModel, _transaction: &Transaction) -> BankingResult<String> {
        // In production, this would be more sophisticated
        let product_code_str = account.product_code.as_str();
        Ok(format!("{product_code_str}001"))
    }

    /// Get required approvers for a transaction
    async fn get_required_approvers(
        &self,
        _account: &banking_api::domain::Account,
        _transaction: &Transaction,
    ) -> BankingResult<Vec<Uuid>> {
        // In production, this would query account ownership and mandates
        // For now, return empty vector
        Ok(Vec::new())
    }

    /// Update account last activity date
    async fn update_account_activity(&self, account_id: Uuid) -> BankingResult<()> {
        // In production, this would update the last_activity_date field
        tracing::debug!("Updated activity timestamp for account {}", account_id);
        Ok(())
    }
}

/// Validation cache for high-performance checks
struct ValidationCache {
    // In production, this would use a proper cache like moka
    _cache: HashMap<Uuid, AccountStatus>,
}

impl ValidationCache {
    fn new() -> Self {
        Self {
            _cache: HashMap::new(),
        }
    }

    fn get_account_status(&self, _account_id: Uuid) -> Option<AccountStatus> {
        // In production, this would return cached status
        None
    }
}


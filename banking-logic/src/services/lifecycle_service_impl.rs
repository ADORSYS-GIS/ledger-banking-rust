use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

use banking_api::{
    BankingResult,
    service::{AccountLifecycleService, CalendarService},
    domain::{
        AccountWorkflow, WorkflowType, WorkflowStep, WorkflowStatus,
        AccountOpeningRequest, ClosureRequest, DormancyAssessment,
        FinalSettlement, AccountStatus, KycResult, StatusChangeRecord,
    },
};
use banking_db::repository::{AccountRepository, WorkflowRepository};
use crate::{mappers::AccountMapper, integration::ProductCatalogClient};

/// Production implementation of AccountLifecycleService
/// Handles comprehensive account lifecycle management with workflow orchestration
pub struct AccountLifecycleServiceImpl {
    account_repository: Arc<dyn AccountRepository>,
    workflow_repository: Arc<dyn WorkflowRepository>,
    product_catalog_client: Arc<ProductCatalogClient>,
    #[allow(dead_code)]
    calendar_service: Arc<dyn CalendarService>,
}

impl AccountLifecycleServiceImpl {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
        workflow_repository: Arc<dyn WorkflowRepository>,
        product_catalog_client: Arc<ProductCatalogClient>,
        calendar_service: Arc<dyn CalendarService>,
    ) -> Self {
        Self {
            account_repository,
            workflow_repository,
            product_catalog_client,
            calendar_service,
        }
    }
}

#[async_trait]
impl AccountLifecycleService for AccountLifecycleServiceImpl {
    /// Initiate account opening workflow with comprehensive validation
    async fn initiate_account_opening(&self, request: AccountOpeningRequest) -> BankingResult<AccountWorkflow> {
        // Validate product eligibility
        self.validate_product_eligibility(&request).await?;

        // Create workflow
        let workflow = AccountWorkflow {
            workflow_id: Uuid::new_v4(),
            account_id: Uuid::new_v4(), // This will be the future account ID
            workflow_type: WorkflowType::AccountOpening,
            current_step: WorkflowStep::InitiateRequest,
            status: WorkflowStatus::InProgress,
            initiated_by: request.initiated_by.clone(),
            initiated_at: Utc::now(),
            completed_at: None,
            steps_completed: Vec::new(),
            next_action_required: Some("KYC verification required".to_string()),
            timeout_at: Some(Utc::now() + chrono::Duration::days(30)), // 30-day timeout
        };

        // Convert to model and persist workflow
        let workflow_model = self.to_workflow_model(&workflow);
        self.workflow_repository.create_workflow(&workflow_model).await?;

        tracing::info!(
            "Account opening workflow {} initiated for customer {} with product {}",
            workflow.workflow_id, request.customer_id, request.product_code
        );

        Ok(workflow)
    }

    /// Complete KYC verification step in account opening
    async fn complete_kyc_verification(&self, account_id: Uuid, verification_result: KycResult) -> BankingResult<()> {
        // Find active workflow for this account
        let workflow = self.workflow_repository
            .find_active_workflow(account_id, "AccountOpening")
            .await?
            .ok_or(banking_api::BankingError::ValidationError {
                field: "workflow".to_string(),
                message: format!("No active account opening workflow found for account {account_id}"),
            })?;

        match verification_result.status {
            banking_api::domain::KycStatus::Approved => {
                // Advance workflow to next step
                self.advance_workflow_step(
                    workflow.workflow_id,
                    WorkflowStep::DocumentVerification,
                    "KYC verification completed successfully",
                ).await?;

                tracing::info!(
                    "KYC verification completed for account {}. Workflow advanced.",
                    account_id
                );
            }
            banking_api::domain::KycStatus::Rejected => {
                // Fail the workflow
                self.fail_workflow(
                    workflow.workflow_id,
                    &format!("KYC verification failed: {} missing documents", verification_result.missing_documents.len()),
                ).await?;

                return Err(banking_api::BankingError::ValidationError {
                    field: "kyc_verification".to_string(),
                    message: format!("KYC verification failed for account {} - {} missing documents", account_id, verification_result.missing_documents.len()),
                });
            }
            banking_api::domain::KycStatus::Pending => {
                // Update workflow with pending status
                self.update_workflow_status(
                    workflow.workflow_id,
                    WorkflowStatus::PendingAction,
                    "Awaiting additional KYC documentation",
                ).await?;
            }
            banking_api::domain::KycStatus::NotStarted => {
                // KYC not started - treat as pending
                self.update_workflow_status(
                    workflow.workflow_id,
                    WorkflowStatus::PendingAction,
                    "KYC verification not started",
                ).await?;
            }
            banking_api::domain::KycStatus::InProgress => {
                // KYC in progress - continue waiting
                self.update_workflow_status(
                    workflow.workflow_id,
                    WorkflowStatus::InProgress,
                    "KYC verification in progress",
                ).await?;
            }
            banking_api::domain::KycStatus::Complete => {
                // Complete is same as approved for our purposes
                self.advance_workflow_step(
                    workflow.workflow_id,
                    WorkflowStep::DocumentVerification,
                    "KYC verification completed successfully",
                ).await?;
            }
            banking_api::domain::KycStatus::RequiresUpdate => {
                // Requires update - treat as pending
                self.update_workflow_status(
                    workflow.workflow_id,
                    WorkflowStatus::PendingAction,
                    "KYC verification requires update",
                ).await?;
            }
            banking_api::domain::KycStatus::Failed => {
                // Failed is same as rejected
                self.fail_workflow(
                    workflow.workflow_id,
                    &format!("KYC verification failed: {} missing documents", verification_result.missing_documents.len()),
                ).await?;

                return Err(banking_api::BankingError::ValidationError {
                    field: "kyc_verification".to_string(),
                    message: format!("KYC verification failed for account {} - {} missing documents", account_id, verification_result.missing_documents.len()),
                });
            }
        }

        Ok(())
    }

    /// Activate account after all verifications complete
    async fn activate_account(&self, account_id: Uuid, authorized_by: String) -> BankingResult<()> {
        // Validate account exists and is in pending state
        let account_model = self.account_repository
            .find_by_id(account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(account_id))?;

        let account = AccountMapper::from_model(account_model)?;

        if account.account_status != AccountStatus::PendingApproval {
            return Err(banking_api::BankingError::ValidationError {
                field: "account_status".to_string(),
                message: format!("Invalid account activation: account {} is not in PendingApproval status (current: {:?})", account_id, account.account_status),
            });
        }

        // Update account status to Active
        self.account_repository
            .update_status(account_id, "Active", "Account approved and activated", &authorized_by)
            .await?;

        // Complete workflow
        if let Ok(Some(workflow)) = self.workflow_repository
            .find_active_workflow(account_id, "AccountOpening")
            .await 
        {
            self.complete_workflow(workflow.workflow_id, "Account successfully activated").await?;
        }

        tracing::info!(
            "Account {} activated by {}",
            account_id, authorized_by
        );

        Ok(())
    }

    /// Check account eligibility for dormancy status
    async fn check_dormancy_eligibility(&self, account_id: Uuid) -> BankingResult<DormancyAssessment> {
        let account_model = self.account_repository
            .find_by_id(account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(account_id))?;

        let account = AccountMapper::from_model(account_model)?;

        // Get product-specific dormancy rules
        let product_rules = self.product_catalog_client.get_product_rules(&account.product_code).await?;
        let threshold_days = account.dormancy_threshold_days
            .unwrap_or(product_rules.default_dormancy_days.unwrap_or(90));

        // Calculate days since last activity
        let days_inactive = if let Some(last_activity) = account.last_activity_date {
            let today = chrono::Utc::now().date_naive();
            (today - last_activity).num_days() as i32
        } else {
            // No activity since account opening
            let today = chrono::Utc::now().date_naive();
            (today - account.open_date).num_days() as i32
        };

        let is_eligible = account.account_status == AccountStatus::Active 
            && days_inactive >= threshold_days;

        Ok(DormancyAssessment {
            is_eligible,
            last_activity_date: account.last_activity_date,
            days_inactive,
            threshold_days,
            product_specific_rules: vec![
                format!("Product: {}", account.product_code),
                format!("Threshold: {} days", threshold_days),
            ],
        })
    }

    /// Mark account as dormant (automated process)
    async fn mark_account_dormant(&self, account_id: Uuid, system_triggered: bool) -> BankingResult<()> {
        // Validate dormancy eligibility
        let assessment = self.check_dormancy_eligibility(account_id).await?;
        
        if !assessment.is_eligible {
            return Err(banking_api::BankingError::ValidationError {
                field: "dormancy_eligibility".to_string(),
                message: "Account is not eligible for dormancy status".to_string(),
            });
        }

        // Update account status
        let updated_by = if system_triggered {
            "SYSTEM_DORMANCY_JOB".to_string()
        } else {
            "MANUAL_DORMANCY_TRIGGER".to_string()
        };

        self.account_repository
            .update_status(account_id, "Dormant", "Account marked dormant due to inactivity", &updated_by)
            .await?;

        tracing::info!(
            "Account {} marked as dormant. Days inactive: {}",
            account_id, assessment.days_inactive
        );

        Ok(())
    }

    /// Initiate account reactivation workflow
    async fn initiate_reactivation(&self, account_id: Uuid, requested_by: String) -> BankingResult<AccountWorkflow> {
        let account_model = self.account_repository
            .find_by_id(account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(account_id))?;

        let account = AccountMapper::from_model(account_model)?;

        if account.account_status != AccountStatus::Dormant {
            return Err(banking_api::BankingError::ValidationError {
                field: "account_status".to_string(),
                message: "Only dormant accounts can be reactivated".to_string(),
            });
        }

        // Create reactivation workflow
        let workflow = AccountWorkflow {
            workflow_id: Uuid::new_v4(),
            account_id,
            workflow_type: WorkflowType::AccountReactivation,
            current_step: WorkflowStep::InitiateRequest,
            status: WorkflowStatus::InProgress,
            initiated_by: requested_by,
            initiated_at: Utc::now(),
            completed_at: None,
            steps_completed: Vec::new(),
            next_action_required: Some("Mini-KYC verification required".to_string()),
            timeout_at: Some(Utc::now() + chrono::Duration::days(7)), // 7-day timeout
        };

        // Update account status to pending reactivation
        self.account_repository
            .update_status(account_id, "PendingReactivation", "Account reactivation initiated", "SYSTEM")
            .await?;

        // Convert to model and persist workflow
        let workflow_model = self.to_workflow_model(&workflow);
        self.workflow_repository.create_workflow(&workflow_model).await?;

        tracing::info!(
            "Reactivation workflow {} initiated for dormant account {}",
            workflow.workflow_id, account_id
        );

        Ok(workflow)
    }

    /// Complete mini-KYC for account reactivation
    async fn complete_mini_kyc(&self, account_id: Uuid, verification_result: KycResult) -> BankingResult<()> {
        let workflow = self.workflow_repository
            .find_active_workflow(account_id, "AccountReactivation")
            .await?
            .ok_or(banking_api::BankingError::ValidationError {
                field: "workflow".to_string(),
                message: format!("No active account reactivation workflow found for account {account_id}"),
            })?;

        match verification_result.status {
            banking_api::domain::KycStatus::Approved => {
                // Reactivate account
                self.account_repository
                    .update_status(account_id, "Active", "Account reactivated successfully", &workflow.initiated_by)
                    .await?;

                // Complete workflow
                self.complete_workflow(workflow.workflow_id, "Account reactivated after mini-KYC").await?;

                tracing::info!(
                    "Account {} reactivated after successful mini-KYC",
                    account_id
                );
            }
            banking_api::domain::KycStatus::Rejected => {
                // Keep account dormant, fail workflow
                self.account_repository
                    .update_status(account_id, "Dormant", "KYC verification failed", "SYSTEM")
                    .await?;

                self.fail_workflow(
                    workflow.workflow_id,
                    &format!("Mini-KYC failed: {} missing documents", verification_result.missing_documents.len()),
                ).await?;

                return Err(banking_api::BankingError::ValidationError {
                    field: "mini_kyc_verification".to_string(),
                    message: format!("Mini-KYC verification failed for account {} - {} missing documents", account_id, verification_result.missing_documents.len()),
                });
            }
            banking_api::domain::KycStatus::Pending => {
                self.update_workflow_status(
                    workflow.workflow_id,
                    WorkflowStatus::PendingAction,
                    "Awaiting additional mini-KYC documentation",
                ).await?;
            }
            banking_api::domain::KycStatus::NotStarted => {
                self.update_workflow_status(
                    workflow.workflow_id,
                    WorkflowStatus::PendingAction,
                    "Mini-KYC verification not started",
                ).await?;
            }
            banking_api::domain::KycStatus::InProgress => {
                self.update_workflow_status(
                    workflow.workflow_id,
                    WorkflowStatus::InProgress,
                    "Mini-KYC verification in progress",
                ).await?;
            }
            banking_api::domain::KycStatus::Complete => {
                // Treat Complete same as Approved
                self.account_repository
                    .update_status(account_id, "Active", "Account reactivated successfully", &workflow.initiated_by)
                    .await?;
                self.complete_workflow(workflow.workflow_id, "Account reactivated after mini-KYC").await?;
            }
            banking_api::domain::KycStatus::RequiresUpdate => {
                self.update_workflow_status(
                    workflow.workflow_id,
                    WorkflowStatus::PendingAction,
                    "Mini-KYC verification requires update",
                ).await?;
            }
            banking_api::domain::KycStatus::Failed => {
                // Treat Failed same as Rejected
                self.account_repository
                    .update_status(account_id, "Dormant", "KYC verification failed", "SYSTEM")
                    .await?;
                self.fail_workflow(
                    workflow.workflow_id,
                    &format!("Mini-KYC failed: {} missing documents", verification_result.missing_documents.len()),
                ).await?;
                return Err(banking_api::BankingError::ValidationError {
                    field: "mini_kyc_verification".to_string(),
                    message: format!("Mini-KYC verification failed for account {} - {} missing documents", account_id, verification_result.missing_documents.len()),
                });
            }
        }

        Ok(())
    }

    /// Initiate account closure workflow
    async fn initiate_closure(&self, account_id: Uuid, closure_request: ClosureRequest) -> BankingResult<AccountWorkflow> {
        let account_model = self.account_repository
            .find_by_id(account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(account_id))?;

        let account = AccountMapper::from_model(account_model)?;

        // Validate account can be closed
        if matches!(account.account_status, AccountStatus::Closed | AccountStatus::PendingClosure) {
            return Err(banking_api::BankingError::ValidationError {
                field: "account_status".to_string(),
                message: "Account is already closed or pending closure".to_string(),
            });
        }

        // Create closure workflow
        let workflow = AccountWorkflow {
            workflow_id: Uuid::new_v4(),
            account_id,
            workflow_type: WorkflowType::AccountClosure,
            current_step: WorkflowStep::InitiateRequest,
            status: WorkflowStatus::InProgress,
            initiated_by: closure_request.requested_by.clone(),
            initiated_at: Utc::now(),
            completed_at: None,
            steps_completed: Vec::new(),
            next_action_required: Some("Final settlement calculation required".to_string()),
            timeout_at: Some(Utc::now() + chrono::Duration::days(30)), // 30-day timeout
        };

        // Update account status
        self.account_repository
            .update_status(account_id, "PendingClosure", "Account closure requested", &closure_request.requested_by)
            .await?;

        // Convert to model and persist workflow
        let workflow_model = self.to_workflow_model(&workflow);
        self.workflow_repository.create_workflow(&workflow_model).await?;

        tracing::info!(
            "Account closure workflow {} initiated for account {} (reason: {:?})",
            workflow.workflow_id, account_id, closure_request.reason
        );

        Ok(workflow)
    }

    /// Calculate final settlement amounts for account closure
    async fn calculate_final_settlement(&self, account_id: Uuid) -> BankingResult<FinalSettlement> {
        let account_model = self.account_repository
            .find_by_id(account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(account_id))?;

        let account = AccountMapper::from_model(account_model)?;

        // Calculate accrued interest up to today
        let _today = chrono::Utc::now().date_naive();
        let accrued_interest = if account.accrued_interest > Decimal::ZERO {
            account.accrued_interest
        } else {
            // Calculate from last interest posting date
            Decimal::ZERO // Simplified for this implementation
        };

        // Get closure fees from product catalog
        let product_rules = self.product_catalog_client.get_product_rules(&account.product_code).await?;
        let closure_fees = product_rules.closure_fee;

        // Calculate final amount
        let final_amount = account.current_balance + accrued_interest - closure_fees;

        let settlement = FinalSettlement {
            current_balance: account.current_balance,
            accrued_interest,
            pending_fees: Decimal::ZERO, // Would calculate pending fees
            closure_fees,
            final_amount,
            requires_disbursement: final_amount > Decimal::ZERO,
        };

        tracing::debug!(
            "Final settlement calculated for account {}: Final amount = {}",
            account_id, final_amount
        );

        Ok(settlement)
    }

    /// Process final disbursement for account closure
    async fn process_final_disbursement(&self, account_id: Uuid, disbursement: banking_api::domain::DisbursementInstructions) -> BankingResult<()> {
        let settlement = self.calculate_final_settlement(account_id).await?;

        if !settlement.requires_disbursement {
            return Ok(()); // No disbursement needed
        }

        // Process disbursement based on method
        match disbursement.method {
            banking_api::domain::DisbursementMethod::Transfer => {
                // Process transfer to target account
                tracing::info!(
                    "Processing transfer disbursement of {} for account closure {}",
                    settlement.final_amount, account_id
                );
            }
            banking_api::domain::DisbursementMethod::CashWithdrawal => {
                // Process cash withdrawal
                tracing::info!(
                    "Processing cash withdrawal disbursement of {} for account closure {}",
                    settlement.final_amount, account_id
                );
            }
            banking_api::domain::DisbursementMethod::Check => {
                // Process check issuance
                tracing::info!(
                    "Processing check disbursement of {} for account closure {}",
                    settlement.final_amount, account_id
                );
            }
            banking_api::domain::DisbursementMethod::HoldFunds => {
                // Hold funds for dispute resolution
                tracing::info!(
                    "Holding funds of {} for disputed account closure {}",
                    settlement.final_amount, account_id
                );
            }
        }

        // In production, this would create disbursement transactions
        Ok(())
    }

    /// Finalize account closure
    async fn finalize_closure(&self, account_id: Uuid) -> BankingResult<()> {
        // Update account status to closed
        self.account_repository
            .update_status(account_id, "Closed", "Account closure completed", "SYSTEM")
            .await?;

        // Complete workflow if exists
        if let Ok(Some(workflow)) = self.workflow_repository
            .find_active_workflow(account_id, "AccountClosure")
            .await 
        {
            self.complete_workflow(workflow.workflow_id, "Account closure finalized").await?;
        }

        tracing::info!("Account {} closure finalized", account_id);

        Ok(())
    }

    /// Update account status with comprehensive validation and audit
    async fn update_account_status(
        &self,
        account_id: Uuid,
        new_status: AccountStatus,
        reason: String,
        authorized_by: String,
    ) -> BankingResult<()> {
        let account_model = self.account_repository
            .find_by_id(account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(account_id))?;

        let account = AccountMapper::from_model(account_model)?;

        // Validate status transition
        self.validate_status_transition(account.account_status, new_status)?;

        // Update status
        self.account_repository
            .update_status(
                account_id,
                &crate::mappers::AccountMapper::account_status_to_string(new_status),
                "Manual status update",
                &authorized_by,
            )
            .await?;

        tracing::info!(
            "Account {} status updated from {:?} to {:?} by {} (reason: {})",
            account_id, account.account_status, new_status, authorized_by, reason
        );

        Ok(())
    }

    /// Get account status change history
    async fn get_status_history(&self, _account_id: Uuid) -> BankingResult<Vec<StatusChangeRecord>> {
        // In production, this would query the account_status_history table
        Ok(Vec::new()) // Simplified for this implementation
    }

    /// Find workflow by ID
    async fn find_workflow_by_id(&self, _workflow_id: Uuid) -> BankingResult<Option<banking_api::domain::AccountWorkflow>> {
        todo!("Implement find_workflow_by_id")
    }

    /// Find workflows by account
    async fn find_workflows_by_account(&self, _account_id: Uuid) -> BankingResult<Vec<banking_api::domain::AccountWorkflow>> {
        todo!("Implement find_workflows_by_account")
    }

    /// Update workflow status
    async fn update_workflow_status(&self, _workflow_id: Uuid, _status: banking_api::domain::WorkflowStatus) -> BankingResult<()> {
        todo!("Implement update_workflow_status")
    }

    /// Advance workflow step
    async fn advance_workflow_step(&self, _workflow_id: Uuid, _completed_by: String, _notes: Option<String>) -> BankingResult<()> {
        todo!("Implement advance_workflow_step")
    }

    /// Reject workflow
    async fn reject_workflow(&self, _workflow_id: Uuid, _reason: String, _rejected_by: String) -> BankingResult<()> {
        todo!("Implement reject_workflow")
    }

    /// Find pending activations
    async fn find_pending_activations(&self) -> BankingResult<Vec<banking_api::domain::AccountWorkflow>> {
        todo!("Implement find_pending_activations")
    }

    /// Find pending closures
    async fn find_pending_closures(&self) -> BankingResult<Vec<banking_api::domain::AccountWorkflow>> {
        todo!("Implement find_pending_closures")
    }

    /// Find accounts eligible for dormancy
    async fn find_accounts_eligible_for_dormancy(&self, _threshold_days: i32) -> BankingResult<Vec<Uuid>> {
        todo!("Implement find_accounts_eligible_for_dormancy")
    }

    /// Batch process dormancy
    async fn batch_process_dormancy(&self, _processing_date: chrono::NaiveDate) -> BankingResult<banking_api::service::DormancyReport> {
        todo!("Implement batch_process_dormancy")
    }

    /// Batch process closures
    async fn batch_process_closures(&self, _processing_date: chrono::NaiveDate) -> BankingResult<banking_api::service::MaintenanceReport> {
        todo!("Implement batch_process_closures")
    }

    /// Trigger compliance check
    async fn trigger_compliance_check(&self, _account_id: Uuid, _check_type: String) -> BankingResult<()> {
        todo!("Implement trigger_compliance_check")
    }

    /// Handle compliance result
    async fn handle_compliance_result(&self, _account_id: Uuid, _result: banking_api::service::ComplianceCheckResult) -> BankingResult<()> {
        todo!("Implement handle_compliance_result")
    }
}

impl AccountLifecycleServiceImpl {
    /// Validate product eligibility for account opening
    async fn validate_product_eligibility(&self, request: &AccountOpeningRequest) -> BankingResult<()> {
        // Check if product exists
        let _product_rules = self.product_catalog_client
            .get_product_rules(&request.product_code)
            .await
            .map_err(|_| banking_api::BankingError::InvalidProductCode(request.product_code.clone()))?;

        // Additional eligibility checks would go here
        // (customer type compatibility, minimum deposits, etc.)

        Ok(())
    }

    /// Advance workflow to next step
    async fn advance_workflow_step(
        &self,
        workflow_id: Uuid,
        next_step: WorkflowStep,
        notes: &str,
    ) -> BankingResult<()> {
        let step_str = format!("{next_step:?}");
        self.workflow_repository
            .update_workflow_step(workflow_id, &step_str)
            .await?;

        tracing::debug!(
            "Workflow {} advanced to step {:?}: {}",
            workflow_id, next_step, notes
        );

        Ok(())
    }

    /// Complete workflow successfully
    async fn complete_workflow(&self, workflow_id: Uuid, completion_notes: &str) -> BankingResult<()> {
        self.workflow_repository
            .complete_workflow(workflow_id, completion_notes)
            .await?;

        tracing::info!(
            "Workflow {} completed: {}",
            workflow_id, completion_notes
        );

        Ok(())
    }

    /// Fail workflow with reason
    async fn fail_workflow(&self, workflow_id: Uuid, failure_reason: &str) -> BankingResult<()> {
        self.workflow_repository
            .fail_workflow(workflow_id, failure_reason)
            .await?;

        tracing::warn!(
            "Workflow {} failed: {}",
            workflow_id, failure_reason
        );

        Ok(())
    }

    /// Update workflow status
    async fn update_workflow_status(
        &self,
        workflow_id: Uuid,
        status: WorkflowStatus,
        notes: &str,
    ) -> BankingResult<()> {
        let status_str = format!("{status:?}");
        self.workflow_repository
            .update_workflow_status(workflow_id, &status_str, notes)
            .await?;

        tracing::debug!(
            "Workflow {} status updated to {:?}: {}",
            workflow_id, status, notes
        );

        Ok(())
    }

    /// Validate account status transitions
    fn validate_status_transition(
        &self,
        current: AccountStatus,
        new: AccountStatus,
    ) -> BankingResult<()> {
        use AccountStatus::*;

        let valid_transitions = match current {
            PendingApproval => vec![Active, Closed],
            Active => vec![Dormant, Frozen, PendingClosure, Closed],
            Dormant => vec![Active, PendingReactivation, Closed],
            Frozen => vec![Active, Closed], // Only compliance can unfreeze
            PendingReactivation => vec![Active, Dormant],
            PendingClosure => vec![Closed, Active], // Can revert closure
            Closed => vec![], // Closed is terminal
        };

        if !valid_transitions.contains(&new) {
            return Err(banking_api::BankingError::ValidationError {
                field: "status_transition".to_string(),
                message: format!("Invalid status transition from {current:?} to {new:?}"),
            });
        }

        Ok(())
    }

    /// Convert domain AccountWorkflow to database AccountWorkflowModel
    fn to_workflow_model(&self, workflow: &AccountWorkflow) -> banking_db::models::AccountWorkflowModel {
        banking_db::models::AccountWorkflowModel {
            workflow_id: workflow.workflow_id,
            account_id: workflow.account_id,
            workflow_type: format!("{:?}", workflow.workflow_type),
            current_step: format!("{:?}", workflow.current_step),
            status: format!("{:?}", workflow.status),
            initiated_by: workflow.initiated_by.clone(),
            initiated_at: workflow.initiated_at,
            completed_at: workflow.completed_at,
            next_action_required: workflow.next_action_required.clone(),
            timeout_at: workflow.timeout_at,
            metadata: None, // Could serialize steps_completed if needed
            priority: "Medium".to_string(), // Default priority
            assigned_to: None,
            created_at: chrono::Utc::now(),
            last_updated_at: chrono::Utc::now(),
            updated_by: "SYSTEM".to_string(),
        }
    }
}
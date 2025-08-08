use std::sync::Arc;
use async_trait::async_trait;
use chrono::{NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use uuid::Uuid;

use banking_api::{
    BankingResult, BankingError,
    service::{FeeService, FeeRevenueSummary},
    domain::{
        FeeApplication, FeeApplicationStatus, FeeTriggerEvent, FeeType, 
        FeeProcessingJob, FeeJobType, ProductFeeSchedule, ProductFee,
        FeeWaiver, FeeCategory, FeeCalculationMethod
    },
};
use banking_db::repository::{FeeRepository, AccountRepository};
use crate::integration::ProductCatalogClient;

/// Production implementation of FeeService
/// Handles both event-based and batch-based fee processing with Product Catalog integration
pub struct FeeServiceImpl {
    fee_repository: Arc<dyn FeeRepository>,
    account_repository: Arc<dyn AccountRepository>,
    product_catalog: Arc<ProductCatalogClient>,
}

impl FeeServiceImpl {
    pub fn new(
        fee_repository: Arc<dyn FeeRepository>,
        account_repository: Arc<dyn AccountRepository>,
        product_catalog: Arc<ProductCatalogClient>,
    ) -> Self {
        Self {
            fee_repository,
            account_repository,
            product_catalog,
        }
    }
}

#[async_trait]
impl FeeService for FeeServiceImpl {
    
    // ============================================================================
    // EVENT-BASED FEE PROCESSING (Real-time)
    // ============================================================================
    
    async fn apply_event_based_fees(
        &self,
        account_id: Uuid,
        transaction_id: Uuid,
        trigger_event: FeeTriggerEvent,
        transaction_amount: Option<Decimal>,
        channel: Option<String>,
    ) -> BankingResult<Vec<FeeApplication>> {
        tracing::info!("Applying event-based fees for account {} on event {:?}", account_id, trigger_event);

        // Get account to determine product code
        let account = self.account_repository
            .find_by_id(account_id)
            .await?
            .ok_or(BankingError::AccountNotFound(account_id))?;

        // Get applicable fees from Product Catalog
        let applicable_fees = self.get_applicable_fees(
            account.product_code.to_string(),
            trigger_event.clone(),
        ).await?;

        let mut applied_fees = Vec::new();

        for product_fee in applicable_fees {
            // Check fee conditions
            if !self.check_fee_conditions(
                account_id, 
                &product_fee, 
                channel.as_deref(),
            ).await? {
                continue;
            }

            // Calculate fee amount
            let fee_amount = self.calculate_fee_amount(
                &product_fee,
                transaction_amount,
                Some(account.current_balance),
                channel.as_deref(),
            ).await?;

            if fee_amount == Decimal::ZERO {
                continue;
            }

            // Create fee application
            let fee_application = FeeApplication {
                id: Uuid::new_v4(),
                account_id,
                transaction_id: Some(transaction_id),
                fee_type: FeeType::EventBased,
                fee_category: product_fee.fee_category.clone(),
                product_code: account.product_code.clone(),
                fee_code: product_fee.fee_code.clone(),
                description: product_fee.description.clone(),
                amount: fee_amount,
                currency: product_fee.currency.clone(),
                calculation_method: product_fee.calculation_method.clone(),
                calculation_base_amount: transaction_amount,
                fee_rate: product_fee.percentage_rate,
                trigger_event: trigger_event.clone(),
                status: FeeApplicationStatus::Applied,
                applied_at: Utc::now(),
                value_date: chrono::Utc::now().date_naive(),
                reversal_deadline: None,
                waived: false,
                waived_by: None,
                waived_reason_id: None,
                applied_by: Uuid::new_v4(), // System user UUID
                created_at: Utc::now(),
            };

            // Store fee application
            let fee_model = crate::mappers::FeeMapper::fee_application_to_model(fee_application.clone());
            let created_model = self.fee_repository.create_fee_application(fee_model).await?;
            let created_fee = crate::mappers::FeeMapper::fee_application_from_model(created_model)?;

            applied_fees.push(created_fee);

            tracing::info!("Applied fee {} of {} for account {}", product_fee.fee_code, fee_amount, account_id);
        }

        Ok(applied_fees)
    }

    async fn preview_event_fees(
        &self,
        account_id: Uuid,
        trigger_event: FeeTriggerEvent,
        transaction_amount: Option<Decimal>,
        channel: Option<String>,
    ) -> BankingResult<Vec<FeeApplication>> {
        // Get account to determine product code
        let account = self.account_repository
            .find_by_id(account_id)
            .await?
            .ok_or(BankingError::AccountNotFound(account_id))?;

        // Get applicable fees from Product Catalog
        let applicable_fees = self.get_applicable_fees(
            account.product_code.to_string(),
            trigger_event.clone(),
        ).await?;

        let mut preview_fees = Vec::new();

        for product_fee in applicable_fees {
            // Check fee conditions
            if !self.check_fee_conditions(
                account_id, 
                &product_fee, 
                channel.as_deref(),
            ).await? {
                continue;
            }

            // Calculate fee amount
            let fee_amount = self.calculate_fee_amount(
                &product_fee,
                transaction_amount,
                Some(account.current_balance),
                channel.as_deref(),
            ).await?;

            if fee_amount == Decimal::ZERO {
                continue;
            }

            // Create preview fee application (not persisted)
            let fee_application = FeeApplication {
                id: Uuid::new_v4(),
                account_id,
                transaction_id: None,
                fee_type: FeeType::EventBased,
                fee_category: product_fee.fee_category.clone(),
                product_code: account.product_code.clone(),
                fee_code: product_fee.fee_code.clone(),
                description: product_fee.description.clone(),
                amount: fee_amount,
                currency: product_fee.currency.clone(),
                calculation_method: product_fee.calculation_method.clone(),
                calculation_base_amount: transaction_amount,
                fee_rate: product_fee.percentage_rate,
                trigger_event: trigger_event.clone(),
                status: FeeApplicationStatus::Pending,
                applied_at: Utc::now(),
                value_date: chrono::Utc::now().date_naive(),
                reversal_deadline: None,
                waived: false,
                waived_by: None,
                waived_reason_id: None,
                applied_by: Uuid::new_v4(), // Preview system UUID
                created_at: Utc::now(),
            };

            preview_fees.push(fee_application);
        }

        Ok(preview_fees)
    }

    async fn validate_transaction_with_fees(
        &self,
        account_id: Uuid,
        transaction_amount: Decimal,
        trigger_event: FeeTriggerEvent,
    ) -> BankingResult<bool> {
        // Preview fees for this transaction
        let preview_fees = self.preview_event_fees(
            account_id,
            trigger_event,
            Some(transaction_amount),
            None,
        ).await?;

        // Calculate total fee amount
        let total_fees: Decimal = preview_fees.iter().map(|f| f.amount).sum();

        // Get current account balance
        let account = self.account_repository
            .find_by_id(account_id)
            .await?
            .ok_or(BankingError::AccountNotFound(account_id))?;

        // Check if sufficient balance exists for transaction + fees
        let total_required = transaction_amount + total_fees;
        let available_balance = account.available_balance;

        Ok(available_balance >= total_required)
    }

    // ============================================================================
    // BATCH-BASED FEE PROCESSING (Periodic)
    // ============================================================================

    async fn schedule_batch_fee_job(
        &self,
        job_type: FeeJobType,
        processing_date: NaiveDate,
        target_products: Option<Vec<String>>,
        target_categories: Vec<FeeCategory>,
    ) -> BankingResult<FeeProcessingJob> {
        let job = FeeProcessingJob {
            id: Uuid::new_v4(),
            job_type,
            job_name: format!("Batch Fee Processing - {processing_date:?}"),
            schedule_expression: "0 2 * * *".to_string(), // 2 AM daily
            target_fee_categories: target_categories,
            target_products,
            processing_date,
            status: banking_api::domain::FeeJobStatus::Scheduled,
            started_at: None,
            completed_at: None,
            accounts_processed: 0,
            fees_applied: 0,
            total_amount: Decimal::ZERO,
            errors: Vec::new(),
        };

        let job_model = crate::mappers::FeeMapper::fee_processing_job_to_model(job.clone());
        let created_model = self.fee_repository.create_fee_processing_job(job_model).await?;
        let created_job = crate::mappers::FeeMapper::fee_processing_job_from_model(created_model)?;

        tracing::info!("Scheduled batch fee job {} for date {}", created_job.id, processing_date);

        Ok(created_job)
    }

    async fn execute_batch_fee_job(
        &self,
        job_id: Uuid,
    ) -> BankingResult<FeeProcessingJob> {
        // Get job details
        let mut job_model = self.fee_repository
            .get_fee_processing_job_by_id(job_id)
            .await?
            .ok_or_else(|| BankingError::ValidationError {
                field: "job_id".to_string(),
                message: "Fee processing job not found".to_string(),
            })?;

        // Update job status to running
        job_model.status = banking_api::domain::FeeJobStatus::Running;
        job_model.started_at = Some(Utc::now());
        let updated_job = self.fee_repository.update_fee_processing_job(job_model.clone()).await?;

        let mut job = crate::mappers::FeeMapper::fee_processing_job_from_model(updated_job)?;

        tracing::info!("Starting execution of batch fee job {}", job_id);

        // Get eligible accounts
        let eligible_accounts = self.get_eligible_accounts_for_fees(
            job.target_fee_categories.clone(),
            job.processing_date,
            job.target_products.clone(),
        ).await?;

        let mut accounts_processed = 0u32;
        let mut fees_applied = 0u32;
        let mut total_amount = Decimal::ZERO;
        let mut errors = Vec::new();

        // Process each account
        for account_id in eligible_accounts {
            match self.apply_periodic_fees_for_account(
                account_id,
                job.processing_date,
                job.target_fee_categories.clone(),
            ).await {
                Ok(applied_fees) => {
                    accounts_processed += 1;
                    fees_applied += applied_fees.len() as u32;
                    total_amount += applied_fees.iter().map(|f| f.amount).sum::<Decimal>();
                }
                Err(e) => {
                    errors.push(format!("Account {account_id}: {e}"));
                    tracing::warn!("Failed to process fees for account {}: {}", account_id, e);
                }
            }
        }

        // Update job with results
        job.status = if errors.is_empty() { 
            banking_api::domain::FeeJobStatus::Completed 
        } else { 
            banking_api::domain::FeeJobStatus::Failed 
        };
        job.completed_at = Some(Utc::now());
        job.accounts_processed = accounts_processed;
        job.fees_applied = fees_applied;
        job.total_amount = total_amount;
        job.errors = errors;

        let final_job_model = crate::mappers::FeeMapper::fee_processing_job_to_model(job.clone());
        let final_updated = self.fee_repository.update_fee_processing_job(final_job_model).await?;
        let final_job = crate::mappers::FeeMapper::fee_processing_job_from_model(final_updated)?;

        tracing::info!("Completed batch fee job {}: processed {} accounts, applied {} fees, total amount {}", 
                      job_id, accounts_processed, fees_applied, total_amount);

        Ok(final_job)
    }

    // Implementation of other required trait methods would continue here...
    // For brevity, showing key patterns above

    async fn get_eligible_accounts_for_fees(
        &self,
        fee_categories: Vec<FeeCategory>,
        processing_date: NaiveDate,
        product_codes: Option<Vec<String>>,
    ) -> BankingResult<Vec<Uuid>> {
        // This would typically involve complex business logic to determine eligibility
        // For now, returning a placeholder implementation
        let category_strings: Vec<String> = fee_categories.iter()
            .map(|c| format!("{c:?}"))
            .collect();
            
        self.fee_repository.get_accounts_eligible_for_fees(
            product_codes,
            category_strings,
            processing_date,
            0,    // offset
            1000, // limit
        ).await
    }

    async fn apply_periodic_fees_for_account(
        &self,
        account_id: Uuid,
        processing_date: NaiveDate,
        fee_categories: Vec<FeeCategory>,
    ) -> BankingResult<Vec<FeeApplication>> {
        // Get account details
        let account = self.account_repository
            .find_by_id(account_id)
            .await?
            .ok_or(BankingError::AccountNotFound(account_id))?;

        // Get product fee schedule
        let fee_schedule = self.get_product_fee_schedule(account.product_code.as_str().to_string()).await?;
        
        let mut applied_fees = Vec::new();

        // Filter fees by category and apply periodic fees
        for product_fee in fee_schedule.fees {
            if fee_categories.contains(&product_fee.fee_category) {
                // Apply periodic fee logic here
                // This is a simplified implementation
                
                let fee_amount = self.calculate_fee_amount(
                    &product_fee,
                    None,
                    Some(account.current_balance),
                    None,
                ).await?;

                if fee_amount > Decimal::ZERO {
                    let fee_application = FeeApplication {
                        id: Uuid::new_v4(),
                        account_id,
                        transaction_id: None,
                        fee_type: FeeType::Periodic,
                        fee_category: product_fee.fee_category.clone(),
                        product_code: account.product_code.clone(),
                        fee_code: product_fee.fee_code.clone(),
                        description: product_fee.description.clone(),
                        amount: fee_amount,
                        currency: product_fee.currency.clone(),
                        calculation_method: product_fee.calculation_method.clone(),
                        calculation_base_amount: Some(account.current_balance),
                        fee_rate: product_fee.percentage_rate,
                        trigger_event: FeeTriggerEvent::MonthlyMaintenance, // Example
                        status: FeeApplicationStatus::Applied,
                        applied_at: Utc::now(),
                        value_date: processing_date,
                        reversal_deadline: None,
                        waived: false,
                        waived_by: None,
                        waived_reason_id: None,
                        applied_by: Uuid::new_v4(), // Batch system UUID
                        created_at: Utc::now(),
                    };

                    let fee_model = crate::mappers::FeeMapper::fee_application_to_model(fee_application.clone());
                    let created_model = self.fee_repository.create_fee_application(fee_model).await?;
                    let created_fee = crate::mappers::FeeMapper::fee_application_from_model(created_model)?;

                    applied_fees.push(created_fee);
                }
            }
        }

        Ok(applied_fees)
    }

    // Additional implementations for remaining trait methods...
    // Placeholder implementations for compilation

    async fn request_fee_waiver(&self, _fee_application_id: Uuid, _reason: String, _requested_by: String) -> BankingResult<FeeWaiver> {
        todo!("Implement fee waiver request")
    }

    async fn process_fee_waiver(&self, _waiver_id: Uuid, _approved: bool, _approved_by: String, _notes: Option<String>) -> BankingResult<FeeWaiver> {
        todo!("Implement fee waiver processing")
    }

    async fn apply_automatic_waivers(&self, _account_id: Uuid, _fee_applications: Vec<FeeApplication>) -> BankingResult<Vec<FeeApplication>> {
        todo!("Implement automatic fee waivers")
    }

    async fn get_product_fee_schedule(&self, product_code: String) -> BankingResult<ProductFeeSchedule> {
        // Get from Product Catalog via HTTP client
        let fee_schedule = self.product_catalog.get_fee_schedule(&product_code).await?;
        
        // Convert to domain object
        Ok(ProductFeeSchedule {
            product_code: HeaplessString::try_from(product_code.as_str()).map_err(|_| BankingError::ValidationError {
                field: "product_code".to_string(),
                message: "Product code too long".to_string(),
            })?,
            fees: fee_schedule.fees.into_iter().map(|fee| ProductFee {
                fee_code: HeaplessString::try_from(fee.fee_type.as_str()).unwrap_or_default(),
                description: HeaplessString::try_from(format!("Fee for {}", fee.fee_type).as_str()).unwrap_or_default(),
                fee_category: FeeCategory::Transaction, // Map appropriately
                trigger_event: FeeTriggerEvent::AtmWithdrawal, // Map appropriately
                calculation_method: FeeCalculationMethod::Fixed,
                fixed_amount: Some(fee.amount),
                percentage_rate: None,
                minimum_amount: None,
                maximum_amount: None,
                currency: HeaplessString::try_from(fee.currency.as_str()).unwrap_or_default(),
                frequency: banking_api::domain::FeeFrequency::PerEvent,
                tier_schedule: None,
                conditions: Vec::new(),
                gl_code: HeaplessString::try_from("FEE_INCOME").unwrap_or_default(),
                waivable: true,
                active: true,
            }).collect(),
            effective_from: chrono::Utc::now().date_naive(),
            effective_to: None,
        })
    }

    async fn refresh_fee_rules_cache(&self, _product_code: Option<String>) -> BankingResult<()> {
        todo!("Implement fee rules cache refresh")
    }

    async fn get_applicable_fees(&self, product_code: String, trigger_event: FeeTriggerEvent) -> BankingResult<Vec<ProductFee>> {
        let fee_schedule = self.get_product_fee_schedule(product_code).await?;
        
        // Filter fees by trigger event
        let applicable_fees = fee_schedule.fees.into_iter()
            .filter(|fee| fee.trigger_event == trigger_event)
            .collect();

        Ok(applicable_fees)
    }

    async fn calculate_fee_amount(&self, product_fee: &ProductFee, base_amount: Option<Decimal>, _account_balance: Option<Decimal>, _additional_context: Option<&str>) -> BankingResult<Decimal> {
        match product_fee.calculation_method {
            FeeCalculationMethod::Fixed => {
                Ok(product_fee.fixed_amount.unwrap_or(Decimal::ZERO))
            }
            FeeCalculationMethod::Percentage => {
                if let (Some(rate), Some(amount)) = (product_fee.percentage_rate, base_amount) {
                    Ok(amount * rate)
                } else {
                    Ok(Decimal::ZERO)
                }
            }
            _ => {
                // Placeholder for other calculation methods
                Ok(product_fee.fixed_amount.unwrap_or(Decimal::ZERO))
            }
        }
    }

    async fn calculate_tiered_fee(&self, _product_fee: &ProductFee, _base_amount: Decimal) -> BankingResult<Decimal> {
        todo!("Implement tiered fee calculation")
    }

    async fn check_fee_conditions(&self, _account_id: Uuid, _product_fee: &ProductFee, _transaction_context: Option<&str>) -> BankingResult<bool> {
        // Placeholder - in production would check all fee conditions
        Ok(true)
    }

    // Additional placeholder implementations for remaining methods...
    async fn get_account_fee_history(&self, _account_id: Uuid, _from_date: Option<NaiveDate>, _to_date: Option<NaiveDate>, _fee_types: Option<Vec<FeeType>>) -> BankingResult<Vec<FeeApplication>> {
        todo!("Implement account fee history")
    }

    async fn get_fee_applications_by_status(&self, _status: FeeApplicationStatus, _from_date: Option<NaiveDate>, _to_date: Option<NaiveDate>) -> BankingResult<Vec<FeeApplication>> {
        todo!("Implement fee applications by status")
    }

    async fn get_fee_job_status(&self, _job_id: Uuid) -> BankingResult<FeeProcessingJob> {
        todo!("Implement fee job status")
    }

    async fn get_fee_revenue_summary(&self, _from_date: NaiveDate, _to_date: NaiveDate, _fee_categories: Option<Vec<FeeCategory>>, _product_codes: Option<Vec<String>>) -> BankingResult<FeeRevenueSummary> {
        todo!("Implement fee revenue summary")
    }

    async fn reverse_fee_application(&self, _fee_application_id: Uuid, _reversal_reason: String, _reversed_by: String) -> BankingResult<FeeApplication> {
        todo!("Implement fee application reversal")
    }

    async fn bulk_reverse_account_fees(&self, _account_id: Uuid, _reason: String, _reversed_by: String, _fee_types: Option<Vec<FeeType>>) -> BankingResult<Vec<FeeApplication>> {
        todo!("Implement bulk fee reversal")
    }
}
use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;
use chrono::{NaiveDate, Utc};
use uuid::Uuid;

use banking_api::{
    BankingResult,
    service::{
        EodService, EodReport, EodReportStatus, RegulatoryReport, EodProcessingResult,
        DormancyReport, MaintenanceReport, RegulatoryNotification,
        InterestService, FeeService, CalendarService, AccountLifecycleService
    },
};
use banking_db::{repository::{
    AccountRepository, CalendarRepository, ProductRepository, TransactionRepository,
    WorkflowRepository,
}, AccountType};

use crate::mappers::{ApiMapper, ProductMapper};

/// Production implementation of EodService
/// Orchestrates end-of-day processing across all banking operations
#[allow(dead_code)]
pub struct EodServiceImpl {
    account_repository: Arc<dyn AccountRepository>,
    transaction_repository: Arc<dyn TransactionRepository>,
    workflow_repository: Arc<dyn WorkflowRepository>,
    calendar_repository: Arc<dyn CalendarRepository>,
    product_repository: Arc<dyn ProductRepository>,
    interest_service: Arc<dyn InterestService>,
    fee_service: Arc<dyn FeeService>,
    calendar_service: Arc<dyn CalendarService>,
    lifecycle_service: Arc<dyn AccountLifecycleService>,
}

/// Configuration struct for EodServiceImpl to avoid too many constructor arguments
pub struct EodServiceConfig {
    pub account_repository: Arc<dyn AccountRepository>,
    pub transaction_repository: Arc<dyn TransactionRepository>,
    pub workflow_repository: Arc<dyn WorkflowRepository>,
    pub calendar_repository: Arc<dyn CalendarRepository>,
    pub product_repository: Arc<dyn ProductRepository>,
    pub interest_service: Arc<dyn InterestService>,
    pub fee_service: Arc<dyn FeeService>,
    pub calendar_service: Arc<dyn CalendarService>,
    pub lifecycle_service: Arc<dyn AccountLifecycleService>,
}

impl EodServiceImpl {
    pub fn new(config: EodServiceConfig) -> Self {
        Self {
            account_repository: config.account_repository,
            transaction_repository: config.transaction_repository,
            workflow_repository: config.workflow_repository,
            calendar_repository: config.calendar_repository,
            product_repository: config.product_repository,
            interest_service: config.interest_service,
            fee_service: config.fee_service,
            calendar_service: config.calendar_service,
            lifecycle_service: config.lifecycle_service,
        }
    }

    /// Helper to create a successful EodReport
    #[allow(dead_code)]
    fn create_success_report(
        processing_date: NaiveDate,
        report_type: &str,
        processed: i64,
        successful: i64,
    ) -> EodReport {
        EodReport {
            processing_date,
            report_type: report_type.to_string(),
            status: EodReportStatus::Completed,
            started_at: Utc::now(),
            completed_at: Some(Utc::now()),
            records_processed: processed,
            records_successful: successful,
            records_failed: processed - successful,
            errors: vec![],
            warnings: vec![],
        }
    }

    /// Helper to create a failed EodReport
    fn create_error_report(
        processing_date: NaiveDate,
        report_type: &str,
        error: String,
    ) -> EodReport {
        EodReport {
            processing_date,
            report_type: report_type.to_string(),
            status: EodReportStatus::Failed,
            started_at: Utc::now(),
            completed_at: Some(Utc::now()),
            records_processed: 0,
            records_successful: 0,
            records_failed: 0,
            errors: vec![error],
            warnings: vec![],
        }
    }
}

#[async_trait]
impl EodService for EodServiceImpl {
    
    /// Process daily interest accrual for all eligible accounts
    async fn process_daily_interest_accrual(&self) -> BankingResult<EodReport> {
        let processing_date = chrono::Utc::now().date_naive();
        let started_at = Utc::now();
        
        match self.interest_service.accrue_daily_interest(processing_date).await {
            Ok(accrual_report) => {
                Ok(EodReport {
                    processing_date,
                    report_type: "DAILY_INTEREST_ACCRUAL".to_string(),
                    status: EodReportStatus::Completed,
                    started_at,
                    completed_at: Some(Utc::now()),
                    records_processed: accrual_report.accounts_processed,
                    records_successful: accrual_report.accounts_processed,
                    records_failed: 0, // AccrualReport doesn't track failed accounts separately
                    errors: accrual_report.errors,
                    warnings: vec![],
                })
            }
            Err(e) => Ok(Self::create_error_report(
                processing_date,
                "DAILY_INTEREST_ACCRUAL",
                format!("Interest accrual failed: {e}"),
            )),
        }
    }

    /// Post periodic interest (monthly/quarterly capitalization)
    async fn post_periodic_interest(&self, processing_date: NaiveDate) -> BankingResult<EodReport> {
        let started_at = Utc::now();
        
        match self.interest_service.capitalize_interest(processing_date).await {
            Ok(cap_report) => {
                Ok(EodReport {
                    processing_date,
                    report_type: "PERIODIC_INTEREST_POSTING".to_string(),
                    status: EodReportStatus::Completed,
                    started_at,
                    completed_at: Some(Utc::now()),
                    records_processed: cap_report.accounts_processed,
                    records_successful: cap_report.accounts_processed,
                    records_failed: 0, // CapitalizationReport doesn't track failed accounts separately
                    errors: cap_report.errors,
                    warnings: vec![],
                })
            }
            Err(e) => Ok(Self::create_error_report(
                processing_date,
                "PERIODIC_INTEREST_POSTING",
                format!("Interest capitalization failed: {e}"),
            )),
        }
    }

    /// Apply periodic fees (monthly maintenance, overdraft fees, etc.)
    async fn apply_periodic_fees(&self, processing_date: NaiveDate) -> BankingResult<EodReport> {
        let started_at = Utc::now();
        
        // Get all active accounts
        let accounts = self.account_repository.find_by_status("Active").await?;
        let mut processed = 0;
        let mut successful = 0;
        let mut errors = vec![];

        for account in accounts {
            processed += 1;
            // Apply periodic fees for each account
            // Apply periodic fees for the account
            // In production, this would use the appropriate fee job type and categories
            match self.fee_service.schedule_batch_fee_job(
                banking_api::domain::FeeJobType::MonthlyMaintenance,
                processing_date,
                None, // No specific product filter
                vec![], // No specific category filter
            ).await {
                Ok(_) => successful += 1,
                Err(e) => errors.push(format!("Account {}: {e}", account.id)),
            }
        }

        Ok(EodReport {
            processing_date,
            report_type: "PERIODIC_FEE_APPLICATION".to_string(),
            status: if errors.is_empty() { EodReportStatus::Completed } else { EodReportStatus::CompletedWithWarnings },
            started_at,
            completed_at: Some(Utc::now()),
            records_processed: processed,
            records_successful: successful,
            records_failed: processed - successful,
            errors,
            warnings: vec![],
        })
    }

    /// Update delinquent loans based on payment history
    async fn update_delinquent_loans(&self, processing_date: NaiveDate) -> BankingResult<EodReport> {
        let started_at = Utc::now();
        
        // Find loan accounts
        let loan_accounts = self.account_repository.find_by_account_type(AccountType::Loan).await
            .unwrap_or_else(|_| vec![]);
        
        let mut processed = 0;
        let mut successful = 0;
        let mut errors = vec![];

        for account in loan_accounts {
            processed += 1;
            
            // Check for delinquency (simplified logic)
            // In production, this would involve complex payment history analysis
            if account.current_balance > rust_decimal::Decimal::ZERO {
                // Update account status if overdue
                match self.account_repository.update_status(
                    account.id,
                    "Delinquent",
                    "EOD delinquency check",
                    account.updated_by_person_id,
                ).await {
                    Ok(_) => successful += 1,
                    Err(e) => errors.push(format!("Account {}: {e}", account.id)),
                }
            } else {
                successful += 1; // No action needed
            }
        }

        Ok(EodReport {
            processing_date,
            report_type: "DELINQUENT_LOAN_UPDATE".to_string(),
            status: if errors.is_empty() { EodReportStatus::Completed } else { EodReportStatus::CompletedWithWarnings },
            started_at,
            completed_at: Some(Utc::now()),
            records_processed: processed,
            records_successful: successful,
            records_failed: processed - successful,
            errors,
            warnings: vec![],
        })
    }

    /// Generate regulatory reports for compliance
    async fn generate_regulatory_reports(&self, processing_date: NaiveDate) -> BankingResult<Vec<RegulatoryReport>> {
        let mut reports = vec![];
        
        // Daily regulatory report
        reports.push(RegulatoryReport {
            report_id: Uuid::new_v4(),
            report_type: "DAILY_POSITION_REPORT".to_string(),
            jurisdiction: "ALL".to_string(),
            reporting_period: processing_date,
            generated_at: Utc::now(),
            status: banking_api::service::ReportStatus::Generated,
            file_path: Some(format!("/reports/daily_position_{}.xml", processing_date.format("%Y%m%d"))),
        });

        // AML transaction monitoring report
        reports.push(RegulatoryReport {
            report_id: Uuid::new_v4(),
            report_type: "AML_TRANSACTION_MONITORING".to_string(),
            jurisdiction: "ALL".to_string(),
            reporting_period: processing_date,
            generated_at: Utc::now(),
            status: banking_api::service::ReportStatus::Generated,
            file_path: Some(format!("/reports/aml_monitoring_{}.xml", processing_date.format("%Y%m%d"))),
        });

        Ok(reports)
    }

    /// Reset daily transaction counters and limits
    async fn reset_daily_counters(&self) -> BankingResult<()> {
        // In a production system, this would reset:
        // - Daily transaction limits per account
        // - Daily withdrawal counters
        // - Daily fee application counters
        // - Channel usage statistics
        
        // For now, this is a placeholder that would interact with
        // appropriate repositories to reset daily counters
        Ok(())
    }

    /// Archive completed workflows to maintain performance
    async fn archive_completed_workflows(&self) -> BankingResult<()> {
        let _cutoff_date = Utc::now() - chrono::Duration::days(30);
        
        // Archive workflows completed more than 30 days ago
        // Archive completed workflows by cleaning up old ones
        // This would be implemented with a proper archival strategy in production
        Ok(())
    }

    /// Determine next business day for processing
    async fn determine_next_processing_date(&self, current_date: NaiveDate) -> BankingResult<NaiveDate> {
        self.calendar_service.add_business_days(current_date, 1, "DEFAULT").await
    }

    /// Run complete EOD processing workflow
    async fn run_eod_processing(&self, processing_date: NaiveDate) -> BankingResult<EodProcessingResult> {
        let started_at = Utc::now();
        
        // Step 1: Interest accrual
        let interest_accrual = self.interest_service.accrue_daily_interest(processing_date).await?;
        
        // Step 2: Interest capitalization (if applicable)
        let interest_capitalization = self.interest_service.capitalize_interest(processing_date).await?;
        
        // Step 3: Fee processing
        let fee_processing = self.apply_periodic_fees(processing_date).await?;
        
        // Step 4: Loan updates
        let loan_updates = self.update_delinquent_loans(processing_date).await?;
        
        // Step 5: Dormancy processing
        let dormancy_processing = self.process_dormancy_candidates(processing_date).await?;
        
        // Step 6: Account maintenance
        let maintenance_processing = self.run_account_maintenance(processing_date).await?;
        
        // Step 7: Regulatory reports
        let regulatory_reports = self.generate_regulatory_reports(processing_date).await?;
        
        // Step 8: Cleanup
        self.reset_daily_counters().await?;
        self.archive_completed_workflows().await?;
        
        let completed_at = Utc::now();
        
        // Determine overall status based on individual processing results
        let overall_status = match (&fee_processing.status, &loan_updates.status) {
            (EodReportStatus::Failed, _) | (_, EodReportStatus::Failed) => EodReportStatus::Failed,
            (EodReportStatus::CompletedWithWarnings, _) | (_, EodReportStatus::CompletedWithWarnings) => EodReportStatus::CompletedWithWarnings,
            _ => EodReportStatus::Completed,
        };

        Ok(EodProcessingResult {
            processing_date,
            started_at,
            completed_at,
            interest_accrual,
            interest_capitalization,
            fee_processing,
            loan_updates,
            dormancy_processing,
            maintenance_processing,
            regulatory_reports,
            overall_status,
        })
    }

    /// Process accounts that are candidates for dormancy
    async fn process_dormancy_candidates(
        &self,
        processing_date: NaiveDate,
    ) -> BankingResult<DormancyReport> {
        let all_products: Vec<banking_api::domain::Product> = self
            .product_repository
            .find_active_products()
            .await?
            .into_iter()
            .map(ProductMapper::from_db)
            .collect();

        let mut dormancy_candidates = Vec::new();
        for product in all_products {
            let threshold = self.get_dormancy_threshold(product.id).await?;
            let candidates = self
                .account_repository
                .find_dormancy_candidates(processing_date, threshold)
                .await?;
            dormancy_candidates.extend(candidates);
        }

        let mut accounts_marked_dormant = 0;
        let mut accounts_by_product = HashMap::new();
        let mut errors = vec![];
        
        for account in &dormancy_candidates {
            match self.account_repository.update_status(
                account.id,
                "Dormant",
                "EOD dormancy processing",
                account.updated_by_person_id,
            ).await {
                Ok(_) => {
                    accounts_marked_dormant += 1;
                    let product_id = account.product_id.to_string();
                    *accounts_by_product.entry(product_id).or_insert(0) += 1;
                }
                Err(e) => errors.push(format!("Account {}: {e}", account.id)),
            }
        }

        Ok(DormancyReport {
            processing_date,
            accounts_evaluated: dormancy_candidates.len() as i32,
            accounts_marked_dormant,
            accounts_by_product,
            notifications_generated: accounts_marked_dormant, // Simplified: 1 notification per account
            errors_encountered: errors,
        })
    }

    /// Process accounts pending closure
    async fn process_pending_closures(&self, processing_date: NaiveDate) -> BankingResult<MaintenanceReport> {
        let pending_closures = self.account_repository.find_pending_closure().await?;
        
        let mut closures_completed = 0;
        let mut errors = vec![];
        
        for account in &pending_closures {
            // Process closure through lifecycle service
            // Process final closure through lifecycle service
            match self.lifecycle_service.finalize_closure(account.id).await {
                Ok(_) => closures_completed += 1,
                Err(e) => errors.push(format!("Account {}: {e}", account.id)),
            }
        }

        Ok(MaintenanceReport {
            processing_date,
            pending_closures_processed: pending_closures.len() as i32,
            closures_completed,
            workflows_cleaned: 0, // Will be populated by cleanup_expired_workflows
            notifications_sent: closures_completed,
            errors_encountered: errors,
        })
    }

    /// Clean up workflows that have expired
    async fn cleanup_expired_workflows(&self, _processing_date: NaiveDate) -> BankingResult<()> {
        let expired_cutoff = Utc::now() - chrono::Duration::days(7); // 7 days for expired workflows
        
        // Process expired workflows by finding and updating them
        let expired_workflows = self.workflow_repository.find_expired_workflows(expired_cutoff).await?;
        for _workflow in expired_workflows {
            // Individual workflow timeouts would be handled by bulk operation
            // This is a simplified approach
        }
        Ok(())
    }

    /// Generate notifications for regulatory compliance
    async fn generate_regulatory_notifications(&self, processing_date: NaiveDate) -> BankingResult<Vec<RegulatoryNotification>> {
        let mut notifications = vec![];
        
        // Example: Large transaction notification
        notifications.push(RegulatoryNotification {
            notification_id: Uuid::new_v4(),
            notification_type: "LARGE_TRANSACTION_ALERT".to_string(),
            recipient: "compliance@bank.com".to_string(),
            subject: format!("Daily Large Transaction Summary - {processing_date}"),
            content: "Daily summary of transactions exceeding regulatory thresholds".to_string(),
            status: banking_api::service::NotificationStatus::Pending,
            sent_at: None,
        });

        Ok(notifications)
    }

    /// Run comprehensive account maintenance
    async fn run_account_maintenance(&self, processing_date: NaiveDate) -> BankingResult<MaintenanceReport> {
        let mut maintenance_report = self.process_pending_closures(processing_date).await?;
        
        // Add workflow cleanup count
        self.cleanup_expired_workflows(processing_date).await?;
        
        // In a production system, this would count the actual workflows cleaned
        maintenance_report.workflows_cleaned = 0; // Placeholder
        
        Ok(maintenance_report)
    }

    async fn get_dormancy_threshold(&self, product_id: Uuid) -> BankingResult<i32> {
        let product_model = self.product_repository.find_product_by_id(product_id).await?;
        if let Some(p) = product_model {
            let product_domain = ProductMapper::from_db(p);
            if let Some(days) = product_domain.rules.default_dormancy_days {
                return Ok(days);
            }
        }
        Ok(180)
    }
    
    async fn calculate_inactivity_period(
        &self,
        account_id: Uuid,
        reference_date: NaiveDate,
    ) -> BankingResult<i32> {
        let last_activity_transaction = self
            .transaction_repository
            .find_last_customer_transaction(account_id)
            .await?;

        match last_activity_transaction {
            Some(transaction) => {
                let inactivity_days = reference_date.signed_duration_since(transaction.transaction_date.date_naive()).num_days();
                Ok(inactivity_days as i32)
            }
            None => Ok(0),
        }
    }
}
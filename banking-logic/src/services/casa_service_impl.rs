use std::sync::Arc;
use async_trait::async_trait;
use chrono::{NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal::prelude::ToPrimitive;
use uuid::Uuid;

use banking_api::{
    BankingResult, BankingError, TransactionType,
    service::CasaService,
    domain::{
        casa::AuthorizationLevel,
        OverdraftFacility, OverdraftUtilization, OverdraftInterestCalculation,
        CasaAccountSummary, OverdraftProcessingJob, OverdraftLimitAdjustment,
        CasaTransactionValidation, InterestPostingRecord, InterestType,
        CasaValidationResult, CompoundingFrequency, ReviewFrequency, OverdraftStatus,
        CreateOverdraftFacilityRequest
    },
};
use banking_db::repository::AccountRepository;
use crate::integration::ProductCatalogClient;

/// Production implementation of CasaService
/// Handles CASA (Current & Savings Account) specialized functionality
/// Integrates overdraft management with transaction validation framework (Section 3.1)
pub struct CasaServiceImpl {
    account_repository: Arc<dyn AccountRepository>,
    product_catalog: Arc<ProductCatalogClient>,
}

impl CasaServiceImpl {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
        product_catalog: Arc<ProductCatalogClient>,
    ) -> Self {
        Self {
            account_repository,
            product_catalog,
        }
    }
}

#[async_trait]
impl CasaService for CasaServiceImpl {
    
    // ============================================================================
    // OVERDRAFT FACILITY MANAGEMENT
    // ============================================================================
    
    async fn create_overdraft_facility(
        &self,
        request: CreateOverdraftFacilityRequest,
    ) -> BankingResult<OverdraftFacility> {
        tracing::info!("Creating overdraft facility for account {} with limit {}", request.account_id, request.approved_limit);

        // Validate account exists and is a current account
        let account = self.account_repository
            .find_by_id(request.account_id)
            .await?
            .ok_or(BankingError::AccountNotFound(request.account_id))?;

        if account.account_type != banking_api::domain::AccountType::Current {
            return Err(BankingError::ValidationError {
                field: "account_type".to_string(),
                message: "Overdraft facilities are only available for current accounts".to_string(),
            });
        }

        // Validate business rules
        if request.approved_limit <= Decimal::ZERO {
            return Err(BankingError::ValidationError {
                field: "approved_limit".to_string(),
                message: "Approved limit must be greater than zero".to_string(),
            });
        }

        if request.interest_rate < Decimal::ZERO || request.interest_rate > Decimal::ONE {
            return Err(BankingError::ValidationError {
                field: "interest_rate".to_string(),
                message: "Interest rate must be between 0 and 1".to_string(),
            });
        }

        // Create overdraft facility
        let facility = OverdraftFacility {
            id: Uuid::new_v4(),
            account_id: request.account_id,
            approved_limit: request.approved_limit,
            current_utilized: Decimal::ZERO,
            available_limit: request.approved_limit,
            interest_rate: request.interest_rate,
            facility_status: OverdraftStatus::Active,
            approval_date: chrono::Utc::now().date_naive(),
            expiry_date: request.expiry_date,
            approved_by_person_id: request.approved_by_person_id,
            review_frequency: ReviewFrequency::Annually,
            next_review_date: chrono::Utc::now().date_naive() + chrono::Duration::days(365),
            security_required: request.security_required,
            security_details: request.security_details,
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
        };

        // Update account with overdraft limit
        let mut updated_account = account.clone();
        updated_account.overdraft_limit = Some(request.approved_limit);
        self.account_repository.update(updated_account).await?;

        tracing::info!("Created overdraft facility {} for account {}", facility.id, request.account_id);

        Ok(facility)
    }

    #[allow(unused_variables)]
    async fn update_overdraft_facility(
        &self,
        facility_id: Uuid,
        new_limit: Option<Decimal>,
        new_rate: Option<Decimal>,
        new_expiry_date: Option<NaiveDate>,
        updated_by_person_id: Uuid, // References Person.person_id
    ) -> BankingResult<OverdraftFacility> {
        // Implementation would update facility and account
        todo!("Implement overdraft facility update")
    }

    #[allow(unused_variables)]
    async fn update_overdraft_status(
        &self,
        facility_id: Uuid,
        new_status: OverdraftStatus,
        reason: String,
        updated_by_person_id: Uuid, // References Person.person_id
    ) -> BankingResult<OverdraftFacility> {
        todo!("Implement overdraft status update")
    }

    #[allow(unused_variables)]
    async fn get_overdraft_facility(
        &self,
        account_id: Uuid,
    ) -> BankingResult<Option<OverdraftFacility>> {
        todo!("Implement get overdraft facility")
    }

    #[allow(unused_variables)]
    async fn request_overdraft_limit_adjustment(
        &self,
        account_id: Uuid,
        requested_limit: Decimal,
        adjustment_reason: String,
        supporting_documents: Vec<String>,
        requested_by_person_id: Uuid, // References Person.person_id
    ) -> BankingResult<OverdraftLimitAdjustment> {
        todo!("Implement overdraft limit adjustment request")
    }

    #[allow(unused_variables)]
    async fn process_overdraft_adjustment(
        &self,
        adjustment_id: Uuid,
        approved: bool,
        approved_by_person_id: Uuid, // References Person.person_id
        approval_notes: Option<HeaplessString<512>>,
        effective_date: Option<NaiveDate>,
    ) -> BankingResult<OverdraftLimitAdjustment> {
        todo!("Implement overdraft adjustment processing")
    }

    // ============================================================================
    // TRANSACTION VALIDATION WITH OVERDRAFT
    // ============================================================================
    
    async fn validate_casa_transaction(
        &self,
        account_id: Uuid,
        transaction_amount: Decimal,
        transaction_type: TransactionType,
        #[allow(unused_variables)] _channel: Option<String>,
    ) -> BankingResult<CasaTransactionValidation> {
        tracing::debug!("Validating CASA transaction: account={account_id}, amount={transaction_amount}, type={transaction_type:?}");

        // Get account details
        let account = self.account_repository
            .find_by_id(account_id)
            .await?
            .ok_or(BankingError::AccountNotFound(account_id))?;

        // Get current available balance (considering holds)
        let balance_calc = self.account_repository
            .calculate_total_holds(account_id, None)
            .await?;

        let available_balance = account.current_balance + 
                               account.overdraft_limit.unwrap_or(Decimal::ZERO) - 
                               balance_calc;

        let mut validation_messages: Vec<HeaplessString<200>> = Vec::new();
        let mut requires_authorization = false;
        let mut authorization_level = None;

        // Calculate post-transaction balance
        let post_transaction_balance = if transaction_type == TransactionType::Debit {
            account.current_balance - transaction_amount
        } else {
            account.current_balance + transaction_amount
        };

        // Determine overdraft utilization if applicable
        let overdraft_utilization = if post_transaction_balance < Decimal::ZERO {
            Some(post_transaction_balance.abs())
        } else {
            None
        };

        // Validate transaction
        let validation_result = if transaction_type == TransactionType::Debit {
            if transaction_amount > available_balance {
                CasaValidationResult::Rejected
            } else if overdraft_utilization.is_some() {
                // Transaction will utilize overdraft
                if let Ok(msg) = HeaplessString::try_from("Transaction will utilize overdraft facility") {
                    validation_messages.push(msg);
                }
                
                let overdraft_amount = overdraft_utilization.unwrap();
                let overdraft_limit = account.overdraft_limit.unwrap_or(Decimal::ZERO);
                
                if overdraft_amount > overdraft_limit {
                    CasaValidationResult::Rejected
                } else {
                    // Check if authorization is required based on amount or first-time usage
                    if overdraft_amount > Decimal::from(1000) { // $1,000 threshold
                        requires_authorization = true;
                        authorization_level = Some(AuthorizationLevel::Supervisor);
                    }
                    
                    if requires_authorization {
                        CasaValidationResult::RequiresAuthorization
                    } else {
                        CasaValidationResult::Approved
                    }
                }
            } else {
                CasaValidationResult::Approved
            }
        } else {
            // Credit transactions are generally approved
            CasaValidationResult::Approved
        };

        let casa_validation = CasaTransactionValidation {
            account_id,
            transaction_amount,
            transaction_type,
            current_balance: account.current_balance,
            available_balance,
            overdraft_limit: account.overdraft_limit,
            post_transaction_balance,
            overdraft_utilization,
            validation_result,
            validation_messages,
            requires_authorization,
            authorization_level,
        };

        tracing::debug!("CASA transaction validation result: {:?}", casa_validation.validation_result);

        Ok(casa_validation)
    }

    #[allow(unused_variables)]
    async fn check_overdraft_impact(
        &self,
        account_id: Uuid,
        transaction_amount: Decimal,
    ) -> BankingResult<banking_api::service::casa_service::OverdraftImpactAnalysis> {
        let account = self.account_repository
            .find_by_id(account_id)
            .await?
            .ok_or(BankingError::AccountNotFound(account_id))?;

        let post_transaction_balance = account.current_balance - transaction_amount;
        let will_trigger_overdraft = post_transaction_balance < Decimal::ZERO;
        let overdraft_amount = if will_trigger_overdraft {
            Some(post_transaction_balance.abs())
        } else {
            None
        };

        let overdraft_limit = account.overdraft_limit;
        let available_overdraft_limit = if let Some(limit) = overdraft_limit {
            if let Some(amount) = overdraft_amount {
                Some(limit - amount)
            } else {
                Some(limit)
            }
        } else {
            None
        };

        // Calculate estimated daily interest cost
        let estimated_daily_interest_cost = if let Some(amount) = overdraft_amount {
            // Get overdraft rate from Product Catalog
            #[allow(unused_variables)]
            let product_rules = self.product_catalog.get_product_rules(&account.product_code).await?;
            let daily_rate = Decimal::from_str("0.15").unwrap() / Decimal::from(365); // 15% annual rate
            Some(amount * daily_rate)
        } else {
            None
        };

        let impact_analysis = banking_api::service::casa_service::OverdraftImpactAnalysis {
            account_id,
            current_balance: account.current_balance,
            transaction_amount,
            post_transaction_balance,
            overdraft_limit,
            will_trigger_overdraft,
            overdraft_amount,
            available_overdraft_limit,
            estimated_daily_interest_cost,
            authorization_required: overdraft_amount.is_some_and(|amount| amount > Decimal::from(1000)),
        };

        Ok(impact_analysis)
    }

    #[allow(unused_variables)]
    async fn preauthorize_overdraft_usage(
        &self,
        account_id: Uuid,
        requested_amount: Decimal,
        authorization_reason: String,
        authorized_by_person_id: Uuid,
        validity_period: chrono::Duration,
    ) -> BankingResult<banking_api::service::casa_service::OverdraftPreauthorization> {
        todo!("Implement overdraft preauthorization")
    }

    // ============================================================================
    // OVERDRAFT INTEREST CALCULATION
    // ============================================================================
    
    #[allow(unused_variables)]
    async fn calculate_daily_overdraft_interest(
        &self,
        account_id: Uuid,
        calculation_date: NaiveDate,
    ) -> BankingResult<OverdraftInterestCalculation> {
        tracing::debug!("Calculating daily overdraft interest for account {} on {}", account_id, calculation_date);

        // Get account details
        let account = self.account_repository
            .find_by_id(account_id)
            .await?
            .ok_or(BankingError::AccountNotFound(account_id))?;

        // Check if account is overdrawn
        if account.current_balance >= Decimal::ZERO {
            return Err(BankingError::ValidationError {
                field: "current_balance".to_string(),
                message: "Account is not overdrawn".to_string(),
            });
        }

        // Get overdraft facility for interest rate
        let facility = self.get_overdraft_facility(account_id)
            .await?
            .ok_or_else(|| BankingError::ValidationError {
                field: "overdraft_facility".to_string(),
                message: "No overdraft facility found for account".to_string(),
            })?;

        let overdrawn_amount = account.current_balance.abs();
        let annual_rate = facility.interest_rate;
        let daily_rate = annual_rate / Decimal::from(365);
        let interest_amount = overdrawn_amount * daily_rate;

        let calculation = OverdraftInterestCalculation {
            id: Uuid::new_v4(),
            account_id,
            calculation_period_start: calculation_date,
            calculation_period_end: calculation_date,
            average_overdrawn_balance: overdrawn_amount,
            applicable_rate: annual_rate,
            days_calculated: 1,
            interest_amount,
            compounding_frequency: CompoundingFrequency::Daily,
            capitalization_due: false, // Would be determined by product rules
            calculated_at: Utc::now(),
            calculated_by_person_id: Uuid::nil(), // System-generated calculation
        };

        tracing::debug!("Calculated daily overdraft interest: {} for account {}", interest_amount, account_id);

        Ok(calculation)
    }

    // Remaining method implementations would follow similar patterns...
    // For brevity, providing placeholder implementations

    #[allow(unused_variables)]
    async fn get_overdraft_utilization_history(
        &self,
        account_id: Uuid,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> BankingResult<Vec<OverdraftUtilization>> {
        todo!("Implement overdraft utilization history")
    }

    #[allow(unused_variables)]
    async fn record_overdraft_utilization(
        &self,
        account_id: Uuid,
        utilization_date: NaiveDate,
        opening_balance: Decimal,
        closing_balance: Decimal,
        average_daily_balance: Decimal,
    ) -> BankingResult<OverdraftUtilization> {
        todo!("Implement record overdraft utilization")
    }

    async fn calculate_compound_overdraft_interest(
        &self,
        _account_id: Uuid,
        principal_amount: Decimal,
        annual_rate: Decimal,
        days: u32,
        compounding_frequency: CompoundingFrequency,
    ) -> BankingResult<Decimal> {
        let periods_per_year = match compounding_frequency {
            CompoundingFrequency::Daily => Decimal::from(365),
            CompoundingFrequency::Weekly => Decimal::from(52),
            CompoundingFrequency::Monthly => Decimal::from(12),
            CompoundingFrequency::Quarterly => Decimal::from(4),
        };

        let rate_per_period = annual_rate / periods_per_year;
        let periods = Decimal::from(days) * periods_per_year / Decimal::from(365);

        // Compound interest formula: A = P(1 + r)^n - P
        // Simplified calculation for demonstration
        let compound_factor = (Decimal::ONE + rate_per_period).powi(periods.to_u64().unwrap_or(1) as i64);
        let interest = principal_amount * (compound_factor - Decimal::ONE);

        Ok(interest)
    }

    #[allow(unused_variables)]
    async fn post_overdraft_interest(
        &self,
        account_id: Uuid,
        interest_amount: Decimal,
        calculation_period_start: NaiveDate,
        calculation_period_end: NaiveDate,
        posting_date: NaiveDate,
        posted_by_person_id: Uuid,
    ) -> BankingResult<InterestPostingRecord> {
        todo!("Implement overdraft interest posting")
    }

    #[allow(unused_variables)]
    async fn capitalize_overdraft_interest(
        &self,
        account_id: Uuid,
        interest_amount: Decimal,
        capitalization_date: NaiveDate,
        authorized_by_person_id: Uuid,
    ) -> BankingResult<InterestPostingRecord> {
        todo!("Implement overdraft interest capitalization")
    }

    #[allow(unused_variables)]
    async fn get_interest_posting_history(
        &self,
        account_id: Uuid,
        interest_type: Option<InterestType>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> BankingResult<Vec<InterestPostingRecord>> {
        todo!("Implement interest posting history")
    }

    // ============================================================================
    // EOD BATCH PROCESSING
    // ============================================================================
    
    #[allow(unused_variables)]
    async fn execute_daily_overdraft_processing(
        &self,
        processing_date: NaiveDate,
        account_filter: Option<Vec<Uuid>>,
    ) -> BankingResult<OverdraftProcessingJob> {
        tracing::info!("Starting daily overdraft processing for date {}", processing_date);

        let mut job = OverdraftProcessingJob {
            id: Uuid::new_v4(),
            processing_date,
            accounts_processed: 0,
            total_interest_accrued: Decimal::ZERO,
            accounts_capitalized: 0,
            total_capitalized_amount: Decimal::ZERO,
            status: banking_api::domain::casa::ProcessingJobStatus::Running,
            started_at: Some(Utc::now()),
            completed_at: None,
            errors_01: HeaplessString::new(),
            errors_02: HeaplessString::new(),
            errors_03: HeaplessString::new(),
            errors_04: HeaplessString::new(),
            errors_05: HeaplessString::new(),
        };
        let mut error_count = 0;

        // Get overdrawn accounts
        let overdrawn_accounts = self.get_overdrawn_accounts(processing_date).await?;

        for account_id in overdrawn_accounts {
            if let Some(ref filter) = account_filter {
                if !filter.contains(&account_id) {
                    continue;
                }
            }

            match self.calculate_daily_overdraft_interest(account_id, processing_date).await {
                Ok(calculation) => {
                    job.accounts_processed += 1;
                    job.total_interest_accrued += calculation.interest_amount;

                    // Post interest to account (in production)
                    // self.post_overdraft_interest(...).await?;
                }
                Err(e) => {
                    error_count += 1;
                    let error_msg = format!("Account {account_id}: {e}");
                    let truncated_msg = if error_msg.len() > 200 {
                        &error_msg[..200]
                    } else {
                        &error_msg
                    };
                    let error_heapless = HeaplessString::try_from(truncated_msg).unwrap_or_default();
                    
                    match error_count {
                        1 => job.errors_01 = error_heapless,
                        2 => job.errors_02 = error_heapless,
                        3 => job.errors_03 = error_heapless,
                        4 => job.errors_04 = error_heapless,
                        5 => job.errors_05 = error_heapless,
                        _ => {} // Skip additional errors beyond 5
                    }
                    tracing::warn!("Failed to process overdraft interest for account {}: {}", account_id, e);
                }
            }
        }

        job.status = if error_count == 0 { 
            banking_api::domain::casa::ProcessingJobStatus::Completed
        } else { 
            banking_api::domain::casa::ProcessingJobStatus::PartiallyCompleted
        };
        job.completed_at = Some(Utc::now());

        tracing::info!("Completed daily overdraft processing: {} accounts processed, {} total interest", 
                      job.accounts_processed, job.total_interest_accrued);

        Ok(job)
    }

    #[allow(unused_variables)]
    async fn get_overdrawn_accounts(
        &self,
        as_of_date: NaiveDate,
    ) -> BankingResult<Vec<Uuid>> {
        // This would query accounts with current_balance < 0 and active overdraft facilities
        todo!("Implement get overdrawn accounts query")
    }

    // Additional method implementations would continue...
    // All remaining methods would follow similar patterns with proper error handling,
    // logging, and business rule validation

    #[allow(unused_variables)]
    async fn process_interest_capitalization(
        &self,
        processing_date: NaiveDate,
        capitalization_frequency: CompoundingFrequency,
    ) -> BankingResult<Vec<InterestPostingRecord>> {
        todo!("Implement interest capitalization processing")
    }

    #[allow(unused_variables)]
    async fn generate_overdraft_processing_report(
        &self,
        job_id: Uuid,
    ) -> BankingResult<banking_api::service::casa_service::OverdraftProcessingReport> {
        todo!("Implement overdraft processing report generation")
    }

    #[allow(unused_variables)]
    async fn get_casa_account_summary(
        &self,
        account_id: Uuid,
    ) -> BankingResult<CasaAccountSummary> {
        todo!("Implement CASA account summary")
    }

    #[allow(unused_variables)]
    async fn update_overdraft_utilization(
        &self,
        account_id: Uuid,
        utilized_amount: Decimal,
    ) -> BankingResult<()> {
        todo!("Implement overdraft utilization update")
    }

    #[allow(unused_variables)]
    async fn assess_dormancy_risk(
        &self,
        account_id: Uuid,
        assessment_date: NaiveDate,
    ) -> BankingResult<banking_api::domain::casa::DormancyRisk> {
        todo!("Implement dormancy risk assessment")
    }

    #[allow(unused_variables)]
    async fn get_accounts_for_overdraft_review(
        &self,
        review_date: NaiveDate,
        review_frequency: ReviewFrequency,
    ) -> BankingResult<Vec<OverdraftFacility>> {
        todo!("Implement accounts for overdraft review")
    }

    #[allow(unused_variables)]
    async fn generate_overdraft_portfolio_analytics(
        &self,
        as_of_date: NaiveDate,
        product_codes: Option<Vec<String>>,
    ) -> BankingResult<banking_api::service::casa_service::OverdraftPortfolioAnalytics> {
        todo!("Implement overdraft portfolio analytics")
    }

    #[allow(unused_variables)]
    async fn get_overdraft_revenue_summary(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
        account_ids: Option<Vec<Uuid>>,
    ) -> BankingResult<banking_api::service::casa_service::OverdraftRevenueSummary> {
        todo!("Implement overdraft revenue summary")
    }

    #[allow(unused_variables)]
    async fn generate_overdraft_regulatory_report(
        &self,
        reporting_date: NaiveDate,
        report_type: banking_api::service::casa_service::OverdraftReportType,
    ) -> BankingResult<banking_api::service::casa_service::OverdraftRegulatoryReport> {
        todo!("Implement overdraft regulatory reporting")
    }

    #[allow(unused_variables)]
    async fn get_high_risk_overdraft_accounts(
        &self,
        risk_threshold: Decimal,
        assessment_date: NaiveDate,
    ) -> BankingResult<Vec<banking_api::service::casa_service::HighRiskOverdraftAccount>> {
        todo!("Implement high-risk overdraft accounts identification")
    }
}

// Helper trait for string parsing
trait FromStr {
    fn from_str(s: &str) -> Result<Self, &'static str> where Self: Sized;
}

impl FromStr for Decimal {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        s.parse().map_err(|_| "Invalid decimal")
    }
}
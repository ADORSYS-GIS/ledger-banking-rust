use std::sync::Arc;
use async_trait::async_trait;
use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;
use heapless::String as HeaplessString;

use banking_api::{
    BankingResult, BankingError,
    service::{InterestService, CalendarService},
    domain::{AccountType, TransactionType, TransactionStatus, Transaction},
};
use banking_db::{
    repository::{AccountRepository, TransactionRepository},
};
use crate::{
    mappers::{AccountMapper, TransactionMapper},
    integration::ProductCatalogClient,
};

/// Production implementation of InterestService
/// Provides product catalog-driven interest calculations with business day awareness
pub struct InterestServiceImpl {
    account_repository: Arc<dyn AccountRepository>,
    transaction_repository: Arc<dyn TransactionRepository>,
    product_catalog_client: Arc<ProductCatalogClient>,
    calendar_service: Arc<dyn CalendarService>,
}

impl InterestServiceImpl {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
        transaction_repository: Arc<dyn TransactionRepository>,
        product_catalog_client: Arc<ProductCatalogClient>,
        calendar_service: Arc<dyn CalendarService>,
    ) -> Self {
        Self {
            account_repository,
            transaction_repository,
            product_catalog_client,
            calendar_service,
        }
    }
}

#[async_trait]
impl InterestService for InterestServiceImpl {
    /// Calculate daily interest accrual for a specific account
    async fn calculate_daily_interest(&self, account_id: Uuid) -> BankingResult<Decimal> {
        // Get account information
        let account_model = self.account_repository
            .find_by_id(account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(account_id))?;

        let account = AccountMapper::from_model(account_model)?;

        // Only calculate interest for interest-bearing accounts
        let daily_interest = match account.account_type {
            AccountType::Savings => {
                self.calculate_savings_daily_interest(&account).await?
            }
            AccountType::Loan => {
                self.calculate_loan_daily_interest(&account).await?
            }
            AccountType::Current => {
                // Current accounts typically don't earn interest, but may have overdraft interest
                if account.current_balance < Decimal::ZERO {
                    self.calculate_overdraft_daily_interest(&account).await?
                } else {
                    Decimal::ZERO
                }
            }
        };

        tracing::debug!(
            "Daily interest calculated for account {}: {}",
            account_id, daily_interest
        );

        Ok(daily_interest)
    }

    /// Post periodic interest to account balance (capitalization)
    async fn post_periodic_interest(&self, account_id: Uuid) -> BankingResult<()> {
        let account_model = self.account_repository
            .find_by_id(account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(account_id))?;

        let account = AccountMapper::from_model(account_model)?;

        // Only post interest if there's accrued interest
        if account.accrued_interest <= Decimal::ZERO {
            return Ok(());
        }

        // Determine if today is an interest posting day
        let today = chrono::Utc::now().date_naive();
        if !self.should_post_interest(account_id, today).await? {
            return Ok(());
        }

        // Create interest credit transaction
        let interest_transaction = Transaction {
            id: Uuid::new_v4(),
            account_id,
            transaction_code: HeaplessString::try_from("INT_POST").map_err(|_| BankingError::ValidationError {
                field: "transaction_code".to_string(),
                message: "Transaction code too long".to_string(),
            })?,
            transaction_type: TransactionType::Credit,
            amount: account.accrued_interest,
            currency: account.currency.clone(),
            description: {
                let desc_str = format!("Interest posting for period ending {today}");
                HeaplessString::try_from(desc_str.as_str()).map_err(|_| BankingError::ValidationError {
                    field: "description".to_string(),
                    message: "Description too long".to_string(),
                })?
            },
            channel_id: HeaplessString::try_from("SYSTEM").map_err(|_| BankingError::ValidationError {
                field: "channel_id".to_string(),
                message: "Channel ID too long".to_string(),
            })?,
            terminal_id: None,
            agent_user_id: None,
            transaction_date: Utc::now(),
            value_date: today,
            status: TransactionStatus::Posted,
            reference_number: {
                let ref_num = self.generate_interest_reference(&account, today).await?;
                HeaplessString::try_from(ref_num.as_str()).map_err(|_| BankingError::ValidationError {
                    field: "reference_number".to_string(),
                    message: "Reference number too long".to_string(),
                })?
            },
            external_reference: None,
            gl_code: {
                let gl_code_str = self.get_interest_gl_code(account.product_code.as_str()).await?;
                HeaplessString::try_from(gl_code_str.as_str()).map_err(|_| BankingError::ValidationError {
                    field: "gl_code".to_string(),
                    message: "GL code too long".to_string(),
                })?
            },
            requires_approval: false,
            approval_status: None,
            risk_score: Some(Decimal::ZERO), // System transaction, no risk
            created_at: Utc::now(),
        };

        // Post the interest transaction
        let transaction_model = TransactionMapper::to_model(interest_transaction);
        self.transaction_repository.create(transaction_model).await?;

        // Update account balance and reset accrued interest
        let new_balance = account.current_balance + account.accrued_interest;
        let new_available = account.available_balance + account.accrued_interest;
        self.account_repository.update_balance(account_id, new_balance, new_available).await?;
        self.account_repository.reset_accrued_interest(account_id).await?;

        tracing::info!(
            "Posted interest of {} to account {}. New balance: {}",
            account.accrued_interest, account_id, new_balance
        );

        Ok(())
    }

    /// Calculate loan installment amount using standard amortization formula
    async fn calculate_loan_installment(
        &self,
        principal: Decimal,
        rate: Decimal,
        term_months: i32,
    ) -> BankingResult<Decimal> {
        if principal <= Decimal::ZERO {
            return Err(banking_api::BankingError::ValidationError {
                field: "principal".to_string(),
                message: "Principal must be greater than zero".to_string(),
            });
        }

        if rate < Decimal::ZERO {
            return Err(banking_api::BankingError::ValidationError {
                field: "rate".to_string(),
                message: "Interest rate cannot be negative".to_string(),
            });
        }

        if term_months <= 0 {
            return Err(banking_api::BankingError::ValidationError {
                field: "term_months".to_string(),
                message: "Term must be greater than zero".to_string(),
            });
        }

        // Handle zero interest rate case
        if rate == Decimal::ZERO {
            return Ok(principal / Decimal::from(term_months));
        }

        // Monthly interest rate
        let monthly_rate = rate / Decimal::from(12);

        // Amortization formula: PMT = P * [r(1+r)^n] / [(1+r)^n - 1]
        let one_plus_r = Decimal::ONE + monthly_rate;
        let numerator = principal * monthly_rate * self.decimal_power(one_plus_r, term_months)?;
        let denominator = self.decimal_power(one_plus_r, term_months)? - Decimal::ONE;

        let installment = numerator / denominator;

        tracing::debug!(
            "Calculated loan installment: Principal={}, Rate={}, Term={} months, Installment={}",
            principal, rate, term_months, installment
        );

        Ok(installment)
    }

    /// Calculate accrued interest over a date range with business day awareness
    async fn calculate_accrued_interest(
        &self,
        account_id: Uuid,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> BankingResult<Decimal> {
        let account_model = self.account_repository
            .find_by_id(account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(account_id))?;

        let account = AccountMapper::from_model(account_model)?;

        let mut total_accrued = Decimal::ZERO;
        let mut current_date = from_date;

        // Get product rules to determine accrual frequency
        let product_rules = self.product_catalog_client.get_product_rules(account.product_code.as_str()).await?;

        while current_date <= to_date {
            // Check if we should accrue interest on this date
            let should_accrue = match product_rules.accrual_frequency {
                crate::integration::AccrualFrequency::Daily => true,
                crate::integration::AccrualFrequency::BusinessDaysOnly => {
                    self.calendar_service.is_business_day(current_date, account.currency.as_str()).await?
                }
                crate::integration::AccrualFrequency::None => false,
            };

            if should_accrue {
                let daily_interest = self.calculate_historical_daily_interest(&account, current_date).await?;
                total_accrued += daily_interest;
            }

            current_date += chrono::Duration::days(1);
        }

        tracing::debug!(
            "Accrued interest calculated for account {} from {} to {}: {}",
            account_id, from_date, to_date, total_accrued
        );

        Ok(total_accrued)
    }

    /// Determine if interest should be posted on a specific date
    async fn should_post_interest(&self, account_id: Uuid, date: NaiveDate) -> BankingResult<bool> {
        let account_model = self.account_repository
            .find_by_id(account_id)
            .await?
            .ok_or(banking_api::BankingError::AccountNotFound(account_id))?;

        let account = AccountMapper::from_model(account_model)?;

        // Get product rules
        let product_rules = self.product_catalog_client.get_product_rules(account.product_code.as_str()).await?;

        // Check posting frequency
        match product_rules.interest_posting_frequency {
            crate::integration::PostingFrequency::Daily => Ok(true),
            crate::integration::PostingFrequency::Weekly => {
                use chrono::Datelike;
                // Post on Fridays or last business day of week
                Ok(date.weekday() == chrono::Weekday::Fri || 
                   self.is_last_business_day_of_week(date).await?)
            }
            crate::integration::PostingFrequency::Monthly => {
                // Post on last business day of month
                self.is_last_business_day_of_month(date).await
            }
            crate::integration::PostingFrequency::Quarterly => {
                // Post on last business day of quarter
                self.is_last_business_day_of_quarter(date).await
            }
            crate::integration::PostingFrequency::Annually => {
                // Post on last business day of year
                self.is_last_business_day_of_year(date).await
            }
        }
    }

    /// Accrue daily interest for all interest-bearing accounts
    async fn accrue_daily_interest(&self, _processing_date: NaiveDate) -> BankingResult<banking_api::service::AccrualReport> {
        todo!("Implement daily interest accrual")
    }

    /// Capitalize accrued interest into account balance
    async fn capitalize_interest(&self, _processing_date: NaiveDate) -> BankingResult<banking_api::service::CapitalizationReport> {
        todo!("Implement interest capitalization")
    }

    /// Calculate interest rate for an account based on balance tiers
    async fn calculate_interest_rate(&self, _product_code: &str, _balance: rust_decimal::Decimal, _account_type: banking_api::domain::AccountType) -> BankingResult<rust_decimal::Decimal> {
        todo!("Implement interest rate calculation")
    }

    /// Get interest rate tiers for a product
    async fn get_interest_rate_tiers(&self, _product_code: &str) -> BankingResult<Vec<banking_api::service::InterestRateTier>> {
        todo!("Implement get_interest_rate_tiers")
    }

    /// Check if account should accrue interest
    async fn should_accrue_interest(&self, _account_id: Uuid, _processing_date: NaiveDate) -> BankingResult<bool> {
        todo!("Implement should accrue interest check")
    }
}

impl InterestServiceImpl {
    /// Calculate daily interest for savings accounts with tiered rates
    async fn calculate_savings_daily_interest(&self, account: &banking_api::domain::Account) -> BankingResult<Decimal> {
        if account.current_balance <= Decimal::ZERO {
            return Ok(Decimal::ZERO);
        }

        // Get tiered interest rate based on balance
        let interest_rate = self.get_tiered_savings_rate(account.product_code.as_str(), account.current_balance).await?;

        // Calculate simple daily interest: (Balance * Rate) / 365
        let daily_interest = (account.current_balance * interest_rate) / Decimal::from(365);

        Ok(daily_interest)
    }

    /// Calculate daily interest for loan accounts
    async fn calculate_loan_daily_interest(&self, account: &banking_api::domain::Account) -> BankingResult<Decimal> {
        let outstanding_principal = account.outstanding_principal.unwrap_or(Decimal::ZERO);
        
        if outstanding_principal <= Decimal::ZERO {
            return Ok(Decimal::ZERO);
        }

        let loan_rate = account.loan_interest_rate.unwrap_or(Decimal::ZERO);

        // Calculate daily interest on outstanding principal
        let daily_interest = (outstanding_principal * loan_rate) / Decimal::from(365);

        Ok(daily_interest)
    }

    /// Calculate daily overdraft interest for current accounts
    async fn calculate_overdraft_daily_interest(&self, account: &banking_api::domain::Account) -> BankingResult<Decimal> {
        if account.current_balance >= Decimal::ZERO {
            return Ok(Decimal::ZERO);
        }

        let overdraft_amount = account.current_balance.abs();
        
        // Get overdraft interest rate from product catalog
        let product_rules = self.product_catalog_client.get_product_rules(account.product_code.as_str()).await?;
        let overdraft_rate = product_rules.overdraft_interest_rate.unwrap_or(Decimal::ZERO);

        // Calculate daily overdraft interest
        let daily_interest = (overdraft_amount * overdraft_rate) / Decimal::from(365);

        Ok(daily_interest)
    }

    /// Get tiered savings rate based on balance
    async fn get_tiered_savings_rate(&self, product_code: &str, balance: Decimal) -> BankingResult<Decimal> {
        let rate_tiers = self.product_catalog_client.get_interest_rate_tiers(product_code).await?;

        // Find applicable tier (start from highest tier)
        for tier in rate_tiers.iter().rev() {
            if balance >= tier.minimum_balance {
                return Ok(tier.interest_rate);
            }
        }

        // Return base rate if no tier matches
        Ok(rate_tiers.first()
            .map(|t| t.interest_rate)
            .unwrap_or(Decimal::ZERO))
    }

    /// Calculate historical daily interest for a specific date
    async fn calculate_historical_daily_interest(&self, account: &banking_api::domain::Account, _date: NaiveDate) -> BankingResult<Decimal> {
        // In production, this would get the balance as of the specific date
        // For now, use current balance
        self.calculate_daily_interest(account.id).await
    }

    /// Generate reference number for interest transactions
    async fn generate_interest_reference(&self, account: &banking_api::domain::Account, date: NaiveDate) -> BankingResult<String> {
        Ok(format!(
            "INT_{}_{}",
            account.id.to_string().replace('-', "")[..8].to_uppercase(),
            date.format("%Y%m%d")
        ))
    }

    /// Get GL code for interest transactions
    async fn get_interest_gl_code(&self, product_code: &str) -> BankingResult<String> {
        // In production, this would come from product catalog GL mapping
        Ok(format!("INT_{product_code}"))
    }

    /// Power function for Decimal (simple implementation)
    fn decimal_power(&self, base: Decimal, exponent: i32) -> BankingResult<Decimal> {
        if exponent == 0 {
            return Ok(Decimal::ONE);
        }

        let mut result = base;
        for _ in 1..exponent.abs() {
            result *= base;
        }

        if exponent < 0 {
            result = Decimal::ONE / result;
        }

        Ok(result)
    }

    /// Check if date is last business day of week
    async fn is_last_business_day_of_week(&self, date: NaiveDate) -> BankingResult<bool> {
        use chrono::Datelike;
        let mut next_day = date + chrono::Duration::days(1);
        while next_day.weekday() != chrono::Weekday::Mon {
            if self.calendar_service.is_business_day(next_day, "USD").await? {
                return Ok(false); // Found another business day in the same week
            }
            next_day += chrono::Duration::days(1);
        }
        Ok(true)
    }

    /// Check if date is last business day of month
    async fn is_last_business_day_of_month(&self, date: NaiveDate) -> BankingResult<bool> {
        let next_month = if date.month() == 12 {
            chrono::NaiveDate::from_ymd_opt(date.year() + 1, 1, 1)
        } else {
            chrono::NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1)
        }.ok_or(banking_api::BankingError::DateCalculationError(format!("Invalid date: {date}")))?;

        use chrono::Datelike;
        let mut check_date = date + chrono::Duration::days(1);
        while check_date < next_month {
            if self.calendar_service.is_business_day(check_date, "USD").await? {
                return Ok(false); // Found another business day in the same month
            }
            check_date += chrono::Duration::days(1);
        }
        Ok(true)
    }

    /// Check if date is last business day of quarter
    async fn is_last_business_day_of_quarter(&self, date: NaiveDate) -> BankingResult<bool> {
        use chrono::Datelike;
        let quarter_end = match date.month() {
            1..=3 => NaiveDate::from_ymd_opt(date.year(), 3, 31),
            4..=6 => NaiveDate::from_ymd_opt(date.year(), 6, 30),
            7..=9 => NaiveDate::from_ymd_opt(date.year(), 9, 30),
            10..=12 => NaiveDate::from_ymd_opt(date.year(), 12, 31),
            _ => None,
        }.ok_or(banking_api::BankingError::DateCalculationError(format!("Invalid date: {date}")))?;

        self.is_last_business_day_up_to_date(date, quarter_end).await
    }

    /// Check if date is last business day of year
    async fn is_last_business_day_of_year(&self, date: NaiveDate) -> BankingResult<bool> {
        use chrono::Datelike;
        let year_end = NaiveDate::from_ymd_opt(date.year(), 12, 31)
            .ok_or(banking_api::BankingError::DateCalculationError(format!("Invalid date: {date}")))?;

        self.is_last_business_day_up_to_date(date, year_end).await
    }

    /// Helper function to check if date is last business day up to end_date
    async fn is_last_business_day_up_to_date(&self, date: NaiveDate, end_date: NaiveDate) -> BankingResult<bool> {
        let mut check_date = date + chrono::Duration::days(1);
        while check_date <= end_date {
            if self.calendar_service.is_business_day(check_date, "USD").await? {
                return Ok(false); // Found another business day
            }
            check_date += chrono::Duration::days(1);
        }
        Ok(true)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_calculate_loan_installment() {
        let mock_account_repo = Arc::new(MockAccountRepository);
        let mock_transaction_repo = Arc::new(MockTransactionRepository);
        let mock_product_client = Arc::new(ProductCatalogClient::new("http://localhost".to_string()).unwrap());
        let mock_calendar = Arc::new(MockCalendarService);

        let service = InterestServiceImpl::new(
            mock_account_repo,
            mock_transaction_repo,
            mock_product_client,
            mock_calendar,
        );

        // Test standard loan calculation
        let principal = Decimal::new(100000, 2); // $1,000.00
        let rate = Decimal::new(500, 4); // 5%
        let term = 12; // 12 months

        let installment = service.calculate_loan_installment(principal, rate, term).await.unwrap();

        // Expected installment should be approximately $85.61
        assert!(installment > Decimal::new(8500, 2) && installment < Decimal::new(8600, 2));
    }

    // Mock implementations for testing
    struct MockAccountRepository;
    struct MockTransactionRepository;
    struct MockCalendarService;

    #[async_trait]
    impl AccountRepository for MockAccountRepository {
        async fn find_by_id(&self, _account_id: Uuid) -> BankingResult<Option<banking_db::models::AccountModel>> {
            Ok(None)
        }
        async fn update_balance(&self, _account_id: Uuid, _current_balance: Decimal, _available_balance: Decimal) -> BankingResult<()> { Ok(()) }
        async fn reset_accrued_interest(&self, _account_id: Uuid) -> BankingResult<()> { Ok(()) }
        async fn exists(&self, _account_id: Uuid) -> BankingResult<bool> { Ok(true) }
        
        // Add all other required methods with todo!()
        async fn create(&self, _account: banking_db::models::AccountModel) -> BankingResult<banking_db::models::AccountModel> { todo!() }
        async fn update(&self, _account: banking_db::models::AccountModel) -> BankingResult<banking_db::models::AccountModel> { todo!() }
        async fn find_by_customer_id(&self, _customer_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountModel>> { todo!() }
        async fn find_by_product_code(&self, _product_code: &str) -> BankingResult<Vec<banking_db::models::AccountModel>> { todo!() }
        async fn find_by_status(&self, _status: &str) -> BankingResult<Vec<banking_db::models::AccountModel>> { todo!() }
        async fn find_dormancy_candidates(&self, _reference_date: chrono::NaiveDate, _threshold_days: i32) -> BankingResult<Vec<banking_db::models::AccountModel>> { todo!() }
        async fn find_pending_closure(&self) -> BankingResult<Vec<banking_db::models::AccountModel>> { todo!() }
        async fn find_interest_bearing_accounts(&self) -> BankingResult<Vec<banking_db::models::AccountModel>> { todo!() }
        async fn update_status(&self, _account_id: Uuid, _status: &str, _reason: &str, _changed_by: Uuid) -> BankingResult<()> { todo!() }
        async fn update_accrued_interest(&self, _account_id: Uuid, _accrued_interest: Decimal) -> BankingResult<()> { todo!() }
        async fn create_ownership(&self, _ownership: banking_db::models::AccountOwnershipModel) -> BankingResult<banking_db::models::AccountOwnershipModel> { todo!() }
        async fn find_ownership_by_account(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountOwnershipModel>> { todo!() }
        async fn find_accounts_by_owner(&self, _customer_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountOwnershipModel>> { todo!() }
        async fn delete_ownership(&self, _ownership_id: Uuid) -> BankingResult<()> { todo!() }
        async fn create_relationship(&self, _relationship: banking_db::models::AccountRelationshipModel) -> BankingResult<banking_db::models::AccountRelationshipModel> { todo!() }
        async fn find_relationships_by_account(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountRelationshipModel>> { todo!() }
        async fn find_relationships_by_entity(&self, _entity_id: Uuid, _relationship_type: &str) -> BankingResult<Vec<banking_db::models::AccountRelationshipModel>> { todo!() }
        async fn update_relationship(&self, _relationship: banking_db::models::AccountRelationshipModel) -> BankingResult<banking_db::models::AccountRelationshipModel> { todo!() }
        async fn delete_relationship(&self, _relationship_id: Uuid) -> BankingResult<()> { todo!() }
        async fn create_mandate(&self, _mandate: banking_db::models::AccountMandateModel) -> BankingResult<banking_db::models::AccountMandateModel> { todo!() }
        async fn find_mandates_by_account(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountMandateModel>> { todo!() }
        async fn find_mandates_by_grantee(&self, _grantee_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountMandateModel>> { todo!() }
        async fn update_mandate_status(&self, _mandate_id: Uuid, _status: &str) -> BankingResult<()> { todo!() }
        async fn find_active_mandates(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountMandateModel>> { todo!() }
        async fn create_hold(&self, _hold: banking_db::models::AccountHoldModel) -> BankingResult<banking_db::models::AccountHoldModel> { todo!() }
        async fn find_holds_by_account(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> { todo!() }
        async fn find_active_holds(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> { todo!() }
        async fn release_hold(&self, _hold_id: Uuid, _released_by: Uuid) -> BankingResult<()> { todo!() }
        async fn release_expired_holds(&self, _expiry_cutoff: chrono::DateTime<chrono::Utc>) -> BankingResult<i64> { todo!() }
        async fn create_final_settlement(&self, _settlement: banking_db::models::AccountFinalSettlementModel) -> BankingResult<banking_db::models::AccountFinalSettlementModel> { todo!() }
        async fn find_settlement_by_account(&self, _account_id: Uuid) -> BankingResult<Option<banking_db::models::AccountFinalSettlementModel>> { todo!() }
        async fn update_settlement_status(&self, _settlement_id: Uuid, _status: &str) -> BankingResult<()> { todo!() }
        async fn get_status_history(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountStatusHistoryModel>> { todo!() }
        async fn add_status_change(&self, _status_change: banking_db::models::AccountStatusHistoryModel) -> BankingResult<banking_db::models::AccountStatusHistoryModel> { todo!() }
        async fn count_by_customer(&self, _customer_id: Uuid) -> BankingResult<i64> { todo!() }
        async fn count_by_product(&self, _product_code: &str) -> BankingResult<i64> { todo!() }
        async fn list(&self, _limit: i64, _offset: i64) -> BankingResult<Vec<banking_db::models::AccountModel>> { todo!() }
        async fn count(&self) -> BankingResult<i64> { todo!() }
        async fn update_last_activity_date(&self, _account_id: Uuid, _activity_date: chrono::NaiveDate) -> BankingResult<()> { todo!() }

        // Hold-related methods
        async fn update_hold(&self, _hold: banking_db::models::AccountHoldModel) -> BankingResult<banking_db::models::AccountHoldModel> { todo!() }
        async fn get_hold_by_id(&self, _hold_id: Uuid) -> BankingResult<Option<banking_db::models::AccountHoldModel>> { todo!() }
        async fn get_active_holds_for_account(&self, _account_id: Uuid, _hold_types: Option<Vec<String>>) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> { todo!() }
        async fn get_holds_by_status(&self, _account_id: Option<Uuid>, _status: String, _from_date: Option<chrono::NaiveDate>, _to_date: Option<chrono::NaiveDate>) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> { todo!() }
        async fn get_holds_by_type(&self, _hold_type: String, _status: Option<String>, _account_ids: Option<Vec<Uuid>>, _limit: Option<i32>) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> { todo!() }
        async fn get_hold_history(&self, _account_id: Uuid, _from_date: Option<chrono::NaiveDate>, _to_date: Option<chrono::NaiveDate>, _include_released: bool) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> { todo!() }
        async fn calculate_total_holds(&self, _account_id: Uuid, _exclude_types: Option<Vec<String>>) -> BankingResult<rust_decimal::Decimal> { todo!() }
        async fn get_hold_amounts_by_priority(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::repository::HoldPrioritySummary>> { todo!() }
        async fn get_hold_breakdown(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::repository::HoldTypeSummary>> { todo!() }
        async fn cache_balance_calculation(&self, _calculation: banking_db::models::AccountBalanceCalculationModel) -> BankingResult<banking_db::models::AccountBalanceCalculationModel> { todo!() }
        async fn get_cached_balance_calculation(&self, _account_id: Uuid, _max_age_seconds: u64) -> BankingResult<Option<banking_db::models::AccountBalanceCalculationModel>> { todo!() }
        async fn release_hold_detailed(&self, _hold_id: Uuid, _release_amount: Option<rust_decimal::Decimal>, _release_reason_id: Uuid, _released_by: Uuid, _released_at: chrono::DateTime<chrono::Utc>) -> BankingResult<banking_db::models::AccountHoldModel> { todo!() }
        async fn create_hold_release_record(&self, _release_record: banking_db::models::HoldReleaseRecordModel) -> BankingResult<banking_db::models::HoldReleaseRecordModel> { todo!() }
        async fn get_hold_release_records(&self, _hold_id: Uuid) -> BankingResult<Vec<banking_db::models::HoldReleaseRecordModel>> { todo!() }
        async fn bulk_release_holds(&self, _hold_ids: Vec<Uuid>, _release_reason_id: Uuid, _released_by: Uuid) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> { todo!() }
        async fn get_expired_holds(&self, _cutoff_date: chrono::DateTime<chrono::Utc>, _hold_types: Option<Vec<String>>, _limit: Option<i32>) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> { todo!() }
        async fn get_auto_release_eligible_holds(&self, _processing_date: chrono::NaiveDate, _hold_types: Option<Vec<String>>) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> { todo!() }
        async fn create_hold_expiry_job(&self, _expiry_job: banking_db::models::AccountHoldExpiryJobModel) -> BankingResult<banking_db::models::AccountHoldExpiryJobModel> { todo!() }
        async fn update_hold_expiry_job(&self, _expiry_job: banking_db::models::AccountHoldExpiryJobModel) -> BankingResult<banking_db::models::AccountHoldExpiryJobModel> { todo!() }
        async fn bulk_place_holds(&self, _holds: Vec<banking_db::models::AccountHoldModel>) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> { todo!() }
        async fn update_hold_priorities(&self, _account_id: Uuid, _priority_updates: Vec<(Uuid, String)>, _updated_by_person_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> { todo!() }
        async fn get_overrideable_holds(&self, _account_id: Uuid, _required_amount: rust_decimal::Decimal, _override_reason: String) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> { todo!() }
        async fn create_hold_override(&self, _account_id: Uuid, _hold_ids: Vec<Uuid>, _override_amount: rust_decimal::Decimal, _authorized_by: Uuid, _override_reason_id: Uuid) -> BankingResult<banking_db::repository::HoldOverrideRecord> { todo!() }
        async fn get_judicial_holds_by_reference(&self, _source_reference: String) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> { todo!() }
        async fn update_loan_pledge_holds(&self, _collateral_id: Uuid, _affected_accounts: Vec<Uuid>, _new_pledge_amount: rust_decimal::Decimal, _updated_by_person_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> { todo!() }
        async fn get_compliance_holds_by_alert(&self, _alert_id: Uuid) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> { todo!() }
        async fn get_hold_analytics(&self, _from_date: chrono::NaiveDate, _to_date: chrono::NaiveDate, _product_codes: Option<Vec<String>>) -> BankingResult<banking_db::repository::HoldAnalyticsSummary> { todo!() }
        async fn get_high_hold_ratio_accounts(&self, _threshold_percentage: rust_decimal::Decimal, _product_codes: Option<Vec<String>>, _limit: i32) -> BankingResult<Vec<banking_db::repository::HighHoldRatioAccount>> { todo!() }
        async fn generate_judicial_hold_report(&self, _from_date: chrono::NaiveDate, _to_date: chrono::NaiveDate) -> BankingResult<banking_db::repository::JudicialHoldReportData> { todo!() }
        async fn get_hold_aging_report(&self, _product_codes: Option<Vec<String>>, _aging_buckets: Vec<i32>) -> BankingResult<Vec<banking_db::repository::HoldAgingBucket>> { todo!() }
        async fn validate_hold_amounts(&self, _account_id: Uuid) -> BankingResult<Vec<banking_db::repository::HoldValidationError>> { todo!() }
        async fn find_orphaned_holds(&self, _max_age_days: Option<i32>) -> BankingResult<Vec<banking_db::models::AccountHoldModel>> { todo!() }
        async fn cleanup_old_holds(&self, _cutoff_date: chrono::NaiveDate, _statuses_to_clean: Vec<String>) -> BankingResult<u32> { todo!() }
    }

    #[async_trait]
    impl TransactionRepository for MockTransactionRepository {
        async fn create(&self, transaction: banking_db::models::TransactionModel) -> BankingResult<banking_db::models::TransactionModel> {
            Ok(transaction)
        }
        async fn update(&self, transaction: banking_db::models::TransactionModel) -> BankingResult<banking_db::models::TransactionModel> {
            Ok(transaction)
        }
        async fn find_by_id(&self, _transaction_id: Uuid) -> BankingResult<Option<banking_db::models::TransactionModel>> {
            Ok(None)
        }
        async fn find_by_account_id(&self, _account_id: Uuid, _from_date: Option<chrono::NaiveDate>, _to_date: Option<chrono::NaiveDate>) -> BankingResult<Vec<banking_db::models::TransactionModel>> {
            Ok(Vec::new())
        }
        async fn find_by_account_date_range(&self, _account_id: Uuid, _from: NaiveDate, _to: NaiveDate) -> BankingResult<Vec<banking_db::models::TransactionModel>> {
            Ok(Vec::new())
        }
        async fn find_by_reference(&self, _reference_number: &str) -> BankingResult<Option<banking_db::models::TransactionModel>> {
            Ok(None)
        }
        async fn find_by_external_reference(&self, _external_reference: &str) -> BankingResult<Vec<banking_db::models::TransactionModel>> {
            Ok(Vec::new())
        }
        async fn find_by_status(&self, _status: &str) -> BankingResult<Vec<banking_db::models::TransactionModel>> {
            Ok(Vec::new())
        }
        async fn find_requiring_approval(&self) -> BankingResult<Vec<banking_db::models::TransactionModel>> {
            Ok(Vec::new())
        }
        async fn find_by_terminal_id(&self, _terminal_id: Uuid, _from_date: Option<chrono::NaiveDate>, _to_date: Option<chrono::NaiveDate>) -> BankingResult<Vec<banking_db::models::TransactionModel>> {
            Ok(Vec::new())
        }
        async fn find_by_agent_user_id(&self, _agent_user_id: Uuid, _from_date: Option<chrono::NaiveDate>, _to_date: Option<chrono::NaiveDate>) -> BankingResult<Vec<banking_db::models::TransactionModel>> {
            Ok(Vec::new())
        }
        async fn find_by_channel(&self, _channel_id: &str, _from_date: Option<chrono::NaiveDate>, _to_date: Option<chrono::NaiveDate>) -> BankingResult<Vec<banking_db::models::TransactionModel>> {
            Ok(Vec::new())
        }
        async fn update_status(&self, _transaction_id: Uuid, _status: &str, _reason: &str) -> BankingResult<()> {
            Ok(())
        }
        async fn update_approval_status(&self, _transaction_id: Uuid, _approval_status: &str) -> BankingResult<()> {
            Ok(())
        }
        async fn find_last_customer_transaction(&self, _account_id: Uuid) -> BankingResult<Option<banking_db::models::TransactionModel>> {
            Ok(None)
        }
        async fn calculate_daily_volume_by_terminal(&self, _terminal_id: Uuid, _date: chrono::NaiveDate) -> BankingResult<rust_decimal::Decimal> {
            Ok(rust_decimal::Decimal::ZERO)
        }
        async fn calculate_daily_volume_by_branch(&self, _branch_id: Uuid, _date: chrono::NaiveDate) -> BankingResult<rust_decimal::Decimal> {
            Ok(rust_decimal::Decimal::ZERO)
        }
        async fn calculate_daily_volume_by_network(&self, _network_id: Uuid, _date: chrono::NaiveDate) -> BankingResult<rust_decimal::Decimal> {
            Ok(rust_decimal::Decimal::ZERO)
        }
        async fn reverse_transaction(&self, _original_transaction_id: Uuid, reversal_transaction: banking_db::models::TransactionModel) -> BankingResult<banking_db::models::TransactionModel> {
            Ok(reversal_transaction)
        }
        async fn find_for_reconciliation(&self, _channel_id: &str, _date: chrono::NaiveDate) -> BankingResult<Vec<banking_db::models::TransactionModel>> {
            Ok(Vec::new())
        }
        async fn create_workflow(&self, workflow: banking_db::models::ApprovalWorkflowModel) -> BankingResult<banking_db::models::ApprovalWorkflowModel> {
            Ok(workflow)
        }
        async fn find_workflow_by_id(&self, _id: Uuid) -> BankingResult<Option<banking_db::models::ApprovalWorkflowModel>> {
            Ok(None)
        }
        async fn find_workflow_by_transaction(&self, _transaction_id: Uuid) -> BankingResult<Option<banking_db::models::ApprovalWorkflowModel>> {
            Ok(None)
        }
        async fn update_workflow_status(&self, _id: Uuid, _status: &str) -> BankingResult<()> {
            Ok(())
        }
        async fn find_pending_workflows(&self) -> BankingResult<Vec<banking_db::models::ApprovalWorkflowModel>> {
            Ok(Vec::new())
        }
        async fn find_expired_workflows(&self, _reference_time: chrono::DateTime<Utc>) -> BankingResult<Vec<banking_db::models::ApprovalWorkflowModel>> {
            Ok(Vec::new())
        }
        async fn create_approval(&self, approval: banking_db::models::workflow::WorkflowTransactionApprovalModel) -> BankingResult<banking_db::models::workflow::WorkflowTransactionApprovalModel> {
            Ok(approval)
        }
        async fn find_approvals_by_workflow(&self, _id: Uuid) -> BankingResult<Vec<banking_db::models::workflow::WorkflowTransactionApprovalModel>> {
            Ok(Vec::new())
        }
        async fn find_approvals_by_approver(&self, _approver_id: Uuid) -> BankingResult<Vec<banking_db::models::workflow::WorkflowTransactionApprovalModel>> {
            Ok(Vec::new())
        }
        async fn count_approvals_for_workflow(&self, _id: Uuid) -> BankingResult<i64> {
            Ok(0)
        }
        async fn exists(&self, _transaction_id: Uuid) -> BankingResult<bool> {
            Ok(false)
        }
        async fn count_by_account(&self, _account_id: Uuid, _from_date: Option<chrono::NaiveDate>, _to_date: Option<chrono::NaiveDate>) -> BankingResult<i64> {
            Ok(0)
        }
        async fn list(&self, _offset: i64, _limit: i64) -> BankingResult<Vec<banking_db::models::TransactionModel>> {
            Ok(Vec::new())
        }
        async fn count(&self) -> BankingResult<i64> {
            Ok(0)
        }
    }

    #[async_trait]
    impl CalendarService for MockCalendarService {
        async fn is_business_day(&self, _date: NaiveDate, _jurisdiction: &str) -> BankingResult<bool> {
            Ok(true)
        }
        async fn next_business_day(&self, date: NaiveDate, _jurisdiction: &str) -> BankingResult<NaiveDate> {
            Ok(date + chrono::Duration::days(1))
        }
        async fn previous_business_day(&self, date: NaiveDate, _jurisdiction: &str) -> BankingResult<NaiveDate> {
            Ok(date - chrono::Duration::days(1))
        }
        async fn add_business_days(&self, date: NaiveDate, days: i32, _jurisdiction: &str) -> BankingResult<NaiveDate> {
            Ok(date + chrono::Duration::days(days as i64))
        }
        async fn count_business_days(&self, _from: NaiveDate, _to: NaiveDate, _jurisdiction: &str) -> BankingResult<i32> {
            Ok(1)
        }
        async fn add_bank_holiday(&self, _holiday: banking_api::domain::BankHoliday) -> BankingResult<()> {
            Ok(())
        }
        async fn remove_bank_holiday(&self, _holiday_id: uuid::Uuid) -> BankingResult<()> {
            Ok(())
        }
        async fn get_holidays(&self, _jurisdiction: &str, _year: i32) -> BankingResult<Vec<banking_api::domain::BankHoliday>> {
            Ok(Vec::new())
        }
        async fn calculate_business_day(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<banking_api::domain::BusinessDayCalculation> {
            Ok(banking_api::domain::BusinessDayCalculation {
                requested_date: date,
                adjusted_date: date,
                is_business_day: true,
                applied_rule: banking_api::domain::DateShiftRule::NoShift,
                jurisdiction: heapless::String::try_from(jurisdiction).unwrap_or_default(),
            })
        }
        async fn batch_calculate_business_days(&self, dates: Vec<NaiveDate>, jurisdiction: &str) -> BankingResult<Vec<banking_api::domain::BusinessDayCalculation>> {
            Ok(dates.into_iter().map(|date| banking_api::domain::BusinessDayCalculation {
                requested_date: date,
                adjusted_date: date,
                is_business_day: true,
                applied_rule: banking_api::domain::DateShiftRule::NoShift,
                jurisdiction: heapless::String::try_from(jurisdiction).unwrap_or_default(),
            }).collect())
        }
        async fn is_weekend(&self, _date: NaiveDate, _jurisdiction: &str) -> BankingResult<bool> {
            Ok(false)
        }
        async fn get_weekend_days(&self, _jurisdiction: &str) -> BankingResult<Vec<chrono::Weekday>> {
            Ok(vec![chrono::Weekday::Sat, chrono::Weekday::Sun])
        }
    }
}
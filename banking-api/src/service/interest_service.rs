use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::error::BankingResult;

#[async_trait]
pub trait InterestService: Send + Sync {
    /// Calculate daily interest for an account
    async fn calculate_daily_interest(&self, account_id: Uuid) -> BankingResult<Decimal>;
    
    /// Post periodic interest to account balance
    async fn post_periodic_interest(&self, account_id: Uuid) -> BankingResult<()>;
    
    /// Calculate loan installment amount
    async fn calculate_loan_installment(&self, principal: Decimal, rate: Decimal, term_months: i32) -> BankingResult<Decimal>;
    
    /// Business day aware processing
    async fn calculate_accrued_interest(&self, account_id: Uuid, from_date: NaiveDate, to_date: NaiveDate) -> BankingResult<Decimal>;
    async fn should_post_interest(&self, account_id: Uuid, date: NaiveDate) -> BankingResult<bool>;

    /// Daily interest accrual for EOD processing
    async fn accrue_daily_interest(&self, processing_date: NaiveDate) -> BankingResult<AccrualReport>;

    /// Interest capitalization for eligible accounts
    async fn capitalize_interest(&self, processing_date: NaiveDate) -> BankingResult<CapitalizationReport>;

    /// Calculate interest for specific account type and balance
    async fn calculate_interest_rate(&self, product_code: &str, balance: Decimal, account_type: crate::domain::AccountType) -> BankingResult<Decimal>;

    /// Get interest rate tiers for a product
    async fn get_interest_rate_tiers(&self, product_code: &str) -> BankingResult<Vec<InterestRateTier>>;

    /// Check if account should accrue interest on given date
    async fn should_accrue_interest(&self, account_id: Uuid, date: NaiveDate) -> BankingResult<bool>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccrualReport {
    pub processing_date: NaiveDate,
    pub accounts_processed: i64,
    pub total_interest_accrued: Decimal,
    pub account_accruals: Vec<AccountAccrual>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccountAccrual {
    pub account_id: Uuid,
    pub daily_interest: Decimal,
    pub interest_rate: Decimal,
    pub principal_balance: Decimal,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CapitalizationReport {
    pub processing_date: NaiveDate,
    pub accounts_processed: i64,
    pub total_interest_capitalized: Decimal,
    pub capitalizations: Vec<CapitalizationResult>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CapitalizationResult {
    pub account_id: Uuid,
    pub capitalization_date: NaiveDate,
    pub amount_capitalized: Decimal,
    pub transaction_id: Uuid,
    pub new_balance: Decimal,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InterestRateTier {
    pub minimum_balance: Decimal,
    pub interest_rate: Decimal,
    pub tier_name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CalculationMethod {
    DailyBalance,
    AverageDailyBalance,
    CompoundDaily,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AccrualFrequency {
    Daily,
    BusinessDaysOnly,
    None,
}
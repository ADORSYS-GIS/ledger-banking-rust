use async_trait::async_trait;
use chrono::NaiveDate;

use crate::{
    domain::{BankHoliday, BusinessDayCalculation},
    error::BankingResult,
};

#[async_trait]
pub trait CalendarService: Send + Sync {
    /// Check if a date is a business day
    async fn is_business_day(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<bool>;
    
    /// Get next business day
    async fn next_business_day(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<NaiveDate>;
    
    /// Get previous business day
    async fn previous_business_day(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<NaiveDate>;
    
    /// Add business days to a date
    async fn add_business_days(&self, date: NaiveDate, days: i32, jurisdiction: &str) -> BankingResult<NaiveDate>;
    
    /// Count business days between two dates
    async fn count_business_days(&self, from: NaiveDate, to: NaiveDate, jurisdiction: &str) -> BankingResult<i32>;

    /// Add a bank holiday
    async fn add_bank_holiday(&self, holiday: BankHoliday) -> BankingResult<()>;

    /// Remove a bank holiday
    async fn remove_bank_holiday(&self, holiday_id: uuid::Uuid) -> BankingResult<()>;

    /// Get all holidays for a jurisdiction and year
    async fn get_holidays(&self, jurisdiction: &str, year: i32) -> BankingResult<Vec<BankHoliday>>;

    /// Calculate business day with rule application
    async fn calculate_business_day(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<BusinessDayCalculation>;

    /// Batch business day calculations for performance
    async fn batch_calculate_business_days(&self, dates: Vec<NaiveDate>, jurisdiction: &str) -> BankingResult<Vec<BusinessDayCalculation>>;

    /// Check if date falls on weekend
    async fn is_weekend(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<bool>;

    /// Get weekend days for jurisdiction
    async fn get_weekend_days(&self, jurisdiction: &str) -> BankingResult<Vec<chrono::Weekday>>;
}
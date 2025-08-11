use async_trait::async_trait;
use banking_api::BankingResult;
use uuid::Uuid;
use chrono::NaiveDate;

use crate::models::{BankHolidayModel};

#[async_trait]
pub trait CalendarRepository: Send + Sync {
    /// Bank Holiday Operations
    async fn create_holiday(&self, holiday: BankHolidayModel) -> BankingResult<BankHolidayModel>;
    async fn update_holiday(&self, holiday: BankHolidayModel) -> BankingResult<BankHolidayModel>;
    async fn find_holiday_by_id(&self, holiday_id: Uuid) -> BankingResult<Option<BankHolidayModel>>;
    async fn find_holiday_by_date(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<Option<BankHolidayModel>>;
    async fn find_holidays_by_jurisdiction(&self, jurisdiction: &str) -> BankingResult<Vec<BankHolidayModel>>;
    async fn find_holidays_by_type(&self, holiday_type: &str) -> BankingResult<Vec<BankHolidayModel>>;
    async fn find_holidays_in_range(&self, start_date: NaiveDate, end_date: NaiveDate, jurisdiction: &str) -> BankingResult<Vec<BankHolidayModel>>;
    async fn find_holidays_by_year(&self, year: i32, jurisdiction: &str) -> BankingResult<Vec<BankHolidayModel>>;
    async fn delete_holiday(&self, holiday_id: Uuid) -> BankingResult<()>;
    
    /// Business Day Calculations
    async fn is_holiday(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<bool>;
    async fn is_weekend(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<bool>;
    async fn is_business_day(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<bool>;
    
    /// Date Navigation
    async fn next_business_day(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<NaiveDate>;
    async fn previous_business_day(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<NaiveDate>;
    async fn add_business_days(&self, date: NaiveDate, days: i32, jurisdiction: &str) -> BankingResult<NaiveDate>;
    async fn subtract_business_days(&self, date: NaiveDate, days: i32, jurisdiction: &str) -> BankingResult<NaiveDate>;
    async fn count_business_days(&self, start_date: NaiveDate, end_date: NaiveDate, jurisdiction: &str) -> BankingResult<i32>;
    
    /// Weekend Configuration
    async fn get_weekend_days(&self, jurisdiction: &str) -> BankingResult<Vec<chrono::Weekday>>;
    async fn set_weekend_days(&self, jurisdiction: &str, weekend_days: Vec<chrono::Weekday>) -> BankingResult<()>;
    
    /// Recurring Holiday Management
    async fn find_recurring_holidays(&self, jurisdiction: &str) -> BankingResult<Vec<BankHolidayModel>>;
    async fn generate_recurring_holidays(&self, year: i32, jurisdiction: &str) -> BankingResult<Vec<BankHolidayModel>>;
    async fn create_recurring_holidays_for_year(&self, year: i32, jurisdiction: &str) -> BankingResult<i64>;
    
    /// Date Shift Rule Operations
    async fn apply_date_shift_rule(&self, date: NaiveDate, jurisdiction: &str, shift_rule: &str) -> BankingResult<NaiveDate>;
    async fn get_maturity_date(&self, start_date: NaiveDate, term_months: i32, jurisdiction: &str) -> BankingResult<NaiveDate>;
    async fn get_payment_due_date(&self, original_date: NaiveDate, jurisdiction: &str, product_id: Option<Uuid>) -> BankingResult<NaiveDate>;
    
    /// Calendar Maintenance
    async fn bulk_create_holidays(&self, holidays: Vec<BankHolidayModel>) -> BankingResult<i64>;
    async fn delete_holidays_by_year(&self, year: i32, jurisdiction: &str) -> BankingResult<i64>;
    async fn cleanup_past_holidays(&self, before_date: NaiveDate) -> BankingResult<i64>;
    
    /// Jurisdiction Management
    async fn get_supported_jurisdictions(&self) -> BankingResult<Vec<String>>;
    async fn add_jurisdiction(&self, jurisdiction: &str, weekend_days: Vec<chrono::Weekday>) -> BankingResult<()>;
    async fn remove_jurisdiction(&self, jurisdiction: &str) -> BankingResult<()>;
    
    /// Holiday Validation and Import
    async fn validate_holiday_data(&self, holidays: Vec<BankHolidayModel>) -> BankingResult<ValidationResult>;
    async fn import_holidays_from_source(&self, jurisdiction: &str, year: i32, source: &str) -> BankingResult<ImportResult>;
    
    /// Reporting and Analytics
    async fn get_calendar_summary(&self, jurisdiction: &str, year: i32) -> BankingResult<CalendarSummaryReport>;
    async fn get_business_days_in_month(&self, year: i32, month: u32, jurisdiction: &str) -> BankingResult<i32>;
    async fn get_business_days_in_year(&self, year: i32, jurisdiction: &str) -> BankingResult<i32>;
    
    /// Cache Management (for implementation optimization)
    async fn refresh_calendar_cache(&self, jurisdiction: &str) -> BankingResult<()>;
    async fn invalidate_calendar_cache(&self, jurisdiction: &str) -> BankingResult<()>;
    
    /// Utility Operations
    async fn holiday_exists(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<bool>;
    async fn count_holidays(&self, jurisdiction: &str) -> BankingResult<i64>;
    async fn count_holidays_by_type(&self, holiday_type: &str, jurisdiction: &str) -> BankingResult<i64>;
    async fn list_holidays(&self, offset: i64, limit: i64) -> BankingResult<Vec<BankHolidayModel>>;
}

/// Supporting structures for calendar operations
pub struct ValidationResult {
    pub valid_holidays: Vec<BankHolidayModel>,
    pub invalid_holidays: Vec<HolidayValidationError>,
    pub duplicate_holidays: Vec<BankHolidayModel>,
    pub warnings: Vec<String>,
}

pub struct HolidayValidationError {
    pub holiday: BankHolidayModel,
    pub error_message: String,
}

pub struct ImportResult {
    pub jurisdiction: String,
    pub year: i32,
    pub holidays_imported: i64,
    pub holidays_skipped: i64,
    pub errors: Vec<String>,
    pub import_source: String,
}

pub struct CalendarSummaryReport {
    pub jurisdiction: String,
    pub year: i32,
    pub total_days: i32,
    pub total_holidays: i32,
    pub total_weekends: i32,
    pub total_business_days: i32,
    pub holidays_by_type: Vec<HolidayTypeCount>,
    pub holidays_by_month: Vec<MonthlyHolidayCount>,
}

pub struct HolidayTypeCount {
    pub holiday_type: String,
    pub count: i32,
}

pub struct MonthlyHolidayCount {
    pub month: u32,
    pub month_name: String,
    pub holiday_count: i32,
    pub business_days: i32,
}
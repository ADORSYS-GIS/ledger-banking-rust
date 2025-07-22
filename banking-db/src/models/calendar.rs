use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;

/// Bank Holiday database model
#[derive(Debug, Clone)]
pub struct BankHolidayModel {
    pub holiday_id: Uuid,
    pub jurisdiction: String, // Country/region code (e.g., "US", "UK", "CM")
    pub holiday_date: NaiveDate,
    pub holiday_name: String,
    pub holiday_type: String, // National, Religious, Banking, Custom
    pub is_recurring: bool,   // Annual recurrence flag
    pub description: Option<String>,
    pub is_observed: bool,     // Whether the holiday is actively observed
    pub observance_rule: Option<String>, // Special observance rules (JSON)
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: String,
}

/// Weekend Configuration database model
#[derive(Debug, Clone)]
pub struct WeekendConfigurationModel {
    pub config_id: Uuid,
    pub jurisdiction: String,
    pub weekend_days: String, // JSON array of weekday numbers (0=Sunday, 1=Monday, etc.)
    pub effective_date: NaiveDate,
    pub is_active: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: String,
}

/// Date Calculation Rules database model
#[derive(Debug, Clone)]
pub struct DateCalculationRulesModel {
    pub rule_id: Uuid,
    pub jurisdiction: String,
    pub rule_name: String,
    pub rule_type: String, // DateShift, MaturityCalculation, PaymentDue
    pub default_shift_rule: String, // NextBusinessDay, PreviousBusinessDay, NoShift
    pub weekend_treatment: String,  // SaturdaySunday, FridayOnly, Custom
    pub product_specific_overrides: Option<String>, // JSON with product-specific rules
    pub priority: i32, // Rule precedence order
    pub is_active: bool,
    pub effective_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: String,
}

/// Holiday Import Log database model (for audit trail of holiday imports)
#[derive(Debug, Clone)]
pub struct HolidayImportLogModel {
    pub import_id: Uuid,
    pub jurisdiction: String,
    pub import_year: i32,
    pub import_source: String,
    pub holidays_imported: i32,
    pub holidays_updated: i32,
    pub holidays_skipped: i32,
    pub import_status: String, // Success, Partial, Failed
    pub error_details: Option<String>,
    pub imported_by: String,
    pub imported_at: DateTime<Utc>,
}

/// Business Day Cache database model (for performance optimization)
#[derive(Debug, Clone)]
pub struct BusinessDayCacheModel {
    pub cache_id: Uuid,
    pub jurisdiction: String,
    pub date: NaiveDate,
    pub is_business_day: bool,
    pub is_holiday: bool,
    pub is_weekend: bool,
    pub holiday_name: Option<String>,
    pub cached_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
}
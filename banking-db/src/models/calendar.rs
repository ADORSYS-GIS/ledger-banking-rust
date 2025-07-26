use chrono::{DateTime, Utc, NaiveDate, Weekday};
use uuid::Uuid;
use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};

/// Holiday Type enum matching domain model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "holiday_type", rename_all = "PascalCase"))]
pub enum HolidayType { 
    National, 
    Regional,
    Religious, 
    Banking
}

/// Date Shift Rule enum matching domain model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DateShiftRule { 
    NextBusinessDay,
    PreviousBusinessDay,
    NoShift,
}

/// Weekend Treatment enum matching domain model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WeekendTreatment { 
    SaturdaySunday, 
    FridayOnly, 
    Custom(Vec<Weekday>) 
}

/// Import Status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ImportStatus {
    Success,
    Partial,
    Failed,
}

/// Bank Holiday database model - simplified to match domain
#[derive(Debug, Clone)]
pub struct BankHolidayModel {
    pub holiday_id: Uuid,
    pub jurisdiction: HeaplessString<10>, // Country/region code (e.g., "US", "UK", "CM")
    pub holiday_date: NaiveDate,
    pub holiday_name: HeaplessString<255>,
    pub holiday_type: HolidayType, // Use enum instead of HeaplessString
    pub is_recurring: bool,   // Annual recurrence flag
    pub description: Option<HeaplessString<256>>,
    pub created_by: Uuid, // References ReferencedPerson.person_id
    pub created_at: DateTime<Utc>,
}

/// Weekend Configuration database model
#[derive(Debug, Clone)]
pub struct WeekendConfigurationModel {
    pub config_id: Uuid,
    pub jurisdiction: HeaplessString<10>,
    pub weekend_days: HeaplessString<100>, // JSON array of weekday numbers (0=Sunday, 1=Monday, etc.)
    pub effective_date: NaiveDate,
    pub is_active: bool,
    pub notes: Option<HeaplessString<256>>,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid, // References ReferencedPerson.person_id
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: Uuid, // References ReferencedPerson.person_id
}

/// Date Calculation Rules database model
#[derive(Debug, Clone)]
pub struct DateCalculationRulesModel {
    pub rule_id: Uuid,
    pub jurisdiction: HeaplessString<10>,
    pub rule_name: HeaplessString<100>,
    pub rule_type: HeaplessString<30>, // DateShift, MaturityCalculation, PaymentDue
    pub default_shift_rule: DateShiftRule, // Use enum instead of HeaplessString
    pub weekend_treatment: WeekendTreatment, // Use enum instead of HeaplessString
    pub product_specific_overrides: Option<HeaplessString<1000>>, // JSON with product-specific rules
    pub priority: i32, // Rule precedence order
    pub is_active: bool,
    pub effective_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid, // References ReferencedPerson.person_id
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: Uuid, // References ReferencedPerson.person_id
}

/// Holiday Import Log database model (for audit trail of holiday imports)
#[derive(Debug, Clone)]
pub struct HolidayImportLogModel {
    pub import_id: Uuid,
    pub jurisdiction: HeaplessString<10>,
    pub import_year: i32,
    pub import_source: HeaplessString<100>,
    pub holidays_imported: i32,
    pub holidays_updated: i32,
    pub holidays_skipped: i32,
    pub import_status: ImportStatus, // Use enum instead of HeaplessString
    pub error_details: Option<HeaplessString<1000>>,
    pub imported_by: Uuid, // References ReferencedPerson.person_id
    pub imported_at: DateTime<Utc>,
}

/// Business Day Cache database model (for performance optimization)
#[derive(Debug, Clone)]
pub struct BusinessDayCacheModel {
    pub cache_id: Uuid,
    pub jurisdiction: HeaplessString<10>,
    pub date: NaiveDate,
    pub is_business_day: bool,
    pub is_holiday: bool,
    pub is_weekend: bool,
    pub holiday_name: Option<HeaplessString<255>>,
    pub cached_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
}
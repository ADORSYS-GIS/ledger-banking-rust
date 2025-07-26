use chrono::{DateTime, NaiveDate, Utc, Weekday};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct BankHoliday {
    pub holiday_id: Uuid,
    #[validate(length(min = 2, max = 10))]
    pub jurisdiction: String,
    pub holiday_date: NaiveDate,
    #[validate(length(min = 1, max = 255))]
    pub holiday_name: String,
    pub holiday_type: HolidayType,
    pub is_recurring: bool,
    pub description: Option<String>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HolidayType { 
    National, 
    Regional,
    Religious, 
    Banking
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DateCalculationRules {
    pub default_shift_rule: DateShiftRule,
    pub weekend_treatment: WeekendTreatment,
    #[validate(length(min = 2, max = 10))]
    pub jurisdiction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DateShiftRule { 
    NextBusinessDay,
    PreviousBusinessDay,
    NoShift,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeekendTreatment { 
    SaturdaySunday, 
    FridayOnly, 
    Custom(Vec<Weekday>) 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessDayCalculation {
    pub requested_date: NaiveDate,
    pub adjusted_date: NaiveDate,
    pub is_business_day: bool,
    pub applied_rule: DateShiftRule,
    pub jurisdiction: String,
}
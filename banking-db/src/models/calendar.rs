use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;
use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "weekday", rename_all = "PascalCase")]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl FromStr for Weekday {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Monday" => Ok(Weekday::Monday),
            "Tuesday" => Ok(Weekday::Tuesday),
            "Wednesday" => Ok(Weekday::Wednesday),
            "Thursday" => Ok(Weekday::Thursday),
            "Friday" => Ok(Weekday::Friday),
            "Saturday" => Ok(Weekday::Saturday),
            "Sunday" => Ok(Weekday::Sunday),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct WeekendDaysModel {
    pub id: Uuid,
    pub name_l1: HeaplessString<50>,
    pub name_l2: Option<HeaplessString<50>>,
    pub name_l3: Option<HeaplessString<50>>,
    pub weekend_day_01: Option<Weekday>,
    pub weekend_day_02: Option<Weekday>,
    pub weekend_day_03: Option<Weekday>,
    pub weekend_day_04: Option<Weekday>,
    pub weekend_day_05: Option<Weekday>,
    pub weekend_day_06: Option<Weekday>,
    pub weekend_day_07: Option<Weekday>,
    pub valid_from: DateTime<Utc>,
    pub valid_to: Option<DateTime<Utc>>,
    pub created_by_person_id: Uuid,
    pub created_at: DateTime<Utc>,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "holiday_type", rename_all = "PascalCase")]
pub enum HolidayType {
    National,
    Regional,
    Religious,
    Banking,
}

impl FromStr for HolidayType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "National" => Ok(HolidayType::National),
            "Regional" => Ok(HolidayType::Regional),
            "Religious" => Ok(HolidayType::Religious),
            "Banking" => Ok(HolidayType::Banking),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "date_shift_rule", rename_all = "PascalCase")]
pub enum DateShiftRule {
    NextBusinessDay,
    PreviousBusinessDay,
    NoShift,
}

impl FromStr for DateShiftRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NextBusinessDay" => Ok(DateShiftRule::NextBusinessDay),
            "PreviousBusinessDay" => Ok(DateShiftRule::PreviousBusinessDay),
            "NoShift" => Ok(DateShiftRule::NoShift),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "import_status", rename_all = "PascalCase")]
pub enum ImportStatus {
    Success,
    Partial,
    Failed,
}

impl FromStr for ImportStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Success" => Ok(ImportStatus::Success),
            "Partial" => Ok(ImportStatus::Partial),
            "Failed" => Ok(ImportStatus::Failed),
            _ => Err(()),
        }
    }
}

/// Bank Holiday database model - simplified to match domain
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct BankHolidayModel {
    pub id: Uuid,
    pub jurisdiction: HeaplessString<10>, // Country/region code (e.g., "US", "UK", "CM")
    pub holiday_date: NaiveDate,
    pub holiday_name: HeaplessString<50>, // Updated to match API domain
    #[serde(serialize_with = "serialize_holiday_type", deserialize_with = "deserialize_holiday_type")]
    pub holiday_type: HolidayType, // Use enum instead of HeaplessString
    pub is_recurring: bool,   // Annual recurrence flag
    pub description: Option<HeaplessString<200>>, // Updated to match API domain
    pub created_by_person_id: Uuid, // References Person.id
    pub created_at: DateTime<Utc>,
}

/// Weekend Configuration database model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct WeekendConfigurationModel {
    pub id: Uuid,
    pub jurisdiction: HeaplessString<10>,
    pub weekend_days: HeaplessString<100>, // JSON array of weekday numbers (0=Sunday, 1=Monday, etc.)
    pub effective_date: NaiveDate,
    pub is_active: bool,
    pub notes: Option<HeaplessString<256>>,
    pub created_at: DateTime<Utc>,
    pub created_by_person_id: Uuid, // References Person.id
    pub last_updated_at: DateTime<Utc>,
    pub updated_by_person_id: Uuid, // References Person.id
}

/// Date Calculation Rules database model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct DateCalculationRulesModel {
    pub id: Uuid,
    pub jurisdiction: HeaplessString<10>,
    pub rule_name: HeaplessString<100>,
    pub rule_type: HeaplessString<30>, // DateShift, MaturityCalculation, PaymentDue
    pub default_shift_rule: DateShiftRule, // Use enum instead of HeaplessString
    pub weekend_days_id: Uuid,
    pub product_specific_overrides: Option<HeaplessString<1000>>, // JSON with product-specific rules
    pub priority: i32, // Rule precedence order
    pub is_active: bool,
    pub effective_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub created_by_person_id: Uuid, // References Person.id
    pub last_updated_at: DateTime<Utc>,
    pub updated_by_person_id: Uuid, // References Person.id
}

/// Holiday Import Log database model (for audit trail of holiday imports)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct HolidayImportLogModel {
    pub id: Uuid,
    pub jurisdiction: HeaplessString<10>,
    pub import_year: i32,
    pub import_source: HeaplessString<100>,
    pub holidays_imported: i32,
    pub holidays_updated: i32,
    pub holidays_skipped: i32,
    pub import_status: ImportStatus, // Use enum instead of HeaplessString
    pub error_details: Option<HeaplessString<1000>>,
    pub imported_by_person_id: Uuid, // References Person.id
    pub imported_at: DateTime<Utc>,
}

/// Business Day Cache database model (for performance optimization)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct BusinessDayCacheModel {
    pub id: Uuid,
    pub jurisdiction: HeaplessString<10>,
    pub date: NaiveDate,
    pub is_business_day: bool,
    pub is_holiday: bool,
    pub is_weekend: bool,
    pub holiday_name: Option<HeaplessString<255>>,
    pub cached_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
}

// Serialization functions for HolidayType
fn serialize_holiday_type<S>(holiday_type: &HolidayType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match holiday_type {
        HolidayType::National => "National",
        HolidayType::Regional => "Regional",
        HolidayType::Religious => "Religious",
        HolidayType::Banking => "Banking",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_holiday_type<'de, D>(deserializer: D) -> Result<HolidayType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "National" => Ok(HolidayType::National),
        "Regional" => Ok(HolidayType::Regional),
        "Religious" => Ok(HolidayType::Religious),
        "Banking" => Ok(HolidayType::Banking),
        _ => Err(serde::de::Error::custom(format!("Unknown holiday type: {s}"))),
    }
}
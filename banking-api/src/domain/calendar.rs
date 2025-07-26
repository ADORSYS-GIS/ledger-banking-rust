use chrono::{DateTime, NaiveDate, Utc, Weekday};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use heapless::String as HeaplessString;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankHoliday {
    pub holiday_id: Uuid,
    pub jurisdiction: HeaplessString<10>,
    pub holiday_date: NaiveDate,
    pub holiday_name: HeaplessString<255>,
    pub holiday_type: HolidayType,
    pub is_recurring: bool,
    pub description: Option<HeaplessString<256>>,
    /// References ReferencedPerson.person_id
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

impl BankHoliday {
    pub fn new(
        holiday_id: Uuid,
        jurisdiction: &str,
        holiday_date: NaiveDate,
        holiday_name: &str,
        holiday_type: HolidayType,
        is_recurring: bool,
        description: Option<&str>,
        created_by: Uuid,
        created_at: DateTime<Utc>,
    ) -> Result<Self, &'static str> {
        if jurisdiction.len() < 2 {
            return Err("Jurisdiction must be at least 2 characters");
        }
        if holiday_name.is_empty() {
            return Err("Holiday name cannot be empty");
        }

        let jurisdiction = HeaplessString::try_from(jurisdiction)
            .map_err(|_| "Jurisdiction exceeds maximum length of 10 characters")?;
        let holiday_name = HeaplessString::try_from(holiday_name)
            .map_err(|_| "Holiday name exceeds maximum length of 255 characters")?;
        let description = match description {
            Some(desc) => Some(HeaplessString::try_from(desc)
                .map_err(|_| "Description exceeds maximum length of 256 characters")?),
            None => None,
        };

        Ok(BankHoliday {
            holiday_id,
            jurisdiction,
            holiday_date,
            holiday_name,
            holiday_type,
            is_recurring,
            description,
            created_by,
            created_at,
        })
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.jurisdiction.len() < 2 {
            return Err("Jurisdiction must be at least 2 characters");
        }
        if self.holiday_name.is_empty() {
            return Err("Holiday name cannot be empty");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HolidayType { 
    National, 
    Regional,
    Religious, 
    Banking
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateCalculationRules {
    pub default_shift_rule: DateShiftRule,
    pub weekend_treatment: WeekendTreatment,
    pub jurisdiction: HeaplessString<10>,
}

impl DateCalculationRules {
    pub fn new(
        default_shift_rule: DateShiftRule,
        weekend_treatment: WeekendTreatment,
        jurisdiction: &str,
    ) -> Result<Self, &'static str> {
        if jurisdiction.len() < 2 {
            return Err("Jurisdiction must be at least 2 characters");
        }

        let jurisdiction = HeaplessString::try_from(jurisdiction)
            .map_err(|_| "Jurisdiction exceeds maximum length of 10 characters")?;

        Ok(DateCalculationRules {
            default_shift_rule,
            weekend_treatment,
            jurisdiction,
        })
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.jurisdiction.len() < 2 {
            return Err("Jurisdiction must be at least 2 characters");
        }
        Ok(())
    }
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
    pub jurisdiction: HeaplessString<10>,
}

impl BusinessDayCalculation {
    pub fn new(
        requested_date: NaiveDate,
        adjusted_date: NaiveDate,
        is_business_day: bool,
        applied_rule: DateShiftRule,
        jurisdiction: &str,
    ) -> Result<Self, &'static str> {
        let jurisdiction = HeaplessString::try_from(jurisdiction)
            .map_err(|_| "Jurisdiction exceeds maximum length of 10 characters")?;

        Ok(BusinessDayCalculation {
            requested_date,
            adjusted_date,
            is_business_day,
            applied_rule,
            jurisdiction,
        })
    }
}
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use heapless::String as HeaplessString;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekendDays {
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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankHoliday {
    pub id: Uuid,
    pub jurisdiction: HeaplessString<10>,
    pub holiday_date: NaiveDate,
    pub holiday_name: HeaplessString<50>,
    pub holiday_type: HolidayType,
    pub is_recurring: bool,
    pub description: Option<HeaplessString<200>>,
    /// References Person.id
    pub created_by_person_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl BankHoliday {
    /// Builder for BankHoliday construction - preferred approach
    pub fn builder(id: Uuid, holiday_type: HolidayType) -> BankHolidayBuilder {
        BankHolidayBuilder::new(id, holiday_type)
    }

    /// Legacy constructor - deprecated in favor of builder pattern
    #[deprecated(since = "0.1.0", note = "Use BankHoliday::builder() instead")]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        jurisdiction: &str,
        holiday_date: NaiveDate,
        holiday_name: &str,
        holiday_type: HolidayType,
        is_recurring: bool,
        description: Option<&str>,
        created_by_person_id: Uuid,
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
            id,
            jurisdiction,
            holiday_date,
            holiday_name,
            holiday_type,
            is_recurring,
            description,
            created_by_person_id,
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

/// Builder for BankHoliday construction
pub struct BankHolidayBuilder {
    id: Uuid,
    holiday_type: HolidayType,
    jurisdiction: Option<String>,
    holiday_date: Option<NaiveDate>,
    holiday_name: Option<String>,
    is_recurring: bool,
    description: Option<String>,
    created_by_person_id: Option<Uuid>,
    created_at: Option<DateTime<Utc>>,
}

impl BankHolidayBuilder {
    pub fn new(id: Uuid, holiday_type: HolidayType) -> Self {
        Self {
            id,
            holiday_type,
            jurisdiction: None,
            holiday_date: None,
            holiday_name: None,
            is_recurring: false,
            description: None,
            created_by_person_id: None,
            created_at: None,
        }
    }

    pub fn jurisdiction(mut self, jurisdiction: &str) -> Self {
        self.jurisdiction = Some(jurisdiction.to_string());
        self
    }

    pub fn holiday_date(mut self, holiday_date: NaiveDate) -> Self {
        self.holiday_date = Some(holiday_date);
        self
    }

    pub fn holiday_name(mut self, holiday_name: &str) -> Self {
        self.holiday_name = Some(holiday_name.to_string());
        self
    }

    pub fn is_recurring(mut self, is_recurring: bool) -> Self {
        self.is_recurring = is_recurring;
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn created_by(mut self, created_by_person_id: Uuid) -> Self {
        self.created_by_person_id = Some(created_by_person_id);
        self
    }

    pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn build(self) -> Result<BankHoliday, &'static str> {
        let jurisdiction = self.jurisdiction.ok_or("Jurisdiction is required")?;
        let holiday_date = self.holiday_date.ok_or("Holiday date is required")?;
        let holiday_name = self.holiday_name.ok_or("Holiday name is required")?;
        let created_by_person_id = self.created_by_person_id.ok_or("Created by is required")?;
        let created_at = self.created_at.ok_or("Created at is required")?;

        if jurisdiction.len() < 2 {
            return Err("Jurisdiction must be at least 2 characters");
        }
        if holiday_name.is_empty() {
            return Err("Holiday name cannot be empty");
        }

        let jurisdiction = HeaplessString::try_from(jurisdiction.as_str())
            .map_err(|_| "Jurisdiction exceeds maximum length of 10 characters")?;
        let holiday_name = HeaplessString::try_from(holiday_name.as_str())
            .map_err(|_| "Holiday name exceeds maximum length of 255 characters")?;
        let description = match self.description {
            Some(desc) => Some(HeaplessString::try_from(desc.as_str())
                .map_err(|_| "Description exceeds maximum length of 256 characters")?),
            None => None,
        };

        Ok(BankHoliday {
            id: self.id,
            jurisdiction,
            holiday_date,
            holiday_name,
            holiday_type: self.holiday_type,
            is_recurring: self.is_recurring,
            description,
            created_by_person_id,
            created_at,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HolidayType {
    National,
    Regional,
    Religious,
    Banking
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateCalculationRules {
    pub default_shift_rule: DateShiftRule,
    pub weekend_days: Uuid,
    pub jurisdiction: HeaplessString<10>,
}

impl DateCalculationRules {
    pub fn new(
        default_shift_rule: DateShiftRule,
        weekend_days: Uuid,
        jurisdiction: &str,
    ) -> Result<Self, &'static str> {
        if jurisdiction.len() < 2 {
            return Err("Jurisdiction must be at least 2 characters");
        }

        let jurisdiction = HeaplessString::try_from(jurisdiction)
            .map_err(|_| "Jurisdiction exceeds maximum length of 10 characters")?;

        Ok(DateCalculationRules {
            default_shift_rule,
            weekend_days,
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_bank_holiday_builder_pattern() {
        let holiday_id = uuid::Uuid::new_v4();
        let created_by_person_id = uuid::Uuid::new_v4();
        let created_at = Utc::now();
        let holiday_date = chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        let holiday = BankHoliday::builder(holiday_id, HolidayType::National)
            .jurisdiction("US")
            .holiday_date(holiday_date)
            .holiday_name("New Year's Day")
            .is_recurring(true)
            .description("Annual national holiday")
            .created_by(created_by_person_id)
            .created_at(created_at)
            .build()
            .expect("Should build successfully");

        assert_eq!(holiday.id, holiday_id);
        assert_eq!(holiday.holiday_type, HolidayType::National);
        assert_eq!(holiday.jurisdiction.as_str(), "US");
        assert_eq!(holiday.holiday_date, holiday_date);
        assert_eq!(holiday.holiday_name.as_str(), "New Year's Day");
        assert!(holiday.is_recurring);
        assert_eq!(holiday.description.as_ref().unwrap().as_str(), "Annual national holiday");
        assert_eq!(holiday.created_by_person_id, created_by_person_id);
        assert_eq!(holiday.created_at, created_at);
    }

    #[test]
    fn test_bank_holiday_builder_validation() {
        let holiday_id = uuid::Uuid::new_v4();

        // Missing required fields should fail
        let result = BankHoliday::builder(holiday_id, HolidayType::National)
            .jurisdiction("US")
            .build();
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Holiday date is required");
    }
}
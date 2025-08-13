use std::sync::Arc;
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

use banking_api::{
    domain::{BankHoliday, BusinessDayCalculation, HolidayType, DateShiftRule, WeekendDays, Weekday},
    error::{BankingError, BankingResult},
    service::CalendarService,
};
use banking_db::repository::CalendarRepository;
use crate::mappers::CalendarMapper;
use crate::validation::CalendarValidation;

/// Production implementation of CalendarService with comprehensive validation
pub struct CalendarServiceImpl {
    calendar_repository: Arc<dyn CalendarRepository>,
}

impl CalendarServiceImpl {
    pub fn new(calendar_repository: Arc<dyn CalendarRepository>) -> Self {
        Self { calendar_repository }
    }

    /// Get weekend configuration for jurisdiction with validation
    async fn get_weekend_days_for_jurisdiction(&self, jurisdiction: &str) -> BankingResult<WeekendDays> {
        // Validate jurisdiction format
        if jurisdiction.trim().is_empty() {
            return Err(BankingError::ValidationFailed("Jurisdiction cannot be empty".to_string()));
        }

        let chrono_weekdays = self.calendar_repository
            .get_weekend_days(jurisdiction)
            .await?;
        
        // Convert chrono weekdays back to WeekendDays domain object
        let now = chrono::Utc::now();
        let system_user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000")
            .map_err(|e| BankingError::ValidationError {
                field: "system_user_id".to_string(),
                message: format!("Invalid UUID: {e}")
            })?;
        
        let name_l1 = heapless::String::try_from(jurisdiction)
            .map_err(|_| BankingError::ValidationError {
                field: "jurisdiction".to_string(),
                message: "Jurisdiction name too long".to_string(),
            })?;
        
        let mut weekend_days = WeekendDays {
            id: Uuid::new_v4(),
            name_l1,
            name_l2: None,
            name_l3: None,
            weekend_day_01: None,
            weekend_day_02: None,
            weekend_day_03: None,
            weekend_day_04: None,
            weekend_day_05: None,
            weekend_day_06: None,
            weekend_day_07: None,
            valid_from: now,
            valid_to: None,
            created_by_person_id: system_user_id,
            created_at: now,
        };
        
        // Map chrono weekdays to WeekendDays fields
        for (i, &weekday) in chrono_weekdays.iter().enumerate() {
            let domain_weekday = Self::chrono_weekday_to_domain_weekday(weekday);
            match i {
                0 => weekend_days.weekend_day_01 = Some(domain_weekday),
                1 => weekend_days.weekend_day_02 = Some(domain_weekday),
                2 => weekend_days.weekend_day_03 = Some(domain_weekday),
                3 => weekend_days.weekend_day_04 = Some(domain_weekday),
                4 => weekend_days.weekend_day_05 = Some(domain_weekday),
                5 => weekend_days.weekend_day_06 = Some(domain_weekday),
                6 => weekend_days.weekend_day_07 = Some(domain_weekday),
                _ => break, // More than 7 days, ignore extras
            }
        }
        
        Ok(weekend_days)
    }

    /// Get holidays for jurisdiction and date range
    async fn get_holidays_for_date_range(
        &self,
        jurisdiction: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> BankingResult<Vec<NaiveDate>> {
        let holidays = self.calendar_repository
            .find_holidays_in_range(start_date, end_date, jurisdiction)
            .await?;

        Ok(holidays.into_iter().map(|h| h.holiday_date).collect())
    }
    
    fn get_weekdays_from_weekend_days(weekend_days: &WeekendDays) -> Vec<chrono::Weekday> {
        let mut weekdays = Vec::new();
        if let Some(day) = &weekend_days.weekend_day_01 { weekdays.push(Self::domain_weekday_to_chrono_weekday(day)); }
        if let Some(day) = &weekend_days.weekend_day_02 { weekdays.push(Self::domain_weekday_to_chrono_weekday(day)); }
        if let Some(day) = &weekend_days.weekend_day_03 { weekdays.push(Self::domain_weekday_to_chrono_weekday(day)); }
        if let Some(day) = &weekend_days.weekend_day_04 { weekdays.push(Self::domain_weekday_to_chrono_weekday(day)); }
        if let Some(day) = &weekend_days.weekend_day_05 { weekdays.push(Self::domain_weekday_to_chrono_weekday(day)); }
        if let Some(day) = &weekend_days.weekend_day_06 { weekdays.push(Self::domain_weekday_to_chrono_weekday(day)); }
        if let Some(day) = &weekend_days.weekend_day_07 { weekdays.push(Self::domain_weekday_to_chrono_weekday(day)); }
        weekdays
    }

    fn domain_weekday_to_chrono_weekday(weekday: &Weekday) -> chrono::Weekday {
        match weekday {
            Weekday::Monday => chrono::Weekday::Mon,
            Weekday::Tuesday => chrono::Weekday::Tue,
            Weekday::Wednesday => chrono::Weekday::Wed,
            Weekday::Thursday => chrono::Weekday::Thu,
            Weekday::Friday => chrono::Weekday::Fri,
            Weekday::Saturday => chrono::Weekday::Sat,
            Weekday::Sunday => chrono::Weekday::Sun,
        }
    }

    fn chrono_weekday_to_domain_weekday(weekday: chrono::Weekday) -> Weekday {
        match weekday {
            chrono::Weekday::Mon => Weekday::Monday,
            chrono::Weekday::Tue => Weekday::Tuesday,
            chrono::Weekday::Wed => Weekday::Wednesday,
            chrono::Weekday::Thu => Weekday::Thursday,
            chrono::Weekday::Fri => Weekday::Friday,
            chrono::Weekday::Sat => Weekday::Saturday,
            chrono::Weekday::Sun => Weekday::Sunday,
        }
    }
}

#[async_trait]
impl CalendarService for CalendarServiceImpl {
    /// Check if a date is a business day with comprehensive validation
    async fn is_business_day(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<bool> {
        // Get weekend configuration with validation
        let weekend_days = self.get_weekend_days_for_jurisdiction(jurisdiction).await?;
        let weekdays = Self::get_weekdays_from_weekend_days(&weekend_days);
        
        // Get holidays for the specific date
        let holidays = self.get_holidays_for_date_range(jurisdiction, date, date).await?;

        // Use validation helper to determine if it's a business day
        Ok(CalendarValidation::is_business_day_from_weekdays(date, &weekdays, &holidays))
    }

    /// Get next business day with validation
    async fn next_business_day(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<NaiveDate> {
        let weekend_days = self.get_weekend_days_for_jurisdiction(jurisdiction).await?;
        let weekdays = Self::get_weekdays_from_weekend_days(&weekend_days);
        
        // Get holidays for a reasonable range (next 30 days to handle consecutive holidays)
        let end_range = date + chrono::Duration::days(30);
        let holidays = self.get_holidays_for_date_range(jurisdiction, date, end_range).await?;

        Ok(CalendarValidation::next_business_day_from_weekdays(date, &weekdays, &holidays))
    }

    /// Get previous business day with validation
    async fn previous_business_day(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<NaiveDate> {
        let weekend_days = self.get_weekend_days_for_jurisdiction(jurisdiction).await?;
        let weekdays = Self::get_weekdays_from_weekend_days(&weekend_days);
        
        // Get holidays for a reasonable range (previous 30 days)
        let start_range = date - chrono::Duration::days(30);
        let holidays = self.get_holidays_for_date_range(jurisdiction, start_range, date).await?;

        Ok(CalendarValidation::previous_business_day_from_weekdays(date, &weekdays, &holidays))
    }

    /// Add business days to a date with validation
    async fn add_business_days(&self, date: NaiveDate, days: i32, jurisdiction: &str) -> BankingResult<NaiveDate> {
        if days == 0 {
            return Ok(date);
        }

        let weekend_days = self.get_weekend_days_for_jurisdiction(jurisdiction).await?;
        let weekdays = Self::get_weekdays_from_weekend_days(&weekend_days);
        
        // Get holidays for a reasonable range based on days to add
        let range_days = (days.abs() * 2).max(60); // Buffer for weekends and holidays
        let (start_range, end_range) = if days > 0 {
            (date, date + chrono::Duration::days(range_days as i64))
        } else {
            (date - chrono::Duration::days(range_days as i64), date)
        };
        
        let holidays = self.get_holidays_for_date_range(jurisdiction, start_range, end_range).await?;

        let mut current_date = date;
        let mut remaining_days = days.abs();

        if days > 0 {
            // Add business days forward
            while remaining_days > 0 {
                current_date = CalendarValidation::next_business_day_from_weekdays(current_date, &weekdays, &holidays);
                remaining_days -= 1;
            }
        } else {
            // Add business days backward
            while remaining_days > 0 {
                current_date = CalendarValidation::previous_business_day_from_weekdays(current_date, &weekdays, &holidays);
                remaining_days -= 1;
            }
        }

        Ok(current_date)
    }

    /// Count business days between two dates with validation
    async fn count_business_days(&self, from: NaiveDate, to: NaiveDate, jurisdiction: &str) -> BankingResult<i32> {
        if from >= to {
            return Ok(0);
        }

        let weekend_days = self.get_weekend_days_for_jurisdiction(jurisdiction).await?;
        let weekdays = Self::get_weekdays_from_weekend_days(&weekend_days);
        let holidays = self.get_holidays_for_date_range(jurisdiction, from, to).await?;

        Ok(CalendarValidation::count_business_days_from_weekdays(from, to, &weekdays, &holidays))
    }

    /// Add a bank holiday with comprehensive validation
    async fn add_bank_holiday(&self, holiday: BankHoliday) -> BankingResult<()> {
        // Convert HolidayType enum to string for validation
        let holiday_type_str = match &holiday.holiday_type {
            HolidayType::National => "National",
            HolidayType::Regional => "Regional",
            HolidayType::Religious => "Religious",
            HolidayType::Banking => "Banking",
        };

        // Validate holiday configuration
        CalendarValidation::validate_holiday_config(
            &holiday.jurisdiction,
            holiday.holiday_date,
            &holiday.holiday_name,
            holiday_type_str,
        )?;

        // Convert to model and save using create_holiday (the method that exists)
        let holiday_model = CalendarMapper::holiday_to_model(holiday);
        self.calendar_repository.create_holiday(holiday_model).await?;

        Ok(())
    }

    /// Remove a bank holiday
    async fn remove_bank_holiday(&self, _holiday_id: Uuid) -> BankingResult<()> {
        // Since the repository doesn't have a method to check by holiday_id,
        // we'll just try to delete it. The repository should handle the case
        // where it doesn't exist.
        
        // Get the holiday to find its date and jurisdiction
        // Since we don't have a method to get by ID, we'll have to work around this
        // For now, we'll just return an error since the repository doesn't support this operation
        
        return Err(BankingError::NotImplemented(
            "Removing holidays by ID is not supported by the current repository".to_string()
        ));
    }

    /// Get all holidays for a jurisdiction and year
    async fn get_holidays(&self, jurisdiction: &str, year: i32) -> BankingResult<Vec<BankHoliday>> {
        // Validate inputs
        if jurisdiction.trim().is_empty() {
            return Err(BankingError::ValidationFailed("Jurisdiction cannot be empty".to_string()));
        }

        if !(1900..=2100).contains(&year) {
            return Err(BankingError::ValidationFailed("Year must be between 1900 and 2100".to_string()));
        }

        // Since get_holidays_by_year doesn't exist, we'll use find_holidays_in_range
        let start_date = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();
        
        let holiday_models = self.calendar_repository
            .find_holidays_in_range(start_date, end_date, jurisdiction)
            .await?;

        Ok(holiday_models.into_iter()
            .map(CalendarMapper::holiday_from_model)
            .collect())
    }

    /// Calculate business day with rule application
    async fn calculate_business_day(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<BusinessDayCalculation> {
        let weekend_days = self.get_weekend_days_for_jurisdiction(jurisdiction).await?;
        let weekdays = Self::get_weekdays_from_weekend_days(&weekend_days);
        let holidays = self.get_holidays_for_date_range(jurisdiction, date, date).await?;

        let is_business_day = CalendarValidation::is_business_day_from_weekdays(date, &weekdays, &holidays);
        let _is_weekend = CalendarValidation::is_weekend_day_from_weekdays(date, &weekdays);
        let _is_holiday = holidays.contains(&date);

        let adjusted_date = if is_business_day {
            date
        } else {
            CalendarValidation::next_business_day_from_weekdays(date, &weekdays, &holidays)
        };

        let applied_rule = if is_business_day {
            DateShiftRule::NoShift
        } else {
            DateShiftRule::NextBusinessDay
        };

        BusinessDayCalculation::new(
            date,
            adjusted_date,
            is_business_day,
            applied_rule,
            jurisdiction,
        ).map_err(|e| BankingError::ValidationError {
            field: "jurisdiction".to_string(),
            message: e.to_string(),
        })
    }

    /// Batch business day calculations for performance
    async fn batch_calculate_business_days(
        &self,
        dates: Vec<NaiveDate>,
        jurisdiction: &str,
    ) -> BankingResult<Vec<BusinessDayCalculation>> {
        if dates.is_empty() {
            return Ok(Vec::new());
        }

        let weekend_days = self.get_weekend_days_for_jurisdiction(jurisdiction).await?;
        let weekdays = Self::get_weekdays_from_weekend_days(&weekend_days);
        
        // Get the date range for all dates
        let min_date = *dates.iter().min().unwrap();
        let max_date = *dates.iter().max().unwrap();
        
        let holidays = self.get_holidays_for_date_range(jurisdiction, min_date, max_date).await?;

        let mut results = Vec::with_capacity(dates.len());
        
        for date in dates {
            let is_business_day = CalendarValidation::is_business_day_from_weekdays(date, &weekdays, &holidays);
            let _is_weekend = CalendarValidation::is_weekend_day_from_weekdays(date, &weekdays);
            let _is_holiday = holidays.contains(&date);

            let adjusted_date = if is_business_day {
                date
            } else {
                CalendarValidation::next_business_day_from_weekdays(date, &weekdays, &holidays)
            };

            let applied_rule = if is_business_day {
                DateShiftRule::NoShift
            } else {
                DateShiftRule::NextBusinessDay
            };

            let calculation = BusinessDayCalculation::new(
                date,
                adjusted_date,
                is_business_day,
                applied_rule,
                jurisdiction,
            ).map_err(|e| BankingError::ValidationError {
                field: "jurisdiction".to_string(),
                message: e.to_string(),
            })?;
            results.push(calculation);
        }

        Ok(results)
    }

    /// Check if date falls on weekend
    async fn is_weekend(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<bool> {
        let weekend_days = self.get_weekend_days_for_jurisdiction(jurisdiction).await?;
        let weekdays = Self::get_weekdays_from_weekend_days(&weekend_days);
        Ok(CalendarValidation::is_weekend_day_from_weekdays(date, &weekdays))
    }

    /// Create a new weekend days configuration
    async fn create_weekend_days(&self, weekend_days: WeekendDays) -> BankingResult<WeekendDays> {
        // Convert the domain weekend days to chrono weekdays for the repository
        let chrono_weekdays = Self::get_weekdays_from_weekend_days(&weekend_days);
        
        // Set the weekend days in the repository using the jurisdiction
        self.calendar_repository.set_weekend_days(&weekend_days.name_l1, chrono_weekdays).await?;
        
        // Return the same weekend days object since we don't have ID-based retrieval
        Ok(weekend_days)
    }

    /// Get a weekend days configuration by its ID
    async fn get_weekend_days_by_id(&self, _weekend_days_id: Uuid) -> BankingResult<Option<WeekendDays>> {
        // Since the repository doesn't support ID-based weekend days retrieval,
        // and we don't have jurisdiction information from just the ID,
        // we return an error indicating this operation is not supported
        Err(BankingError::NotImplemented(
            "Weekend days retrieval by ID is not supported by the current repository implementation".to_string()
        ))
    }

    /// Update an existing weekend days configuration
    async fn update_weekend_days(&self, weekend_days: WeekendDays) -> BankingResult<WeekendDays> {
        // Convert the domain weekend days to chrono weekdays for the repository
        let chrono_weekdays = Self::get_weekdays_from_weekend_days(&weekend_days);
        
        // Update the weekend days in the repository using the jurisdiction
        self.calendar_repository.set_weekend_days(&weekend_days.name_l1, chrono_weekdays).await?;
        
        // Return the same weekend days object since we don't have ID-based retrieval
        Ok(weekend_days)
    }

    /// Delete a weekend days configuration by its ID
    async fn delete_weekend_days(&self, _weekend_days_id: Uuid) -> BankingResult<()> {
        // Since the repository doesn't support ID-based weekend days deletion,
        // and we don't have jurisdiction information from just the ID,
        // we return an error indicating this operation is not supported
        Err(BankingError::NotImplemented(
            "Weekend days deletion by ID is not supported by the current repository implementation".to_string()
        ))
    }
}
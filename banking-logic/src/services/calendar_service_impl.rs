use std::sync::Arc;
use async_trait::async_trait;
use chrono::{NaiveDate, Weekday};
use uuid::Uuid;

use banking_api::{
    BankingResult, BankingError,
    domain::{BankHoliday, BusinessDayCalculation, HolidayType, DateShiftRule},
    service::CalendarService,
};
use banking_db::repository::CalendarRepository;
use crate::validation::CalendarValidation;
use crate::mappers::CalendarMapper;

/// Production implementation of CalendarService with comprehensive validation
pub struct CalendarServiceImpl {
    calendar_repository: Arc<dyn CalendarRepository>,
}

impl CalendarServiceImpl {
    pub fn new(calendar_repository: Arc<dyn CalendarRepository>) -> Self {
        Self { calendar_repository }
    }

    /// Get weekend configuration for jurisdiction with validation
    async fn get_weekend_days_for_jurisdiction(&self, jurisdiction: &str) -> BankingResult<Vec<Weekday>> {
        // Validate jurisdiction format
        if jurisdiction.trim().is_empty() {
            return Err(BankingError::ValidationFailed("Jurisdiction cannot be empty".to_string()));
        }

        let weekend_days = self.calendar_repository
            .get_weekend_days(jurisdiction)
            .await?;
        
        if weekend_days.is_empty() {
            return Err(BankingError::ValidationFailed(
                format!("Weekend configuration not found for jurisdiction: {jurisdiction}")
            ));
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
}

#[async_trait]
impl CalendarService for CalendarServiceImpl {
    /// Check if a date is a business day with comprehensive validation
    async fn is_business_day(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<bool> {
        // Get weekend configuration with validation
        let weekend_days = self.get_weekend_days_for_jurisdiction(jurisdiction).await?;
        
        // Get holidays for the specific date
        let holidays = self.get_holidays_for_date_range(jurisdiction, date, date).await?;

        // Use validation helper to determine if it's a business day
        Ok(CalendarValidation::is_business_day_from_weekdays(date, &weekend_days, &holidays))
    }

    /// Get next business day with validation
    async fn next_business_day(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<NaiveDate> {
        let weekend_days = self.get_weekend_days_for_jurisdiction(jurisdiction).await?;
        
        // Get holidays for a reasonable range (next 30 days to handle consecutive holidays)
        let end_range = date + chrono::Duration::days(30);
        let holidays = self.get_holidays_for_date_range(jurisdiction, date, end_range).await?;

        Ok(CalendarValidation::next_business_day_from_weekdays(date, &weekend_days, &holidays))
    }

    /// Get previous business day with validation
    async fn previous_business_day(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<NaiveDate> {
        let weekend_days = self.get_weekend_days_for_jurisdiction(jurisdiction).await?;
        
        // Get holidays for a reasonable range (previous 30 days)
        let start_range = date - chrono::Duration::days(30);
        let holidays = self.get_holidays_for_date_range(jurisdiction, start_range, date).await?;

        Ok(CalendarValidation::previous_business_day_from_weekdays(date, &weekend_days, &holidays))
    }

    /// Add business days to a date with validation
    async fn add_business_days(&self, date: NaiveDate, days: i32, jurisdiction: &str) -> BankingResult<NaiveDate> {
        if days == 0 {
            return Ok(date);
        }

        let weekend_days = self.get_weekend_days_for_jurisdiction(jurisdiction).await?;
        
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
                current_date = CalendarValidation::next_business_day_from_weekdays(current_date, &weekend_days, &holidays);
                remaining_days -= 1;
            }
        } else {
            // Add business days backward
            while remaining_days > 0 {
                current_date = CalendarValidation::previous_business_day_from_weekdays(current_date, &weekend_days, &holidays);
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
        let holidays = self.get_holidays_for_date_range(jurisdiction, from, to).await?;

        Ok(CalendarValidation::count_business_days_from_weekdays(from, to, &weekend_days, &holidays))
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
        let holiday_model = CalendarMapper::holiday_to_model(holiday)
            .map_err(|e| BankingError::ValidationError {
                field: "holiday".to_string(),
                message: e.to_string(),
            })?;
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
        let holidays = self.get_holidays_for_date_range(jurisdiction, date, date).await?;

        let is_business_day = CalendarValidation::is_business_day_from_weekdays(date, &weekend_days, &holidays);
        let _is_weekend = CalendarValidation::is_weekend_day_from_weekdays(date, &weekend_days);
        let _is_holiday = holidays.contains(&date);

        let adjusted_date = if is_business_day {
            date
        } else {
            CalendarValidation::next_business_day_from_weekdays(date, &weekend_days, &holidays)
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
        
        // Get the date range for all dates
        let min_date = *dates.iter().min().unwrap();
        let max_date = *dates.iter().max().unwrap();
        
        let holidays = self.get_holidays_for_date_range(jurisdiction, min_date, max_date).await?;

        let mut results = Vec::with_capacity(dates.len());
        
        for date in dates {
            let is_business_day = CalendarValidation::is_business_day_from_weekdays(date, &weekend_days, &holidays);
            let _is_weekend = CalendarValidation::is_weekend_day_from_weekdays(date, &weekend_days);
            let _is_holiday = holidays.contains(&date);

            let adjusted_date = if is_business_day {
                date
            } else {
                CalendarValidation::next_business_day_from_weekdays(date, &weekend_days, &holidays)
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
        Ok(CalendarValidation::is_weekend_day_from_weekdays(date, &weekend_days))
    }

    /// Get weekend days for jurisdiction
    async fn get_weekend_days(&self, jurisdiction: &str) -> BankingResult<Vec<Weekday>> {
        self.get_weekend_days_for_jurisdiction(jurisdiction).await
    }
}

/// Extension methods for weekend configuration management
impl CalendarServiceImpl {
    /// Create or update weekend configuration with validation
    pub async fn set_weekend_config_for_jurisdiction(
        &self,
        jurisdiction: &str,
        weekend_days: Vec<Weekday>,
    ) -> BankingResult<()> {
        // Validate the configuration using the proper validation method
        CalendarValidation::validate_weekend_config_params(jurisdiction, &weekend_days)?;

        // Save to repository
        self.calendar_repository.set_weekend_days(jurisdiction, weekend_days).await?;

        Ok(())
    }

    /// Validate and update existing weekend configuration
    pub async fn update_weekend_config_for_jurisdiction(
        &self,
        jurisdiction: &str,
        weekend_days: Vec<Weekday>,
    ) -> BankingResult<()> {
        // Check if configuration exists by trying to get weekend days
        let existing_days = self.calendar_repository.get_weekend_days(jurisdiction).await?;
        if existing_days.is_empty() {
            return Err(BankingError::ValidationFailed(
                format!("Weekend configuration not found for jurisdiction: {jurisdiction}")
            ));
        }

        // Validate the configuration
        CalendarValidation::validate_weekend_config_params(jurisdiction, &weekend_days)?;

        // Update in repository using set_weekend_days (which handles updates)
        self.calendar_repository.set_weekend_days(jurisdiction, weekend_days).await?;

        Ok(())
    }
}
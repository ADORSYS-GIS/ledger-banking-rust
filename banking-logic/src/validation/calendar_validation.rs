use banking_api::{BankingResult, BankingError};
use chrono::{NaiveDate, Datelike, Weekday};

/// Comprehensive validation utilities for calendar and business day management
pub struct CalendarValidation;

impl CalendarValidation {
    /// Validate weekend days configuration
    /// Days are represented as integers: 1=Monday, 2=Tuesday, ..., 7=Sunday
    pub fn validate_weekend_days(weekend_days: &[i32]) -> BankingResult<()> {
        let mut validation_errors = Vec::new();
        let mut invalid_days = Vec::new();

        // Check if array is not empty
        if weekend_days.is_empty() {
            validation_errors.push("Weekend days array cannot be empty".to_string());
        }

        // Check if all days are valid (1-7)
        for &day in weekend_days {
            if !(1..=7).contains(&day) {
                invalid_days.push(day);
            }
        }

        if !invalid_days.is_empty() {
            return Err(BankingError::InvalidWeekendDays { invalid_days });
        }

        // Check for duplicates
        let mut unique_days = weekend_days.to_vec();
        unique_days.sort_unstable();
        unique_days.dedup();
        if unique_days.len() != weekend_days.len() {
            validation_errors.push("Weekend days array contains duplicate values".to_string());
        }

        // Business rule: Maximum 6 weekend days (must have at least 1 business day)
        if weekend_days.len() > 6 {
            validation_errors.push("Cannot have more than 6 weekend days (at least 1 business day required)".to_string());
        }

        if !validation_errors.is_empty() {
            return Err(BankingError::WeekendConfigValidationFailed { validation_errors });
        }

        Ok(())
    }

    /// Validate weekend configuration parameters
    pub fn validate_weekend_config_params(jurisdiction: &str, weekend_days: &[Weekday]) -> BankingResult<()> {
        let mut validation_errors = Vec::new();

        // Validate jurisdiction code (should be 2-3 characters)
        if jurisdiction.trim().is_empty() {
            validation_errors.push("Jurisdiction code cannot be empty".to_string());
        } else if jurisdiction.len() < 2 || jurisdiction.len() > 10 {
            validation_errors.push("Jurisdiction code must be between 2 and 10 characters".to_string());
        }

        // Validate jurisdiction format (should be uppercase)
        if jurisdiction != jurisdiction.to_uppercase() {
            validation_errors.push("Jurisdiction code should be uppercase".to_string());
        }

        // Validate weekend days
        if weekend_days.is_empty() {
            validation_errors.push("Weekend days cannot be empty".to_string());
        }

        // Validate we have at least one but not all days as weekend
        if weekend_days.len() >= 7 {
            validation_errors.push("Cannot have all 7 days as weekend days".to_string());
        }

        if !validation_errors.is_empty() {
            return Err(BankingError::WeekendConfigValidationFailed { validation_errors });
        }

        Ok(())
    }

    /// Check if a given date is a weekend based on weekend configuration (i32 version)
    pub fn is_weekend_day(date: NaiveDate, weekend_days: &[i32]) -> bool {
        let weekday_number = date.weekday().number_from_monday() as i32;
        weekend_days.contains(&weekday_number)
    }

    /// Check if a given date is a weekend based on weekend configuration (Weekday enum version)
    pub fn is_weekend_day_from_weekdays(date: NaiveDate, weekend_days: &[Weekday]) -> bool {
        weekend_days.contains(&date.weekday())
    }

    /// Convert weekday to integer representation (1=Monday, 7=Sunday)
    pub fn weekday_to_int(weekday: Weekday) -> i32 {
        weekday.number_from_monday() as i32
    }

    /// Convert integer to weekday (1=Monday, 7=Sunday)
    pub fn int_to_weekday(day: i32) -> Result<Weekday, String> {
        match day {
            1 => Ok(Weekday::Mon),
            2 => Ok(Weekday::Tue),
            3 => Ok(Weekday::Wed),
            4 => Ok(Weekday::Thu),
            5 => Ok(Weekday::Fri),
            6 => Ok(Weekday::Sat),
            7 => Ok(Weekday::Sun),
            _ => Err(format!("Invalid day number: {day}, must be between 1 and 7")),
        }
    }

    /// Get human-readable weekend day names
    pub fn get_weekend_day_names(weekend_days: &[i32]) -> Vec<String> {
        weekend_days
            .iter()
            .filter_map(|&day| {
                Self::int_to_weekday(day)
                    .ok()
                    .map(|weekday| format!("{weekday:?}"))
            })
            .collect()
    }

    /// Validate holiday configuration
    pub fn validate_holiday_config(
        jurisdiction: &str,
        holiday_date: NaiveDate,
        holiday_name: &str,
        holiday_type: &str,
    ) -> BankingResult<()> {
        let mut validation_errors = Vec::new();

        // Validate jurisdiction
        if jurisdiction.trim().is_empty() {
            validation_errors.push("Jurisdiction code cannot be empty".to_string());
        } else if jurisdiction.len() < 2 || jurisdiction.len() > 10 {
            validation_errors.push("Jurisdiction code must be between 2 and 10 characters".to_string());
        }

        // Validate holiday name
        if holiday_name.trim().is_empty() {
            validation_errors.push("Holiday name cannot be empty".to_string());
        } else if holiday_name.len() > 100 {
            validation_errors.push("Holiday name cannot exceed 100 characters".to_string());
        }

        // Validate holiday type
        let valid_types = ["National", "Regional", "Religious", "Bank", "Banking"];
        if !valid_types.contains(&holiday_type) {
            validation_errors.push(format!(
                "Invalid holiday type '{holiday_type}'. Must be one of: {valid_types:?}"
            ));
        }

        // Business rule: Holiday date should not be too far in the past (more than 10 years)
        let ten_years_ago = chrono::Utc::now().date_naive() - chrono::Duration::days(365 * 10);
        if holiday_date < ten_years_ago {
            validation_errors.push("Holiday date is too far in the past (more than 10 years)".to_string());
        }

        // Business rule: Holiday date should not be too far in the future (more than 5 years)
        let five_years_ahead = chrono::Utc::now().date_naive() + chrono::Duration::days(365 * 5);
        if holiday_date > five_years_ahead {
            validation_errors.push("Holiday date is too far in the future (more than 5 years)".to_string());
        }

        if !validation_errors.is_empty() {
            return Err(BankingError::WeekendConfigValidationFailed { validation_errors });
        }

        Ok(())
    }

    /// Calculate next business day based on weekend configuration and holidays
    pub fn next_business_day(
        from_date: NaiveDate,
        weekend_days: &[i32],
        holidays: &[NaiveDate],
    ) -> NaiveDate {
        let mut next_day = from_date + chrono::Duration::days(1);
        
        loop {
            // Check if it's a weekend
            if Self::is_weekend_day(next_day, weekend_days) {
                next_day += chrono::Duration::days(1);
                continue;
            }

            // Check if it's a holiday
            if holidays.contains(&next_day) {
                next_day += chrono::Duration::days(1);
                continue;
            }

            // Found a business day
            break;
        }

        next_day
    }

    /// Calculate previous business day based on weekend configuration and holidays
    pub fn previous_business_day(
        from_date: NaiveDate,
        weekend_days: &[i32],
        holidays: &[NaiveDate],
    ) -> NaiveDate {
        let mut prev_day = from_date - chrono::Duration::days(1);
        
        loop {
            // Check if it's a weekend
            if Self::is_weekend_day(prev_day, weekend_days) {
                prev_day -= chrono::Duration::days(1);
                continue;
            }

            // Check if it's a holiday
            if holidays.contains(&prev_day) {
                prev_day -= chrono::Duration::days(1);
                continue;
            }

            // Found a business day
            break;
        }

        prev_day
    }

    /// Check if a date is a business day
    pub fn is_business_day(
        date: NaiveDate,
        weekend_days: &[i32],
        holidays: &[NaiveDate],
    ) -> bool {
        !Self::is_weekend_day(date, weekend_days) && !holidays.contains(&date)
    }

    /// Count business days between two dates (exclusive of end date)
    pub fn count_business_days(
        start_date: NaiveDate,
        end_date: NaiveDate,
        weekend_days: &[i32],
        holidays: &[NaiveDate],
    ) -> i32 {
        if start_date >= end_date {
            return 0;
        }

        let mut count = 0;
        let mut current_date = start_date + chrono::Duration::days(1);

        while current_date < end_date {
            if Self::is_business_day(current_date, weekend_days, holidays) {
                count += 1;
            }
            current_date += chrono::Duration::days(1);
        }

        count
    }

    // ===== WEEKDAY ENUM VERSIONS =====
    // These methods work with Weekday enums instead of i32 arrays

    /// Calculate next business day using Weekday enums
    pub fn next_business_day_from_weekdays(
        from_date: NaiveDate,
        weekend_days: &[Weekday],
        holidays: &[NaiveDate],
    ) -> NaiveDate {
        let mut next_day = from_date + chrono::Duration::days(1);
        
        loop {
            // Check if it's a weekend
            if Self::is_weekend_day_from_weekdays(next_day, weekend_days) {
                next_day += chrono::Duration::days(1);
                continue;
            }
            
            // Check if it's a holiday
            if holidays.contains(&next_day) {
                next_day += chrono::Duration::days(1);
                continue;
            }
            
            // It's a business day
            break;
        }

        next_day
    }

    /// Calculate previous business day using Weekday enums
    pub fn previous_business_day_from_weekdays(
        from_date: NaiveDate,
        weekend_days: &[Weekday],
        holidays: &[NaiveDate],
    ) -> NaiveDate {
        let mut prev_day = from_date - chrono::Duration::days(1);
        
        loop {
            // Check if it's a weekend
            if Self::is_weekend_day_from_weekdays(prev_day, weekend_days) {
                prev_day -= chrono::Duration::days(1);
                continue;
            }
            
            // Check if it's a holiday
            if holidays.contains(&prev_day) {
                prev_day -= chrono::Duration::days(1);
                continue;
            }
            
            // It's a business day
            break;
        }

        prev_day
    }

    /// Check if a date is a business day using Weekday enums
    pub fn is_business_day_from_weekdays(
        date: NaiveDate,
        weekend_days: &[Weekday],
        holidays: &[NaiveDate],
    ) -> bool {
        !Self::is_weekend_day_from_weekdays(date, weekend_days) && !holidays.contains(&date)
    }

    /// Count business days between two dates using Weekday enums
    pub fn count_business_days_from_weekdays(
        from_date: NaiveDate,
        to_date: NaiveDate,
        weekend_days: &[Weekday],
        holidays: &[NaiveDate],
    ) -> i32 {
        let mut count = 0;
        let mut current_date = from_date;

        while current_date < to_date {
            if Self::is_business_day_from_weekdays(current_date, weekend_days, holidays) {
                count += 1;
            }
            current_date += chrono::Duration::days(1);
        }

        count
    }
}

/// Weekend configuration validation result
#[derive(Debug, Clone)]
pub struct WeekendConfigValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub weekend_day_names: Vec<String>,
    pub business_days_count: usize,
}

impl WeekendConfigValidationResult {
    pub fn success(weekend_days: &[i32]) -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            weekend_day_names: CalendarValidation::get_weekend_day_names(weekend_days),
            business_days_count: 7 - weekend_days.len(),
        }
    }

    pub fn failure(errors: Vec<String>) -> Self {
        Self {
            is_valid: false,
            errors,
            weekend_day_names: Vec::new(),
            business_days_count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_weekend_days() {
        // Standard Saturday-Sunday weekend
        let weekend_days = vec![6, 7];
        assert!(CalendarValidation::validate_weekend_days(&weekend_days).is_ok());

        // Friday-Saturday weekend (Middle East style)
        let weekend_days = vec![5, 6];
        assert!(CalendarValidation::validate_weekend_days(&weekend_days).is_ok());

        // Single day weekend
        let weekend_days = vec![7];
        assert!(CalendarValidation::validate_weekend_days(&weekend_days).is_ok());
    }

    #[test]
    fn test_invalid_weekend_days() {
        // Invalid day numbers
        let weekend_days = vec![0, 8];
        let result = CalendarValidation::validate_weekend_days(&weekend_days);
        assert!(result.is_err());
        
        if let Err(BankingError::InvalidWeekendDays { invalid_days }) = result {
            assert_eq!(invalid_days, vec![0, 8]);
        } else {
            panic!("Expected InvalidWeekendDays error");
        }

        // Empty array
        let weekend_days = vec![];
        assert!(CalendarValidation::validate_weekend_days(&weekend_days).is_err());

        // Too many weekend days
        let weekend_days = vec![1, 2, 3, 4, 5, 6, 7];
        assert!(CalendarValidation::validate_weekend_days(&weekend_days).is_err());

        // Duplicate days
        let weekend_days = vec![6, 7, 6];
        assert!(CalendarValidation::validate_weekend_days(&weekend_days).is_err());
    }

    #[test]
    fn test_weekend_day_detection() {
        let weekend_days = vec![6, 7]; // Saturday, Sunday
        
        // Test a known Saturday (2024-01-06)
        let saturday = NaiveDate::from_ymd_opt(2024, 1, 6).unwrap();
        assert!(CalendarValidation::is_weekend_day(saturday, &weekend_days));

        // Test a known Monday (2024-01-01)
        let monday = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        assert!(!CalendarValidation::is_weekend_day(monday, &weekend_days));
    }

    #[test]
    fn test_business_day_calculations() {
        let weekend_days = vec![6, 7]; // Saturday, Sunday
        let holidays = vec![
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), // New Year's Day
        ];

        // Test next business day
        let friday = NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(); // Friday
        let next_business = CalendarValidation::next_business_day(friday, &weekend_days, &holidays);
        
        // Should skip weekend and holiday, land on January 8th (Monday)
        let expected = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();
        assert_eq!(next_business, expected);

        // Test business day counting
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();
        let business_days = CalendarValidation::count_business_days(start, end, &weekend_days, &holidays);
        
        // Between Jan 1-8: Jan 2,3,4,5 are business days (Jan 1 is holiday, Jan 6,7 are weekend)
        assert_eq!(business_days, 4);
    }
}
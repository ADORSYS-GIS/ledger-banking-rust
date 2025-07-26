use chrono::{Weekday, Utc};
use banking_api::domain::{BankHoliday, HolidayType};
use banking_db::models::BankHolidayModel;

/// Mapper for converting between domain and database calendar models
pub struct CalendarMapper;

impl CalendarMapper {
    /// Convert domain BankHoliday to database BankHolidayModel
    pub fn holiday_to_model(holiday: BankHoliday) -> BankHolidayModel {
        BankHolidayModel {
            holiday_id: holiday.holiday_id,
            jurisdiction: holiday.jurisdiction,
            holiday_date: holiday.holiday_date,
            holiday_name: holiday.holiday_name,
            holiday_type: Self::holiday_type_to_string(&holiday.holiday_type),
            is_recurring: holiday.is_recurring,
            description: holiday.description,
            is_observed: true, // Default to observed
            observance_rule: None, // No special rules by default
            created_at: holiday.created_at,
            created_by: holiday.created_by.clone(),
            last_updated_at: Utc::now(),
            updated_by: holiday.created_by,
        }
    }

    /// Convert database BankHolidayModel to domain BankHoliday
    pub fn holiday_from_model(model: BankHolidayModel) -> BankHoliday {
        BankHoliday {
            holiday_id: model.holiday_id,
            jurisdiction: model.jurisdiction,
            holiday_date: model.holiday_date,
            holiday_name: model.holiday_name,
            holiday_type: Self::string_to_holiday_type(&model.holiday_type),
            is_recurring: model.is_recurring,
            description: model.description,
            created_by: model.created_by,
            created_at: model.created_at,
        }
    }

    /// Convert HolidayType enum to string
    fn holiday_type_to_string(holiday_type: &HolidayType) -> String {
        match holiday_type {
            HolidayType::National => "National".to_string(),
            HolidayType::Regional => "Regional".to_string(),
            HolidayType::Religious => "Religious".to_string(),
            HolidayType::Banking => "Banking".to_string(),
        }
    }

    /// Convert string to HolidayType enum
    fn string_to_holiday_type(s: &str) -> HolidayType {
        match s {
            "National" => HolidayType::National,
            "Regional" => HolidayType::Regional,
            "Religious" => HolidayType::Religious,
            "Bank" => HolidayType::Banking,
            "Banking" => HolidayType::Banking, // Handle legacy variant
            _ => HolidayType::National, // Default fallback
        }
    }

    /// Convert Weekday to integer (1=Monday, 7=Sunday)
    pub fn weekday_to_int(weekday: Weekday) -> i32 {
        weekday.number_from_monday() as i32
    }

    /// Convert integer to Weekday (1=Monday, 7=Sunday)
    pub fn int_to_weekday(day: i32) -> Option<Weekday> {
        match day {
            1 => Some(Weekday::Mon),
            2 => Some(Weekday::Tue),
            3 => Some(Weekday::Wed),
            4 => Some(Weekday::Thu),
            5 => Some(Weekday::Fri),
            6 => Some(Weekday::Sat),
            7 => Some(Weekday::Sun),
            _ => None,
        }
    }
}
use chrono::Weekday;
use banking_api::domain::{BankHoliday, HolidayType as DomainHolidayType};
use banking_db::models::{BankHolidayModel, HolidayType as ModelHolidayType};

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
            holiday_type: Self::domain_holiday_type_to_model(&holiday.holiday_type),
            is_recurring: holiday.is_recurring,
            description: holiday.description,
            created_at: holiday.created_at,
            created_by: holiday.created_by,
        }
    }

    /// Convert database BankHolidayModel to domain BankHoliday
    pub fn holiday_from_model(model: BankHolidayModel) -> BankHoliday {
        BankHoliday {
            holiday_id: model.holiday_id,
            jurisdiction: model.jurisdiction,
            holiday_date: model.holiday_date,
            holiday_name: model.holiday_name,
            holiday_type: Self::model_holiday_type_to_domain(&model.holiday_type),
            is_recurring: model.is_recurring,
            description: model.description,
            created_by: model.created_by,
            created_at: model.created_at,
        }
    }

    /// Convert domain HolidayType to model HolidayType
    fn domain_holiday_type_to_model(holiday_type: &DomainHolidayType) -> ModelHolidayType {
        match holiday_type {
            DomainHolidayType::National => ModelHolidayType::National,
            DomainHolidayType::Regional => ModelHolidayType::Regional,
            DomainHolidayType::Religious => ModelHolidayType::Religious,
            DomainHolidayType::Banking => ModelHolidayType::Banking,
        }
    }

    /// Convert model HolidayType to domain HolidayType
    fn model_holiday_type_to_domain(holiday_type: &ModelHolidayType) -> DomainHolidayType {
        match holiday_type {
            ModelHolidayType::National => DomainHolidayType::National,
            ModelHolidayType::Regional => DomainHolidayType::Regional,
            ModelHolidayType::Religious => DomainHolidayType::Religious,
            ModelHolidayType::Banking => DomainHolidayType::Banking,
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
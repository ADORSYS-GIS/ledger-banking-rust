use banking_api::domain::{
    BankHoliday, BusinessDayCalculation, DateCalculationRules, DateShiftRule as DomainDateShiftRule,
    HolidayType as DomainHolidayType, Weekday as DomainWeekday, WeekendDays,
};
use banking_db::models::{
    BankHolidayModel, BusinessDayCacheModel, DateCalculationRulesModel, DateShiftRule as ModelDateShiftRule,
    HolidayType as ModelHolidayType, Weekday as ModelWeekday, WeekendDaysModel,
};

/// Mapper for converting between domain and database calendar models
pub struct CalendarMapper;

impl CalendarMapper {
    /// Convert domain WeekendDays to database WeekendDaysModel
    pub fn weekend_days_to_model(weekend_days: WeekendDays) -> WeekendDaysModel {
        WeekendDaysModel {
            id: weekend_days.id,
            name_l1: weekend_days.name_l1,
            name_l2: weekend_days.name_l2,
            name_l3: weekend_days.name_l3,
            weekend_day_01: weekend_days.weekend_day_01.map(Self::domain_weekday_to_model),
            weekend_day_02: weekend_days.weekend_day_02.map(Self::domain_weekday_to_model),
            weekend_day_03: weekend_days.weekend_day_03.map(Self::domain_weekday_to_model),
            weekend_day_04: weekend_days.weekend_day_04.map(Self::domain_weekday_to_model),
            weekend_day_05: weekend_days.weekend_day_05.map(Self::domain_weekday_to_model),
            weekend_day_06: weekend_days.weekend_day_06.map(Self::domain_weekday_to_model),
            weekend_day_07: weekend_days.weekend_day_07.map(Self::domain_weekday_to_model),
            valid_from: weekend_days.valid_from,
            valid_to: weekend_days.valid_to,
            created_by_person_id: weekend_days.created_by_person_id,
            created_at: weekend_days.created_at,
        }
    }

    /// Convert database WeekendDaysModel to domain WeekendDays
    pub fn weekend_days_from_model(model: WeekendDaysModel) -> WeekendDays {
        WeekendDays {
            id: model.id,
            name_l1: model.name_l1,
            name_l2: model.name_l2,
            name_l3: model.name_l3,
            weekend_day_01: model.weekend_day_01.map(Self::model_weekday_to_domain),
            weekend_day_02: model.weekend_day_02.map(Self::model_weekday_to_domain),
            weekend_day_03: model.weekend_day_03.map(Self::model_weekday_to_domain),
            weekend_day_04: model.weekend_day_04.map(Self::model_weekday_to_domain),
            weekend_day_05: model.weekend_day_05.map(Self::model_weekday_to_domain),
            weekend_day_06: model.weekend_day_06.map(Self::model_weekday_to_domain),
            weekend_day_07: model.weekend_day_07.map(Self::model_weekday_to_domain),
            valid_from: model.valid_from,
            valid_to: model.valid_to,
            created_by_person_id: model.created_by_person_id,
            created_at: model.created_at,
        }
    }

    /// Convert domain Weekday to model Weekday
    fn domain_weekday_to_model(weekday: DomainWeekday) -> ModelWeekday {
        match weekday {
            DomainWeekday::Monday => ModelWeekday::Monday,
            DomainWeekday::Tuesday => ModelWeekday::Tuesday,
            DomainWeekday::Wednesday => ModelWeekday::Wednesday,
            DomainWeekday::Thursday => ModelWeekday::Thursday,
            DomainWeekday::Friday => ModelWeekday::Friday,
            DomainWeekday::Saturday => ModelWeekday::Saturday,
            DomainWeekday::Sunday => ModelWeekday::Sunday,
        }
    }

    /// Convert model Weekday to domain Weekday
    fn model_weekday_to_domain(weekday: ModelWeekday) -> DomainWeekday {
        match weekday {
            ModelWeekday::Monday => DomainWeekday::Monday,
            ModelWeekday::Tuesday => DomainWeekday::Tuesday,
            ModelWeekday::Wednesday => DomainWeekday::Wednesday,
            ModelWeekday::Thursday => DomainWeekday::Thursday,
            ModelWeekday::Friday => DomainWeekday::Friday,
            ModelWeekday::Saturday => DomainWeekday::Saturday,
            ModelWeekday::Sunday => DomainWeekday::Sunday,
        }
    }
    /// Convert domain BankHoliday to database BankHolidayModel
    pub fn holiday_to_model(holiday: BankHoliday) -> BankHolidayModel {
        BankHolidayModel {
            id: holiday.id,
            jurisdiction: holiday.jurisdiction,
            holiday_date: holiday.holiday_date,
            holiday_name: holiday.holiday_name,
            holiday_type: Self::domain_holiday_type_to_model(&holiday.holiday_type),
            is_recurring: holiday.is_recurring,
            description: holiday.description,
            created_at: holiday.created_at,
            created_by_person_id: holiday.created_by_person_id,
        }
    }

    /// Convert database BankHolidayModel to domain BankHoliday
    pub fn holiday_from_model(model: BankHolidayModel) -> BankHoliday {
        BankHoliday {
            id: model.id,
            jurisdiction: model.jurisdiction,
            holiday_date: model.holiday_date,
            holiday_name: model.holiday_name,
            holiday_type: Self::model_holiday_type_to_domain(&model.holiday_type),
            is_recurring: model.is_recurring,
            description: model.description,
            created_by_person_id: model.created_by_person_id,
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

    /// Convert domain DateShiftRule to model DateShiftRule
    fn domain_date_shift_rule_to_model(date_shift_rule: &DomainDateShiftRule) -> ModelDateShiftRule {
        match date_shift_rule {
            DomainDateShiftRule::NextBusinessDay => ModelDateShiftRule::NextBusinessDay,
            DomainDateShiftRule::PreviousBusinessDay => ModelDateShiftRule::PreviousBusinessDay,
            DomainDateShiftRule::NoShift => ModelDateShiftRule::NoShift,
        }
    }

    /// Convert model DateShiftRule to domain DateShiftRule
    fn model_date_shift_rule_to_domain(date_shift_rule: &ModelDateShiftRule) -> DomainDateShiftRule {
        match date_shift_rule {
            ModelDateShiftRule::NextBusinessDay => DomainDateShiftRule::NextBusinessDay,
            ModelDateShiftRule::PreviousBusinessDay => DomainDateShiftRule::PreviousBusinessDay,
            ModelDateShiftRule::NoShift => DomainDateShiftRule::NoShift,
        }
    }

    /// Convert domain DateCalculationRules to model DateCalculationRulesModel
    pub fn date_calculation_rules_to_model(rules: DateCalculationRules) -> DateCalculationRulesModel {
        DateCalculationRulesModel {
            id: uuid::Uuid::new_v4(), // Generate new ID for model
            jurisdiction: rules.jurisdiction,
            rule_name: heapless::String::try_from("Default").unwrap_or_default(),
            rule_type: heapless::String::try_from("DateShift").unwrap_or_default(),
            default_shift_rule: Self::domain_date_shift_rule_to_model(&rules.default_shift_rule),
            weekend_days_id: rules.weekend_days,
            product_specific_overrides: None,
            priority: 1,
            is_active: true,
            effective_date: chrono::Utc::now().date_naive(),
            expiry_date: None,
            created_at: chrono::Utc::now(),
            created_by_person_id: uuid::Uuid::new_v4(),
            last_updated_at: chrono::Utc::now(),
            updated_by_person_id: uuid::Uuid::new_v4(),
        }
    }

    /// Convert model DateCalculationRulesModel to domain DateCalculationRules
    pub fn date_calculation_rules_from_model(model: DateCalculationRulesModel) -> Result<DateCalculationRules, &'static str> {
        DateCalculationRules::new(
            Self::model_date_shift_rule_to_domain(&model.default_shift_rule),
            model.weekend_days_id,
            model.jurisdiction.as_str(),
        )
    }

    /// Convert domain BusinessDayCalculation to model BusinessDayCacheModel
    pub fn business_day_calculation_to_model(calculation: BusinessDayCalculation) -> BusinessDayCacheModel {
        BusinessDayCacheModel {
            id: uuid::Uuid::new_v4(),
            jurisdiction: calculation.jurisdiction,
            date: calculation.adjusted_date,
            is_business_day: calculation.is_business_day,
            is_holiday: !calculation.is_business_day,
            is_weekend: false, // Would need additional logic to determine
            holiday_name: None,
            cached_at: chrono::Utc::now(),
            valid_until: chrono::Utc::now() + chrono::Duration::hours(24),
        }
    }
}
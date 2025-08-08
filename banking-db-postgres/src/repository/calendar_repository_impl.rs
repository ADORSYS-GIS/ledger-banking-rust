use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{NaiveDate, Weekday};
use heapless::String as HeaplessString;

use banking_api::BankingResult;
use banking_db::models::{BankHolidayModel, HolidayType};
use banking_db::repository::{CalendarRepository, ValidationResult, ImportResult, CalendarSummaryReport};

pub struct CalendarRepositoryImpl {
    pool: PgPool,
}

impl CalendarRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CalendarRepository for CalendarRepositoryImpl {
    /// Get weekend configuration for jurisdiction
    async fn get_weekend_days(&self, jurisdiction: &str) -> BankingResult<Vec<Weekday>> {
        let config = sqlx::query!(
            r#"
            SELECT weekend_days
            FROM weekend_configuration
            WHERE jurisdiction = $1 AND is_active = true
            ORDER BY effective_date DESC
            LIMIT 1
            "#,
            jurisdiction
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(config) = config {
            // Parse JSON array of integers to Weekday enum
            let day_numbers: Vec<i32> = serde_json::from_str(&config.weekend_days)
                .unwrap_or_else(|_| vec![6, 7]); // Default to Saturday, Sunday
            
            let weekend_days: Vec<Weekday> = day_numbers
                .iter()
                .filter_map(|&day| match day {
                    1 => Some(Weekday::Mon),
                    2 => Some(Weekday::Tue), 
                    3 => Some(Weekday::Wed),
                    4 => Some(Weekday::Thu),
                    5 => Some(Weekday::Fri),
                    6 => Some(Weekday::Sat),
                    7 => Some(Weekday::Sun),
                    _ => None,
                })
                .collect();
            Ok(weekend_days)
        } else {
            // Default to Saturday and Sunday
            Ok(vec![Weekday::Sat, Weekday::Sun])
        }
    }

    /// Set weekend configuration
    async fn set_weekend_days(&self, jurisdiction: &str, weekend_days: Vec<Weekday>) -> BankingResult<()> {
        // Convert Weekday enum to integers
        let day_numbers: Vec<i32> = weekend_days
            .iter()
            .map(|day| match day {
                Weekday::Mon => 1,
                Weekday::Tue => 2,
                Weekday::Wed => 3, 
                Weekday::Thu => 4,
                Weekday::Fri => 5,
                Weekday::Sat => 6,
                Weekday::Sun => 7,
            })
            .collect();

        let config_id = Uuid::new_v4();
        let now = chrono::Utc::now();
        let system_user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000")
            .map_err(|e| banking_api::BankingError::ValidationError { 
                field: "system_user_id".to_string(), 
                message: format!("Invalid UUID: {e}") 
            })?;
        let weekend_days_json = serde_json::to_string(&day_numbers)
            .map_err(|e| banking_api::BankingError::ValidationError { 
                field: "weekend_days".to_string(), 
                message: format!("JSON serialization error: {e}") 
            })?;

        sqlx::query!(
            r#"
            INSERT INTO weekend_configuration (
                id, jurisdiction, weekend_days, effective_date, 
                is_active, created_at, created_by, last_updated_at, updated_by
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (jurisdiction) 
            DO UPDATE SET
                weekend_days = EXCLUDED.weekend_days,
                effective_date = EXCLUDED.effective_date,
                is_active = EXCLUDED.is_active,
                last_updated_at = EXCLUDED.last_updated_at,
                updated_by = EXCLUDED.updated_by
            "#,
            config_id,
            jurisdiction,
            weekend_days_json,
            now.date_naive(),
            true,
            now,
            system_user_id,
            now,
            system_user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Find holidays in date range
    async fn find_holidays_in_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
        jurisdiction: &str,
    ) -> BankingResult<Vec<BankHolidayModel>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, jurisdiction, holiday_date, holiday_name, 
                   holiday_type as "holiday_type: HolidayType", is_recurring, description, 
                   created_at, created_by
            FROM bank_holidays
            WHERE jurisdiction = $1 
              AND holiday_date >= $2 
              AND holiday_date <= $3
            ORDER BY holiday_date
            "#,
            jurisdiction,
            start_date,
            end_date
        )
        .fetch_all(&self.pool)
        .await?;

        // Convert query results to BankHolidayModel
        let holidays = rows.into_iter().map(|row| {
            let jurisdiction_str = HeaplessString::try_from(row.jurisdiction.as_str())
                .unwrap_or_else(|_| HeaplessString::new());
            let holiday_name_str = HeaplessString::try_from(row.holiday_name.as_str())
                .unwrap_or_else(|_| HeaplessString::new());
            
            BankHolidayModel {
                id: row.id,
                jurisdiction: jurisdiction_str,
                holiday_date: row.holiday_date,
                holiday_name: holiday_name_str,
                holiday_type: row.holiday_type,
                is_recurring: row.is_recurring,
                description: row.description.map(|d| HeaplessString::try_from(d.as_str()).unwrap_or_else(|_| HeaplessString::new())),
                created_at: row.created_at,
                created_by: row.created_by,
            }
        }).collect();

        Ok(holidays)
    }

    /// Create holiday
    async fn create_holiday(&self, holiday: BankHolidayModel) -> BankingResult<BankHolidayModel> {
        let description_str = holiday.description.as_ref().map(|d| d.as_str());
        
        let row = sqlx::query!(
            r#"
            INSERT INTO bank_holidays (
                id, jurisdiction, holiday_date, holiday_name,
                holiday_type, is_recurring, description, created_at, created_by
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, jurisdiction, holiday_date, holiday_name, 
                      holiday_type as "holiday_type: HolidayType", is_recurring, description, 
                      created_at, created_by
            "#,
            holiday.id,
            holiday.jurisdiction.as_str(),
            holiday.holiday_date,
            holiday.holiday_name.as_str(),
            holiday.holiday_type as HolidayType,
            holiday.is_recurring,
            description_str,
            holiday.created_at,
            holiday.created_by
        )
        .fetch_one(&self.pool)
        .await?;

        let jurisdiction_str = HeaplessString::try_from(row.jurisdiction.as_str())
            .unwrap_or_else(|_| HeaplessString::new());
        let holiday_name_str = HeaplessString::try_from(row.holiday_name.as_str())
            .unwrap_or_else(|_| HeaplessString::new());

        Ok(BankHolidayModel {
            id: row.id,
            jurisdiction: jurisdiction_str,
            holiday_date: row.holiday_date,
            holiday_name: holiday_name_str,
            holiday_type: row.holiday_type,
            is_recurring: row.is_recurring,
            description: row.description.map(|d| HeaplessString::try_from(d.as_str()).unwrap_or_else(|_| HeaplessString::new())),
            created_at: row.created_at,
            created_by: row.created_by,
        })
    }

    /// Delete holiday
    async fn delete_holiday(&self, id: Uuid) -> BankingResult<()> {
        sqlx::query!(
            "DELETE FROM bank_holidays WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Find holidays by year
    async fn find_holidays_by_year(
        &self,
        year: i32,
        jurisdiction: &str,
    ) -> BankingResult<Vec<BankHolidayModel>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, jurisdiction, holiday_date, holiday_name, 
                   holiday_type as "holiday_type: HolidayType", is_recurring, description, 
                   created_at, created_by
            FROM bank_holidays
            WHERE jurisdiction = $1 
              AND EXTRACT(YEAR FROM holiday_date) = $2
            ORDER BY holiday_date
            "#,
            jurisdiction,
            year as f64
        )
        .fetch_all(&self.pool)
        .await?;

        // Convert query results to BankHolidayModel
        let holidays = rows.into_iter().map(|row| {
            let jurisdiction_str = HeaplessString::try_from(row.jurisdiction.as_str())
                .unwrap_or_else(|_| HeaplessString::new());
            let holiday_name_str = HeaplessString::try_from(row.holiday_name.as_str())
                .unwrap_or_else(|_| HeaplessString::new());
                
            BankHolidayModel {
                id: row.id,
                jurisdiction: jurisdiction_str,
                holiday_date: row.holiday_date,
                holiday_name: holiday_name_str,
                holiday_type: row.holiday_type,
                is_recurring: row.is_recurring,
                description: row.description.map(|d| HeaplessString::try_from(d.as_str()).unwrap_or_else(|_| HeaplessString::new())),
                created_at: row.created_at,
                created_by: row.created_by,
            }
        }).collect();

        Ok(holidays)
    }

    /// Check if holiday exists by date and jurisdiction
    async fn holiday_exists(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<bool> {
        let exists = sqlx::query!(
            "SELECT 1 as exists FROM bank_holidays WHERE holiday_date = $1 AND jurisdiction = $2",
            date,
            jurisdiction
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some();

        Ok(exists)
    }

    // Stub implementations for remaining trait methods - TODO: Implement properly
    async fn update_holiday(&self, holiday: BankHolidayModel) -> BankingResult<BankHolidayModel> {
        // TODO: Implement holiday update
        Ok(holiday)
    }

    async fn find_holiday_by_id(&self, _id: Uuid) -> BankingResult<Option<BankHolidayModel>> {
        // TODO: Implement holiday lookup by ID
        Ok(None)
    }

    async fn find_holiday_by_date(&self, _date: NaiveDate, _jurisdiction: &str) -> BankingResult<Option<BankHolidayModel>> {
        // TODO: Implement holiday lookup by date
        Ok(None)
    }

    async fn find_holidays_by_jurisdiction(&self, _jurisdiction: &str) -> BankingResult<Vec<BankHolidayModel>> {
        // TODO: Implement holidays by jurisdiction
        Ok(vec![])
    }

    async fn find_holidays_by_type(&self, _holiday_type: &str) -> BankingResult<Vec<BankHolidayModel>> {
        // TODO: Implement holidays by type
        Ok(vec![])
    }

    async fn is_holiday(&self, _date: NaiveDate, _jurisdiction: &str) -> BankingResult<bool> {
        // TODO: Implement holiday check
        Ok(false)
    }

    async fn is_weekend(&self, _date: NaiveDate, _jurisdiction: &str) -> BankingResult<bool> {
        // TODO: Implement weekend check
        Ok(false)
    }

    async fn is_business_day(&self, _date: NaiveDate, _jurisdiction: &str) -> BankingResult<bool> {
        // TODO: Implement business day check
        Ok(true)
    }

    async fn next_business_day(&self, date: NaiveDate, _jurisdiction: &str) -> BankingResult<NaiveDate> {
        // TODO: Implement proper business day calculation
        Ok(date + chrono::Duration::days(1))
    }

    async fn previous_business_day(&self, date: NaiveDate, _jurisdiction: &str) -> BankingResult<NaiveDate> {
        // TODO: Implement proper business day calculation
        Ok(date - chrono::Duration::days(1))
    }

    async fn add_business_days(&self, date: NaiveDate, days: i32, _jurisdiction: &str) -> BankingResult<NaiveDate> {
        // TODO: Implement proper business day addition
        Ok(date + chrono::Duration::days(days as i64))
    }

    async fn subtract_business_days(&self, date: NaiveDate, days: i32, _jurisdiction: &str) -> BankingResult<NaiveDate> {
        // TODO: Implement proper business day subtraction
        Ok(date - chrono::Duration::days(days as i64))
    }

    async fn count_business_days(&self, _start_date: NaiveDate, _end_date: NaiveDate, _jurisdiction: &str) -> BankingResult<i32> {
        // TODO: Implement business day counting
        Ok(0)
    }

    async fn find_recurring_holidays(&self, _jurisdiction: &str) -> BankingResult<Vec<BankHolidayModel>> {
        // TODO: Implement recurring holidays lookup
        Ok(vec![])
    }

    async fn generate_recurring_holidays(&self, _year: i32, _jurisdiction: &str) -> BankingResult<Vec<BankHolidayModel>> {
        // TODO: Implement recurring holiday generation
        Ok(vec![])
    }

    async fn create_recurring_holidays_for_year(&self, _year: i32, _jurisdiction: &str) -> BankingResult<i64> {
        // TODO: Implement recurring holiday creation
        Ok(0)
    }

    async fn apply_date_shift_rule(&self, date: NaiveDate, _jurisdiction: &str, _shift_rule: &str) -> BankingResult<NaiveDate> {
        // TODO: Implement date shift rules
        Ok(date)
    }

    async fn get_maturity_date(&self, start_date: NaiveDate, term_months: i32, _jurisdiction: &str) -> BankingResult<NaiveDate> {
        // TODO: Implement proper maturity date calculation
        let months = chrono::Months::new(term_months as u32);
        Ok(start_date + months)
    }

    async fn get_payment_due_date(&self, original_date: NaiveDate, _jurisdiction: &str, _product_code: Option<&str>) -> BankingResult<NaiveDate> {
        // TODO: Implement payment due date calculation
        Ok(original_date)
    }

    async fn bulk_create_holidays(&self, _holidays: Vec<BankHolidayModel>) -> BankingResult<i64> {
        // TODO: Implement bulk holiday creation
        Ok(0)
    }

    async fn delete_holidays_by_year(&self, _year: i32, _jurisdiction: &str) -> BankingResult<i64> {
        // TODO: Implement holiday deletion by year
        Ok(0)
    }

    async fn cleanup_past_holidays(&self, _before_date: NaiveDate) -> BankingResult<i64> {
        // TODO: Implement past holiday cleanup
        Ok(0)
    }

    async fn get_supported_jurisdictions(&self) -> BankingResult<Vec<String>> {
        // TODO: Implement jurisdiction listing
        Ok(vec![])
    }

    async fn add_jurisdiction(&self, _jurisdiction: &str, _weekend_days: Vec<Weekday>) -> BankingResult<()> {
        // TODO: Implement jurisdiction addition
        Ok(())
    }

    async fn remove_jurisdiction(&self, _jurisdiction: &str) -> BankingResult<()> {
        // TODO: Implement jurisdiction removal
        Ok(())
    }

    async fn validate_holiday_data(&self, _holidays: Vec<BankHolidayModel>) -> BankingResult<ValidationResult> {
        // TODO: Implement holiday data validation
        Ok(ValidationResult {
            valid_holidays: vec![],
            invalid_holidays: vec![],
            duplicate_holidays: vec![],
            warnings: vec![],
        })
    }

    async fn import_holidays_from_source(&self, _jurisdiction: &str, _year: i32, _source: &str) -> BankingResult<ImportResult> {
        // TODO: Implement holiday import
        Ok(ImportResult {
            jurisdiction: "".to_string(),
            year: 0,
            holidays_imported: 0,
            holidays_skipped: 0,
            errors: vec![],
            import_source: "".to_string(),
        })
    }

    async fn get_calendar_summary(&self, _jurisdiction: &str, _year: i32) -> BankingResult<CalendarSummaryReport> {
        // TODO: Implement calendar summary
        Ok(CalendarSummaryReport {
            jurisdiction: "".to_string(),
            year: 0,
            total_days: 0,
            total_holidays: 0,
            total_weekends: 0,
            total_business_days: 0,
            holidays_by_type: vec![],
            holidays_by_month: vec![],
        })
    }

    async fn get_business_days_in_month(&self, _year: i32, _month: u32, _jurisdiction: &str) -> BankingResult<i32> {
        // TODO: Implement business days in month calculation
        Ok(22) // Typical business days in a month
    }

    async fn get_business_days_in_year(&self, _year: i32, _jurisdiction: &str) -> BankingResult<i32> {
        // TODO: Implement business days in year calculation
        Ok(252) // Typical business days in a year
    }

    async fn refresh_calendar_cache(&self, _jurisdiction: &str) -> BankingResult<()> {
        // TODO: Implement cache refresh
        Ok(())
    }

    async fn invalidate_calendar_cache(&self, _jurisdiction: &str) -> BankingResult<()> {
        // TODO: Implement cache invalidation
        Ok(())
    }

    async fn count_holidays(&self, _jurisdiction: &str) -> BankingResult<i64> {
        // TODO: Implement holiday counting
        Ok(0)
    }

    async fn count_holidays_by_type(&self, _holiday_type: &str, _jurisdiction: &str) -> BankingResult<i64> {
        // TODO: Implement holiday counting by type
        Ok(0)
    }

    async fn list_holidays(&self, _offset: i64, _limit: i64) -> BankingResult<Vec<BankHolidayModel>> {
        // TODO: Implement holiday listing with pagination
        Ok(vec![])
    }
}
use async_trait::async_trait;
use banking_api::{BankingError, BankingResult};
use std::str::FromStr;
use banking_db::models::{
    AccountBalanceCalculationModel, AccountHoldExpiryJobModel,
    AccountHoldModel, AccountHoldReleaseRequestModel, AccountHoldSummaryModel,
    PlaceHoldRequestModel, HighHoldRatioAccount, HoldAgingBucket, HoldAnalyticsSummary,
    HoldOverrideRecord, HoldPrioritySummary, HoldValidationError,
    JudicialHoldReportData,
};
use banking_db::repository::account_hold_repository::AccountHoldRepository;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use sqlx::{PgPool};
use uuid::Uuid;

use banking_db::models::{HoldPriority, HoldStatus, HoldType};
use sqlx::Row;

trait TryFromRow<R> {
    fn try_from_row(row: &R) -> BankingResult<Self>
    where
        Self: Sized;
}

impl TryFromRow<sqlx::postgres::PgRow> for AccountHoldModel {
    fn try_from_row(row: &sqlx::postgres::PgRow) -> BankingResult<Self> {
        Ok(AccountHoldModel {
            id: row.try_get("id")?,
            account_id: row.try_get("account_id")?,
            amount: row.try_get("amount")?,
            hold_type: row.try_get::<String, _>("hold_type")?.parse::<HoldType>().unwrap(),
            reason_id: row.try_get("reason_id")?,
            additional_details: row.try_get::<Option<String>, _>("additional_details")?.map(|s| heapless::String::from_str(&s).unwrap()),
            placed_by_person_id: row.try_get("placed_by_person_id")?,
            placed_at: row.try_get("placed_at")?,
            expires_at: row.try_get("expires_at")?,
            status: row.try_get::<String, _>("status")?.parse::<HoldStatus>().unwrap(),
            released_at: row.try_get("released_at")?,
            released_by_person_id: row.try_get("released_by_person_id")?,
            priority: row.try_get::<String, _>("priority")?.parse::<HoldPriority>().unwrap(),
            source_reference: row.try_get::<Option<String>, _>("source_reference")?.map(|s| heapless::String::from_str(&s).unwrap()),
            automatic_release: row.try_get("automatic_release")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}
pub struct AccountHoldRepositoryImpl {
    pool: PgPool,
}

impl AccountHoldRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccountHoldRepository for AccountHoldRepositoryImpl {
    async fn create_hold(&self, hold: AccountHoldModel) -> BankingResult<AccountHoldModel> {
        let result = sqlx::query(
            r#"
            INSERT INTO account_holds (
                id, account_id, amount, hold_type, reason_id, additional_details,
                placed_by_person_id, placed_at, expires_at, status, released_at, released_by_person_id,
                priority, source_reference, automatic_release
            )
            VALUES ($1, $2, $3, $4::hold_type, $5, $6, $7, $8, $9, $10::hold_status, $11, $12, $13::hold_priority, $14, $15)
            RETURNING id, account_id, amount, hold_type::text as hold_type, reason_id,
                     additional_details, placed_by_person_id, placed_at, expires_at, status::text as status,
                     released_at, released_by_person_id, priority::text as priority, source_reference, automatic_release,
                     created_at, updated_at
            "#,
        )
        .bind(hold.id)
        .bind(hold.account_id)
        .bind(hold.amount)
        .bind(hold.hold_type.to_string())
        .bind(hold.reason_id)
        .bind(hold.additional_details.as_deref())
        .bind(hold.placed_by_person_id)
        .bind(hold.placed_at)
        .bind(hold.expires_at)
        .bind(hold.status.to_string())
        .bind(hold.released_at)
        .bind(hold.released_by_person_id)
        .bind(hold.priority.to_string())
        .bind(hold.source_reference.as_deref())
        .bind(hold.automatic_release)
        .fetch_one(&self.pool)
        .await?;

        Ok(AccountHoldModel::try_from_row(&result)?)
    }

    async fn find_holds_by_account(&self, account_id: Uuid) -> BankingResult<Vec<AccountHoldModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, amount, hold_type::text as hold_type, reason_id,
                   additional_details, placed_by_person_id, placed_at, expires_at, status::text as status,
                   released_at, released_by_person_id, priority::text as priority, source_reference, automatic_release,
                   created_at, updated_at
            FROM account_holds
            WHERE account_id = $1
            ORDER BY placed_at DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await?;

        let mut holds = Vec::new();
        for row in rows {
            holds.push(AccountHoldModel::try_from_row(&row)?);
        }
        Ok(holds)
    }

    async fn find_active_holds(&self, account_id: Uuid) -> BankingResult<Vec<AccountHoldModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, amount, hold_type::text as hold_type, reason_id,
                   additional_details, placed_by_person_id, placed_at, expires_at, status::text as status,
                   released_at, released_by_person_id, priority::text as priority, source_reference, automatic_release,
                   created_at, updated_at
            FROM account_holds
            WHERE account_id = $1 AND status = 'Active'
              AND (expires_at IS NULL OR expires_at > NOW())
            ORDER BY placed_at DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await?;

        let mut holds = Vec::new();
        for row in rows {
            holds.push(AccountHoldModel::try_from_row(&row)?);
        }
        Ok(holds)
    }

    async fn release_hold(&self, hold_id: Uuid, released_by_person_id: Uuid) -> BankingResult<()> {
        sqlx::query(
            r#"
            UPDATE account_holds
            SET status = 'Released',
                released_at = NOW(),
                released_by_person_id = $2
            WHERE id = $1
            "#,
        )
        .bind(hold_id)
        .bind(released_by_person_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn release_expired_holds(&self, reference_date: DateTime<Utc>) -> BankingResult<i64> {
        let result = sqlx::query(
            r#"
            UPDATE account_holds
            SET status = 'Expired',
                released_at = NOW()
            WHERE status = 'Active'
              AND expires_at IS NOT NULL
              AND expires_at <= $1
              AND automatic_release = true
            "#,
        )
        .bind(reference_date)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() as i64)
    }

    // Additional Hold Methods - Migrated from HoldRepositoryImpl
    async fn update_hold(&self, hold: AccountHoldModel) -> BankingResult<AccountHoldModel> {
        let row = sqlx::query(
            "UPDATE account_holds SET
                amount = $2, hold_type = $3::hold_type, reason_id = $4, additional_details = $5,
                expires_at = $6, status = $7::hold_status, released_at = $8, released_by_person_id = $9,
                priority = $10::hold_priority, source_reference = $11, automatic_release = $12, updated_at = NOW()
            WHERE id = $1
            RETURNING id, account_id, amount, hold_type::text, reason_id, additional_details,
                     placed_by_person_id, placed_at, expires_at, status::text, released_at, released_by_person_id,
                     priority::text, source_reference, automatic_release, created_at, updated_at"
        )
        .bind(hold.id)
        .bind(hold.amount)
        .bind(hold.hold_type.to_string())
        .bind(hold.reason_id)
        .bind(hold.additional_details.as_deref())
        .bind(hold.expires_at)
        .bind(hold.status.to_string())
        .bind(hold.released_at)
        .bind(hold.released_by_person_id)
        .bind(hold.priority.to_string())
        .bind(hold.source_reference.as_deref())
        .bind(hold.automatic_release)
        .fetch_one(&self.pool)
        .await
        .map_err(BankingError::from)?;

        AccountHoldModel::try_from_row(&row)
    }

    async fn get_hold_by_id(&self, hold_id: Uuid) -> BankingResult<Option<AccountHoldModel>> {
        let row = sqlx::query(
            "SELECT id, account_id, amount, hold_type::text, reason_id, additional_details,
                    placed_by_person_id, placed_at, expires_at, status::text, released_at, released_by_person_id,
                    priority::text, source_reference, automatic_release, created_at, updated_at
             FROM account_holds WHERE id = $1"
        )
        .bind(hold_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(BankingError::from)?;

        match row {
            Some(row) => Ok(Some(AccountHoldModel::try_from_row(&row)?)),
            None => Ok(None),
        }
    }

    async fn get_active_holds_for_account(&self, account_id: Uuid, hold_types: Option<Vec<String>>) -> BankingResult<Vec<AccountHoldModel>> {
        let mut query = "SELECT id, account_id, amount, hold_type::text, reason_id, additional_details,
                                placed_by_person_id, placed_at, expires_at, status::text, released_at, released_by_person_id,
                                priority::text, source_reference, automatic_release, created_at, updated_at
                         FROM account_holds WHERE account_id = $1 AND status = 'Active'".to_string();

        if let Some(types) = &hold_types {
            if !types.is_empty() {
                let type_placeholders: Vec<String> = (2..=types.len() + 1)
                    .map(|i| format!("${i}"))
                    .collect();
                query.push_str(&format!(" AND hold_type::text IN ({})", type_placeholders.join(",")));
            }
        }

        query.push_str(" ORDER BY priority DESC, placed_at ASC");

        let mut sql_query = sqlx::query(&query).bind(account_id);

        if let Some(types) = &hold_types {
            for hold_type in types {
                sql_query = sql_query.bind(hold_type);
            }
        }

        let rows = sql_query.fetch_all(&self.pool).await.map_err(BankingError::from)?;

        let mut holds = Vec::new();
        for row in rows {
            holds.push(AccountHoldModel::try_from_row(&row)?);
        }

        Ok(holds)
    }

    async fn get_holds_by_status(&self, account_id: Option<Uuid>, status: String, from_date: Option<NaiveDate>, to_date: Option<NaiveDate>) -> BankingResult<Vec<AccountHoldModel>> {
        let mut query = "SELECT id, account_id, amount, hold_type::text, reason_id, additional_details,
                                placed_by_person_id, placed_at, expires_at, status::text, released_at, released_by_person_id,
                                priority::text, source_reference, automatic_release, created_at, updated_at
                         FROM account_holds WHERE status::text = $1".to_string();

        let mut param_count = 1;

        if account_id.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND account_id = ${param_count}"));
        }

        if from_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND placed_at >= ${param_count}"));
        }

        if to_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND placed_at < ${param_count}"));
        }

        query.push_str(" ORDER BY priority DESC, placed_at ASC");

        let mut sql_query = sqlx::query(&query).bind(status);

        if let Some(acct_id) = account_id {
            sql_query = sql_query.bind(acct_id);
        }

        if let Some(from) = from_date {
            sql_query = sql_query.bind(from.and_hms_opt(0, 0, 0).unwrap().and_utc());
        }

        if let Some(to) = to_date {
            sql_query = sql_query.bind(to.and_hms_opt(23, 59, 59).unwrap().and_utc());
        }

        let rows = sql_query
            .fetch_all(&self.pool)
            .await
            .map_err(BankingError::from)?;

        let mut holds = Vec::new();
        for row in rows {
            holds.push(AccountHoldModel::try_from_row(&row)?);
        }

        Ok(holds)
    }

    #[allow(dead_code, unused_variables)]
    async fn get_holds_by_type(&self, hold_type: String, status: Option<String>, account_ids: Option<Vec<Uuid>>, limit: Option<i32>) -> BankingResult<Vec<AccountHoldModel>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn get_hold_history(&self, account_id: Uuid, from_date: Option<NaiveDate>, to_date: Option<NaiveDate>, include_released: bool) -> BankingResult<Vec<AccountHoldModel>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn calculate_total_holds(&self, account_id: Uuid, exclude_hold_types: Option<Vec<String>>) -> BankingResult<Decimal> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn get_hold_amounts_by_priority(&self, account_id: Uuid) -> BankingResult<Vec<HoldPrioritySummary>> {
        unimplemented!()
    }


    #[allow(unused_variables)]
    async fn cache_balance_calculation(&self, calculation: AccountBalanceCalculationModel) -> BankingResult<AccountBalanceCalculationModel> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn get_cached_balance_calculation(&self, account_id: Uuid, max_age_seconds: u64) -> BankingResult<Option<AccountBalanceCalculationModel>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn release_hold_detailed(&self, hold_id: Uuid, release_amount: Option<Decimal>, release_reason_id: Uuid, released_by: Uuid, released_at: DateTime<Utc>) -> BankingResult<AccountHoldModel> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn create_hold_release_record(&self, release_record: banking_db::models::HoldReleaseRecordModel) -> BankingResult<banking_db::models::HoldReleaseRecordModel> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn get_hold_release_records(&self, hold_id: Uuid) -> BankingResult<Vec<banking_db::models::HoldReleaseRecordModel>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn bulk_release_holds(&self, hold_ids: Vec<Uuid>, release_reason_id: Uuid, released_by: Uuid) -> BankingResult<Vec<AccountHoldModel>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn get_expired_holds(&self, cutoff_date: DateTime<Utc>, hold_types: Option<Vec<String>>, limit: Option<i32>) -> BankingResult<Vec<AccountHoldModel>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn get_auto_release_eligible_holds(&self, processing_date: NaiveDate, hold_types: Option<Vec<String>>) -> BankingResult<Vec<AccountHoldModel>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn create_hold_expiry_job(&self, job: AccountHoldExpiryJobModel) -> BankingResult<AccountHoldExpiryJobModel> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn update_hold_expiry_job(&self, job: AccountHoldExpiryJobModel) -> BankingResult<AccountHoldExpiryJobModel> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn bulk_place_holds(&self, holds: Vec<AccountHoldModel>) -> BankingResult<Vec<AccountHoldModel>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn update_hold_priorities(&self, account_id: Uuid, hold_priority_updates: Vec<(Uuid, String)>, updated_by_person_id: Uuid) -> BankingResult<Vec<AccountHoldModel>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn get_overrideable_holds(&self, account_id: Uuid, required_amount: Decimal, override_priority: String) -> BankingResult<Vec<AccountHoldModel>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn create_hold_override(&self, account_id: Uuid, overridden_holds: Vec<Uuid>, override_amount: Decimal, authorized_by: Uuid, override_reason_id: Uuid) -> BankingResult<HoldOverrideRecord> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn get_judicial_holds_by_reference(&self, court_reference: String) -> BankingResult<Vec<AccountHoldModel>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn update_loan_pledge_holds(&self, loan_account_id: Uuid, collateral_account_ids: Vec<Uuid>, new_pledge_amount: Decimal, updated_by_person_id: Uuid) -> BankingResult<Vec<AccountHoldModel>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn get_compliance_holds_by_alert(&self, compliance_alert_id: Uuid) -> BankingResult<Vec<AccountHoldModel>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn get_hold_analytics(&self, from_date: NaiveDate, to_date: NaiveDate, hold_types: Option<Vec<String>>) -> BankingResult<HoldAnalyticsSummary> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn get_high_hold_ratio_accounts(&self, minimum_ratio: Decimal, exclude_hold_types: Option<Vec<String>>, limit: i32) -> BankingResult<Vec<HighHoldRatioAccount>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn generate_judicial_hold_report(&self, from_date: NaiveDate, to_date: NaiveDate) -> BankingResult<JudicialHoldReportData> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn get_hold_aging_report(&self, hold_types: Option<Vec<String>>, aging_buckets: Vec<i32>) -> BankingResult<Vec<HoldAgingBucket>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn validate_hold_amounts(&self, account_id: Uuid) -> BankingResult<Vec<HoldValidationError>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn find_orphaned_holds(&self, limit: Option<i32>) -> BankingResult<Vec<AccountHoldModel>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn cleanup_old_holds(&self, cutoff_date: NaiveDate, hold_statuses: Vec<String>) -> BankingResult<u32> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn create_balance_calculation(&self, calc: AccountBalanceCalculationModel) -> BankingResult<AccountBalanceCalculationModel> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn find_balance_calculation_by_id(&self, id: Uuid) -> BankingResult<Option<AccountBalanceCalculationModel>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn create_hold_summary(&self, summary: AccountHoldSummaryModel) -> BankingResult<AccountHoldSummaryModel> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn find_hold_summaries_by_calc_id(&self, calc_id: Uuid) -> BankingResult<Vec<AccountHoldSummaryModel>> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn create_hold_release_request(&self, request: AccountHoldReleaseRequestModel) -> BankingResult<AccountHoldReleaseRequestModel> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn create_place_hold_request(&self, request: PlaceHoldRequestModel) -> BankingResult<PlaceHoldRequestModel> {
        unimplemented!()
    }
}
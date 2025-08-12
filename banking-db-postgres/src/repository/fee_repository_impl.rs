use async_trait::async_trait;
use banking_api::{BankingResult, BankingError};
// Fee enums are used in model deserialization
use banking_db::models::{
    FeeApplicationModel, FeeWaiverModel, FeeProcessingJobModel, 
    ProductFeeScheduleModel, FeeCalculationCacheModel
};
use banking_db::repository::{
    FeeRepository, FeeRevenueSummary, TopFeeAccount, FeeStatistic
};
use sqlx::{PgPool, Row};
use uuid::Uuid;
// Decimal is used in reporting structs
use chrono::{DateTime, Utc, NaiveDate};
use heapless::String as HeaplessString;
use std::collections::HashMap;

pub struct FeeRepositoryImpl {
    pool: PgPool,
}

impl FeeRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

impl TryFromRow<sqlx::postgres::PgRow> for FeeApplicationModel {
    fn try_from_row(row: &sqlx::postgres::PgRow) -> BankingResult<Self> {
        Ok(FeeApplicationModel {
            id: row.get("id"),
            account_id: row.get("account_id"),
            transaction_id: row.get("transaction_id"),
            fee_type: row.get::<String, _>("fee_type").parse().map_err(|_| 
                BankingError::ValidationError {
                    field: "fee_type".to_string(),
                    message: "Invalid fee type".to_string(),
                }
            )?,
            fee_category: row.get::<String, _>("fee_category").parse().map_err(|_| 
                BankingError::ValidationError {
                    field: "fee_category".to_string(),
                    message: "Invalid fee category".to_string(),
                }
            )?,
            product_id: row.get("product_id"),
            fee_code: HeaplessString::try_from(
                row.get::<String, _>("fee_code").as_str()
            ).map_err(|_| BankingError::ValidationError {
                field: "fee_code".to_string(),
                message: "Fee code too long".to_string(),
            })?,
            description: HeaplessString::try_from(
                row.get::<String, _>("description").as_str()
            ).map_err(|_| BankingError::ValidationError {
                field: "description".to_string(),
                message: "Description too long".to_string(),
            })?,
            amount: row.get("amount"),
            currency: HeaplessString::try_from(
                row.get::<String, _>("currency").as_str()
            ).map_err(|_| BankingError::ValidationError {
                field: "currency".to_string(),
                message: "Currency code too long".to_string(),
            })?,
            calculation_method: row.get::<String, _>("calculation_method").parse().map_err(|_| 
                BankingError::ValidationError {
                    field: "calculation_method".to_string(),
                    message: "Invalid calculation method".to_string(),
                }
            )?,
            calculation_base_amount: row.get("calculation_base_amount"),
            fee_rate: row.get("fee_rate"),
            trigger_event: row.get::<String, _>("trigger_event").parse().map_err(|_| 
                BankingError::ValidationError {
                    field: "trigger_event".to_string(),
                    message: "Invalid trigger event".to_string(),
                }
            )?,
            status: row.get::<String, _>("status").parse().map_err(|_| 
                BankingError::ValidationError {
                    field: "status".to_string(),
                    message: "Invalid status".to_string(),
                }
            )?,
            applied_at: row.get("applied_at"),
            value_date: row.get("value_date"),
            reversal_deadline: row.get("reversal_deadline"),
            waived: row.get("waived"),
            waived_by: row.get("waived_by"),
            waived_reason_id: row.get("waived_reason_id"),
            applied_by: row.get("applied_by"),
            created_at: row.get("created_at"),
        })
    }
}

impl TryFromRow<sqlx::postgres::PgRow> for FeeWaiverModel {
    fn try_from_row(row: &sqlx::postgres::PgRow) -> BankingResult<Self> {
        Ok(FeeWaiverModel {
            id: row.get("id"),
            fee_application_id: row.get("fee_application_id"),
            account_id: row.get("account_id"),
            waived_amount: row.get("waived_amount"),
            reason_id: row.get("reason_id"),
            additional_details: row.get::<Option<String>, _>("additional_details")
                .and_then(|s| HeaplessString::try_from(s.as_str()).ok()),
            waived_by: row.get("waived_by"),
            waived_at: row.get("waived_at"),
            approval_required: row.get("approval_required"),
            approved_by: row.get("approved_by"),
            approved_at: row.get("approved_at"),
        })
    }
}

impl TryFromRow<sqlx::postgres::PgRow> for FeeProcessingJobModel {
    fn try_from_row(row: &sqlx::postgres::PgRow) -> BankingResult<Self> {
        Ok(FeeProcessingJobModel {
            id: row.get("id"),
            job_type: row.get::<String, _>("job_type").parse().map_err(|_| 
                BankingError::ValidationError {
                    field: "job_type".to_string(),
                    message: "Invalid job type".to_string(),
                }
            )?,
            job_name: row.get("job_name"),
            schedule_expression: HeaplessString::try_from(row.get::<String, _>("schedule_expression").as_str()).unwrap_or_default(),
            target_fee_categories_01: row.get::<String, _>("target_fee_categories_01").parse().map_err(|_| BankingError::ValidationError {
                field: "target_fee_categories_01".to_string(),
                message: "Invalid fee category".to_string(),
            })?,
            target_fee_categories_02: row.get::<String, _>("target_fee_categories_02").parse().map_err(|_| BankingError::ValidationError {
                field: "target_fee_categories_02".to_string(),
                message: "Invalid fee category".to_string(),
            })?,
            target_fee_categories_03: row.get::<String, _>("target_fee_categories_03").parse().map_err(|_| BankingError::ValidationError {
                field: "target_fee_categories_03".to_string(),
                message: "Invalid fee category".to_string(),
            })?,
            target_fee_categories_04: row.get::<String, _>("target_fee_categories_04").parse().map_err(|_| BankingError::ValidationError {
                field: "target_fee_categories_04".to_string(),
                message: "Invalid fee category".to_string(),
            })?,
            target_fee_categories_05: row.get::<String, _>("target_fee_categories_05").parse().map_err(|_| BankingError::ValidationError {
                field: "target_fee_categories_05".to_string(),
                message: "Invalid fee category".to_string(),
            })?,
            target_product_id_01: row.get("target_product_id_01"),
            target_product_id_02: row.get("target_product_id_02"),
            target_product_id_03: row.get("target_product_id_03"),
            target_product_id_04: row.get("target_product_id_04"),
            target_product_id_05: row.get("target_product_id_05"),
            processing_date: row.get("processing_date"),
            status: row.get::<String, _>("status").parse().map_err(|_|
                BankingError::ValidationError {
                    field: "status".to_string(),
                    message: "Invalid status".to_string(),
                }
            )?,
            started_at: row.get("started_at"),
            completed_at: row.get("completed_at"),
            accounts_processed: row.get("accounts_processed"),
            fees_applied: row.get("fees_applied"),
            total_amount: row.get("total_amount"),
            errors_01: HeaplessString::try_from(row.get::<String, _>("errors_01").as_str()).unwrap_or_default(),
            errors_02: HeaplessString::try_from(row.get::<String, _>("errors_02").as_str()).unwrap_or_default(),
            errors_03: HeaplessString::try_from(row.get::<String, _>("errors_03").as_str()).unwrap_or_default(),
            errors_04: HeaplessString::try_from(row.get::<String, _>("errors_04").as_str()).unwrap_or_default(),
            errors_05: HeaplessString::try_from(row.get::<String, _>("errors_05").as_str()).unwrap_or_default(),
            created_at: row.get("created_at"),
        })
    }
}

trait TryFromRow<R> {
    fn try_from_row(row: &R) -> BankingResult<Self>
    where
        Self: Sized;
}

#[async_trait]
impl FeeRepository for FeeRepositoryImpl {

    // ============================================================================
    // FEE APPLICATION CRUD OPERATIONS
    // ============================================================================
    
    async fn create_fee_application(
        &self,
        fee_application: FeeApplicationModel,
    ) -> BankingResult<FeeApplicationModel> {
        let result = sqlx::query(
            r#"
            INSERT INTO fee_applications (
                id, account_id, transaction_id, fee_type, fee_category,
                product_id, fee_code, description, amount, currency, calculation_method,
                calculation_base_amount, fee_rate, trigger_event, status, applied_at,
                value_date, reversal_deadline, waived, waived_by, waived_reason_id, applied_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
            RETURNING id, account_id, transaction_id, fee_type, fee_category,
                     product_id, fee_code, description, amount, currency, calculation_method,
                     calculation_base_amount, fee_rate, trigger_event, status, applied_at,
                     value_date, reversal_deadline, waived, waived_by, waived_reason_id, applied_by, created_at
            "#
        )
        .bind(fee_application.id)
        .bind(fee_application.account_id)
        .bind(fee_application.transaction_id)
        .bind(fee_application.fee_type.to_string())
        .bind(fee_application.fee_category.to_string())
        .bind(fee_application.product_id)
        .bind(fee_application.fee_code.as_str())
        .bind(fee_application.description.as_str())
        .bind(fee_application.amount)
        .bind(fee_application.currency.as_str())
        .bind(fee_application.calculation_method.to_string())
        .bind(fee_application.calculation_base_amount)
        .bind(fee_application.fee_rate)
        .bind(fee_application.trigger_event.to_string())
        .bind(fee_application.status.to_string())
        .bind(fee_application.applied_at)
        .bind(fee_application.value_date)
        .bind(fee_application.reversal_deadline)
        .bind(fee_application.waived)
        .bind(fee_application.waived_by)
        .bind(fee_application.waived_reason_id)
        .bind(fee_application.applied_by)
        .fetch_one(&self.pool)
        .await?;
        
        FeeApplicationModel::try_from_row(&result)
    }
    
    async fn update_fee_application(
        &self,
        fee_application: FeeApplicationModel,
    ) -> BankingResult<FeeApplicationModel> {
        let result = sqlx::query(
            r#"
            UPDATE fee_applications SET
                account_id = $2, transaction_id = $3, fee_type = $4, fee_category = $5,
                product_id = $6, fee_code = $7, description = $8, amount = $9, currency = $10,
                calculation_method = $11, calculation_base_amount = $12, fee_rate = $13,
                trigger_event = $14, status = $15, applied_at = $16, value_date = $17,
                reversal_deadline = $18, waived = $19, waived_by = $20, waived_reason_id = $21,
                applied_by = $22
            WHERE id = $1
            RETURNING id, account_id, transaction_id, fee_type, fee_category,
                     product_id, fee_code, description, amount, currency, calculation_method,
                     calculation_base_amount, fee_rate, trigger_event, status, applied_at,
                     value_date, reversal_deadline, waived, waived_by, waived_reason_id, applied_by, created_at
            "#
        )
        .bind(fee_application.id)
        .bind(fee_application.account_id)
        .bind(fee_application.transaction_id)
        .bind(fee_application.fee_type.to_string())
        .bind(fee_application.fee_category.to_string())
        .bind(fee_application.product_id)
        .bind(fee_application.fee_code.as_str())
        .bind(fee_application.description.as_str())
        .bind(fee_application.amount)
        .bind(fee_application.currency.as_str())
        .bind(fee_application.calculation_method.to_string())
        .bind(fee_application.calculation_base_amount)
        .bind(fee_application.fee_rate)
        .bind(fee_application.trigger_event.to_string())
        .bind(fee_application.status.to_string())
        .bind(fee_application.applied_at)
        .bind(fee_application.value_date)
        .bind(fee_application.reversal_deadline)
        .bind(fee_application.waived)
        .bind(fee_application.waived_by)
        .bind(fee_application.waived_reason_id)
        .bind(fee_application.applied_by)
        .fetch_one(&self.pool)
        .await?;
        
        FeeApplicationModel::try_from_row(&result)
    }
    
    async fn get_fee_application_by_id(
        &self,
        id: Uuid,
    ) -> BankingResult<Option<FeeApplicationModel>> {
        let result = sqlx::query(
            "SELECT * FROM fee_applications WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        match result {
            Some(row) => Ok(Some(FeeApplicationModel::try_from_row(&row)?)),
            None => Ok(None),
        }
    }
    
    async fn get_fee_applications_for_account(
        &self,
        account_id: Uuid,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
        status_filter: Option<String>,
    ) -> BankingResult<Vec<FeeApplicationModel>> {
        let mut query = String::from(
            "SELECT * FROM fee_applications WHERE account_id = $1"
        );
        let mut param_count = 1;
        
        if from_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND value_date >= ${param_count}"));
        }
        
        if to_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND value_date <= ${param_count}"));
        }
        
        if status_filter.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND status = ${param_count}"));
        }
        
        query.push_str(" ORDER BY applied_at DESC");
        
        let mut sql_query = sqlx::query(&query).bind(account_id);
        
        if let Some(from) = from_date {
            sql_query = sql_query.bind(from);
        }
        
        if let Some(to) = to_date {
            sql_query = sql_query.bind(to);
        }
        
        if let Some(status) = status_filter {
            sql_query = sql_query.bind(status);
        }
        
        let rows = sql_query
            .fetch_all(&self.pool)
            .await?;
        
        let mut applications = Vec::new();
        for row in rows {
            applications.push(FeeApplicationModel::try_from_row(&row)?);
        }
        
        Ok(applications)
    }
    
    async fn get_fee_applications_by_status(
        &self,
        status: String,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
        limit: Option<i32>,
    ) -> BankingResult<Vec<FeeApplicationModel>> {
        let mut query = String::from(
            "SELECT * FROM fee_applications WHERE status = $1"
        );
        let mut param_count = 1;
        
        if from_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND value_date >= ${param_count}"));
        }
        
        if to_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND value_date <= ${param_count}"));
        }
        
        query.push_str(" ORDER BY applied_at DESC");
        
        if limit.is_some() {
            param_count += 1;
            query.push_str(&format!(" LIMIT ${param_count}"));
        }
        
        let mut sql_query = sqlx::query(&query).bind(status);
        
        if let Some(from) = from_date {
            sql_query = sql_query.bind(from);
        }
        
        if let Some(to) = to_date {
            sql_query = sql_query.bind(to);
        }
        
        if let Some(lim) = limit {
            sql_query = sql_query.bind(lim);
        }
        
        let rows = sql_query
            .fetch_all(&self.pool)
            .await?;
        
        let mut applications = Vec::new();
        for row in rows {
            applications.push(FeeApplicationModel::try_from_row(&row)?);
        }
        
        Ok(applications)
    }
    
    async fn bulk_create_fee_applications(
        &self,
        fee_applications: Vec<FeeApplicationModel>,
    ) -> BankingResult<Vec<FeeApplicationModel>> {
        let mut tx = self.pool.begin().await?;
        
        let mut created_applications = Vec::new();
        
        for app in fee_applications {
            let result = sqlx::query(
                r#"
                INSERT INTO fee_applications (
                    id, account_id, transaction_id, fee_type, fee_category,
                    product_id, fee_code, description, amount, currency, calculation_method,
                    calculation_base_amount, fee_rate, trigger_event, status, applied_at,
                    value_date, reversal_deadline, waived, waived_by, waived_reason_id, applied_by
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
                RETURNING id, account_id, transaction_id, fee_type, fee_category,
                         product_id, fee_code, description, amount, currency, calculation_method,
                         calculation_base_amount, fee_rate, trigger_event, status, applied_at,
                         value_date, reversal_deadline, waived, waived_by, waived_reason_id, applied_by, created_at
                "#
            )
            .bind(app.id)
            .bind(app.account_id)
            .bind(app.transaction_id)
            .bind(app.fee_type.to_string())
            .bind(app.fee_category.to_string())
            .bind(app.product_id)
            .bind(app.fee_code.as_str())
            .bind(app.description.as_str())
            .bind(app.amount)
            .bind(app.currency.as_str())
            .bind(app.calculation_method.to_string())
            .bind(app.calculation_base_amount)
            .bind(app.fee_rate)
            .bind(app.trigger_event.to_string())
            .bind(app.status.to_string())
            .bind(app.applied_at)
            .bind(app.value_date)
            .bind(app.reversal_deadline)
            .bind(app.waived)
            .bind(app.waived_by)
            .bind(app.waived_reason_id)
            .bind(app.applied_by)
            .fetch_one(&mut *tx)
            .await?;
            
            created_applications.push(FeeApplicationModel::try_from_row(&result)?);
        }
        
        tx.commit().await?;
        
        Ok(created_applications)
    }

    // ============================================================================
    // FEE WAIVER OPERATIONS
    // ============================================================================
    
    async fn create_fee_waiver(
        &self,
        fee_waiver: FeeWaiverModel,
    ) -> BankingResult<FeeWaiverModel> {
        let result = sqlx::query(
            r#"
            INSERT INTO fee_waivers (
                id, fee_application_id, account_id, waived_amount, reason_id,
                additional_details, waived_by, waived_at, approval_required, approved_by, approved_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id, fee_application_id, account_id, waived_amount, reason_id,
                     additional_details, waived_by, waived_at, approval_required, approved_by, approved_at
            "#
        )
        .bind(fee_waiver.id)
        .bind(fee_waiver.fee_application_id)
        .bind(fee_waiver.account_id)
        .bind(fee_waiver.waived_amount)
        .bind(fee_waiver.reason_id)
        .bind(fee_waiver.additional_details.as_ref().map(|s| s.as_str()))
        .bind(fee_waiver.waived_by)
        .bind(fee_waiver.waived_at)
        .bind(fee_waiver.approval_required)
        .bind(fee_waiver.approved_by)
        .bind(fee_waiver.approved_at)
        .fetch_one(&self.pool)
        .await?;
        
        FeeWaiverModel::try_from_row(&result)
    }
    
    async fn update_fee_waiver_approval(
        &self,
        id: Uuid,
        approved_by: String,
        approved_at: DateTime<Utc>,
    ) -> BankingResult<FeeWaiverModel> {
        let approved_by_uuid = Uuid::parse_str(&approved_by).map_err(|_| BankingError::ValidationError {
            field: "approved_by".to_string(),
            message: "Invalid UUID for approved_by".to_string(),
        })?;
        
        let result = sqlx::query(
            r#"
            UPDATE fee_waivers SET
                approved_by = $2, approved_at = $3
            WHERE id = $1
            RETURNING id, fee_application_id, account_id, waived_amount, reason_id,
                     additional_details, waived_by, waived_at, approval_required, approved_by, approved_at
            "#
        )
        .bind(id)
        .bind(approved_by_uuid)
        .bind(approved_at)
        .fetch_one(&self.pool)
        .await?;
        
        FeeWaiverModel::try_from_row(&result)
    }
    
    async fn get_fee_waivers_for_account(
        &self,
        account_id: Uuid,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> BankingResult<Vec<FeeWaiverModel>> {
        let mut query = String::from(
            "SELECT * FROM fee_waivers WHERE account_id = $1"
        );
        let mut param_count = 1;
        
        if from_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND DATE(waived_at) >= ${param_count}"));
        }
        
        if to_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND DATE(waived_at) <= ${param_count}"));
        }
        
        query.push_str(" ORDER BY waived_at DESC");
        
        let mut sql_query = sqlx::query(&query).bind(account_id);
        
        if let Some(from) = from_date {
            sql_query = sql_query.bind(from);
        }
        
        if let Some(to) = to_date {
            sql_query = sql_query.bind(to);
        }
        
        let rows = sql_query
            .fetch_all(&self.pool)
            .await?;
        
        let mut waivers = Vec::new();
        for row in rows {
            waivers.push(FeeWaiverModel::try_from_row(&row)?);
        }
        
        Ok(waivers)
    }
    
    async fn get_pending_fee_waivers(
        &self,
        limit: Option<i32>,
    ) -> BankingResult<Vec<FeeWaiverModel>> {
        let mut query = String::from(
            "SELECT * FROM fee_waivers WHERE approval_required = TRUE AND approved_by IS NULL ORDER BY waived_at ASC"
        );
        
        if let Some(lim) = limit {
            query.push_str(&format!(" LIMIT {lim}"));
        }
        
        let rows = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await?;
        
        let mut waivers = Vec::new();
        for row in rows {
            waivers.push(FeeWaiverModel::try_from_row(&row)?);
        }
        
        Ok(waivers)
    }

    // ============================================================================
    // BATCH PROCESSING OPERATIONS
    // ============================================================================
    
    async fn create_fee_processing_job(
        &self,
        _job: FeeProcessingJobModel,
    ) -> BankingResult<FeeProcessingJobModel> {
        // Since there's no fee_processing_jobs table in the schema, we'll implement a stub
        Err(BankingError::NotImplemented("Fee processing jobs table not implemented in schema".to_string()))
    }
    
    async fn update_fee_processing_job(
        &self,
        _job: FeeProcessingJobModel,
    ) -> BankingResult<FeeProcessingJobModel> {
        Err(BankingError::NotImplemented("Fee processing jobs table not implemented in schema".to_string()))
    }
    
    async fn get_fee_processing_job_by_id(
        &self,
        _id: Uuid,
    ) -> BankingResult<Option<FeeProcessingJobModel>> {
        Err(BankingError::NotImplemented("Fee processing jobs table not implemented in schema".to_string()))
    }
    
    async fn get_fee_processing_jobs(
        &self,
        _status: Option<String>,
        _from_date: Option<NaiveDate>,
        _to_date: Option<NaiveDate>,
    ) -> BankingResult<Vec<FeeProcessingJobModel>> {
        Err(BankingError::NotImplemented("Fee processing jobs table not implemented in schema".to_string()))
    }
    
    async fn get_accounts_eligible_for_fees(
        &self,
        product_ids: Option<Vec<Uuid>>,
        _fee_categories: Vec<String>,
        _processing_date: NaiveDate,
        offset: i32,
        limit: i32,
    ) -> BankingResult<Vec<Uuid>> {
        let mut query = String::from(
            "SELECT id FROM accounts WHERE account_status = 'Active'"
        );
        let mut param_count = 0;
        
        if let Some(ref codes) = product_ids {
            if !codes.is_empty() {
                param_count += 1;
                query.push_str(&format!(" AND product_id = ANY(${param_count})"));
            }
        }
        
        param_count += 1;
        query.push_str(&format!(" ORDER BY id LIMIT ${param_count}"));
        param_count += 1;
        query.push_str(&format!(" OFFSET ${param_count}"));
        
        let mut sql_query = sqlx::query(&query);
        
        if let Some(codes) = product_ids {
            if !codes.is_empty() {
                sql_query = sql_query.bind(codes);
            }
        }
        
        sql_query = sql_query.bind(limit).bind(offset);
        
        let rows = sql_query
            .fetch_all(&self.pool)
            .await?;
        
        let mut account_ids = Vec::new();
        for row in rows {
            account_ids.push(row.get("id"));
        }
        
        Ok(account_ids)
    }

    // ============================================================================
    // PRODUCT CATALOG INTEGRATION
    // ============================================================================
    
    async fn cache_product_fee_schedule(
        &self,
        _schedule: ProductFeeScheduleModel,
    ) -> BankingResult<ProductFeeScheduleModel> {
        Err(BankingError::NotImplemented("Product fee schedule caching not implemented in schema".to_string()))
    }
    
    async fn get_cached_product_fee_schedule(
        &self,
        _product_id: Uuid,
        _effective_date: NaiveDate,
    ) -> BankingResult<Option<ProductFeeScheduleModel>> {
        Err(BankingError::NotImplemented("Product fee schedule caching not implemented in schema".to_string()))
    }
    
    async fn invalidate_fee_schedule_cache(
        &self,
        _product_id: Uuid,
    ) -> BankingResult<()> {
        Err(BankingError::NotImplemented("Product fee schedule caching not implemented in schema".to_string()))
    }

    // ============================================================================
    // FEE CALCULATION CACHE
    // ============================================================================
    
    async fn cache_fee_calculation(
        &self,
        _cache_entry: FeeCalculationCacheModel,
    ) -> BankingResult<FeeCalculationCacheModel> {
        Err(BankingError::NotImplemented("Fee calculation cache not implemented in schema".to_string()))
    }
    
    async fn get_cached_fee_calculation(
        &self,
        _calculation_key: String,
    ) -> BankingResult<Option<FeeCalculationCacheModel>> {
        Err(BankingError::NotImplemented("Fee calculation cache not implemented in schema".to_string()))
    }
    
    async fn clean_expired_fee_cache(
        &self,
        _cutoff_date: DateTime<Utc>,
    ) -> BankingResult<u32> {
        Err(BankingError::NotImplemented("Fee calculation cache not implemented in schema".to_string()))
    }

    // ============================================================================
    // REPORTING AND ANALYTICS
    // ============================================================================
    
    async fn get_fee_revenue_summary(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
        product_ids: Option<Vec<Uuid>>,
        fee_categories: Option<Vec<String>>,
    ) -> BankingResult<FeeRevenueSummary> {
        let mut query = String::from(
            r#"
            SELECT 
                COALESCE(SUM(CASE WHEN status = 'Applied' AND waived = FALSE THEN amount ELSE 0 END), 0) as total_revenue,
                COUNT(CASE WHEN status = 'Applied' THEN 1 END) as fee_count,
                COALESCE(SUM(CASE WHEN waived = TRUE THEN amount ELSE 0 END), 0) as waived_amount,
                COALESCE(SUM(CASE WHEN status = 'Reversed' THEN amount ELSE 0 END), 0) as reversed_amount
            FROM fee_applications 
            WHERE value_date >= $1 AND value_date <= $2
            "#
        );
        let mut param_count = 2;
        
        if let Some(codes) = &product_ids {
            if !codes.is_empty() {
                param_count += 1;
                query.push_str(&format!(" AND product_id = ANY(${param_count})"));
            }
        }
        
        if let Some(categories) = &fee_categories {
            if !categories.is_empty() {
                param_count += 1;
                query.push_str(&format!(" AND fee_category = ANY(${param_count})"));
            }
        }
        
        let mut sql_query = sqlx::query(&query)
            .bind(from_date)
            .bind(to_date);
        
        if let Some(codes) = product_ids {
            if !codes.is_empty() {
                sql_query = sql_query.bind(codes);
            }
        }
        
        if let Some(categories) = fee_categories {
            if !categories.is_empty() {
                sql_query = sql_query.bind(categories);
            }
        }
        
        let row = sql_query
            .fetch_one(&self.pool)
            .await?;
        
        Ok(FeeRevenueSummary {
            total_revenue: row.get("total_revenue"),
            fee_count: row.get::<i64, _>("fee_count") as u32,
            waived_amount: row.get("waived_amount"),
            reversed_amount: row.get("reversed_amount"),
            revenue_by_category: HashMap::new(), // TODO: Implement category breakdown
            revenue_by_product: HashMap::new(), // TODO: Implement product breakdown
        })
    }
    
    async fn get_top_fee_accounts(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
        limit: i32,
    ) -> BankingResult<Vec<TopFeeAccount>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                fa.account_id,
                COALESCE(SUM(CASE WHEN fa.status = 'Applied' AND fa.waived = FALSE THEN fa.amount ELSE 0 END), 0) as total_fees,
                COUNT(CASE WHEN fa.status = 'Applied' THEN 1 END) as fee_count,
                COALESCE(AVG(CASE WHEN fa.status = 'Applied' AND fa.waived = FALSE THEN fa.amount END), 0) as avg_fee_amount,
                a.product_id as product_id
            FROM fee_applications fa
            JOIN accounts a ON fa.account_id = a.id
            WHERE fa.value_date >= $1 AND fa.value_date <= $2
            GROUP BY fa.account_id, a.product_id
            ORDER BY total_fees DESC
            LIMIT $3
            "#
        )
        .bind(from_date)
        .bind(to_date)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        
        let mut top_accounts = Vec::new();
        for row in rows {
            top_accounts.push(TopFeeAccount {
                account_id: row.get("account_id"),
                total_fees: row.get("total_fees"),
                fee_count: row.get::<i64, _>("fee_count") as u32,
                avg_fee_amount: row.get("avg_fee_amount"),
                product_id: row.get("product_id"),
            });
        }
        
        Ok(top_accounts)
    }
    
    async fn get_fee_application_statistics(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
        group_by: String,
    ) -> BankingResult<Vec<FeeStatistic>> {
        let date_format = match group_by.as_str() {
            "day" => "YYYY-MM-DD",
            "week" => "YYYY-\"W\"IW",
            "month" => "YYYY-MM",
            _ => return Err(BankingError::ValidationError {
                field: "group_by".to_string(),
                message: "Invalid group_by value. Must be 'day', 'week', or 'month'".to_string(),
            }),
        };
        
        let rows = sqlx::query(&format!(
            r#"
            SELECT 
                TO_CHAR(value_date, '{date_format}') as period,
                fee_category,
                COUNT(*) as application_count,
                COALESCE(SUM(CASE WHEN status = 'Applied' AND waived = FALSE THEN amount ELSE 0 END), 0) as total_amount,
                COUNT(CASE WHEN waived = TRUE THEN 1 END) as waived_count,
                COALESCE(SUM(CASE WHEN waived = TRUE THEN amount ELSE 0 END), 0) as waived_amount
            FROM fee_applications
            WHERE value_date >= $1 AND value_date <= $2
            GROUP BY TO_CHAR(value_date, '{date_format}'), fee_category
            ORDER BY period, fee_category
            "#
        ))
        .bind(from_date)
        .bind(to_date)
        .fetch_all(&self.pool)
        .await?;
        
        let mut statistics = Vec::new();
        for row in rows {
            statistics.push(FeeStatistic {
                period: row.get("period"),
                fee_category: row.get("fee_category"),
                application_count: row.get::<i64, _>("application_count") as u32,
                total_amount: row.get("total_amount"),
                waived_count: row.get::<i64, _>("waived_count") as u32,
                waived_amount: row.get("waived_amount"),
            });
        }
        
        Ok(statistics)
    }

    // ============================================================================
    // REVERSAL AND CORRECTION OPERATIONS
    // ============================================================================
    
    async fn reverse_fee_application(
        &self,
        id: Uuid,
        reversal_reason: String,
        reversed_by: String,
        _reversed_at: DateTime<Utc>,
    ) -> BankingResult<FeeApplicationModel> {
        let reversed_by_uuid = Uuid::parse_str(&reversed_by).map_err(|_| BankingError::ValidationError {
            field: "reversed_by".to_string(),
            message: "Invalid UUID for reversed_by".to_string(),
        })?;
        
        let result = sqlx::query(
            r#"
            UPDATE fee_applications SET
                status = 'Reversed',
                waived_reason_id = (
                    SELECT id FROM reason_and_purpose 
                    WHERE category = 'TransactionReversal' 
                    LIMIT 1
                ),
                applied_by = $3
            WHERE id = $1 AND status != 'Reversed'
            RETURNING id, account_id, transaction_id, fee_type, fee_category,
                     product_id, fee_code, description, amount, currency, calculation_method,
                     calculation_base_amount, fee_rate, trigger_event, status, applied_at,
                     value_date, reversal_deadline, waived, waived_by, waived_reason_id, applied_by, created_at
            "#
        )
        .bind(id)
        .bind(reversal_reason)
        .bind(reversed_by_uuid)
        .fetch_one(&self.pool)
        .await?;
        
        FeeApplicationModel::try_from_row(&result)
    }
    
    async fn bulk_reverse_account_fees(
        &self,
        account_id: Uuid,
        ids: Vec<Uuid>,
        reversal_reason: String,
        reversed_by: String,
    ) -> BankingResult<Vec<FeeApplicationModel>> {
        let reversed_by_uuid = Uuid::parse_str(&reversed_by).map_err(|_| BankingError::ValidationError {
            field: "reversed_by".to_string(),
            message: "Invalid UUID for reversed_by".to_string(),
        })?;
        
        let mut tx = self.pool.begin().await?;
        
        let mut reversed_applications = Vec::new();
        
        for fee_id in ids {
            let result = sqlx::query(
                r#"
                UPDATE fee_applications SET
                    status = 'Reversed',
                    waived_reason_id = (
                        SELECT id FROM reason_and_purpose 
                        WHERE category = 'TransactionReversal' 
                        LIMIT 1
                    ),
                    applied_by = $4
                WHERE id = $1 AND account_id = $2 AND status != 'Reversed'
                RETURNING id, account_id, transaction_id, fee_type, fee_category,
                         product_id, fee_code, description, amount, currency, calculation_method,
                         calculation_base_amount, fee_rate, trigger_event, status, applied_at,
                         value_date, reversal_deadline, waived, waived_by, waived_reason_id, applied_by, created_at
                "#
            )
            .bind(fee_id)
            .bind(account_id)
            .bind(reversal_reason.clone())
            .bind(reversed_by_uuid)
            .fetch_optional(&mut *tx)
            .await?;
            
            if let Some(row) = result {
                reversed_applications.push(FeeApplicationModel::try_from_row(&row)?);
            }
        }
        
        tx.commit().await?;
        
        Ok(reversed_applications)
    }
}
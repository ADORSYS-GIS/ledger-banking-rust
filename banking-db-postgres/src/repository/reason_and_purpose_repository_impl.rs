use async_trait::async_trait;
use banking_api::{BankingResult, BankingError};
use banking_api::domain::{ReasonCategory, ReasonContext, ReasonSeverity};
use banking_db::models::ReasonAndPurpose as ReasonAndPurposeModel;
use banking_db::repository::{
    ReasonAndPurposeRepository, ReasonUsageStatistics, ReasonChangeRecord, 
    ReasonValidationRules, BulkOperationResult, 
    LocalizedReasonModel, DataIntegrityReport
};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{Utc, NaiveDate};
use heapless::String as HeaplessString;

pub struct ReasonAndPurposeRepositoryImpl {
    pool: PgPool,
}

impl ReasonAndPurposeRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

trait TryFromRow<R> {
    fn try_from_row(row: &R) -> BankingResult<Self>
    where
        Self: Sized;
}

impl TryFromRow<sqlx::postgres::PgRow> for ReasonAndPurposeModel {
    fn try_from_row(row: &sqlx::postgres::PgRow) -> BankingResult<Self> {
        Ok(ReasonAndPurposeModel {
            id: row.get("id"),
            code: HeaplessString::try_from(
                row.get::<String, _>("code").as_str()
            ).map_err(|_| BankingError::ValidationError {
                field: "code".to_string(),
                message: "Code too long".to_string(),
            })?,
            category: row.get::<String, _>("category").parse().map_err(|_| 
                BankingError::ValidationError {
                    field: "category".to_string(),
                    message: "Invalid reason category".to_string(),
                }
            )?,
            context: row.get::<String, _>("context").parse().map_err(|_| 
                BankingError::ValidationError {
                    field: "context".to_string(),
                    message: "Invalid reason context".to_string(),
                }
            )?,
            l1_content: row.get::<Option<String>, _>("l1_content")
                .map(|s| HeaplessString::try_from(s.as_str()))
                .transpose()
                .map_err(|_| BankingError::ValidationError {
                    field: "l1_content".to_string(),
                    message: "L1 content too long".to_string(),
                })?,
            l2_content: row.get::<Option<String>, _>("l2_content")
                .map(|s| HeaplessString::try_from(s.as_str()))
                .transpose()
                .map_err(|_| BankingError::ValidationError {
                    field: "l2_content".to_string(),
                    message: "L2 content too long".to_string(),
                })?,
            l3_content: row.get::<Option<String>, _>("l3_content")
                .map(|s| HeaplessString::try_from(s.as_str()))
                .transpose()
                .map_err(|_| BankingError::ValidationError {
                    field: "l3_content".to_string(),
                    message: "L3 content too long".to_string(),
                })?,
            l1_language_code: row.get::<Option<String>, _>("l1_language_code")
                .map(|s| {
                    let bytes = s.as_bytes();
                    if bytes.len() == 3 {
                        Ok([bytes[0], bytes[1], bytes[2]])
                    } else {
                        Err(BankingError::ValidationError {
                            field: "l1_language_code".to_string(),
                            message: "Language code must be 3 characters".to_string(),
                        })
                    }
                })
                .transpose()?,
            l2_language_code: row.get::<Option<String>, _>("l2_language_code")
                .map(|s| {
                    let bytes = s.as_bytes();
                    if bytes.len() == 3 {
                        Ok([bytes[0], bytes[1], bytes[2]])
                    } else {
                        Err(BankingError::ValidationError {
                            field: "l2_language_code".to_string(),
                            message: "Language code must be 3 characters".to_string(),
                        })
                    }
                })
                .transpose()?,
            l3_language_code: row.get::<Option<String>, _>("l3_language_code")
                .map(|s| {
                    let bytes = s.as_bytes();
                    if bytes.len() == 3 {
                        Ok([bytes[0], bytes[1], bytes[2]])
                    } else {
                        Err(BankingError::ValidationError {
                            field: "l3_language_code".to_string(),
                            message: "Language code must be 3 characters".to_string(),
                        })
                    }
                })
                .transpose()?,
            requires_details: row.get("requires_details"),
            is_active: row.get("is_active"),
            severity: row.get::<Option<String>, _>("severity")
                .map(|s| s.parse())
                .transpose()
                .map_err(|_| BankingError::ValidationError {
                    field: "severity".to_string(),
                    message: "Invalid reason severity".to_string(),
                })?,
            display_order: row.get("display_order"),
            compliance_metadata: None, // TODO: Implement JSON parsing for compliance metadata
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            created_by_person_id: HeaplessString::try_from(
                row.get::<String, _>("created_by_person_id").as_str()
            ).map_err(|_| BankingError::ValidationError {
                field: "created_by_person_id".to_string(),
                message: "Created by too long".to_string(),
            })?,
            updated_by_person_id: HeaplessString::try_from(
                row.get::<String, _>("updated_by_person_id").as_str()
            ).map_err(|_| BankingError::ValidationError {
                field: "updated_by_person_id".to_string(),
                message: "Updated by too long".to_string(),
            })?,
        })
    }
}

#[async_trait]
impl ReasonAndPurposeRepository for ReasonAndPurposeRepositoryImpl {
    // ============================================================================
    // CRUD OPERATIONS
    // ============================================================================
    
    async fn create(&self, reason: ReasonAndPurposeModel) -> BankingResult<ReasonAndPurposeModel> {
        let row = sqlx::query(
            "INSERT INTO reason_and_purpose 
                (id, code, category, context, l1_content, l2_content, l3_content,
                 l1_language_code, l2_language_code, l3_language_code, requires_details,
                 is_active, severity, display_order, created_at, updated_at, created_by_person_id, updated_by_person_id)
             VALUES ($1, $2, $3::reason_category, $4::reason_context, $5, $6, $7, $8, $9, $10, 
                     $11, $12, $13::reason_severity, $14, $15, $16, $17, $18)
             RETURNING id, code, category::text, context::text, l1_content, l2_content, l3_content,
                      l1_language_code, l2_language_code, l3_language_code, requires_details,
                      is_active, severity::text, display_order, created_at, updated_at, created_by_person_id, updated_by_person_id"
        )
        .bind(reason.id)
        .bind(reason.code.as_str())
        .bind(reason.category.to_string())
        .bind(reason.context.to_string())
        .bind(reason.l1_content.as_ref().map(|s| s.as_str()))
        .bind(reason.l2_content.as_ref().map(|s| s.as_str()))
        .bind(reason.l3_content.as_ref().map(|s| s.as_str()))
        .bind(reason.l1_language_code.as_ref().map(|c| String::from_utf8_lossy(c).to_string()))
        .bind(reason.l2_language_code.as_ref().map(|c| String::from_utf8_lossy(c).to_string()))
        .bind(reason.l3_language_code.as_ref().map(|c| String::from_utf8_lossy(c).to_string()))
        .bind(reason.requires_details)
        .bind(reason.is_active)
        .bind(reason.severity.as_ref().map(|s| s.to_string()))
        .bind(reason.display_order)
        .bind(reason.created_at)
        .bind(reason.updated_at)
        .bind(reason.created_by_person_id.as_str())
        .bind(reason.updated_by_person_id.as_str())
        .fetch_one(&self.pool)
        .await
        .map_err(BankingError::from)?;

        ReasonAndPurposeModel::try_from_row(&row)
    }
    
    async fn find_by_id(&self, reason_id: Uuid) -> BankingResult<Option<ReasonAndPurposeModel>> {
        let row = sqlx::query(
            "SELECT id, code, category::text, context::text, l1_content, l2_content, l3_content,
                    l1_language_code, l2_language_code, l3_language_code, requires_details,
                    is_active, severity::text, display_order, created_at, updated_at, created_by_person_id, updated_by_person_id
             FROM reason_and_purpose WHERE id = $1"
        )
        .bind(reason_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(BankingError::from)?;

        match row {
            Some(row) => Ok(Some(ReasonAndPurposeModel::try_from_row(&row)?)),
            None => Ok(None),
        }
    }
    
    async fn find_by_code(&self, code: &str) -> BankingResult<Option<ReasonAndPurposeModel>> {
        let row = sqlx::query(
            "SELECT id, code, category::text, context::text, l1_content, l2_content, l3_content,
                    l1_language_code, l2_language_code, l3_language_code, requires_details,
                    is_active, severity::text, display_order, created_at, updated_at, created_by_person_id, updated_by_person_id
             FROM reason_and_purpose WHERE code = $1"
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await
        .map_err(BankingError::from)?;

        match row {
            Some(row) => Ok(Some(ReasonAndPurposeModel::try_from_row(&row)?)),
            None => Ok(None),
        }
    }
    
    async fn update(&self, reason: ReasonAndPurposeModel) -> BankingResult<ReasonAndPurposeModel> {
        let row = sqlx::query(
            "UPDATE reason_and_purpose SET
                code = $2, category = $3::reason_category, context = $4::reason_context,
                l1_content = $5, l2_content = $6, l3_content = $7,
                l1_language_code = $8, l2_language_code = $9, l3_language_code = $10,
                requires_details = $11, is_active = $12, severity = $13::reason_severity,
                display_order = $14, updated_at = $15, updated_by_person_id = $16
             WHERE id = $1
             RETURNING id, code, category::text, context::text, l1_content, l2_content, l3_content,
                      l1_language_code, l2_language_code, l3_language_code, requires_details,
                      is_active, severity::text, display_order, created_at, updated_at, created_by_person_id, updated_by_person_id"
        )
        .bind(reason.id)
        .bind(reason.code.as_str())
        .bind(reason.category.to_string())
        .bind(reason.context.to_string())
        .bind(reason.l1_content.as_ref().map(|s| s.as_str()))
        .bind(reason.l2_content.as_ref().map(|s| s.as_str()))
        .bind(reason.l3_content.as_ref().map(|s| s.as_str()))
        .bind(reason.l1_language_code.as_ref().map(|c| String::from_utf8_lossy(c).to_string()))
        .bind(reason.l2_language_code.as_ref().map(|c| String::from_utf8_lossy(c).to_string()))
        .bind(reason.l3_language_code.as_ref().map(|c| String::from_utf8_lossy(c).to_string()))
        .bind(reason.requires_details)
        .bind(reason.is_active)
        .bind(reason.severity.as_ref().map(|s| s.to_string()))
        .bind(reason.display_order)
        .bind(reason.updated_at)
        .bind(reason.updated_by_person_id.as_str())
        .fetch_one(&self.pool)
        .await
        .map_err(BankingError::from)?;

        ReasonAndPurposeModel::try_from_row(&row)
    }
    
    async fn delete(&self, reason_id: Uuid) -> BankingResult<()> {
        sqlx::query("DELETE FROM reason_and_purpose WHERE id = $1")
            .bind(reason_id)
            .execute(&self.pool)
            .await
            .map_err(BankingError::from)?;
        
        Ok(())
    }
    
    async fn deactivate(&self, reason_id: Uuid, deactivated_by: &str) -> BankingResult<()> {
        sqlx::query(
            "UPDATE reason_and_purpose SET is_active = false, updated_at = NOW(), updated_by_person_id = $2 WHERE id = $1"
        )
        .bind(reason_id)
        .bind(deactivated_by)
        .execute(&self.pool)
        .await
        .map_err(BankingError::from)?;
        
        Ok(())
    }
    
    async fn reactivate(&self, reason_id: Uuid, reactivated_by: &str) -> BankingResult<()> {
        sqlx::query(
            "UPDATE reason_and_purpose SET is_active = true, updated_at = NOW(), updated_by_person_id = $2 WHERE id = $1"
        )
        .bind(reason_id)
        .bind(reactivated_by)
        .execute(&self.pool)
        .await
        .map_err(BankingError::from)?;
        
        Ok(())
    }
    
    // ============================================================================
    // QUERY OPERATIONS
    // ============================================================================
    
    async fn find_all_active(&self) -> BankingResult<Vec<ReasonAndPurposeModel>> {
        let rows = sqlx::query(
            "SELECT id, code, category::text, context::text, l1_content, l2_content, l3_content,
                    l1_language_code, l2_language_code, l3_language_code, requires_details,
                    is_active, severity::text, display_order, created_at, updated_at, created_by_person_id, updated_by_person_id
             FROM reason_and_purpose WHERE is_active = true ORDER BY display_order, code"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(BankingError::from)?;

        let mut reasons = Vec::new();
        for row in rows {
            reasons.push(ReasonAndPurposeModel::try_from_row(&row)?);
        }
        
        Ok(reasons)
    }
    
    async fn find_by_category(&self, category: ReasonCategory) -> BankingResult<Vec<ReasonAndPurposeModel>> {
        let rows = sqlx::query(
            "SELECT id, code, category::text, context::text, l1_content, l2_content, l3_content,
                    l1_language_code, l2_language_code, l3_language_code, requires_details,
                    is_active, severity::text, display_order, created_at, updated_at, created_by_person_id, updated_by_person_id
             FROM reason_and_purpose WHERE category = $1::reason_category ORDER BY display_order, code"
        )
        .bind(category.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(BankingError::from)?;

        let mut reasons = Vec::new();
        for row in rows {
            reasons.push(ReasonAndPurposeModel::try_from_row(&row)?);
        }
        
        Ok(reasons)
    }
    
    async fn find_by_context(&self, context: ReasonContext) -> BankingResult<Vec<ReasonAndPurposeModel>> {
        let rows = sqlx::query(
            "SELECT id, code, category::text, context::text, l1_content, l2_content, l3_content,
                    l1_language_code, l2_language_code, l3_language_code, requires_details,
                    is_active, severity::text, display_order, created_at, updated_at, created_by_person_id, updated_by_person_id
             FROM reason_and_purpose WHERE context = $1::reason_context ORDER BY display_order, code"
        )
        .bind(context.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(BankingError::from)?;

        let mut reasons = Vec::new();
        for row in rows {
            reasons.push(ReasonAndPurposeModel::try_from_row(&row)?);
        }
        
        Ok(reasons)
    }
    
    async fn find_by_category_and_context(&self, category: ReasonCategory, context: ReasonContext) -> BankingResult<Vec<ReasonAndPurposeModel>> {
        let rows = sqlx::query(
            "SELECT id, code, category::text, context::text, l1_content, l2_content, l3_content,
                    l1_language_code, l2_language_code, l3_language_code, requires_details,
                    is_active, severity::text, display_order, created_at, updated_at, created_by_person_id, updated_by_person_id
             FROM reason_and_purpose 
             WHERE category = $1::reason_category AND context = $2::reason_context 
             ORDER BY display_order, code"
        )
        .bind(category.to_string())
        .bind(context.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(BankingError::from)?;

        let mut reasons = Vec::new();
        for row in rows {
            reasons.push(ReasonAndPurposeModel::try_from_row(&row)?);
        }
        
        Ok(reasons)
    }
    
    async fn find_by_severity(&self, severity: ReasonSeverity) -> BankingResult<Vec<ReasonAndPurposeModel>> {
        let rows = sqlx::query(
            "SELECT id, code, category::text, context::text, l1_content, l2_content, l3_content,
                    l1_language_code, l2_language_code, l3_language_code, requires_details,
                    is_active, severity::text, display_order, created_at, updated_at, created_by_person_id, updated_by_person_id
             FROM reason_and_purpose WHERE severity = $1::reason_severity ORDER BY display_order, code"
        )
        .bind(severity.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(BankingError::from)?;

        let mut reasons = Vec::new();
        for row in rows {
            reasons.push(ReasonAndPurposeModel::try_from_row(&row)?);
        }
        
        Ok(reasons)
    }
    
    async fn search_by_content(&self, search_term: &str, _language_codes: Option<Vec<[u8; 3]>>) -> BankingResult<Vec<ReasonAndPurposeModel>> {
        let search_pattern = format!("%{search_term}%", );
        
        let rows = sqlx::query(
            "SELECT id, code, category::text, context::text, l1_content, l2_content, l3_content,
                    l1_language_code, l2_language_code, l3_language_code, requires_details,
                    is_active, severity::text, display_order, created_at, updated_at, created_by_person_id, updated_by_person_id
             FROM reason_and_purpose 
             WHERE l1_content ILIKE $1 OR l2_content ILIKE $1 OR l3_content ILIKE $1 OR code ILIKE $1
             ORDER BY display_order, code"
        )
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await
        .map_err(BankingError::from)?;

        let mut reasons = Vec::new();
        for row in rows {
            reasons.push(ReasonAndPurposeModel::try_from_row(&row)?);
        }
        
        Ok(reasons)
    }
    
    async fn find_for_display(&self, category: Option<ReasonCategory>, context: Option<ReasonContext>, active_only: bool) -> BankingResult<Vec<ReasonAndPurposeModel>> {
        let mut query = "SELECT id, code, category::text, context::text, l1_content, l2_content, l3_content,
                                l1_language_code, l2_language_code, l3_language_code, requires_details,
                                is_active, severity::text, display_order, created_at, updated_at, created_by_person_id, updated_by_person_id
                         FROM reason_and_purpose WHERE 1=1".to_string();
        
        let mut param_count = 0;
        
        if category.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND category = ${param_count}::reason_category"));
        }
        
        if context.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND context = ${param_count}::reason_context"));
        }
        
        if active_only {
            query.push_str(" AND is_active = true");
        }
        
        query.push_str(" ORDER BY display_order, code");
        
        let mut sql_query = sqlx::query(&query);
        
        if let Some(cat) = category {
            sql_query = sql_query.bind(cat.to_string());
        }
        
        if let Some(ctx) = context {
            sql_query = sql_query.bind(ctx.to_string());
        }
        
        let rows = sql_query.fetch_all(&self.pool).await.map_err(BankingError::from)?;

        let mut reasons = Vec::new();
        for row in rows {
            reasons.push(ReasonAndPurposeModel::try_from_row(&row)?);
        }
        
        Ok(reasons)
    }
    
    // ============================================================================
    // COMPLIANCE-SPECIFIC QUERIES - Placeholder implementations
    // ============================================================================
    
    async fn find_reportable_compliance_reasons(&self) -> BankingResult<Vec<ReasonAndPurposeModel>> {
        let rows = sqlx::query(
            "SELECT id, code, category::text, context::text, l1_content, l2_content, l3_content,
                    l1_language_code, l2_language_code, l3_language_code, requires_details,
                    is_active, severity::text, display_order, created_at, updated_at, created_by_person_id, updated_by_person_id
             FROM reason_and_purpose WHERE reportable = true ORDER BY display_order, code"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(BankingError::from)?;

        let mut reasons = Vec::new();
        for row in rows {
            reasons.push(ReasonAndPurposeModel::try_from_row(&row)?);
        }
        
        Ok(reasons)
    }
    
    async fn find_sar_triggering_reasons(&self) -> BankingResult<Vec<ReasonAndPurposeModel>> {
        let rows = sqlx::query(
            "SELECT id, code, category::text, context::text, l1_content, l2_content, l3_content,
                    l1_language_code, l2_language_code, l3_language_code, requires_details,
                    is_active, severity::text, display_order, created_at, updated_at, created_by_person_id, updated_by_person_id
             FROM reason_and_purpose WHERE requires_sar = true ORDER BY display_order, code"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(BankingError::from)?;

        let mut reasons = Vec::new();
        for row in rows {
            reasons.push(ReasonAndPurposeModel::try_from_row(&row)?);
        }
        
        Ok(reasons)
    }
    
    async fn find_ctr_triggering_reasons(&self) -> BankingResult<Vec<ReasonAndPurposeModel>> {
        let rows = sqlx::query(
            "SELECT id, code, category::text, context::text, l1_content, l2_content, l3_content,
                    l1_language_code, l2_language_code, l3_language_code, requires_details,
                    is_active, severity::text, display_order, created_at, updated_at, created_by_person_id, updated_by_person_id
             FROM reason_and_purpose WHERE requires_ctr = true ORDER BY display_order, code"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(BankingError::from)?;

        let mut reasons = Vec::new();
        for row in rows {
            reasons.push(ReasonAndPurposeModel::try_from_row(&row)?);
        }
        
        Ok(reasons)
    }
    
    // Placeholder implementations for remaining methods
    async fn find_aml_ctf_reasons(&self) -> BankingResult<Vec<ReasonAndPurposeModel>> {
        self.find_by_context(ReasonContext::AmlCtf).await
    }
    
    async fn find_kyc_reasons(&self) -> BankingResult<Vec<ReasonAndPurposeModel>> {
        self.find_by_context(ReasonContext::Kyc).await
    }
    
    async fn find_by_jurisdiction(&self, _jurisdiction_code: [u8; 2]) -> BankingResult<Vec<ReasonAndPurposeModel>> {
        // TODO: Implement jurisdiction filtering using compliance metadata
        Ok(vec![])
    }
    
    async fn find_escalation_required_reasons(&self) -> BankingResult<Vec<ReasonAndPurposeModel>> {
        let rows = sqlx::query(
            "SELECT id, code, category::text, context::text, l1_content, l2_content, l3_content,
                    l1_language_code, l2_language_code, l3_language_code, requires_details,
                    is_active, severity::text, display_order, created_at, updated_at, created_by_person_id, updated_by_person_id
             FROM reason_and_purpose WHERE escalation_required = true ORDER BY display_order, code"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(BankingError::from)?;

        let mut reasons = Vec::new();
        for row in rows {
            reasons.push(ReasonAndPurposeModel::try_from_row(&row)?);
        }
        
        Ok(reasons)
    }
    
    // Placeholder implementations for analytics and other methods
    async fn get_usage_count(&self, _reason_id: Uuid, _from_date: NaiveDate, _to_date: NaiveDate) -> BankingResult<u64> {
        Ok(0) // TODO: Implement usage tracking
    }
    
    async fn get_usage_statistics(&self, _reason_id: Uuid, _from_date: NaiveDate, _to_date: NaiveDate) -> BankingResult<ReasonUsageStatistics> {
        Err(BankingError::NotImplemented("Usage statistics not yet implemented".to_string()))
    }
    
    async fn get_top_used_reasons_by_category(&self, _category: ReasonCategory, _limit: i32, _from_date: NaiveDate, _to_date: NaiveDate) -> BankingResult<Vec<ReasonUsageStatistics>> {
        Ok(vec![])
    }
    
    async fn find_unused_reasons(&self, _since_date: NaiveDate) -> BankingResult<Vec<ReasonAndPurposeModel>> {
        Ok(vec![])
    }
    
    async fn record_usage(&self, _reason_id: Uuid, _context: ReasonContext, _used_by: &str, _additional_context: Option<&str>) -> BankingResult<()> {
        Ok(()) // TODO: Implement usage recording
    }
    
    async fn get_change_history(&self, _reason_id: Uuid) -> BankingResult<Vec<ReasonChangeRecord>> {
        Ok(vec![])
    }
    
    async fn record_change(&self, _change_record: ReasonChangeRecord) -> BankingResult<ReasonChangeRecord> {
        Err(BankingError::NotImplemented("Change recording not yet implemented".to_string()))
    }
    
    async fn code_exists(&self, code: &str, exclude_id: Option<Uuid>) -> BankingResult<bool> {
        let mut query = "SELECT COUNT(*) as count FROM reason_and_purpose WHERE code = $1".to_string();
        let mut param_count = 1;
        
        if exclude_id.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND id != ${param_count}"));
        }
        
        let mut sql_query = sqlx::query(&query).bind(code);
        
        if let Some(id) = exclude_id {
            sql_query = sql_query.bind(id);
        }
        
        let row = sql_query.fetch_one(&self.pool).await.map_err(BankingError::from)?;
        let count: i64 = row.get("count");
        
        Ok(count > 0)
    }
    
    async fn is_active(&self, reason_id: Uuid) -> BankingResult<bool> {
        let row = sqlx::query("SELECT is_active FROM reason_and_purpose WHERE id = $1")
            .bind(reason_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(BankingError::from)?;
        
        match row {
            Some(row) => Ok(row.get("is_active")),
            None => Ok(false),
        }
    }
    
    async fn is_valid_for_context(&self, reason_id: Uuid, context: ReasonContext) -> BankingResult<bool> {
        let row = sqlx::query("SELECT context::text FROM reason_and_purpose WHERE id = $1 AND is_active = true")
            .bind(reason_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(BankingError::from)?;
        
        match row {
            Some(row) => {
                let reason_context: String = row.get("context");
                let parsed_context: ReasonContext = reason_context.parse()
                    .map_err(|_| BankingError::ValidationError {
                        field: "context".to_string(),
                        message: "Invalid context".to_string(),
                    })?;
                Ok(parsed_context == context)
            }
            None => Ok(false),
        }
    }
    
    async fn get_validation_rules(&self, _reason_id: Uuid) -> BankingResult<Option<ReasonValidationRules>> {
        Ok(None) // TODO: Implement validation rules
    }
    
    async fn bulk_insert(&self, _reasons: Vec<ReasonAndPurposeModel>) -> BankingResult<BulkOperationResult> {
        Err(BankingError::NotImplemented("Bulk insert not yet implemented".to_string()))
    }
    
    async fn bulk_update_display_orders(&self, _category: ReasonCategory, _order_updates: Vec<(Uuid, i32)>, _updated_by_person_id: &str) -> BankingResult<()> {
        Ok(()) // TODO: Implement bulk display order updates
    }
    
    async fn bulk_update_status(&self, _reason_ids: Vec<Uuid>, _is_active: bool, _updated_by_person_id: &str) -> BankingResult<BulkOperationResult> {
        Err(BankingError::NotImplemented("Bulk status update not yet implemented".to_string()))
    }
    
    async fn update_localized_content(&self, _reason_id: Uuid, _language_code: [u8; 3], _content: &str, _updated_by_person_id: &str) -> BankingResult<()> {
        Ok(()) // TODO: Implement localized content updates
    }
    
    async fn remove_localized_content(&self, _reason_id: Uuid, _language_code: [u8; 3], _updated_by_person_id: &str) -> BankingResult<()> {
        Ok(()) // TODO: Implement localized content removal
    }
    
    async fn find_with_languages(&self, _language_codes: &[[u8; 3]], _category: Option<ReasonCategory>, _context: Option<ReasonContext>) -> BankingResult<Vec<LocalizedReasonModel>> {
        Ok(vec![]) // TODO: Implement localized reason queries
    }
    
    async fn find_missing_localization(&self, _language_code: [u8; 3], _category: Option<ReasonCategory>) -> BankingResult<Vec<ReasonAndPurposeModel>> {
        Ok(vec![]) // TODO: Implement missing localization detection
    }
    
    async fn count_total(&self) -> BankingResult<i64> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM reason_and_purpose")
            .fetch_one(&self.pool)
            .await
            .map_err(BankingError::from)?;
        
        Ok(row.get("count"))
    }
    
    async fn count_by_category(&self, category: ReasonCategory) -> BankingResult<i64> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM reason_and_purpose WHERE category = $1::reason_category")
            .bind(category.to_string())
            .fetch_one(&self.pool)
            .await
            .map_err(BankingError::from)?;
        
        Ok(row.get("count"))
    }
    
    async fn count_by_context(&self, context: ReasonContext) -> BankingResult<i64> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM reason_and_purpose WHERE context = $1::reason_context")
            .bind(context.to_string())
            .fetch_one(&self.pool)
            .await
            .map_err(BankingError::from)?;
        
        Ok(row.get("count"))
    }
    
    async fn validate_data_integrity(&self) -> BankingResult<DataIntegrityReport> {
        let total_reasons = self.count_total().await?;
        let active_reasons = sqlx::query("SELECT COUNT(*) as count FROM reason_and_purpose WHERE is_active = true")
            .fetch_one(&self.pool)
            .await
            .map_err(BankingError::from)?
            .get::<i64, _>("count");
        
        Ok(DataIntegrityReport {
            total_reasons,
            active_reasons,
            inactive_reasons: total_reasons - active_reasons,
            orphaned_reasons: 0, // TODO: Implement orphan detection
            duplicate_codes: vec![], // TODO: Implement duplicate detection
            invalid_language_codes: vec![], // TODO: Implement language code validation
            missing_primary_content: vec![], // TODO: Implement content validation
            constraint_violations: vec![], // TODO: Implement constraint validation
            generated_at: Utc::now(),
        })
    }
    
    async fn get_categories_in_use(&self) -> BankingResult<Vec<ReasonCategory>> {
        let rows = sqlx::query("SELECT DISTINCT category::text FROM reason_and_purpose WHERE is_active = true")
            .fetch_all(&self.pool)
            .await
            .map_err(BankingError::from)?;
        
        let mut categories = Vec::new();
        for row in rows {
            let category_str: String = row.get("category");
            if let Ok(category) = category_str.parse() {
                categories.push(category);
            }
        }
        
        Ok(categories)
    }
    
    async fn get_contexts_in_use(&self) -> BankingResult<Vec<ReasonContext>> {
        let rows = sqlx::query("SELECT DISTINCT context::text FROM reason_and_purpose WHERE is_active = true")
            .fetch_all(&self.pool)
            .await
            .map_err(BankingError::from)?;
        
        let mut contexts = Vec::new();
        for row in rows {
            let context_str: String = row.get("context");
            if let Ok(context) = context_str.parse() {
                contexts.push(context);
            }
        }
        
        Ok(contexts)
    }
}
use async_trait::async_trait;
use banking_db::models::{CollateralModel, CollateralEnforcementModel};
use banking_db::repository::CollateralRepository;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::NaiveDate;
use heapless::String as HeaplessString;

pub struct CollateralRepositoryImpl {
    pool: PgPool,
}

impl CollateralRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

/// Helper trait to extract models from database rows
trait TryFromRow<R> {
    fn try_from_row(row: &R) -> Result<Self, String>
    where
        Self: Sized;
}

impl TryFromRow<sqlx::postgres::PgRow> for CollateralModel {
    fn try_from_row(row: &sqlx::postgres::PgRow) -> Result<Self, String> {
        Ok(CollateralModel {
            collateral_id: row.get("collateral_id"),
            customer_id: row.get("customer_id"),
            collateral_type: row.get::<String, _>("collateral_type").parse().map_err(|_| {
                "Invalid collateral type".to_string()
            })?,
            category: row.get::<String, _>("category").parse().map_err(|_| {
                "Invalid collateral category".to_string()
            })?,
            description: HeaplessString::try_from(row.get::<String, _>("description").as_str()).map_err(|_| {
                "Description too long".to_string()
            })?,
            location: match row.get::<Option<String>, _>("location") {
                Some(loc) => Some(HeaplessString::try_from(loc.as_str()).map_err(|_| {
                    "Location field too long".to_string()
                })?),
                None => None,
            },
            custody_location: row.get::<String, _>("custody_location").parse().map_err(|_| {
                "Invalid custody location".to_string()
            })?,
            original_value: row.get("original_value"),
            current_market_value: row.get("current_market_value"),
            forced_sale_value: row.get("forced_sale_value"),
            loan_to_value_ratio: row.get("loan_to_value_ratio"),
            valuation_date: row.get("valuation_date"),
            next_valuation_due: row.get("next_valuation_due"),
            valuation_frequency_months: row.get("valuation_frequency_months"),
            perfection_status: row.get::<String, _>("perfection_status").parse().map_err(|_| {
                "Invalid perfection status".to_string()
            })?,
            perfection_date: row.get("perfection_date"),
            perfection_expiry: row.get("perfection_expiry"),
            insurance_required: row.get("insurance_required"),
            insurance_value: row.get("insurance_value"),
            insurance_expiry: row.get("insurance_expiry"),
            environmental_risk: row.get::<String, _>("environmental_risk").parse().map_err(|_| {
                "Invalid environmental risk".to_string()
            })?,
            risk_rating: row.get::<String, _>("risk_rating").parse().map_err(|_| {
                "Invalid risk rating".to_string()
            })?,
            status: HeaplessString::try_from(row.get::<String, _>("status").as_str()).map_err(|_| {
                "Status field too long".to_string()
            })?,
            notes: match row.get::<Option<String>, _>("notes") {
                Some(notes) => Some(HeaplessString::try_from(notes.as_str()).map_err(|_| {
                    "Notes field too long".to_string()
                })?),
                None => None,
            },
            legal_description: match row.get::<Option<String>, _>("legal_description") {
                Some(desc) => Some(HeaplessString::try_from(desc.as_str()).map_err(|_| {
                    "Legal description too long".to_string()
                })?),
                None => None,
            },
            appraisal_reference: match row.get::<Option<String>, _>("appraisal_reference") {
                Some(ref_str) => Some(HeaplessString::try_from(ref_str.as_str()).map_err(|_| {
                    "Appraisal reference too long".to_string()
                })?),
                None => None,
            },
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            created_by: row.get("created_by"),
            updated_by: row.get("updated_by"),
        })
    }
}

impl TryFromRow<sqlx::postgres::PgRow> for CollateralEnforcementModel {
    fn try_from_row(row: &sqlx::postgres::PgRow) -> Result<Self, String> {
        Ok(CollateralEnforcementModel {
            enforcement_id: row.get("enforcement_id"),
            collateral_id: row.get("collateral_id"),
            loan_account_id: row.get("loan_account_id"),
            enforcement_type: HeaplessString::try_from(row.get::<String, _>("enforcement_type").as_str()).map_err(|_| {
                "Enforcement type too long".to_string()
            })?,
            status: HeaplessString::try_from(row.get::<String, _>("status").as_str()).map_err(|_| {
                "Status field too long".to_string()
            })?,
            initiated_date: row.get("initiated_date"),
            initiated_by: row.get("initiated_by"),
            target_recovery_amount: row.get("target_recovery_amount"),
            actual_recovery_amount: row.get("actual_recovery_amount"),
            enforcement_costs: row.get("enforcement_costs"),
            net_recovery: row.get("net_recovery"),
            completion_date: row.get("completion_date"),
            completion_notes: match row.get::<Option<String>, _>("completion_notes") {
                Some(notes) => Some(HeaplessString::try_from(notes.as_str()).map_err(|_| {
                    "Completion notes too long".to_string()
                })?),
                None => None,
            },
            legal_proceedings: row.get("legal_proceedings"),
            external_agent: match row.get::<Option<String>, _>("external_agent") {
                Some(agent) => Some(HeaplessString::try_from(agent.as_str()).map_err(|_| {
                    "External agent field too long".to_string()
                })?),
                None => None,
            },
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

#[async_trait]
impl CollateralRepository for CollateralRepositoryImpl {
    // === CORE COLLATERAL CRUD ===
    
    async fn save_collateral(&self, collateral: &CollateralModel) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO collaterals (
                collateral_id, customer_id, collateral_type, category, description, location,
                custody_location, original_value, current_market_value, forced_sale_value,
                loan_to_value_ratio, valuation_date, next_valuation_due, valuation_frequency_months,
                perfection_status, perfection_date, perfection_expiry, insurance_required,
                insurance_value, insurance_expiry, environmental_risk, risk_rating, status,
                notes, legal_description, appraisal_reference, created_at, updated_at,
                created_by, updated_by
            )
            VALUES (
                $1, $2, $3::collateral_type, $4::collateral_category, $5, $6,
                $7::custody_location, $8, $9, $10, $11, $12, $13, $14,
                $15::perfection_status, $16, $17, $18, $19, $20,
                $21::collateral_risk_rating, $22::collateral_risk_rating, $23, $24, $25, $26,
                $27, $28, $29, $30
            )
            ON CONFLICT (collateral_id) DO UPDATE SET
                customer_id = EXCLUDED.customer_id,
                collateral_type = EXCLUDED.collateral_type,
                category = EXCLUDED.category,
                description = EXCLUDED.description,
                location = EXCLUDED.location,
                custody_location = EXCLUDED.custody_location,
                original_value = EXCLUDED.original_value,
                current_market_value = EXCLUDED.current_market_value,
                forced_sale_value = EXCLUDED.forced_sale_value,
                loan_to_value_ratio = EXCLUDED.loan_to_value_ratio,
                valuation_date = EXCLUDED.valuation_date,
                next_valuation_due = EXCLUDED.next_valuation_due,
                valuation_frequency_months = EXCLUDED.valuation_frequency_months,
                perfection_status = EXCLUDED.perfection_status,
                perfection_date = EXCLUDED.perfection_date,
                perfection_expiry = EXCLUDED.perfection_expiry,
                insurance_required = EXCLUDED.insurance_required,
                insurance_value = EXCLUDED.insurance_value,
                insurance_expiry = EXCLUDED.insurance_expiry,
                environmental_risk = EXCLUDED.environmental_risk,
                risk_rating = EXCLUDED.risk_rating,
                status = EXCLUDED.status,
                notes = EXCLUDED.notes,
                legal_description = EXCLUDED.legal_description,
                appraisal_reference = EXCLUDED.appraisal_reference,
                updated_at = EXCLUDED.updated_at,
                updated_by = EXCLUDED.updated_by
            "#
        )
        .bind(collateral.collateral_id)
        .bind(collateral.customer_id)
        .bind(collateral.collateral_type.to_string())
        .bind(collateral.category.to_string())
        .bind(collateral.description.as_str())
        .bind(collateral.location.as_ref().map(|s| s.as_str()))
        .bind(collateral.custody_location.to_string())
        .bind(collateral.original_value)
        .bind(collateral.current_market_value)
        .bind(collateral.forced_sale_value)
        .bind(collateral.loan_to_value_ratio)
        .bind(collateral.valuation_date)
        .bind(collateral.next_valuation_due)
        .bind(collateral.valuation_frequency_months)
        .bind(collateral.perfection_status.to_string())
        .bind(collateral.perfection_date)
        .bind(collateral.perfection_expiry)
        .bind(collateral.insurance_required)
        .bind(collateral.insurance_value)
        .bind(collateral.insurance_expiry)
        .bind(collateral.environmental_risk.to_string())
        .bind(collateral.risk_rating.to_string())
        .bind(collateral.status.as_str())
        .bind(collateral.notes.as_ref().map(|s| s.as_str()))
        .bind(collateral.legal_description.as_ref().map(|s| s.as_str()))
        .bind(collateral.appraisal_reference.as_ref().map(|s| s.as_str()))
        .bind(collateral.created_at)
        .bind(collateral.updated_at)
        .bind(collateral.created_by)
        .bind(collateral.updated_by)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save collateral: {}", e))?;

        Ok(())
    }
    
    async fn find_collateral_by_id(&self, collateral_id: Uuid) -> Result<Option<CollateralModel>, String> {
        let result = sqlx::query(
            r#"
            SELECT collateral_id, customer_id, collateral_type::text as collateral_type, 
                   category::text as category, description, location, custody_location::text as custody_location,
                   original_value, current_market_value, forced_sale_value, loan_to_value_ratio,
                   valuation_date, next_valuation_due, valuation_frequency_months,
                   perfection_status::text as perfection_status, perfection_date, perfection_expiry,
                   insurance_required, insurance_value, insurance_expiry,
                   environmental_risk::text as environmental_risk, risk_rating::text as risk_rating,
                   status, notes, legal_description, appraisal_reference,
                   created_at, updated_at, created_by, updated_by
            FROM collaterals 
            WHERE collateral_id = $1
            "#
        )
        .bind(collateral_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find collateral: {}", e))?;

        match result {
            Some(row) => Ok(Some(CollateralModel::try_from_row(&row)?)),
            None => Ok(None),
        }
    }
    
    async fn find_collaterals_by_customer(&self, customer_id: Uuid) -> Result<Vec<CollateralModel>, String> {
        let results = sqlx::query(
            r#"
            SELECT collateral_id, customer_id, collateral_type::text as collateral_type, 
                   category::text as category, description, location, custody_location::text as custody_location,
                   original_value, current_market_value, forced_sale_value, loan_to_value_ratio,
                   valuation_date, next_valuation_due, valuation_frequency_months,
                   perfection_status::text as perfection_status, perfection_date, perfection_expiry,
                   insurance_required, insurance_value, insurance_expiry,
                   environmental_risk::text as environmental_risk, risk_rating::text as risk_rating,
                   status, notes, legal_description, appraisal_reference,
                   created_at, updated_at, created_by, updated_by
            FROM collaterals 
            WHERE customer_id = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find collaterals by customer: {}", e))?;

        let mut collaterals = Vec::new();
        for row in results {
            collaterals.push(CollateralModel::try_from_row(&row)?);
        }
        Ok(collaterals)
    }
    
    async fn find_collaterals_by_type(&self, collateral_type: String) -> Result<Vec<CollateralModel>, String> {
        let results = sqlx::query(
            r#"
            SELECT collateral_id, customer_id, collateral_type::text as collateral_type, 
                   category::text as category, description, location, custody_location::text as custody_location,
                   original_value, current_market_value, forced_sale_value, loan_to_value_ratio,
                   valuation_date, next_valuation_due, valuation_frequency_months,
                   perfection_status::text as perfection_status, perfection_date, perfection_expiry,
                   insurance_required, insurance_value, insurance_expiry,
                   environmental_risk::text as environmental_risk, risk_rating::text as risk_rating,
                   status, notes, legal_description, appraisal_reference,
                   created_at, updated_at, created_by, updated_by
            FROM collaterals 
            WHERE collateral_type = $1::collateral_type
            ORDER BY created_at DESC
            "#
        )
        .bind(collateral_type)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find collaterals by type: {}", e))?;

        let mut collaterals = Vec::new();
        for row in results {
            collaterals.push(CollateralModel::try_from_row(&row)?);
        }
        Ok(collaterals)
    }
    
    async fn find_collaterals_by_status(&self, status: String) -> Result<Vec<CollateralModel>, String> {
        let results = sqlx::query(
            r#"
            SELECT collateral_id, customer_id, collateral_type::text as collateral_type, 
                   category::text as category, description, location, custody_location::text as custody_location,
                   original_value, current_market_value, forced_sale_value, loan_to_value_ratio,
                   valuation_date, next_valuation_due, valuation_frequency_months,
                   perfection_status::text as perfection_status, perfection_date, perfection_expiry,
                   insurance_required, insurance_value, insurance_expiry,
                   environmental_risk::text as environmental_risk, risk_rating::text as risk_rating,
                   status, notes, legal_description, appraisal_reference,
                   created_at, updated_at, created_by, updated_by
            FROM collaterals 
            WHERE status = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(status)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find collaterals by status: {}", e))?;

        let mut collaterals = Vec::new();
        for row in results {
            collaterals.push(CollateralModel::try_from_row(&row)?);
        }
        Ok(collaterals)
    }
    
    async fn search_collaterals(
        &self,
        collateral_type: Option<String>,
        risk_rating: Option<String>,
        status: Option<String>,
        limit: u32,
        offset: u32
    ) -> Result<Vec<CollateralModel>, String> {
        let mut query = String::from(
            r#"
            SELECT collateral_id, customer_id, collateral_type::text as collateral_type, 
                   category::text as category, description, location, custody_location::text as custody_location,
                   original_value, current_market_value, forced_sale_value, loan_to_value_ratio,
                   valuation_date, next_valuation_due, valuation_frequency_months,
                   perfection_status::text as perfection_status, perfection_date, perfection_expiry,
                   insurance_required, insurance_value, insurance_expiry,
                   environmental_risk::text as environmental_risk, risk_rating::text as risk_rating,
                   status, notes, legal_description, appraisal_reference,
                   created_at, updated_at, created_by, updated_by
            FROM collaterals WHERE 1=1
            "#
        );
        
        let mut bind_index = 1;
        if collateral_type.is_some() {
            query.push_str(&format!(" AND collateral_type = ${}::collateral_type", bind_index));
            bind_index += 1;
        }
        if risk_rating.is_some() {
            query.push_str(&format!(" AND risk_rating = ${}::collateral_risk_rating", bind_index));
            bind_index += 1;
        }
        if status.is_some() {
            query.push_str(&format!(" AND status = ${}", bind_index));
            bind_index += 1;
        }
        
        query.push_str(&format!(" ORDER BY created_at DESC LIMIT ${} OFFSET ${}", bind_index, bind_index + 1));

        let mut sql_query = sqlx::query(&query);
        
        if let Some(ct) = collateral_type {
            sql_query = sql_query.bind(ct);
        }
        if let Some(rr) = risk_rating {
            sql_query = sql_query.bind(rr);
        }
        if let Some(st) = status {
            sql_query = sql_query.bind(st);
        }
        
        sql_query = sql_query.bind(limit as i64).bind(offset as i64);

        let results = sql_query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to search collaterals: {}", e))?;

        let mut collaterals = Vec::new();
        for row in results {
            collaterals.push(CollateralModel::try_from_row(&row)?);
        }
        Ok(collaterals)
    }
    
    async fn count_collaterals(
        &self,
        collateral_type: Option<String>,
        risk_rating: Option<String>,
        status: Option<String>
    ) -> Result<u64, String> {
        let mut query = String::from("SELECT COUNT(*) as count FROM collaterals WHERE 1=1");
        
        let mut bind_index = 1;
        if collateral_type.is_some() {
            query.push_str(&format!(" AND collateral_type = ${}::collateral_type", bind_index));
            bind_index += 1;
        }
        if risk_rating.is_some() {
            query.push_str(&format!(" AND risk_rating = ${}::collateral_risk_rating", bind_index));
            bind_index += 1;
        }
        if status.is_some() {
            query.push_str(&format!(" AND status = ${}", bind_index));
        }

        let mut sql_query = sqlx::query(&query);
        
        if let Some(ct) = collateral_type {
            sql_query = sql_query.bind(ct);
        }
        if let Some(rr) = risk_rating {
            sql_query = sql_query.bind(rr);
        }
        if let Some(st) = status {
            sql_query = sql_query.bind(st);
        }

        let result = sql_query
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Failed to count collaterals: {}", e))?;

        Ok(result.get::<i64, _>("count") as u64)
    }
    
    async fn update_collateral_status(&self, collateral_id: Uuid, status: String, updated_by: Uuid) -> Result<(), String> {
        sqlx::query(
            r#"
            UPDATE collaterals SET
                status = $2,
                updated_by = $3,
                updated_at = CURRENT_TIMESTAMP
            WHERE collateral_id = $1
            "#
        )
        .bind(collateral_id)
        .bind(status)
        .bind(updated_by)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to update collateral status: {}", e))?;

        Ok(())
    }
    
    async fn update_market_value(&self, collateral_id: Uuid, new_value: Decimal, valuation_date: NaiveDate, updated_by: Uuid) -> Result<(), String> {
        sqlx::query(
            r#"
            UPDATE collaterals SET
                current_market_value = $2,
                valuation_date = $3,
                updated_by = $4,
                updated_at = CURRENT_TIMESTAMP
            WHERE collateral_id = $1
            "#
        )
        .bind(collateral_id)
        .bind(new_value)
        .bind(valuation_date)
        .bind(updated_by)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to update market value: {}", e))?;

        Ok(())
    }

    // === VALUATION OPERATIONS ===
    
    async fn save_valuation(&self, collateral_id: Uuid, valuation_data: String) -> Result<(), String> {
        // For now, just log the valuation - in a full implementation this would be stored in a valuations table
        Ok(())
    }
    
    async fn find_valuations_by_collateral(&self, collateral_id: Uuid) -> Result<Vec<String>, String> {
        // Placeholder - would return actual valuation records
        Ok(Vec::new())
    }
    
    async fn find_latest_valuation(&self, collateral_id: Uuid) -> Result<Option<String>, String> {
        // Placeholder - would return the most recent valuation
        Ok(None)
    }
    
    async fn find_valuations_due(&self, reference_date: NaiveDate) -> Result<Vec<CollateralModel>, String> {
        let results = sqlx::query(
            r#"
            SELECT collateral_id, customer_id, collateral_type::text as collateral_type, 
                   category::text as category, description, location, custody_location::text as custody_location,
                   original_value, current_market_value, forced_sale_value, loan_to_value_ratio,
                   valuation_date, next_valuation_due, valuation_frequency_months,
                   perfection_status::text as perfection_status, perfection_date, perfection_expiry,
                   insurance_required, insurance_value, insurance_expiry,
                   environmental_risk::text as environmental_risk, risk_rating::text as risk_rating,
                   status, notes, legal_description, appraisal_reference,
                   created_at, updated_at, created_by, updated_by
            FROM collaterals 
            WHERE next_valuation_due <= $1 AND status = 'Active'
            ORDER BY next_valuation_due ASC
            "#
        )
        .bind(reference_date)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find valuations due: {}", e))?;

        let mut collaterals = Vec::new();
        for row in results {
            collaterals.push(CollateralModel::try_from_row(&row)?);
        }
        Ok(collaterals)
    }
    
    async fn find_overdue_valuations(&self, reference_date: NaiveDate) -> Result<Vec<CollateralModel>, String> {
        let results = sqlx::query(
            r#"
            SELECT collateral_id, customer_id, collateral_type::text as collateral_type, 
                   category::text as category, description, location, custody_location::text as custody_location,
                   original_value, current_market_value, forced_sale_value, loan_to_value_ratio,
                   valuation_date, next_valuation_due, valuation_frequency_months,
                   perfection_status::text as perfection_status, perfection_date, perfection_expiry,
                   insurance_required, insurance_value, insurance_expiry,
                   environmental_risk::text as environmental_risk, risk_rating::text as risk_rating,
                   status, notes, legal_description, appraisal_reference,
                   created_at, updated_at, created_by, updated_by
            FROM collaterals 
            WHERE next_valuation_due < $1 AND status = 'Active'
            ORDER BY next_valuation_due ASC
            "#
        )
        .bind(reference_date)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find overdue valuations: {}", e))?;

        let mut collaterals = Vec::new();
        for row in results {
            collaterals.push(CollateralModel::try_from_row(&row)?);
        }
        Ok(collaterals)
    }
    
    async fn find_valuations_by_date_range(&self, _from_date: NaiveDate, _to_date: NaiveDate) -> Result<Vec<String>, String> {
        // Placeholder implementation
        Ok(Vec::new())
    }

    // === PLEDGE OPERATIONS - Simplified implementations ===
    
    async fn save_pledge(&self, _collateral_id: Uuid, _pledge_data: String) -> Result<(), String> {
        Ok(())
    }
    
    async fn find_pledge_by_id(&self, _pledge_id: Uuid) -> Result<Option<String>, String> {
        Ok(None)
    }
    
    async fn find_pledges_by_collateral(&self, _collateral_id: Uuid) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }
    
    async fn find_pledges_by_loan_account(&self, _loan_account_id: Uuid) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }
    
    async fn find_active_pledges_by_collateral(&self, _collateral_id: Uuid) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }
    
    async fn update_pledge_status(&self, _pledge_id: Uuid, _status: String, _updated_by: Uuid) -> Result<(), String> {
        Ok(())
    }
    
    async fn update_pledged_amount(&self, _pledge_id: Uuid, _new_amount: Decimal, _updated_by: Uuid) -> Result<(), String> {
        Ok(())
    }
    
    async fn find_pledges_by_priority(&self, _priority: String) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }

    // === ALERT OPERATIONS - Simplified implementations ===
    
    async fn save_alert(&self, _collateral_id: Uuid, _alert_data: String) -> Result<(), String> {
        Ok(())
    }
    
    async fn find_alert_by_id(&self, _alert_id: Uuid) -> Result<Option<String>, String> {
        Ok(None)
    }
    
    async fn find_alerts_by_collateral(&self, _collateral_id: Uuid) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }
    
    async fn find_active_alerts(&self) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }
    
    async fn find_alerts_by_severity(&self, _severity: String) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }
    
    async fn find_alerts_by_type(&self, _alert_type: String) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }
    
    async fn find_alerts_by_assignee(&self, _assigned_to: Uuid) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }
    
    async fn update_alert_status(&self, _alert_id: Uuid, _status: String, _updated_by: Uuid) -> Result<(), String> {
        Ok(())
    }
    
    async fn resolve_alert(&self, _alert_id: Uuid, _resolution_notes: String, _resolved_by: Uuid) -> Result<(), String> {
        Ok(())
    }

    // === ENFORCEMENT OPERATIONS ===
    
    async fn save_enforcement(&self, enforcement: &CollateralEnforcementModel) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO collateral_enforcements (
                enforcement_id, collateral_id, loan_account_id, enforcement_type, status,
                initiated_date, initiated_by, target_recovery_amount, actual_recovery_amount,
                enforcement_costs, net_recovery, completion_date, completion_notes,
                legal_proceedings, external_agent, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            ON CONFLICT (enforcement_id) DO UPDATE SET
                status = EXCLUDED.status,
                actual_recovery_amount = EXCLUDED.actual_recovery_amount,
                enforcement_costs = EXCLUDED.enforcement_costs,
                net_recovery = EXCLUDED.net_recovery,
                completion_date = EXCLUDED.completion_date,
                completion_notes = EXCLUDED.completion_notes,
                legal_proceedings = EXCLUDED.legal_proceedings,
                external_agent = EXCLUDED.external_agent,
                updated_at = EXCLUDED.updated_at
            "#
        )
        .bind(enforcement.enforcement_id)
        .bind(enforcement.collateral_id)
        .bind(enforcement.loan_account_id)
        .bind(enforcement.enforcement_type.as_str())
        .bind(enforcement.status.as_str())
        .bind(enforcement.initiated_date)
        .bind(enforcement.initiated_by)
        .bind(enforcement.target_recovery_amount)
        .bind(enforcement.actual_recovery_amount)
        .bind(enforcement.enforcement_costs)
        .bind(enforcement.net_recovery)
        .bind(enforcement.completion_date)
        .bind(enforcement.completion_notes.as_ref().map(|s| s.as_str()))
        .bind(enforcement.legal_proceedings)
        .bind(enforcement.external_agent.as_ref().map(|s| s.as_str()))
        .bind(enforcement.created_at)
        .bind(enforcement.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save enforcement: {}", e))?;

        Ok(())
    }
    
    async fn find_enforcement_by_id(&self, enforcement_id: Uuid) -> Result<Option<CollateralEnforcementModel>, String> {
        let result = sqlx::query(
            r#"
            SELECT enforcement_id, collateral_id, loan_account_id, enforcement_type, status,
                   initiated_date, initiated_by, target_recovery_amount, actual_recovery_amount,
                   enforcement_costs, net_recovery, completion_date, completion_notes,
                   legal_proceedings, external_agent, created_at, updated_at
            FROM collateral_enforcements 
            WHERE enforcement_id = $1
            "#
        )
        .bind(enforcement_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find enforcement: {}", e))?;

        match result {
            Some(row) => Ok(Some(CollateralEnforcementModel::try_from_row(&row)?)),
            None => Ok(None),
        }
    }
    
    async fn find_enforcements_by_collateral(&self, collateral_id: Uuid) -> Result<Vec<CollateralEnforcementModel>, String> {
        let results = sqlx::query(
            r#"
            SELECT enforcement_id, collateral_id, loan_account_id, enforcement_type, status,
                   initiated_date, initiated_by, target_recovery_amount, actual_recovery_amount,
                   enforcement_costs, net_recovery, completion_date, completion_notes,
                   legal_proceedings, external_agent, created_at, updated_at
            FROM collateral_enforcements 
            WHERE collateral_id = $1
            ORDER BY initiated_date DESC
            "#
        )
        .bind(collateral_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find enforcements by collateral: {}", e))?;

        let mut enforcements = Vec::new();
        for row in results {
            enforcements.push(CollateralEnforcementModel::try_from_row(&row)?);
        }
        Ok(enforcements)
    }
    
    async fn find_enforcements_by_loan_account(&self, loan_account_id: Uuid) -> Result<Vec<CollateralEnforcementModel>, String> {
        let results = sqlx::query(
            r#"
            SELECT enforcement_id, collateral_id, loan_account_id, enforcement_type, status,
                   initiated_date, initiated_by, target_recovery_amount, actual_recovery_amount,
                   enforcement_costs, net_recovery, completion_date, completion_notes,
                   legal_proceedings, external_agent, created_at, updated_at
            FROM collateral_enforcements 
            WHERE loan_account_id = $1
            ORDER BY initiated_date DESC
            "#
        )
        .bind(loan_account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find enforcements by loan account: {}", e))?;

        let mut enforcements = Vec::new();
        for row in results {
            enforcements.push(CollateralEnforcementModel::try_from_row(&row)?);
        }
        Ok(enforcements)
    }
    
    async fn find_enforcements_by_status(&self, status: String) -> Result<Vec<CollateralEnforcementModel>, String> {
        let results = sqlx::query(
            r#"
            SELECT enforcement_id, collateral_id, loan_account_id, enforcement_type, status,
                   initiated_date, initiated_by, target_recovery_amount, actual_recovery_amount,
                   enforcement_costs, net_recovery, completion_date, completion_notes,
                   legal_proceedings, external_agent, created_at, updated_at
            FROM collateral_enforcements 
            WHERE status = $1
            ORDER BY initiated_date DESC
            "#
        )
        .bind(status)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find enforcements by status: {}", e))?;

        let mut enforcements = Vec::new();
        for row in results {
            enforcements.push(CollateralEnforcementModel::try_from_row(&row)?);
        }
        Ok(enforcements)
    }
    
    async fn update_enforcement_status(&self, enforcement_id: Uuid, status: String, updated_by: Uuid) -> Result<(), String> {
        sqlx::query(
            r#"
            UPDATE collateral_enforcements SET
                status = $2,
                updated_at = CURRENT_TIMESTAMP
            WHERE enforcement_id = $1
            "#
        )
        .bind(enforcement_id)
        .bind(status)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to update enforcement status: {}", e))?;

        Ok(())
    }
    
    async fn complete_enforcement(
        &self,
        enforcement_id: Uuid,
        recovery_amount: Decimal,
        enforcement_costs: Decimal,
        net_recovery: Decimal,
        completed_by: Uuid
    ) -> Result<(), String> {
        sqlx::query(
            r#"
            UPDATE collateral_enforcements SET
                actual_recovery_amount = $2,
                enforcement_costs = $3,
                net_recovery = $4,
                completion_date = CURRENT_DATE,
                status = 'Completed',
                updated_at = CURRENT_TIMESTAMP
            WHERE enforcement_id = $1
            "#
        )
        .bind(enforcement_id)
        .bind(recovery_amount)
        .bind(enforcement_costs)
        .bind(net_recovery)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to complete enforcement: {}", e))?;

        Ok(())
    }

    // === PORTFOLIO AND ANALYTICS - Simplified implementations ===
    
    async fn calculate_total_portfolio_value(&self, _portfolio_id: Uuid) -> Result<Decimal, String> {
        Ok(Decimal::ZERO)
    }
    
    async fn calculate_total_pledged_value(&self, _portfolio_id: Uuid) -> Result<Decimal, String> {
        Ok(Decimal::ZERO)
    }
    
    async fn calculate_weighted_average_ltv(&self, _portfolio_id: Uuid) -> Result<Decimal, String> {
        Ok(Decimal::ZERO)
    }
    
    async fn get_concentration_by_type(&self, _portfolio_id: Uuid) -> Result<Vec<(String, u32, Decimal)>, String> {
        Ok(Vec::new())
    }
    
    async fn get_concentration_by_location(&self, _portfolio_id: Uuid) -> Result<Vec<(String, u32, Decimal)>, String> {
        Ok(Vec::new())
    }
    
    async fn get_risk_distribution(&self, _portfolio_id: Uuid) -> Result<Vec<(String, u32, Decimal)>, String> {
        Ok(Vec::new())
    }
    
    async fn get_valuation_status_summary(&self, _portfolio_id: Uuid) -> Result<(u32, u32, u32, i32), String> {
        Ok((0, 0, 0, 0))
    }
    
    async fn get_compliance_summary(&self, _portfolio_id: Uuid) -> Result<(u32, u32, u32, u32), String> {
        Ok((0, 0, 0, 0))
    }
    
    async fn find_collaterals_by_ltv_threshold(&self, min_ltv: Decimal, max_ltv: Option<Decimal>) -> Result<Vec<CollateralModel>, String> {
        let mut query = String::from(
            r#"
            SELECT collateral_id, customer_id, collateral_type::text as collateral_type, 
                   category::text as category, description, location, custody_location::text as custody_location,
                   original_value, current_market_value, forced_sale_value, loan_to_value_ratio,
                   valuation_date, next_valuation_due, valuation_frequency_months,
                   perfection_status::text as perfection_status, perfection_date, perfection_expiry,
                   insurance_required, insurance_value, insurance_expiry,
                   environmental_risk::text as environmental_risk, risk_rating::text as risk_rating,
                   status, notes, legal_description, appraisal_reference,
                   created_at, updated_at, created_by, updated_by
            FROM collaterals 
            WHERE loan_to_value_ratio >= $1
            "#
        );
        
        let mut sql_query = sqlx::query(&query).bind(min_ltv);
        
        if let Some(max) = max_ltv {
            query.push_str(" AND loan_to_value_ratio <= $2");
            sql_query = sql_query.bind(max);
        }
        
        query.push_str(" ORDER BY loan_to_value_ratio DESC");

        let results = sql_query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to find collaterals by LTV: {}", e))?;

        let mut collaterals = Vec::new();
        for row in results {
            collaterals.push(CollateralModel::try_from_row(&row)?);
        }
        Ok(collaterals)
    }

    // === BATCH OPERATIONS - Simplified implementations ===
    
    async fn batch_update_market_values(&self, updates: Vec<(Uuid, Decimal, NaiveDate)>, updated_by: Uuid) -> Result<u32, String> {
        let mut count = 0;
        for (collateral_id, value, date) in updates {
            self.update_market_value(collateral_id, value, date, updated_by).await?;
            count += 1;
        }
        Ok(count)
    }
    
    async fn batch_create_alerts(&self, _alert_data: Vec<String>) -> Result<u32, String> {
        Ok(0)
    }
    
    async fn batch_update_pledge_statuses(&self, _updates: Vec<(Uuid, String)>, _updated_by: Uuid) -> Result<u32, String> {
        Ok(0)
    }

    // === REPORTING QUERIES - Simplified implementations ===
    
    async fn find_collaterals_by_custody_location(&self, custody_location: String) -> Result<Vec<CollateralModel>, String> {
        self.find_collaterals_by_type(custody_location).await
    }
    
    async fn find_collaterals_requiring_insurance_review(&self, reference_date: NaiveDate) -> Result<Vec<CollateralModel>, String> {
        let results = sqlx::query(
            r#"
            SELECT collateral_id, customer_id, collateral_type::text as collateral_type, 
                   category::text as category, description, location, custody_location::text as custody_location,
                   original_value, current_market_value, forced_sale_value, loan_to_value_ratio,
                   valuation_date, next_valuation_due, valuation_frequency_months,
                   perfection_status::text as perfection_status, perfection_date, perfection_expiry,
                   insurance_required, insurance_value, insurance_expiry,
                   environmental_risk::text as environmental_risk, risk_rating::text as risk_rating,
                   status, notes, legal_description, appraisal_reference,
                   created_at, updated_at, created_by, updated_by
            FROM collaterals 
            WHERE insurance_required = true AND insurance_expiry <= $1
            ORDER BY insurance_expiry ASC
            "#
        )
        .bind(reference_date)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find collaterals requiring insurance review: {}", e))?;

        let mut collaterals = Vec::new();
        for row in results {
            collaterals.push(CollateralModel::try_from_row(&row)?);
        }
        Ok(collaterals)
    }
    
    async fn find_collaterals_with_expiring_perfection(&self, days_ahead: i32) -> Result<Vec<CollateralModel>, String> {
        let target_date = chrono::Utc::now().date_naive() + chrono::Duration::days(days_ahead as i64);
        
        let results = sqlx::query(
            r#"
            SELECT collateral_id, customer_id, collateral_type::text as collateral_type, 
                   category::text as category, description, location, custody_location::text as custody_location,
                   original_value, current_market_value, forced_sale_value, loan_to_value_ratio,
                   valuation_date, next_valuation_due, valuation_frequency_months,
                   perfection_status::text as perfection_status, perfection_date, perfection_expiry,
                   insurance_required, insurance_value, insurance_expiry,
                   environmental_risk::text as environmental_risk, risk_rating::text as risk_rating,
                   status, notes, legal_description, appraisal_reference,
                   created_at, updated_at, created_by, updated_by
            FROM collaterals 
            WHERE perfection_expiry <= $1 AND perfection_status = 'perfected'
            ORDER BY perfection_expiry ASC
            "#
        )
        .bind(target_date)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find collaterals with expiring perfection: {}", e))?;

        let mut collaterals = Vec::new();
        for row in results {
            collaterals.push(CollateralModel::try_from_row(&row)?);
        }
        Ok(collaterals)
    }
    
    async fn get_collateral_performance_history(&self, _collateral_id: Uuid, _from_date: NaiveDate, _to_date: NaiveDate) -> Result<Vec<(NaiveDate, Decimal)>, String> {
        Ok(Vec::new())
    }
    
    async fn find_collaterals_by_environmental_risk(&self, risk_level: String) -> Result<Vec<CollateralModel>, String> {
        let results = sqlx::query(
            r#"
            SELECT collateral_id, customer_id, collateral_type::text as collateral_type, 
                   category::text as category, description, location, custody_location::text as custody_location,
                   original_value, current_market_value, forced_sale_value, loan_to_value_ratio,
                   valuation_date, next_valuation_due, valuation_frequency_months,
                   perfection_status::text as perfection_status, perfection_date, perfection_expiry,
                   insurance_required, insurance_value, insurance_expiry,
                   environmental_risk::text as environmental_risk, risk_rating::text as risk_rating,
                   status, notes, legal_description, appraisal_reference,
                   created_at, updated_at, created_by, updated_by
            FROM collaterals 
            WHERE environmental_risk = $1::collateral_risk_rating
            ORDER BY created_at DESC
            "#
        )
        .bind(risk_level)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find collaterals by environmental risk: {}", e))?;

        let mut collaterals = Vec::new();
        for row in results {
            collaterals.push(CollateralModel::try_from_row(&row)?);
        }
        Ok(collaterals)
    }

    // === COVENANT MONITORING - Simplified implementations ===
    
    async fn find_covenant_breaches(&self, _reference_date: NaiveDate) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }
    
    async fn update_covenant_compliance(&self, _pledge_id: Uuid, _compliance_data: String, _updated_by: Uuid) -> Result<(), String> {
        Ok(())
    }
    
    async fn find_pledges_requiring_covenant_review(&self, _reference_date: NaiveDate) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }

    // === AUDIT AND HISTORY - Simplified implementations ===
    
    async fn get_collateral_audit_trail(&self, _collateral_id: Uuid) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }
    
    async fn get_pledge_audit_trail(&self, _pledge_id: Uuid) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }
    
    async fn get_valuation_history(&self, _collateral_id: Uuid) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }

    // === CLEANUP AND MAINTENANCE - Simplified implementations ===
    
    async fn archive_old_alerts(&self, _cutoff_date: NaiveDate) -> Result<u32, String> {
        Ok(0)
    }
    
    async fn archive_completed_enforcements(&self, _cutoff_date: NaiveDate) -> Result<u32, String> {
        Ok(0)
    }
    
    async fn cleanup_temporary_valuations(&self, _cutoff_date: NaiveDate) -> Result<u32, String> {
        Ok(0)
    }
}

#[cfg(feature = "postgres_tests")]
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use banking_db::models::collateral::{CollateralType, CollateralCategory, CustodyLocation, PerfectionStatus, CollateralRiskRating};

    async fn setup_test_db() -> PgPool {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://ledger_user:ledger_password@localhost:5432/ledger_banking".to_string());
        
        PgPool::connect(&database_url).await.expect("Failed to connect to test database")
    }

    fn create_test_collateral() -> CollateralModel {
        CollateralModel {
            collateral_id: Uuid::new_v4(),
            customer_id: Uuid::new_v4(),
            collateral_type: CollateralType::ResidentialProperty,
            category: CollateralCategory::Immovable,
            description: HeaplessString::try_from("Test Residential Property").unwrap(),
            location: Some(HeaplessString::try_from("123 Test Street").unwrap()),
            custody_location: CustodyLocation::ClientPremises,
            original_value: Decimal::from(500000),
            current_market_value: Decimal::from(480000),
            forced_sale_value: Some(Decimal::from(400000)),
            loan_to_value_ratio: Some(Decimal::from_str("0.75").unwrap()),
            valuation_date: Some(Utc::now().date_naive()),
            next_valuation_due: Some(Utc::now().date_naive() + chrono::Duration::days(365)),
            valuation_frequency_months: Some(12),
            perfection_status: PerfectionStatus::Perfected,
            perfection_date: Some(Utc::now().date_naive()),
            perfection_expiry: Some(Utc::now().date_naive() + chrono::Duration::days(1825)), // 5 years
            insurance_required: true,
            insurance_value: Some(Decimal::from(500000)),
            insurance_expiry: Some(Utc::now().date_naive() + chrono::Duration::days(365)),
            environmental_risk: CollateralRiskRating::Good,
            risk_rating: CollateralRiskRating::Good,
            status: HeaplessString::try_from("Active").unwrap(),
            notes: Some(HeaplessString::try_from("Test collateral notes").unwrap()),
            legal_description: Some(HeaplessString::try_from("Lot 1, Block 2, Test Subdivision").unwrap()),
            appraisal_reference: Some(HeaplessString::try_from("APR-2024-001").unwrap()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: Uuid::new_v4(),
            updated_by: Uuid::new_v4(),
        }
    }

    #[tokio::test]
    async fn test_save_and_find_collateral() {
        let pool = setup_test_db().await;
        let repo = CollateralRepositoryImpl::new(pool);
        let test_collateral = create_test_collateral();

        // Save collateral
        let result = repo.save_collateral(&test_collateral).await;
        assert!(result.is_ok());
        
        // Find by ID
        let found = repo.find_collateral_by_id(test_collateral.collateral_id).await;
        assert!(found.is_ok());
        
        let collateral = found.unwrap();
        assert!(collateral.is_some());
        let found_collateral = collateral.unwrap();
        assert_eq!(found_collateral.collateral_id, test_collateral.collateral_id);
        assert_eq!(found_collateral.customer_id, test_collateral.customer_id);
    }

    #[tokio::test]
    async fn test_find_collaterals_by_customer() {
        let pool = setup_test_db().await;
        let repo = CollateralRepositoryImpl::new(pool);
        let test_collateral = create_test_collateral();

        // Save collateral
        repo.save_collateral(&test_collateral).await.expect("Failed to save collateral");
        
        // Find by customer
        let result = repo.find_collaterals_by_customer(test_collateral.customer_id).await;
        assert!(result.is_ok());
        
        let collaterals = result.unwrap();
        assert!(!collaterals.is_empty());
        let found_collateral = &collaterals[0];
        assert_eq!(found_collateral.customer_id, test_collateral.customer_id);
    }

    #[tokio::test]
    async fn test_update_collateral_status() {
        let pool = setup_test_db().await;
        let repo = CollateralRepositoryImpl::new(pool);
        let test_collateral = create_test_collateral();

        // Save collateral
        repo.save_collateral(&test_collateral).await.expect("Failed to save collateral");
        
        // Update status
        let result = repo.update_collateral_status(
            test_collateral.collateral_id, 
            "Inactive".to_string(), 
            test_collateral.updated_by
        ).await;
        assert!(result.is_ok());
        
        // Verify update
        let found = repo.find_collateral_by_id(test_collateral.collateral_id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().status.as_str(), "Inactive");
    }

    #[tokio::test]
    async fn test_find_valuations_due() {
        let pool = setup_test_db().await;
        let repo = CollateralRepositoryImpl::new(pool);
        let mut test_collateral = create_test_collateral();
        
        // Set valuation due to yesterday
        test_collateral.next_valuation_due = Some(Utc::now().date_naive() - chrono::Duration::days(1));
        test_collateral.status = HeaplessString::try_from("Active").unwrap();

        // Save collateral
        repo.save_collateral(&test_collateral).await.expect("Failed to save collateral");
        
        // Find valuations due
        let result = repo.find_valuations_due(Utc::now().date_naive()).await;
        assert!(result.is_ok());
        
        let collaterals = result.unwrap();
        let found = collaterals.iter().find(|c| c.collateral_id == test_collateral.collateral_id);
        assert!(found.is_some());
    }
}
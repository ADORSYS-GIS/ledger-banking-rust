use async_trait::async_trait;
use banking_api::BankingResult;
use banking_db::models::{CustomerModel, CustomerPortfolioModel, CustomerDocumentModel, CustomerAuditModel};
use banking_db::repository::CustomerRepository;
use sqlx::PgPool;
use uuid::Uuid;
use std::str::FromStr;

pub struct PostgresCustomerRepository {
    pool: PgPool,
}

impl PostgresCustomerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CustomerRepository for PostgresCustomerRepository {
    async fn create(&self, customer: CustomerModel) -> BankingResult<CustomerModel> {
        let row = sqlx::query!(
            r#"
            INSERT INTO customers (
                customer_id, customer_type, full_name, id_type, id_number,
                risk_rating, status, created_at, last_updated_at, updated_by
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
            customer.customer_id,
            customer.customer_type,
            customer.full_name.as_str(),
            customer.id_type,
            customer.id_number.as_str(),
            customer.risk_rating,
            customer.status,
            customer.created_at,
            customer.last_updated_at,
            customer.updated_by.as_str()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(CustomerModel {
            customer_id: row.customer_id,
            customer_type: row.customer_type,
            full_name: heapless::String::try_from(row.full_name.as_str()).unwrap_or_default(),
            id_type: row.id_type,
            id_number: heapless::String::try_from(row.id_number.as_str()).unwrap_or_default(),
            risk_rating: row.risk_rating,
            status: row.status,
            created_at: row.created_at,
            last_updated_at: row.last_updated_at,
            updated_by: heapless::String::try_from(row.updated_by.as_str()).unwrap_or_default(),
        })
    }

    async fn update(&self, customer: CustomerModel) -> BankingResult<CustomerModel> {
        let row = sqlx::query!(
            r#"
            UPDATE customers 
            SET customer_type = $2, full_name = $3, id_type = $4, id_number = $5,
                risk_rating = $6, status = $7, last_updated_at = $8, updated_by = $9
            WHERE customer_id = $1
            RETURNING *
            "#,
            customer.customer_id,
            customer.customer_type,
            customer.full_name.as_str(),
            customer.id_type,
            customer.id_number.as_str(),
            customer.risk_rating,
            customer.status,
            customer.last_updated_at,
            customer.updated_by.as_str()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(CustomerModel {
            customer_id: row.customer_id,
            customer_type: row.customer_type,
            full_name: heapless::String::try_from(row.full_name.as_str()).unwrap_or_default(),
            id_type: row.id_type,
            id_number: heapless::String::try_from(row.id_number.as_str()).unwrap_or_default(),
            risk_rating: row.risk_rating,
            status: row.status,
            created_at: row.created_at,
            last_updated_at: row.last_updated_at,
            updated_by: heapless::String::try_from(row.updated_by.as_str()).unwrap_or_default(),
        })
    }

    async fn find_by_id(&self, customer_id: Uuid) -> BankingResult<Option<CustomerModel>> {
        let row = sqlx::query!(
            "SELECT * FROM customers WHERE customer_id = $1",
            customer_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| CustomerModel {
            customer_id: row.customer_id,
            customer_type: row.customer_type,
            full_name: heapless::String::try_from(row.full_name.as_str()).unwrap_or_default(),
            id_type: row.id_type,
            id_number: heapless::String::try_from(row.id_number.as_str()).unwrap_or_default(),
            risk_rating: row.risk_rating,
            status: row.status,
            created_at: row.created_at,
            last_updated_at: row.last_updated_at,
            updated_by: heapless::String::try_from(row.updated_by.as_str()).unwrap_or_default(),
        }))
    }

    async fn find_by_identity(&self, id_type: &str, id_number: &str) -> BankingResult<Option<CustomerModel>> {
        let row = sqlx::query!(
            "SELECT * FROM customers WHERE id_type = $1 AND id_number = $2",
            id_type,
            id_number
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| CustomerModel {
            customer_id: row.customer_id,
            customer_type: row.customer_type,
            full_name: heapless::String::try_from(row.full_name.as_str()).unwrap_or_default(),
            id_type: row.id_type,
            id_number: heapless::String::try_from(row.id_number.as_str()).unwrap_or_default(),
            risk_rating: row.risk_rating,
            status: row.status,
            created_at: row.created_at,
            last_updated_at: row.last_updated_at,
            updated_by: heapless::String::try_from(row.updated_by.as_str()).unwrap_or_default(),
        }))
    }

    async fn find_by_risk_rating(&self, risk_rating: &str) -> BankingResult<Vec<CustomerModel>> {
        let rows = sqlx::query!(
            "SELECT * FROM customers WHERE risk_rating = $1 ORDER BY created_at DESC",
            risk_rating
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| CustomerModel {
                customer_id: row.customer_id,
                customer_type: row.customer_type,
                full_name: heapless::String::try_from(row.full_name.as_str()).unwrap_or_default(),
                id_type: row.id_type,
                id_number: heapless::String::try_from(row.id_number.as_str()).unwrap_or_default(),
                risk_rating: row.risk_rating,
                status: row.status,
                created_at: row.created_at,
                last_updated_at: row.last_updated_at,
                updated_by: heapless::String::try_from(row.updated_by.as_str()).unwrap_or_default(),
            })
            .collect())
    }

    async fn find_requiring_review(&self) -> BankingResult<Vec<CustomerModel>> {
        let rows = sqlx::query!(
            r#"
            SELECT DISTINCT c.* FROM customers c
            LEFT JOIN kyc_records k ON c.customer_id = k.customer_id
            WHERE c.status = 'PendingVerification' 
               OR k.status = 'RequiresReview'
               OR (c.risk_rating = 'High' AND k.last_review_date < NOW() - INTERVAL '90 days')
            ORDER BY c.created_at ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| CustomerModel {
                customer_id: row.customer_id,
                customer_type: row.customer_type,
                full_name: heapless::String::try_from(row.full_name.as_str()).unwrap_or_default(),
                id_type: row.id_type,
                id_number: heapless::String::try_from(row.id_number.as_str()).unwrap_or_default(),
                risk_rating: row.risk_rating,
                status: row.status,
                created_at: row.created_at,
                last_updated_at: row.last_updated_at,
                updated_by: heapless::String::try_from(row.updated_by.as_str()).unwrap_or_default(),
            })
            .collect())
    }

    async fn get_portfolio(&self, customer_id: Uuid) -> BankingResult<Option<CustomerPortfolioModel>> {
        let row = sqlx::query!(
            r#"
            SELECT 
                c.customer_id,
                COUNT(DISTINCT a.account_id) as total_accounts,
                COALESCE(SUM(a.current_balance), 0) as total_balance,
                MAX(t.transaction_date) as last_activity_date,
                COALESCE(cr.risk_score, 0) as risk_score,
                COALESCE(k.status, 'Unknown') as kyc_status,
                CASE WHEN s.last_screening_date IS NOT NULL THEN true ELSE false END as sanctions_checked,
                s.last_screening_date
            FROM customers c
            LEFT JOIN account_ownership ao ON c.customer_id = ao.customer_id
            LEFT JOIN accounts a ON ao.account_id = a.account_id
            LEFT JOIN transactions t ON a.account_id = t.account_id
            LEFT JOIN compliance_risk_scores cr ON c.customer_id = cr.customer_id
            LEFT JOIN kyc_records k ON c.customer_id = k.customer_id
            LEFT JOIN sanctions_screening s ON c.customer_id = s.customer_id
            WHERE c.customer_id = $1
            GROUP BY c.customer_id, cr.risk_score, k.status, s.last_screening_date
            "#,
            customer_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| CustomerPortfolioModel {
            customer_id: row.customer_id,
            total_accounts: row.total_accounts.unwrap_or(0),
            total_balance: row.total_balance.map(|bd| rust_decimal::Decimal::from_str(&bd.to_string()).unwrap_or(rust_decimal::Decimal::ZERO)).unwrap_or(rust_decimal::Decimal::ZERO),
            last_activity_date: row.last_activity_date,
            risk_score: row.risk_score.and_then(|bd| rust_decimal::Decimal::from_str(&bd.to_string()).ok()),
            kyc_status: row.kyc_status.unwrap_or("Unknown".to_string()),
            sanctions_checked: row.sanctions_checked.unwrap_or(false),
            last_screening_date: row.last_screening_date,
        }))
    }

    async fn update_risk_rating(&self, customer_id: Uuid, risk_rating: &str, authorized_by: &str) -> BankingResult<()> {
        let mut tx = self.pool.begin().await?;

        // Get current risk rating for audit trail
        let current = sqlx::query!(
            "SELECT risk_rating FROM customers WHERE customer_id = $1",
            customer_id
        )
        .fetch_optional(&mut *tx)
        .await?;

        // Update the customer record
        sqlx::query!(
            r#"
            UPDATE customers 
            SET risk_rating = $2, last_updated_at = NOW(), updated_by = $3
            WHERE customer_id = $1
            "#,
            customer_id,
            risk_rating,
            authorized_by
        )
        .execute(&mut *tx)
        .await?;

        // Add audit trail entry
        if let Some(current_record) = current {
            sqlx::query!(
                r#"
                INSERT INTO customer_audit_trail (
                    audit_id, customer_id, field_name, old_value, new_value,
                    changed_at, changed_by, reason
                ) VALUES ($1, $2, 'risk_rating', $3, $4, NOW(), $5, 'Risk rating update')
                "#,
                Uuid::new_v4(),
                customer_id,
                current_record.risk_rating,
                risk_rating,
                authorized_by
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn update_status(&self, customer_id: Uuid, status: &str, reason: &str) -> BankingResult<()> {
        let mut tx = self.pool.begin().await?;

        // Get current status for audit trail
        let current = sqlx::query!(
            "SELECT status FROM customers WHERE customer_id = $1",
            customer_id
        )
        .fetch_optional(&mut *tx)
        .await?;

        // Update the customer record
        sqlx::query!(
            r#"
            UPDATE customers 
            SET status = $2, last_updated_at = NOW()
            WHERE customer_id = $1
            "#,
            customer_id,
            status
        )
        .execute(&mut *tx)
        .await?;

        // Add audit trail entry
        if let Some(current_record) = current {
            sqlx::query!(
                r#"
                INSERT INTO customer_audit_trail (
                    audit_id, customer_id, field_name, old_value, new_value,
                    changed_at, changed_by, reason
                ) VALUES ($1, $2, 'status', $3, $4, NOW(), 'SYSTEM', $5)
                "#,
                Uuid::new_v4(),
                customer_id,
                current_record.status,
                status,
                reason
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn add_document(&self, document: CustomerDocumentModel) -> BankingResult<CustomerDocumentModel> {
        let row = sqlx::query!(
            r#"
            INSERT INTO customer_documents (
                document_id, customer_id, document_type, document_path, status,
                uploaded_at, uploaded_by, verified_at, verified_by
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
            document.document_id,
            document.customer_id,
            document.document_type,
            document.document_path,
            document.status,
            document.uploaded_at,
            document.uploaded_by,
            document.verified_at,
            document.verified_by
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(CustomerDocumentModel {
            document_id: row.document_id,
            customer_id: row.customer_id,
            document_type: row.document_type,
            document_path: row.document_path,
            status: row.status,
            uploaded_at: row.uploaded_at,
            uploaded_by: row.uploaded_by,
            verified_at: row.verified_at,
            verified_by: row.verified_by,
        })
    }

    async fn get_documents(&self, customer_id: Uuid) -> BankingResult<Vec<CustomerDocumentModel>> {
        let rows = sqlx::query!(
            "SELECT * FROM customer_documents WHERE customer_id = $1 ORDER BY uploaded_at DESC",
            customer_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| CustomerDocumentModel {
                document_id: row.document_id,
                customer_id: row.customer_id,
                document_type: row.document_type,
                document_path: row.document_path,
                status: row.status,
                uploaded_at: row.uploaded_at,
                uploaded_by: row.uploaded_by,
                verified_at: row.verified_at,
                verified_by: row.verified_by,
            })
            .collect())
    }

    async fn add_audit_entry(&self, audit: CustomerAuditModel) -> BankingResult<CustomerAuditModel> {
        let row = sqlx::query!(
            r#"
            INSERT INTO customer_audit_trail (
                audit_id, customer_id, field_name, old_value, new_value,
                changed_at, changed_by, reason
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
            audit.audit_id,
            audit.customer_id,
            audit.field_name,
            audit.old_value,
            audit.new_value,
            audit.changed_at,
            audit.changed_by,
            audit.reason
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(CustomerAuditModel {
            audit_id: row.audit_id,
            customer_id: row.customer_id,
            field_name: row.field_name,
            old_value: row.old_value,
            new_value: row.new_value,
            changed_at: row.changed_at,
            changed_by: row.changed_by,
            reason: row.reason,
        })
    }

    async fn get_audit_trail(&self, customer_id: Uuid) -> BankingResult<Vec<CustomerAuditModel>> {
        let rows = sqlx::query!(
            "SELECT * FROM customer_audit_trail WHERE customer_id = $1 ORDER BY changed_at DESC",
            customer_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| CustomerAuditModel {
                audit_id: row.audit_id,
                customer_id: row.customer_id,
                field_name: row.field_name,
                old_value: row.old_value,
                new_value: row.new_value,
                changed_at: row.changed_at,
                changed_by: row.changed_by,
                reason: row.reason,
            })
            .collect())
    }

    async fn delete(&self, customer_id: Uuid, deleted_by: &str) -> BankingResult<()> {
        let mut tx = self.pool.begin().await?;

        // Soft delete by updating status
        sqlx::query!(
            r#"
            UPDATE customers 
            SET status = 'Deleted', last_updated_at = NOW(), updated_by = $2
            WHERE customer_id = $1
            "#,
            customer_id,
            deleted_by
        )
        .execute(&mut *tx)
        .await?;

        // Add audit trail entry
        sqlx::query!(
            r#"
            INSERT INTO customer_audit_trail (
                audit_id, customer_id, field_name, old_value, new_value,
                changed_at, changed_by, reason
            ) VALUES ($1, $2, 'status', 'Active', 'Deleted', NOW(), $3, 'Customer deletion')
            "#,
            Uuid::new_v4(),
            customer_id,
            deleted_by
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn exists(&self, customer_id: Uuid) -> BankingResult<bool> {
        let row = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM customers WHERE customer_id = $1 AND status != 'Deleted') as exists",
            customer_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.exists.unwrap_or(false))
    }

    async fn list(&self, offset: i64, limit: i64) -> BankingResult<Vec<CustomerModel>> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM customers 
            WHERE status != 'Deleted'
            ORDER BY created_at DESC 
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| CustomerModel {
                customer_id: row.customer_id,
                customer_type: row.customer_type,
                full_name: heapless::String::try_from(row.full_name.as_str()).unwrap_or_default(),
                id_type: row.id_type,
                id_number: heapless::String::try_from(row.id_number.as_str()).unwrap_or_default(),
                risk_rating: row.risk_rating,
                status: row.status,
                created_at: row.created_at,
                last_updated_at: row.last_updated_at,
                updated_by: heapless::String::try_from(row.updated_by.as_str()).unwrap_or_default(),
            })
            .collect())
    }

    async fn count(&self) -> BankingResult<i64> {
        let row = sqlx::query!(
            "SELECT COUNT(*) as count FROM customers WHERE status != 'Deleted'"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.count.unwrap_or(0))
    }
}
use async_trait::async_trait;
use banking_api::{BankingResult, BankingError};
use banking_db::models::{
    CustomerModel, CustomerPortfolioModel, CustomerDocumentModel, CustomerAuditModel
};
use banking_db::repository::CustomerRepository;
use sqlx::{PgPool, Row, postgres::PgRow};
use uuid::Uuid;
use heapless::String as HeaplessString;

/// PostgreSQL implementation of CustomerRepository
pub struct PostgresCustomerRepository {
    pool: PgPool,
}

impl PostgresCustomerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

/// Manual row extraction for CustomerModel
impl TryFromRow<PgRow> for CustomerModel {
    fn try_from_row(row: &PgRow) -> BankingResult<Self> {
        Ok(CustomerModel {
            id: row.get("id"),
            customer_type: row.get::<String, _>("customer_type").parse().map_err(|_| 
                BankingError::ValidationError {
                    field: "customer_type".to_string(),
                    message: "Invalid customer type".to_string(),
                }
            )?,
            full_name: HeaplessString::try_from(
                row.get::<String, _>("full_name").as_str()
            ).map_err(|_| BankingError::ValidationError {
                field: "full_name".to_string(),
                message: "Full name too long".to_string(),
            })?,
            id_type: row.get::<String, _>("id_type").parse().map_err(|_| 
                BankingError::ValidationError {
                    field: "id_type".to_string(),
                    message: "Invalid identity type".to_string(),
                }
            )?,
            id_number: HeaplessString::try_from(
                row.get::<String, _>("id_number").as_str()
            ).map_err(|_| BankingError::ValidationError {
                field: "id_number".to_string(),
                message: "ID number too long".to_string(),
            })?,
            risk_rating: row.get::<String, _>("risk_rating").parse().map_err(|_| 
                BankingError::ValidationError {
                    field: "risk_rating".to_string(),
                    message: "Invalid risk rating".to_string(),
                }
            )?,
            status: row.get::<String, _>("status").parse().map_err(|_| 
                BankingError::ValidationError {
                    field: "status".to_string(),
                    message: "Invalid customer status".to_string(),
                }
            )?,
            created_at: row.get("created_at"),
            last_updated_at: row.get("last_updated_at"),
            updated_by: row.get("updated_by"),
        })
    }
}

/// Manual row extraction for CustomerDocumentModel
impl TryFromRow<PgRow> for CustomerDocumentModel {
    fn try_from_row(row: &PgRow) -> BankingResult<Self> {
        Ok(CustomerDocumentModel {
            id: row.get("id"),
            customer_id: row.get("customer_id"),
            document_type: HeaplessString::try_from(
                row.get::<String, _>("document_type").as_str()
            ).map_err(|_| BankingError::ValidationError {
                field: "document_type".to_string(),
                message: "Document type too long".to_string(),
            })?,
            document_path: row.get::<Option<String>, _>("document_path")
                .map(|s| HeaplessString::try_from(s.as_str()))
                .transpose()
                .map_err(|_| BankingError::ValidationError {
                    field: "document_path".to_string(),
                    message: "Document path too long".to_string(),
                })?,
            status: row.get::<String, _>("status").parse().map_err(|_| 
                BankingError::ValidationError {
                    field: "status".to_string(),
                    message: "Invalid document status".to_string(),
                }
            )?,
            uploaded_at: row.get("uploaded_at"),
            uploaded_by: row.get("uploaded_by"),
            verified_at: row.get("verified_at"),
            verified_by: row.get("verified_by"),
        })
    }
}

/// Manual row extraction for CustomerAuditModel
impl TryFromRow<PgRow> for CustomerAuditModel {
    fn try_from_row(row: &PgRow) -> BankingResult<Self> {
        Ok(CustomerAuditModel {
            id: row.get("id"),
            customer_id: row.get("customer_id"),
            field_name: HeaplessString::try_from(
                row.get::<String, _>("field_name").as_str()
            ).map_err(|_| BankingError::ValidationError {
                field: "field_name".to_string(),
                message: "Field name too long".to_string(),
            })?,
            old_value: row.get::<Option<String>, _>("old_value")
                .map(|s| HeaplessString::try_from(s.as_str()))
                .transpose()
                .map_err(|_| BankingError::ValidationError {
                    field: "old_value".to_string(),
                    message: "Old value too long".to_string(),
                })?,
            new_value: row.get::<Option<String>, _>("new_value")
                .map(|s| HeaplessString::try_from(s.as_str()))
                .transpose()
                .map_err(|_| BankingError::ValidationError {
                    field: "new_value".to_string(),
                    message: "New value too long".to_string(),
                })?,
            changed_at: row.get("changed_at"),
            changed_by: row.get("changed_by"),
            reason: row.get::<Option<String>, _>("reason")
                .map(|s| HeaplessString::try_from(s.as_str()))
                .transpose()
                .map_err(|_| BankingError::ValidationError {
                    field: "reason".to_string(),
                    message: "Reason too long".to_string(),
                })?,
        })
    }
}

/// Custom trait for row extraction
trait TryFromRow<R> {
    fn try_from_row(row: &R) -> BankingResult<Self>
    where
        Self: Sized;
}

#[async_trait]
impl CustomerRepository for PostgresCustomerRepository {
    async fn create(&self, customer: CustomerModel) -> BankingResult<CustomerModel> {
        let result = sqlx::query(
            r#"
            INSERT INTO customers (
                id, customer_type, full_name, id_type, id_number,
                risk_rating, status, created_at, last_updated_at, updated_by
            )
            VALUES (
                $1, $2::customer_type, $3, $4::identity_type, $5,
                $6::risk_rating, $7::customer_status, $8, $9, $10
            )
            RETURNING id, customer_type::text as customer_type, full_name,
                     id_type::text as id_type, id_number, risk_rating::text as risk_rating,
                     status::text as status, created_at, last_updated_at, updated_by
            "#
        )
        .bind(customer.id)
        .bind(customer.customer_type.to_string())
        .bind(customer.full_name.as_str())
        .bind(customer.id_type.to_string())
        .bind(customer.id_number.as_str())
        .bind(customer.risk_rating.to_string())
        .bind(customer.status.to_string())
        .bind(customer.created_at)
        .bind(customer.last_updated_at)
        .bind(customer.updated_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to create customer: {e}")))?
        ;

        CustomerModel::try_from_row(&result)
    }

    async fn update(&self, customer: CustomerModel) -> BankingResult<CustomerModel> {
        let result = sqlx::query(
            r#"
            UPDATE customers 
            SET customer_type = $2::customer_type, full_name = $3, id_type = $4::identity_type,
                id_number = $5, risk_rating = $6::risk_rating, status = $7::customer_status,
                last_updated_at = $8, updated_by = $9
            WHERE id = $1
            RETURNING id, customer_type::text as customer_type, full_name,
                     id_type::text as id_type, id_number, risk_rating::text as risk_rating,
                     status::text as status, created_at, last_updated_at, updated_by
            "#
        )
        .bind(customer.id)
        .bind(customer.customer_type.to_string())
        .bind(customer.full_name.as_str())
        .bind(customer.id_type.to_string())
        .bind(customer.id_number.as_str())
        .bind(customer.risk_rating.to_string())
        .bind(customer.status.to_string())
        .bind(customer.last_updated_at)
        .bind(customer.updated_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to update customer: {e}")))?
        ;

        CustomerModel::try_from_row(&result)
    }

    async fn find_by_id(&self, customer_id: Uuid) -> BankingResult<Option<CustomerModel>> {
        let result = sqlx::query(
            r#"
            SELECT id, customer_type::text as customer_type, full_name,
                   id_type::text as id_type, id_number, risk_rating::text as risk_rating,
                   status::text as status, created_at, last_updated_at, updated_by
            FROM customers 
            WHERE id = $1
            "#
        )
        .bind(customer_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to find customer by ID: {e}")))?
        ;

        match result {
            Some(row) => Ok(Some(CustomerModel::try_from_row(&row)?)),
            None => Ok(None),
        }
    }

    async fn find_by_identity(&self, id_type: &str, id_number: &str) -> BankingResult<Option<CustomerModel>> {
        let result = sqlx::query(
            r#"
            SELECT id, customer_type::text as customer_type, full_name,
                   id_type::text as id_type, id_number, risk_rating::text as risk_rating,
                   status::text as status, created_at, last_updated_at, updated_by
            FROM customers 
            WHERE id_type = $1::identity_type AND id_number = $2
            "#
        )
        .bind(id_type)
        .bind(id_number)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to find customer by identity: {e}")))?
        ;

        match result {
            Some(row) => Ok(Some(CustomerModel::try_from_row(&row)?)),
            None => Ok(None),
        }
    }

    async fn find_by_risk_rating(&self, risk_rating: &str) -> BankingResult<Vec<CustomerModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, customer_type::text as customer_type, full_name,
                   id_type::text as id_type, id_number, risk_rating::text as risk_rating,
                   status::text as status, created_at, last_updated_at, updated_by
            FROM customers 
            WHERE risk_rating = $1::risk_rating
            ORDER BY full_name
            "#
        )
        .bind(risk_rating)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to find customers by risk rating: {e}")))?
        ;

        let mut customers = Vec::new();
        for row in rows {
            customers.push(CustomerModel::try_from_row(&row)?);
        }
        Ok(customers)
    }

    async fn find_requiring_review(&self) -> BankingResult<Vec<CustomerModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, customer_type::text as customer_type, full_name,
                   id_type::text as id_type, id_number, risk_rating::text as risk_rating,
                   status::text as status, created_at, last_updated_at, updated_by
            FROM customers 
            WHERE status = 'PendingVerification' OR risk_rating = 'High' OR risk_rating = 'Blacklisted'
               OR last_updated_at < NOW() - INTERVAL '1 year'
            ORDER BY risk_rating DESC, last_updated_at ASC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to find customers requiring review: {e}")))?
        ;

        let mut customers = Vec::new();
        for row in rows {
            customers.push(CustomerModel::try_from_row(&row)?);
        }
        Ok(customers)
    }

    async fn get_portfolio(&self, customer_id: Uuid) -> BankingResult<Option<CustomerPortfolioModel>> {
        let result = sqlx::query(
            r#"
            SELECT 
                $1::uuid as customer_id,
                0::bigint as total_accounts,
                0::decimal as total_balance,
                0::decimal as total_loan_outstanding,
                NULL::timestamp as last_activity_date,
                NULL::decimal as risk_score,
                'NotStarted'::text as kyc_status,
                false as sanctions_checked,
                NULL::timestamp as last_screening_date
            "#
        )
        .bind(customer_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to get customer portfolio: {e}")))?
        ;

        match result {
            Some(row) => {
                Ok(Some(CustomerPortfolioModel {
                    customer_id: row.get("customer_id"),
                    total_accounts: row.get("total_accounts"),
                    total_balance: row.get("total_balance"),
                    total_loan_outstanding: row.get("total_loan_outstanding"),
                    last_activity_date: row.get("last_activity_date"),
                    risk_score: row.get("risk_score"),
                    kyc_status: row.get::<String, _>("kyc_status").parse().map_err(|_| 
                        BankingError::ValidationError {
                            field: "kyc_status".to_string(),
                            message: "Invalid KYC status".to_string(),
                        }
                    )?,
                    sanctions_checked: row.get("sanctions_checked"),
                    last_screening_date: row.get("last_screening_date"),
                }))
            },
            None => Ok(None),
        }
    }

    async fn update_risk_rating(&self, customer_id: Uuid, risk_rating: &str, authorized_by: Uuid) -> BankingResult<()> {
        let mut tx = self.pool.begin().await.map_err(|e| BankingError::Internal(format!("Failed to start transaction: {e}")))?
        ;

        // Get current risk rating for audit
        let current_customer = sqlx::query(
            "SELECT risk_rating::text as risk_rating FROM customers WHERE id = $1"
        )
        .bind(customer_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to get current risk rating: {e}")))?
        ;

        let old_risk_rating: String = current_customer.get("risk_rating");

        // Update risk rating
        sqlx::query(
            "UPDATE customers SET risk_rating = $1::risk_rating, last_updated_at = NOW(), updated_by = $2 WHERE id = $3"
        )
        .bind(risk_rating)
        .bind(authorized_by)
        .bind(customer_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to update risk rating: {e}")))?
        ;

        // Add audit trail entry
        sqlx::query(
            r#"
            INSERT INTO customer_audit_trail (
                id, customer_id, field_name, old_value, new_value, changed_at, changed_by, reason
            )
            VALUES ($1, $2, $3, $4, $5, NOW(), $6, $7)
            "#
        )
        .bind(Uuid::new_v4())
        .bind(customer_id)
        .bind("risk_rating")
        .bind(&old_risk_rating)
        .bind(risk_rating)
        .bind(authorized_by)
        .bind("Risk rating update")
        .execute(&mut *tx)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to add audit trail: {e}")))?
        ;

        tx.commit().await.map_err(|e| BankingError::Internal(format!("Failed to commit transaction: {e}")))?;
        Ok(())
    }

    async fn update_status(&self, customer_id: Uuid, status: &str, reason: &str) -> BankingResult<()> {
        let mut tx = self.pool.begin().await.map_err(|e| BankingError::Internal(format!("Failed to start transaction: {e}")))?
        ;

        // Get current status for audit
        let current_customer = sqlx::query(
            "SELECT status::text as status FROM customers WHERE id = $1"
        )
        .bind(customer_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to get current status: {e}")))?
        ;

        let old_status: String = current_customer.get("status");

        // Update status
        sqlx::query(
            "UPDATE customers SET status = $1::customer_status, last_updated_at = NOW() WHERE id = $2"
        )
        .bind(status)
        .bind(customer_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to update status: {e}")))?
        ;

        // Add audit trail entry
        sqlx::query(
            r#"
            INSERT INTO customer_audit_trail (
                id, customer_id, field_name, old_value, new_value, changed_at, changed_by, reason
            )
            VALUES ($1, $2, $3, $4, $5, NOW(), $6, $7)
            "#
        )
        .bind(Uuid::new_v4())
        .bind(customer_id)
        .bind("status")
        .bind(&old_status)
        .bind(status)
        .bind(Uuid::new_v4()) // System user for status changes
        .bind(reason)
        .execute(&mut *tx)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to add audit trail: {e}")))?
        ;

        tx.commit().await.map_err(|e| BankingError::Internal(format!("Failed to commit transaction: {e}")))?;
        Ok(())
    }

    async fn add_document(&self, document: CustomerDocumentModel) -> BankingResult<CustomerDocumentModel> {
        let result = sqlx::query(
            r#"
            INSERT INTO customer_documents (
                id, customer_id, document_type, document_path, status,
                uploaded_at, uploaded_by, verified_at, verified_by
            )
            VALUES ($1, $2, $3, $4, $5::document_status, $6, $7, $8, $9)
            RETURNING id, customer_id, document_type, document_path,
                     status::text as status, uploaded_at, uploaded_by, verified_at, verified_by
            "#
        )
        .bind(document.id)
        .bind(document.customer_id)
        .bind(document.document_type.as_str())
        .bind(document.document_path.as_ref().map(|s| s.as_str()))
        .bind(document.status.to_string())
        .bind(document.uploaded_at)
        .bind(document.uploaded_by)
        .bind(document.verified_at)
        .bind(document.verified_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to add document: {e}")))?
        ;

        CustomerDocumentModel::try_from_row(&result)
    }

    async fn get_documents(&self, customer_id: Uuid) -> BankingResult<Vec<CustomerDocumentModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, customer_id, document_type, document_path,
                   status::text as status, uploaded_at, uploaded_by, verified_at, verified_by
            FROM customer_documents 
            WHERE customer_id = $1
            ORDER BY uploaded_at DESC
            "#
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to get documents: {e}")))?
        ;

        let mut documents = Vec::new();
        for row in rows {
            documents.push(CustomerDocumentModel::try_from_row(&row)?);
        }
        Ok(documents)
    }

    async fn add_audit_entry(&self, audit: CustomerAuditModel) -> BankingResult<CustomerAuditModel> {
        let result = sqlx::query(
            r#"
            INSERT INTO customer_audit_trail (
                id, customer_id, field_name, old_value, new_value,
                changed_at, changed_by, reason
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, customer_id, field_name, old_value, new_value,
                     changed_at, changed_by, reason
            "#
        )
        .bind(audit.id)
        .bind(audit.customer_id)
        .bind(audit.field_name.as_str())
        .bind(audit.old_value.as_ref().map(|s| s.as_str()))
        .bind(audit.new_value.as_ref().map(|s| s.as_str()))
        .bind(audit.changed_at)
        .bind(audit.changed_by)
        .bind(audit.reason.as_ref().map(|s| s.as_str()))
        .fetch_one(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to add audit entry: {e}")))?
        ;

        CustomerAuditModel::try_from_row(&result)
    }

    async fn get_audit_trail(&self, customer_id: Uuid) -> BankingResult<Vec<CustomerAuditModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, customer_id, field_name, old_value, new_value,
                   changed_at, changed_by, reason
            FROM customer_audit_trail 
            WHERE customer_id = $1
            ORDER BY changed_at DESC
            "#
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to get audit trail: {e}")))?
        ;

        let mut audit_entries = Vec::new();
        for row in rows {
            audit_entries.push(CustomerAuditModel::try_from_row(&row)?);
        }
        Ok(audit_entries)
    }

    async fn delete(&self, customer_id: Uuid, deleted_by: Uuid) -> BankingResult<()> {
        let mut tx = self.pool.begin().await.map_err(|e| BankingError::Internal(format!("Failed to start transaction: {e}")))?
        ;

        // Add audit trail entry for deletion
        sqlx::query(
            r#"
            INSERT INTO customer_audit_trail (
                id, customer_id, field_name, old_value, new_value, changed_at, changed_by, reason
            )
            VALUES ($1, $2, $3, $4, $5, NOW(), $6, $7)
            "#
        )
        .bind(Uuid::new_v4())
        .bind(customer_id)
        .bind("status")
        .bind("Active")
        .bind("Deleted")
        .bind(deleted_by)
        .bind("Customer deletion")
        .execute(&mut *tx)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to add deletion audit: {e}")))?
        ;

        // Soft delete by updating status
        sqlx::query(
            "UPDATE customers SET status = 'Deceased'::customer_status, last_updated_at = NOW(), updated_by = $1 WHERE id = $2"
        )
        .bind(deleted_by)
        .bind(customer_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to delete customer: {e}")))?
        ;

        tx.commit().await.map_err(|e| BankingError::Internal(format!("Failed to commit transaction: {e}")))?;
        Ok(())
    }

    async fn exists(&self, customer_id: Uuid) -> BankingResult<bool> {
        let result = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM customers WHERE id = $1) as exists"
        )
        .bind(customer_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to check customer existence: {e}")))?
        ;

        Ok(result.get("exists"))
    }

    async fn list(&self, offset: i64, limit: i64) -> BankingResult<Vec<CustomerModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, customer_type::text as customer_type, full_name,
                   id_type::text as id_type, id_number, risk_rating::text as risk_rating,
                   status::text as status, created_at, last_updated_at, updated_by
            FROM customers 
            ORDER BY full_name
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to list customers: {e}")))?
        ;

        let mut customers = Vec::new();
        for row in rows {
            customers.push(CustomerModel::try_from_row(&row)?);
        }
        Ok(customers)
    }

    async fn count(&self) -> BankingResult<i64> {
        let result = sqlx::query(
            "SELECT COUNT(*) as count FROM customers"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to count customers: {e}")))?
        ;

        Ok(result.get("count"))
    }
}
use async_trait::async_trait;
use banking_db::{
    models::audit::AuditLogModel,
    repository::{
        batch_repository::BatchRepository,
    },
};
use chrono::{DateTime, Utc};
use sqlx::Postgres;
use std::error::Error;
use uuid::Uuid;

use crate::repository::{audit::audit_log_repository::repo_impl::AuditLogRepositoryImpl, executor::Executor};

type AuditLogTuple = (Uuid, DateTime<Utc>, Uuid);

#[async_trait]
impl BatchRepository<Postgres, AuditLogModel> for AuditLogRepositoryImpl {
    async fn create_batch(
        &self,
        items: Vec<AuditLogModel>,
        _audit_log_id: Uuid,
    ) -> Result<Vec<AuditLogModel>, Box<dyn Error + Send + Sync>> {
        if items.is_empty() {
            return Ok(Vec::new());
        }

        let tuples: Vec<AuditLogTuple> = items
            .iter()
            .map(|item| (item.id, item.updated_at, item.updated_by_person_id))
            .collect();

        self.execute_audit_log_insert(tuples).await?;

        Ok(items)
    }

    async fn load_batch(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<Option<AuditLogModel>>, Box<dyn Error + Send + Sync>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let query = r#"SELECT * FROM audit_log WHERE id = ANY($1)"#;
        let rows = match &self.executor {
            Executor::Pool(pool) => {
                sqlx::query_as(query)
                    .bind(ids)
                    .fetch_all(&**pool)
                    .await?
            }
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query_as(query)
                    .bind(ids)
                    .fetch_all(&mut **tx)
                    .await?
            }
        };
        let mut item_map = std::collections::HashMap::new();
        for row in rows {
            let item: AuditLogModel = row;
            item_map.insert(item.id, item);
        }
        let mut result = Vec::with_capacity(ids.len());
        for id in ids {
            result.push(item_map.remove(id));
        }
        Ok(result)
    }

    async fn update_batch(
        &self,
        _items: Vec<AuditLogModel>,
        _audit_log_id: Uuid,
    ) -> Result<Vec<AuditLogModel>, Box<dyn Error + Send + Sync>> {
        unimplemented!("Audit logs are immutable and cannot be updated in batch.")
    }

    async fn delete_batch(
        &self,
        ids: &[Uuid],
    ) -> Result<usize, Box<dyn Error + Send + Sync>> {
        if ids.is_empty() {
            return Ok(0);
        }

        let query = "DELETE FROM audit_log WHERE id = ANY($1)";
        let result = match &self.executor {
            Executor::Pool(pool) => {
                sqlx::query(query).bind(ids).execute(&**pool).await?
            }
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query).bind(ids).execute(&mut **tx).await?
            }
        };

        Ok(result.rows_affected() as usize)
    }
}

impl AuditLogRepositoryImpl {
    async fn execute_audit_log_insert(
        &self,
        values: Vec<AuditLogTuple>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (ids, updated_ats, updated_by_person_ids) =
            values
                .into_iter()
                .fold((Vec::new(), Vec::new(), Vec::new()), |mut acc, val| {
                    acc.0.push(val.0);
                    acc.1.push(val.1);
                    acc.2.push(val.2);
                    acc
                });

        let query = r#"
            INSERT INTO audit_log (id, updated_at, updated_by_person_id)
            SELECT * FROM UNNEST($1::uuid[], $2::timestamptz[], $3::uuid[])
        "#;

        match &self.executor {
            Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(ids)
                    .bind(updated_ats)
                    .bind(updated_by_person_ids)
                    .execute(&**pool)
                    .await?;
            }
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(ids)
                    .bind(updated_ats)
                    .bind(updated_by_person_ids)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }
}
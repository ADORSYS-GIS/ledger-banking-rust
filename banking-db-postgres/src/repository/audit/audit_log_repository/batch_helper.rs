use crate::repository::executor::Executor;
use chrono::{DateTime, Utc};
use std::error::Error;
use uuid::Uuid;

type AuditLogTuple = (Uuid, DateTime<Utc>, Uuid);

pub async fn execute_audit_log_insert(
    executor: &Executor,
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

    match executor {
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
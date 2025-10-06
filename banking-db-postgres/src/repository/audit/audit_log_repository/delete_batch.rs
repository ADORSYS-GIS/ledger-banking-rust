use crate::repository::executor::Executor;
use std::error::Error;
use uuid::Uuid;

pub async fn delete_batch(
    executor: &Executor,
    ids: &[Uuid],
) -> Result<usize, Box<dyn Error + Send + Sync>> {
    if ids.is_empty() {
        return Ok(0);
    }

    let query = "DELETE FROM audit_log WHERE id = ANY($1)";
    let result = match executor {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::audit::audit_log_repository::create_batch::create_batch;
    use crate::repository::executor::Executor;
    use banking_db::models::audit::AuditLogModel;
    use chrono::Utc;
    use sqlx::{postgres::PgPoolOptions, PgPool};
    use std::sync::Arc;
    use uuid::Uuid;

    async fn get_pool() -> PgPool {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://user:password@localhost:5432/mydb".to_string());
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    fn new_test_audit_log() -> AuditLogModel {
        AuditLogModel {
            id: Uuid::new_v4(),
            updated_at: Utc::now(),
            updated_by_person_id: Uuid::new_v4(),
        }
    }

    #[tokio::test]
    async fn test_delete_batch() {
        let pool = get_pool().await;
        let executor = Executor::Pool(Arc::new(pool.clone()));

        let log1 = new_test_audit_log();
        let log2 = new_test_audit_log();
        let items = vec![log1.clone(), log2.clone()];
        let item_ids: Vec<Uuid> = items.iter().map(|i| i.id).collect();

        create_batch(&executor, items).await.unwrap();

        let result = delete_batch(&executor, &item_ids).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);

        // Verify the logs are deleted
        let fetched_logs: Vec<AuditLogModel> =
            sqlx::query_as("SELECT * FROM audit_log WHERE id = ANY($1)")
                .bind(&item_ids)
                .fetch_all(&pool)
                .await
                .unwrap();
        assert!(fetched_logs.is_empty());
    }
}
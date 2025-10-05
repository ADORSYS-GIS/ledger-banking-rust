use banking_db::models::audit::AuditLogModel;
use std::error::Error;
use crate::repository::executor::Executor;

pub async fn create_batch(
    executor: &Executor,
    items: Vec<AuditLogModel>,
) -> Result<Vec<AuditLogModel>, Box<dyn Error + Send + Sync>> {
    if items.is_empty() {
        return Ok(Vec::new());
    }

    let tuples = items
        .iter()
        .map(|item| (item.id, item.updated_at, item.updated_by_person_id))
        .collect();

    super::batch_helper::execute_audit_log_insert(executor, tuples).await?;

    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;
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
    async fn test_create_batch() {
        let pool = get_pool().await;
        let executor = Executor::Pool(Arc::new(pool.clone()));

        let log1 = new_test_audit_log();
        let log2 = new_test_audit_log();
        let items = vec![log1.clone(), log2.clone()];
        let item_ids: Vec<Uuid> = items.iter().map(|i| i.id).collect();

        let result = create_batch(&executor, items).await;
        assert!(result.is_ok());
        let created_items = result.unwrap();
        assert_eq!(created_items.len(), 2);

        // Verify the logs exist in the database
        let fetched_logs: Vec<AuditLogModel> =
            sqlx::query_as("SELECT * FROM audit_log WHERE id = ANY($1)")
                .bind(&item_ids)
                .fetch_all(&pool)
                .await
                .unwrap();
        assert_eq!(fetched_logs.len(), 2);

        // Cleanup
        sqlx::query("DELETE FROM audit_log WHERE id = ANY($1)")
            .bind(&item_ids)
            .execute(&pool)
            .await
            .unwrap();
    }
}
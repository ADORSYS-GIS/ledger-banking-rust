use banking_db::models::audit::AuditLogModel;
use crate::repository::executor::Executor;
use std::error::Error;
use uuid::Uuid;

pub async fn load_batch(
    executor: &Executor,
    ids: &[Uuid],
) -> Result<Vec<Option<AuditLogModel>>, Box<dyn Error + Send + Sync>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let query = r#"SELECT * FROM audit_log WHERE id = ANY($1)"#;
    let rows = match executor {
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
    async fn test_load_batch() {
        let pool = get_pool().await;
        let executor = Executor::Pool(Arc::new(pool.clone()));

        let log1 = new_test_audit_log();
        let log2 = new_test_audit_log();
        let items = vec![log1.clone(), log2.clone()];
        let item_ids: Vec<Uuid> = items.iter().map(|i| i.id).collect();

        create_batch(&executor, items).await.unwrap();

        let result = load_batch(&executor, &item_ids).await;
        assert!(result.is_ok());
        let loaded_items = result.unwrap();
        assert_eq!(loaded_items.len(), 2);
        assert!(loaded_items.iter().all(|i| i.is_some()));

        // Test with a mix of existing and non-existing IDs
        let mut mixed_ids = item_ids.clone();
        mixed_ids.push(Uuid::new_v4());
        let mixed_result = load_batch(&executor, &mixed_ids).await.unwrap();
        assert_eq!(mixed_result.len(), 3);
        assert_eq!(mixed_result.iter().filter(|i| i.is_some()).count(), 2);
        assert!(mixed_result.iter().any(|i| i.is_none()));

        // Cleanup
        sqlx::query("DELETE FROM audit_log WHERE id = ANY($1)")
            .bind(&item_ids)
            .execute(&pool)
            .await
            .unwrap();
    }
}
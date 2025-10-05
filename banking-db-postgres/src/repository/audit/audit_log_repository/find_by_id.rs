use banking_db::{
    models::audit::AuditLogModel,
    repository::audit_repository::AuditLogResult,
};
use crate::repository::executor::Executor;
use sqlx::Row;
use uuid::Uuid;

pub async fn find_by_id(
    executor: &Executor,
    id: Uuid,
) -> AuditLogResult<Option<AuditLogModel>> {
    let query = sqlx::query(
        r#"
        SELECT * FROM audit_log WHERE id = $1
        "#,
    )
    .bind(id);

    let row = match executor {
        Executor::Pool(pool) => query.fetch_optional(&**pool).await?,
        Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            query.fetch_optional(&mut **tx).await?
        }
    };

    match row {
        Some(row) => Ok(Some(AuditLogModel {
            id: row.get("id"),
            updated_at: row.get("updated_at"),
            updated_by_person_id: row.get("updated_by_person_id"),
        })),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::audit::audit_log_repository::create::create;
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
    async fn test_find_by_id() {
        let pool = get_pool().await;
        let executor = Executor::Pool(Arc::new(pool.clone()));

        let audit_log_model = new_test_audit_log();
        create(&executor, &audit_log_model).await.unwrap();

        let result = find_by_id(&executor, audit_log_model.id).await;
        assert!(result.is_ok());
        let fetched_log = result.unwrap();
        assert!(fetched_log.is_some());
        assert_eq!(fetched_log.unwrap().id, audit_log_model.id);

        // Test not found
        let not_found_result = find_by_id(&executor, Uuid::new_v4()).await;
        assert!(not_found_result.is_ok());
        assert!(not_found_result.unwrap().is_none());

        // Cleanup
        sqlx::query("DELETE FROM audit_log WHERE id = $1")
            .bind(audit_log_model.id)
            .execute(&pool)
            .await
            .unwrap();
    }
}
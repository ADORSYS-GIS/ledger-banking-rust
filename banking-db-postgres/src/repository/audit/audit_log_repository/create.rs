use banking_db::{
    models::audit::AuditLogModel,
    repository::audit_repository::AuditLogResult,
};
use crate::repository::executor::Executor;

pub async fn create(
    executor: &Executor,
    audit_log: &AuditLogModel,
) -> AuditLogResult<AuditLogModel> {
    let query = sqlx::query(
        r#"
        INSERT INTO audit_log (id, updated_at, updated_by_person_id)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(audit_log.id)
    .bind(audit_log.updated_at)
    .bind(audit_log.updated_by_person_id);

    // Match on the executor type to run the query
    match executor {
        Executor::Pool(pool) => {
            query.execute(&**pool).await?;
        }
        Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            query.execute(&mut **tx).await?;
        }
    };

    Ok(audit_log.clone())
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
    async fn test_create_audit_log() {
        let pool = get_pool().await;
        let executor = Executor::Pool(Arc::new(pool.clone()));

        let audit_log_model = new_test_audit_log();
        let result = create(&executor, &audit_log_model).await;

        assert!(result.is_ok());
        let created_log = result.unwrap();
        assert_eq!(created_log.id, audit_log_model.id);

        // Verify the log exists in the database
        let fetched_log: Option<AuditLogModel> =
            sqlx::query_as("SELECT * FROM audit_log WHERE id = $1")
                .bind(audit_log_model.id)
                .fetch_optional(&pool)
                .await
                .unwrap();
        assert!(fetched_log.is_some());
        assert_eq!(fetched_log.unwrap().id, audit_log_model.id);

        // Cleanup
        sqlx::query("DELETE FROM audit_log WHERE id = $1")
            .bind(audit_log_model.id)
            .execute(&pool)
            .await
            .unwrap();
    }
}
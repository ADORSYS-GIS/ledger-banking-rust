use banking_db::models::person::LocalityModel;
use banking_db::repository::{LocalityRepositoryError, LocalityResult};
use crate::repository::executor::Executor;
use crate::repository::person::locality_repository::LocalityRepositoryImpl;
use crate::utils::TryFromRow;
use uuid::Uuid;

pub async fn load(repo: &LocalityRepositoryImpl, id: Uuid) -> LocalityResult<LocalityModel> {
    let query = sqlx::query(
        r#"
        SELECT * FROM locality WHERE id = $1
        "#,
    )
    .bind(id);

    let row = match &repo.executor {
        Executor::Pool(pool) => query
            .fetch_one(&**pool)
            .await
            .map_err(|e| LocalityRepositoryError::RepositoryError(e.into()))?,
        Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            query
                .fetch_one(&mut **tx)
                .await
                .map_err(|e| LocalityRepositoryError::RepositoryError(e.into()))?
        }
    };

    LocalityModel::try_from_row(&row)
        .map_err(LocalityRepositoryError::RepositoryError)
}
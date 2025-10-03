use banking_db::models::person::LocationModel;
use banking_db::repository::{LocationRepositoryError, LocationResult};
use crate::repository::executor::Executor;
use crate::repository::person::location_repository::LocationRepositoryImpl;
use crate::utils::TryFromRow;
use uuid::Uuid;

pub async fn load(repo: &LocationRepositoryImpl, id: Uuid) -> LocationResult<LocationModel> {
    let query = sqlx::query(
        r#"
        SELECT * FROM location WHERE id = $1
        "#,
    )
    .bind(id);

    let row = match &repo.executor {
        Executor::Pool(pool) => query.fetch_one(&**pool).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?,
        Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            query.fetch_one(&mut **tx).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?
        }
    };

    LocationModel::try_from_row(&row).map_err(LocationRepositoryError::RepositoryError)
}
use banking_db::models::person::CountrySubdivisionModel;
use banking_db::repository::{CountrySubdivisionRepositoryError, CountrySubdivisionResult};
use uuid::Uuid;

use crate::repository::executor::Executor;
use crate::utils::TryFromRow;

use super::repo_impl::CountrySubdivisionRepositoryImpl;

pub async fn load(
    repo: &CountrySubdivisionRepositoryImpl,
    id: Uuid,
) -> CountrySubdivisionResult<CountrySubdivisionModel> {
    let query = sqlx::query(
        r#"
        SELECT * FROM country_subdivision WHERE id = $1
        "#,
    )
    .bind(id);

    let row = match &repo.executor {
        Executor::Pool(pool) => query
            .fetch_one(&**pool)
            .await
            .map_err(|e| CountrySubdivisionRepositoryError::RepositoryError(e.into()))?,
        Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            query
                .fetch_one(&mut **tx)
                .await
                .map_err(|e| CountrySubdivisionRepositoryError::RepositoryError(e.into()))?
        }
    };

    CountrySubdivisionModel::try_from_row(&row)
        .map_err(CountrySubdivisionRepositoryError::RepositoryError)
}
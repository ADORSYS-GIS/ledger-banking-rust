use banking_db::models::person::PersonModel;
use banking_db::repository::{PersonRepositoryError, PersonResult};
use crate::repository::person::person_repository::repo_impl::PersonRepositoryImpl;
use crate::utils::TryFromRow;
use uuid::Uuid;

pub async fn load(repo: &PersonRepositoryImpl, id: Uuid) -> PersonResult<PersonModel> {
    let query = sqlx::query(
        r#"
        SELECT * FROM person WHERE id = $1
        "#,
    )
    .bind(id);

    let row = match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => query.fetch_one(&**pool).await?,
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            query.fetch_one(&mut **tx).await?
        }
    };

    PersonModel::try_from_row(&row).map_err(PersonRepositoryError::RepositoryError)
}
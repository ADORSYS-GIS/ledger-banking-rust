use crate::repository::executor::Executor;
use crate::repository::person::country_repository::repo_impl::CountryRepositoryImpl;
use crate::utils::TryFromRow;
use banking_db::models::person::CountryModel;
use banking_db::repository::person::country_repository::{
    CountryRepositoryError, CountryResult,
};
use uuid::Uuid;

pub(crate) async fn load(
    repo: &CountryRepositoryImpl,
    id: Uuid,
) -> CountryResult<CountryModel> {
    let query = sqlx::query(
        r#"
        SELECT * FROM country WHERE id = $1
        "#,
    )
    .bind(id);

    let row = match &repo.executor {
        Executor::Pool(pool) => query.fetch_one(&**pool).await,
        Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            query.fetch_one(&mut **tx).await
        }
    };

    match row {
        Ok(row) => {
            CountryModel::try_from_row(&row).map_err(CountryRepositoryError::RepositoryError)
        }
        Err(sqlx::Error::RowNotFound) => Err(CountryRepositoryError::CountryNotFound(id)),
        Err(e) => Err(CountryRepositoryError::RepositoryError(e.into())),
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::person::country_repository::test_helpers::setup_test_country;
    use crate::test_helper::setup_test_context;
    use banking_db::repository::person::country_repository::{
        CountryRepository, CountryRepositoryError,
    };
    use banking_db::repository::PersonRepos;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_load_country() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();

        // 1. Setup: Create and save a country
        let mut country_model = setup_test_country().await;
        country_model = country_repo.save(country_model).await?;

        // 2. Test loading an existing country
        let loaded_country = country_repo.load(country_model.id).await?;
        assert_eq!(loaded_country.id, country_model.id);
        assert_eq!(loaded_country.iso2, country_model.iso2);

        // 3. Test loading a non-existent country
        let non_existent_id = Uuid::new_v4();
        let result = country_repo.load(non_existent_id).await;
        assert!(matches!(
            result,
            Err(CountryRepositoryError::CountryNotFound(id)) if id == non_existent_id
        ));

        Ok(())
    }
}
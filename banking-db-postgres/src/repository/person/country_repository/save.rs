use crate::repository::executor::Executor;
use crate::repository::person::country_repository::repo_impl::CountryRepositoryImpl;
use banking_db::models::person::{CountryIdxModel, CountryModel};
use banking_db::repository::person::country_repository::{CountryRepositoryError, CountryResult};

pub(crate) async fn save(
    repo: &CountryRepositoryImpl,
    country: CountryModel,
) -> CountryResult<CountryModel> {
    // Check if a country with this ISO2 already exists
    {
        let cache = repo.country_idx_cache.read().await;
        if cache.get_by_iso2(&country.iso2).is_some() {
            return Err(CountryRepositoryError::DuplicateCountryISO2(
                country.iso2.to_string(),
            ));
        }
    }

    let query1 = sqlx::query(
        r#"
        INSERT INTO country (id, iso2, name_l1, name_l2, name_l3)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(country.id)
    .bind(country.iso2.as_str())
    .bind(country.name_l1.as_str())
    .bind(country.name_l2.as_ref().map(|s| s.as_str()))
    .bind(country.name_l3.as_ref().map(|s| s.as_str()));

    let query2 = sqlx::query(
        r#"
        INSERT INTO country_idx (country_id, iso2)
        VALUES ($1, $2)
        "#,
    )
    .bind(country.id)
    .bind(country.iso2.as_str());

    let execute_queries = async {
        match &repo.executor {
            Executor::Pool(pool) => {
                query1.execute(&**pool).await?;
                query2.execute(&**pool).await?;
            }
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query1.execute(&mut **tx).await?;
                query2.execute(&mut **tx).await?;
            }
        }
        Ok::<(), sqlx::Error>(())
    };

    if let Err(e) = execute_queries.await {
        if let Some(db_err) = e.as_database_error() {
            if db_err.is_unique_violation() {
                return Err(CountryRepositoryError::DuplicateCountryISO2(
                    country.iso2.to_string(),
                ));
            }
        }
        return Err(CountryRepositoryError::RepositoryError(e.into()));
    }

    let new_idx_model = CountryIdxModel {
        country_id: country.id,
        iso2: country.iso2.clone(),
    };
    repo.country_idx_cache.read().await.add(new_idx_model);

    Ok(country)
}

#[cfg(test)]
mod tests {
    use crate::repository::person::country_repository::test_helpers::setup_test_country;
    use crate::test_helper::setup_test_context;
    use banking_db::repository::person::country_repository::{
        CountryRepository, CountryRepositoryError,
    };
    use banking_db::repository::PersonRepos;

    #[tokio::test]
    async fn test_save_country() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();

        // 1. Test saving a new country
        let country_model = setup_test_country().await;
        let saved_country = country_repo.save(country_model.clone()).await?;
        assert_eq!(saved_country.id, country_model.id);

        // 2. Test saving a country with a duplicate iso2
        let result = country_repo.save(country_model).await;
        assert!(matches!(
            result,
            Err(CountryRepositoryError::DuplicateCountryISO2(_))
        ));

        Ok(())
    }
}
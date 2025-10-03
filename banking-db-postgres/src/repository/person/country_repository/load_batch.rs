// FILE: banking-db-postgres/src/repository/person/country_repository/load_batch.rs

use crate::repository::person::country_repository::repo_impl::CountryRepositoryImpl;
use crate::utils::TryFromRow;
use banking_db::models::person::CountryModel;
use std::error::Error;
use uuid::Uuid;

pub(crate) async fn load_batch(
    repo: &CountryRepositoryImpl,
    ids: &[Uuid],
) -> Result<Vec<Option<CountryModel>>, Box<dyn Error + Send + Sync>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let query = r#"SELECT * FROM country WHERE id = ANY($1)"#;
    let rows = match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(query).bind(ids).fetch_all(&**pool).await?
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(query).bind(ids).fetch_all(&mut **tx).await?
        }
    };
    let mut item_map = std::collections::HashMap::new();
    for row in rows {
        let item = CountryModel::try_from_row(&row)?;
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
    use banking_db::repository::{BatchRepository, PersonRepos};
    use heapless::String as HeaplessString;
    use uuid::Uuid;
    use crate::test_helper::setup_test_context;
    use crate::repository::person::country_repository::test_helpers::setup_test_country;

    #[tokio::test]
    async fn test_load_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();

        let mut countries = Vec::new();
        for i in 0..5 {
            let mut country = setup_test_country().await;
            country.iso2 =
                HeaplessString::try_from(format!("L{i}").as_str()).unwrap();
            country.name_l1 =
                HeaplessString::try_from(format!("Test Country {i}").as_str()).unwrap();
            countries.push(country);
        }

        let audit_log_id = Uuid::new_v4();

        let saved_countries = country_repo
            .create_batch(countries.clone(), audit_log_id)
            .await?;
        let ids: Vec<Uuid> = saved_countries.iter().map(|c| c.id).collect();

        let loaded_countries = country_repo.load_batch(&ids).await?;
        assert_eq!(loaded_countries.len(), 5);
        for country_opt in loaded_countries {
            assert!(country_opt.is_some());
        }

        Ok(())
    }
}
// FILE: banking-db-postgres/src/repository/person/country_repository/delete_batch.rs

use crate::repository::person::country_repository::repo_impl::CountryRepositoryImpl;
use banking_db::repository::CountryRepository;
use std::error::Error;
use uuid::Uuid;

pub(crate) async fn delete_batch(
    repo: &CountryRepositoryImpl,
    ids: &[Uuid],
) -> Result<usize, Box<dyn Error + Send + Sync>> {
    if ids.is_empty() {
        return Ok(0);
    }

    let existings = repo.find_by_ids(ids).await?;
    let existing_ids: Vec<Uuid> = existings.iter().map(|p| p.country_id).collect();

    {
        let cache = repo.country_idx_cache.write().await;
        for id in &existing_ids {
            cache.remove(id);
        }
    }

    let delete_query = r#"DELETE FROM country WHERE id = ANY($1)"#;
    let delete_idx_query = r#"DELETE FROM country_idx WHERE country_id = ANY($1)"#;

    match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(delete_idx_query).bind(&existing_ids).execute(&**pool).await?;
            sqlx::query(delete_query).bind(&existing_ids).execute(&**pool).await?;
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(delete_idx_query).bind(&existing_ids).execute(&mut **tx).await?;
            sqlx::query(delete_query).bind(&existing_ids).execute(&mut **tx).await?;
        }
    }

    Ok(existing_ids.len())
}

#[cfg(test)]
mod tests {
    use banking_db::repository::{BatchRepository, PersonRepos, CountryRepository};
    use heapless::String as HeaplessString;
    use uuid::Uuid;
    use crate::test_helper::setup_test_context;
    use crate::repository::person::country_repository::test_helpers::setup_test_country;

    #[tokio::test]
    async fn test_delete_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();

        let mut countries = Vec::new();
        for i in 0..5 {
            let mut country = setup_test_country().await;
            country.iso2 =
                HeaplessString::try_from(format!("D{i}").as_str()).unwrap();
            country.name_l1 =
                HeaplessString::try_from(format!("Test Country {i}").as_str()).unwrap();
            countries.push(country);
        }

        let audit_log_id = Uuid::new_v4();

        let saved_countries = country_repo
            .create_batch(countries.clone(), audit_log_id)
            .await?;
        let ids: Vec<Uuid> = saved_countries.iter().map(|c| c.id).collect();

        let deleted_count = country_repo.delete_batch(&ids).await?;
        assert_eq!(deleted_count, 5);

        for id in ids {
            assert!(!country_repo.exists_by_id(id).await?);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_batch_with_non_existing_countries() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();

        let mut ids = Vec::new();
        for _ in 0..2 {
            ids.push(Uuid::new_v4());
        }

        let deleted_count = country_repo.delete_batch(&ids).await?;
        assert_eq!(deleted_count, 0);

        Ok(())
    }
}
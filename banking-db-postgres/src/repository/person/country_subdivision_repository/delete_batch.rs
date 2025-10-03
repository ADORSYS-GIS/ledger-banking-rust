use crate::repository::person::country_subdivision_repository::CountrySubdivisionRepositoryImpl;
use banking_db::repository::{CountrySubdivisionRepositoryError, LocalityRepository};
use std::error::Error;
use uuid::Uuid;

pub async fn delete_batch(
    repo: &CountrySubdivisionRepositoryImpl,
    _ids: &[Uuid],
) -> Result<usize, Box<dyn Error + Send + Sync>> {
    if _ids.is_empty() {
        return Ok(0);
    }

    let locality_repo = repo.locality_repository.get().unwrap();
    let mut dependent_localities = Vec::new();
    for id in _ids {
        let localities = locality_repo
            .find_by_country_subdivision_id(*id, 0, 1)
            .await?;
        if !localities.is_empty() {
            dependent_localities.push(*id);
        }
    }

    if !dependent_localities.is_empty() {
        return Err(Box::new(
            CountrySubdivisionRepositoryError::HasDependentLocalities(dependent_localities),
        ));
    }

    let cache = repo.country_subdivision_idx_cache.read().await;
    for id in _ids {
        cache.remove(id);
    }

    let delete_query = r#"DELETE FROM country_subdivision WHERE id = ANY($1)"#;
    let delete_idx_query = r#"DELETE FROM country_subdivision_idx WHERE country_subdivision_id = ANY($1)"#;

    match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(delete_idx_query).bind(_ids).execute(&**pool).await?;
            sqlx::query(delete_query).bind(_ids).execute(&**pool).await?;
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(delete_idx_query).bind(_ids).execute(&mut **tx).await?;
            sqlx::query(delete_query).bind(_ids).execute(&mut **tx).await?;
        }
    }

    Ok(_ids.len())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::person::test_helpers::create_test_country_model;
    use crate::test_helper::setup_test_context;
    use banking_db::repository::{BatchRepository, CountryRepository, CountrySubdivisionRepository, PersonRepos};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_delete_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();
        let country_subdivision_repo = ctx.person_repos().country_subdivisions();

        let country = create_test_country_model("US", "United States");
        country_repo.save(country.clone()).await?;

        let mut country_subdivisions = Vec::new();
        let mut ids = Vec::new();
        for i in 0..5 {
            let subdivision =
                crate::repository::person::test_helpers::create_test_country_subdivision_model(
                    country.id,
                    &format!("CD{i:03}"),
                    &format!("Test Subdivision {i}"),
                );
            ids.push(subdivision.id);
            country_subdivisions.push(subdivision);
        }

        let audit_log_id = Uuid::new_v4();
        country_subdivision_repo
            .create_batch(country_subdivisions.clone(), audit_log_id)
            .await?;

        let result = country_subdivision_repo.delete_batch(&ids).await?;
        assert_eq!(result, 5);

        for id in &ids {
            assert!(!country_subdivision_repo.exists_by_id(*id).await?);
        }

        Ok(())
    }
}
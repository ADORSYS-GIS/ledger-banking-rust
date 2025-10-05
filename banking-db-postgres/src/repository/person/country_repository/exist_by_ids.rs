use crate::repository::person::country_repository::repo_impl::CountryRepositoryImpl;
use banking_db::repository::person::country_repository::CountryResult;
use uuid::Uuid;

pub(crate) async fn exist_by_ids(
    repo: &CountryRepositoryImpl,
    ids: &[Uuid],
) -> CountryResult<Vec<(Uuid, bool)>> {
    let mut result = Vec::new();
    let cache = repo.country_idx_cache.read().await;
    for &id in ids {
        result.push((id, cache.contains_primary(&id)));
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::repository::person::country_repository::test_helpers::setup_test_country;
    use crate::test_helper::setup_test_context;
    use banking_db::repository::{CountryRepository, PersonRepos};
    use heapless::String as HeaplessString;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_exist_by_ids() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();

        // 1. Setup: Create and save two countries within a single transaction
        let mut country1 = setup_test_country().await;
        country1.iso2 = HeaplessString::try_from("U1").unwrap();
        country_repo.save(country1.clone()).await?;

        let mut country2 = setup_test_country().await;
        country2.iso2 = HeaplessString::try_from("U2").unwrap();
        country_repo.save(country2.clone()).await?;

        // 2. Call the function with a mix of existing and non-existing IDs
        // The cache is transaction-aware, so it should find the newly saved countries.
        let non_existent_id = Uuid::new_v4();
        let ids_to_check = vec![country1.id, non_existent_id, country2.id];
        let results = country_repo.exist_by_ids(&ids_to_check).await?;

        // 3. Assert the results
        assert_eq!(results.len(), 3);

        let mut results_map = std::collections::HashMap::new();
        for (id, exists) in results {
            results_map.insert(id, exists);
        }

        assert_eq!(results_map.get(&country1.id), Some(&true));
        assert_eq!(results_map.get(&country2.id), Some(&true));
        assert_eq!(results_map.get(&non_existent_id), Some(&false));

        Ok(())
    }
}
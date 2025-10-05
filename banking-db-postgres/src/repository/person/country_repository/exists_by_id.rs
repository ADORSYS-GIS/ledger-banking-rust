use crate::repository::person::country_repository::repo_impl::CountryRepositoryImpl;
use banking_db::repository::person::country_repository::CountryResult;
use uuid::Uuid;

pub(crate) async fn exists_by_id(
    repo: &CountryRepositoryImpl,
    id: Uuid,
) -> CountryResult<bool> {
    Ok(repo.country_idx_cache.read().await.contains_primary(&id))
}

#[cfg(test)]
mod tests {
    use crate::repository::person::country_repository::test_helpers::setup_test_country;
    use crate::test_helper::setup_test_context;
    use banking_db::repository::{CountryRepository, PersonRepos};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_exists_by_id() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();

        // 1. Setup: Create and save a country
        let country = setup_test_country().await;
        country_repo.save(country.clone()).await?;

        // 2. Test with an existing ID
        let exists = country_repo.exists_by_id(country.id).await?;
        assert!(exists);

        // 3. Test with a non-existing ID
        let non_existent_id = Uuid::new_v4();
        let exists = country_repo.exists_by_id(non_existent_id).await?;
        assert!(!exists);

        Ok(())
    }
}
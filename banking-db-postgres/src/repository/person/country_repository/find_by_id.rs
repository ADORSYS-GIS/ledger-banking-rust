use crate::repository::person::country_repository::repo_impl::CountryRepositoryImpl;
use banking_db::models::person::CountryIdxModel;
use banking_db::repository::person::country_repository::CountryResult;
use uuid::Uuid;

pub(crate) async fn find_by_id(
    repo: &CountryRepositoryImpl,
    id: Uuid,
) -> CountryResult<Option<CountryIdxModel>> {
    let cache = repo.country_idx_cache.read().await;
    Ok(cache.get_by_primary(&id))
}

#[cfg(test)]
mod tests {
    use crate::repository::person::country_repository::test_helpers::setup_test_country;
    use crate::test_helper::setup_test_context;
    use banking_db::repository::{CountryRepository, PersonRepos};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_find_by_id() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();

        // 1. Setup: Create and save a country
        let country_model = setup_test_country().await;
        country_repo.save(country_model.clone()).await?;

        // 2. Test with an existing ID
        let found_country = country_repo.find_by_id(country_model.id).await?;
        assert!(found_country.is_some());
        assert_eq!(found_country.unwrap().country_id, country_model.id);

        // 3. Test with a non-existing ID
        let non_existent_id = Uuid::new_v4();
        let found_country = country_repo.find_by_id(non_existent_id).await?;
        assert!(found_country.is_none());

        Ok(())
    }
}
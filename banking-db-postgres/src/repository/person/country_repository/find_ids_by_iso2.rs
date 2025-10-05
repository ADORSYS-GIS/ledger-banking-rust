use crate::repository::person::country_repository::repo_impl::CountryRepositoryImpl;
use banking_db::repository::person::country_repository::{
    CountryRepositoryError, CountryResult,
};
use heapless::String as HeaplessString;
use std::str::FromStr;
use uuid::Uuid;

pub(crate) async fn find_ids_by_iso2(
    repo: &CountryRepositoryImpl,
    iso2: &str,
) -> CountryResult<Vec<Uuid>> {
    let iso2_heapless = HeaplessString::<2>::from_str(iso2)
        .map_err(|_| CountryRepositoryError::InvalidCountryISO2(iso2.to_string()))?;
    let mut result = Vec::new();
    if let Some(country_id) = repo.country_idx_cache.read().await.get_by_iso2(&iso2_heapless) {
        result.push(country_id);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::repository::person::country_repository::test_helpers::setup_test_country;
    use crate::test_helper::setup_test_context;
    use banking_db::repository::{CountryRepository, PersonRepos};
    use heapless::String as HeaplessString;

    #[tokio::test]
    async fn test_find_ids_by_iso2() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();

        // 1. Setup: Create and save a country with a unique ISO2 code
        let mut country_model = setup_test_country().await;
        let unique_iso2 = "T3";
        country_model.iso2 = HeaplessString::try_from(unique_iso2).unwrap();
        country_repo.save(country_model.clone()).await?;

        // 2. Test with an existing ISO2 code
        let found_ids = country_repo.find_ids_by_iso2(unique_iso2).await?;
        assert_eq!(found_ids.len(), 1);
        assert_eq!(found_ids[0], country_model.id);

        // 3. Test with a non-existing ISO2 code
        let non_existent_iso2 = "T4";
        let found_ids = country_repo.find_ids_by_iso2(non_existent_iso2).await?;
        assert!(found_ids.is_empty());

        Ok(())
    }
}
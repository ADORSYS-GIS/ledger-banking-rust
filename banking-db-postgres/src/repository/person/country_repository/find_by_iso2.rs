use crate::repository::person::country_repository::repo_impl::CountryRepositoryImpl;
use banking_db::models::person::CountryIdxModel;
use banking_db::repository::person::country_repository::{
    CountryRepositoryError, CountryResult,
};
use heapless::String as HeaplessString;
use std::str::FromStr;

pub(crate) async fn find_by_iso2(
    repo: &CountryRepositoryImpl,
    iso2: &str,
    _page: i32,
    _page_size: i32,
) -> CountryResult<Vec<CountryIdxModel>> {
    let mut result = Vec::new();
    let iso2_heapless = HeaplessString::<2>::from_str(iso2)
        .map_err(|_| CountryRepositoryError::InvalidCountryISO2(iso2.to_string()))?;
    let cache = repo.country_idx_cache.read().await;
    if let Some(country_id) = cache.get_by_iso2(&iso2_heapless) {
        if let Some(country_idx) = cache.get_by_primary(&country_id) {
            result.push(country_idx);
        }
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
    async fn test_find_by_iso2() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();

        // 1. Setup: Create and save a country with a unique ISO2 code
        let mut country_model = setup_test_country().await;
        let unique_iso2 = "T1";
        country_model.iso2 = HeaplessString::try_from(unique_iso2).unwrap();
        country_repo.save(country_model.clone()).await?;

        // 2. Test with an existing ISO2 code
        let found_countries = country_repo.find_by_iso2(unique_iso2, 1, 10).await?;
        assert_eq!(found_countries.len(), 1);
        assert_eq!(found_countries[0].country_id, country_model.id);
        assert_eq!(found_countries[0].iso2.as_str(), unique_iso2);

        // 3. Test with a non-existing ISO2 code
        let non_existent_iso2 = "T2";
        let found_countries = country_repo.find_by_iso2(non_existent_iso2, 1, 10).await?;
        assert!(found_countries.is_empty());

        Ok(())
    }
}
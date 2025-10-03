// FILE: banking-db-postgres/src/repository/person/country_repository/create_batch.rs

use crate::repository::person::country_repository::repo_impl::CountryRepositoryImpl;
use banking_db::models::person::{CountryIdxModel, CountryModel};
use banking_db::repository::{
    CountryRepository,
    CountryRepositoryError,
};
use std::error::Error;
use uuid::Uuid;

pub(crate) async fn create_batch(
    repo: &CountryRepositoryImpl,
    items: Vec<CountryModel>,
    _audit_log_id: Uuid,
) -> Result<Vec<CountryModel>, Box<dyn Error + Send + Sync>> {
    if items.is_empty() {
        return Ok(Vec::new());
    }

    let ids: Vec<Uuid> = items.iter().map(|p| p.id).collect();
    let existing_check = repo.exist_by_ids(&ids).await?;
    let truly_existing_ids: Vec<Uuid> = existing_check
        .into_iter()
        .filter_map(|(id, exists)| if exists { Some(id) } else { None })
        .collect();

    if !truly_existing_ids.is_empty() {
        return Err(Box::new(CountryRepositoryError::ManyCountriesExist(
            truly_existing_ids,
        )));
    }

    let cache = repo.country_idx_cache.read().await;
    for item in &items {
        let idx_model = CountryIdxModel {
            country_id: item.id,
            iso2: item.iso2.clone(),
        };
        cache.add(idx_model);
    }

    let mut country_values = Vec::new();
    let mut country_idx_values = Vec::new();
    let mut saved_items = Vec::new();

    for item in items {
        country_values.push((
            item.id,
            item.iso2.to_string(),
            item.name_l1.to_string(),
            item.name_l2.as_ref().map(|s| s.to_string()),
            item.name_l3.as_ref().map(|s| s.to_string()),
        ));

        country_idx_values.push((
            item.id,
            item.iso2.to_string(),
        ));
        saved_items.push(item);
    }

    if !country_values.is_empty() {
        repo.execute_country_insert(country_values).await?;
        repo.execute_country_idx_insert(country_idx_values).await?;
    }

    Ok(saved_items)
}

#[cfg(test)]
mod tests {
    use banking_db::repository::{BatchRepository, CountryRepository, CountryRepositoryError, PersonRepos};
    use heapless::String as HeaplessString;
    use uuid::Uuid;
    use crate::test_helper::setup_test_context;
    use crate::repository::person::country_repository::test_helpers::setup_test_country;

    #[tokio::test]
    async fn test_create_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();

        let mut countries = Vec::new();
        for i in 0..5 {
            let mut country = setup_test_country().await;
            country.iso2 =
                HeaplessString::try_from(format!("C{i}").as_str()).unwrap();
            country.name_l1 =
                HeaplessString::try_from(format!("Test Country {i}").as_str()).unwrap();
            countries.push(country);
        }

        let audit_log_id = Uuid::new_v4();

        let saved_countries = country_repo
            .create_batch(countries.clone(), audit_log_id)
            .await?;

        assert_eq!(saved_countries.len(), 5);

        for country in &saved_countries {
            assert!(country_repo.exists_by_id(country.id).await?);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_create_batch_with_existing_countries() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();

        let mut countries = Vec::new();
        for i in 0..2 {
            let mut country = setup_test_country().await;
            country.iso2 =
                HeaplessString::try_from(format!("E{i}").as_str()).unwrap();
            country.name_l1 =
                HeaplessString::try_from(format!("Test Country {i}").as_str()).unwrap();
            countries.push(country);
        }

        let audit_log_id = Uuid::new_v4();

        country_repo
            .create_batch(countries.clone(), audit_log_id)
            .await?;

        let result = country_repo
            .create_batch(countries.clone(), audit_log_id)
            .await;

        assert!(result.is_err());
        if let Err(err) = result {
            let err_str = err.to_string();
            let expected_err = CountryRepositoryError::ManyCountriesExist(countries.iter().map(|c| c.id).collect()).to_string();
            assert_eq!(err_str, expected_err);
        }

        Ok(())
    }
}
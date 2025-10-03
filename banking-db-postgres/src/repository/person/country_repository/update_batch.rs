// FILE: banking-db-postgres/src/repository/person/country_repository/update_batch.rs

use crate::repository::person::country_repository::repo_impl::CountryRepositoryImpl;
use banking_db::models::person::CountryModel;
use banking_db::repository::{
    CountryRepository,
    CountryRepositoryError,
};
use std::error::Error;
use uuid::Uuid;

pub(crate) async fn update_batch(
    repo: &CountryRepositoryImpl,
    items: Vec<CountryModel>,
    _audit_log_id: Uuid,
) -> Result<Vec<CountryModel>, Box<dyn Error + Send + Sync>> {
    if items.is_empty() {
        return Ok(Vec::new());
    }

    let ids: Vec<Uuid> = items.iter().map(|p| p.id).collect();
    let existing_check = repo.exist_by_ids(&ids).await?;
    let missing_ids: Vec<Uuid> = existing_check
        .into_iter()
        .filter_map(|(id, exists)| if !exists { Some(id) } else { None })
        .collect();

    if !missing_ids.is_empty() {
        return Err(Box::new(CountryRepositoryError::ManyCountriesNotFound(
            missing_ids,
        )));
    }

    let mut country_values = Vec::new();
    let mut updated_items = Vec::new();

    for item in items {
        country_values.push((
            item.id,
            item.iso2.to_string(),
            item.name_l1.to_string(),
            item.name_l2.as_ref().map(|s| s.to_string()),
            item.name_l3.as_ref().map(|s| s.to_string()),
        ));
        updated_items.push(item);
    }

    if !country_values.is_empty() {
        repo.execute_country_update(country_values).await?;
    }

    Ok(updated_items)
}

#[cfg(test)]
mod tests {
    use banking_db::repository::{BatchRepository, PersonRepos, CountryRepositoryError, CountryRepository};
    use heapless::String as HeaplessString;
    use uuid::Uuid;
    use crate::test_helper::setup_test_context;
    use crate::repository::person::country_repository::test_helpers::setup_test_country;

    #[tokio::test]
    async fn test_update_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();

        let mut countries = Vec::new();
        for i in 0..5 {
            let mut country = setup_test_country().await;
            country.iso2 =
                HeaplessString::try_from(format!("U{i}").as_str()).unwrap();
            country.name_l1 =
                HeaplessString::try_from(format!("Test Country {i}").as_str()).unwrap();
            countries.push(country);
        }

        let audit_log_id = Uuid::new_v4();

        let saved_countries = country_repo
            .create_batch(countries.clone(), audit_log_id)
            .await?;
        
        let mut countries_to_update = Vec::new();
        for mut country in saved_countries {
            country.name_l1 = HeaplessString::try_from("Updated Name").unwrap();
            countries_to_update.push(country);
        }

        let updated_countries = country_repo
            .update_batch(countries_to_update, audit_log_id)
            .await?;

        assert_eq!(updated_countries.len(), 5);

        for country in &updated_countries {
            let loaded_country = country_repo.load(country.id).await?;
            assert_eq!(loaded_country.name_l1, "Updated Name");
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_update_batch_with_non_existing_countries() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();

        let mut countries = Vec::new();
        let mut non_existing_countries = Vec::new();
        for i in 0..2 {
            let mut country = setup_test_country().await;
            country.iso2 =
                HeaplessString::try_from(format!("N{i}").as_str()).unwrap();
            country.name_l1 =
                HeaplessString::try_from(format!("Test Country {i}").as_str()).unwrap();
            countries.push(country.clone());
            non_existing_countries.push(country);
        }

        let audit_log_id = Uuid::new_v4();

        let result = country_repo
            .update_batch(non_existing_countries.clone(), audit_log_id)
            .await;

        assert!(result.is_err());
        if let Err(err) = result {
            let err_str = err.to_string();
            let expected_err = CountryRepositoryError::ManyCountriesNotFound(non_existing_countries.iter().map(|c| c.id).collect()).to_string();
            assert_eq!(err_str, expected_err);
        }

        Ok(())
    }
}
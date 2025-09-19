// FILE: banking-db-postgres/tests/suites/person/country_batch_operations_test.rs

use banking_db::models::person::CountryModel;
use banking_db::repository::{BatchRepository, CountryRepository, PersonRepos};
use heapless::String as HeaplessString;
use uuid::Uuid;

use crate::suites::test_helper::setup_test_context;

pub async fn setup_test_country() -> CountryModel {
    CountryModel {
        id: Uuid::new_v4(),
        iso2: HeaplessString::try_from("CM").unwrap(),
        name_l1: HeaplessString::try_from("Cameroon").unwrap(),
        name_l2: Some(HeaplessString::try_from("Cameroun").unwrap()),
        name_l3: None,
    }
}

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
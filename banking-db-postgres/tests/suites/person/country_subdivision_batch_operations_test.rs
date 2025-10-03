// FILE: banking-db-postgres/tests/suites/person/country_subdivision_batch_operations_test.rs

use banking_db::models::person::{CountrySubdivisionModel};
use banking_db::repository::{BatchRepository, CountryRepository, CountrySubdivisionRepository, PersonRepos};
use heapless::String as HeaplessString;
use uuid::Uuid;

use crate::suites::test_helper::setup_test_context;
use crate::suites::person::helpers::setup_test_country;

pub async fn setup_test_country_subdivision(country_id: Uuid) -> CountrySubdivisionModel {
    CountrySubdivisionModel {
        id: Uuid::new_v4(),
        country_id,
        code: HeaplessString::try_from("TEST").unwrap(),
        name_l1: HeaplessString::try_from("Test Subdivision").unwrap(),
        name_l2: None,
        name_l3: None,
    }
}

#[tokio::test]
async fn test_create_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = setup_test_context().await?;
    let country_repo = ctx.person_repos().countries();
    let country_subdivision_repo = ctx.person_repos().country_subdivisions();

    let country = setup_test_country().await;
    country_repo.save(country.clone()).await?;

    let mut country_subdivisions = Vec::new();
    for i in 0..5 {
        let mut subdivision = setup_test_country_subdivision(country.id).await;
        subdivision.code =
            HeaplessString::try_from(format!("CD{i:03}").as_str()).unwrap();
        subdivision.name_l1 =
            HeaplessString::try_from(format!("Test Subdivision {i}").as_str()).unwrap();
        country_subdivisions.push(subdivision);
    }

    let audit_log_id = Uuid::new_v4();

    let saved_subdivisions = country_subdivision_repo
        .create_batch(country_subdivisions.clone(), audit_log_id)
        .await?;

    assert_eq!(saved_subdivisions.len(), 5);

    for subdivision in &saved_subdivisions {
        assert!(country_subdivision_repo.exists_by_id(subdivision.id).await?);
    }

    Ok(())
}

#[tokio::test]
async fn test_load_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = setup_test_context().await?;
    let country_repo = ctx.person_repos().countries();
    let country_subdivision_repo = ctx.person_repos().country_subdivisions();

    let country = setup_test_country().await;
    country_repo.save(country.clone()).await?;

    let mut country_subdivisions = Vec::new();
    let mut ids = Vec::new();
    for i in 0..5 {
        let mut subdivision = setup_test_country_subdivision(country.id).await;
        subdivision.code =
            HeaplessString::try_from(format!("CD{i:03}").as_str()).unwrap();
        subdivision.name_l1 =
            HeaplessString::try_from(format!("Test Subdivision {i}").as_str()).unwrap();
        ids.push(subdivision.id);
        country_subdivisions.push(subdivision);
    }

    let audit_log_id = Uuid::new_v4();
    country_subdivision_repo
        .create_batch(country_subdivisions.clone(), audit_log_id)
        .await?;

    let loaded_subdivisions = country_subdivision_repo.load_batch(&ids).await?;
    assert_eq!(loaded_subdivisions.len(), 5);
    for (i, subdivision) in loaded_subdivisions.iter().enumerate() {
        assert!(subdivision.is_some());
        assert_eq!(subdivision.as_ref().unwrap().id, country_subdivisions[i].id);
    }

    Ok(())
}
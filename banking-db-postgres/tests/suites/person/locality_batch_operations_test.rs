// FILE: banking-db-postgres/tests/suites/person/locality_batch_operations_test.rs

use banking_db::models::person::{LocalityModel};
use banking_db::repository::{
    BatchRepository, CountryRepository, CountrySubdivisionRepository, LocalityRepository, PersonRepos,
};
use heapless::String as HeaplessString;
use uuid::Uuid;

use crate::suites::test_helper::setup_test_context;
use crate::suites::person::helpers::setup_test_country;
use crate::suites::person::country_subdivision_batch_operations_test::setup_test_country_subdivision;

pub async fn setup_test_locality(country_subdivision_id: Uuid) -> LocalityModel {
    LocalityModel {
        id: Uuid::new_v4(),
        country_subdivision_id,
        code: HeaplessString::try_from("TEST").unwrap(),
        name_l1: HeaplessString::try_from("Test Locality").unwrap(),
        name_l2: None,
        name_l3: None,
    }
}

#[tokio::test]
async fn test_create_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = setup_test_context().await?;
    let country_repo = ctx.person_repos().countries();
    let country_subdivision_repo = ctx.person_repos().country_subdivisions();
    let locality_repo = ctx.person_repos().localities();

    let country = setup_test_country().await;
    country_repo.save(country.clone()).await?;

    let subdivision = setup_test_country_subdivision(country.id).await;
    country_subdivision_repo.save(subdivision.clone()).await?;

    let mut localities = Vec::new();
    for i in 0..5 {
        let mut locality = setup_test_locality(subdivision.id).await;
        locality.code = HeaplessString::try_from(format!("CD{i:03}").as_str()).unwrap();
        locality.name_l1 = HeaplessString::try_from(format!("Test Locality {i}").as_str()).unwrap();
        localities.push(locality);
    }

    let audit_log_id = Uuid::new_v4();

    let saved_localities = locality_repo
        .create_batch(localities.clone(), audit_log_id)
        .await?;

    assert_eq!(saved_localities.len(), 5);

    for locality in &saved_localities {
        assert!(locality_repo.exists_by_id(locality.id).await?);
    }

    Ok(())
}

#[tokio::test]
async fn test_load_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = setup_test_context().await?;
    let country_repo = ctx.person_repos().countries();
    let country_subdivision_repo = ctx.person_repos().country_subdivisions();
    let locality_repo = ctx.person_repos().localities();

    let country = setup_test_country().await;
    country_repo.save(country.clone()).await?;

    let subdivision = setup_test_country_subdivision(country.id).await;
    country_subdivision_repo.save(subdivision.clone()).await?;

    let mut localities = Vec::new();
    let mut ids = Vec::new();
    for i in 0..5 {
        let mut locality = setup_test_locality(subdivision.id).await;
        locality.code = HeaplessString::try_from(format!("CD{i:03}").as_str()).unwrap();
        locality.name_l1 = HeaplessString::try_from(format!("Test Locality {i}").as_str()).unwrap();
        ids.push(locality.id);
        localities.push(locality);
    }

    let audit_log_id = Uuid::new_v4();
    locality_repo
        .create_batch(localities.clone(), audit_log_id)
        .await?;

    let loaded_localities = locality_repo.load_batch(&ids).await?;
    assert_eq!(loaded_localities.len(), 5);
    for (i, locality) in loaded_localities.iter().enumerate() {
        assert!(locality.is_some());
        assert_eq!(locality.as_ref().unwrap().id, localities[i].id);
    }

    Ok(())
}
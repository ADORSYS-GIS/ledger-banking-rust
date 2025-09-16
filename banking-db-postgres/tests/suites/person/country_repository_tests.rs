use banking_db::repository::{CountryRepository, PersonRepos};
use uuid::Uuid;

use crate::suites::test_helper::setup_test_context;
use crate::suites::person::helpers::create_test_country_model;

#[tokio::test]
async fn test_country_repository() {
    let ctx = setup_test_context().await.unwrap();
    let repo = ctx.person_repos().countries();

    // Test save and find_by_id
    // Use unique ISO2 codes for each test to avoid conflicts
    let unique_iso2 = format!("T{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
    let new_country = create_test_country_model(&unique_iso2, "Test Country 1");
    let saved_country = repo.save(new_country.clone()).await.unwrap();
    assert_eq!(new_country.id, saved_country.id);

    let found_country = repo.find_by_id(new_country.id).await.unwrap().unwrap();
    assert_eq!(new_country.id, found_country.country_id);

    // Test exists_by_id
    let found_countries = repo.find_by_ids(&[new_country.id]).await.unwrap();
    assert_eq!(found_countries.len(), 1);

    // Test find_by_ids
    let unique_iso2_2 = format!("U{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
    let new_country2 = create_test_country_model(&unique_iso2_2, "Test Country 2");
    repo.save(new_country2.clone()).await.unwrap();
    let ids = vec![new_country.id, new_country2.id];
    let found_countries = repo.find_by_ids(&ids).await.unwrap();
    assert_eq!(found_countries.len(), 2);

    // Test exists_by_id
    assert!(repo.exists_by_id(new_country.id).await.unwrap());
    assert!(!repo.exists_by_id(Uuid::new_v4()).await.unwrap());

    // Test find_by_iso2
    let found_by_iso2 = repo.find_by_iso2(&unique_iso2_2, 1, 10).await.unwrap();
    assert!(!found_by_iso2.is_empty());
}
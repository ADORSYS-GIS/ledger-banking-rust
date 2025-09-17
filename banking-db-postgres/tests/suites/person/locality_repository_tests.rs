use banking_db::repository::{CountryRepository, CountrySubdivisionRepository, LocalityRepository, PersonRepos};
use uuid::Uuid;

use crate::suites::test_helper::setup_test_context;
use crate::suites::person::helpers::{
    create_test_country_model, create_test_country_subdivision_model, create_test_locality_model,
};

#[tokio::test]
async fn test_locality_repository() {
    let ctx = setup_test_context().await.unwrap();
    let country_repo = ctx.person_repos().countries();
    let country_subdivision_repo = ctx.person_repos().country_subdivisions();
    let repo = ctx.person_repos().localities();
    
    // Use unique codes for test isolation
    let unique_iso2 = format!("L{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
    let country = create_test_country_model(&unique_iso2, "Test Country");
    country_repo.save(country.clone()).await.unwrap();
    
    let unique_subdivision_code = format!("LS{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
    let country_subdivision = create_test_country_subdivision_model(country.id, &unique_subdivision_code, "Test Subdivision");
    country_subdivision_repo
        .save(country_subdivision.clone())
        .await
        .unwrap();

    // Test save and find_by_id
    let unique_locality_code = format!("LC{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
    let new_locality = create_test_locality_model(country_subdivision.id, &unique_locality_code, "Test Locality");
    let saved_locality = repo.save(new_locality.clone()).await.unwrap();
    assert_eq!(new_locality.id, saved_locality.id);

    let found_locality = repo.find_by_id(new_locality.id).await.unwrap().unwrap();
    assert_eq!(new_locality.id, found_locality.locality_id);

    // Test find_by_country_subdivision_id
    let localities_in_country_subdivision = repo
        .find_by_country_subdivision_id(country_subdivision.id, 1, 10)
        .await
        .unwrap();
    assert_eq!(localities_in_country_subdivision.len(), 1);
}
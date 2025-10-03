use banking_db::repository::{
    CountryRepository, CountrySubdivisionRepository, LocalityRepository, LocationRepository, PersonRepos,
};
use uuid::Uuid;

use banking_db_postgres::test_helper::setup_test_context;
use banking_db_postgres::repository::person::test_helpers::{
    create_test_country_model, create_test_country_subdivision_model, create_test_locality_model,
    create_test_location_model,
};

#[tokio::test]
async fn test_location_repository() {
    let ctx = setup_test_context().await.unwrap();
    let country_repo = ctx.person_repos().countries();
    let country_subdivision_repo = ctx.person_repos().country_subdivisions();
    let locality_repo = ctx.person_repos().localities();
    let repo = ctx.person_repos().locations();
    
    // Use unique codes for test isolation
    let unique_iso2 = format!("O{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
    let country = create_test_country_model(&unique_iso2, "Test Country");
    country_repo.save(country.clone()).await.unwrap();
    
    let unique_subdivision_code = format!("OS{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
    let country_subdivision = create_test_country_subdivision_model(country.id, &unique_subdivision_code, "Test Subdivision");
    country_subdivision_repo
        .save(country_subdivision.clone())
        .await
        .unwrap();
    
    let unique_locality_code = format!("OL{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
    let locality = create_test_locality_model(country_subdivision.id, &unique_locality_code, "Test Locality");
    locality_repo.save(locality.clone()).await.unwrap();

    // Test save and find_by_id
    let new_location = create_test_location_model(locality.id, "Test Street", "12345");
    let audit_log_id = Uuid::new_v4();
    let saved_location = repo.save(new_location.clone(), audit_log_id).await.unwrap();
    assert_eq!(new_location.id, saved_location.id);

    let found_location = repo.find_by_id(new_location.id).await.unwrap().unwrap();
    assert_eq!(new_location.id, found_location.location_id);

    // Test find_by_locality_id
    let locations_in_locality = repo.find_by_locality_id(locality.id, 1, 10).await.unwrap();
    assert_eq!(locations_in_locality.len(), 1);
}
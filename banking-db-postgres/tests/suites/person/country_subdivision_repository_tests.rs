use banking_db::repository::{CountryRepository, CountrySubdivisionRepository, PersonRepos};
use uuid::Uuid;

use crate::suites::test_helper::setup_test_context;
use crate::suites::person::helpers::{
    create_test_country_model, create_test_country_subdivision_model,
};

#[tokio::test]
async fn test_country_subdivision_repository() {
    let ctx = setup_test_context().await.unwrap();
    let country_repo = ctx.person_repos().countries();
    let repo = ctx.person_repos().country_subdivisions();
    
    // Use unique ISO2 codes for test isolation
    let unique_iso2 = format!("C{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
    let country = create_test_country_model(&unique_iso2, "Test Country");
    country_repo.save(country.clone()).await.unwrap();

    // Test save and find_by_id
    let unique_code = format!("S{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
    let new_country_subdivision = create_test_country_subdivision_model(country.id, &unique_code, "Test Subdivision");
    let saved_country_subdivision = repo.save(new_country_subdivision.clone()).await.unwrap();
    assert_eq!(new_country_subdivision.id, saved_country_subdivision.id);

    let found_country_subdivision = repo
        .find_by_id(new_country_subdivision.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        new_country_subdivision.id,
        found_country_subdivision.country_subdivision_id
    );

    // Test find_by_country_id
    let country_subdivisions_in_country =
        repo.find_by_country_id(country.id, 1, 10).await.unwrap();
    assert_eq!(country_subdivisions_in_country.len(), 1);
}
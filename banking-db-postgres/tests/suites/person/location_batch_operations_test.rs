use banking_db::models::person::{LocationModel, LocationType};
use banking_db::repository::{
    BatchRepository, CountryRepository, CountrySubdivisionRepository, LocationRepository,
    LocalityRepository, PersonRepos,
};
use heapless::String as HeaplessString;
use uuid::Uuid;
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::suites::person::helpers::setup_test_country;
use crate::suites::person::country_subdivision_batch_operations_test::setup_test_country_subdivision;
use crate::suites::person::locality_batch_operations_test::setup_test_locality;
use crate::suites::test_helper::setup_test_context;

pub async fn setup_test_location(locality_id: Uuid) -> LocationModel {
    LocationModel {
        id: Uuid::new_v4(),
        locality_id,
        street_line1: HeaplessString::try_from("123 Main St").unwrap(),
        street_line2: None,
        street_line3: None,
        street_line4: None,
        postal_code: Some(HeaplessString::try_from("12345").unwrap()),
        latitude: Some(Decimal::from_str("34.0522").unwrap()),
        longitude: Some(Decimal::from_str("-118.2437").unwrap()),
        accuracy_meters: Some(10.0),
        location_type: LocationType::Residential,
    }
}

#[tokio::test]
async fn test_create_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = setup_test_context().await?;
    let person_repos = ctx.person_repos();
    let country_repo = person_repos.countries();
    let subdivision_repo = person_repos.country_subdivisions();
    let locality_repo = person_repos.localities();
    let location_repo = person_repos.locations();

    let country = setup_test_country().await;
    country_repo.save(country.clone()).await?;

    let subdivision = setup_test_country_subdivision(country.id).await;
    subdivision_repo
        .save(subdivision.clone())
        .await?;

    let locality = setup_test_locality(subdivision.id).await;
    locality_repo.save(locality.clone()).await?;

    let mut locations = Vec::new();
    for i in 0..5 {
        let mut location = setup_test_location(locality.id).await;
        location.street_line1 =
            HeaplessString::try_from(format!("Street {i}").as_str()).unwrap();
        locations.push(location);
    }

    let audit_log_id = Uuid::new_v4();

    let saved_locations = location_repo
        .create_batch(locations.clone(), audit_log_id)
        .await?;

    assert_eq!(saved_locations.len(), 5);

    for location in &saved_locations {
        assert!(location_repo.exists_by_id(location.id).await?);
    }

    Ok(())
}

#[tokio::test]
async fn test_load_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = setup_test_context().await?;
    let person_repos = ctx.person_repos();
    let country_repo = person_repos.countries();
    let subdivision_repo = person_repos.country_subdivisions();
    let locality_repo = person_repos.localities();
    let location_repo = person_repos.locations();

    let country = setup_test_country().await;
    country_repo.save(country.clone()).await?;

    let subdivision = setup_test_country_subdivision(country.id).await;
    subdivision_repo
        .save(subdivision.clone())
        .await?;

    let locality = setup_test_locality(subdivision.id).await;
    locality_repo.save(locality.clone()).await?;

    let mut locations = Vec::new();
    for i in 0..5 {
        let mut location = setup_test_location(locality.id).await;
        location.street_line1 =
            HeaplessString::try_from(format!("Street {i}").as_str()).unwrap();
        locations.push(location);
    }

    let audit_log_id = Uuid::new_v4();

    let saved_locations = location_repo
        .create_batch(locations.clone(), audit_log_id)
        .await?;
    assert_eq!(saved_locations.len(), 5);

    let ids: Vec<Uuid> = saved_locations.iter().map(|l| l.id).collect();
    let loaded_locations = location_repo.load_batch(&ids).await?;

    assert_eq!(loaded_locations.len(), 5);
    for (i, location) in loaded_locations.iter().enumerate() {
        assert_eq!(location.as_ref().unwrap().id, saved_locations[i].id);
    }

    Ok(())
}
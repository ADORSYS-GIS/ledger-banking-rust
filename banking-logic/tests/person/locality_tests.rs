use crate::person::mock_country_repository::create_test_country;
use crate::person::mock_country_subdivision_repository::create_test_country_subdivision;
use crate::person::mock_locality_repository::create_test_locality;
use crate::person::common::create_test_services;
use banking_api::service::{CountryService, CountrySubdivisionService, LocalityService};

#[tokio::test]
async fn test_create_locality() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    services
        .mock_country_subdivision_repository
        .valid_country_ids
        .lock()
        .unwrap()
        .insert(country.id);
    let country_subdivision = create_test_country_subdivision(country.id);
    services
        .country_subdivision_service
        .create_country_subdivision(country_subdivision.clone())
        .await
        .unwrap();
    let locality = create_test_locality(country_subdivision.id);
    let created_locality = services
        .locality_service
        .create_locality(locality.clone())
        .await
        .unwrap();
    assert_eq!(locality.id, created_locality.id);
}

#[tokio::test]
async fn test_find_locality_by_id() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    services
        .mock_country_subdivision_repository
        .valid_country_ids
        .lock()
        .unwrap()
        .insert(country.id);
    let country_subdivision = create_test_country_subdivision(country.id);
    services
        .country_subdivision_service
        .create_country_subdivision(country_subdivision.clone())
        .await
        .unwrap();
    let locality = create_test_locality(country_subdivision.id);
    services
        .locality_service
        .create_locality(locality.clone())
        .await
        .unwrap();
    let found_locality = services
        .locality_service
        .find_locality_by_id(locality.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(locality.id, found_locality.id);
}

#[tokio::test]
async fn test_find_localities_by_country_subdivision_id() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    services
        .mock_country_subdivision_repository
        .valid_country_ids
        .lock()
        .unwrap()
        .insert(country.id);
    let country_subdivision = create_test_country_subdivision(country.id);
    services
        .country_subdivision_service
        .create_country_subdivision(country_subdivision.clone())
        .await
        .unwrap();
    let locality = create_test_locality(country_subdivision.id);
    services
        .locality_service
        .create_locality(locality.clone())
        .await
        .unwrap();
    let localities = services
        .locality_service
        .find_localities_by_country_subdivision_id(country_subdivision.id)
        .await
        .unwrap();
    assert!(!localities.is_empty());
}

#[tokio::test]
async fn test_find_locality_by_code() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    services
        .mock_country_subdivision_repository
        .valid_country_ids
        .lock()
        .unwrap()
        .insert(country.id);
    let country_subdivision = create_test_country_subdivision(country.id);
    services
        .country_subdivision_service
        .create_country_subdivision(country_subdivision.clone())
        .await
        .unwrap();
    let locality = create_test_locality(country_subdivision.id);
    services
        .locality_service
        .create_locality(locality.clone())
        .await
        .unwrap();
    let found_locality = services
        .locality_service
        .find_locality_by_code(country_subdivision.id, locality.code.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(locality.id, found_locality.id);
}
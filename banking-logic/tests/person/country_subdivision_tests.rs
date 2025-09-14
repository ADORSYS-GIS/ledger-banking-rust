use crate::person::mock_country_repository::create_test_country;
use crate::person::mock_country_subdivision_repository::create_test_country_subdivision;
use crate::person::common::create_test_services;
use banking_api::service::{CountryService, CountrySubdivisionService};

#[tokio::test]
async fn test_create_country_subdivision() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    let created_country_subdivision = services
        .country_subdivision_service
        .create_country_subdivision(country_subdivision.clone())
        .await
        .unwrap();
    assert_eq!(country_subdivision.id, created_country_subdivision.id);
}

#[tokio::test]
async fn test_find_country_subdivision_by_id() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    services
        .country_subdivision_service
        .create_country_subdivision(country_subdivision.clone())
        .await
        .unwrap();
    let found_country_subdivision = services
        .country_subdivision_service
        .find_country_subdivision_by_id(country_subdivision.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(country_subdivision.id, found_country_subdivision.id);
}

#[tokio::test]
async fn test_find_country_subdivisions_by_country_id() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    services
        .country_subdivision_service
        .create_country_subdivision(country_subdivision.clone())
        .await
        .unwrap();
    let country_subdivisions = services
        .country_subdivision_service
        .find_country_subdivisions_by_country_id(country.id)
        .await
        .unwrap();
    assert!(!country_subdivisions.is_empty());
}

#[tokio::test]
async fn test_find_country_subdivision_by_code() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    services
        .country_subdivision_service
        .create_country_subdivision(country_subdivision.clone())
        .await
        .unwrap();
    let found_country_subdivision = services
        .country_subdivision_service
        .find_country_subdivision_by_code(country.id, country_subdivision.code.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(country_subdivision.id, found_country_subdivision.id);
}
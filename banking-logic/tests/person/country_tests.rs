use crate::person::mock_country_repository::create_test_country;
use banking_api::service::CountryService;
use crate::person::common::create_test_services;

#[tokio::test]
async fn test_create_country() {
    let services = create_test_services();
    let country = create_test_country();
    let created_country = services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    assert_eq!(country.id, created_country.id);
}

#[tokio::test]
async fn test_find_country_by_id() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let found_country = services
        .country_service
        .find_country_by_id(country.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(country.id, found_country.id);
}

#[tokio::test]
async fn test_find_country_by_iso2() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let found_country = services
        .country_service
        .find_country_by_iso2(country.iso2.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(country.id, found_country.id);
}

#[tokio::test]
async fn test_get_all_countries() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let countries = services
        .country_service
        .get_all_countries()
        .await
        .unwrap();
    assert!(countries.is_empty());
}
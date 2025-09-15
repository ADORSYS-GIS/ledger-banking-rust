use crate::person::mock_country_repository::create_test_country;
use crate::person::mock_country_subdivision_repository::create_test_country_subdivision;
use crate::person::mock_locality_repository::create_test_locality;
use crate::person::mock_location_repository::create_test_location;
use crate::person::common::{create_test_audit_log, create_test_services};
use banking_api::service::{CountryService, CountrySubdivisionService, LocalityService, LocationService};

#[tokio::test]
async fn test_create_location() {
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
    let location = create_test_location(locality.id);
    let audit_log = create_test_audit_log();
    let created_location = services
        .location_service
        .create_location(location.clone(), audit_log)
        .await
        .unwrap();
    assert_eq!(location.id, created_location.id);
}

#[tokio::test]
async fn test_find_location_by_id() {
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
    let location = create_test_location(locality.id);
    services
        .location_service
        .create_location(location.clone(), create_test_audit_log())
        .await
        .unwrap();
    let found_location = services
        .location_service
        .find_location_by_id(location.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(location.id, found_location.id);
}

#[tokio::test]
async fn test_find_locations_by_locality_id() {
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
    let location = create_test_location(locality.id);
    services
        .location_service
        .create_location(location.clone(), create_test_audit_log())
        .await
        .unwrap();
    let locations = services
        .location_service
        .find_locations_by_locality_id(locality.id)
        .await
        .unwrap();
    assert!(!locations.is_empty());
}
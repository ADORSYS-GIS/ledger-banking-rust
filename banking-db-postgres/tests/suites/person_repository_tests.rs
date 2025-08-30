use banking_db::models::person::{
    LocationModel, LocationType, LocalityModel, CountryModel, MessagingModel, MessagingType, PersonModel,
    PersonType, CountrySubdivisionModel,
};
use banking_db::repository::{
    LocationRepository, LocalityRepository, CountryRepository, MessagingRepository, PersonRepository,
    CountrySubdivisionRepository,
};
use banking_db_postgres::repository::{
    person_country_repository_impl::CountryRepositoryImpl,
    person_country_subdivision_repository_impl::CountrySubdivisionRepositoryImpl,
    person_locality_repository_impl::LocalityRepositoryImpl,
    person_location_repository_impl::LocationRepositoryImpl,
    person_messaging_repository_impl::MessagingRepositoryImpl,
    person_person_repository_impl::PersonRepositoryImpl,
};
use heapless::String as HeaplessString;
use std::sync::Arc;
use uuid::Uuid;

use super::commons::commons;

// Helper functions for creating test models

fn create_test_person_model() -> PersonModel {
    PersonModel {
        id: Uuid::new_v4(),
        person_type: PersonType::Natural,
        display_name: HeaplessString::try_from("John Doe").unwrap(),
        external_identifier: Some(
            HeaplessString::try_from(Uuid::new_v4().to_string().as_str()).unwrap(),
        ),
        entity_reference_count: 0,
        organization_person_id: None,
        messaging1_id: None,
        messaging1_type: None,
        messaging2_id: None,
        messaging2_type: None,
        messaging3_id: None,
        messaging3_type: None,
        messaging4_id: None,
        messaging4_type: None,
        messaging5_id: None,
        messaging5_type: None,
        department: None,
        location_id: None,
        duplicate_of_person_id: None,
    }
}

fn create_test_country_model() -> CountryModel {
    CountryModel {
        id: Uuid::new_v4(),
        iso2: HeaplessString::try_from("US").unwrap(),
        name_l1: HeaplessString::try_from("United States").unwrap(),
        name_l2: None,
        name_l3: None,
    }
}

fn create_test_country_subdivision_model(country_id: Uuid) -> CountrySubdivisionModel {
    CountrySubdivisionModel {
        id: Uuid::new_v4(),
        country_id,
        code: HeaplessString::try_from("CA").unwrap(),
        name_l1: HeaplessString::try_from("California").unwrap(),
        name_l2: None,
        name_l3: None,
    }
}

fn create_test_locality_model(country_subdivision_id: Uuid) -> LocalityModel {
    LocalityModel {
        id: Uuid::new_v4(),
        country_subdivision_id,
        code: HeaplessString::try_from("LA").unwrap(),
        name_l1: HeaplessString::try_from("Los Angeles").unwrap(),
        name_l2: None,
        name_l3: None,
    }
}

fn create_test_location_model(locality_id: Uuid) -> LocationModel {
    LocationModel {
        id: Uuid::new_v4(),
        location_type: LocationType::Residential,
        street_line1: HeaplessString::try_from(format!("123 Main St {}", Uuid::new_v4()).as_str())
            .unwrap(),
        street_line2: None,
        street_line3: None,
        street_line4: None,
        locality_id,
        postal_code: Some(HeaplessString::try_from("90210").unwrap()),
        latitude: None,
        longitude: None,
        accuracy_meters: None,
    }
}

fn create_test_messaging_model() -> MessagingModel {
    MessagingModel {
        id: Uuid::new_v4(),
        messaging_type: MessagingType::Email,
        value: HeaplessString::try_from(format!("test_{}@example.com", Uuid::new_v4()).as_str()).unwrap(),
        other_type: None,
    }
}

#[tokio::test]
async fn test_person_repository() {
    let db_pool = commons::establish_connection().await;
    commons::cleanup_database(&db_pool).await;
    let repo = PersonRepositoryImpl::new(Arc::new(db_pool.clone())).await;

    // Test save and find_by_id
    let audit_log_id =  Uuid::new_v4();

    let new_person = create_test_person_model();
    let saved_person = repo.save(new_person.clone(), audit_log_id).await.unwrap();
    assert_eq!(new_person.id, saved_person.id);

    let found_person_idx = repo.find_by_id(new_person.id).await.unwrap().unwrap();
    assert_eq!(new_person.id, found_person_idx.person_id);

    // Test exists_by_id
    assert!(repo.exists_by_id(new_person.id).await.unwrap());
    assert!(!repo.exists_by_id(Uuid::new_v4()).await.unwrap());

    // Test find_by_ids
    let new_person2 = create_test_person_model();
    let audit_log_id =  Uuid::new_v4();
    repo.save(new_person2.clone(), audit_log_id).await.unwrap();
    let ids = vec![new_person.id, new_person2.id];
    let found_persons = repo.find_by_ids(&ids).await.unwrap();
    assert_eq!(found_persons.len(), 2);

    // Test get_by_external_identifier
    let found_by_ext_id = repo
        .get_by_external_identifier(new_person.external_identifier.as_ref().unwrap().as_str())
        .await
        .unwrap();
    assert_eq!(found_by_ext_id.len(), 1);

}

#[tokio::test]
async fn test_country_repository() {
    let db_pool = commons::establish_connection().await;
    commons::cleanup_database(&db_pool).await;
    let repo = CountryRepositoryImpl::new(Arc::new(db_pool.clone()));

    // Test save and find_by_id
    let new_country = create_test_country_model();
    let saved_country = repo.save(new_country.clone()).await.unwrap();
    assert_eq!(new_country.id, saved_country.id);

    let found_country = repo.find_by_id(new_country.id).await.unwrap().unwrap();
    assert_eq!(new_country.id, found_country.country_id);

    // Test exists_by_id
    let found_countries = repo.find_by_ids(&[new_country.id]).await.unwrap();
    assert_eq!(found_countries.len(), 1);

    // Test find_by_ids
    let new_country2 = create_test_country_model();
    repo.save(new_country2.clone()).await.unwrap();
    let ids = vec![new_country.id, new_country2.id];
    let found_countries = repo.find_by_ids(&ids).await.unwrap();
    assert_eq!(found_countries.len(), 2);

    // Test exists_by_id
    assert!(repo.exists_by_id(new_country.id).await.unwrap());
    assert!(!repo.exists_by_id(Uuid::new_v4()).await.unwrap());

    // Test find_by_iso2
    let found_by_iso2 = repo.find_by_iso2("US", 1, 10).await.unwrap();
    assert!(!found_by_iso2.is_empty());
}

#[tokio::test]
async fn test_country_subdivision_repository() {
    let db_pool = commons::establish_connection().await;
    commons::cleanup_database(&db_pool).await;
    let country_repo = CountryRepositoryImpl::new(Arc::new(db_pool.clone()));
    let country = create_test_country_model();
    country_repo.save(country.clone()).await.unwrap();
    let repo = CountrySubdivisionRepositoryImpl::new(Arc::new(db_pool.clone()));

    // Test save and find_by_id
    let new_country_subdivision = create_test_country_subdivision_model(country.id);
    let saved_country_subdivision = repo.save(new_country_subdivision.clone()).await.unwrap();
    assert_eq!(new_country_subdivision.id, saved_country_subdivision.id);

    let found_country_subdivision = repo.find_by_id(new_country_subdivision.id).await.unwrap().unwrap();
    assert_eq!(new_country_subdivision.id, found_country_subdivision.country_subdivision_id);

    // Test find_by_country_id
    let country_subdivisions_in_country = repo.find_by_country_id(country.id, 1, 10).await.unwrap();
    assert_eq!(country_subdivisions_in_country.len(), 1);
}

#[tokio::test]
async fn test_locality_repository() {
    let db_pool = commons::establish_connection().await;
    commons::cleanup_database(&db_pool).await;
    let country_repo = CountryRepositoryImpl::new(Arc::new(db_pool.clone()));
    let country = create_test_country_model();
    country_repo.save(country.clone()).await.unwrap();
    let country_subdivision_repo = CountrySubdivisionRepositoryImpl::new(Arc::new(db_pool.clone()));
    let country_subdivision = create_test_country_subdivision_model(country.id);
    country_subdivision_repo.save(country_subdivision.clone()).await.unwrap();
    let repo = LocalityRepositoryImpl::new(Arc::new(db_pool.clone()));

    // Test save and find_by_id
    let new_locality = create_test_locality_model(country_subdivision.id);
    let saved_locality = repo.save(new_locality.clone()).await.unwrap();
    assert_eq!(new_locality.id, saved_locality.id);

    let found_locality = repo.find_by_id(new_locality.id).await.unwrap().unwrap();
    assert_eq!(new_locality.id, found_locality.locality_id);

    // Test find_by_country_subdivision_id
    let localities_in_country_subdivision = repo.find_by_country_subdivision_id(country_subdivision.id, 1, 10).await.unwrap();
    assert_eq!(localities_in_country_subdivision.len(), 1);
}

#[tokio::test]
async fn test_location_repository() {
    let db_pool = commons::establish_connection().await;
    commons::cleanup_database(&db_pool).await;
    let country_repo = CountryRepositoryImpl::new(Arc::new(db_pool.clone()));
    let country = create_test_country_model();
    country_repo.save(country.clone()).await.unwrap();
    let country_subdivision_repo = CountrySubdivisionRepositoryImpl::new(Arc::new(db_pool.clone()));
    let country_subdivision = create_test_country_subdivision_model(country.id);
    country_subdivision_repo.save(country_subdivision.clone()).await.unwrap();
    let locality_repo = LocalityRepositoryImpl::new(Arc::new(db_pool.clone()));
    let locality = create_test_locality_model(country_subdivision.id);
    locality_repo.save(locality.clone()).await.unwrap();
    let repo = LocationRepositoryImpl::new(Arc::new(db_pool.clone())).await;

    // Test save and find_by_id
    let new_location = create_test_location_model(locality.id);
    let audit_log_id =  Uuid::new_v4();
    let saved_location = repo.save(new_location.clone(), audit_log_id).await.unwrap();
    assert_eq!(new_location.id, saved_location.id);

    let found_location = repo.find_by_id(new_location.id).await.unwrap().unwrap();
    assert_eq!(new_location.id, found_location.location_id);

    // Test find_by_locality_id
    let locations_in_locality = repo.find_by_locality_id(locality.id, 1, 10).await.unwrap();
    assert_eq!(locations_in_locality.len(), 1);
}

#[tokio::test]
async fn test_messaging_repository() {
    let db_pool = commons::establish_connection().await;
    commons::cleanup_database(&db_pool).await;
    let repo = MessagingRepositoryImpl::new(Arc::new(db_pool.clone())).await;

    // Test save and find_by_id
    let new_messaging = create_test_messaging_model();
    let audit_log_id =  Uuid::new_v4();

    let saved_messaging = repo.save(new_messaging.clone(), audit_log_id).await.unwrap();
    assert_eq!(new_messaging.id, saved_messaging.id);

    let found_messaging_idx = repo.find_by_id(new_messaging.id).await.unwrap().unwrap();
    assert_eq!(new_messaging.id, found_messaging_idx.messaging_id);

    // Test find_ids_by_value
    let ids = repo
        .find_ids_by_value(new_messaging.value.as_str())
        .await
        .unwrap();
    assert_eq!(ids.len(), 1);
}
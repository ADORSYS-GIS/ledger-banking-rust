use banking_db::models::person::{
    AddressModel, AddressType, CityModel, CountryModel, MessagingModel, MessagingType, PersonModel,
    PersonType, StateProvinceModel,
};
use banking_db::repository::{
    AddressRepository, CityRepository, CountryRepository, MessagingRepository, PersonRepository,
    StateProvinceRepository,
};
use banking_db_postgres::repository::person_repository_impl::{
    AddressRepositoryImpl, CityRepositoryImpl, CountryRepositoryImpl, MessagingRepositoryImpl,
    PersonRepositoryImpl, StateProvinceRepositoryImpl,
};
use chrono::Utc;
use heapless::String as HeaplessString;
use std::sync::Arc;
use uuid::Uuid;

use super::commons;

// Helper functions for creating test models

fn create_test_person_model() -> PersonModel {
    PersonModel {
        id: Uuid::new_v4(),
        person_type: PersonType::Natural,
        display_name: HeaplessString::try_from("John Doe").unwrap(),
        external_identifier: Some(
            HeaplessString::try_from(Uuid::new_v4().to_string().as_str()).unwrap(),
        ),
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
        location_address_id: None,
        duplicate_of_person_id: None,
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn create_test_country_model(created_by: Uuid) -> CountryModel {
    CountryModel {
        id: Uuid::new_v4(),
        iso2: HeaplessString::try_from("US").unwrap(),
        name_l1: HeaplessString::try_from("United States").unwrap(),
        name_l2: None,
        name_l3: None,
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        created_by_person_id: created_by,
        updated_by_person_id: created_by,
    }
}

fn create_test_state_model(country_id: Uuid, created_by: Uuid) -> StateProvinceModel {
    StateProvinceModel {
        id: Uuid::new_v4(),
        country_id,
        state_province_code: HeaplessString::try_from("CA").unwrap(),
        name_l1: HeaplessString::try_from("California").unwrap(),
        name_l2: None,
        name_l3: None,
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        created_by_person_id: created_by,
        updated_by_person_id: created_by,
    }
}

fn create_test_city_model(country_id: Uuid, state_id: Uuid, created_by: Uuid) -> CityModel {
    CityModel {
        id: Uuid::new_v4(),
        country_id,
        state_id: Some(state_id),
        city_code: HeaplessString::try_from("LA").unwrap(),
        name_l1: HeaplessString::try_from("Los Angeles").unwrap(),
        name_l2: None,
        name_l3: None,
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        created_by_person_id: created_by,
        updated_by_person_id: created_by,
    }
}

fn create_test_address_model(city_id: Uuid, created_by: Uuid) -> AddressModel {
    AddressModel {
        id: Uuid::new_v4(),
        address_type: AddressType::Residential,
        street_line1: HeaplessString::try_from(format!("123 Main St {}", Uuid::new_v4()).as_str()).unwrap(),
        street_line2: None,
        street_line3: None,
        street_line4: None,
        city_id,
        postal_code: Some(HeaplessString::try_from("90210").unwrap()),
        latitude: None,
        longitude: None,
        accuracy_meters: None,
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        created_by_person_id: created_by,
        updated_by_person_id: created_by,
    }
}

fn create_test_messaging_model() -> MessagingModel {
    MessagingModel {
        id: Uuid::new_v4(),
        messaging_type: MessagingType::Email,
        value: HeaplessString::try_from(format!("test_{}@example.com", Uuid::new_v4()).as_str()).unwrap(),
        other_type: None,
        is_active: true,
        priority: Some(1),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[tokio::test]
async fn test_person_repository() {
    let db_pool = commons::establish_connection().await;
    commons::cleanup_database(&db_pool).await;
    let repo = PersonRepositoryImpl::new(Arc::new(db_pool.clone()));

    // Test save and find_by_id
    let new_person = create_test_person_model();
    let saved_person = repo.save(new_person.clone()).await.unwrap();
    assert_eq!(new_person.id, saved_person.id);

    let found_person = repo.find_by_id(new_person.id).await.unwrap().unwrap();
    assert_eq!(new_person.id, found_person.id);

    // Test exists_by_id
    assert!(repo.exists_by_id(new_person.id).await.unwrap());
    assert!(!repo.exists_by_id(Uuid::new_v4()).await.unwrap());

    // Test find_by_ids
    let new_person2 = create_test_person_model();
    repo.save(new_person2.clone()).await.unwrap();
    let ids = vec![new_person.id, new_person2.id];
    let found_persons = repo.find_by_ids(&ids).await.unwrap();
    assert_eq!(found_persons.len(), 2);

    // Test find_by_person_type
    let natural_persons = repo
        .find_by_person_type(PersonType::Natural, 1, 10)
        .await
        .unwrap();
    assert!(!natural_persons.is_empty());

    // Test find_by_is_active
    let active_persons = repo.find_by_is_active(true, 1, 10).await.unwrap();
    assert!(!active_persons.is_empty());

    // Test get_by_external_identifier
    let found_by_ext_id = repo
        .get_by_external_identifier(new_person.external_identifier.as_ref().unwrap())
        .await
        .unwrap();
    assert_eq!(found_by_ext_id.len(), 1);

    // Test search_by_name
    let found_by_name = repo.search_by_name("John Doe").await.unwrap();
    assert!(!found_by_name.is_empty());

    // Test mark_as_duplicate
    let duplicate_person = create_test_person_model();
    repo.save(duplicate_person.clone()).await.unwrap();
    repo.mark_as_duplicate(duplicate_person.id, new_person.id)
        .await
        .unwrap();
    let fetched_duplicate = repo.find_by_id(duplicate_person.id).await.unwrap().unwrap();
    assert!(!fetched_duplicate.is_active);
    assert_eq!(fetched_duplicate.duplicate_of_person_id, Some(new_person.id));
}

#[tokio::test]
async fn test_country_repository() {
    let db_pool = commons::establish_connection().await;
    commons::cleanup_database(&db_pool).await;
    let person_id = commons::create_test_person(&db_pool).await;
    let repo = CountryRepositoryImpl::new(Arc::new(db_pool.clone()));

    // Test save and find_by_id
    let new_country = create_test_country_model(person_id);
    let saved_country = repo.save(new_country.clone()).await.unwrap();
    assert_eq!(new_country.id, saved_country.id);

    let found_country = repo.find_by_id(new_country.id).await.unwrap().unwrap();
    assert_eq!(new_country.id, found_country.id);

    // Test exists_by_id
    assert!(repo.exists_by_id(new_country.id).await.unwrap());
    assert!(!repo.exists_by_id(Uuid::new_v4()).await.unwrap());

    // Test find_by_ids
    let new_country2 = create_test_country_model(person_id);
    repo.save(new_country2.clone()).await.unwrap();
    let ids = vec![new_country.id, new_country2.id];
    let found_countries = repo.find_by_ids(&ids).await.unwrap();
    assert_eq!(found_countries.len(), 2);

    // Test find_by_iso2
    let found_by_iso2 = repo.find_by_iso2("US", 1, 10).await.unwrap();
    assert!(!found_by_iso2.is_empty());

    // Test find_by_is_active
    let active_countries = repo.find_by_is_active(true, 1, 10).await.unwrap();
    assert!(!active_countries.is_empty());
}

#[tokio::test]
async fn test_state_province_repository() {
    let db_pool = commons::establish_connection().await;
    commons::cleanup_database(&db_pool).await;
    let person_id = commons::create_test_person(&db_pool).await;
    let country_repo = CountryRepositoryImpl::new(Arc::new(db_pool.clone()));
    let country = create_test_country_model(person_id);
    country_repo.save(country.clone()).await.unwrap();
    let repo = StateProvinceRepositoryImpl::new(Arc::new(db_pool.clone()));

    // Test save and find_by_id
    let new_state = create_test_state_model(country.id, person_id);
    let saved_state = repo.save(new_state.clone()).await.unwrap();
    assert_eq!(new_state.id, saved_state.id);

    let found_state = repo.find_by_id(new_state.id).await.unwrap().unwrap();
    assert_eq!(new_state.id, found_state.id);

    // Test find_by_country_id
    let states_in_country = repo.find_by_country_id(country.id, 1, 10).await.unwrap();
    assert_eq!(states_in_country.len(), 1);
}

#[tokio::test]
async fn test_city_repository() {
    let db_pool = commons::establish_connection().await;
    commons::cleanup_database(&db_pool).await;
    let person_id = commons::create_test_person(&db_pool).await;
    let country_repo = CountryRepositoryImpl::new(Arc::new(db_pool.clone()));
    let country = create_test_country_model(person_id);
    country_repo.save(country.clone()).await.unwrap();
    let state_repo = StateProvinceRepositoryImpl::new(Arc::new(db_pool.clone()));
    let state = create_test_state_model(country.id, person_id);
    state_repo.save(state.clone()).await.unwrap();
    let repo = CityRepositoryImpl::new(Arc::new(db_pool.clone()));

    // Test save and find_by_id
    let new_city = create_test_city_model(country.id, state.id, person_id);
    let saved_city = repo.save(new_city.clone()).await.unwrap();
    assert_eq!(new_city.id, saved_city.id);

    let found_city = repo.find_by_id(new_city.id).await.unwrap().unwrap();
    assert_eq!(new_city.id, found_city.id);

    // Test find_by_state_id
    let cities_in_state = repo.find_by_state_id(state.id, 1, 10).await.unwrap();
    assert_eq!(cities_in_state.len(), 1);
}

#[tokio::test]
async fn test_address_repository() {
    let db_pool = commons::establish_connection().await;
    commons::cleanup_database(&db_pool).await;
    let person_id = commons::create_test_person(&db_pool).await;
    let country_repo = CountryRepositoryImpl::new(Arc::new(db_pool.clone()));
    let country = create_test_country_model(person_id);
    country_repo.save(country.clone()).await.unwrap();
    let state_repo = StateProvinceRepositoryImpl::new(Arc::new(db_pool.clone()));
    let state = create_test_state_model(country.id, person_id);
    state_repo.save(state.clone()).await.unwrap();
    let city_repo = CityRepositoryImpl::new(Arc::new(db_pool.clone()));
    let city = create_test_city_model(country.id, state.id, person_id);
    city_repo.save(city.clone()).await.unwrap();
    let repo = AddressRepositoryImpl::new(Arc::new(db_pool.clone()));

    // Test save and find_by_id
    let new_address = create_test_address_model(city.id, person_id);
    let saved_address = repo.save(new_address.clone()).await.unwrap();
    assert_eq!(new_address.id, saved_address.id);

    let found_address = repo.find_by_id(new_address.id).await.unwrap().unwrap();
    assert_eq!(new_address.id, found_address.id);

    // Test find_by_city_id
    let addresses_in_city = repo.find_by_city_id(city.id, 1, 10).await.unwrap();
    assert_eq!(addresses_in_city.len(), 1);

    // Test find_ids_by_street_line1
    let ids = repo
        .find_ids_by_street_line1(new_address.street_line1.as_str())
        .await
        .unwrap();
    assert_eq!(ids.len(), 1);
}

#[tokio::test]
async fn test_messaging_repository() {
    let db_pool = commons::establish_connection().await;
    commons::cleanup_database(&db_pool).await;
    let repo = MessagingRepositoryImpl::new(Arc::new(db_pool.clone()));

    // Test save and find_by_id
    let new_messaging = create_test_messaging_model();
    let saved_messaging = repo.save(new_messaging.clone()).await.unwrap();
    assert_eq!(new_messaging.id, saved_messaging.id);

    let found_messaging = repo.find_by_id(new_messaging.id).await.unwrap().unwrap();
    assert_eq!(new_messaging.id, found_messaging.id);

    // Test find_by_messaging_type
    let emails = repo
        .find_by_messaging_type(MessagingType::Email, 1, 10)
        .await
        .unwrap();
    assert!(!emails.is_empty());

    // Test find_ids_by_value
    let ids = repo
        .find_ids_by_value(new_messaging.value.as_str())
        .await
        .unwrap();
    assert_eq!(ids.len(), 1);
}
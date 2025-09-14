use banking_db::models::person::{
    CountryIdxModelCache, CountryModel, CountrySubdivisionIdxModelCache, CountrySubdivisionModel,
    LocalityIdxModelCache, LocalityModel, LocationIdxModelCache, LocationModel, LocationType,
    MessagingIdxModelCache, MessagingModel, MessagingType, PersonIdxModelCache, PersonModel,
    PersonType,
};
use banking_db::repository::{
    LocationRepository, LocalityRepository, CountryRepository, MessagingRepository, PersonRepository,
    CountrySubdivisionRepository,
};
use banking_db_postgres::repository::{
    person::country_repository_impl::CountryRepositoryImpl,
    person::country_subdivision_repository_impl::CountrySubdivisionRepositoryImpl,
    person::locality_repository_impl::LocalityRepositoryImpl,
    person::location_repository_impl::LocationRepositoryImpl,
    person::messaging_repository_impl::MessagingRepositoryImpl,
    person::person_repository_impl::PersonRepositoryImpl,
};
use banking_db_postgres::repository::executor::Executor;
use heapless::String as HeaplessString;
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use super::commons::commons;

// Helper functions for creating test models

fn create_test_person_model(name: &str) -> PersonModel {
    PersonModel {
        id: Uuid::new_v4(),
        person_type: PersonType::Natural,
        // "John Doe"
        display_name: HeaplessString::try_from(name).unwrap(),
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

fn create_test_country_model(iso2: &str, name_l1: &str) -> CountryModel {
    CountryModel {
        id: Uuid::new_v4(),
        // US
        iso2: HeaplessString::try_from(iso2).unwrap(),
        // "United States"
        name_l1: HeaplessString::try_from(name_l1).unwrap(),
        name_l2: None,
        name_l3: None,
    }
}

fn create_test_country_subdivision_model(country_id: Uuid, code: &str, name_l1: &str) -> CountrySubdivisionModel {
    CountrySubdivisionModel {
        id: Uuid::new_v4(),
        country_id,
        // "CA"
        code: HeaplessString::try_from(code).unwrap(),
        // "California"
        name_l1: HeaplessString::try_from(name_l1).unwrap(),
        name_l2: None,
        name_l3: None,
    }
}

fn create_test_locality_model(country_subdivision_id: Uuid, code: &str, name_l1: &str) -> LocalityModel {
    LocalityModel {
        id: Uuid::new_v4(),
        country_subdivision_id,
        // "LA"
        code: HeaplessString::try_from(code).unwrap(),
        // "Los Angeles"
        name_l1: HeaplessString::try_from(name_l1).unwrap(),
        name_l2: None,
        name_l3: None,
    }
}

fn create_test_location_model(locality_id: Uuid, street_line1: &str, postal_code: &str) -> LocationModel {
    LocationModel {
        id: Uuid::new_v4(),
        location_type: LocationType::Residential,
        // format!("123 Main St {}", Uuid::new_v4()).as_str()
        street_line1: HeaplessString::try_from(street_line1)
            .unwrap(),
        street_line2: None,
        street_line3: None,
        street_line4: None,
        locality_id,
        // "90210"
        postal_code: Some(HeaplessString::try_from(postal_code).unwrap()),
        latitude: None,
        longitude: None,
        accuracy_meters: None,
    }
}

fn create_test_messaging_model(email: &str) -> MessagingModel {
    MessagingModel {
        id: Uuid::new_v4(),
        messaging_type: MessagingType::Email,
        // format!("test_{}@example.com", Uuid::new_v4()).as_str()
        value: HeaplessString::try_from(email).unwrap(),
        other_type: None,
    }
}

#[tokio::test]
async fn test_person_repository() {
    let db_pool = commons::establish_connection().await;
    commons::cleanup_database(&db_pool).await;
    let executor = Executor::Pool(Arc::new(db_pool));

    let country_idx_models = CountryRepositoryImpl::load_all_country_idx(&executor)
        .await
        .unwrap();
    let country_idx_cache =
        Arc::new(RwLock::new(CountryIdxModelCache::new(country_idx_models).unwrap()));
    let country_repo = Arc::new(CountryRepositoryImpl::new(
        executor.clone(),
        country_idx_cache,
    ));

    let country_subdivision_idx_models =
        CountrySubdivisionRepositoryImpl::load_all_country_subdivision_idx(&executor)
            .await
            .unwrap();
    let country_subdivision_idx_cache = Arc::new(RwLock::new(
        CountrySubdivisionIdxModelCache::new(country_subdivision_idx_models).unwrap(),
    ));
    let country_subdivision_repo = Arc::new(CountrySubdivisionRepositoryImpl::new(
        executor.clone(),
        country_repo.clone(),
        country_subdivision_idx_cache,
    ));

    let locality_idx_models = LocalityRepositoryImpl::load_all_locality_idx(&executor)
        .await
        .unwrap();
    let locality_idx_cache =
        Arc::new(RwLock::new(LocalityIdxModelCache::new(locality_idx_models).unwrap()));
    let locality_repo = Arc::new(LocalityRepositoryImpl::new(
        executor.clone(),
        country_subdivision_repo.clone(),
        locality_idx_cache,
    ));

    let location_idx_models = LocationRepositoryImpl::load_all_location_idx(&executor)
        .await
        .unwrap();
    let location_idx_cache =
        Arc::new(RwLock::new(LocationIdxModelCache::new(location_idx_models).unwrap()));
    let location_repo = Arc::new(LocationRepositoryImpl::new(
        executor.clone(),
        locality_repo.clone(),
        location_idx_cache,
    ));

    let person_idx_models = PersonRepositoryImpl::load_all_person_idx(&executor)
        .await
        .unwrap();
    let person_idx_cache =
        Arc::new(RwLock::new(PersonIdxModelCache::new(person_idx_models).unwrap()));
    let repo = PersonRepositoryImpl::new(executor, location_repo.clone(), person_idx_cache);

    // Test save and find_by_id
    let audit_log_id = Uuid::new_v4();

    let new_person = create_test_person_model("John Doe");
    let saved_person = repo.save(new_person.clone(), audit_log_id).await.unwrap();
    assert_eq!(new_person.id, saved_person.id);

    let found_person_idx = repo.find_by_id(new_person.id).await.unwrap().unwrap();
    assert_eq!(new_person.id, found_person_idx.person_id);

    // Test exists_by_id
    assert!(repo.exists_by_id(new_person.id).await.unwrap());
    assert!(!repo.exists_by_id(Uuid::new_v4()).await.unwrap());

    // Test find_by_ids
    let new_person2 = create_test_person_model("Nathan Clark");
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
    let executor = Executor::Pool(Arc::new(db_pool));
    let country_idx_models = CountryRepositoryImpl::load_all_country_idx(&executor)
        .await
        .unwrap();
    let country_idx_cache =
        Arc::new(RwLock::new(CountryIdxModelCache::new(country_idx_models).unwrap()));
    let repo = CountryRepositoryImpl::new(executor, country_idx_cache);

    // Test save and find_by_id
    let new_country = create_test_country_model("CM", "Cameroon");
    let saved_country = repo.save(new_country.clone()).await.unwrap();
    assert_eq!(new_country.id, saved_country.id);

    let found_country = repo.find_by_id(new_country.id).await.unwrap().unwrap();
    assert_eq!(new_country.id, found_country.country_id);

    // Test exists_by_id
    let found_countries = repo.find_by_ids(&[new_country.id]).await.unwrap();
    assert_eq!(found_countries.len(), 1);

    // Test find_by_ids
    let new_country2 = create_test_country_model("GA", "Gabon");
    repo.save(new_country2.clone()).await.unwrap();
    let ids = vec![new_country.id, new_country2.id];
    let found_countries = repo.find_by_ids(&ids).await.unwrap();
    assert_eq!(found_countries.len(), 2);

    // Test exists_by_id
    assert!(repo.exists_by_id(new_country.id).await.unwrap());
    assert!(!repo.exists_by_id(Uuid::new_v4()).await.unwrap());

    // Test find_by_iso2
    let found_by_iso2 = repo.find_by_iso2("GA", 1, 10).await.unwrap();
    assert!(!found_by_iso2.is_empty());
}

#[tokio::test]
async fn test_country_subdivision_repository() {
    let db_pool = commons::establish_connection().await;
    commons::cleanup_database(&db_pool).await;
    let executor = Executor::Pool(Arc::new(db_pool));
    let country_idx_models = CountryRepositoryImpl::load_all_country_idx(&executor)
        .await
        .unwrap();
    let country_idx_cache =
        Arc::new(RwLock::new(CountryIdxModelCache::new(country_idx_models).unwrap()));
    let country_repo = Arc::new(CountryRepositoryImpl::new(
        executor.clone(),
        country_idx_cache,
    ));
    let country = create_test_country_model("CM", "Cameroon");
    country_repo.save(country.clone()).await.unwrap();

    let country_subdivision_idx_models =
        CountrySubdivisionRepositoryImpl::load_all_country_subdivision_idx(&executor)
            .await
            .unwrap();
    let country_subdivision_idx_cache = Arc::new(RwLock::new(
        CountrySubdivisionIdxModelCache::new(country_subdivision_idx_models).unwrap(),
    ));
    let repo = CountrySubdivisionRepositoryImpl::new(
        executor,
        country_repo.clone(),
        country_subdivision_idx_cache,
    );

    // Test save and find_by_id
    let new_country_subdivision = create_test_country_subdivision_model(country.id, "OU", "Ouest");
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
    let executor = Executor::Pool(Arc::new(db_pool));
    let country_idx_models = CountryRepositoryImpl::load_all_country_idx(&executor)
        .await
        .unwrap();
    let country_idx_cache =
        Arc::new(RwLock::new(CountryIdxModelCache::new(country_idx_models).unwrap()));
    let country_repo = Arc::new(CountryRepositoryImpl::new(
        executor.clone(),
        country_idx_cache,
    ));
    let country = create_test_country_model("CM", "Cameroon");
    country_repo.save(country.clone()).await.unwrap();

    let country_subdivision_idx_models =
        CountrySubdivisionRepositoryImpl::load_all_country_subdivision_idx(&executor)
            .await
            .unwrap();
    let country_subdivision_idx_cache = Arc::new(RwLock::new(
        CountrySubdivisionIdxModelCache::new(country_subdivision_idx_models).unwrap(),
    ));
    let country_subdivision_repo = Arc::new(CountrySubdivisionRepositoryImpl::new(
        executor.clone(),
        country_repo.clone(),
        country_subdivision_idx_cache,
    ));
    let country_subdivision = create_test_country_subdivision_model(country.id, "OU", "Ouest");
    country_subdivision_repo
        .save(country_subdivision.clone())
        .await
        .unwrap();

    let locality_idx_models = LocalityRepositoryImpl::load_all_locality_idx(&executor)
        .await
        .unwrap();
    let locality_idx_cache =
        Arc::new(RwLock::new(LocalityIdxModelCache::new(locality_idx_models).unwrap()));
    let repo = LocalityRepositoryImpl::new(
        executor,
        country_subdivision_repo.clone(),
        locality_idx_cache,
    );

    // Test save and find_by_id
    let new_locality = create_test_locality_model(country_subdivision.id, "BANA_001", "Bana");
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
    let executor = Executor::Pool(Arc::new(db_pool));

    let country_idx_models = CountryRepositoryImpl::load_all_country_idx(&executor)
        .await
        .unwrap();
    let country_idx_cache =
        Arc::new(RwLock::new(CountryIdxModelCache::new(country_idx_models).unwrap()));
    let country_repo = Arc::new(CountryRepositoryImpl::new(
        executor.clone(),
        country_idx_cache,
    ));
    let country = create_test_country_model("CM", "Cameroon");
    country_repo.save(country.clone()).await.unwrap();

    let country_subdivision_idx_models =
        CountrySubdivisionRepositoryImpl::load_all_country_subdivision_idx(&executor)
            .await
            .unwrap();
    let country_subdivision_idx_cache = Arc::new(RwLock::new(
        CountrySubdivisionIdxModelCache::new(country_subdivision_idx_models).unwrap(),
    ));
    let country_subdivision_repo = Arc::new(CountrySubdivisionRepositoryImpl::new(
        executor.clone(),
        country_repo.clone(),
        country_subdivision_idx_cache,
    ));
    let country_subdivision = create_test_country_subdivision_model(country.id, "OU", "Ouest");
    country_subdivision_repo
        .save(country_subdivision.clone())
        .await
        .unwrap();

    let locality_idx_models = LocalityRepositoryImpl::load_all_locality_idx(&executor)
        .await
        .unwrap();
    let locality_idx_cache =
        Arc::new(RwLock::new(LocalityIdxModelCache::new(locality_idx_models).unwrap()));
    let locality_repo = Arc::new(LocalityRepositoryImpl::new(
        executor.clone(),
        country_subdivision_repo.clone(),
        locality_idx_cache,
    ));
    let locality = create_test_locality_model(country_subdivision.id, "BANA_001", "Bana");
    locality_repo.save(locality.clone()).await.unwrap();

    let location_idx_models = LocationRepositoryImpl::load_all_location_idx(&executor)
        .await
        .unwrap();
    let location_idx_cache =
        Arc::new(RwLock::new(LocationIdxModelCache::new(location_idx_models).unwrap()));
    let repo = LocationRepositoryImpl::new(executor, locality_repo, location_idx_cache);

    // Test save and find_by_id
    let new_location = create_test_location_model(locality.id, "Mission Catholique", "30321");
    let audit_log_id = Uuid::new_v4();
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
    let executor = Executor::Pool(Arc::new(db_pool));
    let messaging_idx_models = MessagingRepositoryImpl::load_all_messaging_idx(&executor)
        .await
        .unwrap();
    let messaging_idx_cache = Arc::new(RwLock::new(
        MessagingIdxModelCache::new(messaging_idx_models).unwrap(),
    ));
    let repo = MessagingRepositoryImpl::new(executor, messaging_idx_cache);

    // Test save and find_by_id
    let new_messaging = create_test_messaging_model("francis@ledgers-rust.com");
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
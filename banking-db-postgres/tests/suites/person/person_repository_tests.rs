use banking_db::models::person::{
    CountryIdxModelCache, CountrySubdivisionIdxModelCache, LocalityIdxModelCache,
    LocationIdxModelCache, PersonIdxModelCache,
};
use banking_db::repository::PersonRepository;
use banking_db_postgres::repository::{
    person::country_repository_impl::CountryRepositoryImpl,
    person::country_subdivision_repository_impl::CountrySubdivisionRepositoryImpl,
    person::locality_repository_impl::LocalityRepositoryImpl,
    person::location_repository_impl::LocationRepositoryImpl,
    person::person_repository_impl::PersonRepositoryImpl,
};
use banking_db_postgres::repository::executor::Executor;
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::suites::commons::commons;

use crate::suites::person::helpers::create_test_person_model;

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
    let audit_log_id = Uuid::new_v4();
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
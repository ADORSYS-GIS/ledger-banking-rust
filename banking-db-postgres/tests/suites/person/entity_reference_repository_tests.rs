use banking_db::models::person::{
    CountryIdxModelCache, CountrySubdivisionIdxModelCache, EntityReferenceIdxModelCache,
    LocalityIdxModelCache, LocationIdxModelCache, PersonIdxModelCache, RelationshipRole,
};
use banking_db::repository::{EntityReferenceRepository, PersonRepository};
use banking_db_postgres::repository::executor::Executor;
use banking_db_postgres::repository::person::country_repository_impl::CountryRepositoryImpl;
use banking_db_postgres::repository::person::country_subdivision_repository_impl::CountrySubdivisionRepositoryImpl;
use banking_db_postgres::repository::person::entity_reference_repository_impl::EntityReferenceRepositoryImpl;
use banking_db_postgres::repository::person::locality_repository_impl::LocalityRepositoryImpl;
use banking_db_postgres::repository::person::location_repository_impl::LocationRepositoryImpl;
use banking_db_postgres::repository::person::person_repository_impl::PersonRepositoryImpl;
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::suites::commons::commons;
use crate::suites::person::helpers::{
    create_test_entity_reference_model, create_test_person_model,
};

#[tokio::test]
async fn test_entity_reference_repository() {
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
    let person_idx_cache = Arc::new(RwLock::new(
        PersonIdxModelCache::new(person_idx_models).unwrap(),
    ));
    let person_repo = Arc::new(PersonRepositoryImpl::new(
        executor.clone(),
        location_repo.clone(),
        person_idx_cache,
    ));

    let new_person = create_test_person_model("John Doe");
    let audit_log_id = Uuid::new_v4();
    person_repo
        .save(new_person.clone(), audit_log_id)
        .await
        .unwrap();

    let entity_reference_idx_models =
        EntityReferenceRepositoryImpl::load_all_entity_reference_idx(&executor)
            .await
            .unwrap();
    let entity_reference_idx_cache = Arc::new(RwLock::new(
        EntityReferenceIdxModelCache::new(entity_reference_idx_models).unwrap(),
    ));
    let repo = EntityReferenceRepositoryImpl::new(
        executor,
        person_repo.clone(),
        entity_reference_idx_cache,
    );

    // Test save and find_by_id
    let new_entity_ref = create_test_entity_reference_model(
        new_person.id,
        RelationshipRole::Customer,
        "CUST-12345",
    );
    let saved_entity_ref = repo
        .save(new_entity_ref.clone(), audit_log_id)
        .await
        .unwrap();
    assert_eq!(new_entity_ref.id, saved_entity_ref.id);

    let found_entity_ref = repo
        .find_by_id(new_entity_ref.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(new_entity_ref.id, found_entity_ref.entity_reference_id);

    // Test find_by_person_id
    let refs_by_person = repo
        .find_by_person_id(new_person.id, 1, 10)
        .await
        .unwrap();
    assert_eq!(refs_by_person.len(), 1);

    // Test find_by_reference_external_id
    let refs_by_ext_id = repo
        .find_by_reference_external_id("CUST-12345", 1, 10)
        .await
        .unwrap();
    assert_eq!(refs_by_ext_id.len(), 1);

    // Test exists_by_id
    assert!(repo.exists_by_id(new_entity_ref.id).await.unwrap());
    assert!(!repo.exists_by_id(Uuid::new_v4()).await.unwrap());

    // Test find_by_ids
    let new_entity_ref2 = create_test_entity_reference_model(
        new_person.id,
        RelationshipRole::Employee,
        "EMP-54321",
    );
    repo.save(new_entity_ref2.clone(), audit_log_id)
        .await
        .unwrap();
    let ids = vec![new_entity_ref.id, new_entity_ref2.id];
    let found_refs = repo.find_by_ids(&ids).await.unwrap();
    assert_eq!(found_refs.len(), 2);

    // Test find_ids_by_person_id
    let ref_ids = repo.find_ids_by_person_id(new_person.id).await.unwrap();
    assert_eq!(ref_ids.len(), 2);
}
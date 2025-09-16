use banking_db::models::person::{
    CountryIdxModelCache, CountrySubdivisionIdxModelCache,
    LocalityIdxModelCache, LocationIdxModelCache,
};
use banking_db::repository::{
    CountryRepository, CountrySubdivisionRepository, LocalityRepository, LocationRepository,
};
use banking_db_postgres::repository::{
    person::country_repository_impl::CountryRepositoryImpl,
    person::country_subdivision_repository_impl::CountrySubdivisionRepositoryImpl,
    person::locality_repository_impl::LocalityRepositoryImpl,
    person::location_repository_impl::LocationRepositoryImpl,
};
use banking_db_postgres::repository::executor::Executor;
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::suites::commons::commons;
use crate::suites::person::helpers::{
    create_test_country_model, create_test_country_subdivision_model, create_test_locality_model,
    create_test_location_model,
};

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
use banking_db::models::person::{
    CountryIdxModelCache, CountrySubdivisionIdxModelCache,
};
use banking_db::repository::{CountryRepository, CountrySubdivisionRepository};
use banking_db_postgres::repository::{
    person::country_repository_impl::CountryRepositoryImpl,
    person::country_subdivision_repository_impl::CountrySubdivisionRepositoryImpl,
};
use banking_db_postgres::repository::executor::Executor;
use parking_lot::RwLock;
use std::sync::Arc;

use crate::suites::commons::commons;
use crate::suites::person::helpers::{
    create_test_country_model, create_test_country_subdivision_model,
};

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

    let found_country_subdivision = repo
        .find_by_id(new_country_subdivision.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        new_country_subdivision.id,
        found_country_subdivision.country_subdivision_id
    );

    // Test find_by_country_id
    let country_subdivisions_in_country =
        repo.find_by_country_id(country.id, 1, 10).await.unwrap();
    assert_eq!(country_subdivisions_in_country.len(), 1);
}
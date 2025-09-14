use banking_db::models::person::CountryIdxModelCache;
use banking_db::repository::CountryRepository;
use banking_db_postgres::repository::executor::Executor;
use banking_db_postgres::repository::person::country_repository_impl::CountryRepositoryImpl;
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::suites::commons::commons;

use crate::suites::person::helpers::create_test_country_model;

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
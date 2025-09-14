use banking_db::models::person::MessagingIdxModelCache;
use banking_db::repository::MessagingRepository;
use banking_db_postgres::repository::executor::Executor;
use banking_db_postgres::repository::person::messaging_repository_impl::MessagingRepositoryImpl;
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::suites::commons::commons;

use crate::suites::person::helpers::create_test_messaging_model;

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
    let audit_log_id = Uuid::new_v4();

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
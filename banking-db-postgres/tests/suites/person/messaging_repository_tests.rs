use banking_db::repository::{MessagingRepository, PersonRepos};
use uuid::Uuid;

use crate::suites::test_helper::setup_test_context;
use crate::suites::person::helpers::create_test_messaging_model;

#[tokio::test]
async fn test_messaging_repository() {
    let ctx = setup_test_context().await.unwrap();
    let repo = ctx.person_repos().messagings();

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
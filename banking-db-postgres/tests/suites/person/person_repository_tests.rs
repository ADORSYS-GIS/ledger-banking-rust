use banking_db::repository::{PersonRepository, PersonRepos};
use uuid::Uuid;

use crate::suites::person::helpers::create_test_person_model;
use crate::suites::test_helper::setup_test_context;

#[tokio::test]
async fn test_person_repository() {
    let ctx = setup_test_context().await.unwrap();
    let repo = ctx.person_repos().persons();

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
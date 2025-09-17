use banking_db::models::person::RelationshipRole;
use banking_db::repository::{EntityReferenceRepository, PersonRepository, PersonRepos};
use uuid::Uuid;

use crate::suites::test_helper::setup_test_context;
use crate::suites::person::helpers::{
    create_test_entity_reference_model, create_test_person_model,
};

#[tokio::test]
async fn test_entity_reference_repository() {
    let ctx = setup_test_context().await.unwrap();
    let person_repo = ctx.person_repos().persons();
    let repo = ctx.person_repos().entity_references();
    
    let new_person = create_test_person_model("John Doe");
    let audit_log_id = Uuid::new_v4();
    person_repo
        .save(new_person.clone(), audit_log_id)
        .await
        .unwrap();

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
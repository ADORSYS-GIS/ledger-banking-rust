use crate::person::mock_entity_reference_repository::create_test_entity_reference;
use crate::person::mock_person_repository::create_test_person;
use crate::person::common::{create_test_audit_log, create_test_services};
use banking_api::service::{EntityReferenceService, PersonService};

#[tokio::test]
async fn test_create_entity_reference() {
    let services = create_test_services();
    let person = create_test_person();
    services
        .person_service
        .create_person(person.clone(), create_test_audit_log())
        .await
        .unwrap();
    let entity_ref = create_test_entity_reference(person.id);
    let created_entity_ref = services
        .entity_reference_service
        .create_entity_reference(entity_ref.clone(), create_test_audit_log())
        .await
        .unwrap();
    assert_eq!(entity_ref.id, created_entity_ref.id);
}

#[tokio::test]
async fn test_find_entity_reference_by_id() {
    let services = create_test_services();
    let person = create_test_person();
    services
        .person_service
        .create_person(person.clone(), create_test_audit_log())
        .await
        .unwrap();
    let entity_ref = create_test_entity_reference(person.id);
    services
        .entity_reference_service
        .create_entity_reference(entity_ref.clone(), create_test_audit_log())
        .await
        .unwrap();
    let found_entity_ref = services
        .entity_reference_service
        .find_entity_reference_by_id(entity_ref.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(entity_ref.id, found_entity_ref.id);
}

#[tokio::test]
async fn test_find_entity_references_by_person_id() {
    let services = create_test_services();
    let person = create_test_person();
    services
        .person_service
        .create_person(person.clone(), create_test_audit_log())
        .await
        .unwrap();
    let entity_ref = create_test_entity_reference(person.id);
    services
        .entity_reference_service
        .create_entity_reference(entity_ref.clone(), create_test_audit_log())
        .await
        .unwrap();
    let entity_refs = services
        .entity_reference_service
        .find_entity_references_by_person_id(person.id)
        .await
        .unwrap();
    assert!(!entity_refs.is_empty());
}
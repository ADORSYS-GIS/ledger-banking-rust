use crate::person::mock_person_repository::create_test_person;
use crate::person::common::{create_test_audit_log, create_test_services};
use banking_api::service::PersonService;

#[tokio::test]
async fn test_create_person() {
    let services = create_test_services();
    let person = create_test_person();
    let created_person = services
        .person_service
        .create_person(person.clone(), create_test_audit_log())
        .await
        .unwrap();
    assert_eq!(person.id, created_person.id);
}

#[tokio::test]
async fn test_find_person_by_id() {
    let services = create_test_services();
    let person = create_test_person();
    services
        .person_service
        .create_person(person.clone(), create_test_audit_log())
        .await
        .unwrap();
    let found_person = services
        .person_service
        .find_person_by_id(person.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(person.id, found_person.id);
}

#[tokio::test]
async fn test_get_person_by_external_identifier() {
    let services = create_test_services();
    let person = create_test_person();
    services
        .person_service
        .create_person(person.clone(), create_test_audit_log())
        .await
        .unwrap();
    let found_person = services
        .person_service
        .get_persons_by_external_identifier(person.external_identifier.clone().unwrap())
        .await
        .unwrap();
    assert_eq!(person.id, found_person[0].id);
}
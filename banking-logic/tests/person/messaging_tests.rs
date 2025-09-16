use crate::person::mock_messaging_repository::create_test_messaging;
use crate::person::common::{create_test_audit_log, create_test_services};
use banking_api::service::MessagingService;

#[tokio::test]
async fn test_create_messaging() {
    let services = create_test_services();
    let messaging = create_test_messaging();
    let created_messaging = services
        .messaging_service
        .create_messaging(messaging.clone(), create_test_audit_log())
        .await
        .unwrap();
    assert_eq!(messaging.id, created_messaging.id);
}

#[tokio::test]
async fn test_find_messaging_by_id() {
    let services = create_test_services();
    let messaging = create_test_messaging();
    services
        .messaging_service
        .create_messaging(messaging.clone(), create_test_audit_log())
        .await
        .unwrap();
    let found_messaging = services
        .messaging_service
        .find_messaging_by_id(messaging.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(messaging.id, found_messaging.id);
}

#[tokio::test]
async fn test_find_messaging_by_value() {
    let services = create_test_services();
    let messaging = create_test_messaging();
    services
        .messaging_service
        .create_messaging(messaging.clone(), create_test_audit_log())
        .await
        .unwrap();
    let found_messaging = services
        .messaging_service
        .find_messaging_by_value(messaging.value.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(messaging.id, found_messaging.id);
}
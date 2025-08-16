use banking_db::models::person::{PersonModel, PersonType};
use banking_db::repository::PersonRepository;
use banking_db_postgres::repository::person_repository_impl::PersonRepositoryImpl;
use chrono::Utc;
use heapless::String as HeaplessString;
use std::sync::Arc;
use uuid::Uuid;

mod commons;
#[cfg(feature = "person_repository")]
mod person_init;

#[tokio::test]
#[cfg(feature = "person_repository")]
async fn test_create_and_get_person() {
    let db_pool = commons::establish_connection().await;
    person_init::create_test_person(&db_pool).await;
    let person_repo = PersonRepositoryImpl::new(Arc::new(db_pool));

    let new_person = PersonModel {
        id: Uuid::new_v4(),
        person_type: PersonType::Natural,
        display_name: HeaplessString::try_from("John Doe").unwrap(),
        external_identifier: Some(HeaplessString::try_from("JD001").unwrap()),
        organization_person_id: None,
        messaging1_id: None,
        messaging1_type: None,
        messaging2_id: None,
        messaging2_type: None,
        messaging3_id: None,
        messaging3_type: None,
        messaging4_id: None,
        messaging4_type: None,
        messaging5_id: None,
        messaging5_type: None,
        department: None,
        location_address_id: None,
        duplicate_of_person_id: None,
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let created_person = person_repo.save(new_person.clone()).await.unwrap();
    assert_eq!(new_person.id, created_person.id);

    let fetched_person = person_repo.find_by_id(new_person.id).await.unwrap().unwrap();
    assert_eq!(new_person.id, fetched_person.id);
}
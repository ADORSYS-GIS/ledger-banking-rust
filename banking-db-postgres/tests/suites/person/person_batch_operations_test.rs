use banking_db::models::person::{PersonModel, PersonType};
use banking_db::repository::{BatchRepository, PersonRepository, PersonRepos};
use heapless::String as HeaplessString;
use uuid::Uuid;

use crate::suites::test_helper::setup_test_context;

async fn setup_test_person() -> PersonModel {
    PersonModel {
        id: Uuid::new_v4(),
        person_type: PersonType::Natural,
        display_name: HeaplessString::try_from("Test Person").unwrap(),
        external_identifier: Some(HeaplessString::try_from("EXT001").unwrap()),
        entity_reference_count: 0,
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
        location_id: None,
        duplicate_of_person_id: None,
    }
}

#[tokio::test]
async fn test_create_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = setup_test_context().await?;
    let person_repo = ctx.person_repos().persons();

    let mut persons = Vec::new();
    for i in 0..5 {
        let mut person = setup_test_person().await;
        person.display_name =
            HeaplessString::try_from(format!("Test Person {i}").as_str()).unwrap();
        person.external_identifier =
            Some(HeaplessString::try_from(format!("EXT{i:03}").as_str()).unwrap());
        persons.push(person);
    }

    let audit_log_id = Uuid::new_v4();

    let saved_persons = person_repo
        .create_batch(persons.clone(), audit_log_id)
        .await?;

    assert_eq!(saved_persons.len(), 5);

    for person in &saved_persons {
        assert!(person_repo.exists_by_id(person.id).await?);
    }

    Ok(())
}

#[tokio::test]
async fn test_load_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = setup_test_context().await?;
    let person_repo = ctx.person_repos().persons();

    let mut persons = Vec::new();
    let mut test_ids = Vec::new();

    for i in 0..3 {
        let mut person = setup_test_person().await;
        person.display_name =
            HeaplessString::try_from(format!("Load Test {i}").as_str()).unwrap();
        person.external_identifier =
            Some(HeaplessString::try_from(format!("EXT_LOAD_{i}").as_str()).unwrap());
        persons.push(person.clone());
        test_ids.push(person.id);
    }

    let audit_log_id = Uuid::new_v4();
    person_repo
        .create_batch(persons, audit_log_id)
        .await?;

    let loaded_persons = person_repo
        .load_batch(&test_ids)
        .await?;

    assert_eq!(loaded_persons.len(), 3);

    let mut loaded_persons: Vec<PersonModel> = loaded_persons.into_iter().flatten().collect();
    assert_eq!(
        loaded_persons.len(),
        3,
        "Expected to load 3 persons, but found {}",
        loaded_persons.len()
    );
    loaded_persons.sort_by(|a, b| a.display_name.cmp(&b.display_name));

    for (i, person) in loaded_persons.iter().enumerate().take(3) {
        assert_eq!(person.display_name.as_str(), format!("Load Test {i}"));
    }

    Ok(())
}

#[tokio::test]
async fn test_update_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = setup_test_context().await?;
    let person_repo = ctx.person_repos().persons();

    let mut persons = Vec::new();
    for i in 0..3 {
        let mut person = setup_test_person().await;
        person.display_name = HeaplessString::try_from(format!("Original {i}").as_str()).unwrap();
        persons.push(person);
    }

    let audit_log_id = Uuid::new_v4();
    let saved_persons = person_repo
        .create_batch(persons.clone(), audit_log_id)
        .await?;

    // Update display names
    let mut updated_persons = saved_persons.clone();
    for (i, person) in updated_persons.iter_mut().enumerate() {
        person.display_name = HeaplessString::try_from(format!("Updated {i}").as_str()).unwrap();
    }

    person_repo
        .update_batch(updated_persons.clone(), audit_log_id)
        .await?;

    // Verify updates
    let test_ids: Vec<Uuid> = updated_persons.iter().map(|p| p.id).collect();
    let loaded_persons = person_repo
        .load_batch(&test_ids)
        .await?;

    let mut loaded_persons: Vec<PersonModel> = loaded_persons.into_iter().flatten().collect();
    assert_eq!(
        loaded_persons.len(),
        3,
        "Expected to load 3 persons, but found {}",
        loaded_persons.len()
    );
    loaded_persons.sort_by(|a, b| a.display_name.cmp(&b.display_name));

    for (i, person) in loaded_persons.iter().enumerate().take(3) {
        assert_eq!(person.display_name.as_str(), format!("Updated {i}"));
    }

    Ok(())
}

#[tokio::test]
async fn test_exists_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = setup_test_context().await?;
    let person_repo = ctx.person_repos().persons();

    let mut persons = Vec::new();
    let mut test_ids = Vec::new();

    for i in 0..3 {
        let mut person = setup_test_person().await;
        person.display_name =
            HeaplessString::try_from(format!("Exists Test {i}").as_str()).unwrap();
        persons.push(person.clone());
        test_ids.push(person.id);
    }

    let audit_log_id = Uuid::new_v4();
    person_repo
        .create_batch(persons, audit_log_id)
        .await?;

    // Add a non-existent ID
    test_ids.push(Uuid::new_v4());

    // Check existence
    let exists_results = person_repo
        .exist_by_ids(&test_ids)
        .await?;

    assert_eq!(exists_results.len(), 4);
    assert!(exists_results[0].1);
    assert!(exists_results[1].1);
    assert!(exists_results[2].1);
    assert!(!exists_results[3].1); // Non-existent ID

    Ok(())
}

#[tokio::test]
async fn test_delete_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = setup_test_context().await?;
    let person_repo = ctx.person_repos().persons();

    let mut persons = Vec::new();
    for i in 0..3 {
        let mut person = setup_test_person().await;
        person.display_name =
            HeaplessString::try_from(format!("Delete Test {i}").as_str()).unwrap();
        persons.push(person);
    }

    let audit_log_id = Uuid::new_v4();
    let saved_persons = person_repo
        .create_batch(persons, audit_log_id)
        .await?;

    // Delete first two persons
    let ids_to_delete: Vec<Uuid> = saved_persons.iter().take(2).map(|p| p.id).collect();

    person_repo
        .delete_batch(&ids_to_delete)
        .await?;

    // Verify deletions
    let all_ids: Vec<Uuid> = saved_persons.iter().map(|p| p.id).collect();
    let exists_results = person_repo
        .exist_by_ids(&all_ids)
        .await?;

    assert!(!exists_results[0].1); // Deleted
    assert!(!exists_results[1].1); // Deleted
    assert!(exists_results[2].1); // Still exists

    Ok(())
}

#[tokio::test]
async fn test_create_batch_chunked() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = setup_test_context().await?;
    let person_repo = ctx.person_repos().persons();

    let mut persons = Vec::new();
    for i in 0..25 {
        let mut person = setup_test_person().await;
        person.display_name = HeaplessString::try_from(format!("Chunked {i}").as_str()).unwrap();
        person.external_identifier =
            Some(HeaplessString::try_from(format!("CHK{i:03}").as_str()).unwrap());
        persons.push(person);
    }

    let audit_log_id = Uuid::new_v4();

    // Save with chunk size of 10
    let saved_persons = person_repo
        .create_batch_chunked(persons, audit_log_id, 10)
        .await?;

    assert_eq!(saved_persons.items.len(), 25);

    // Verify all persons exist
    for person in &saved_persons.items {
        assert!(person_repo.exists_by_id(person.id).await?);
    }

    Ok(())
}

#[tokio::test]
async fn test_validate_create_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = setup_test_context().await?;
    let person_repo = ctx.person_repos().persons();

    let mut persons = Vec::new();
    for i in 0..3 {
        let mut person = setup_test_person().await;
        person.display_name = HeaplessString::try_from(format!("Valid {i}").as_str()).unwrap();
        persons.push(person);
    }

    // Add invalid person (referencing non-existent organization)
    let mut invalid_person = setup_test_person().await;
    invalid_person.organization_person_id = Some(Uuid::new_v4());
    persons.push(invalid_person);

    // Validate batch
    let validation_results = person_repo
        .validate_create_batch(&persons)
        .await?;

    assert_eq!(validation_results.len(), 4);
    assert!(validation_results[0]); // Valid
    assert!(validation_results[1]); // Valid
    assert!(validation_results[2]); // Valid
    assert!(!validation_results[3]); // Invalid - org not found

    Ok(())
}

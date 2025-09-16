use banking_db::models::person::{
    CountryIdxModelCache, CountrySubdivisionIdxModelCache, LocalityIdxModelCache,
    LocationIdxModelCache, PersonIdxModelCache, PersonModel, PersonType,
};
use banking_db::repository::{
    BatchRepository, PersonRepos, PersonRepository, UnitOfWork, UnitOfWorkSession,
};
use banking_db_postgres::repository::unit_of_work_impl::PostgresUnitOfWork;
use heapless::String as HeaplessString;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use uuid::Uuid;

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

async fn setup_unit_of_work() -> Result<PostgresUnitOfWork, Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://user:password@localhost:5432/mydb".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let cleanup_path = std::path::Path::new(&manifest_dir)
        .join("tests")
        .join("fixtures")
        .join("cleanup.sql");
    let cleanup_sql = std::fs::read_to_string(cleanup_path)?;
    sqlx::query(&cleanup_sql).execute(&pool).await?;

    let uow = PostgresUnitOfWork::new(Arc::new(pool)).await;
    Ok(uow)
}

#[tokio::test]
async fn test_save_batch() -> Result<(), Box<dyn std::error::Error>> {
    let uow = setup_unit_of_work().await?;
    let session = uow.begin().await?;
    let person_repo = session.person_repos().persons();

    let mut persons = Vec::new();
    for i in 0..5 {
        let mut person = setup_test_person().await;
        person.display_name =
            HeaplessString::try_from(format!("Test Person {}", i).as_str()).unwrap();
        person.external_identifier =
            Some(HeaplessString::try_from(format!("EXT{:03}", i).as_str()).unwrap());
        persons.push(person);
    }

    let audit_log_id = Uuid::new_v4();

    let saved_persons = person_repo
        .save_batch(persons.clone(), audit_log_id)
        .await
        .map_err(|e| e.to_string())?;

    assert_eq!(saved_persons.len(), 5);

    for person in &saved_persons {
        assert!(person_repo.exists_by_id(person.id).await?);
    }

    Ok(())
}

#[tokio::test]
async fn test_load_batch() -> Result<(), Box<dyn std::error::Error>> {
    let uow = setup_unit_of_work().await?;
    let session = uow.begin().await?;
    let person_repo = session.person_repos().persons();

    let mut persons = Vec::new();
    let mut test_ids = Vec::new();

    for i in 0..3 {
        let mut person = setup_test_person().await;
        person.display_name =
            HeaplessString::try_from(format!("Load Test {}", i).as_str()).unwrap();
        person.external_identifier =
            Some(HeaplessString::try_from(format!("EXT_LOAD_{}", i).as_str()).unwrap());
        persons.push(person.clone());
        test_ids.push(person.id);
    }

    let audit_log_id = Uuid::new_v4();
    person_repo
        .save_batch(persons, audit_log_id)
        .await
        .map_err(|e| e.to_string())?;

    let loaded_persons = person_repo
        .load_batch(&test_ids)
        .await
        .map_err(|e| e.to_string())?;

    assert_eq!(loaded_persons.len(), 3);

    let mut loaded_persons: Vec<PersonModel> = loaded_persons.into_iter().flatten().collect();
    assert_eq!(
        loaded_persons.len(),
        3,
        "Expected to load 3 persons, but found {}",
        loaded_persons.len()
    );
    loaded_persons.sort_by(|a, b| a.display_name.cmp(&b.display_name));

    for i in 0..3 {
        assert_eq!(
            loaded_persons[i].display_name.as_str(),
            format!("Load Test {}", i)
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_update_batch() -> Result<(), Box<dyn std::error::Error>> {
    let uow = setup_unit_of_work().await?;
    let session = uow.begin().await?;
    let person_repo = session.person_repos().persons();

    let mut persons = Vec::new();
    for i in 0..3 {
        let mut person = setup_test_person().await;
        person.display_name = HeaplessString::try_from(format!("Original {}", i).as_str()).unwrap();
        persons.push(person);
    }

    let audit_log_id = Uuid::new_v4();
    let saved_persons = person_repo
        .save_batch(persons.clone(), audit_log_id)
        .await
        .map_err(|e| e.to_string())?;

    // Update display names
    let mut updated_persons = saved_persons.clone();
    for (i, person) in updated_persons.iter_mut().enumerate() {
        person.display_name = HeaplessString::try_from(format!("Updated {}", i).as_str()).unwrap();
    }

    person_repo
        .update_batch(updated_persons.clone(), audit_log_id)
        .await
        .map_err(|e| e.to_string())?;

    // Verify updates
    let test_ids: Vec<Uuid> = updated_persons.iter().map(|p| p.id).collect();
    let loaded_persons = person_repo
        .load_batch(&test_ids)
        .await
        .map_err(|e| e.to_string())?;

    let mut loaded_persons: Vec<PersonModel> = loaded_persons.into_iter().flatten().collect();
    assert_eq!(
        loaded_persons.len(),
        3,
        "Expected to load 3 persons, but found {}",
        loaded_persons.len()
    );
    loaded_persons.sort_by(|a, b| a.display_name.cmp(&b.display_name));

    for i in 0..3 {
        assert_eq!(
            loaded_persons[i].display_name.as_str(),
            format!("Updated {}", i)
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_exists_batch() -> Result<(), Box<dyn std::error::Error>> {
    let uow = setup_unit_of_work().await?;
    let session = uow.begin().await?;
    let person_repo = session.person_repos().persons();

    let mut persons = Vec::new();
    let mut test_ids = Vec::new();

    for i in 0..3 {
        let mut person = setup_test_person().await;
        person.display_name =
            HeaplessString::try_from(format!("Exists Test {}", i).as_str()).unwrap();
        persons.push(person.clone());
        test_ids.push(person.id);
    }

    let audit_log_id = Uuid::new_v4();
    person_repo
        .save_batch(persons, audit_log_id)
        .await
        .map_err(|e| e.to_string())?;

    // Add a non-existent ID
    test_ids.push(Uuid::new_v4());

    // Check existence
    let exists_results = person_repo
        .exist_by_ids(&test_ids)
        .await
        .map_err(|e| e.to_string())?;

    assert_eq!(exists_results.len(), 4);
    assert_eq!(exists_results[0].1, true);
    assert_eq!(exists_results[1].1, true);
    assert_eq!(exists_results[2].1, true);
    assert_eq!(exists_results[3].1, false); // Non-existent ID

    Ok(())
}

#[tokio::test]
async fn test_delete_batch() -> Result<(), Box<dyn std::error::Error>> {
    let uow = setup_unit_of_work().await?;
    let session = uow.begin().await?;
    let person_repo = session.person_repos().persons();

    let mut persons = Vec::new();
    for i in 0..3 {
        let mut person = setup_test_person().await;
        person.display_name =
            HeaplessString::try_from(format!("Delete Test {}", i).as_str()).unwrap();
        persons.push(person);
    }

    let audit_log_id = Uuid::new_v4();
    let saved_persons = person_repo
        .save_batch(persons, audit_log_id)
        .await
        .map_err(|e| e.to_string())?;

    // Delete first two persons
    let ids_to_delete: Vec<Uuid> = saved_persons.iter().take(2).map(|p| p.id).collect();

    person_repo
        .delete_batch(&ids_to_delete)
        .await
        .map_err(|e| e.to_string())?;

    // Verify deletions
    let all_ids: Vec<Uuid> = saved_persons.iter().map(|p| p.id).collect();
    let exists_results = person_repo
        .exist_by_ids(&all_ids)
        .await
        .map_err(|e| e.to_string())?;

    assert_eq!(exists_results[0].1, false); // Deleted
    assert_eq!(exists_results[1].1, false); // Deleted
    assert_eq!(exists_results[2].1, true); // Still exists

    Ok(())
}

#[tokio::test]
async fn test_save_batch_chunked() -> Result<(), Box<dyn std::error::Error>> {
    let uow = setup_unit_of_work().await?;
    let session = uow.begin().await?;
    let person_repo = session.person_repos().persons();

    let mut persons = Vec::new();
    for i in 0..25 {
        let mut person = setup_test_person().await;
        person.display_name = HeaplessString::try_from(format!("Chunked {}", i).as_str()).unwrap();
        person.external_identifier =
            Some(HeaplessString::try_from(format!("CHK{:03}", i).as_str()).unwrap());
        persons.push(person);
    }

    let audit_log_id = Uuid::new_v4();

    // Save with chunk size of 10
    let saved_persons = person_repo
        .save_batch_chunked(persons, audit_log_id, 10)
        .await
        .map_err(|e| e.to_string())?;

    assert_eq!(saved_persons.items.len(), 25);

    // Verify all persons exist
    for person in &saved_persons.items {
        assert!(person_repo.exists_by_id(person.id).await?);
    }

    Ok(())
}

#[tokio::test]
async fn test_validate_batch() -> Result<(), Box<dyn std::error::Error>> {
    let uow = setup_unit_of_work().await?;
    let session = uow.begin().await?;
    let person_repo = session.person_repos().persons();

    let mut persons = Vec::new();
    for i in 0..3 {
        let mut person = setup_test_person().await;
        person.display_name = HeaplessString::try_from(format!("Valid {}", i).as_str()).unwrap();
        persons.push(person);
    }

    // Add invalid person (referencing non-existent organization)
    let mut invalid_person = setup_test_person().await;
    invalid_person.organization_person_id = Some(Uuid::new_v4());
    persons.push(invalid_person);

    // Validate batch
    let validation_results = person_repo
        .validate_batch(&persons)
        .await
        .map_err(|e| e.to_string())?;

    assert_eq!(validation_results.len(), 4);
    assert!(validation_results[0]); // Valid
    assert!(validation_results[1]); // Valid
    assert!(validation_results[2]); // Valid
    assert!(!validation_results[3]); // Invalid - org not found

    Ok(())
}

#[cfg(test)]
mod batch_tests {
    use banking_db::models::person::{PersonModel, PersonType, PersonIdxModel, PersonIdxModelCache};
    use banking_db::repository::{BatchRepository, PersonRepository};
    use banking_db_postgres::repository::person_person_repository_impl::PersonRepositoryImpl;
    use banking_db_postgres::repository::person_location_repository_impl::LocationRepositoryImpl;
    use banking_db_postgres::repository::executor::Executor;
    use heapless::String as HeaplessString;
    use parking_lot::RwLock;
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

    async fn setup_test_repository() -> Result<PersonRepositoryImpl, Box<dyn std::error::Error>> {
        // Create test database connection
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost/test_db".to_string());
        
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
        
        let executor = Executor::Pool(Arc::new(pool));
        
        // Load existing indexes for cache
        let person_indexes = PersonRepositoryImpl::load_all_person_idx(&executor).await?;
        let person_cache = PersonIdxModelCache::new(person_indexes)
            .map_err(|e| format!("Failed to create person cache: {}", e))?;
        
        // Create location repository (assuming it has similar setup)
        let location_repository = Arc::new(LocationRepositoryImpl::new(executor.clone()));
        
        // Create person repository with all required parameters
        let person_repo = PersonRepositoryImpl::new(
            executor,
            location_repository,
            Arc::new(RwLock::new(person_cache))
        );
        
        Ok(person_repo)
    }

    #[tokio::test]
    async fn test_save_batch() -> Result<(), Box<dyn std::error::Error>> {
        let person_repo = setup_test_repository().await?;
        
        // Create test persons
        let mut persons = Vec::new();
        for i in 0..5 {
            let mut person = setup_test_person().await;
            person.display_name = HeaplessString::try_from(format!("Test Person {}", i)).unwrap();
            person.external_identifier = Some(HeaplessString::try_from(format!("EXT{:03}", i)).unwrap());
            persons.push(person);
        }
        
        let audit_log_id = Uuid::new_v4();
        
        // Save batch
        let saved_persons = person_repo
            .save_batch(persons.clone(), audit_log_id)
            .await?;
        
        assert_eq!(saved_persons.len(), 5);
        
        // Verify all persons exist
        for person in &saved_persons {
            assert!(person_repo.exists_by_id(person.id).await?);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_load_batch() -> Result<(), Box<dyn std::error::Error>> {
        let person_repo = setup_test_repository().await?;
        
        // Create and save test persons first
        let mut persons = Vec::new();
        let mut test_ids = Vec::new();
        
        for i in 0..3 {
            let mut person = setup_test_person().await;
            person.display_name = HeaplessString::try_from(format!("Load Test {}", i)).unwrap();
            persons.push(person.clone());
            test_ids.push(person.id);
        }
        
        let audit_log_id = Uuid::new_v4();
        person_repo
            .save_batch(persons, audit_log_id)
            .await?;
        
        // Load batch
        let loaded_persons = person_repo.load_batch(&test_ids).await?;
        
        assert_eq!(loaded_persons.len(), 3);
        for (i, person) in loaded_persons.iter().enumerate() {
            assert_eq!(person.display_name.as_str(), format!("Load Test {}", i));
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_update_batch() -> Result<(), Box<dyn std::error::Error>> {
        let person_repo = setup_test_repository().await?;
        
        // Create and save test persons
        let mut persons = Vec::new();
        for i in 0..3 {
            let mut person = setup_test_person().await;
            person.display_name = HeaplessString::try_from(format!("Original {}", i)).unwrap();
            persons.push(person);
        }
        
        let audit_log_id = Uuid::new_v4();
        let saved_persons = person_repo
            .save_batch(persons.clone(), audit_log_id)
            .await?;
        
        // Update display names
        let mut updated_persons = saved_persons.clone();
        for (i, person) in updated_persons.iter_mut().enumerate() {
            person.display_name = HeaplessString::try_from(format!("Updated {}", i)).unwrap();
        }
        
        person_repo
            .update_batch(updated_persons.clone(), audit_log_id)
            .await?;
        
        // Verify updates
        let test_ids: Vec<Uuid> = updated_persons.iter().map(|p| p.id).collect();
        let loaded_persons = person_repo.load_batch(&test_ids).await?;
        
        for (i, person) in loaded_persons.iter().enumerate() {
            assert_eq!(person.display_name.as_str(), format!("Updated {}", i));
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_exists_batch() -> Result<(), Box<dyn std::error::Error>> {
        let person_repo = setup_test_repository().await?;
        
        // Create and save test persons
        let mut persons = Vec::new();
        let mut test_ids = Vec::new();
        
        for i in 0..3 {
            let mut person = setup_test_person().await;
            person.display_name = HeaplessString::try_from(format!("Exists Test {}", i)).unwrap();
            persons.push(person.clone());
            test_ids.push(person.id);
        }
        
        let audit_log_id = Uuid::new_v4();
        person_repo
            .save_batch(persons, audit_log_id)
            .await?;
        
        // Add a non-existent ID
        test_ids.push(Uuid::new_v4());
        
        // Check existence
        let exists_results = person_repo.exists_batch(&test_ids).await?;
        
        assert_eq!(exists_results.len(), 4);
        assert_eq!(exists_results[0], true);
        assert_eq!(exists_results[1], true);
        assert_eq!(exists_results[2], true);
        assert_eq!(exists_results[3], false); // Non-existent ID
        
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_batch() -> Result<(), Box<dyn std::error::Error>> {
        let person_repo = setup_test_repository().await?;
        
        // Create and save test persons
        let mut persons = Vec::new();
        for i in 0..3 {
            let mut person = setup_test_person().await;
            person.display_name = HeaplessString::try_from(format!("Delete Test {}", i)).unwrap();
            persons.push(person);
        }
        
        let audit_log_id = Uuid::new_v4();
        let saved_persons = person_repo
            .save_batch(persons, audit_log_id)
            .await?;
        
        // Delete first two persons
        let ids_to_delete: Vec<Uuid> = saved_persons.iter().take(2).map(|p| p.id).collect();
        
        person_repo
            .delete_batch(&ids_to_delete)
            .await?;
        
        // Verify deletions
        let all_ids: Vec<Uuid> = saved_persons.iter().map(|p| p.id).collect();
        let exists_results = person_repo
            .exists_batch(&all_ids)
            .await?;
        
        assert_eq!(exists_results[0], false); // Deleted
        assert_eq!(exists_results[1], false); // Deleted
        assert_eq!(exists_results[2], true);  // Still exists
        
        Ok(())
    }

    #[tokio::test]
    async fn test_save_batch_chunked() -> Result<(), Box<dyn std::error::Error>> {
        let person_repo = setup_test_repository().await?;
        
        // Create 25 test persons
        let mut persons = Vec::new();
        for i in 0..25 {
            let mut person = setup_test_person().await;
            person.display_name = HeaplessString::try_from(format!("Chunked {}", i)).unwrap();
            person.external_identifier = Some(HeaplessString::try_from(format!("CHK{:03}", i)).unwrap());
            persons.push(person);
        }
        
        let audit_log_id = Uuid::new_v4();
        
        // Save with chunk size of 10
        let saved_persons = person_repo
            .save_batch_chunked(persons, audit_log_id, 10)
            .await?;
        
        assert_eq!(saved_persons.len(), 25);
        
        // Verify all persons exist
        for person in &saved_persons {
            assert!(person_repo.exists_by_id(person.id).await?);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_validate_batch() -> Result<(), Box<dyn std::error::Error>> {
        let person_repo = setup_test_repository().await?;
        
        // Create valid persons
        let mut persons = Vec::new();
        for i in 0..3 {
            let mut person = setup_test_person().await;
            person.display_name = HeaplessString::try_from(format!("Valid {}", i)).unwrap();
            persons.push(person);
        }
        
        // Add invalid person (referencing non-existent organization)
        let mut invalid_person = setup_test_person().await;
        invalid_person.organization_person_id = Some(Uuid::new_v4());
        persons.push(invalid_person);
        
        // Validate batch
        let validation_results = person_repo
            .validate_batch(&persons)
            .await?;
        
        assert_eq!(validation_results.len(), 4);
        assert!(validation_results[0].is_none()); // Valid
        assert!(validation_results[1].is_none()); // Valid
        assert!(validation_results[2].is_none()); // Valid
        assert!(validation_results[3].is_some()); // Invalid - org not found
        
        Ok(())
    }
}
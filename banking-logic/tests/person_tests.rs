#[cfg(feature = "person_tests")]
mod person_tests {
    use banking_api::{BankingResult, Person, PersonService, PersonType};
    use banking_db_postgres::{
        LocationRepositoryImpl, LocalityRepositoryImpl, CountryRepositoryImpl, EntityReferenceRepositoryImpl,
        MessagingRepositoryImpl, PersonRepositoryImpl, CountrySubdivisionRepositoryImpl, AuditLogRepositoryImpl,
    };
    use banking_logic::services::PersonServiceImpl;
    use sqlx::PgPool;
    use std::sync::Arc;
    use uuid::Uuid;
    use banking_api::domain::AuditLog;
    use heapless::String as HeaplessString;

    async fn setup() -> (Arc<dyn PersonService>, PgPool) {
        let pool = PgPool::connect("postgres://user:password@localhost:5432/testdb")
            .await
            .unwrap();
        let pool = Arc::new(pool);

        let person_repo = Arc::new(PersonRepositoryImpl::new(pool.clone()));
        let audit_log_repo = Arc::new(AuditRepositoryImpl::new(pool.clone()));
        let country_repo = Arc::new(CountryRepositoryImpl::new(pool.clone()));
        let country_subdivision_repo = Arc::new(CountrySubdivisionRepositoryImpl::new(pool.clone()));
        let locality_repo = Arc::new(LocalityRepositoryImpl::new(pool.clone()));
        let location_repo = Arc::new(LocationRepositoryImpl::new(pool.clone()));
        let messaging_repo = Arc::new(MessagingRepositoryImpl::new(pool.clone()));
        let entity_ref_repo = Arc::new(EntityReferenceRepositoryImpl::new(pool.clone()));

        let person_service = Arc::new(PersonServiceImpl::new(
            person_repo,
            country_repo,
            country_subdivision_repo,
            locality_repo,
            location_repo,
            messaging_repo,
            entity_ref_repo,
        ));

        (person_service, (*pool).clone())
    }

    #[tokio::test]
    async fn test_create_and_find_person() -> BankingResult<()> {
        let (person_service, _pool) = setup().await;

        let new_person = Person {
            id: Uuid::new_v4(),
            version: 1,
            person_type: PersonType::Natural,
            display_name: HeaplessString::try_from("John Doe").unwrap(),
            external_identifier: None,
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
            audit_log_id: Uuid::new_v4(),
        };

        let created_person = person_service.create_person(new_person.clone()).await?;
        assert_eq!(created_person.display_name, new_person.display_name);

        let found_person = person_service
            .find_person_by_id(created_person.id)
            .await?
            .unwrap();
        assert_eq!(found_person.id, created_person.id);

        Ok(())
    }
}
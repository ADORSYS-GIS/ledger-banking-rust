#[cfg(feature = "person_tests")]
mod person_tests {
    use banking_api::{BankingResult, Person, PersonService, PersonType};
    use banking_db_postgres::{
        AuditLogRepositoryImpl, CountryRepositoryImpl, CountrySubdivisionRepositoryImpl,
        EntityReferenceRepositoryImpl, LocalityRepositoryImpl, LocationRepositoryImpl,
        MessagingRepositoryImpl, PersonRepositoryImpl,
    };
    use banking_logic::services::{repositories::Repositories, PersonServiceImpl};
    use heapless::String as HeaplessString;
    use sqlx::PgPool;
    use std::sync::Arc;
    use uuid::Uuid;

    use banking_db_postgres::test_utils::commons;

    async fn setup() -> (Arc<dyn PersonService>, PgPool) {
        let pool = commons::establish_connection().await;
        let pool = Arc::new(pool);

        let repositories = Repositories {
            person_repository: Arc::new(PersonRepositoryImpl::new(pool.clone())),
            audit_log_repository: Arc::new(AuditLogRepositoryImpl::new(pool.clone())),
            country_repository: Arc::new(CountryRepositoryImpl::new(pool.clone())),
            country_subdivision_repository: Arc::new(CountrySubdivisionRepositoryImpl::new(
                pool.clone(),
            )),
            locality_repository: Arc::new(LocalityRepositoryImpl::new(pool.clone())),
            location_repository: Arc::new(LocationRepositoryImpl::new(pool.clone())),
            messaging_repository: Arc::new(MessagingRepositoryImpl::new(pool.clone())),
            entity_reference_repository: Arc::new(EntityReferenceRepositoryImpl::new(
                pool.clone(),
            )),
        };

        let person_service = Arc::new(PersonServiceImpl::new(repositories));

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
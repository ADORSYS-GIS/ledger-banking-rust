#[cfg(feature = "person_tests")]
mod person_tests {
    use banking_api::{BankingResult, Person, PersonService, PersonType};
    use banking_db_postgres::repository::{
        AddressRepositoryImpl, CityRepositoryImpl, CountryRepositoryImpl, EntityReferenceRepositoryImpl,
        MessagingRepositoryImpl, PersonRepositoryImpl, StateProvinceRepositoryImpl,
    };
    use banking_logic::services::PersonServiceImpl;
    use sqlx::PgPool;
    use std::sync::Arc;
    use uuid::Uuid;

    async fn setup() -> (Arc<dyn PersonService>, PgPool) {
        let pool = PgPool::connect("postgres://user:password@localhost:5432/testdb")
            .await
            .unwrap();
        let pool = Arc::new(pool);

        let person_repo = Arc::new(PersonRepositoryImpl::new(pool.clone()));
        let country_repo = Arc::new(CountryRepositoryImpl::new(pool.clone()));
        let state_repo = Arc::new(StateProvinceRepositoryImpl::new(pool.clone()));
        let city_repo = Arc::new(CityRepositoryImpl::new(pool.clone()));
        let address_repo = Arc::new(AddressRepositoryImpl::new(pool.clone()));
        let messaging_repo = Arc::new(MessagingRepositoryImpl::new(pool.clone()));
        let entity_ref_repo = Arc::new(EntityReferenceRepositoryImpl::new(pool.clone()));

        let person_service = Arc::new(PersonServiceImpl::new(
            person_repo,
            country_repo,
            state_repo,
            city_repo,
            address_repo,
            messaging_repo,
            entity_ref_repo,
        ));

        (person_service, (*pool).clone())
    }

    #[tokio::test]
    async fn test_create_and_find_person() -> BankingResult<()> {
        let (person_service, _pool) = setup().await;

        let new_person = Person::new(
            Uuid::new_v4(),
            PersonType::Natural,
            "John Doe",
        )
        .unwrap();

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
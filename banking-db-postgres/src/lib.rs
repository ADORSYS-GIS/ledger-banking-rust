use banking_logic::services::repositories::Repositories;
use sqlx::{PgPool, Postgres};
use std::sync::Arc;

pub mod repository;
pub mod utils;
pub mod test_utils;

pub use repository::audit_repository_impl::AuditLogRepositoryImpl;
pub use repository::person_country_repository_impl::CountryRepositoryImpl;
pub use repository::person_country_subdivision_repository_impl::CountrySubdivisionRepositoryImpl;
pub use repository::person_entity_reference_repository_impl::EntityReferenceRepositoryImpl;
pub use repository::person_locality_repository_impl::LocalityRepositoryImpl;
pub use repository::person_location_repository_impl::LocationRepositoryImpl;
pub use repository::person_messaging_repository_impl::MessagingRepositoryImpl;
pub use repository::person_person_repository_impl::PersonRepositoryImpl;
pub use repository::unit_of_work_impl;

pub struct PostgresRepositories {
    pool: Arc<PgPool>,
}

impl PostgresRepositories {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create_person_service_repositories(&self) -> Repositories<Postgres> {
        let executor = repository::executor::Executor::Pool(self.pool.clone());
        let country_repository =
            Arc::new(CountryRepositoryImpl::new(executor.clone()).await);
        let country_subdivision_repository = Arc::new(
            CountrySubdivisionRepositoryImpl::new(executor.clone(), country_repository.clone())
                .await,
        );
        let locality_repository = Arc::new(
            LocalityRepositoryImpl::new(
                executor.clone(),
                country_subdivision_repository.clone(),
            )
            .await,
        );
        let location_repository = Arc::new(
            LocationRepositoryImpl::new(executor.clone(), locality_repository.clone()).await,
        );
        let person_repository = Arc::new(
            PersonRepositoryImpl::new(executor.clone(), location_repository.clone()).await,
        );
        let entity_reference_repository = Arc::new(
            EntityReferenceRepositoryImpl::new(executor.clone(), person_repository.clone())
                .await,
        );
        Repositories {
            person_repository,
            audit_log_repository: Arc::new(AuditLogRepositoryImpl::new(executor.clone())),
            country_repository,
            country_subdivision_repository,
            locality_repository,
            location_repository,
            messaging_repository: Arc::new(MessagingRepositoryImpl::new(executor.clone()).await),
            entity_reference_repository,
        }
    }
}
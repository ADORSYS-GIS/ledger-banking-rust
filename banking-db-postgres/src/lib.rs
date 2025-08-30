use banking_logic::services::repositories::Repositories;
use sqlx::PgPool;
use std::sync::Arc;

pub mod repository;
pub mod utils;
pub mod test_utils;

pub use repository::audit_repository_impl::AuditLogRepositoryImpl;
pub use repository::person_repository_impl::{
    CountryRepositoryImpl, CountrySubdivisionRepositoryImpl, EntityReferenceRepositoryImpl,
    LocalityRepositoryImpl, LocationRepositoryImpl, MessagingRepositoryImpl, PersonRepositoryImpl,
};

pub struct PostgresRepositories {
    pool: Arc<PgPool>,
}

impl PostgresRepositories {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create_person_service_repositories(&self) -> Repositories {
        Repositories {
            person_repository: Arc::new(PersonRepositoryImpl::new(self.pool.clone()).await),
            audit_log_repository: Arc::new(AuditLogRepositoryImpl::new(self.pool.clone())),
            country_repository: Arc::new(CountryRepositoryImpl::new(self.pool.clone())),
            country_subdivision_repository: Arc::new(CountrySubdivisionRepositoryImpl::new(
                self.pool.clone(),
            )),
            locality_repository: Arc::new(LocalityRepositoryImpl::new(self.pool.clone())),
            location_repository: Arc::new(LocationRepositoryImpl::new(self.pool.clone())),
            messaging_repository: Arc::new(MessagingRepositoryImpl::new(self.pool.clone())),
            entity_reference_repository: Arc::new(EntityReferenceRepositoryImpl::new(
                self.pool.clone(),
            )),
        }
    }
}
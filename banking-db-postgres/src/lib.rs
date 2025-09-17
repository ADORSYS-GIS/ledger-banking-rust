use banking_db::models::person::{
    CountryIdxModelCache, CountrySubdivisionIdxModelCache, EntityReferenceIdxModelCache,
    LocalityIdxModelCache, LocationIdxModelCache, MessagingIdxModelCache, PersonIdxModelCache,
};
use banking_logic::services::repositories::Repositories;
use parking_lot::RwLock;
use sqlx::{PgPool, Postgres};
use std::sync::Arc;

pub mod repository;
pub mod utils;

pub use repository::audit_repository_impl::AuditLogRepositoryImpl;
pub use repository::person::country_repository_impl::CountryRepositoryImpl;
pub use repository::person::country_subdivision_repository_impl::CountrySubdivisionRepositoryImpl;
pub use repository::person::entity_reference_repository_impl::EntityReferenceRepositoryImpl;
pub use repository::person::locality_repository_impl::LocalityRepositoryImpl;
pub use repository::person::location_repository_impl::LocationRepositoryImpl;
pub use repository::person::messaging_repository_impl::MessagingRepositoryImpl;
pub use repository::person::person_repository_impl::PersonRepositoryImpl;
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

        let country_idx_models = CountryRepositoryImpl::load_all_country_idx(&executor)
            .await
            .expect("Failed to load country index");
        let country_idx_cache = Arc::new(RwLock::new(
            CountryIdxModelCache::new(country_idx_models)
                .expect("Failed to create country index cache"),
        ));

        let country_subdivision_idx_models =
            CountrySubdivisionRepositoryImpl::load_all_country_subdivision_idx(&executor)
                .await
                .expect("Failed to load country subdivision index");
        let country_subdivision_idx_cache = Arc::new(RwLock::new(
            CountrySubdivisionIdxModelCache::new(country_subdivision_idx_models)
                .expect("Failed to create country subdivision index cache"),
        ));

        let locality_idx_models = LocalityRepositoryImpl::load_all_locality_idx(&executor)
            .await
            .expect("Failed to load locality index");
        let locality_idx_cache = Arc::new(RwLock::new(
            LocalityIdxModelCache::new(locality_idx_models)
                .expect("Failed to create locality index cache"),
        ));

        let location_idx_models = LocationRepositoryImpl::load_all_location_idx(&executor)
            .await
            .expect("Failed to load location index");
        let location_idx_cache = Arc::new(RwLock::new(
            LocationIdxModelCache::new(location_idx_models)
                .expect("Failed to create location index cache"),
        ));

        let person_idx_models = PersonRepositoryImpl::load_all_person_idx(&executor)
            .await
            .expect("Failed to load person index");
        let person_idx_cache = Arc::new(RwLock::new(
            PersonIdxModelCache::new(person_idx_models).expect("Failed to create person index cache"),
        ));

        let country_repository = Arc::new(CountryRepositoryImpl::new(
            executor.clone(),
            country_idx_cache,
        ));
        let country_subdivision_repository = Arc::new(CountrySubdivisionRepositoryImpl::new(
            executor.clone(),
            country_repository.clone(),
            country_subdivision_idx_cache,
        ));
        let locality_repository = Arc::new(LocalityRepositoryImpl::new(
            executor.clone(),
            country_subdivision_repository.clone(),
            locality_idx_cache,
        ));
        let location_repository = Arc::new(LocationRepositoryImpl::new(
            executor.clone(),
            locality_repository.clone(),
            location_idx_cache,
        ));
        if locality_repository
            .location_repository
            .set(location_repository.clone())
            .is_err()
        {
            // This should not happen in this setup, as it's initialized only once.
            panic!("Attempted to set location_repository more than once.");
        }
        let person_repository = Arc::new(PersonRepositoryImpl::new(
            executor.clone(),
            location_repository.clone(),
            person_idx_cache,
        ));
        let messaging_idx_models = MessagingRepositoryImpl::load_all_messaging_idx(&executor)
            .await
            .expect("Failed to load messaging index");
        let messaging_idx_cache = Arc::new(RwLock::new(
            MessagingIdxModelCache::new(messaging_idx_models)
                .expect("Failed to create messaging index cache"),
        ));

        let entity_reference_idx_models =
            EntityReferenceRepositoryImpl::load_all_entity_reference_idx(&executor)
                .await
                .expect("Failed to load entity reference index");
        let entity_reference_idx_cache = Arc::new(RwLock::new(
            EntityReferenceIdxModelCache::new(entity_reference_idx_models)
                .expect("Failed to create entity reference index cache"),
        ));

        let entity_reference_repository = Arc::new(EntityReferenceRepositoryImpl::new(
            executor.clone(),
            person_repository.clone(),
            entity_reference_idx_cache,
        ));
        let messaging_repository = Arc::new(MessagingRepositoryImpl::new(
            executor.clone(),
            messaging_idx_cache,
        ));
        Repositories {
            person_repository,
            audit_log_repository: Arc::new(AuditLogRepositoryImpl::new(executor.clone())),
            country_repository,
            country_subdivision_repository,
            locality_repository,
            location_repository,
            messaging_repository,
            entity_reference_repository,
        }
    }
}
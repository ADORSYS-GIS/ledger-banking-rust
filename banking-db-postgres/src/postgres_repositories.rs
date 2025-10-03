use banking_db::models::person::{
    CountryIdxModelCache, CountrySubdivisionIdxModelCache, EntityReferenceIdxModelCache,
    LocalityIdxModelCache, LocationIdxModelCache, PersonIdxModelCache,
};
use banking_logic::services::repositories::Repositories;
use parking_lot::RwLock;
use sqlx::{PgPool, Postgres};
use std::sync::Arc;

use crate::repository::{
    audit_repository_impl::AuditLogRepositoryImpl,
    executor::Executor,
    person::{
        country_repository_impl::CountryRepositoryImpl,
        country_subdivision_repository_impl::CountrySubdivisionRepositoryImpl,
        entity_reference_repository_impl::EntityReferenceRepositoryImpl,
        locality_repository_impl::LocalityRepositoryImpl,
        location_repository_impl::LocationRepositoryImpl,
        person_repository_impl::PersonRepositoryImpl,
    },
};

pub struct PostgresRepositories {
    pool: Arc<PgPool>,
}

impl PostgresRepositories {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create_person_service_repositories(&self) -> Repositories<Postgres> {
        let executor = Executor::Pool(self.pool.clone());

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
        Repositories {
            person_repository,
            audit_log_repository: Arc::new(AuditLogRepositoryImpl::new(executor.clone())),
            country_repository,
            country_subdivision_repository,
            locality_repository,
            location_repository,
            entity_reference_repository,
        }
    }
}
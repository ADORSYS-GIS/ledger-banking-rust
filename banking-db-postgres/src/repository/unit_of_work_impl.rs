use async_trait::async_trait;
use banking_api::BankingResult;
use banking_db::{
    models::person::{
        CountryIdxModelCache, CountrySubdivisionIdxModelCache, EntityReferenceIdxModelCache,
        LocalityIdxModelCache, LocationIdxModelCache, MessagingIdxModelCache, PersonIdxModelCache,
    },
    repository::{UnitOfWork, UnitOfWorkSession},
};
use parking_lot::RwLock;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;

use crate::repository::{
    audit_repository_impl::AuditLogRepositoryImpl,
    executor::Executor,
    person_country_repository_impl::CountryRepositoryImpl,
    person_country_subdivision_repository_impl::CountrySubdivisionRepositoryImpl,
    person_entity_reference_repository_impl::EntityReferenceRepositoryImpl,
    person_locality_repository_impl::LocalityRepositoryImpl,
    person_location_repository_impl::LocationRepositoryImpl,
    person_messaging_repository_impl::MessagingRepositoryImpl,
    person_person_repository_impl::PersonRepositoryImpl,
};

#[derive(Clone)]
pub struct PersonCaches {
    pub country_idx_cache: Arc<RwLock<CountryIdxModelCache>>,
    pub country_subdivision_idx_cache: Arc<RwLock<CountrySubdivisionIdxModelCache>>,
    pub locality_idx_cache: Arc<RwLock<LocalityIdxModelCache>>,
    pub location_idx_cache: Arc<RwLock<LocationIdxModelCache>>,
    pub person_idx_cache: Arc<RwLock<PersonIdxModelCache>>,
    pub messaging_idx_cache: Arc<RwLock<MessagingIdxModelCache>>,
    pub entity_reference_idx_cache: Arc<RwLock<EntityReferenceIdxModelCache>>,
}

pub struct PostgresUnitOfWork {
    pool: Arc<PgPool>,
    caches: PersonCaches,
}

impl PostgresUnitOfWork {
    pub async fn new(pool: Arc<PgPool>) -> Self {
        let executor = Executor::Pool(pool.clone());

        let country_idx_models = CountryRepositoryImpl::load_all_country_idx(&executor)
            .await
            .unwrap();
        let country_idx_cache =
            Arc::new(RwLock::new(CountryIdxModelCache::new(country_idx_models).unwrap()));

        let country_subdivision_idx_models =
            CountrySubdivisionRepositoryImpl::load_all_country_subdivision_idx(&executor)
                .await
                .unwrap();
        let country_subdivision_idx_cache = Arc::new(RwLock::new(
            CountrySubdivisionIdxModelCache::new(country_subdivision_idx_models).unwrap(),
        ));

        let locality_idx_models = LocalityRepositoryImpl::load_all_locality_idx(&executor)
            .await
            .unwrap();
        let locality_idx_cache = Arc::new(RwLock::new(
            LocalityIdxModelCache::new(locality_idx_models).unwrap(),
        ));

        let location_idx_models = LocationRepositoryImpl::load_all_location_idx(&executor)
            .await
            .unwrap();
        let location_idx_cache = Arc::new(RwLock::new(
            LocationIdxModelCache::new(location_idx_models).unwrap(),
        ));

        let person_idx_models = PersonRepositoryImpl::load_all_person_idx(&executor)
            .await
            .unwrap();
        let person_idx_cache =
            Arc::new(RwLock::new(PersonIdxModelCache::new(person_idx_models).unwrap()));

        let messaging_idx_models = MessagingRepositoryImpl::load_all_messaging_idx(&executor)
            .await
            .unwrap();
        let messaging_idx_cache = Arc::new(RwLock::new(
            MessagingIdxModelCache::new(messaging_idx_models).unwrap(),
        ));

        let entity_reference_idx_models =
            EntityReferenceRepositoryImpl::load_all_entity_reference_idx(&executor)
                .await
                .unwrap();
        let entity_reference_idx_cache = Arc::new(RwLock::new(
            EntityReferenceIdxModelCache::new(entity_reference_idx_models).unwrap(),
        ));

        let caches = PersonCaches {
            country_idx_cache,
            country_subdivision_idx_cache,
            locality_idx_cache,
            location_idx_cache,
            person_idx_cache,
            messaging_idx_cache,
            entity_reference_idx_cache,
        };

        Self { pool, caches }
    }
}

#[async_trait]
impl UnitOfWork<Postgres> for PostgresUnitOfWork {
    type Session = PostgresUnitOfWorkSession;
    async fn begin(&self) -> BankingResult<Self::Session> {
        let tx = self.pool.begin().await?;
        Ok(PostgresUnitOfWorkSession::new(tx, self.caches.clone()))
    }
}

pub struct PostgresUnitOfWorkSession {
    tx: crate::repository::executor::Executor,
    audit_logs: Arc<AuditLogRepositoryImpl>,
    persons: Arc<PersonRepositoryImpl>,
    countries: Arc<CountryRepositoryImpl>,
    country_subdivisions: Arc<CountrySubdivisionRepositoryImpl>,
    localities: Arc<LocalityRepositoryImpl>,
    locations: Arc<LocationRepositoryImpl>,
    messagings: Arc<MessagingRepositoryImpl>,
    entity_references: Arc<EntityReferenceRepositoryImpl>,
}

impl PostgresUnitOfWorkSession {
    pub fn new(tx: Transaction<'static, Postgres>, caches: PersonCaches) -> Self {
        let executor =
            crate::repository::executor::Executor::Tx(Arc::new(tokio::sync::Mutex::new(tx)));
        let audit_logs = Arc::new(AuditLogRepositoryImpl::new(executor.clone()));
        let countries =
            Arc::new(CountryRepositoryImpl::new(executor.clone(), caches.country_idx_cache));
        let country_subdivisions = Arc::new(CountrySubdivisionRepositoryImpl::new(
            executor.clone(),
            countries.clone(),
            caches.country_subdivision_idx_cache,
        ));
        let localities = Arc::new(LocalityRepositoryImpl::new(
            executor.clone(),
            country_subdivisions.clone(),
            caches.locality_idx_cache,
        ));
        let locations = Arc::new(LocationRepositoryImpl::new(
            executor.clone(),
            localities.clone(),
            caches.location_idx_cache,
        ));
        let persons = Arc::new(PersonRepositoryImpl::new(
            executor.clone(),
            locations.clone(),
            caches.person_idx_cache,
        ));
        let messagings = Arc::new(MessagingRepositoryImpl::new(
            executor.clone(),
            caches.messaging_idx_cache,
        ));
        let entity_references = Arc::new(EntityReferenceRepositoryImpl::new(
            executor.clone(),
            persons.clone(),
            caches.entity_reference_idx_cache,
        ));

        Self {
            tx: executor,
            audit_logs,
            persons,
            countries,
            country_subdivisions,
            localities,
            locations,
            messagings,
            entity_references,
        }
    }
}

#[async_trait]
impl UnitOfWorkSession<Postgres> for PostgresUnitOfWorkSession {
    type AuditLogRepo = AuditLogRepositoryImpl;
    type PersonRepo = PersonRepositoryImpl;
    type CountryRepo = CountryRepositoryImpl;
    type CountrySubdivisionRepo = CountrySubdivisionRepositoryImpl;
    type LocalityRepo = LocalityRepositoryImpl;
    type LocationRepo = LocationRepositoryImpl;
    type MessagingRepo = MessagingRepositoryImpl;
    type EntityReferenceRepo = EntityReferenceRepositoryImpl;

    fn audit_logs(&self) -> &Self::AuditLogRepo {
        &self.audit_logs
    }

    fn persons(&self) -> &Self::PersonRepo {
        &self.persons
    }

    fn countries(&self) -> &Self::CountryRepo {
        &self.countries
    }

    fn country_subdivisions(&self) -> &Self::CountrySubdivisionRepo {
        &self.country_subdivisions
    }

    fn localities(&self) -> &Self::LocalityRepo {
        &self.localities
    }

    fn locations(&self) -> &Self::LocationRepo {
        &self.locations
    }

    fn messagings(&self) -> &Self::MessagingRepo {
        &self.messagings
    }

    fn entity_references(&self) -> &Self::EntityReferenceRepo {
        &self.entity_references
    }

    async fn commit(self) -> BankingResult<()> {
        if let crate::repository::executor::Executor::Tx(tx_arc) = self.tx {
            let tx = Arc::try_unwrap(tx_arc)
                .expect("Cannot commit transaction with multiple references")
                .into_inner();
            tx.commit().await?;
        }
        Ok(())
    }

    async fn rollback(self) -> BankingResult<()> {
        if let crate::repository::executor::Executor::Tx(tx_arc) = self.tx {
            let tx = Arc::try_unwrap(tx_arc)
                .expect("Cannot rollback transaction with multiple references")
                .into_inner();
            tx.rollback().await?;
        }
        Ok(())
    }
}
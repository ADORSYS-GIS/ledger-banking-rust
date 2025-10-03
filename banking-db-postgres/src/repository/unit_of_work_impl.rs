use async_trait::async_trait;
use banking_api::BankingResult;
use banking_db::{
    models::person::{
        CountryIdxModelCache, CountrySubdivisionIdxModelCache, EntityReferenceIdxModelCache,
        LocalityIdxModelCache, LocationIdxModelCache, PersonIdxModelCache,
    },
    repository::{PersonRepos, TransactionAware, UnitOfWork, UnitOfWorkSession},
};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;

use crate::repository::{
    audit_repository_impl::AuditLogRepositoryImpl,
    executor::Executor,
    person::country_repository::repo_impl::CountryRepositoryImpl,
    person::country_subdivision_repository_impl::CountrySubdivisionRepositoryImpl,
    person::entity_reference_repository_impl::EntityReferenceRepositoryImpl,
    person::locality_repository_impl::LocalityRepositoryImpl,
    person::location_repository_impl::LocationRepositoryImpl,
    person::person_repository_impl::PersonRepositoryImpl,
};

#[derive(Clone)]
pub struct PersonCaches {
    pub country_idx_cache: Arc<RwLock<CountryIdxModelCache>>,
    pub country_subdivision_idx_cache: Arc<RwLock<CountrySubdivisionIdxModelCache>>,
    pub locality_idx_cache: Arc<RwLock<LocalityIdxModelCache>>,
    pub location_idx_cache: Arc<RwLock<LocationIdxModelCache>>,
    pub person_idx_cache: Arc<RwLock<PersonIdxModelCache>>,
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


        let entity_reference_idx_models =
            EntityReferenceRepositoryImpl::load_all_entity_reference_idx(&executor)
                .await
                .expect("Failed to load entity reference index");
        let entity_reference_idx_cache = Arc::new(RwLock::new(
            EntityReferenceIdxModelCache::new(entity_reference_idx_models)
                .expect("Failed to create entity reference index cache"),
        ));

        let caches = PersonCaches {
            country_idx_cache,
            country_subdivision_idx_cache,
            locality_idx_cache,
            location_idx_cache,
            person_idx_cache,
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

pub struct PostgresPersonRepos {
    executor: Executor,
    caches: PersonCaches,
    persons: OnceCell<Arc<PersonRepositoryImpl>>,
    countries: OnceCell<Arc<CountryRepositoryImpl>>,
    country_subdivisions: OnceCell<Arc<CountrySubdivisionRepositoryImpl>>,
    localities: OnceCell<Arc<LocalityRepositoryImpl>>,
    locations: OnceCell<Arc<LocationRepositoryImpl>>,
    entity_references: OnceCell<Arc<EntityReferenceRepositoryImpl>>,
}

impl PostgresPersonRepos {
    fn new(executor: Executor, caches: PersonCaches) -> Self {
        Self {
            executor,
            caches,
            persons: OnceCell::new(),
            countries: OnceCell::new(),
            country_subdivisions: OnceCell::new(),
            localities: OnceCell::new(),
            locations: OnceCell::new(),
            entity_references: OnceCell::new(),
        }
    }
}

impl PersonRepos<Postgres> for PostgresPersonRepos {
    type PersonRepo = PersonRepositoryImpl;
    type CountryRepo = CountryRepositoryImpl;
    type CountrySubdivisionRepo = CountrySubdivisionRepositoryImpl;
    type LocalityRepo = LocalityRepositoryImpl;
    type LocationRepo = LocationRepositoryImpl;
    type EntityReferenceRepo = EntityReferenceRepositoryImpl;

    fn persons(&self) -> &Self::PersonRepo {
        self.persons.get_or_init(|| {
            Arc::new(PersonRepositoryImpl::new(
                self.executor.clone(),
                {
                    self.locations();
                    self.locations
                        .get()
                        .expect("Location repository not initialized")
                        .clone()
                },
                self.caches.person_idx_cache.clone(),
            ))
        })
    }

    fn countries(&self) -> &Self::CountryRepo {
        self.countries.get_or_init(|| {
            Arc::new(CountryRepositoryImpl::new(
                self.executor.clone(),
                self.caches.country_idx_cache.clone(),
            ))
        })
    }

    fn country_subdivisions(&self) -> &Self::CountrySubdivisionRepo {
        self.country_subdivisions.get_or_init(|| {
            Arc::new(CountrySubdivisionRepositoryImpl::new(
                self.executor.clone(),
                {
                    self.countries();
                    self.countries
                        .get()
                        .expect("Country repository not initialized")
                        .clone()
                },
                self.caches.country_subdivision_idx_cache.clone(),
            ))
        })
    }

    fn localities(&self) -> &Self::LocalityRepo {
        self.localities.get_or_init(|| {
            Arc::new(LocalityRepositoryImpl::new(
                self.executor.clone(),
                {
                    self.country_subdivisions();
                    self.country_subdivisions
                        .get()
                        .expect("Country subdivision repository not initialized")
                        .clone()
                },
                self.caches.locality_idx_cache.clone(),
            ))
        })
    }

    fn locations(&self) -> &Self::LocationRepo {
        let location_repo = self.locations.get_or_init(|| {
            Arc::new(LocationRepositoryImpl::new(
                self.executor.clone(),
                {
                    self.localities();
                    self.localities
                        .get()
                        .expect("Locality repository not initialized")
                        .clone()
                },
                self.caches.location_idx_cache.clone(),
            ))
        });
        let locality_repo = self
            .localities
            .get()
            .expect("Locality repository not initialized");
        if locality_repo.location_repository.get().is_none() {
            locality_repo
                .location_repository
                .set(location_repo.clone())
                .ok();
        }
        location_repo
    }


    fn entity_references(&self) -> &Self::EntityReferenceRepo {
        self.entity_references.get_or_init(|| {
            Arc::new(EntityReferenceRepositoryImpl::new(
                self.executor.clone(),
                {
                    self.persons();
                    self.persons
                        .get()
                        .expect("Person repository not initialized")
                        .clone()
                },
                self.caches.entity_reference_idx_cache.clone(),
            ))
        })
    }
}

#[async_trait]
impl TransactionAware for PostgresPersonRepos {
    async fn on_commit(&self) -> BankingResult<()> {
        if let Some(persons) = self.persons.get() {
            persons.on_commit().await?;
        }
        if let Some(countries) = self.countries.get() {
            countries.on_commit().await?;
        }
        if let Some(country_subdivisions) = self.country_subdivisions.get() {
            country_subdivisions.on_commit().await?;
        }
        if let Some(localities) = self.localities.get() {
            localities.on_commit().await?;
        }
        if let Some(locations) = self.locations.get() {
            locations.on_commit().await?;
        }
        if let Some(entity_references) = self.entity_references.get() {
            entity_references.on_commit().await?;
        }
        Ok(())
    }

    async fn on_rollback(&self) -> BankingResult<()> {
        if let Some(persons) = self.persons.get() {
            persons.on_rollback().await?;
        }
        if let Some(countries) = self.countries.get() {
            countries.on_rollback().await?;
        }
        if let Some(country_subdivisions) = self.country_subdivisions.get() {
            country_subdivisions.on_rollback().await?;
        }
        if let Some(localities) = self.localities.get() {
            localities.on_rollback().await?;
        }
        if let Some(locations) = self.locations.get() {
            locations.on_rollback().await?;
        }
        if let Some(entity_references) = self.entity_references.get() {
            entity_references.on_rollback().await?;
        }
        Ok(())
    }
}

/// Represents a single database transaction and provides access to repositories.
///
/// This session implements the Unit of Work pattern, ensuring that all database
/// operations within its scope are part of a single, atomic transaction.
/// Repositories are instantiated on-demand the first time they are accessed,
/// improving performance by avoiding unnecessary object creation.
pub struct PostgresUnitOfWorkSession {
    tx: crate::repository::executor::Executor,
    caches: PersonCaches,
    audit_logs: OnceCell<Arc<AuditLogRepositoryImpl>>,
    person_repos: OnceCell<Arc<PostgresPersonRepos>>,
    observers: Arc<RwLock<Vec<Arc<dyn TransactionAware>>>>,
}

impl PostgresUnitOfWorkSession {
    pub fn new(tx: Transaction<'static, Postgres>, caches: PersonCaches) -> Self {
        let executor =
            crate::repository::executor::Executor::Tx(Arc::new(tokio::sync::Mutex::new(tx)));

        Self {
            tx: executor,
            caches,
            audit_logs: OnceCell::new(),
            person_repos: OnceCell::new(),
            observers: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[async_trait]
impl UnitOfWorkSession<Postgres> for PostgresUnitOfWorkSession {
    type AuditLogRepo = AuditLogRepositoryImpl;
    type PersonRepos = PostgresPersonRepos;

    fn audit_logs(&self) -> &Self::AuditLogRepo {
        self.audit_logs.get_or_init(|| {
            Arc::new(AuditLogRepositoryImpl::new(self.tx.clone()))
        })
    }

    fn person_repos(&self) -> &Self::PersonRepos {
        let person_repos = self.person_repos.get_or_init(|| {
            Arc::new(PostgresPersonRepos::new(
                self.tx.clone(),
                self.caches.clone(),
            ))
        });

        // Initialize all repositories by calling a leaf in the dependency graph.
        // entity_references -> persons -> locations -> localities -> country_subdivisions -> countries
        person_repos.entity_references();

        // Wire the circular dependency between CountrySubdivision and Locality.
        let cs_repo = person_repos
            .country_subdivisions
            .get()
            .expect("Country subdivision repository not initialized");
        let l_repo = person_repos
            .localities
            .get()
            .expect("Locality repository not initialized");
        cs_repo.locality_repository.set(l_repo.clone()).ok();

        self.register_transaction_aware(person_repos.clone());
        person_repos
    }

    fn register_transaction_aware(&self, observer: Arc<dyn TransactionAware>) {
        self.observers.write().push(observer);
    }

    async fn commit(self) -> BankingResult<()> {
        if let crate::repository::executor::Executor::Tx(tx_arc) = self.tx {
            let tx = Arc::try_unwrap(tx_arc)
                .expect("Cannot commit transaction with multiple references")
                .into_inner();
            tx.commit().await?;
        }
        let observers = self.observers.read().clone();
        for observer in observers.iter() {
            observer.on_commit().await?;
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
        let observers = self.observers.read().clone();
        for observer in observers.iter() {
            observer.on_rollback().await?;
        }
        Ok(())
    }
}
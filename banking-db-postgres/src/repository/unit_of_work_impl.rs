use async_trait::async_trait;
use banking_api::BankingResult;
use banking_db::repository::{
    AuditLogRepository, CountryRepository, CountrySubdivisionRepository, EntityReferenceRepository,
    LocationRepository, LocalityRepository, MessagingRepository, PersonRepository, UnitOfWork,
    UnitOfWorkSession,
};
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;

use crate::repository::{
    AuditLogRepositoryImpl, CountryRepositoryImpl, CountrySubdivisionRepositoryImpl,
    EntityReferenceRepositoryImpl, LocalityRepositoryImpl, LocationRepositoryImpl,
    MessagingRepositoryImpl, PersonRepositoryImpl,
};

pub struct PostgresUnitOfWork {
    pool: Arc<PgPool>,
}

impl PostgresUnitOfWork {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UnitOfWork<Postgres> for PostgresUnitOfWork {
    type Session = PostgresUnitOfWorkSession;
    async fn begin(&self) -> BankingResult<Self::Session> {
        let tx = self.pool.begin().await?;
        Ok(PostgresUnitOfWorkSession::new(tx).await)
    }
}

pub struct PostgresUnitOfWorkSession {
    tx: Transaction<'static, Postgres>,
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
    pub async fn new(tx: Transaction<'static, Postgres>) -> Self {
        let audit_logs = Arc::new(AuditLogRepositoryImpl::new(tx.clone()));
        let countries = Arc::new(CountryRepositoryImpl::new(tx.clone()).await);
        let country_subdivisions =
            Arc::new(CountrySubdivisionRepositoryImpl::new(tx.clone(), countries.clone()).await);
        let localities =
            Arc::new(LocalityRepositoryImpl::new(tx.clone(), country_subdivisions.clone()).await);
        let locations =
            Arc::new(LocationRepositoryImpl::new(tx.clone(), localities.clone()).await);
        let persons = Arc::new(PersonRepositoryImpl::new(tx.clone(), locations.clone()).await);
        let messagings = Arc::new(MessagingRepositoryImpl::new(tx.clone()).await);
        let entity_references =
            Arc::new(EntityReferenceRepositoryImpl::new(tx.clone(), persons.clone()).await);

        Self {
            tx,
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
        self.tx.commit().await?;
        Ok(())
    }

    async fn rollback(self) -> BankingResult<()> {
        self.tx.rollback().await?;
        Ok(())
    }
}
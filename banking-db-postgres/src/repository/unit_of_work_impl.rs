use async_trait::async_trait;
use banking_api::BankingResult;
use banking_db::repository::{UnitOfWork, UnitOfWorkSession};
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;

use crate::repository::{
    audit_repository_impl::AuditLogRepositoryImpl,
    person_country_repository_impl::CountryRepositoryImpl,
    person_country_subdivision_repository_impl::CountrySubdivisionRepositoryImpl,
    person_entity_reference_repository_impl::EntityReferenceRepositoryImpl,
    person_locality_repository_impl::LocalityRepositoryImpl,
    person_location_repository_impl::LocationRepositoryImpl,
    person_messaging_repository_impl::MessagingRepositoryImpl,
    person_person_repository_impl::PersonRepositoryImpl,
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
    pub async fn new(tx: Transaction<'static, Postgres>) -> Self {
        let executor =
            crate::repository::executor::Executor::Tx(Arc::new(tokio::sync::Mutex::new(tx)));
        let audit_logs = Arc::new(AuditLogRepositoryImpl::new(executor.clone()));
        let countries = Arc::new(CountryRepositoryImpl::new(executor.clone()).await);
        let country_subdivisions = Arc::new(
            CountrySubdivisionRepositoryImpl::new(executor.clone(), countries.clone()).await,
        );
        let localities = Arc::new(
            LocalityRepositoryImpl::new(executor.clone(), country_subdivisions.clone()).await,
        );
        let locations =
            Arc::new(LocationRepositoryImpl::new(executor.clone(), localities.clone()).await);
        let persons =
            Arc::new(PersonRepositoryImpl::new(executor.clone(), locations.clone()).await);
        let messagings = Arc::new(MessagingRepositoryImpl::new(executor.clone()).await);
        let entity_references = Arc::new(
            EntityReferenceRepositoryImpl::new(executor.clone(), persons.clone()).await,
        );

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
use async_trait::async_trait;
use banking_api::BankingResult;
use sqlx::Database;

use crate::repository::{
    AuditLogRepository, CountryRepository, CountrySubdivisionRepository, EntityReferenceRepository,
    LocationRepository, LocalityRepository, MessagingRepository, PersonRepository,
};

#[async_trait]
pub trait UnitOfWork<DB: Database>: Send + Sync {
    type Session: UnitOfWorkSession<DB>;
    async fn begin(&self) -> BankingResult<Self::Session>;
}

#[async_trait]
pub trait UnitOfWorkSession<DB: Database>: Send + Sync {
    type AuditLogRepo: AuditLogRepository<DB> + Send + Sync;
    type PersonRepo: PersonRepository<DB> + Send + Sync;
    type CountryRepo: CountryRepository<DB> + Send + Sync;
    type CountrySubdivisionRepo: CountrySubdivisionRepository<DB> + Send + Sync;
    type LocalityRepo: LocalityRepository<DB> + Send + Sync;
    type LocationRepo: LocationRepository<DB> + Send + Sync;
    type MessagingRepo: MessagingRepository<DB> + Send + Sync;
    type EntityReferenceRepo: EntityReferenceRepository<DB> + Send + Sync;

    fn audit_logs(&self) -> &Self::AuditLogRepo;
    fn persons(&self) -> &Self::PersonRepo;
    fn countries(&self) -> &Self::CountryRepo;
    fn country_subdivisions(&self) -> &Self::CountrySubdivisionRepo;
    fn localities(&self) -> &Self::LocalityRepo;
    fn locations(&self) -> &Self::LocationRepo;
    fn messagings(&self) -> &Self::MessagingRepo;
    fn entity_references(&self) -> &Self::EntityReferenceRepo;

    async fn commit(self) -> BankingResult<()>;
    async fn rollback(self) -> BankingResult<()>;
}
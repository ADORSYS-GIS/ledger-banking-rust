use async_trait::async_trait;
use banking_api::BankingResult;
use sqlx::Database;

use crate::repository::{
    AuditLogRepository, PersonRepos,
};

#[async_trait]
pub trait UnitOfWork<DB: Database>: Send + Sync {
    type Session: UnitOfWorkSession<DB>;
    async fn begin(&self) -> BankingResult<Self::Session>;
}

#[async_trait]
pub trait UnitOfWorkSession<DB: Database>: Send + Sync {
    type AuditLogRepo: AuditLogRepository<DB> + Send + Sync;
    type PersonRepos: PersonRepos<DB> + Send + Sync;

    fn audit_logs(&self) -> &Self::AuditLogRepo;
    fn person_repos(&self) -> &Self::PersonRepos;

    async fn commit(self) -> BankingResult<()>;
    async fn rollback(self) -> BankingResult<()>;
}
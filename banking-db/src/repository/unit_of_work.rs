use async_trait::async_trait;
use banking_api::BankingResult;
use sqlx::Database;
use std::sync::Arc;

use crate::repository::{AuditLogRepository, PersonRepository};

#[async_trait]
pub trait UnitOfWork<DB: Database>: Send + Sync {
    type Session: UnitOfWorkSession<DB>;
    async fn begin(&self) -> BankingResult<Self::Session>;
}

#[async_trait]
pub trait UnitOfWorkSession<DB: Database>: Send + Sync {
    type AuditLogRepo: AuditLogRepository<DB> + Send + Sync;
    type PersonRepo: PersonRepository<DB> + Send + Sync;

    fn audit_logs(&self) -> Arc<Self::AuditLogRepo>;
    fn persons(&self) -> Arc<Self::PersonRepo>;

    async fn commit(self) -> BankingResult<()>;
    async fn rollback(self) -> BankingResult<()>;
}
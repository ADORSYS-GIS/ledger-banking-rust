use async_trait::async_trait;
use banking_api::BankingResult;
use banking_db::repository::{AuditLogRepository, PersonRepository, UnitOfWork, UnitOfWorkSession};
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;

use crate::repository::{AuditLogRepositoryImpl, PersonRepositoryImpl};

pub struct PostgresUnitOfWork {
    pool: PgPool,
}

impl PostgresUnitOfWork {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UnitOfWork for PostgresUnitOfWork {
    async fn begin(&self) -> BankingResult<Box<dyn UnitOfWorkSession>> {
        let tx = self.pool.begin().await?;
        Ok(Box::new(PostgresUnitOfWorkSession::new(tx)))
    }
}

pub struct PostgresUnitOfWorkSession {
    tx: Transaction<'static, Postgres>,
}

impl PostgresUnitOfWorkSession {
    pub fn new(tx: Transaction<'static, Postgres>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl UnitOfWorkSession for PostgresUnitOfWorkSession {
    fn audit_logs(&self) -> Arc<dyn AuditLogRepository> {
        Arc::new(AuditLogRepositoryImpl::new(self.tx.clone()))
    }

    fn persons(&self) -> Arc<dyn PersonRepository> {
        Arc::new(PersonRepositoryImpl::new(self.tx.clone()))
    }

    async fn commit(self: Box<Self>) -> BankingResult<()> {
        self.tx.commit().await?;
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> BankingResult<()> {
        self.tx.rollback().await?;
        Ok(())
    }
}
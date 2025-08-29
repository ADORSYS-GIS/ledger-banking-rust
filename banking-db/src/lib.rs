use sqlx::{Postgres, Transaction, pool::PoolConnection, PgPool};
use async_trait::async_trait;

pub mod models;
pub mod repository;
pub mod utils;

#[async_trait]
pub trait DatabaseExecutor: Send + Sync {
    async fn begin(&self) -> Result<Transaction<'_, Postgres>, sqlx::Error>;
    async fn acquire(&self) -> Result<PoolConnection<Postgres>, sqlx::Error>;
}

#[async_trait]
impl DatabaseExecutor for PgPool {
    async fn begin(&self) -> Result<Transaction<'_, Postgres>, sqlx::Error> {
        self.begin().await
    }

    async fn acquire(&self) -> Result<PoolConnection<Postgres>, sqlx::Error> {
        self.acquire().await
    }
}
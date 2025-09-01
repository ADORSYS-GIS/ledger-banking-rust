use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;

/// A handle to a database executor, which can be either a connection pool
/// or an active transaction. Using `Arc<Mutex<...>>` for the transaction
/// allows it to be shared across multiple repository instances within the
/// same unit of work.
#[derive(Clone)]
pub enum Executor {
    Pool(Arc<PgPool>),
    Tx(Arc<Mutex<Transaction<'static, Postgres>>>),
}
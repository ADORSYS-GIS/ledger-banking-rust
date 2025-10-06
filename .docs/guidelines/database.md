# Database Guidelines

## Transactional Query Execution

In this project, database queries are primarily executed within a transactional context using the `UnitOfWork` pattern, as detailed in the [Transactional Command Execution Pattern](./transactional-command.md). Repositories are provided with an `Executor` that abstracts over a direct connection pool and a transaction.

### The `Executor` Enum

To allow repository methods to operate both within a transaction and with a direct connection pool, we use an `Executor` enum.

```rust
pub enum Executor {
    Pool(Arc<PgPool>),
    Tx(Arc<Mutex<Transaction<'static, Postgres>>>),
}
```

### Repository Query Pattern

The following example shows the standard pattern for executing a query within a repository method. It correctly handles both transactional (`Executor::Tx`) and non-transactional (`Executor::Pool`) cases. This ensures that when an operation is part of a larger command, it correctly participates in the ongoing transaction.

```rust
// ✅ Correct: Execute query using the Executor pattern
let query = sqlx::query(
    "INSERT INTO accounts (account_type, account_status) VALUES ($1::account_type, $2::account_status)"
)
.bind(account.account_type)
.bind(account.account_status);

let result = match &self.executor {
    Executor::Pool(pool) => query.execute(&**pool).await?,
    Executor::Tx(tx) => {
        let mut tx = tx.lock().await;
        query.execute(&mut **tx).await?
    }
};
```

## Type Mappings (Rust → PostgreSQL)

- `Uuid` → `UUID`
- `HeaplessString<N>` → `VARCHAR(N)`
- `Decimal` → `DECIMAL(15,2)`
- `DateTime<Utc>` → `TIMESTAMP WITH TIME ZONE`

## Database Mapping Rules

- Foreign key constraints must be enforced in the repository layer, not the database. Use database schema for structure only, with no foreign key constraints.
- Date fields (e.g. `created_at`, `updated_at`) must be initialized in the domain layer. Database triggers for timestamp management are prohibited.
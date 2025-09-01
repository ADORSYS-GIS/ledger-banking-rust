# Transactional Command Execution Pattern

This document outlines the pattern for creating and executing commands within a transactional context in the ledger-banking-rust project. This pattern ensures that all database operations within a single command are atomic, meaning they either all succeed or all fail together.

## Core Components

1.  **`UnitOfWork` Trait**: Defined in `banking-db`, this trait provides a `begin` method to start a new transactional session.
2.  **`UnitOfWorkSession` Trait**: Also in `banking-db`, this trait represents an active transaction. It provides access to all the repository instances that will operate within this transaction.
3.  **`Command` Trait**: Defined in `banking-api`, this trait represents a single, executable action. Its `execute` method now takes a `Services` struct as its context.
4.  **`Services` Struct**: A simple struct in `banking-api` that holds all the service traits required by the commands (e.g., `Arc<dyn PersonService>`).
5.  **`ServiceFactory` Trait**: Defined in `banking-logic`, this trait is responsible for creating the `Services` struct from a `UnitOfWorkSession`. The concrete implementation of this factory will be provided at the application's composition root (e.g., in `main.rs`).
6.  **`CommandExecutor` Trait**: A generic trait in `banking-api` that defines how to execute a command.
7.  **`CommandExecutorImpl`**: The concrete implementation in `banking-logic`. This struct is generic over the database type and a `ServiceFactory`. It manages the transaction lifecycle: begin, execute, commit/rollback.

## Workflow

1.  An application component (e.g., an API endpoint handler) receives a request to perform an action.
2.  It creates a command object (e.g., `AddPersonOfInterestCommand`) with the necessary data.
3.  It invokes the `execute` method on the `CommandExecutorImpl`, passing in the `UnitOfWork` instance and the command.
4.  The `CommandExecutorImpl` begins a new transaction by calling `uow.begin()`.
5.  It uses the `ServiceFactory` to create a `Services` struct, which contains instances of all the necessary services, all sharing the same transactional session.
6.  It calls the `execute` method on the command, passing the `Services` struct as the context.
7.  The command's `execute` method uses the services to perform its business logic. All database operations performed by the services will be part of the same transaction.
8.  If the command's execution is successful, the `CommandExecutorImpl` commits the transaction.
9.  If the command's execution fails at any point, the `CommandExecutorImpl` rolls back the transaction, undoing all changes.

## Panic Safety and Transaction Rollback

A key aspect of this pattern's robustness is its handling of panics. If a panic occurs during the execution of a command, the transaction must be rolled back to prevent the database from being left in an inconsistent state.

The `PostgresUnitOfWorkSession` holds an `sqlx::Transaction`. The `sqlx::Transaction` type is designed to automatically roll back the transaction when it is dropped if it has not been explicitly committed. This is a standard RAII (Resource Acquisition Is Initialization) pattern in Rust.

Therefore, **there is no need to add an explicit `Drop` implementation to `PostgresUnitOfWorkSession`**. If a panic occurs, the stack will unwind, the `PostgresUnitOfWorkSession` will be dropped, which in turn drops the `sqlx::Transaction`, triggering an automatic rollback. This ensures that the transaction is always cleaned up correctly, even in the case of unexpected panics.


## Caching Strategy

To optimize performance, the system employs in-memory caches for frequently accessed, rarely changing data (e.g., countries, subdivisions).

-   **Cache Lifecycle**: Caches are initialized once when the `PostgresUnitOfWork` is created. This ensures that all subsequent transactions share the same cache instances, avoiding redundant database queries.
-   **Cache Propagation**: The `PostgresUnitOfWork` holds the caches and passes them down to the `PostgresUnitOfWorkSession` when a transaction begins. The session then provides the caches to the repositories that need them.
-   **Implementation**: The caches are implemented using `parking_lot::RwLock` for efficient, thread-safe access.

## Repository Implementation with `Executor`

To allow repository methods to operate both within a transaction and with a direct connection pool, we use an `Executor` enum. This pattern is crucial for code reuse and flexibility, as some operations might not need to be part of a larger transaction.

### The `Executor` Enum

Defined in `banking-db-postgres/src/repository/executor.rs`, the `Executor` is a simple enum that holds either a database pool or a transaction:

```rust
#[derive(Clone)]
pub enum Executor {
    Pool(Arc<PgPool>),
    Tx(Arc<Mutex<Transaction<'static, Postgres>>>),
}
```

-   `Executor::Pool`: Wraps an `Arc<PgPool>`, representing a connection to the database connection pool. Used for non-transactional operations.
-   `Executor::Tx`: Wraps an `Arc<Mutex<Transaction<'...>>>`, representing an active database transaction. This is used when repositories are created within a `UnitOfWorkSession`. The `Mutex` is necessary to allow mutable access to the transaction across `async` calls.

### Usage in Repositories

Each repository implementation holds an instance of `Executor`. Inside the repository methods, a `match` statement is used to get the underlying database executor (`&PgPool` or `&mut Transaction`).

This allows the same method to execute a query against either a standalone connection or within an ongoing transaction.

Here is a typical implementation within a repository method:

```rust
// Inside a repository method
let query = sqlx::query("SELECT * FROM country WHERE id = $1").bind(id);

let row = match &self.executor {
    Executor::Pool(pool) => {
        // Execute directly on the pool
        query.fetch_one(&**pool).await?
    }
    Executor::Tx(tx) => {
        // Lock the mutex and execute on the transaction
        let mut tx = tx.lock().await;
        query.fetch_one(&mut **tx).await?
    }
};
```

This pattern centralizes the logic for handling both transactional and non-transactional database access, making the repositories clean and adaptable. When a `UnitOfWorkSession` is created, it instantiates repositories with an `Executor::Tx`, ensuring all subsequent operations are part of the same transaction.
## Example Usage

```rust
// In main.rs or a similar composition root

// 1. Create the database pool and the UnitOfWork
let pool = Arc::new(PgPoolOptions::new().connect(&db_url).await?);
let uow = Arc::new(PostgresUnitOfWork::new(pool.clone()).await);

// 2. Create the concrete ServiceFactory
struct AppServiceFactory;
impl ServiceFactory<Postgres, PostgresUnitOfWorkSession> for AppServiceFactory {
    fn build_services(&self, session: &PostgresUnitOfWorkSession) -> Services {
        let repositories = Repositories {
            person_repository: Arc::new(session.persons().clone()),
            // ... other repositories
        };
        let person_service = Arc::new(PersonServiceImpl::new(repositories));
        Services { person_service }
    }
}

// 3. Create the CommandExecutor
let command_executor = CommandExecutorImpl::new(AppServiceFactory);

// 4. Create and execute a command
let command = PersonCommand::AddPersonOfInterest(Box::new(AddPersonOfInterestCommand {
    person_data: person,
    audit_log: audit_log,
}));

let result = command_executor.execute(uow, command).await?;
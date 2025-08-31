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

## Example Usage

```rust
// In main.rs or a similar composition root

// 1. Create the database pool and the UnitOfWork
let pool = Arc::new(PgPoolOptions::new().connect(&db_url).await?);
let uow = Arc::new(PostgresUnitOfWork::new(pool.clone()));

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
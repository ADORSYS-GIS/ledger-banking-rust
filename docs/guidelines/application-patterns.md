# Application Design Patterns

## Command Pattern for Business Logic Execution

To maintain a clean separation of concerns and ensure that business logic is decoupled from the API/delivery layer, the system uses a Command pattern. This pattern encapsulates all the information needed to perform an action into a single object (a "command").

### Core Components

1.  **`Command` Trait (`banking-api/src/command/command.rs`)**: The core trait that defines what a command is. It has an `execute` method and associated types for the `Context` (dependencies like services) and the `Result`.

    ```rust
    #[async_trait]
    pub trait Command: Send + Sync {
        type Context;
        type Result: Send + 'static;

        async fn execute(&self, context: &Self::Context) -> Result<Self::Result, BankingError>;
    }
    ```

2.  **Concrete Command Structs** (e.g., `banking-api/src/command/person.rs`): These are the actual implementations of the `Command` trait. Each struct holds the specific data required for its operation.

    ```rust
    // Example: Command to populate geographical data
    pub struct PopulateGeoDataCommand {
        pub json_data: String,
    }

    #[async_trait]
    impl Command for PopulateGeoDataCommand {
        type Context = Arc<dyn PersonService>;
        type Result = ();

        async fn execute(&self, context: &Self::Context) -> Result<Self::Result, BankingError> {
            // ... implementation ...
        }
    }
    ```

3.  **`AppCommand` Enum (`banking-api/src/command/command.rs`)**: A single enum that wraps all possible commands in the application. This provides a unified interface for the command executor.

    ```rust
    pub enum AppCommand {
        AddPersonOfInterest(AddPersonOfInterestCommand),
        PopulateGeoData(super::person::PopulateGeoDataCommand),
        // ... other commands
    }
    ```

4.  **`CommandExecutor` Trait & Implementation (`banking-logic/src/commands/executor.rs`)**: The executor is responsible for running the commands. It takes an `AppCommand`, matches on the variant, and calls the command's `execute` method with the correct dependencies.

    ```rust
    // Trait in banking-api
    #[async_trait]
    pub trait CommandExecutor: Send + Sync {
        async fn execute(&self, command: AppCommand) -> Result<CommandResult, BankingError>;
    }

    // Implementation in banking-logic
    pub struct CommandExecutorImpl {
        person_service: Arc<dyn PersonService>,
        // ... other services
    }

    #[async_trait::async_trait]
    impl CommandExecutor for CommandExecutorImpl {
        async fn execute(&self, command: AppCommand) -> Result<CommandResult, BankingError> {
            match command {
                AppCommand::PopulateGeoData(cmd) => {
                    let result = cmd.execute(&self.person_service).await?;
                    Ok(Box::new(result) as Box<dyn Any + Send>)
                }
                // ... other command handlers
            }
        }
    }
    ```

### How to Add a New Command

1.  **Define the Command Struct**: Create a new struct in the appropriate module within `banking-api/src/command/`. This struct will hold the data for the command.
2.  **Implement the `Command` Trait**: Implement the `Command` trait for your new struct. Define the `Context` (the service it needs) and the `Result` type. Implement the `execute` logic.
3.  **Add to `AppCommand` Enum**: Add a new variant to the `AppCommand` enum in `banking-api/src/command/command.rs` to include your new command.
4.  **Update the `CommandExecutorImpl`**: Add a new match arm in `CommandExecutorImpl::execute` in `banking-logic/src/commands/executor.rs`. This arm will handle your new command, call its `execute` method with the required service, and wrap the result.
5.  **Inject Dependencies**: If your command requires a new service, make sure to add it to `CommandExecutorImpl` and inject it during its construction.

## Transaction Management with the Unit of Work Pattern

To ensure data consistency across multiple repository operations, the system uses a Unit of Work pattern. This pattern guarantees that a series of operations are executed within a single database transaction, which can be either committed or rolled back as a single atomic unit.

### Core Components

1.  **`UnitOfWork` Trait (`banking-db/src/repository/unit_of_work.rs`)**: The core trait that defines the pattern. It has a `begin` method that starts a new session.

    ```rust
    #[async_trait]
    pub trait UnitOfWork<DB: Database>: Send + Sync {
        type Session: UnitOfWorkSession<DB>;
        async fn begin(&self) -> BankingResult<Self::Session>;
    }
    ```

2.  **`UnitOfWorkSession` Trait (`banking-db/src/repository/unit_of_work.rs`)**: Represents an active session with repository access and transaction control.

    ```rust
    #[async_trait]
    pub trait UnitOfWorkSession<DB: Database>: Send + Sync {
        // ... repository accessors
        async fn commit(self) -> BankingResult<()>;
        async fn rollback(self) -> BankingResult<()>;
    }
    ```

3.  **Implementation (`banking-db-postgres/src/repository/unit_of_work_impl.rs`)**: The PostgreSQL-specific implementation that manages `sqlx::Transaction`.

### How to Use the Unit of Work

To perform multiple repository operations within a single transaction, you must use a `UnitOfWorkSession`.

1.  **Begin a Session**: Start by calling `begin()` on a `UnitOfWork` instance.
2.  **Access Repositories**: Use the session to get repository instances. All operations on these repositories will be part of the same transaction.
3.  **Commit or Rollback**: Once all operations are complete, call `commit()` to save the changes or `rollback()` to discard them.

```rust
// Example: Creating a person and an audit log in a single transaction
async fn create_person_with_audit(
    uow: &PostgresUnitOfWork,
    display_name: &str,
) -> BankingResult<Person> {
    let session = uow.begin().await?;

    let audit_log_repo = session.audit_logs();
    let person_repo = session.persons();

    let audit_log = audit_log_repo.create(/* ... */).await?;
    let person = person_repo.create(display_name, audit_log.id).await?;

    session.commit().await?;

    Ok(person)
}
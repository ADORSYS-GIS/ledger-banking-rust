### Test Isolation with Unit of Work

Repository and service tests use a **Unit of Work** pattern to ensure complete test isolation. Each test runs within a database transaction that is **automatically rolled back** at the end of the test. This approach eliminates data pollution between tests and allows them to run in parallel without interference.

The core of this pattern is the `setup_test_context` helper function, which provides a `TestContext` for each test.

```rust
#[tokio::test]
async fn test_with_transactional_isolation() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Arrange: Set up the test context. This begins a transaction.
    let ctx = setup_test_context().await?;
    
    // Get a repository that operates within the transaction
    let person_repo = ctx.person_repos().persons();

    // 2. Act: Perform database operations
    let new_person = create_test_person_model("John Doe");
    let saved_person = person_repo.save(new_person.clone(), Uuid::new_v4()).await?;

    // 3. Assert: Verify the results
    let found_person = person_repo.find_by_id(saved_person.id).await?;
    assert!(found_person.is_some());

    Ok(())
} // <- The transaction is automatically rolled back here when `ctx` is dropped
```

### Database Testing

Database tests can now run in parallel, as each test is perfectly isolated within its own transaction.

```bash
# Set the database URL (from project root)
export DATABASE_URL="postgresql://user:password@localhost:5432/mydb"

# Run all tests for a specific package (can run in parallel)
cargo test -p banking-db-postgres

# Schema changes (from project root)
docker compose down -v && docker compose up -d postgres
sqlx migrate run --source banking-db-postgres/migrations
```

## Service Implementation Testing

When a service implementation (e.g., `PersonServiceImpl`) is complete, it is crucial to create a corresponding test suite to ensure its correctness and maintainability. The primary goal is to have a dedicated test for every public method in the service.

### 1. Test File Structure and Co-location

To ensure that tests are closely coupled with the implementation they verify, unit tests for a given function must be co-located within the same file as the function itself. This is achieved by placing the tests inside a `#[cfg(test)]` module at the end of the file.

This approach improves discoverability and makes it easier to maintain tests alongside the code they cover, whether for repository functions or service implementations.

**Example File Structure (`save.rs`):**

```rust
// In banking-db-postgres/src/repository/person/country_repository/save.rs

use crate::repository::executor::Executor;
// ... other imports

pub(crate) async fn save(
    repo: &CountryRepositoryImpl,
    country: CountryModel,
) -> CountryResult<CountryModel> {
    // ... function implementation
}

#[cfg(test)]
mod tests {
    use crate::test_helper::setup_test_context;
    // ... other test imports

    #[tokio::test]
    async fn test_save_country() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // ... test implementation
    }
}
```

This co-location strategy is the standard for unit-testing individual functions. Higher-level integration tests that span multiple components or validate complex workflows may still reside in the `tests/` directory.

### 2. Mocking Dependencies

Service tests must be isolated from the database. This is achieved by creating mock implementations for each repository dependency directly within the test module of the service file.

**Co-located Mock Implementation:**

By defining mocks inside the `#[cfg(test)]` block, they are kept private to the test environment and live alongside the service implementation that uses them.

```rust
// In banking-logic/src/services/person/person_service_impl.rs

// ... service implementation ...

#[cfg(test)]
mod tests {
    // ... other test imports

    #[derive(Default)]
    struct MockCountryRepository {
        // ... mock fields
    }

    #[async_trait]
    impl CountryRepository<Postgres> for MockCountryRepository {
        async fn save(&self, country: CountryModel) -> CountryResult<CountryModel> {
            // ... mock implementation
        }
        // ... other mocked methods
    }
}
```

### 3. Test Setup Helpers

With co-located tests, setup logic is also defined within the `#[cfg(test)]` module. This eliminates the need for a separate `common.rs` file and keeps test helpers close to the tests that use them.

**Co-located Service Instantiation:**

A helper function can create an instance of the service with its mocked dependencies for use in tests.

```rust
// In banking-logic/src/services/person/person_service_impl.rs

#[cfg(test)]
mod tests {
    // ... imports and mock definitions ...

    fn setup_test_service() -> (PersonServiceImpl<Postgres>, Arc<MockPersonRepository>) {
        let mock_person_repo = Arc::new(MockPersonRepository::default());
        
        let repositories = Repositories {
            person_repository: mock_person_repo.clone(),
            // ... other mocked repositories initialized as needed
        };

        let service = PersonServiceImpl::new(repositories);
        (service, mock_person_repo)
    }

    // Other local helpers can be defined here
    fn create_test_person() -> Person {
        // ...
    }
}
```

### 4. Writing Unit Tests

Tests are written as standard functions within the `tests` module, following the Arrange-Act-Assert pattern.

**Test Structure (Arrange-Act-Assert):**

```rust
// In banking-logic/src/services/person/person_service_impl.rs

#[cfg(test)]
mod tests {
    // ... imports, mocks, and helpers ...

    #[tokio::test]
    async fn test_create_person() {
        // 1. Arrange: Set up the service and test data
        let (service, mock_repo) = setup_test_service();
        let person = create_test_person();

        // 2. Act: Call the service method
        let created_person = service
            .create_person(person.clone())
            .await
            .unwrap();

        // 3. Assert: Verify the outcome
        assert_eq!(person.id, created_person.id);
        // Optionally, assert that mock was called
        assert_eq!(mock_repo.calls.lock().unwrap().len(), 1);
    }
}
```

### 5. Running Tests

With tests co-located in the source files, you can run all tests for a crate using a standard cargo command from the project root.

```bash
cargo test -p banking-logic
```

Cargo will automatically discover and run all functions annotated with `#[test]`, including those inside `#[cfg(test)]` modules.

## Repository Integration Testing

Repository tests are integration tests that validate the PostgreSQL implementations against a live database. They ensure that SQL queries, data mapping, and repository logic function correctly.

### 1. Test File Structure

Unit tests for repository functions are co-located in the same file as the implementation. This is the standard for testing individual database operations like `save`, `load`, or `find_by_id`.

The `banking-db-postgres/tests/` directory is reserved for higher-level integration tests that validate complex interactions, batch operations, or workflows spanning multiple repository methods.

### 2. Database Connection and Isolation

All repository tests are isolated using the **Unit of Work** pattern. The `setup_test_context` helper function, located in `banking-db-postgres/src/repository/person/test_helpers.rs`, provides a transactional session for each test.

-   **Transactional Context**: Call `setup_test_context().await?` at the beginning of each test to get a `TestContext`.
-   **Error Handling**: Test functions should return `Result<(), Box<dyn std::error::Error + Send + Sync>>` and use the `?` operator for concise error propagation. This avoids using `.unwrap()` and provides consistent error handling across the test suite.
-   **Automatic Rollback**: The transaction is automatically rolled back when the `TestContext` goes out of scope, ensuring a clean database state for subsequent tests.

```rust
use crate::repository::person::test_helpers::setup_test_context;

#[tokio::test]
async fn test_my_repository() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Arrange: Set up the transactional context
    let ctx = setup_test_context().await?;
    
    // Get a repository from the context
    let repo = ctx.person_repos().persons();
    
    // ... rest of the test
    Ok(())
}
```

### 3. Test Data Generation

Reusable test data helpers should be placed in a dedicated `test_helpers.rs` file within the same module as the repository implementation.

-   **Centralization**: A `test_helpers.rs` file in a module (e.g., `country_repository/`) can contain `pub` functions to create test models (`setup_test_country`).
-   **Reusability**: These public helper functions can be easily imported and used by the co-located unit tests for `save.rs`, `load.rs`, etc., as well as by higher-level integration tests in the `tests/` directory.

```rust
// In banking-db-postgres/src/repository/person/country_repository/test_helpers.rs
use banking_db::models::person::CountryModel;
// ... other imports

pub async fn setup_test_country() -> CountryModel {
    // ... implementation to create a valid CountryModel
}

// In banking-db-postgres/src/repository/person/country_repository/load.rs
#[cfg(test)]
mod tests {
    use crate::repository::person::country_repository::test_helpers::setup_test_country;
    use crate::repository::person::test_helpers::setup_test_context;
    // ...

    #[tokio::test]
    async fn test_load_country() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();
        let mut country_model = setup_test_country().await;
        // ...
    }
}
```

### 4. Writing Repository Tests

Structure co-located tests using the **Arrange-Act-Assert** pattern. Each test function should be focused on a single behavior of the repository function in that file.

```rust
// In banking-db-postgres/src/repository/person/country_repository/save.rs

#[cfg(test)]
mod tests {
    use crate::repository::person::country_repository::test_helpers::setup_test_country;
    use crate::repository::person::test_helpers::setup_test_context;
    use banking_db::repository::person::country_repository::{
        CountryRepository, CountryRepositoryError,
    };
    use banking_db::repository::PersonRepos;

    #[tokio::test]
    async fn test_save_country() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 1. Arrange
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();
        let country_model = setup_test_country().await;

        // 2. Act
        let saved_country = country_repo.save(country_model.clone()).await?;

        // 3. Assert
        assert_eq!(saved_country.id, country_model.id);

        // Arrange for failure case
        let result = country_repo.save(country_model).await;
        
        // Assert failure case
        assert!(matches!(
            result,
            Err(CountryRepositoryError::DuplicateCountryISO2(_))
        ));

        Ok(())
    }
}
```

### 5. Running Repository Tests

Repository tests can now be run in parallel, thanks to transaction-based isolation.

```bash
# Set the database URL
export DATABASE_URL="postgresql://user:password@localhost:5432/mydb"

# Run all integration tests in parallel
cargo test -p banking-db-postgres -- --test-threads=1

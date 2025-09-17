### Test Isolation with Unit of Work

Repository and service tests use a **Unit of Work** pattern to ensure complete test isolation. Each test runs within a database transaction that is **automatically rolled back** at the end of the test. This approach eliminates data pollution between tests and allows them to run in parallel without interference.

The core of this pattern is the `setup_test_context` helper function, which provides a `TestContext` for each test.

```rust
#[tokio::test]
async fn test_with_transactional_isolation() {
    // 1. Arrange: Set up the test context. This begins a transaction.
    let ctx = setup_test_context().await.unwrap();
    
    // Get a repository that operates within the transaction
    let person_repo = ctx.person_repos().persons();

    // 2. Act: Perform database operations
    let new_person = create_test_person_model("John Doe");
    let saved_person = person_repo.save(new_person.clone(), Uuid::new_v4()).await.unwrap();

    // 3. Assert: Verify the results
    let found_person = person_repo.find_by_id(saved_person.id).await.unwrap();
    assert!(found_person.is_some());

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

### 1. Test File Structure

For a service like `PersonService`, located at `banking-logic/src/services/person/person_service_impl.rs`, the corresponding tests should be organized into a dedicated module at `banking-logic/tests/person/`.

This modular structure enhances clarity and scalability. The main test file, `banking-logic/tests/person_module_tests.rs`, simply declares the module:

```rust
// In banking-logic/tests/person_module_tests.rs
mod person;
```

The directory structure for the `person` service tests would look like this:

```
banking-logic/tests/
├── person_module_tests.rs  // Main test entry point
└── person/
    ├── mod.rs                 // Declares all sub-modules
    ├── common.rs              // Shared test helpers (e.g., service setup)
    ├── country_tests.rs       // Tests for Country related logic
    ├── locality_tests.rs      // Tests for Locality related logic
    ├── ...                    // Other test files for different entities
    ├── mock_country_repository.rs // Mock implementation for CountryRepository
    ├── mock_locality_repository.rs// Mock implementation for LocalityRepository
    └── ...                    // Other mock repository files
```

The `person/mod.rs` file ties everything together:

```rust
// In banking-logic/tests/person/mod.rs
pub mod common;
pub mod country_tests;
pub mod mock_country_repository;
// ... other modules
```

### 2. Mocking Dependencies

Service tests must be isolated from the database. This is achieved by creating mock implementations for each repository dependency.

Each mock repository should reside in its own file within the test module. For example, `MockCountryRepository` is located in `banking-logic/tests/person/mock_country_repository.rs`.

**Mock Repository Implementation:**

```rust
// In banking-logic/tests/person/mock_country_repository.rs

use std::sync::Mutex;
// ... other necessary imports

#[derive(Default)]
pub struct MockCountryRepository {
    countries: Mutex<Vec<CountryModel>>,
    country_ixes: Mutex<Vec<CountryIdxModel>>,
}

#[async_trait]
impl CountryRepository<Postgres> for MockCountryRepository {
    async fn save(&self, country: CountryModel) -> CountryResult<CountryModel> {
        // ... implementation to save to in-memory Vecs
    }

    // ... other required trait methods
}

// Helper for creating test data can also be included here
pub fn create_test_country() -> Country {
    // ...
}
```

This approach ensures that each mock is self-contained and easy to locate.

### 3. Test Setup Helpers

To keep tests clean, a `common.rs` file within the test module centralizes setup logic.

**Service Instantiation (`common.rs`):**

This file contains a `TestServices` struct and a helper function to instantiate all services with their mocked dependencies.

```rust
// In banking-logic/tests/person/common.rs

// ... imports for all services and mock repositories

pub struct TestServices {
    pub country_service: CountryServiceImpl<Postgres>,
    // ... other services
}

pub fn create_test_services() -> TestServices {
    let repositories = Repositories {
        country_repository: Arc::new(MockCountryRepository::default()),
        // ... other mocked repositories
    };
    TestServices {
        country_service: CountryServiceImpl::new(repositories.clone()),
        // ... instantiating other services
    }
}

// Other common helpers like creating an audit log
pub fn create_test_audit_log() -> banking_api::domain::AuditLog {
    // ...
}
```

### 4. Writing Unit Tests

Each entity or logical group of functions within the service gets its own test file (e.g., `country_tests.rs`).

**Test Structure (Arrange-Act-Assert):**

```rust
// In banking-logic/tests/person/country_tests.rs

use crate::person::common::create_test_services;
use crate::person::mock_country_repository::create_test_country;
use banking_api::service::CountryService;

#[tokio::test]
async fn test_create_country() {
    // 1. Arrange: Set up the services and test data
    let services = create_test_services();
    let country = create_test_country();

    // 2. Act: Call the service method
    let created_country = services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();

    // 3. Assert: Verify the outcome
    assert_eq!(country.id, created_country.id);
}
```

### 5. Running Tests

Run the tests for the specific service from the project root:

```bash
cargo test -p banking-logic --test person_module_tests
```

This modular structure keeps the test suite organized, maintainable, and easy to navigate, even as the number of tests grows.

## Repository Integration Testing

Repository tests are integration tests that validate the PostgreSQL implementations against a live database. They ensure that SQL queries, data mapping, and repository logic function correctly.

### 1. Test File Structure

For repositories related to a specific domain, like `person`, tests are organized into a dedicated module. For a repository like `PersonRepositoryImpl`, the tests are located at `banking-db-postgres/tests/suites/person/`.

The main test file for the suites, `banking-db-postgres/tests/suites/mod.rs`, declares the `person` module:

```rust
// In banking-db-postgres/tests/suites/mod.rs
#[path = "person/mod.rs"]
pub mod person;
// ... other suite modules
```

The directory structure for the `person` repository tests is as follows:

```
banking-db-postgres/tests/suites/
├── mod.rs       // Main suite entry point, declares the person module
└── person/
    ├── mod.rs   // Declares all repository test sub-modules
    ├── helpers.rs // Shared helper functions for creating test models
    ├── person_repository_tests.rs
    ├── country_repository_tests.rs
    └── ...      // Other repository test files
```

The `person/mod.rs` file includes all the individual test files:

```rust
// In banking-db-postgres/tests/suites/person/mod.rs
pub mod helpers;
pub mod person_repository_tests;
pub mod country_repository_tests;
// ... other modules
```

### 2. Database Connection and Isolation

All repository tests are isolated using the **Unit of Work** pattern. The `setup_test_context` helper function, located in `banking-db-postgres/tests/suites/test_helper.rs`, provides a transactional session for each test.

-   **Transactional Context**: Call `setup_test_context().await` at the beginning of each test to get a `TestContext`.
-   **Automatic Rollback**: The transaction is automatically rolled back when the `TestContext` goes out of scope, ensuring a clean database state for subsequent tests.

```rust
use crate::suites::test_helper::setup_test_context;

#[tokio::test]
async fn test_my_repository() {
    // 1. Arrange: Set up the transactional context
    let ctx = setup_test_context().await.unwrap();
    
    // Get a repository from the context
    let repo = ctx.person_repos().persons();
    
    // ... rest of the test
}
```

### 3. Test Data Generation

Helper functions for creating test models are centralized in `person/helpers.rs`.

```rust
// In banking-db-postgres/tests/suites/person/helpers.rs
use banking_db::models::person::{CountryModel, ...};
// ... other imports

pub fn create_test_country_model(iso2: &str, name_l1: &str) -> CountryModel {
    // ... implementation
}

// ... other helper functions
```

### 4. Writing Repository Tests

Structure tests using the **Arrange-Act-Assert** pattern. A single test function can validate multiple methods of the same repository for efficiency.

```rust
use crate::suites::test_helper::setup_test_context;
use crate::suites::person::helpers::create_test_person_model;
use banking_db::repository::PersonRepository;
use uuid::Uuid;

#[tokio::test]
async fn test_person_repository() {
    // 1. Arrange: Set up the transactional context and repository
    let ctx = setup_test_context().await.unwrap();
    let repo = ctx.person_repos().persons();

    // 2. Act & 3. Assert for the 'save' and 'find_by_id' methods
    let new_person = create_test_person_model("John Doe");
    let saved_person = repo.save(new_person.clone(), Uuid::new_v4()).await.unwrap();
    assert_eq!(new_person.id, saved_person.id);

    let found_person = repo.find_by_id(new_person.id).await.unwrap().unwrap();
    assert_eq!(new_person.id, found_person.person_id);

    // Act & Assert for the 'exists_by_id' method
    assert!(repo.exists_by_id(new_person.id).await.unwrap());
    assert!(!repo.exists_by_id(Uuid::new_v4()).await.unwrap());

    // ... continue testing other repository methods
}
```

### 5. Running Repository Tests

Repository tests can now be run in parallel, thanks to transaction-based isolation.

```bash
# Set the database URL
export DATABASE_URL="postgresql://user:password@localhost:5432/mydb"

# Run all integration tests in parallel
cargo test -p banking-db-postgres --test integration
```

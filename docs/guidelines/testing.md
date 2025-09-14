### Test Isolation
```rust
#[tokio::test]
async fn test_with_isolation() {
    // Each test establishes its own connection
    let pool = commons::establish_connection().await;
    
    // Module-specific setup is handled within the test file
    // This avoids monolithic test helpers and ensures clarity
    person_init::create_test_person(&pool).await;

    // Test logic follows...
    let new_person = PersonModel { /* ... */ };
    let person_repo = PersonRepositoryImpl::new(Arc::new(pool));
    let created_person = person_repo.save(new_person.clone()).await.unwrap();
    assert_eq!(new_person.id, created_person.id);
}
```

### Database Testing
**⚠️ Critical**: Database tests **must run sequentially** to avoid data pollution:

```bash
# Set the database URL (from project root)
export DATABASE_URL="postgresql://user:password@localhost:5432/mydb"

# Run all tests for a specific package (sequentially)
cargo test -p banking-db-postgres -- --test-threads=1

# Run a specific test with its required features
cargo test -p banking-db-postgres --test person_repository_tests --features person_repository -- --test-threads=1

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

**Crucially, all repository tests must run against a real database and be properly isolated to prevent data conflicts.**

-   **Connection**: Use the `commons::establish_connection().await` helper at the beginning of each test to get a database pool.
-   **Cleanup**: Call `commons::cleanup_database(&db_pool).await` immediately after establishing a connection in every test function. This truncates all relevant tables to ensure a clean state for each test.

```rust
#[tokio::test]
async fn test_my_repository() {
    let db_pool = commons::establish_connection().await;
    commons::cleanup_database(&db_pool).await;
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
#[tokio::test]
async fn test_my_repository() {
    // 1. Arrange: Set up DB, dependencies, and repository instance.
    let db_pool = commons::establish_connection().await;
    commons::cleanup_database(&db_pool).await;

    // Create any prerequisite data (e.g., a user for the 'created_by' field)
    let person_id = commons::create_test_person(&db_pool).await;

    // Instantiate the repository implementation
    let repo = MyRepositoryImpl::new(Arc::new(db_pool.clone()));

    // 2. Act & 3. Assert for the 'save' and 'find_by_id' methods
    let new_item = create_test_item_model(person_id);
    let saved_item = repo.save(new_item.clone()).await.unwrap();
    assert_eq!(new_item.id, saved_item.id);

    let found_item = repo.find_by_id(new_item.id).await.unwrap().unwrap();
    assert_eq!(new_item.id, found_item.id);

    // Act & Assert for the 'exists_by_id' method
    assert!(repo.exists_by_id(new_item.id).await.unwrap());
    assert!(!repo.exists_by_id(Uuid::new_v4()).await.unwrap());

    // ... continue testing other repository methods
}
```

### 5. Running Repository Tests

Repository tests **must** be run sequentially.

```bash
# Set the database URL
export DATABASE_URL="postgresql://user:password@localhost:5432/mydb"

# Run all integration tests sequentially
cargo test -p banking-db-postgres --test integration -- --test-threads=1
```

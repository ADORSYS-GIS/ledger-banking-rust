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

For a service located at `banking-logic/src/services/my_service_impl.rs`, the corresponding test file should be created at `banking-logic/tests/my_service_tests.rs`.

### 2. Mocking Dependencies

Service tests should be isolated from the database and other external layers. This is achieved by mocking the repository dependencies.

For each repository trait the service depends on, create a mock struct:

```rust
// In banking-logic/tests/my_service_tests.rs

use std::sync::{Arc, Mutex};

#[derive(Default)]
struct MockMyRepository {
    // Use a Mutex to hold an in-memory collection of models
    items: Mutex<Vec<MyModel>>,
}

#[async_trait]
impl MyRepository for MockMyRepository {
    async fn save(&self, item: MyModel) -> Result<MyModel, Box<dyn Error + Send + Sync>> {
        self.items.lock().unwrap().push(item.clone());
        Ok(item)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<MyModel>, Box<dyn Error + Send + Sync>> {
        Ok(self.items.lock().unwrap().iter().find(|i| i.id == id).cloned())
    }

    // ... implement other required trait methods
}
```

**Handling `Idx` and `Audit` Models in Mocks**

It is crucial that mock repositories accurately reflect the behavior of their real counterparts, especially concerning related data models like indexes (`Idx`) and audit trails (`Audit`).

When implementing a mock repository, always inspect the corresponding model definition in `banking-db/src/models/`. If the documentation comments indicate the presence of an `Idx` or `Audit` model, the mock must be extended to handle them.

**Example:**

If `MyModel` has `MyIdxModel` and `MyAuditModel`, the mock repository should be structured as follows:

```rust
#[derive(Default)]
struct MockMyRepository {
    items: Mutex<Vec<MyModel>>,
    item_ixes: Mutex<Vec<MyIdxModel>>,   // For the index model
    item_audits: Mutex<Vec<MyAuditModel>>, // For the audit model
}

#[async_trait]
impl MyRepository for MockMyRepository {
    async fn save(&self, item: MyModel, audit_log_id: Uuid) -> Result<MyModel, sqlx::Error> {
        // 1. Save the main model
        self.items.lock().unwrap().push(item.clone());

        // 2. Create and save the index model
        let item_idx = MyIdxModel {
            item_id: item.id,
            // ... populate other fields (use dummy values if necessary)
        };
        self.item_ixes.lock().unwrap().push(item_idx);

        // 3. Create and save the audit model
        let item_audit = MyAuditModel {
            item_id: item.id,
            // ... copy relevant fields from the main model
            audit_log_id,
        };
        self.item_audits.lock().unwrap().push(item_audit);

        Ok(item)
    }

    // ... other methods
}
```

This ensures that service logic relying on the existence of these related records can be tested accurately.

### 3. Test Setup Helpers

To keep tests clean and readable, create helper functions for setup:

**Service Instantiation:**
Create a function to instantiate the service with all its mocked dependencies.

```rust
fn create_test_service() -> MyServiceImpl {
    MyServiceImpl::new(
        Arc::new(MockMyRepository::default()),
        Arc::new(MockAnotherRepository::default()),
        // ... other mocked dependencies
    )
}
```

**Test Data Generation:**
Create helper functions for each domain model to easily generate test data. Avoid using `..Default::default()` if the domain models do not implement the `Default` trait. Instead, initialize all fields explicitly.

```rust
fn create_test_model() -> MyDomainModel {
    MyDomainModel {
        id: Uuid::new_v4(),
        name: HeaplessString::try_from("Test Name").unwrap(),
        is_active: true,
        // ... all other required fields
    }
}
```

### 4. Writing Unit Tests

Each public method in the service implementation must have a corresponding test function.

**Naming Convention:**
Use the `test_` prefix for all test functions (e.g., `test_create_item`).

**Test Structure (Arrange-Act-Assert):**

```rust
#[tokio::test]
async fn test_create_item() {
    // 1. Arrange: Set up the service and test data
    let service = create_test_service();
    let item = create_test_model();

    // 2. Act: Call the service method
    let created_item = service.create_item(item.clone()).await.unwrap();

    // 3. Assert: Verify the outcome
    assert_eq!(item.id, created_item.id);
}

#[tokio::test]
async fn test_find_item_by_id() {
    // 1. Arrange
    let service = create_test_service();
    let item = create_test_model();
    // Pre-load data into the mock repository if needed for "find" operations
    service.create_item(item.clone()).await.unwrap();

    // 2. Act
    let found_item = service.find_item_by_id(item.id).await.unwrap().unwrap();

    // 3. Assert
    assert_eq!(item.id, found_item.id);
}
```

### 5. Running Tests

Run the tests for the specific service from the project root:

```bash
cargo test -p banking-logic --test my_service_tests
```
This ensures that all methods are thoroughly tested in isolation, leading to a robust and reliable service layer.

## Repository Integration Testing

Repository tests are integration tests that validate the PostgreSQL implementations against a live database. They ensure that SQL queries, data mapping, and repository logic function correctly.

### 1. Test File Structure

For a repository defined in `banking-db-postgres/src/repository/my_repository_impl.rs`, the corresponding test file should be created at `banking-db-postgres/tests/suites/my_repository_tests.rs`.

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

Use helper functions to create consistent and valid test models.

-   **Location**: These helpers should be defined at the top of the test file.
-   **Naming**: `create_test_*_model()`.
-   **Foreign Keys**: For models with foreign key relationships (e.g., `created_by_person_id`), pass the required IDs as arguments to the helper function.
-   **Uniqueness**: For fields with `UNIQUE` constraints, generate a random value to avoid collisions (e.g., using `Uuid::new_v4()`).

```rust
fn create_test_item_model(created_by: Uuid) -> ItemModel {
    ItemModel {
        id: Uuid::new_v4(),
        name: HeaplessString::try_from("Test Item").unwrap(),
        // Use a UUID to ensure the unique value is always different
        unique_value: HeaplessString::try_from(Uuid::new_v4().to_string().as_str()).unwrap(),
        created_by_person_id: created_by,
        // ... other fields
    }
}
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

Repository tests **must** be run sequentially to ensure proper database cleanup and avoid race conditions.

```bash
# Set the database URL (from project root)
export DATABASE_URL="postgresql://user:password@localhost:5432/mydb"

# Run a specific repository test suite sequentially
cargo test -p banking-db-postgres --test my_repository_tests -- --test-threads=1
```

# Repository and Indexing Strategy

## Architectural Strategy: Distributed Data & Application-Layer Logic

To ensure high scalability and resilience, this system is designed to support a distributed data architecture. This means data can be partitioned across multiple databases and even stored in different types of storage systems (Polyglot Persistence). To achieve this, logic traditionally handled by a monolithic database is moved into the application layer.

### 1. Application-Managed Indexes for Finder Methods

Instead of relying on database indexes for finder methods (e.g., `find_by_status`), we will create and manage our own indexes in the model layer. This is the key to querying data that may be partitioned across multiple data stores.

**Core Principles:**

*   **Index Tables:** For each entity that requires complex finders (especially static or infrequently updated entities like `Person`), a corresponding index table (e.g., `person_idx`) will be created in the database. This table stores the indexed values and the `Uuid` of the corresponding entity.
*   **Transactional Consistency:** The index table MUST be updated within the same database transaction as the main entity table to ensure data is always consistent.
*   **In-Memory Loading:** The repository layer is responsible for loading the index table into an in-memory, `HashMap`-based cache on application startup or on-demand.
*   **Repository Implementation:** Repository finder methods should first consult the in-memory index to retrieve a `Vec<Uuid>` of matching entities. Pagination should be performed on this vector of IDs. Finally, the repository will fetch the full records for the paginated IDs using a `find_by_ids` method.

**Example: Indexing `Person` by `is_active` status**

**Database Schema (`person_idx` table):**
```sql
CREATE TABLE person_idx (
    person_id UUID PRIMARY KEY,
    is_active BOOLEAN NOT NULL
    -- other indexed fields...
);
```

**Repository Logic (Conceptual):**
```rust
// In PersonRepository implementation

// 1. Load the index into an in-memory cache
async fn load_index_cache(&self) -> Result<Arc<InMemoryPersonIndex>, Error> {
    // ... logic to load all records from `person_idx` table into a structured cache
}

// 2. Implement finder using the cache
async fn find_active_persons(&self, page: u32, page_size: u32) -> Result<Vec<Person>, Error> {
    let index_cache = self.get_index_cache().await?; // Get or load the cache

    // 3. Resolve IDs and paginate in-memory
    let all_active_ids = index_cache.get_ids_by_active_status(true);
    let page_size = page_size as usize;
    let start = ((page - 1) as usize) * page_size;
    
    if start >= all_active_ids.len() {
        return Ok(vec![]);
    }

    let end = std::cmp::min(start + page_size, all_active_ids.len());
    let ids_for_page = &all_active_ids[start..end];

    // 4. Fetch only the required records
    self.find_persons_by_ids(ids_for_page).await
}
```

### 2. Application-Layer Referential Integrity

To support data partitioning and polyglot persistence, **no foreign key constraints** should be defined in the database schema.

*   **Responsibility:** The application's repository and service layers are responsible for enforcing referential integrity.
*   **Implementation:** Before creating or updating an entity with a foreign key (e.g., an `Account` with a `customer_id`), the repository must first verify that the referenced entity (the `Customer`) exists by performing an `exists_by_id(customer_id)` check. The method for this check should be:
    *   **In-Memory Check (Preferred):** If an in-memory index is available for the referenced entity (e.g., `Person`), the `exists_by_id` check should be performed against the in-memory cache. This is the most efficient method and should be used whenever possible.
    *   **Database Check (Fallback):** If no in-memory index is available (e.g., for highly dynamic or non-indexed entities), the repository should fall back to a direct database lookup to verify existence.
*   **Database Schema:** Database schemas should define table structures and native data types only. All relationships and business rules are managed by the application.

#### Repository Dependencies for Constraint Validation

When a repository (`Repo A`) needs to validate a constraint against an entity managed by another repository (`Repo B`), `Repo B` must be injected into `Repo A` as a dependency. This is achieved through constructor injection.

*   **Dependency Injection:** The dependent repository (`Repo A`) should hold an `Arc<dyn TraitOfRepoB>` to the dependency. This dependency is passed in during instantiation.
*   **Centralized Instantiation:** All repositories are instantiated centrally (e.g., within the `UnitOfWork` or a dedicated factory like `PostgresRepositories`). This central location is responsible for creating repository instances and wiring their dependencies correctly.
*   **Validation Logic:** Inside the `save` method of `Repo A`, it will call `repo_b.exists_by_id(foreign_key_id)` before proceeding with the insert or update operation. If the check fails, it should return an appropriate error (e.g., `sqlx::Error::RowNotFound`).

**Example: `CountrySubdivisionRepository` depending on `CountryRepository`**

```rust
// In banking-db-postgres/src/repository/person_country_subdivision_repository_impl.rs

pub struct CountrySubdivisionRepositoryImpl {
    pool: Arc<PgPool>,
    country_subdivision_idx_cache: Arc<CountrySubdivisionIdxModelCache>,
    country_repository: Arc<dyn CountryRepository<Postgres>>, // Dependency
}

impl CountrySubdivisionRepositoryImpl {
    pub async fn new(
        pool: Arc<PgPool>,
        country_repository: Arc<dyn CountryRepository<Postgres>>, // Injected
    ) -> Self {
        // ... constructor logic
    }
}

#[async_trait]
impl CountrySubdivisionRepository<Postgres> for CountrySubdivisionRepositoryImpl {
    async fn save(
        &self,
        country_subdivision: CountrySubdivisionModel,
    ) -> Result<CountrySubdivisionModel, sqlx::Error> {
        // Constraint validation using the injected repository
        if !self
            .country_repository
            .exists_by_id(country_subdivision.country_id)
            .await
            .map_err(|e| sqlx::Error::Configuration(e.into()))?
        {
            return Err(sqlx::Error::RowNotFound);
        }

        // ... rest of the save logic
    }
}
```

### 3. Repository Trait Design (`banking-db/src/repository/`)

To ensure consistency and correctly implement the architectural patterns of application-managed indexes and referential integrity, all repository traits must follow these design guidelines.

#### Standard Repository Methods

Every repository trait for an entity (e.g., `ProductRepository` for `ProductModel`) must define a standard set of methods for basic operations and existence checks.

```rust
// Example: banking-db/src/repository/product_repository.rs
use uuid::Uuid;
use std::error::Error;
use crate::models::product::{ProductModel, DbProductType};

pub trait ProductRepository: Send + Sync {
    /// Saves a new or updated product model.
    async fn save(&self, product: ProductModel) -> Result<ProductModel, Box<dyn Error + Send + Sync>>;

    /// Finds a single product by its primary key.
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ProductModel>, Box<dyn Error + Send + Sync>>;

    /// Finds multiple products by a list of primary keys.
    /// This is crucial for fetching paginated data after an index lookup.
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<ProductModel>, Box<dyn Error + Send + Sync>>;

    /// Checks for the existence of a product by its primary key.
    /// This is the primary method for enforcing referential integrity.
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>>;
}
```

#### Batch Operations

For repositories that need to support bulk data processing, such as data migrations or large-scale updates, a separate `BatchRepository` trait should be implemented. This pattern is designed to optimize performance by reducing network round-trips and leveraging database-specific features for bulk operations.

For a detailed explanation of the batch operations pattern, its benefits, and implementation guidelines, please refer to the [Batch Operations Implementation Summary](./batch_operations.md).

#### Finder Methods and Index-Based Lookups

For every finder method that queries entities based on their attributes (e.g., finding products by type), two corresponding methods must be defined in the trait. This pattern separates the index lookup from the final data retrieval.

1.  **The ID-based Index Finder:** Returns a `Vec<Uuid>` of all matching entities. This method is responsible for querying the in-memory index or the `_idx` database table. Its name must be prefixed with `find_ids_by_`.
2.  **The Public Entity Finder:** Returns a paginated `Vec<FooModel>`. This method uses the ID-based finder internally to get all relevant IDs, performs pagination logic on the resulting vector, and then uses `find_by_ids` to fetch the data for the current page.

```rust
// Example: extending ProductRepository with a finder

pub trait ProductRepository: Send + Sync {
    // ... (standard methods from above)

    /// INDEX FINDER: Finds all product IDs for a given product type.
    /// This method queries the index and should be used internally by `find_by_type`.
    async fn find_ids_by_type(&self, product_type: DbProductType) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;

    /// PUBLIC FINDER: Finds a paginated list of products for a given product type.
    /// This method uses `find_ids_by_type` and `find_by_ids` internally.
    async fn find_by_type(
        &self,
        product_type: DbProductType,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<ProductModel>, Box<dyn Error + Send + Sync>>;
}
```

### 4. Advanced Indexing: In-Memory Cache Implementation Guide

Building upon the principle of application-managed indexes, this section provides a detailed guide for implementing a repository that uses an in-memory index cache for optimized query performance. This pattern is particularly useful for frequently accessed data where read performance is critical.

#### 4.1. Overview

The core idea is to maintain a lightweight, in-memory cache of an "index model" (`*_idx`) that contains the primary key, version, a hash of the full model, and any other frequently queried fields (or their hashes). This cache is used to quickly satisfy finder methods and to optimize the `save` operation by avoiding unnecessary database reads.

#### 4.2. Key Components

##### Data Models (`banking-db/src/models/`)

- **`Model`**: The main data model. It should **not** contain `version`, `hash`, or `audit_log_id` fields.
- **`ModelAudit`**: The audit model. It **must** contain `version`, `hash`, and `audit_log_id` fields.
- **`IdxModel`**: The index model. It **must** contain `version` and `hash` fields, along with the primary key and any other indexed fields.

##### Repository Trait (`banking-db/src/repository/`)

- **`load(id: Uuid) -> Result<Model, ...>`**: This is the *only* method that should return the full `Model`. It fetches the complete record from the database by its primary key.
- **`find_by_*` methods**: These methods should return `Option<IdxModel>` or `Vec<IdxModel>`. They should query the in-memory cache, not the database.
- **`exists_by_*` methods**: These should also query the cache and return a `bool`.
- **`save(model: Model) -> Result<Model, ...>`**: This method handles both creation and updates of records.

##### Repository Implementation (`banking-db-postgres/src/repository/`)

- **`struct RepositoryImpl`**: Must contain a `pool: Arc<PgPool>` and a `cache: Arc<RwLock<IdxModelCache>>`.
- **`new()`**: The constructor should pre-load all index records from the `*_idx` table into the cache upon instantiation.
- **`save()` Implementation**:
    - **Calculate Hashes**: Compute a hash of the incoming `Model` object (e.g., by serializing it to JSON and hashing the string). Also compute hashes for any other indexed fields.
    - **Check Cache First**: Before any database operation, check the cache to see if the record exists.
    - **Update Logic**:
        1.  If the record exists in the cache, it's an update.
        2.  Compare the hash of the incoming model with the hash stored in the cached `IdxModel`. If they are the same, no changes are needed, and you can return immediately.
        3.  If the hashes differ, increment the `version` from the cached `IdxModel`.
        4.  Create a `ModelAudit` record (with the new version, hash, and a new `audit_log_id`) and insert it into the `*_audit` table.
        5.  Perform a SQL `UPDATE` on the main table (note: the main table does not have a version or hash column).
        6.  Perform a SQL `UPDATE` on the `*_idx` table to persist the new version, hash, and any other indexed fields.
        7.  Create a new `IdxModel` with the updated information and update the cache.
    - **Insert Logic**:
        1.  If the record does not exist in the cache, it's a new record.
        2.  The `version` is `0`.
        3.  Create and insert a `ModelAudit` record (with version 0, the new hash, and a new `audit_log_id`).
        4.  Perform a SQL `INSERT` into the main table.
        5.  Create and insert a new `IdxModel` record into the `*_idx` table.
        6.  Add the new `IdxModel` to the in-memory cache.
- **Finder Implementations (`find_by_*`, `exists_by_*`)**: These methods should *only* interact with the `RwLock` on the cache to perform read operations. They should not touch the database.

##### Thread Safety

- **Scoping Locks**: `RwLockReadGuard` and `RwLockWriteGuard` must be dropped *before* any `.await` calls to avoid `Send` trait compilation errors. Use block scopes to manage the lifetime of the guards.

```rust
// Correctly scoped read lock
let maybe_existing_idx = {
    let cache_read_guard = self.person_idx_cache.read().unwrap();
    cache_read_guard.get_by_primary(&person.id)
}; // guard is dropped here

// .await call is safe now
if let Some(existing_idx) = maybe_existing_idx {
    // ...
}
```

#### 4.3. Example Flow: `save()` Method

1.  Calculate the hash of the incoming `Model`.
2.  Acquire a read lock on the cache to check if an `IdxModel` exists for the given ID. **Drop the lock immediately after the check.**
3.  **If `IdxModel` exists**:
    a. Compare the new hash with the hash in the cached `IdxModel`. If they match, return.
    b. Increment version from the cached `IdxModel`.
    c. `await` the insertion of a new `ModelAudit` record (with the new version, hash, and a new `audit_log_id`).
    d. `await` the `UPDATE` of the `Model` in the database.
    e. `await` the `UPDATE` of the `IdxModel` in the `*_idx` table.
    f. Acquire a write lock on the cache and update the `IdxModel` with the new version and hash.
4.  **If `IdxModel` does not exist**:
    a. `await` the insertion of a new `ModelAudit` record (with version 0, the new hash, and a new `audit_log_id`).
    b. `await` the `INSERT` of the new `Model` into the database.
    c. `await` the `INSERT` of the new `IdxModel` into the `*_idx` table.
    d. Acquire a write lock on the cache and add the new `IdxModel`.
5.  Return the saved `Model`.

#### 4.4. Other Indexing Patterns

##### The `<ModelName>IdxModel` Pattern (Comment-Driven)

For every main database model (e.g., `CountryModel`) that requires indexed lookups, a corresponding `<ModelName>IdxModel` struct is generated based on structured comments. This ensures that the index definition lives directly alongside the data model it supports.

**Generation Rules:**
-   **Trigger:** The generation is triggered by a `/// # Index: <ModelName>IdxModel` comment above the main model struct.
-   **Fields:** The fields of the `IdxModel` are derived from fields in the main model that are explicitly annotated for indexing with `/// # Index: ...`.
-   **Primary vs. Secondary:** The nature of the index (primary or secondary) is defined under a `/// ## Nature` sub-comment.
-   **Required Fields:** The `IdxModel` **must** include `version: i32` and `hash: i64` fields. The `audit_log_id` field should **not** be included, as it is exclusive to the audit model.

This pattern centralizes the definition of the model and its associated index, improving maintainability and ensuring consistency.

##### When to Create an Index

An index field should be added to the `<ModelName>IdxModel` and its corresponding `_idx` table whenever a repository trait defines a finder method that queries by that field.

-   **`find_by...` methods:** Methods that retrieve full, paginated models based on a field.
-   **`get_by...` methods:** Methods that retrieve one or more models based on a specific, often unique, field.

##### Synchronous Index Management in `save`

To guarantee data consistency, the index model (`<ModelName>IdxModel`) **must** be created or updated within the same database transaction as the main model (`<ModelName>Model`). This logic is centralized in the `save` method of the repository implementation. The `save` method is responsible for persisting both the main entity and its corresponding index entry.

```rust
// Example from PersonRepositoryImpl::save
// 1. Insert into the main 'person' table
sqlx::query("INSERT INTO person (...) VALUES (...)").await?;

// 2. Within the same method, insert into the 'person_idx' table
sqlx::query("INSERT INTO person_idx (...) VALUES (...)").await?;
```

##### Hashing String-Based Indexes

For performance and storage efficiency, long or sensitive string fields used for lookups (e.g., `external_identifier`) should be hashed before being stored in the index table.

**Principles:**
-   **Algorithm:** Use a fast, non-cryptographic hashing algorithm like `xxhash`. The hash should produce a `i64` or `i128` value that can be stored efficiently as a `BIGINT` or `NUMERIC` in the database.
-   **Collision Handling:** Hash collisions are possible. The `get_by...` or `find_by...` method must handle this by first retrieving all records matching the hash, and then performing a final in-memory filter on the full, unhashed string value to find the exact match.
-   **Implementation:** The hashing logic is implemented within the repository. The `save` method calculates the hash before saving to the index, and the `get_ids_by...` or `find_ids_by...` method calculate the same hash from the input string to perform the lookup.

**Example Flow for `get_by_external_identifier`:**
1.  `get_by_external_identifier(identifier: &str)` is called.
2.  It calls `get_ids_by_external_identifier(identifier)`, which hashes the `identifier` string.
3.  `get_ids_by_external_identifier` queries the `person_idx` table for all `person_id`s matching the hash. This is a fast integer-based lookup.
4.  It returns a `Vec<Uuid>` (which may contain IDs from hash collisions).
5.  `get_by_external_identifier` then calls `find_by_ids` with these IDs to fetch the full `PersonModel` objects.
6.  Finally, it filters the resulting `Vec<PersonModel>` to find the one where `person.external_identifier` exactly matches the original `identifier` string.

##### Immutable Caches for Static Data

For data that is static or changes very infrequently (e.g., countries, currencies), a simpler, immutable caching pattern is used.

**Principles:**

-   **`IdxModelCache`**: The cache is still represented by a `struct` (e.g., `CountryIdxModelCache`).
-   **`new()` Constructor**: The constructor takes a `Vec<IdxModel>` and builds `HashMap`s for each indexed field upon instantiation. It returns a `Result<Arc<Self>, ...>` to ensure the cache is immutable and thread-safe.
-   **No Mutable Methods**: The cache struct does not expose any methods that would mutate its internal state (e.g., no `add`, `update`, or `remove` methods).
-   **Repository Implementation**:
    -   The repository `struct` holds an `Arc<IdxModelCache>`.
    -   The `new()` function for the repository is `async`. It loads all records from the `*_idx` table and initializes the cache once.
    -   Finder methods (`find_by_*`, `exists_by_*`) perform read-only lookups against the immutable cache.
    -   The `save` method in the repository does *not* interact with the cache. It only performs the necessary database operations. This is acceptable because the underlying data is considered static.

##### Transaction-Aware Caching for Shared Caches

For shared caches that can be modified during a transaction (e.g., adding a new `Person` or `EntityReference`), the cache must be updated in a transaction-aware manner to maintain consistency. This applies to both statically initialized caches (like `Country`) and dynamically loaded caches (like `Person`).

This is achieved by wrapping the shared cache (e.g., `Arc<parking_lot::RwLock<EntityReferenceIdxModelCache>>`) in a transaction-specific, `Send`-compatible wrapper.

**Principles:**

-   **Wrapper Struct**: A dedicated struct (e.g., `TransactionAwareEntityReferenceIdxModelCache`) is created. It holds a reference to the shared cache (`Arc<parking_lot::RwLock<EntityReferenceIdxModelCache>>`) and transaction-local collections for additions, updates, and deletions (e.g., `parking_lot::RwLock<HashMap<...>>` and `parking_lot::RwLock<HashSet<...>>`).
-   **Repository Storage**: The repository (`EntityReferenceRepositoryImpl`) stores the wrapped, transaction-aware cache inside an `Arc<tokio::sync::RwLock<TransactionAwareEntityReferenceIdxModelCache>>`. The outer `tokio::sync::RwLock` is crucial because its read guard is `Send`, allowing it to be held across `.await` calls, which prevents compilation errors in `async` code.
-   **Delegation and Overlay**: The wrapper's finder methods implement an overlay pattern. They first check the transaction-local additions, updates, and deletions. If a result is found locally, it is returned. Otherwise, the request is delegated to the shared cache, filtering out any entries marked for deletion.
-   **`TransactionAware` Implementation**: The wrapper struct implements the `TransactionAware` trait.
    -   `on_commit`: Acquires a write lock on the shared cache and merges all local additions and processes deletions. The local collections are then cleared.
    -   `on_rollback`: Simply clears the local collections, discarding all changes.
-   **Repository `new()`**: The repository's constructor is responsible for creating the shared cache and wrapping it in the `TransactionAware...Cache`.

This pattern ensures that finder methods within a transaction see a consistent, up-to-date view of the data, and that the global cache is only updated if and when the transaction successfully commits.
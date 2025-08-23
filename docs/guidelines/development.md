# Implementation Patterns & Best Practices

## Domain and Database Models for Enum Types

Assuming the file containing the module uses the file name pattern <file_name>, eg.  for the module file_name=`account`, `banking-api/src/domain/{file_name}.rs` will be represented as `banking-api/src/domain/account.rs` 

The new database model enum must:
1.  Have the exact same variants as its domain counterpart.
2.  Be decorated with the necessary `sqlx` attributes for database mapping.
3.  Implement the required traits for database interaction.

Use the following transformation as a template:

**Source Domain Enum (`banking-api/src/domain/{file_name}.rs`):**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnumName {
    Internal,
    Partner,
    ThirdParty,
}
```

**Target Database Model Enum (`banking-db/src/models/{file_name}.rs`):**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "enum_name", rename_all = "PascalCase")]
pub enum EnumName {
    Internal,
    Partner,
    ThirdParty,
}
```

Additionally, implement the `FromStr` and `Display` trait for the database model enum to allow for conversion from and to a string. This is useful for serialization, deserialization and other contexts where the enum is represented as a string.

**`FromStr` Implementation (`banking-db/src/models/{file_name}.rs`):**
```rust
use std::str::FromStr;

impl FromStr for HoldPriority {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Critical" => Ok(HoldPriority::Critical),
            "High" => Ok(HoldPriority::High),
            "Standard" => Ok(HoldPriority::Standard),
            "Medium" => Ok(HoldPriority::Medium),
            "Low" => Ok(HoldPriority::Low),
            _ => Err(()),
        }
    }
}
```

**`Display` Implementation (`banking-db/src/models/{file_name}.rs`):**
```rust
impl std::fmt::Display for HoldPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HoldPriority::Critical => write!(f, "Critical"),
            HoldPriority::High => write!(f, "High"),
            HoldPriority::Standard => write!(f, "Standard"),
            HoldPriority::Medium => write!(f, "Medium"),
            HoldPriority::Low => write!(f, "Low"),
        }
    }
}
```

**Mapper Implementation (`banking-logic/src/mappers/{file_name}_mapper.rs`):**
```rust
impl AgentNetworkMapper {
    // ... other mapping functions

    // Mapper for Enums
    pub fn network_type_to_db(network_type: NetworkType) -> DbNetworkType {
        match network_type {
            NetworkType::Internal => DbNetworkType::Internal,
            NetworkType::Partner => DbNetworkType::Partner,
            NetworkType::ThirdParty => DbNetworkType::ThirdParty,
        }
    }

    pub fn network_type_from_db(db_type: DbNetworkType) -> NetworkType {
        match db_type {
            DbNetworkType::Internal => NetworkType::Internal,
            DbNetworkType::Partner => NetworkType::Partner,
            DbNetworkType::ThirdParty => NetworkType::ThirdParty,
        }
    }
}
```

**SQLx with PostgreSQL Enums**
```rust
// ✅ Use sqlx::query with manual binding for enums
sqlx::query(
    "INSERT INTO accounts (account_type, account_status) VALUES ($1::account_type, $2::account_status)"
)
.bind(account.account_type)
.bind(account.account_status)
.execute(&pool).await?
```

## Domain and Database Models for struct Types

**Mappers For Structs (`banking-logic/src/mappers/{file_name}_mapper.rs`):**

```rust
    /// Map from domain AgentNetwork to database AgentNetworkModel
    pub fn network_to_model(network: AgentNetwork) -> AgentNetworkModel {
        AgentNetworkModel {
            id: network.id,
            network_name: network.network_name,
            network_type: Self::network_type_to_db(network.network_type),
            status: Self::network_status_to_db(network.status),
            contract_external_id: network.contract_external_id,
            aggregate_daily_limit: network.aggregate_daily_limit,
            current_daily_volume: network.current_daily_volume,
            settlement_gl_code: network.settlement_gl_code,
            created_at: network.created_at,
            last_updated_at: network.created_at,
            updated_by_person_id: Uuid::nil(), // System UUID
        }
    }

    /// Map from database AgentNetworkModel to domain AgentNetwork
    pub fn network_from_model(model: AgentNetworkModel) -> AgentNetwork {
        AgentNetwork {
            id: model.id,
            network_name: model.network_name,
            network_type: Self::network_type_from_db(model.network_type),
            status: Self::network_status_from_db(model.status),
            contract_external_id: model.contract_external_id,
            aggregate_daily_limit: model.aggregate_daily_limit,
            current_daily_volume: model.current_daily_volume,
            settlement_gl_code: model.settlement_gl_code,
            created_at: model.created_at,
        }
    }
```

**Code Patterns**
```rust
// Builder pattern for domain models
let customer = Customer::builder(uuid, CustomerType::Corporate)
    .full_name("ACME Corporation Ltd")
    .risk_rating(RiskRating::Medium)
    .build()?;

// Memory-optimized types
pub id: Uuid,
pub currency: HeaplessString<3>,           // ISO 4217 codes
pub product_id: Uuid,                      // Foreign key
pub name_l1: HeaplessString<100>,             // Names/descriptions
pub account_status: AccountStatus,         // Type-safe enums vs String
pub description: Option<HeaplessString<200>>,
```

**Multi-language 'name' support**
- whenever we find a field named 'name' suggest change to 3 language fields.
pub name_l1: HeaplessString<100>,
pub name_l2: HeaplessString<100>,
pub name_l3: HeaplessString<100>,

**Type Mappings (Rust → PostgreSQL)**
- `Uuid` → `UUID`
- `HeaplessString<N>` → `VARCHAR(N)`
- `Decimal` → `DECIMAL(15,2)`
- `DateTime<Utc>` → `TIMESTAMP WITH TIME ZONE`

## Database Mapping Rules
- Foreign key constraints must be enforced in the repository layer, not the database. Use database schema for structure only, with no foreign key constraints.
- Date fields (e.g. `created_at`, `updated_at`) must be initialized in the domain layer. Database triggers for timestamp management are prohibited.

## Key Rules
1. **Use builders** for domain models (>4 parameters)
2. **HeaplessString<N>** for bounded text fields  
3. **Enums** for status/type fields instead of String
4. **References (&T)** for function parameters
5. **PostgreSQL Enum Casting**: Use `$N::enum_name`

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


### 4. Advanced Indexing Strategies and Patterns

Building upon the principle of application-managed indexes, several patterns ensure consistency, performance, and maintainability.

#### 4.1. The `<ModelName>IdxModel` Pattern (Comment-Driven)

For every main database model (e.g., `CountryModel`) that requires indexed lookups, a corresponding `<ModelName>IdxModel` struct is generated based on structured comments. This ensures that the index definition lives directly alongside the data model it supports.

**Generation Rules:**
-   **Trigger:** The generation is triggered by a `/// # Index: <ModelName>IdxModel` comment above the main model struct.
-   **Fields:** The fields of the `IdxModel` are derived from fields in the main model that are explicitly annotated for indexing with `/// # Index: ...`.
-   **Primary vs. Secondary:** The nature of the index (primary or secondary) is defined under a `/// ## Nature` sub-comment.

This pattern centralizes the definition of the model and its associated index, improving maintainability and ensuring consistency.

#### 4.2. When to Create an Index

An index field should be added to the `<ModelName>IdxModel` and its corresponding `_idx` table whenever a repository trait defines a finder method that queries by that field.

-   **`find_by...` methods:** Methods that retrieve full, paginated models based on a field.
-   **`get_by...` methods:** Methods that retrieve one or more models based on a specific, often unique, field.

#### 4.3. Synchronous Index Management in `save`

To guarantee data consistency, the index model (`<ModelName>IdxModel`) **must** be created or updated within the same database transaction as the main model (`<ModelName>Model`). This logic is centralized in the `save` method of the repository implementation. The `save` method is responsible for persisting both the main entity and its corresponding index entry.

```rust
// Example from PersonRepositoryImpl::save
// 1. Insert into the main 'person' table
sqlx::query("INSERT INTO person (...) VALUES (...)").await?;

// 2. Within the same method, insert into the 'person_idx' table
sqlx::query("INSERT INTO person_idx (...) VALUES (...)").await?;
```

#### 4.4. Hashing String-Based Indexes

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

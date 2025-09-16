# Prompt: Generate Application-Managed Indexes for a Module

Apply the rules defined in 'docs/guidelines/development.md' to generate the necessary code and database schema for application-managed indexes for a given module.

**Rule Precedence:** All generation logic is driven by structured comments within the source `...Model` struct. These in-code instructions always take precedence over general guidelines.

**Inputs:**
-   `<file_name>`: The name of the module (e.g., `person`).

**Infered Parameter**
-   `<ModelName>`: stands for each struct `<ModelName>Model` defined in the module (e.g. PersonModel, LocationModel). 
-   `<attribute>`: an attribute of this model used in a finder.
---


### Step 1: Generate Index Model from Comment Instructions

**File:** `banking-db/src/models/person/<file_name>.rs`

For each main model (e.g., `CountryModel`), generate the corresponding `<ModelName>IdxModel` struct based on structured comments within the main model's definition. The generation is triggered by a top-level `/// # Index: <ModelName>IdxModel` comment.

**Parsing Requirements:**
1.  **Trigger:** Identify the main model struct that has a `/// # Index: <ModelName>IdxModel` comment block.
2.  **Idempotency Check:** Before generating any code, scan the target file for the existence of `pub struct <ModelName>IdxModel`. If this struct already exists, the script must halt execution for this model to ensure idempotency.
3.  **Fields:** Parse the doc comments for each field within that main model.
4.  **Primary Index:** A field is the primary key for the index if its comment contains `/// # Index: <field_name>` and `/// ## Nature` followed by `- primary`. The `id` field of the main model is typically renamed to `<model_name>_id` in the index model.
5.  **Secondary Index:** A field is a secondary index if its comment contains `/// # Index` and `/// ## Nature` followed by `- secondary`.
    *   If the original field is a string type (e.g., `HeaplessString<N>`), the index field should be the specified hash (e.g., `code_hash: i64`).
    *   Otherwise, use the field name and type as is.
6.  **Decoration:** The generated struct must be decorated with `#[derive(Debug, Clone, FromRow)]`.

**Example (Derived from `CountryModel` in `person.rs`):**

**Source Comments in `CountryModel`:**
```rust
/// # Index: CountryIdxModel
// ...
pub struct CountryModel {
    /// # Index: country_id
    /// ## Nature
    /// - primary
    pub id: Uuid,
    
    /// # Index: iso2: HeaplessString<2>
    /// ## Nature
    /// - secondary
    /// - unique
    pub iso2: HeaplessString<2>,
    // ... other fields
}
```

**Generated `CountryIdxModel`:**
```rust
#[derive(Debug, Clone, FromRow)]
pub struct CountryIdxModel {
    /// # Nature
    /// - primary
    pub country_id: Uuid,

    /// # Nature
    /// - secondary
    /// - unique
    pub iso2: HeaplessString<2>,
}
```

---

### Step 2: Generate Repository Methods

**File:** `banking-db/src/repository/person/<file_name>_repository.rs`

Update the repository trait for the module to include methods for referential integrity checks and index-based finders, as per the "Application-Layer Referential Integrity" and "Repository Trait Design" sections of the guidelines.

**Requirements:**
1. Use comment instructions to specify that index is managed by the same repositoy and the corresponding model object. e.g.
banking-db/src/models/person.rs:1023-1040
```
/// # Repository Trait
/// - FQN: banking-db/src/repository/person/person_repository.rs/PersonModelRepository
/// # Cache
/// ...
/// # Trait method
/// - get_all
/// ...
#[derive(Debug, Clone, FromRow)]
pub struct PersonIdxModel {
    pub person_id: Uuid,
    /// # Nature
    /// - Mutable
    pub external_identifier_hash: Option<i64>,
}
```

2. If the index cache contains the `Pg Trigger`
```
/// ...
/// # Pg Trigger
/// - CREATE
/// - UPDATE
...
pub struct EntityReferenceIdxModel {
...
```

add postgress PostgreSQL users: LISTEN/NOTIFY functionality, for the listed database operations.

---

### Step 3: Generate Database Migration Script

**File:** `banking-db-postgres/migrations/<init_order>_initial_schema_person.sql`

Create a new SQL migration file to define the schema for the main table and its corresponding index table.

**Requirements:**
1.  **No Foreign Keys:** Foreign key constraints are managed at the application layer.
2.  **Main Table:** Create the main table for each model (e.g., `person`). Use the type mappings specified in the guidelines (e.g., `Uuid` -> `UUID`, `HeaplessString<N>` -> `VARCHAR(N)`).
3.  **Index Table:** Create the index table named `<model_name>_idx` (e.g., `person_idx`).
    *   The primary key should be `<model_name>_id: Uuid`.
    *   Include columns for all indexed fields.
4.  **Database Indexes:** Do not create database `INDEX`, except on the primary key

**Example (`person` module):**
```sql
-- banking-db-postgres/migrations/001_initial_schema_person.sql

-- Main table for model PersonModel
CREATE TABLE person (
    id UUID PRIMARY KEY,
    -- ... other fields for PersonModel
    is_active BOOLEAN NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Index table for Person
CREATE TABLE person_idx (
    person_id UUID PRIMARY KEY,
    is_active BOOLEAN NOT NULL
    -- ... other indexed fields
);


---

### Step 4: Hashing String-Based Indexes

For string-based index fields, apply the hashing strategy as defined in the "Hashing String-Based Indexes" section of `docs/guidelines/development.md`. This involves using a fast hashing algorithm (e.g., `xxhash`), storing the hash in the index table, and handling potential collisions in the repository.

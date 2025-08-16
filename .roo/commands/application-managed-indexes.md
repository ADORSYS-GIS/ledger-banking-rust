# Prompt: Generate Application-Managed Indexes for a Module

Apply the rules defined in 'docs/guidelines/development.md' to generate the necessary code and database schema for application-managed indexes for a given module.

**Inputs:**
-   `<file_name>`: The name of the module (e.g., `person`).
-   `<init_order>`: The numerical prefix for the database migration file (e.g., `001`).

**Infered Parameter**
-   `<ModelName>`: stands for each struct `<ModelName>Model` defined in the module (e.g. PersonModel, AddressModel). 
-   `<attribute>`: an attribute of this model used in a finder.
---


### Step 1: Generate Index Model Object

**File:** `banking-db/src/models/<file_name>.rs`

For each model (e.g., `PersonModel`), create a corresponding index model struct named `<ModelName>IdxModel` within the same module file. This model is used exclusively for indexing and does not have a counterpart in the domain layer.

**Requirements:**
1.  The struct must be named by appending `Idx` to the ModelName (e.g., `PersonModel` -> `PersonIdxModel`).
2.  It must include the primary key of the parent model, named as `<model_name>_id` (e.g., `person_id`).
3.  It must include all fields from the parent model that will be used for finder methods (e.g., `is_active`, `status`, `type`).
4.  Decorate the struct with `#[derive(Debug, Clone, sqlx::FromRow)]`.

**Example (`PersonIdxModel`):**
```rust
// In banking-db/src/models/person.rs

// ... existing PersonModel struct ...

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PersonIdxModel {
    pub person_id: Uuid,
    pub is_active: bool,
    // ... other indexed fields
}
```

---

### Step 2: Generate Repository Methods

**File:** `banking-db/src/repository/<file_name>_repository.rs`

Update the repository trait for the module to include methods for referential integrity checks and index-based finders, as per the "Application-Layer Referential Integrity" and "Repository Trait Design" sections of the guidelines.

**Requirements:**



---

### Step 3: Generate Database Migration Script

**File:** `banking-db-postgres/migrations/<init_order>_initial_schema_<file_name>.sql`

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

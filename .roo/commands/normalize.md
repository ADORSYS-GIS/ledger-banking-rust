---
description: This template provides a structured approach for aligning domain models with database models, refactoring enums, and normalizing foreign key references in the ledger-banking-rust project.
argument-hint: <file_name to process> <tasks>?
---

# Code Normalization Template

This command combines three normalization tasks:
1.  **Foreign Key Normalization**: Standardizes foreign key field names to include the target entity for clarity and consistency.
2.  **Model-to-Domain Alignment**: Aligns the API domain models in `banking-api` to match the database models in `banking-db`, which serve as the source of truth.
3.  **Enum Refactoring**: Decouples domain enums from database enums, creating separate, specialized versions for each layer.

---

## Usage

Replace `{file_name}` with the actual module name (e.g., customer, account, transaction, etc.)

Replace `<tasks>` with the target tasks. e.g.:
    - `/normalize person` or `/normalize person 1..3` will execute all tasks
    - `/normalize person 2` will perform only task 2
    - `/normalize person 1,3` will perform only tasks 1 and 3

## Prompt Template

```
Using the database models in @banking-db/src/models/{file_name}.rs as the source of truth, align their counterpart domain models in @banking-api/src/domain/{file_name}.rs. Provided or modify service traits in @/banking-api/src/service/{file_name}_service.rs to use domain structs and enums. Provide or modify mappers for structs and enums in @/banking-logic/src/mappers/{file_name}_mapper.rs to fit with domain and model objects. Also modify @banking-db-postgres/migrations/*.sql to fit the new data definition. Modify repository definition in @/banking-db/src/repository/{file_name}_repository.rs and the corresponding implementation @/banking-db-postgres/src/repository/{file_name}_repository_impl.rs. Modify the service implementation in @/banking-logic/src/services/{file_name}_service_impl.rs. 
```

## Rule Precedence
**The single source of truth for repository traits, indexes, and caches is the structured comments within the `...Model` structs.** All normalization and generation tasks must be driven by these in-code instructions, which always override any other guidelines.

## Steps Performed : Analysis

1.  **Read Database Model (`banking-db/src/models/{file_name}.rs`) as Source of Truth**
    -   Identify all `...Model` structs and their fields.
    -   Document all database enums and their variants.
    -   Note field types, especially HeaplessString sizes and enum types.
    -   Identify model-only structs (e.g., `...IdxModel`, `...IdxModelCache`) that should not have a domain counterpart.

2.  **Read API Domain Model (`banking-api/src/domain/{file_name}.rs`) for Comparison**
    -   Compare with the database model to identify discrepancies.
    -   Note any missing structs or enums that need to be created.
    -   Identify fields with mismatched types that need alignment.

3.  **Read Database Schema** (`banking-db-postgres/migrations/*.sql`)
    -   Look for database tables
    -   Look for column definitions that match new enum types
    -   Look for constraints that reflect enum values
    -   Update CHECK constraints for enum validation

4.  **Produce Mapping Tables** (`target/modules/{file_name}.md`)
    -   create or update the mapping file `target/modules/{file_name}.md`
    -   in the first table, list the target state of mapping between domain structs, database model, database tables. also display model object that are not existent as domain struct. Indicate if any flattening will occur and show list domain object flattened.

| Domain Struct (`banking-api/src/domain/{file_name}.rs`) | Database Model (`banking-db/src/models/{file_name}.rs`) | database tables | Flattened / Details | Changed |
|---|---|---|---|---|---|
| `Account` | `AccountModel` | `accounts` | none | Direct mapping. As Is |


    -   in the second table for each struct `{struct_name}` in a file named (`target/modules/{file_name}/{StructName}.md`), list field in domain, database model and database init script.

| `{struct_name}` | Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---| ---|
| Account | `id` | `Uuid` | `id` | `Uuid` | `id` | `UUID` | Unique identifier for the account | none |

5. **Foreign Key Reference Analysis**

    1.1 Analyze the domain structs file: `banking-api/src/domain/{file_name}.rs`. Do not process other files if not explicitely requested.
    1.2. Identify all fields ending with `_id: Uuid` or `_id: Option<Uuid>`
    1.3. Identify all fields ending with `_by: Uuid` or `_by: Option<Uuid>`
    1.4. For each field, determine:
        - Current field name
        - Target struct name and location
        - Whether renaming is needed based on rules above
        - Proposed new field name (if applicable)

6. **Proposal Generation**
Create/update proposal in `target/modules/{file_name}/{StructName}.md` with:

## Fields Requiring Changes
| Current Field | Source Struct |Target Struct | Target Location | Proposed Field | Reason |
|---------------|---------------|--------------|-----------------|----------------|---------|
| field_name | SourceStruct | TargetStruct | path/to/struct.rs | new_field_name | Rule explanation |

7.  **Prompt User to Approve files**
    -   prompt the user to review, modify and approve 
        - `target/modules/{file_name}.md`
        - `target/modules/{file_name}/{StructName}.md`

## Steps Performed : Changes
**Idempotency:** All update operations must be idempotent. Before applying any change, the script must verify that the target code is not already in the desired state. For example, do not attempt to rename a field that has already been renamed, or add a struct that already exists.

Upon approval, perform changes as indicated in the files `target/modules/{file_name}.md` and `target/modules/{file_name}/{StructName}.md`.

8.  **Update Domain Model to Align with Database Model**
    -   Using the database model as the single source of truth, align all corresponding structs and enums in `@banking-api/src/domain/{file_name}.rs`.
    -   For each `...Model` struct in `banking-db`, ensure a corresponding domain struct exists in `banking-api`, unless it is a model-only struct (e.g., `...IdxModel`, `...IdxModelCache`).
    -   Add any missing structs or enums to the domain layer to match the database model layer.
    -   Update fields and enum variants in the domain layer to match the types and definitions in the database model layer.

9.  **Update Service Traits**
    - Provided or modify service traits in @/banking-api/src/service/{file_name}_service.rs to use domain structs and enums.

10.  **Update Database Schema** (`banking-db-postgres/migrations/*.sql`)
    -   Also modify @banking-db-postgres/migrations/.sql to fit the new data definition. 
    -   Update column definitions to match new enum types and struct fields
    -   Ensure constraints reflect enum values
    -   Update CHECK constraints for enum validation

11.  **Update Mappers** (`banking-logic/src/mappers/{file_name}_mapper.rs`)
    -   Provide or modify mappers for enums in @/banking-logic/src/mappers/{file_name}_mapper.rs to fit with domain and model objects.
    -   Provide or modify mappers for structs in @/banking-logic/src/mappers/{file_name}_mapper.rs to fit with domain and model objects.
    -   Ensure bidirectional mapping works correctly

12.  **Update Service Traits** (if needed)
    -   Modify the service traits in banking-api/src/service/{file_name}_service.rs to have method parameters alling with struct field types. Process instruction on service traits and trait method in the code comment, as they have precedence over general conventions. e.g. comments in the following code block indicate the target service traits and trait methods

banking-api/src/domain/person.rs
```
/// # Service Trait
/// - FQN: banking-api/src/service/person_service.rs/PersonService
/// ...
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    /// # Trait method
    /// - find_country_by_id
    /// # Nature
    /// - primary index
    pub id: Uuid,
    /// # Documentation
    /// - ISO 3166-1 alpha-2 country code (e.g., "CM", "US", "GB")
    /// # Trait method
    /// - find_country_by_iso2
    /// # Nature
    /// - unique
    pub iso2: HeaplessString<2>,
    ...
}
```

13. **Update Repository Traits (Comment-Driven)**
    -   **Source of Truth:** The definition of repository traits is now driven by structured comments within the database model files (`banking-db/src/models/{file_name}.rs`).
    -   **Parsing Logic:**
        -   A `/// # Repository Trait` block on a model struct specifies the target repository trait.
        -   `/// # Trait method` comments on individual fields define the methods to be generated in that trait (e.g., `find_by_id`, `find_ids_by_iso2`).
        -   The method signature is derived from the field's type. For `find_ids_by_...` methods, the return type should be `Result<Vec<Uuid>, ...>`.
    -   **Action:** Modify the repository traits in `banking-db/src/repository/{file_name}_repository.rs` to exactly match the methods, names, and types defined in the model's comments. This ensures the trait is always synchronized with its model's declared intentions.

    **Example from `banking-db/src/models/person.rs`:**
    ```rust
    /// # Repository Trait
    /// - FQN: banking-db/src/repository/person_repository.rs/CountryRepository
    #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
    pub struct CountryModel {
        /// # Trait method
        /// - find_by_id
        /// - exists_by_id
        pub id: Uuid,
        
        /// # Trait method
        /// - find_ids_by_iso2
        /// - find_by_iso2
        pub iso2: HeaplessString<2>,
    }
    ```
    This configuration will generate a `CountryRepository` trait with methods like `find_by_id(&self, id: Uuid)` and `find_ids_by_iso2(&self, iso2: &HeaplessString<2>)`.

14. **Update Repository Implementation**
    -   Modify repository implementation in banking-db-postgres/src/repository/{file_name}_repository_impl.rs to allign with the repository trait

13.  **Update Service Implementations** (if needed)
    -   Modify the service implementation in banking-logic/src/services/{file_name}_service_impl.rs to allign with the service trait

# Key Patterns

# Foreign Key Reference Alignment

## Naming Convention Rules

### 1. Explicit Entity Reference Required
When the field name doesn't clearly indicate the target rust file, include the target struct name:
- `domicile_branch_id: Uuid` → `domicile_agency_branch_id: Uuid` (target: `AgencyBranch`)
- `updated_by_person_id: Uuid` → `updated_by_person_id: Uuid` (target: `Person`)
- `controlled_by`  → `controlled_by_person_id: Uuid` (target: `Person`)
The method comment might indicate the tartget struct:
-     /// References Person.person_id
-     `agent_user_id`   → `agent_person_id: Uuid` (target: `Person`)

### 2. Avoid Unnecessary Type Convertion
While defining the service and repository traits, keep the struct field natie types as far as possible. e.g. type conversion to string shall happen at the lattest possible location in the code.
e.g.:
**Bad Case**
banking-db/src/repository/person_repository.rs
```
    /// Get persons by external identifier
    async fn get_by_external_identifier(
        &self,
        identifier: &str,
    ) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>>;

```
**Good Case**
banking-db/src/repository/person_repository.rs
```
    /// Get persons by external identifier
    async fn get_by_external_identifier(
        &self,
        identifier: HeaplessString<50>,
    ) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>>;
```

### 3. Keep Existing When Clear
When the field name already matches or clearly indicates the target entity:
- `collateral_id: Option<Uuid>` → **KEEP AS IS** (target: `Collateral`)
- `customer_id: Uuid` → **KEEP AS IS** (target: `Customer`)

### 4. Compound Names Exception
For fields with compound qualifiers that don't have individual target structs:
- `loan_purpose_id: Option<Uuid>` → **KEEP AS IS** (target: `ReasonAndPurpose`, no `Purpose` struct exists)
- `transaction_reason_id: Option<Uuid>` → **KEEP AS IS** (target: `ReasonAndPurpose`, no `Reason` struct exists)

### 5. Person Reference Pattern
All `*_by` fields reference `Person` and should include explicit `_person_id` suffix:
- `created_by_person_id: Uuid` → **KEEP AS IS** (already follows correct pattern)
- `updated_by_person_id: Uuid` → **KEEP AS IS** (already follows correct pattern)
- `approved_by: Option<Uuid>` → `approved_by_person_id: Option<Uuid>`

### 6. Preserve Existing Correct Patterns
Fields that already follow the explicit entity reference pattern should not be modified:
- If field already includes target entity name (e.g., `created_by_person_id`, `updated_by_person_id`), keep unchanged
- Only modify fields that lack explicit entity reference (e.g., `approved_by` → `approved_by_person_id`, `imported_by` → `imported_by_person_id`)

### 7. String Alignment

- **String to Enum Conversion**
```rust
// BEFORE: String type allowing invalid values
pub status: String,

// AFTER: Proper enum with validation
pub status: StatusEnum,
```

- **Bounded String**
Always keep HeaplessString, do not change them to String.
```rust
// Ensure consistent HeaplessString sizes between API and DB models
pub field_name: HeaplessString<N>, // Same N in both layers
```
Always suggest change of String type to HeaplessString.

### 8. Model Only Struct

Following structs will not have counterpart in the domain:
- **<StructName>IdxModel**
Index model are used to cache indexes in memory. In orther to check if a person with id uuid exists, the repository can simply querry the index repository. there is no need to issue a querry to the database.

Indes models have a corresponding database table representation as describes in .roo/commands/application-managed-indexes.md.

- **<StructName>IdxModelCache**
Cache for index models do not have a domain representation and do not have a database repesentation.

## Serialization

### Database Models Enum Serialization Pattern
```rust
// Custom serialization for database compatibility for {enum_name}
#[serde(serialize_with = "serialize_{enum_name}", deserialize_with = "deserialize_{enum_name}")]
pub field_name: EnumType,

fn serialize_enum_name<S>(value: &EnumType, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        EnumType::Variant1 => "Variant1",
        EnumType::Variant2 => "Variant2",
    };
    serializer.serialize_str(value_str)
}
```
### Database Models Enum Deserialization Pattern
Also provide deserializer.

## Tests

### Test Updates
- Update all test files that reference the changed fields:
  - `banking-logic/src/services/tests/{file_name}_tests.rs`
  - `banking-db-postgres/tests/{file_name}_repository_tests*.rs`
  - Any integration tests
- Update test data creation
- Update assertions and validations

## Benefits

1. **Type Safety**: Enum types prevent invalid values at compile time
2. **Memory Efficiency**: Reduced memory usage compared to String fields
3. **Data Integrity**: Database constraints ensure valid enum values
4. **Maintainability**: Clear separation between domain and database concerns
5. **Performance**: Faster enum comparisons vs string comparisons

## Important Notes

1. **Preserve Relationships**: Ensure all foreign key relationships remain intact after renaming
2. **Database Consistency**: Column renames must be reflected in both schema and queries
3. **Test Coverage**: All tests must pass after changes to ensure functionality is preserved
4. **Incremental Approach**: Process one entity at a time to minimize complexity
5. **Backup Strategy**: Consider creating a branch before starting major refactoring
6. **Documentation**: Update any documentation that references the old field names


## Validation Checklist

-   [ ] Check that all foreign key relationships are preserved
-   [ ] All enums from API domain are present in database model
-   [ ] Custom serialization functions work correctly
-   [ ] Database schema constraints match enum variants, Verify database schema is consistent
-   [ ] HeaplessString sizes match between API and DB models
-   [ ] Field types are consistent between layers
-   [ ] Mappers handle all enum conversions 
-   [ ] Service implementations compile without errors (Run `cargo check` to ensure compilation)
-   [ ] Run `cargo clippy` to check for warnings
-   [ ] Run tests: `cargo test --features postgres_tests -- --test-threads=1
---

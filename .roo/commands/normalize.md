---
description: This template provides a structured approach for aligning domain models between the API layer and database layer, refactoring enums, and normalizing foreign key references in the ledger-banking-rust project.
argument-hint: <file_name to process>
---

# Code Normalization Template

This command combines three normalization tasks:
1.  **Foreign Key Normalization**: Standardizes foreign key field names to include the target entity for clarity and consistency.
2.  **API-Database Alignment**: Aligns domain models, structs, and enums between the `banking-api` and `banking-db` crates.
3.  **Enum Refactoring**: Decouples domain enums from database enums, creating separate, specialized versions for each layer.

---

## Usage

Replace `{file_name}` with the actual module name (e.g., customer, account, transaction, etc.)

## Prompt Template

```
Align all structs and enum in @banking-db/src/models/{file_name}.rs with their counterpart in @banking-api/src/domain/{file_name}.rs and vice versa. Provided or modify service traits in @/banking-api/src/service/{file_name}_service.rs to use domain structs and enums. Provide or modify mappers for structs and enums in @/banking-logic/src/mappers/{file_name}_mapper.rs to fit with domain and model objects. Also modify @banking-db-postgres/migrations/*.sql to fit the new data definition. Modify repository definition in @/banking-db/src/repository/{file_name}_repository.rs and the corresponding implementation @/banking-db-postgres/src/repository/{file_name}_repository_impl.rs. Modify the service implementation in @/banking-logic/src/services/{file_name}_service_impl.rs. 
```

## Steps Performed : Analysis

1.  **Read API Domain Model** (`banking-api/src/domain/{file_name}.rs`)
    -   Identify all structs and their fields
    -   Document all enums and their variants
    -   Note field types, especially HeaplessString sizes and enum types

2.  **Read Database Model** (`banking-db/src/models/{file_name}.rs`)
    -   Compare with API domain model
    -   Identify mismatches in field types
    -   Note missing enums

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
up on approval, perfrom changes as indicated in files `target/modules/{file_name}.md`, `target/modules/{file_name}/{StructName}.md`

8.  **Update Database Model or Domain Model**
    -   Align all structs and enum in @banking-db/src/models/{file_name}.rs with their counterpart in @banking-api/src/domain/{file_name}.rs and vice versa.    -   Add structs and enums present in `banking-api/src/domain/{file_name}.rs` but missing in `banking-db/src/models/{file_name}.rs` 
    -   Add structs and enums present in `banking-db/src/models/{file_name}.rs` but missing in `banking-api/src/domain/{file_name}.rs` 
    -   Convert String fields to appropriate enum types
    -   Add custom serialization functions where needed

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

12. **Update Repository Definition**
    -   Modify repository definition in @/banking-db/src/repository/{file_name}_repository.rs and the corresponding implementation @/banking-db-postgres/src/repository/{file_name}_repository_impl.rs.

13.  **Update Service Implementations** (if needed)
    -   Modify the service implementation in @/banking-logic/src/services/{file_name}_service_impl.rs.
    -   Update any service implementations that use the models
    -   Ensure type compatibility across layers

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


### 2. Keep Existing When Clear
When the field name already matches or clearly indicates the target entity:
- `collateral_id: Option<Uuid>` → **KEEP AS IS** (target: `Collateral`)
- `customer_id: Uuid` → **KEEP AS IS** (target: `Customer`)

### 3. Compound Names Exception
For fields with compound qualifiers that don't have individual target structs:
- `loan_purpose_id: Option<Uuid>` → **KEEP AS IS** (target: `ReasonAndPurpose`, no `Purpose` struct exists)
- `transaction_reason_id: Option<Uuid>` → **KEEP AS IS** (target: `ReasonAndPurpose`, no `Reason` struct exists)

### 4. Person Reference Pattern
All `*_by` fields reference `Person` and should include explicit `_person_id` suffix:
- `created_by_person_id: Uuid` → **KEEP AS IS** (already follows correct pattern)
- `updated_by_person_id: Uuid` → **KEEP AS IS** (already follows correct pattern)
- `approved_by: Option<Uuid>` → `approved_by_person_id: Option<Uuid>`

### 5. Preserve Existing Correct Patterns
Fields that already follow the explicit entity reference pattern should not be modified:
- If field already includes target entity name (e.g., `created_by_person_id`, `updated_by_person_id`), keep unchanged
- Only modify fields that lack explicit entity reference (e.g., `approved_by` → `approved_by_person_id`, `imported_by` → `imported_by_person_id`)


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

## String to Enum Conversion
```rust
// BEFORE: String type allowing invalid values
pub status: String,

// AFTER: Proper enum with validation
pub status: StatusEnum,
```

## String Alignment
Always keep HeaplessString, do not change them to String.
```rust
// Ensure consistent HeaplessString sizes between API and DB models
pub field_name: HeaplessString<N>, // Same N in both layers
```
Always translate Strings to Heapless Strings.



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




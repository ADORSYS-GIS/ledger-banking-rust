---
description: This template provides a structured approach for aligning domain models between the API layer and database layer in the ledger-banking-rust project.
argument-hint: <file_name to process>
---

# API-Database Alignment Template

This template provides a structured approach for aligning domain models between the API layer and database layer in the ledger-banking-rust project.

## Usage

Replace `{file_name}` with the actual module name (e.g., customer, account, transaction, etc.)

## Prompt Template

```
Align all structs in @banking-db/src/models/{file_name}.rs with their counterpart in @banking-api/src/domain/{file_name}.rs. Make sure all enums defined in the @banking-api/src/domain/{file_name}.rs are also defined in the @banking-db/src/models/{file_name}.rs. Also, identify any model structs that do not have a counterpart in the domain. Modify the mappers and implementation in @banking-logic/ accordingly. Also modify @banking-db-postgres/migrations/001_initial_schema.sql to fit the new data definition.
```

## Steps Performed : Analysis

1. **Read API Domain Model** (`banking-api/src/domain/{file_name}.rs`)
   - Identify all structs and their fields
   - Document all enums and their variants
   - Note field types, especially HeaplessString sizes and enum types

2. **Read Database Model** (`banking-db/src/models/{file_name}.rs`)
   - Compare with API domain model
   - Identify mismatches in field types
   - Note missing enums

3. **Read Database Schema** (`banking-db-postgres/migrations/001_initial_schema.sql`)
   - Look for database tables
   - Look for column definitions that match new enum types
   - Look for constraints that reflect enum values
   - Update CHECK constraints for enum validation

4. **Produce Mapping Tables** (`docs/modules/{file_name}.md`)
   - create or update the mapping file `docs/modules/{file_name}.md` 
   - in the first table, list the target state of mapping between domain structs, database model, database tables. also display model object that are not existent as domain struct. Indicate if any flttening will occur and show list domain object flattened. 
| Domain Struct (`banking-api/src/domain/{file_name}.rs`) | Database Model (`banking-db/src/models/{file_name}.rs`) | database tables | Flattened / Details  | Changed | 
|---|---|---|---|---|---|
| `Account` | `AccountModel` | `accounts` | none | Direct mapping. As Is |


   - in the second table for each struct `{struct_name}` in a file named (`docs/modules/{file_name}/{struct_name}.md`), list field in domain, database model and database init script.

   | `{struct_name}` | Domain Field | Domain Type | DB Field | DB Type |  DB column | column type | Description | Change to Perform |
   |---|---|---|---|---|---|---|---| ---|
   | Account | `id` | `Uuid` | `id` | `Uuid` | `id` | `UUID` | Unique identifier for the account | none |

5. **Prompt User to Approve file**
   - prompt the user to review, modify and approve `docs/modules/{file_name}.md`

## Steps Performed : Changes
up on approval, perfrom changes as indicated in `docs/modules/{file_name}.md`

6. **Update Database Model**
   - Add missing enums with proper serialization
   - Convert String fields to appropriate enum types
   - Update field types to match domain model
   - Add custom serialization functions where needed

7. **Update Database Schema** (`banking-db-postgres/migrations/001_initial_schema.sql`)
   - Update column definitions to match new enum types
   - Ensure constraints reflect enum values
   - Update CHECK constraints for enum validation

8. **Update Mappers** (`banking-logic/src/mappers/{file_name}_mapper.rs`)
   - Update conversion functions between domain and database models
   - Handle enum conversions properly
   - Ensure bidirectional mapping works correctly

9. **Update Service Implementations** (if needed)
   - Update any service implementations that use the models
   - Ensure type compatibility across layers

## Key Patterns

### Enum Serialization Pattern
```rust
// Custom serialization for database compatibility
#[serde(serialize_with = "serialize_enum_name", deserialize_with = "deserialize_enum_name")]
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

### String to Enum Conversion
```rust
// BEFORE: String type allowing invalid values
pub status: String,

// AFTER: Proper enum with validation
pub status: StatusEnum,
```

### HeaplessString Alignment
```rust
// Ensure consistent HeaplessString sizes between API and DB models
pub field_name: HeaplessString<N>, // Same N in both layers
```

## Benefits

- **Type Safety**: Enum types prevent invalid values at compile time
- **Memory Efficiency**: Reduced memory usage compared to String fields
- **Data Integrity**: Database constraints ensure valid enum values
- **Maintainability**: Clear separation between domain and database concerns
- **Performance**: Faster enum comparisons vs string comparisons

## Validation Checklist

- [ ] All enums from API domain are present in database model
- [ ] Custom serialization functions work correctly
- [ ] Database schema constraints match enum variants
- [ ] Mappers handle all enum conversions
- [ ] Service implementations compile without errors
- [ ] Field types are consistent between layers
- [ ] HeaplessString sizes match between API and DB models


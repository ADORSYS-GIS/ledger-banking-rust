# Command: Code Normalization

**Objective**: This template provides a structured approach for aligning domain models with database models, refactoring enums, and normalizing foreign key references in the ledger-banking-rust project.

**Parameters**:
-   `<module_name>`: The module name (e.g., `person`).
-   `<entity_name>`: The entity name (e.g., `country`).
-   `<tasks>` (optional): The target tasks to execute.

## Instructions

This command combines three normalization tasks:
1.  **Foreign Key Normalization**: Standardizes foreign key field names for clarity and consistency.
2.  **Model-to-Domain Alignment**: Aligns API domain models with database models.
3.  **Enum Refactoring**: Decouples domain enums from database enums.

For detailed guidelines on modeling, foreign key conventions, and serialization, refer to the [Modeling Guidelines](../../.docs/guidelines/modeling.md).

### Usage

Replace `{module_name}` with the module name (e.g., `person`) and `{entity_name}` with the entity name (e.g., `country`).

Replace `<tasks>` with the target tasks. e.g.:
    - `/normalize person person` or `/normalize person person 1..3` will execute all tasks
    - `/normalize person country 2` will perform only task 2
    - `/normalize person country 1,3` will perform only tasks 1 and 3

### Prompt Template

```
Using the database models in @banking-db/src/models/{module_name}/{entity_name}.rs as the source of truth, align their counterpart domain models in @banking-api/src/domain/{module_name}/{entity_name}.rs. Provided or modify service traits in @/banking-api/src/service/{module_name}/{entity_name}_service.rs to use domain structs and enums. Provide or modify mappers for structs and enums in @/banking-logic/src/mappers/{module_name}_mapper.rs to fit with domain and model objects. Also modify @banking-db-postgres/migrations/*.sql to fit the new data definition. Modify repository definition in @/banking-db/src/repository/{module_name}/{entity_name}_repository.rs and the corresponding implementation @/banking-db-postgres/src/repository/{module_name}/{entity_name}_repository/repo_impl.rs. Modify the service implementation in @/banking-logic/src/services/{module_name}/{entity_name}_service_impl.rs. 
```

### Rule Precedence
**The single source of truth for repository traits, indexes, and caches is the structured comments within the `...Model` structs.** All normalization and generation tasks must be driven by these in-code instructions, which always override any other guidelines.

### Steps Performed : Analysis

1.  **Read Database Model (`banking-db/src/models/{module_name}/{entity_name}.rs`)**
2.  **Read API Domain Model (`banking-api/src/domain/{module_name}/{entity_name}.rs`)**
3.  **Read Database Schema** (`banking-db-postgres/migrations/*.sql`)
4.  **Produce Mapping Tables** (`target/modules/{module_name}/{entity_name}.md`)
5.  **Foreign Key Reference Analysis**
6.  **Proposal Generation** (`target/modules/{module_name}/{entity_name}/{StructName}.md`)
7.  **Prompt User to Approve files**

### Steps Performed : Changes
**Idempotency:** All update operations must be idempotent. Before applying any change, verify that the target code is not already in the desired state.

Upon approval, perform changes as indicated in the generated mapping and proposal files.

1.  **Update Domain Model** (`@banking-api/src/domain/{module_name}/{entity_name}.rs`)
2.  **Update Service Traits** (`@/banking-api/src/service/{module_name}/{entity_name}_service.rs`)
3.  **Update Database Schema** (`banking-db-postgres/migrations/*.sql`)
4.  **Update Mappers** (`banking-logic/src/mappers/{module_name}_mapper.rs`)
5.  **Update Repository Traits (Comment-Driven)** (`banking-db/src/repository/{module_name}/{entity_name}_repository.rs`)
6.  **Update Repository Implementation** (`banking-db-postgres/src/repository/{module_name}/{entity_name}_repository/repo_impl.rs`)
7.  **Update Service Implementations** (`banking-logic/src/services/{module_name}/{entity_name}_service_impl.rs`)
8.  **Update Tests**: Update all affected tests to reflect the changes. For detailed instructions, refer to the [Testing Guidelines](../../.docs/guidelines/testing.md).

### Validation Checklist

-   [ ] Check that all foreign key relationships are preserved.
-   [ ] Verify database schema is consistent.
-   [ ] Ensure HeaplessString sizes and field types are consistent between layers.
-   [ ] Check that mappers handle all enum conversions.
-   [ ] Run `cargo check` to ensure compilation.
-   [ ] Run `cargo clippy` to check for warnings.
-   [ ] Run tests: `cargo test --features postgres_tests -- --test-threads=1`

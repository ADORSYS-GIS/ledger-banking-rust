# Command: Implement Batch Repository

**Objective**: Create the batch repository implementation and test suite for a given domain entity.

**Parameters**:
- `<entity>`: The lowercase, singular name of the entity (e.g., `country`).
- `<module>`: The module the entity belongs to (e.g., `person`).

## 1. Naming Conventions

From the parameters, derive the following names:
- **PascalCase**: `<Entity>` (e.g., `Country`)
- **Plural**: `<entities>` (e.g., `countries`)
- **SNAKE_CASE_UPPER**: `<ENTITY>` (e.g., `COUNTRY`)
- **Repo Trait**: `<Module>Repos` (e.g., `PersonRepos`)

## 2. Instructions

1.  **Analyze Existing Implementation**: Before starting, review the existing `banking-db-postgres/src/repository/<module>/<entity>_repository_impl.rs` to understand if the entity uses auditing, versioning, hashing, or other specific patterns.

2.  **Refer to Guidelines**: For detailed instructions and code templates, please refer to the [Batch Operations Implementation Guidelines](../../docs/guidelines/batch_operations.md).

3.  **Create Files**:
    *   Create the batch implementation file at `banking-db-postgres/src/repository/<module>/<entity>_repository_batch_impl.rs`.
    *   Create the corresponding test file at `banking-db-postgres/tests/suites/<module>/<entity>_batch_operations_test.rs`.

4.  **Update `mod.rs`**:
    *   Add the new implementation file to `banking-db-postgres/src/repository/<module>/mod.rs`.
    *   Add the new test file to `banking-db-postgres/tests/suites/<module>/mod.rs`.
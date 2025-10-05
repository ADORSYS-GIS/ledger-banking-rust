# Command: Implement Batch Repository

**Objective**: Create the batch repository implementation and test suite for a given domain entity.

**Parameters**:
- `<entity>`: The lowercase, singular name of the entity (e.g., `country`).
- `<module>`: The module the entity belongs to (e.g., `person`).

## Instructions

1.  **Determine Naming Conventions**:
    From the parameters, derive the following names:
    - **PascalCase**: `<Entity>` (e.g., `Country`)
    - **Plural**: `<entities>` (e.g., `countries`)
    - **SNAKE_CASE_UPPER**: `<ENTITY>` (e.g., `COUNTRY`)
    - **Repo Trait**: `<Module>Repos` (e.g., `PersonRepos`)

2.  **Analyze Existing Implementation**: Before starting, review the existing `banking-db-postgres/src/repository/<module>/<entity>_repository/repo_impl.rs` to understand if the entity uses auditing, versioning, hashing, or other specific patterns.

3.  **Refer to Guidelines**: For detailed instructions and code templates, please refer to the [Batch Operations Implementation Guidelines](../../docs/guidelines/batch_operations.md).

4.  **Create Files**:
    *   Create the batch implementation file at `banking-db-postgres/src/repository/<module>/<entity>_repository/batch_impl.rs`.
    *   Create the corresponding test file within `batch_impl.rs` under a `#[cfg(test)]` module.

5.  **Update `mod.rs`**:
    *   Add the new implementation file to `banking-db-postgres/src/repository/<module>/<entity>_repository/mod.rs`.
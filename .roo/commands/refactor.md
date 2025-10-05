# Command: Repository Refactoring and Implementation

**Objective**: Centralize all repository creation, refactoring, and feature implementation tasks into a single, unified command. This command handles the creation of new chunked repositories, the addition of batch operations, and the refactoring of legacy monolithic repositories.

---
## Operation: Refactor Module to Entity Layout

**Objective**: Refactors a module to the new entity-based layout.

**Parameters**:
-   `<module_file_path>`: The path to the module file to refactor (e.g., `banking-db/src/models/audit.rs`).

**Instructions**:

Refactor the module at `{{module_file_path}}` to follow the entity-based file layout.

You must adhere to the following process:
1.  Identify all entities within the module by inspecting the contents of `{{module_file_path}}`.
2.  For each entity, apply the file and directory restructuring pattern across all four relevant crates (`banking-db`, `banking-db-postgres`, `banking-logic`, and `banking-api`).
3.  Move the contents of the existing monolithic module files into the new, entity-specific files.
4.  Update all `mod.rs` files to correctly reflect the new module structure.

For a detailed breakdown of the required file structure and the steps to follow, you must use the guide located at: `.docs/guidelines/module_2_entity_layout.md`. Use the `audit` module in that guide as a concrete example of the expected outcome.

---

## Operation: Refactor Legacy Repository to Chunked Pattern

**Objective**: Refactor a monolithic repository implementation into the modern chunked pattern.

**Parameters**:
- `<entity>`: The lowercase, singular name of the entity (e.g., `country`).
- `<module>`: The module the entity belongs to (e.g., `person`).

**Instructions**:
1.  **Create Directory Structure**:
    -   Create `banking-db-postgres/src/repository/<module>/<entity>_repository/`.
2.  **Move and Rename Implementations**:
    -   Move `.../<entity>_repository_impl.rs` to `.../<entity>_repository/repo_impl.rs`.
    -   Move `.../<entity>_repository_batch_impl.rs` to `.../<entity>_repository/batch_impl.rs`.
3.  **Create `mod.rs`**:
    -   Create `.../<entity>_repository/mod.rs`.
    -   Add `pub mod repo_impl;` and `pub use repo_impl::*;`.
    -   Add modules for all repository and batch methods.
4.  **Extract Repository Methods**:
    -   For each method in `repo_impl.rs`, move its implementation to a new file `.../<method_name>.rs`.
    -   Update `repo_impl.rs` to call the new function.
    -   Add the new module to `mod.rs`.
5.  **Extract Batch Methods**:
    -   For each method in `batch_impl.rs`, move its implementation to a new file `.../<method_name>.rs`.
    -   Update `batch_impl.rs` to call the new function.
    -   Add the new module to `mod.rs`.
6.  **Extract Batch Helpers**:
    -   Move helper functions from `batch_impl.rs` to `batch_helper.rs`.
7.  **Move Tests**:
    -   Move tests from the central `banking-db-postgres/tests/suites/` directory into the relevant method files under a `#[cfg(test)]` module.
8.  **Update Module Exports**:
    -   Update `banking-db-postgres/src/repository/<module>/mod.rs` to refer to the new `<entity>_repository` module.

---

## Operation: Create New Repository

**Objective**: Create a new, chunked PostgreSQL repository implementation for a given domain model.

**Parameters**:
- `<db_model_path>`: The file path of the database model (e.g., `@banking-db/src/models/{module_name}/{entity_name}.rs`).

**Instructions**:
1.  Analyze the database model at the provided path.
2.  Create the corresponding repository implementation following the chunked pattern.
3.  The implementation should include:
    -   A directory structure: `banking-db-postgres/src/repository/<module>/<entity>_repository/`.
    -   A `repo_impl.rs` file for the main struct and trait implementation.
    -   Separate files for each repository method.
    -   A `mod.rs` file to declare all public modules.
    -   Co-located unit tests within each method file.
4.  Refer to the [Repository and Indexing Strategy](../../.docs/guidelines/repository-and-indexing.md) for detailed patterns.

---

## Operation: Add Batch Implementation

**Objective**: Add the batch repository implementation and test suite to an existing repository for a given domain entity.

**Parameters**:
- `<entity>`: The lowercase, singular name of the entity (e.g., `country`).
- `<module>`: The module the entity belongs to (e.g., `person`).

**Instructions**:
1.  **Determine Naming Conventions**:
    -   PascalCase: `<Entity>` (e.g., `Country`)
    -   Plural: `<entities>` (e.g., `countries`)
    -   SNAKE_CASE_UPPER: `<ENTITY>` (e.g., `COUNTRY`)
2.  **Analyze Existing Implementation**: Review `banking-db-postgres/src/repository/<module>/<entity>_repository/repo_impl.rs` to understand existing patterns (auditing, versioning, etc.).
3.  **Refer to Guidelines**: For detailed instructions and code templates, see [Batch Operations Implementation Guidelines](../../.docs/guidelines/batch_operations.md).
4.  **Create Files**:
    -   Create the batch implementation file: `banking-db-postgres/src/repository/<module>/<entity>_repository/batch_impl.rs`.
    -   Create the corresponding test file within `batch_impl.rs` under a `#[cfg(test)]` module.
    -   Create chunked files for each batch method (`create_batch`, `load_batch`, `update_batch`, `delete_batch`) and a `batch_helper.rs`.
5.  **Update `mod.rs`**:
    -   Add the new modules to `banking-db-postgres/src/repository/<module>/<entity>_repository/mod.rs`.

---

## Operation: Refactor Domain Errors

**Objective**: Refactor the error handling for a service to improve modularity and align the service layer with the repository's domain-specific errors.

**Parameters**:
- `<entity>`: The name of the service struct to refactor.

**Instructions**:

**Before you begin, you must read and follow the instructions outlined in `'.docs/guidelines/repo-error-handling.md'`.**

Follow these steps precisely:

1.  **Define Domain-Specific Repository Errors** in `'banking-db/src/repository/person/<entity>_repository.rs'`.
2.  **Refactor Service-Level Errors** in `'banking-api/src/service/person/<entity>_service.rs'`.
3.  **Update Service Implementation** in `'banking-logic/src/services/person/<entity>_service_impl.rs'`.
4.  **Fix Affected Tests**.
---

## Operation: Annotate Entity for Generation

**Objective**: Automatically generate structured comment annotations for a given entity to prepare it for index and cache generation. This process requires human review to ensure correctness and optimal configuration.

**Parameters**:
-   `<module_name>`: The name of the module (e.g., `person`).
-   `<entity_name>`: The name of the entity (e.g., `country`).

**Instructions**:

This command analyzes an entity's source code and generates structured comments (`/// # ...`) that drive downstream code generation for repositories, indexes, and caches.

1.  **Analyze Entity**: Inspect the struct definition in `banking-db/src/models/{module_name}/{entity_name}.rs`.
2.  **Infer Conventions**: Identify primary keys, foreign keys, and potential indexable fields based on naming conventions and types.
3.  **Generate Annotations**: Create comment blocks for the main `...Model` struct and its fields. These annotations should specify:
    -   `# Repository Trait`: The FQN of the corresponding repository trait.
    -   `# Index`: Details for the `...IdxModel`, including the cache type.
    -   `# Audit`: Details for the `...AuditModel` if applicable.
    -   `# Trait method`: Repository methods related to specific fields.
    -   `# Index`: Field-specific index properties (primary, secondary, unique).
4.  **Require Review**: Present the generated annotations as a diff. A human developer must review, adjust, and approve these annotations before they are applied to the source file. This step is crucial for validating strategic decisions like cache types and index uniqueness.

---

## Operation: Generate Application-Managed Indexes

**Objective**: Apply the rules defined in '.docs/guidelines/repository-and-indexing.md' to generate the necessary code and database schema for application-managed indexes for a given module.

**Parameters**:
-   `<module_name>`: The name of the module (e.g., `person`).
-   `<entity_name>`: The name of the entity (e.g., `country`).

**Steps**:

**Rule Precedence:** All generation logic is driven by structured comments within the source `...Model` struct. These in-code instructions always take precedence over general guidelines.

1.  **Generate Index Model from Comment Instructions** in `banking-db/src/models/{module_name}/{entity_name}.rs`.
2.  **Generate Repository Methods** in `banking-db/src/repository/{module_name}/{entity_name}_repository.rs`.
3.  **Generate Database Migration Script** in `banking-db-postgres/migrations/<init_order>_initial_schema_{module_name}.sql`.
4.  **Apply Hashing Strategy** for string-based indexes.

For detailed instructions on each step, refer to the [Repository and Indexing Strategy](../../.docs/guidelines/repository-and-indexing.md).

---

## Operation: Generate Immutable Caches

**Objective**: Generates a thread-safe, immutable cache implementation for Rust structs based on caching instructions in comment blocks.

**Parameters**:
-   `<file to process>`: The file containing the Rust structs to generate caches for.

**Instructions**:

This command generates a thread-safe, immutable cache implementation for each Rust struct in a given file, based on caching instructions provided in a `# Cache` comment block.

**Rule Precedence:** The generation of an `...IdxModelCache` is driven entirely by the `/// # Cache` comment block on the corresponding `...Model` struct.

For detailed instructions on the generation process, refer to the [Repository and Indexing Strategy](../../.docs/guidelines/repository-and-indexing.md).
# Command: Repository Refactoring and Implementation

**Objective**: Centralize all repository creation, refactoring, and feature implementation tasks into a single, unified command. This command handles the creation of new chunked repositories, the addition of batch operations, and the refactoring of legacy monolithic repositories.

**Parameters**:
-   `<module_name>`: The module name (e.g., `person`).
-   `<entity_name>`: The entity name (e.g., `country`).
-   `<operations>` (optional): The target operations to execute.

### Usage

Replace `{module_name}` with the module name (e.g., `person`) and `{entity_name}` with the entity name (e.g., `country`).

Replace `<operations>` with the target operations. e.g.:
    - `/refactor person person` or `/refactor person person 1..9` will execute all operations
    - `/refactor person country 2` will perform only operation 2
    - `/refactor person country 2,4` will perform only operations 2 and 4

### Explanation: Full Repository Refactoring Workflow

1.  **`Refactor Module to Entity Layout`**: Restructure the module into the new entity-based layout.
2.  **`Refactor Domain Errors`**: Refactor the domain error handling for each new service.
3.  **`Annotate Entity for Generation`**: Analyze and annotate each entity model for code generation.
4.  **`Generate Application-Managed Indexes`**: Generate application-managed indexes from the annotations.
5.  **`Generate Immutable Caches`**: Generate immutable caches from the annotations.
6.  **`Refactor Legacy Repository to Chunked Pattern`**: For each new entity repository, refactor it to the chunked pattern.
7.  **`Add Batch Implementation`**: For each refactored repository, add a complete batch implementation.
8.  **`Create New Repository`**: Create a new, chunked PostgreSQL repository implementation for a given domain model.

---

## 1. Operation: Refactor Module to Entity Layout

**Objective**: Refactors a module to the new entity-based layout.

**Parameters**:
-   `<module_name>`: The name of the module to refactor (e.g., `audit`).

**Instructions**:

Refactor the module `{{module_name}}` to follow the entity-based file layout.

You must adhere to the following process:
1.  **Pre-condition Check**: Before refactoring, verify that the module has not already been refactored. If the target entity-specific file structure already exists (e.g., `banking-db/src/models/{{module_name}}/<entity_name>.rs`), skip this operation.
2.  Identify all entities within the module by inspecting the contents of `banking-db/src/models/{{module_name}}.rs`.
2.  For each entity, apply the file and directory restructuring pattern across all four relevant crates (`banking-db`, `banking-db-postgres`, `banking-logic`, and `banking-api`).
3.  Move the contents of the existing monolithic module files into the new, entity-specific files.
4.  Update all `mod.rs` files to correctly reflect the new module structure.

For a detailed breakdown of the required file structure and the steps to follow, you must use the guide located at: `.docs/guidelines/module_2_entity_layout.md`. Use the `audit` module in that guide as a concrete example of the expected outcome.

---

## 2. Operation: Refactor Domain Errors

**Objective**: Refactor the error handling for a service to improve modularity and align the service layer with the repository's domain-specific errors.

**Parameters**:
- `<module_name>`: The name of the module (e.g., `person`).
- `<entity_name>`: The name of the service struct to refactor.

**Instructions**:

**Before you begin, you must read and follow the instructions outlined in `'.docs/guidelines/repo-error-handling.md'`.**

Follow these steps precisely:

1.  **Pre-condition Check**: Before applying changes, verify that the domain errors have not already been refactored. Inspect the service implementation; if it already uses domain-specific error patterns (e.g., `map_err(Into::into)`), skip this operation.
2.  **Define Domain-Specific Repository Errors** in `'banking-db/src/repository/{{module_name}}/{{entity_name}}_repository.rs'`.
3.  **Refactor Service-Level Errors** in `'banking-api/src/service/{{module_name}}/{{entity_name}}_service.rs'`.
3.  **Update Service Implementation** in `'banking-logic/src/services/{{module_name}}/{{entity_name}}_service_impl.rs'`.
4.  **Fix Affected Tests**.
---

## 3. Operation: Annotate Entity for Generation

**Objective**: Automatically generate structured comment annotations for a given entity to prepare it for index and cache generation. This process requires human review to ensure correctness and optimal configuration.

**Parameters**:
-   `<module_name>`: The name of the module (e.g., `person`).
-   `<entity_name>`: The name of the entity (e.g., `country`).

**Instructions**:

This command analyzes an entity's source code and generates structured comments (`/// # ...`) that drive downstream code generation for repositories, audit, indexes, and caches.

1.  **Pre-condition Check**: Before generating annotations, check if the entity struct in the target file already contains the intended comments (e.g. `/// # Index`, `/// # Audit`). If an annotation are already present, skip the attempt to generate that annotation again.
2.  **Analyze Entity**: Inspect the struct definition in `banking-db/src/models/{module_name}/{entity_name}.rs`. A concrete example is found in `banking-db/src/models/person/person.rs`
3.  **Infer Conventions**: Identify primary keys, foreign keys, and potential indexable fields based on naming conventions and types.
3.  **Generate Annotations**: Create comment blocks for the main `...Model` struct and its fields. These annotations should specify:
    -   `# Repository Trait`: The FQN of the corresponding repository trait.
    -   `# Index`: Details for the `...IdxModel`, including the cache type. Only generate this instruction if you find a comment `# Indexable` in the struct comment.
    -   `# Audit`: Details for the `...AuditModel` if applicable. Only generate this instruction if you find a comment `# Auditable` in the struct comment.
    -   `# Trait method`: Repository methods related to specific fields.
    -   `# Index`: Field-specific index properties (primary, secondary, unique). Infer the field and guess which one could an index field.
4.  **Require Review**: Present the generated annotations as a diff. A human developer must review, adjust, and approve these annotations before they are applied to the source file. This step is crucial for validating strategic decisions like cache types and index uniqueness.

---

## 4. Operation: Generate Application-Managed Indexes

**Objective**: Apply the rules defined in '.docs/guidelines/repository-and-indexing.md' to generate the necessary code and database schema for application-managed indexes for a given module.

**Parameters**:
-   `<module_name>`: The name of the module (e.g., `person`).
-   `<entity_name>`: The name of the entity (e.g., `country`).

**Steps**:

**Rule Precedence:** All generation logic is driven by structured comments within the source `...Model` struct. These in-code instructions always take precedence over general guidelines.

1.  **Pre-condition Check**: Before generation, verify that the indexes have not already been created. Check for the existence of an `...IdxModel` struct in the entity's module file and for index-related methods in the repository trait. If they already exist, skip this operation.
2.  **Generate Index Model from Comment Instructions** in `banking-db/src/models/{module_name}/{entity_name}.rs`.
3.  **Generate Repository Methods** in `banking-db/src/repository/{module_name}/{entity_name}_repository.rs`.
3.  **Generate Database Migration Script** in `banking-db-postgres/migrations/<init_order>_initial_schema_{module_name}.sql`.
4.  **Apply Hashing Strategy** for string-based indexes.

For detailed instructions on each step, refer to the [Repository and Indexing Strategy](../../.docs/guidelines/repository-and-indexing.md).

---

## 5. Operation: Generate Immutable Caches

**Objective**: Generates a thread-safe, immutable cache implementation for Rust structs based on caching instructions in comment blocks.

**Parameters**:
-   `<module_name>`: The name of the module (e.g., `person`).
-   `<entity_name>`: The name of the entity (e.g., `country`).

**Instructions**:

This command generates a thread-safe, immutable cache implementation for the entity struct in `banking-db/src/models/{{module_name}}/{{entity_name}}.rs`, based on caching instructions provided in a `# Cache` comment block.

**Rule Precedence:** The generation of an `...IdxModelCache` is driven entirely by the `/// # Cache` comment block on the corresponding `...Model` struct.

For detailed instructions on the generation process, refer to the [Repository and Indexing Strategy](../../.docs/guidelines/repository-and-indexing.md).

---

## 6. Operation: Refactor Legacy Repository to Chunked Pattern

**Objective**: Refactor a monolithic or partially refactored repository implementation into the modern chunked pattern.

**Parameters**:
- `<module_name>`: The module the entity belongs to (e.g., `person`).
- `<entity_name>`: The lowercase, singular name of the entity (e.g., `country`).

**Instructions**:
1.  **Pre-condition Check**: Before proceeding, verify that the repository has not already been fully refactored. If all methods in `repo_impl.rs` and `batch_impl.rs` are already extracted into individual files, skip this operation.
2.  **Create Directory Structure (if needed)**:
    -   If it doesn't exist, create `banking-db-postgres/src/repository/{{module_name}}/{{entity_name}}_repository/`.
3.  **Move and Rename Implementations (if needed)**:
    -   If `.../{{entity_name}}_repository_impl.rs` exists at the module root, move it to `.../{{entity_name}}_repository/repo_impl.rs`.
    -   If `.../{{entity_name}}_repository_batch_impl.rs` exists at the module root, move it to `.../{{entity_name}}_repository/batch_impl.rs`.
3.  **Create or Update `mod.rs`**:
    -   Create `.../{{entity_name}}_repository/mod.rs` if it doesn't exist.
    -   Ensure it contains `pub mod repo_impl;` and `pub use repo_impl::*;`. If `batch_impl.rs` exists, also add `pub mod batch_impl;`.
    -   Add module declarations for all repository and batch methods that will be extracted.
4.  **Extract Repository Methods**:
    -   For each trait method implementation inside `repo_impl.rs`, move its implementation to a new file named `.../<method_name>.rs`.
    -   Update `repo_impl.rs` to contain only the struct definition and the trait implementation block, with each method calling the function from its respective file.
    -   Add the new module for the method to `mod.rs`.
5.  **Extract Batch Methods**:
    -   For each trait method implementation inside `batch_impl.rs`, move its implementation to a new file named `.../<method_name>.rs`.
    -   Update `batch_impl.rs` to call the new function.
    -   Add the new module to `mod.rs`.
6.  **Extract Batch Helpers**:
    -   Move any helper functions from `batch_impl.rs` to `batch_helper.rs`.
7.  **Move or Create Tests**:
    -   For each extracted method, move existing tests from `banking-db-postgres/tests/suites/` into the relevant method file under a `#[cfg(test)]` module.
    -   If no tests exist for a given method, create a new test suite within the method's file under the `#[cfg(test)]` module.
8.  **Update Module Exports**:
    -   Update `banking-db-postgres/src/repository/{{module_name}}/mod.rs` to refer to the new `{{entity_name}}_repository` module.
9.  **Consult Guidelines**:
    -   For a detailed explanation of the chunked pattern, refer to the [.docs/guidelines/repository-and-indexing.md](../../.docs/guidelines/repository-and-indexing.md) document.

---

## 7. Operation: Add Batch Implementation

**Objective**: Add the batch repository implementation and test suite to an existing repository for a given domain entity.

**Parameters**:
- `<module_name>`: The module the entity belongs to (e.g., `person`).
- `<entity_name>`: The lowercase, singular name of the entity (e.g., `country`).

**Instructions**:
1.  **Pre-condition Check**: Before adding the implementation, verify that a batch implementation file (`batch_impl.rs`) does not already exist in the target directory. If it exists, skip this operation.
2.  **Determine Naming Conventions**:
    -   PascalCase: `<Entity>` (e.g., `Country`)
    -   Plural: `<entities>` (e.g., `countries`)
    -   SNAKE_CASE_UPPER: `<ENTITY>` (e.g., `COUNTRY`)
3.  **Analyze Existing Implementation**: Review `banking-db-postgres/src/repository/{{module_name}}/{{entity_name}}_repository/repo_impl.rs` to understand existing patterns (auditing, versioning, etc.).
3.  **Refer to Guidelines**: For detailed instructions and code templates, see [Batch Operations Implementation Guidelines](../../.docs/guidelines/batch_operations.md).
4.  **Create Files**:
    -   Create the batch implementation file: `banking-db-postgres/src/repository/{{module_name}}/{{entity_name}}_repository/batch_impl.rs`.
    -   Create the corresponding test file within `batch_impl.rs` under a `#[cfg(test)]` module.
    -   Create chunked files for each batch method (`create_batch`, `load_batch`, `update_batch`, `delete_batch`) and a `batch_helper.rs`.
5.  **Update `mod.rs`**:
    -   Add the new modules to `banking-db-postgres/src/repository/{{module_name}}/{{entity_name}}_repository/mod.rs`.
---

## 8. Operation: Create New Repository

**Objective**: Create a new, chunked PostgreSQL repository implementation for a given domain model.

**Parameters**:
- `<module_name>`: The name of the module (e.g., `person`).
- `<entity_name>`: The name of the entity (e.g., `country`).

**Instructions**:
1.  **Pre-condition Check**: Before creating the repository, verify that the target directory (`banking-db-postgres/src/repository/{{module_name}}/{{entity_name}}_repository/`) does not already exist. If it exists, skip this operation.
2.  Analyze the database model at `banking-db/src/models/{{module_name}}/{{entity_name}}.rs`.
3.  Create the corresponding repository implementation following the chunked pattern.
4.  The implementation should include:
    -   A directory structure: `banking-db-postgres/src/repository/{{module_name}}/{{entity_name}}_repository/`.
    -   A `repo_impl.rs` file for the main struct and trait implementation.
    -   Separate files for each repository method.
    -   A `mod.rs` file to declare all public modules.
    -   Co-located unit tests within each method file.
4.  Refer to the [Repository and Indexing Strategy](../../.docs/guidelines/repository-and-indexing.md) for detailed patterns.


# Command: Create Repository Implementation

**Objective**: Create a PostgreSQL repository implementation for a given domain model, following the patterns outlined in the [Repository and Indexing Strategy](../../docs/guidelines/repository-and-indexing.md).

**Parameters**:
-   `<domain_model_path>`: The file path of the domain model (e.g., `@banking-api/src/domain/{module_name}/{entity_name}.rs`).

## Instructions

Your implementation should include:
- A chunked implementation with separate files for each method.
- A `repo_impl.rs` file for the main struct and trait implementation.
- A `mod.rs` file to declare all public modules.
- Co-located unit tests within each method file.

Please analyze the domain model at the provided path and create the corresponding repository implementation and tests.
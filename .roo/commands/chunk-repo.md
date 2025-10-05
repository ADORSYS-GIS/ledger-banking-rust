# Command: Chunk Repository Implementation

**Objective**: Refactor a monolithic repository implementation into the chunked pattern, where each method resides in its own file.

**Parameters**:
-   `<entity>`: The lowercase, singular name of the entity (e.g., `country`).
-   `<module>`: The module the entity belongs to (e.g., `person`).

## Instructions

1.  **Create Directory Structure**:
    -   Create a new directory: `banking-db-postgres/src/repository/<module>/<entity>_repository/`.

2.  **Move and Rename Implementation**:
    -   Move the existing `banking-db-postgres/src/repository/<module>/<entity>_repository_impl.rs` to `banking-db-postgres/src/repository/<module>/<entity>_repository/repo_impl.rs`.

3.  **Move and Rename Batch Implementation**:
    -   Move the existing `banking-db-postgres/src/repository/<module>/<entity>_repository_batch_impl.rs` to `banking-db-postgres/src/repository/<module>/<entity>_repository/batch_impl.rs`.

4.  **Create `mod.rs`**:
    -   Create a `banking-db-postgres/src/repository/<module>/<entity>_repository/mod.rs` file.
    -   Add `pub mod repo_impl;` and `pub use repo_impl::*;` to this file.
    -   Add `pub mod batch_impl;`, `pub mod batch_helper;`, `pub mod create_batch;`, `pub mod load_batch;`, `pub mod update_batch;`, `pub mod delete_batch;`

5.  **Extract Main Repository Methods**:
    -   For each method in the `impl <Entity>Repository for <Entity>RepositoryImpl` block in `repo_impl.rs`:
        -   Create a new file: `banking-db-postgres/src/repository/<module>/<entity>_repository/<method_name>.rs`.
        -   Move the method's implementation into this new file.
        -   Add the necessary `use` statements at the top of the new file.
        -   Add `pub mod <method_name>;` to `mod.rs`.
        -   Replace the method body in `repo_impl.rs` with a call to the new function (e.g., `crate::repository::<module>::<entity>_repository::<method_name>::<method_name>(self, ...).await`).

6.  **Extract Batch Repository Methods**:
    -   For each method (`create_batch`, `load_batch`, `update_batch`, `delete_batch`) in the `impl BatchRepository for <Entity>RepositoryImpl` block in `batch_impl.rs`:
        -   Create a new file: `banking-db-postgres/src/repository/<module>/<entity>_repository/<method_name>.rs`.
        -   Move the method's implementation into this new file.
        -   Add the necessary `use` statements.
        -   Update `batch_impl.rs` to call the function in the new module (e.g., `crate::repository::<module>::<entity>_repository::<method_name>::<method_name>(...).await`).

7.  **Extract Batch Helpers**:
    -   Move any helper functions from the `impl <Entity>RepositoryImpl` block within `batch_impl.rs` to a new `batch_helper.rs` file.

8.  **Move Tests**:
    -   Move tests from `banking-db-postgres/tests/suites/<module>/<entity>_repository_tests.rs` into the relevant method files (e.g., `save.rs`) under a `#[cfg(test)]` module.
    -   Move tests from `banking-db-postgres/tests/suites/<module>/<entity>_batch_operations_test.rs` into the corresponding new batch method files (e.g., tests for `create_batch` go into `create_batch.rs`).

9.  **Update `<module>/mod.rs`**:
    -   Update `banking-db-postgres/src/repository/<module>/mod.rs` to refer to the new module.
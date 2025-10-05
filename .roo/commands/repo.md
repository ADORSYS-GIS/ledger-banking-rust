You are an expert Rust developer working on a core banking system. When I provide you with a domain model file path (e.g., @banking-api/src/domain/{module_name}/{entity_name}.rs), you will create a PostgreSQL repository implementation following the patterns outlined in the [Repository and Indexing Strategy](../../docs/guidelines/repository-and-indexing.md).

Your implementation should include:
- A chunked implementation with separate files for each method.
- A `repo_impl.rs` file for the main struct and trait implementation.
- A `mod.rs` file to declare all public modules.
- Co-located unit tests within each method file.

Please analyze @banking-api/src/domain/account.rs and create the corresponding repository implementation and tests.
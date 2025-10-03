You are an expert Rust developer working on a core banking system. When I provide you with a domain model file path (e.g., @banking-api/src/domain/account.rs), you will:

1. Analyze the domain model to understand its structure and fields
2. Examine the corresponding database model in @banking-db/src/models/person
3. Review the service implementation in @banking-logic/src/services/person to understand how the repository is used
4. Review the mapper in @banking-logic/src/mappers/ to understand data transformations
5. Analyze the corresponding tables, types and indexes in the database init script @banking-db-postgres/migrations/*_person.sql to understand the database structure
6. Create a PostgreSQL repository implementation in @banking-db-postgres/src/repository/{module_name}/{entity_name}_repository/ following these patterns:
   - **Chunked Implementation**: Each repository method is implemented in its own file within the repository's module.
   - **`repo_impl.rs`**: Contains the main struct definition and the `impl Repository for ...` block, which delegates calls to the specific method files.
   - **`mod.rs`**: Declares all the public modules for each repository method.
   - **Method Files** (e.g., `save.rs`, `load.rs`): Contain the implementation for a single repository method and its corresponding unit tests within a `#[cfg(test)]` block.
   - Use `sqlx::query!()` or `sqlx::query_as!()` for type-safe queries.
   - Handle enum serialization/deserialization using the patterns in the database models.
   - Handle HeaplessString conversions properly.
   - Use proper error handling with BankingResult.
   - Follow the structure shown in the `country_repository` implementation.

7. Create tests within each method file (e.g., `save.rs`) inside a `#[cfg(test)]` module that:
   - Sets up a test database connection using helpers.
   - Tests the specific repository method with realistic data.
   - Verifies correct data transformations between domain and database models.
   - Handles edge cases and error conditions for that method.
   - Uses the `tokio` test runtime.

Please analyze @banking-api/src/domain/account.rs and create the corresponding repository implementation and tests.
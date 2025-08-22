You are an expert Rust developer working on a core banking system. When I provide you with a domain model file path (e.g., @banking-api/src/domain/account.rs), you will:

1. Analyze the domain model to understand its structure and fields
2. Examine the corresponding database model in @banking-db/src/models/
3. Review the service implementation in @banking-logic/src/services/ to understand how the repository is used
4. Review the mapper in @banking-logic/src/mappers/ to understand data transformations
5. Analyze the corresponding tables, types and indexes in the database init script @banking-db-postgres/migrations/*.sql to understand the database structure
6. Create a PostgreSQL repository implementation in @banking-db-postgres/src/repository/ following these patterns:
   - Use `sqlx::query!()` or `sqlx::query_as!()` for type-safe queries
   - Handle enum serialization/deserialization using the patterns in the database models
   - Convert BigDecimal to Decimal via string parsing when needed
   - Handle HeaplessString conversions properly
   - Implement all methods from the repository trait
   - Use proper error handling with BankingResult
   - Follow the structure shown in existing implementations like customer_repository_impl.rs, agent_network_repository_impl.rs, and calendar_repository_impl.rs

7. Create a test file in @banking-db-postgres/tests/ that:
   - Sets up a test database connection
   - Tests all repository methods with realistic data
   - Verifies correct data transformations between domain and database models
   - Handles edge cases and error conditions
   - Uses tokio test runtime

Please analyze @banking-api/src/domain/account.rs and create the corresponding repository implementation and tests.
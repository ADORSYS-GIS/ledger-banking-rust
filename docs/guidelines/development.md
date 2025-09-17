# Implementation Patterns & Best Practices

This document provides a high-level overview of the development guidelines for the Ledger Banking Rust project. For detailed information, please refer to the specific documents linked below.

## 1. Architecture & Design Patterns

*   **[Application Design Patterns](./application-patterns.md)**: Explains the core architectural patterns, including the `Command Pattern` for business logic and the `Unit of Work` pattern for transaction management.

*   **[Transactional Command Execution](./transactional-command.md)**: Outlines the pattern for executing business logic commands within atomic database transactions using the `UnitOfWork` and `ServiceFactory` patterns.

## 2. Data Layer & Repositories

*   **[Repository and Indexing Strategy](./repository-and-indexing.md)**: Describes the strategy for the data access layer, including repository design, application-managed indexes, referential integrity, and caching patterns.

*   **[Batch Operations](./batch_operations.md)**: Provides guidelines for implementing efficient bulk data operations using patterns like PostgreSQL's `UNNEST`.

*   **[Database Guidelines](./database.md)**: Details the rules for database interactions, type mappings, and the transactional `Executor` pattern.

*   **[Repository Error Handling](./repo-error-handling.md)**: Provides guidelines for implementing domain-specific error handling in the repository layer.

## 3. Modeling & Implementation

*   **[Modeling Guidelines](./modeling.md)**: Covers the patterns for defining and mapping domain and database models, including `enums` and `structs`.

## 4. Testing

*   **[Testing Guidelines](./testing.md)**: Covers testing strategies, including database test isolation with transactional rollback, service-level testing with mocks, and repository integration testing.


## 5. String Formatting

When formatting strings, use the new f-string like syntax. For example, instead of `format!("Hello, {}!", name)`, use `format!("Hello, {name}!")`. For debug formatting, instead of `format!("Value: {:?}", value)`, use `format!("Value: {value:?}")`. This is enforced by clippy.

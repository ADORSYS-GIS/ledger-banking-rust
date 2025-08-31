# Implementation Patterns & Best Practices

This document provides a high-level overview of the development guidelines for the Ledger Banking Rust project. For detailed information, please refer to the specific documents linked below.

## Core Guidelines

*   **[Modeling Guidelines](./modeling.md)**: Covers the patterns for defining and mapping domain and database models, including `enums` and `structs`.

*   **[Database Guidelines](./database.md)**: Details the rules for database interactions, type mappings between Rust and PostgreSQL, and the use of `sqlx`.

*   **[Application Design Patterns](./application-patterns.md)**: Explains the high-level architectural patterns used in the system, such as the `Command Pattern` for business logic and the `Unit of Work` pattern for transaction management.

*   **[Repository and Indexing Strategy](./repository-and-indexing.md)**: Describes the strategy for the data access layer, including repository design, application-managed indexes, and referential integrity.

*   **[Transactional Command Execution](./transactional-command.md)**: Outlines the pattern for executing business logic commands within atomic database transactions.

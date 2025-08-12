---
description: This template provides a structured approach for aligning domain models enums between the API layer and database layer in the ledger-banking-rust project.
argument-hint: <file_name to process>
---

# Enum API-Database Alignment Template

## Usage

Replace `{file_name}` with the actual module name (e.g., customer, account, transaction, etc.)

## Prompt Template
Refactor the codebase to decouple domain enums from their database model counterparts. For every enum defined in `banking-api/src/domain/{file_name}.rs`, create a corresponding, separate enum in `banking-db/src/models/{file_name}.rs`.

The new database model enum must:
1.  Have the exact same variants as its domain counterpart.
2.  Be decorated with the necessary `sqlx` attributes for database mapping.
3.  Implement the required traits for database interaction.

Use the following transformation as a template:

**Source Domain Enum (`banking-api/src/domain/agent_network.rs`):**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NetworkType {
    Internal,
    Partner,
    ThirdParty,
}
```

**Target Database Model Enum (`banking-db/src/models/agent_network.rs`):**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "network_type", rename_all = "PascalCase")]
pub enum NetworkType {
    Internal,
    Partner,
    ThirdParty,
}
```

After creating the separate enums, update all relevant mapper functions (e.g., `From<DomainType> for DbType` and `From<DbType> for DomainType` implementations) to correctly convert between the domain enum and its new database model counterpart for all enums within the `banking-logic/src/mappers/{file_name}_mapper.rs` module.
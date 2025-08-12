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

Additionally, implement the `FromStr` trait for the database model enum to allow for conversion from a string. This is useful for deserialization and other contexts where the enum is represented as a string.

**`FromStr` Implementation (`banking-db/src/models/account_hold.rs`):**
```rust
use std::str::FromStr;

impl FromStr for HoldPriority {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Critical" => Ok(HoldPriority::Critical),
            "High" => Ok(HoldPriority::High),
            "Standard" => Ok(HoldPriority::Standard),
            "Medium" => Ok(HoldPriority::Medium),
            "Low" => Ok(HoldPriority::Low),
            _ => Err(()),
        }
    }
}
```

After creating the separate enums, update all relevant mapper functions to correctly convert between the domain enum and its new database model counterpart for all enums within the `banking-logic/src/mappers/{file_name}_mapper.rs` module. Use helper methods within the mapper implementation for the conversions, as shown in the example below:

**Mapper Implementation (`banking-logic/src/mappers/agent_network_mapper.rs`):**
```rust
impl AgentNetworkMapper {
    // ... other mapping functions

    // Helper methods for enum conversions
    fn network_type_to_db(network_type: NetworkType) -> DbNetworkType {
        match network_type {
            NetworkType::Internal => DbNetworkType::Internal,
            NetworkType::Partner => DbNetworkType::Partner,
            NetworkType::ThirdParty => DbNetworkType::ThirdParty,
        }
    }

    fn network_type_from_db(db_type: DbNetworkType) -> NetworkType {
        match db_type {
            DbNetworkType::Internal => NetworkType::Internal,
            DbNetworkType::Partner => NetworkType::Partner,
            DbNetworkType::ThirdParty => NetworkType::ThirdParty,
        }
    }
}
```
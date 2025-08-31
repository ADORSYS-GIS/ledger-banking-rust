# Modeling Guidelines

## Domain and Database Models for Enum Types

Assuming the file containing the module uses the file name pattern <file_name>, eg.  for the module file_name=`account`, `banking-api/src/domain/{file_name}.rs` will be represented as `banking-api/src/domain/account.rs` 

The new database model enum must:
1.  Have the exact same variants as its domain counterpart.
2.  Be decorated with the necessary `sqlx` attributes for database mapping.
3.  Implement the required traits for database interaction.

Use the following transformation as a template:

**Source Domain Enum (`banking-api/src/domain/{file_name}.rs`):**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnumName {
    Internal,
    Partner,
    ThirdParty,
}
```

**Target Database Model Enum (`banking-db/src/models/{file_name}.rs`):**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "enum_name", rename_all = "PascalCase")]
pub enum EnumName {
    Internal,
    Partner,
    ThirdParty,
}
```

Additionally, implement the `FromStr` and `Display` trait for the database model enum to allow for conversion from and to a string. This is useful for serialization, deserialization and other contexts where the enum is represented as a string.

**`FromStr` Implementation (`banking-db/src/models/{file_name}.rs`):**
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

**`Display` Implementation (`banking-db/src/models/{file_name}.rs`):**
```rust
impl std::fmt::Display for HoldPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HoldPriority::Critical => write!(f, "Critical"),
            HoldPriority::High => write!(f, "High"),
            HoldPriority::Standard => write!(f, "Standard"),
            HoldPriority::Medium => write!(f, "Medium"),
            HoldPriority::Low => write!(f, "Low"),
        }
    }
}
```

**Mapper Implementation (`banking-logic/src/mappers/{file_name}_mapper.rs`):**
```rust
impl AgentNetworkMapper {
    // ... other mapping functions

    // Mapper for Enums
    pub fn network_type_to_db(network_type: NetworkType) -> DbNetworkType {
        match network_type {
            NetworkType::Internal => DbNetworkType::Internal,
            NetworkType::Partner => DbNetworkType::Partner,
            NetworkType::ThirdParty => DbNetworkType::ThirdParty,
        }
    }

    pub fn network_type_from_db(db_type: DbNetworkType) -> NetworkType {
        match db_type {
            DbNetworkType::Internal => NetworkType::Internal,
            DbNetworkType::Partner => NetworkType::Partner,
            DbNetworkType::ThirdParty => NetworkType::ThirdParty,
        }
    }
}
```

## Domain and Database Models for struct Types

**Mappers For Structs (`banking-logic/src/mappers/{file_name}_mapper.rs`):**

```rust
    /// Map from domain AgentNetwork to database AgentNetworkModel
    pub fn network_to_model(network: AgentNetwork) -> AgentNetworkModel {
        AgentNetworkModel {
            id: network.id,
            network_name: network.network_name,
            network_type: Self::network_type_to_db(network.network_type),
            status: Self::network_status_to_db(network.status),
            contract_external_id: network.contract_external_id,
            aggregate_daily_limit: network.aggregate_daily_limit,
            current_daily_volume: network.current_daily_volume,
            settlement_gl_code: network.settlement_gl_code,
            created_at: network.created_at,
            last_updated_at: network.created_at,
            updated_by_person_id: Uuid::nil(), // System UUID
        }
    }

    /// Map from database AgentNetworkModel to domain AgentNetwork
    pub fn network_from_model(model: AgentNetworkModel) -> AgentNetwork {
        AgentNetwork {
            id: model.id,
            network_name: model.network_name,
            network_type: Self::network_type_from_db(model.network_type),
            status: Self::network_status_from_db(model.status),
            contract_external_id: model.contract_external_id,
            aggregate_daily_limit: model.aggregate_daily_limit,
            current_daily_volume: model.current_daily_volume,
            settlement_gl_code: model.settlement_gl_code,
            created_at: model.created_at,
        }
    }
```

**Code Patterns**
```rust
// Builder pattern for domain models
let customer = Customer::builder(uuid, CustomerType::Corporate)
    .full_name("ACME Corporation Ltd")
    .risk_rating(RiskRating::Medium)
    .build()?;

// Memory-optimized types
pub id: Uuid,
pub currency: HeaplessString<3>,           // ISO 4217 codes
pub product_id: Uuid,                      // Foreign key
pub name_l1: HeaplessString<100>,             // Names/descriptions
pub account_status: AccountStatus,         // Type-safe enums vs String
pub description: Option<HeaplessString<200>>,
```

**Multi-language 'name' support**
- whenever we find a field named 'name' suggest change to 3 language fields.
pub name_l1: HeaplessString<100>,
pub name_l2: HeaplessString<100>,
pub name_l3: HeaplessString<100>,

## Key Rules
1. **Use builders** for domain models (>4 parameters)
2. **HeaplessString<N>** for bounded text fields  
3. **Enums** for status/type fields instead of String
4. **References (&T)** for function parameters
5. **PostgreSQL Enum Casting**: Use `$N::enum_name`
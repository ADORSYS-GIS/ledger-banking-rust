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

---

# Key Patterns from Normalization

## Foreign Key Reference Alignment

### Naming Convention Rules

#### 1. Explicit Entity Reference Required
When the field name doesn't clearly indicate the target rust file, include the target struct name:
- `domicile_branch_id: Uuid` → `domicile_agency_branch_id: Uuid` (target: `AgencyBranch`)
- `updated_by_person_id: Uuid` → `updated_by_person_id: Uuid` (target: `Person`)
- `controlled_by`  → `controlled_by_person_id: Uuid` (target: `Person`)
The method comment might indicate the tartget struct:
-     /// References Person.person_id
-     `agent_user_id`   → `agent_person_id: Uuid` (target: `Person`)

#### 2. Avoid Unnecessary Type Convertion
While defining the service and repository traits, keep the struct field natie types as far as possible. e.g. type conversion to string shall happen at the lattest possible location in the code.
e.g.:
**Bad Case**
banking-db/src/repository/person_repository.rs
```
    /// Get persons by external identifier
    async fn get_by_external_identifier(
        &self,
        identifier: &str,
    ) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>>;

```
**Good Case**
banking-db/src/repository/person_repository.rs
```
    /// Get persons by external identifier
    async fn get_by_external_identifier(
        &self,
        identifier: HeaplessString<50>,
    ) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>>;
```

#### 3. Keep Existing When Clear
When the field name already matches or clearly indicates the target entity:
- `collateral_id: Option<Uuid>` → **KEEP AS IS** (target: `Collateral`)
- `customer_id: Uuid` → **KEEP AS IS** (target: `Customer`)

#### 4. Compound Names Exception
For fields with compound qualifiers that don't have individual target structs:
- `loan_purpose_id: Option<Uuid>` → **KEEP AS IS** (target: `ReasonAndPurpose`, no `Purpose` struct exists)
- `transaction_reason_id: Option<Uuid>` → **KEEP AS IS** (target: `ReasonAndPurpose`, no `Reason` struct exists)

#### 5. Person Reference Pattern
All `*_by` fields reference `Person` and should include explicit `_person_id` suffix:
- `created_by_person_id: Uuid` → **KEEP AS IS** (already follows correct pattern)
- `updated_by_person_id: Uuid` → **KEEP AS IS** (already follows correct pattern)
- `approved_by: Option<Uuid>` → `approved_by_person_id: Option<Uuid>`

#### 6. Preserve Existing Correct Patterns
Fields that already follow the explicit entity reference pattern should not be modified:
- If field already includes target entity name (e.g., `created_by_person_id`, `updated_by_person_id`), keep unchanged
- Only modify fields that lack explicit entity reference (e.g., `approved_by` → `approved_by_person_id`, `imported_by` → `imported_by_person_id`)

### String Alignment

- **String to Enum Conversion**
```rust
// BEFORE: String type allowing invalid values
pub status: String,

// AFTER: Proper enum with validation
pub status: StatusEnum,
```

- **Bounded String**
Always keep HeaplessString, do not change them to String.
```rust
// Ensure consistent HeaplessString sizes between API and DB models
pub field_name: HeaplessString<N>, // Same N in both layers
```
Always suggest change of String type to HeaplessString.

### Model Only Struct

Following structs will not have counterpart in the domain:
- **`<StructName>IdxModel`**: Index models are used to cache indexes in memory. To check if an entity exists, the repository can query the index cache instead of the database. They have a corresponding database table and must include `version` and `hash` fields.
- **`<StructName>IdxModelCache`**: Cache for index models. They do not have a domain or database representation.
- **`<StructName>AuditModel`**: Audit models are used to track changes to an entity. They have a corresponding database table and must include `version`, `hash`, and `audit_log_id` fields. They do not have a domain representation.

## Serialization

### Database Models Enum Serialization Pattern
```rust
// Custom serialization for database compatibility for {enum_name}
#[serde(serialize_with = "serialize_{enum_name}", deserialize_with = "deserialize_{enum_name}")]
pub field_name: EnumType,

fn serialize_enum_name<S>(value: &EnumType, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        EnumType::Variant1 => "Variant1",
        EnumType::Variant2 => "Variant2",
    };
    serializer.serialize_str(value_str)
}
```
### Database Models Enum Deserialization Pattern
Also provide deserializer.

---

# Memory Optimization

- **HeaplessString<N>**: Stack-allocated bounded strings (60-70% heap reduction)
- **Enum Status Fields**: Type-safe status management vs String
- **Currency Codes**: ISO 4217 compliant HeaplessString<3>
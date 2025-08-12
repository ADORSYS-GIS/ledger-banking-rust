## Development Guidelines

### Code Patterns
```rust
// Builder pattern for domain models
let customer = Customer::builder(uuid, CustomerType::Corporate)
    .full_name("ACME Corporation Ltd")
    .risk_rating(RiskRating::Medium)
    .build()?;

// Memory-optimized types
pub id: Uuid,
pub currency: HeaplessString<3>,           // ISO 4217 codes
pub product_id: Uuid,                      // Replaces product_code
pub name_l1: HeaplessString<100>,             // Names/descriptions
pub account_status: AccountStatus,         // Type-safe enums vs String
pub description: Option<HeaplessString<200>>,
```

// Multi-language 'name' support
- whenever we find a field named 'name' suggest change to 3 language fields.
pub name_l1: HeaplessString<100>,
pub name_l2: HeaplessString<100>,
pub name_l3: HeaplessString<100>,


### Key Rules
1. **Use builders** for domain models (>4 parameters)
2. **HeaplessString<N>** for bounded text fields  
3. **Enums** for status/type fields instead of String
4. **References (&T)** for function parameters
5. **PostgreSQL Enum Casting**: Use `$N::enum_name` in SQL with `.to_string()` binding


### Type Mappings (Rust → PostgreSQL)
- `Uuid` → `UUID`
- `HeaplessString<N>` → `VARCHAR(N)`
- `Decimal` → `DECIMAL(15,2)`
- `DateTime<Utc>` → `TIMESTAMP WITH TIME ZONE`

## Implementation Patterns & Best Practices

### SQLx with PostgreSQL Enums
```rust
// ✅ Use sqlx::query with manual binding for enums
sqlx::query(
    "INSERT INTO accounts (account_type, account_status) VALUES ($1::account_type, $2::account_status)"
)
.bind(account.account_type.to_string())
.bind(account.account_status.to_string())
.execute(&pool).await?

// ✅ Implement Display for all domain enums
impl std::fmt::Display for AccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountStatus::Active => write!(f, "Active"),
            AccountStatus::Frozen => write!(f, "Frozen"),
        }
    }
}
```

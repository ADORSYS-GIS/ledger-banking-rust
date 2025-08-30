# Database Guidelines

## SQLx with PostgreSQL Enums
```rust
// ✅ Use sqlx::query with manual binding for enums
sqlx::query(
    "INSERT INTO accounts (account_type, account_status) VALUES ($1::account_type, $2::account_status)"
)
.bind(account.account_type)
.bind(account.account_status)
.execute(&pool).await?
```

## Type Mappings (Rust → PostgreSQL)
- `Uuid` → `UUID`
- `HeaplessString<N>` → `VARCHAR(N)`
- `Decimal` → `DECIMAL(15,2)`
- `DateTime<Utc>` → `TIMESTAMP WITH TIME ZONE`

## Database Mapping Rules
- Foreign key constraints must be enforced in the repository layer, not the database. Use database schema for structure only, with no foreign key constraints.
- Date fields (e.g. `created_at`, `updated_at`) must be initialized in the domain layer. Database triggers for timestamp management are prohibited.
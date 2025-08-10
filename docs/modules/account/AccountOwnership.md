# Struct: AccountOwnership

| Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---|
| `id` | `Uuid` | `id` | `Uuid` | `id` | `UUID` | Unique identifier | None |
| `account_id` | `Uuid` | `account_id` | `Uuid` | `account_id` | `UUID` | Account identifier | None |
| `customer_id` | `Uuid` | `customer_id` | `Uuid` | `customer_id` | `UUID` | Customer identifier | None |
| `ownership_type` | `OwnershipType` | `ownership_type` | `OwnershipType` | `ownership_type` | `ownership_type` | Type of ownership | None |
| `ownership_percentage` | `Option<Decimal>` | `ownership_percentage` | `Option<Decimal>` | `ownership_percentage` | `DECIMAL(5,2)` | Percentage of ownership | None |
| `created_at` | `DateTime<Utc>` | `created_at` | `DateTime<Utc>` | `created_at` | `TIMESTAMPTZ` | Creation timestamp | None |
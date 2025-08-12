# Product Domain Alignment

This document outlines the alignment between the API domain models and the database models for the `product` module.

## Struct and Table Mappings

| Domain Struct (`banking-api/src/domain/product.rs`) | Database Model (`banking-db/src/models/product.rs`) | Database Tables | Flattened / Details | Change to Perform |
|---|---|---|---|---|
| `Product` | `ProductModel` | `products` | The `rules` field is a referenc to the `ProductRules` struct. |  change field `rules` to `products_rules` |
| `ProductRules` | `ProductRules` | `products` (JSONB) | Embedded within `ProductModel`. | Make it a proper table with field and a new field id: Uuid |
| `GlMapping` | `GlMappingModel` | `gl_mappings` | New table required. | Create `GlMappingModel` and `gl_mappings` table. |
| `InterestRateTier` | `InterestRateTierModel` | `interest_rate_tiers` | New table required. | Create `InterestRateTierModel` and `interest_rate_tiers` table. |

## Field Mappings

### `Product` / `ProductModel`

| `Product` | Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---|---|
| Product | `id` | `Uuid` | `id` | `Uuid` | `id` | `UUID` | Unique identifier | None |
| Product | `product_code` | `HeaplessString<50>` | `product_code` | `heapless::String<50>` | `product_code` | `VARCHAR(50)` | Product code | None |
| Product | `name_l1` | `HeaplessString<100>` | `name_l1` | `heapless::String<100>` | `name_l1` | `VARCHAR(100)` | Name L1 | None |
| Product | `name_l2` | `HeaplessString<100>` | `name_l2` | `heapless::String<100>` | `name_l2` | `VARCHAR(100)` | Name L2 | None |
| Product | `name_l3` | `HeaplessString<100>` | `name_l3` | `heapless::String<100>` | `name_l3` | `VARCHAR(100)` | Name L3 | None |
| Product | `description` | `HeaplessString<255>` | `description` | `heapless::String<255>` | `description` | `VARCHAR(255)` | Description | None |
| Product | `is_active` | `bool` | `is_active` | `bool` | `is_active` | `BOOLEAN` | Is active flag | None |
| Product | `valid_from` | `NaiveDate` | `valid_from` | `NaiveDate` | `valid_from` | `DATE` | Validity start date | None |
| Product | `valid_to` | `Option<NaiveDate>` | `valid_to` | `Option<NaiveDate>` | `valid_to` | `DATE` | Validity end date | None |
| Product | `product_type` | `ProductType` | `product_type` | `ProductType` | `product_type` | `product_type` | Product type enum | None |
| Product | `rules` | `ProductRules` | `rules` | `ProductRules` | `rules` | `JSONB` | Product rules | changed to `product_rules_id` referencing `ProductRules` struct |
| Product | `created_at` | `DateTime<Utc>` | `created_at` | `DateTime<Utc>` | `created_at` | `TIMESTAMPTZ` | Creation timestamp | None |
| Product | `last_updated_at` | `DateTime<Utc>` | `last_updated_at` | `DateTime<Utc>` | `last_updated_at` | `TIMESTAMPTZ` | Last update timestamp | None |
| Product | `updated_by_person_id` | `Uuid` | `updated_by_person_id` | `Uuid` | `updated_by_person_id` | `UUID` | Updater person ID | None |

### `ProductRules`

| `ProductRules` | Domain Field | Domain Type | DB Field | DB Type | Description | Change to Perform |
|---|---|---|---|---|---|---|
| ProductRules | `interest_calculation_method` | `HeaplessString<50>` | `interest_calculation_method` | `heapless::String<50>` | Interest calculation method | None |
| ProductRules | `maintenance_fee_frequency` | `Option<HeaplessString<50>>` | `maintenance_fee_frequency` | `Option<heapless::String<50>>` | Maintenance fee frequency | None |
... keep all additional fields not listed here.

### `GlMapping` / `GlMappingModel` (New)

| `GlMapping` | Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description |
|---|---|---|---|---|---|---|---|
| GlMapping | `product_id` | `Uuid` | `product_id` | `Uuid` | `product_id` | `UUID` | Foreign key to products |
| GlMapping | `customer_account_code` | `HeaplessString<50>` | `customer_account_code` | `heapless::String<50>` | `customer_account_code` | `VARCHAR(50)` | GL code for customer accounts |
| GlMapping | `interest_expense_code` | `HeaplessString<50>` | `interest_expense_code` | `heapless::String<50>` | `interest_expense_code` | `VARCHAR(50)` | GL code for interest expense |
| GlMapping | `fee_income_code` | `HeaplessString<50>` | `fee_income_code` | `heapless::String<50>` | `fee_income_code` | `VARCHAR(50)` | GL code for fee income |
| GlMapping | `overdraft_code` | `Option<HeaplessString<50>>` | `overdraft_code` | `Option<heapless::String<50>>` | `overdraft_code` | `VARCHAR(50)` | GL code for overdraft |

### `InterestRateTier` / `InterestRateTierModel` (New)

| `InterestRateTier` | Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description |
|---|---|---|---|---|---|---|---|
| InterestRateTier | `minimum_balance` | `Decimal` | `minimum_balance` | `Decimal` | `minimum_balance` | `DECIMAL` | Minimum balance for tier |
| InterestRateTier | `maximum_balance` | `Option<Decimal>` | `maximum_balance` | `Option<Decimal>` | `maximum_balance` | `DECIMAL` | Maximum balance for tier |
| InterestRateTier | `interest_rate` | `Decimal` | `interest_rate` | `Decimal` | `interest_rate` | `DECIMAL` | Interest rate for tier |
| InterestRateTier | `tier_name` | `HeaplessString<100>` | `tier_name` | `heapless::String<100>` | `tier_name` | `VARCHAR(100)` | Name of the tier |

Please review and approve this plan.
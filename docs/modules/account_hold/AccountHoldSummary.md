# AccountHoldSummary Field Mapping

| `AccountHoldSummary` | Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---|---|
| AccountHoldSummary | `id` | `Uuid` | `id` | `Uuid` | `id` | `UUID` | Unique identifier for the hold summary | none |
| AccountHoldSummary | `account_balance_calculation_id` | `Uuid` | `account_balance_calculation_id` | `Uuid` | `account_balance_calculation_id` | `UUID` | Foreign key to `account_balance_calculations` table | none |
| AccountHoldSummary | `hold_type` | `HoldType` | `hold_type` | `HoldType` | `hold_type` | `TEXT` | The type of hold | none |
| AccountHoldSummary | `total_amount` | `Decimal` | `total_amount` | `Decimal` | `total_amount` | `DECIMAL` | The total amount of holds of this type | none |
| AccountHoldSummary | `hold_count` | `u32` | `hold_count` | `i32` | `hold_count` | `INTEGER` | The number of holds of this type | none |
| AccountHoldSummary | `priority` | `HoldPriority` | `priority` | `HoldPriority` | `priority` | `TEXT` | The priority of the hold | none |
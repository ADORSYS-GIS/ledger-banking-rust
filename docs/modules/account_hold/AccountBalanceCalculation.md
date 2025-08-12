# AccountBalanceCalculation Field Mapping

| `AccountBalanceCalculation` | Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---|---|
| AccountBalanceCalculation | `id` | `Uuid` | `id` | `Uuid` | `id` | `UUID` | Unique identifier for the balance calculation | none |
| AccountBalanceCalculation | `account_id` | `Uuid` | `account_id` | `Uuid` | `account_id` | `UUID` | Foreign key to the `accounts` table | none |
| AccountBalanceCalculation | `current_balance` | `Decimal` | `current_balance` | `Decimal` | `current_balance` | `DECIMAL` | The current balance of the account | none |
| AccountBalanceCalculation | `available_balance` | `Decimal` | `available_balance` | `Decimal` | `available_balance` | `DECIMAL` | The available balance of the account | none |
| AccountBalanceCalculation | `overdraft_limit` | `Option<Decimal>` | `overdraft_limit` | `Option<Decimal>` | `overdraft_limit` | `DECIMAL` | The overdraft limit of the account | none |
| AccountBalanceCalculation | `total_holds` | `Decimal` | `total_holds` | `Decimal` | `total_holds` | `DECIMAL` | The total amount of holds on the account | none |
| AccountBalanceCalculation | `active_hold_count` | `u32` | `active_hold_count` | `i32` | `active_hold_count` | `INTEGER` | The number of active holds on the account | none |
| AccountBalanceCalculation | `calculation_timestamp` | `DateTime<Utc>` | `calculation_timestamp` | `DateTime<Utc>` | `calculation_timestamp` | `TIMESTAMPTZ` | Timestamp of when the balance was calculated | none |
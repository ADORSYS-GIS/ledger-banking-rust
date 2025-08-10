# AccountHoldExpiryJob Field Mapping

| `AccountHoldExpiryJob` | Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---|---|
| AccountHoldExpiryJob | `id` | `Uuid` | `id` | `Uuid` | `id` | `UUID` | Unique identifier for the expiry job | none |
| AccountHoldExpiryJob | `processing_date` | `NaiveDate` | `processing_date` | `NaiveDate` | `processing_date` | `DATE` | The date the job was processed | none |
| AccountHoldExpiryJob | `expired_holds_count` | `u32` | `expired_holds_count` | `i32` | `expired_holds_count` | `INTEGER` | The number of holds that expired | none |
| AccountHoldExpiryJob | `total_released_amount` | `Decimal` | `total_released_amount` | `Decimal` | `total_released_amount` | `DECIMAL` | The total amount released by the job | none |
| AccountHoldExpiryJob | `processed_at` | `DateTime<Utc>` | `processed_at` | `DateTime<Utc>` | `processed_at` | `TIMESTAMPTZ` | Timestamp of when the job was processed | none |
| AccountHoldExpiryJob | `errors` | `Vec<HeaplessString<100>>` | `errors` | `Option<Vec<String>>` | `errors` | `TEXT[]` | Any errors that occurred during processing | none |
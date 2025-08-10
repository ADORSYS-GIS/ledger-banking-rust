# AccountHold Field Mapping

| `AccountHold` | Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---|---|
| AccountHold | `id` | `Uuid` | `id` | `Uuid` | `id` | `UUID` | Unique identifier for the account hold | none |
| AccountHold | `account_id` | `Uuid` | `account_id` | `Uuid` | `account_id` | `UUID` | Foreign key to the `accounts` table | none |
| AccountHold | `amount` | `Decimal` | `amount` | `Decimal` | `amount` | `DECIMAL(15,2)` | The amount of money being held | none |
| AccountHold | `hold_type` | `HoldType` | `hold_type` | `HoldType` | `hold_type` | `hold_type` | The type of hold | The `hold_type` enum in the database needs to be updated to match the domain enum. |
| AccountHold | `reason_id` | `Uuid` | `reason_id` | `Uuid` | `reason_id` | `UUID` | Foreign key to `reason_and_purpose` table | none |
| AccountHold | `additional_details` | `Option<HeaplessString<200>>` | `additional_details` | `Option<HeaplessString<200>>` | `additional_details` | `VARCHAR(200)` | Additional context for the hold | none |
| AccountHold | `placed_by_person_id` | `Uuid` | `placed_by_person_id` | `Uuid` | `placed_by_person_id` | `UUID` | Foreign key to `persons` table | none |
| AccountHold | `placed_at` | `DateTime<Utc>` | `placed_at` | `DateTime<Utc>` | `placed_at` | `TIMESTAMPTZ` | Timestamp of when the hold was placed | none |
| AccountHold | `expires_at` | `Option<DateTime<Utc>>` | `expires_at` | `Option<DateTime<Utc>>` | `expires_at` | `TIMESTAMPTZ` | Timestamp of when the hold expires | none |
| AccountHold | `status` | `HoldStatus` | `status` | `HoldStatus` | `status` | `hold_status` | The status of the hold | The `hold_status` enum in the database needs to be updated to match the domain enum. |
| AccountHold | `released_at` | `Option<DateTime<Utc>>` | `released_at` | `Option<DateTime<Utc>>` | `released_at` | `TIMESTAMPTZ` | Timestamp of when the hold was released | none |
| AccountHold | `released_by_person_id` | `Option<Uuid>` | `released_by_person_id` | `Option<Uuid>` | `released_by_person_id` | `UUID` | Foreign key to `persons` table | none |
| AccountHold | `priority` | `HoldPriority` | `priority` | `HoldPriority` | `priority` | `hold_priority` | The priority of the hold | The `hold_priority` enum in the database needs to be updated to match the domain enum. |
| AccountHold | `source_reference` | `Option<HeaplessString<100>>` | `source_reference` | `Option<HeaplessString<100>>` | `source_reference` | `VARCHAR(100)` | External reference for the hold | none |
| AccountHold | `automatic_release` | `bool` | `automatic_release` | `bool` | `automatic_release` | `BOOLEAN` | Whether the hold should be automatically released | none |
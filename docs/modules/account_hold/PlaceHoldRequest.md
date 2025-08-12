# PlaceHoldRequest Field Mapping

| `PlaceHoldRequest` | Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---|---|
| PlaceHoldRequest | `account_id` | `Uuid` | `account_id` | `Uuid` | `account_id` | `UUID` | Foreign key to the `accounts` table | none |
| PlaceHoldRequest | `hold_type` | `HoldType` | `hold_type` | `HoldType` | `hold_type` | `TEXT` | The type of hold | none |
| PlaceHoldRequest | `amount` | `Decimal` | `amount` | `Decimal` | `amount` | `DECIMAL` | The amount of money being held | none |
| PlaceHoldRequest | `reason_id` | `Uuid` | `reason_id` | `Uuid` | `reason_id` | `UUID` | Foreign key to `reason_and_purpose` table | none |
| PlaceHoldRequest | `additional_details` | `Option<HeaplessString<200>>` | `additional_details` | `Option<HeaplessString<200>>` | `additional_details` | `VARCHAR(200)` | Additional context for the hold | none |
| PlaceHoldRequest | `placed_by_person_id` | `Uuid` | `placed_by_person_id` | `Uuid` | `placed_by_person_id` | `UUID` | Foreign key to `persons` table | none |
| PlaceHoldRequest | `expires_at` | `Option<DateTime<Utc>>` | `expires_at` | `Option<DateTime<Utc>>` | `expires_at` | `TIMESTAMPTZ` | Timestamp of when the hold expires | none |
| PlaceHoldRequest | `priority` | `HoldPriority` | `priority` | `HoldPriority` | `priority` | `TEXT` | The priority of the hold | none |
| PlaceHoldRequest | `source_reference` | `Option<HeaplessString<100>>` | `source_reference` | `Option<HeaplessString<100>>` | `source_reference` | `VARCHAR(100)` | External reference for the hold | none |
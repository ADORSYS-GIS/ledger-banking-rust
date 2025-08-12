# AccountHoldReleaseRequest Field Mapping

| `AccountHoldReleaseRequest` | Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---|---|
| AccountHoldReleaseRequest | `id` | `Uuid` | `id` | `Uuid` | `id` | `UUID` | Unique identifier for the release request | none |
| AccountHoldReleaseRequest | `hold_id` | `Uuid` | `hold_id` | `Uuid` | `hold_id` | `UUID` | Foreign key to `account_holds` table | none |
| AccountHoldReleaseRequest | `release_amount` | `Option<Decimal>` | `release_amount` | `Option<Decimal>` | `release_amount` | `DECIMAL` | The amount to release | none |
| AccountHoldReleaseRequest | `release_reason_id` | `Uuid` | `release_reason_id` | `Uuid` | `release_reason_id` | `UUID` | Foreign key to `reason_and_purpose` table | none |
| AccountHoldReleaseRequest | `release_additional_details` | `Option<HeaplessString<200>>` | `release_additional_details` | `Option<HeaplessString<200>>` | `release_additional_details` | `VARCHAR(200)` | Additional context for the release | none |
| AccountHoldReleaseRequest | `released_by_person_id` | `Uuid` | `released_by_person_id` | `Uuid` | `released_by_person_id` | `UUID` | Foreign key to `persons` table | none |
| AccountHoldReleaseRequest | `override_authorization` | `bool` | `override_authorization` | `bool` | `override_authorization` | `BOOLEAN` | Whether override authorization was used | none |
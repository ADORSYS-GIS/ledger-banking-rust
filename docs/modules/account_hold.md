# API-Database Alignment for `account_hold`

This document outlines the alignment between the API domain models in `banking-api/src/domain/account_hold.rs` and the database models in `banking-db/src/models/account_hold.rs`.

## Struct Mapping

| Domain Struct (`banking-api/src/domain/account_hold.rs`) | Database Model (`banking-db/src/models/account_hold.rs`) | Database Table | Flattened / Details | Change to Perform |
|---|---|---|---|---|
| `AccountHold` | `AccountHoldModel` | `account_holds` | Direct mapping. | Align `String` to `HeaplessString`, add missing audit fields. |
| `AccountHoldSummary` | `AccountHoldSummaryModel` | `AccountHoldSummary` | Direct mapping. | Align `hold_count` type. |
| `AccountHoldReleaseRequest` | `AccountHoldReleaseRequestModel` | `AccountHoldReleaseRequest` | Direct mapping. | Align `String` to `HeaplessString`. |
| `AccountHoldExpiryJob` | `AccountHoldExpiryJobModel` | `AccountHoldExpiryJob` | Direct mapping. | Align `errors` type. |
| `PlaceHoldRequest` | `PlaceHoldRequestModel` | `PlaceHoldRequest` | Direct mapping. | Align `String` to `HeaplessString`. |
| `AccountBalanceCalculation` | `AccountBalanceCalculationModel` | `AccountBalanceCalculation` | No domain counterpart for `AccountBalanceCalculationModel`. | No change. |
| `HoldReleaseRecordModel` | `HoldReleaseRecordModel` | `N/A` | Model exists in DB, no domain counterpart. | No change. |
| `HoldPrioritySummary` | `HoldPrioritySummary` | `N/A` | Model exists in DB, no domain counterpart. | No change. |
| `HoldOverrideRecord` | `HoldOverrideRecord` | `N/A` | Model exists in DB, no domain counterpart. | No change. |
| `HoldAnalyticsSummary` | `HoldAnalyticsSummary` | `N/A` | Model exists in DB, no domain counterpart. | No change. |
| `HighHoldRatioAccount` | `HighHoldRatioAccount` | `N/A` | Model exists in DB, no domain counterpart. | No change. |
| `JudicialHoldReportData` | `JudicialHoldReportData` | `N/A` | Model exists in DB, no domain counterpart. | No change. |
| `HoldAgingBucket` | `HoldAgingBucket` | `N/A` | Model exists in DB, no domain counterpart. | No change. |
| `HoldValidationError` | `HoldValidationError` | `N/A` | Model exists in DB, no domain counterpart. | No change. |

## Detailed Field Mappings

### `AccountHold` / `AccountHoldModel`

| `AccountHold` | Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---|---|
| AccountHold | `id` | `Uuid` | `id` | `Uuid` | `id` | `UUID` | Unique identifier | None |
| AccountHold | `account_id` | `Uuid` | `account_id` | `Uuid` | `account_id` | `UUID` | Account identifier | None |
| AccountHold | `amount` | `Decimal` | `amount` | `Decimal` | `amount` | `DECIMAL(15,2)` | Hold amount | None |
| AccountHold | `hold_type` | `HoldType` | `hold_type` | `HoldType` | `hold_type` | `hold_type` | Type of hold | None |
| AccountHold | `reason_id` | `Uuid` | `reason_id` | `Uuid` | `reason_id` | `UUID` | Reason for the hold | None |
| AccountHold | `additional_details` | `Option<HeaplessString<200>>` | `additional_details` | `Option<String>` | `additional_details` | `VARCHAR(200)` | Additional details | Change DB model from `String` to `HeaplessString<200>`. |
| AccountHold | `placed_by_person_id` | `Uuid` | `placed_by_person_id` | `Uuid` | `placed_by_person_id` | `UUID` | Person who placed the hold | None |
| AccountHold | `placed_at` | `DateTime<Utc>` | `placed_at` | `DateTime<Utc>` | `placed_at` | `TIMESTAMPTZ` | Timestamp when hold was placed | None |
| AccountHold | `expires_at` | `Option<DateTime<Utc>>` | `expires_at` | `Option<DateTime<Utc>>` | `expires_at` | `TIMESTAMPTZ` | Expiry timestamp | None |
| AccountHold | `status` | `HoldStatus` | `status` | `HoldStatus` | `status` | `hold_status` | Status of the hold | None |
| AccountHold | `released_at` | `Option<DateTime<Utc>>` | `released_at` | `Option<DateTime<Utc>>` | `released_at` | `TIMESTAMPTZ` | Timestamp when hold was released | None |
| AccountHold | `released_by_person_id` | `Option<Uuid>` | `released_by_person_id` | `Option<Uuid>` | `released_by_person_id` | `UUID` | Person who released the hold | None |
| AccountHold | `priority` | `HoldPriority` | `priority` | `HoldPriority` | `priority` | `hold_priority` | Priority of the hold | None |
| AccountHold | `source_reference` | `Option<HeaplessString<100>>` | `source_reference` | `Option<String>` | `source_reference` | `VARCHAR(100)` | External reference | Change DB model from `String` to `HeaplessString<100>`. |
| AccountHold | `automatic_release` | `bool` | `automatic_release` | `bool` | `automatic_release` | `BOOLEAN` | Automatic release flag | None |
| AccountHold | `N/A` | `N/A` | `created_at` | `DateTime<Utc>` | `created_at` | `TIMESTAMPTZ` | Audit field | Add to domain struct. |
| AccountHold | `N/A` | `N/A` | `updated_at` | `DateTime<Utc>` | `updated_at` | `TIMESTAMPTZ` | Audit field | Add to domain struct. |

### `AccountHoldSummary` / `AccountHoldSummaryModel`

| `AccountHoldSummary` | Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---|---|
| AccountHoldSummary | `id` | `Uuid` | `id` | `Uuid` | `id` | `UUID` | Unique identifier | None |
| AccountHoldSummary | `account_balance_calculation_id` | `Uuid` | `account_balance_calculation_id` | `Uuid` | `account_balance_calculation_id` | `UUID` | Balance calculation ID | None |
| AccountHoldSummary | `hold_type` | `HoldType` | `hold_type` | `HoldType` | `hold_type` | `TEXT` | Type of hold | None |
| AccountHoldSummary | `total_amount` | `Decimal` | `total_amount` | `Decimal` | `total_amount` | `DECIMAL` | Total amount of holds | None |
| AccountHoldSummary | `hold_count` | `u32` | `hold_count` | `i32` | `hold_count` | `INTEGER` | Number of holds | Align types (`u32` vs `i32`). Change DB model to `u32`. |
| AccountHoldSummary | `priority` | `HoldPriority` | `priority` | `HoldPriority` | `priority` | `TEXT` | Priority of the hold | None |

### `AccountHoldReleaseRequest` / `AccountHoldReleaseRequestModel`

| `AccountHoldReleaseRequest` | Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---|---|
| AccountHoldReleaseRequest | `id` | `Uuid` | `id` | `Uuid` | `id` | `UUID` | Unique identifier | None |
| AccountHoldReleaseRequest | `hold_id` | `Uuid` | `hold_id` | `Uuid` | `hold_id` | `UUID` | Hold identifier | None |
| AccountHoldReleaseRequest | `release_amount` | `Option<Decimal>` | `release_amount` | `Option<Decimal>` | `release_amount` | `DECIMAL` | Amount to release | None |
| AccountHoldReleaseRequest | `release_reason_id` | `Uuid` | `release_reason_id` | `Uuid` | `release_reason_id` | `UUID` | Reason for release | None |
| AccountHoldReleaseRequest | `release_additional_details` | `Option<HeaplessString<200>>` | `release_additional_details` | `Option<String>` | `release_additional_details` | `VARCHAR(200)` | Additional details for release | Change DB model from `String` to `HeaplessString<200>`. |
| AccountHoldReleaseRequest | `released_by_person_id` | `Uuid` | `released_by_person_id` | `Uuid` | `released_by_person_id` | `UUID` | Person releasing the hold | None |
| AccountHoldReleaseRequest | `override_authorization` | `bool` | `override_authorization` | `bool` | `override_authorization` | `BOOLEAN` | Override flag | None |

### `AccountHoldExpiryJob` / `AccountHoldExpiryJobModel`

| `AccountHoldExpiryJob` | Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---|---|
| AccountHoldExpiryJob | `id` | `Uuid` | `id` | `Uuid` | `id` | `UUID` | Unique identifier | None |
| AccountHoldExpiryJob | `processing_date` | `NaiveDate` | `processing_date` | `NaiveDate` | `processing_date` | `DATE` | Date of processing | None |
| AccountHoldExpiryJob | `expired_holds_count` | `u32` | `expired_holds_count` | `i32` | `expired_holds_count` | `INTEGER` | Count of expired holds | Align types (`u32` vs `i32`). Change DB model to `u32`. |
| AccountHoldExpiryJob | `total_released_amount` | `Decimal` | `total_released_amount` | `Decimal` | `total_released_amount` | `DECIMAL` | Total amount released | None |
| AccountHoldExpiryJob | `processed_at` | `DateTime<Utc>` | `processed_at` | `DateTime<Utc>` | `processed_at` | `TIMESTAMPTZ` | Timestamp of processing | None |
| AccountHoldExpiryJob | `errors` | `Vec<HeaplessString<100>>` | `errors` | `Option<Vec<String>>` | `errors` | `TEXT[]` | List of errors | Change DB model from `Option<Vec<String>>` to `Vec<HeaplessString<100>>`. |

### `PlaceHoldRequest` / `PlaceHoldRequestModel`

| `PlaceHoldRequest` | Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---|---|
| PlaceHoldRequest | `account_id` | `Uuid` | `account_id` | `Uuid` | `account_id` | `UUID` | Account identifier | None |
| PlaceHoldRequest | `hold_type` | `HoldType` | `hold_type` | `HoldType` | `hold_type` | `TEXT` | Type of hold | None |
| PlaceHoldRequest | `amount` | `Decimal` | `amount` | `Decimal` | `amount` | `DECIMAL` | Hold amount | None |
| PlaceHoldRequest | `reason_id` | `Uuid` | `reason_id` | `Uuid` | `reason_id` | `UUID` | Reason for the hold | None |
| PlaceHoldRequest | `additional_details` | `Option<HeaplessString<200>>` | `additional_details` | `Option<String>` | `additional_details` | `VARCHAR(200)` | Additional details | Change DB model from `String` to `HeaplessString<200>`. |
| PlaceHoldRequest | `placed_by_person_id` | `Uuid` | `placed_by_person_id` | `Uuid` | `placed_by_person_id` | `UUID` | Person who placed the hold | None |
| PlaceHoldRequest | `expires_at` | `Option<DateTime<Utc>>` | `expires_at` | `Option<DateTime<Utc>>` | `expires_at` | `TIMESTAMPTZ` | Expiry timestamp | None |
| PlaceHoldRequest | `priority` | `HoldPriority` | `priority` | `HoldPriority` | `priority` | `TEXT` | Priority of the hold | None |
| PlaceHoldRequest | `source_reference` | `Option<HeaplessString<100>>` | `source_reference` | `Option<String>` | `source_reference` | `VARCHAR(100)` | External reference | Change DB model from `String` to `HeaplessString<100>`. |
| PlaceHoldRequest | `N/A` | `N/A` | `id` | `Uuid` | `id` | `UUID` | Unique identifier for the request | Add to domain struct. |

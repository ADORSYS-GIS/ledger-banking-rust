# Struct: AccountStatusChangeRecord

| Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---|
| `id` | `Uuid` | `id` | `Uuid` | `id` | `UUID` | Unique identifier | None |
| `account_id` | `Uuid` | `account_id` | `Uuid` | `account_id` | `UUID` | Account identifier | None |
| `old_status` | `Option<AccountStatus>` | `old_status` | `Option<AccountStatus>` | `old_status` | `VARCHAR(30)` | Previous account status | The column type in `account_status_history` is `VARCHAR(30)`, which is correct, but the model name should be aligned. |
| `new_status` | `AccountStatus` | `new_status` | `AccountStatus` | `new_status` | `VARCHAR(30)` | New account status | The column type in `account_status_history` is `VARCHAR(30)`, which is correct, but the model name should be aligned. |
| `reason_id` | `Uuid` | `reason_id` | `Uuid` | `reason_id` | `UUID` | Reason for the status change | In `AccountStatusHistoryModel`, this is `reason_id`. In `StatusChangeRecordModel`, this is `reason_id`. The column in `account_status_history` is `change_reason_id`. Align to `reason_id`. |
| `additional_context` | `Option<HeaplessString<200>>` | `additional_context` | `Option<HeaplessString<200>>` | `additional_context` | `VARCHAR(200)` | Additional context for the change | None |
| `changed_by_person_id` | `Uuid` | `changed_by_person_id` | `Uuid` | `changed_by_person_id` | `UUID` | Person who made the change | In `StatusChangeRecordModel`, this is `changed_by`. Align to `changed_by_person_id`. |
| `changed_at` | `DateTime<Utc>` | `changed_at` | `DateTime<Utc>` | `changed_at` | `TIMESTAMPTZ` | Timestamp of the change | None |
| `system_triggered` | `bool` | `system_triggered` | `bool` | `system_triggered` | `BOOLEAN` | Whether the change was system-triggered | None |
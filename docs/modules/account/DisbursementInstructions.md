# Struct: DisbursementInstructions

| Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---|
| `id` | `Uuid` | `id` | `Uuid` | `id` | `UUID` | Unique identifier | None |
| `source_account_id` | `Uuid` | `source_account_id` | `Uuid` | `source_account_id` | `UUID` | Source account for funds | None |
| `method` | `DisbursementMethod` | `method` | `DisbursementMethod` | `method` | `disbursement_method` | Disbursement method | None |
| `target_account_id` | `Option<Uuid>` | `target_account_id` | `Option<Uuid>` | `target_account_id` | `UUID` | Target account for transfer | None |
| `cash_pickup_agency_branch_id` | `Option<Uuid>` | `cash_pickup_agency_branch_id` | `Option<Uuid>` | `cash_pickup_agency_branch_id` | `UUID` | Branch for cash pickup | None |
| `authorized_recipient_person_id` | `Option<Uuid>` | `authorized_recipient_person_id` | `Option<Uuid>` | `authorized_recipient_person_id` | `UUID` | Authorized recipient | None |
| `disbursement_amount` | `Option<Decimal>` | `disbursement_amount` | `Option<Decimal>` | `disbursement_amount` | `DECIMAL(15,2)` | Amount to disburse | None |
| `disbursement_date` | `Option<NaiveDate>` | `disbursement_date` | `Option<NaiveDate>` | `disbursement_date` | `DATE` | Date of disbursement | None |
| `stage_number` | `Option<i32>` | `stage_number` | `Option<i32>` | `stage_number` | `INTEGER` | Stage number for staged release | None |
| `stage_description` | `Option<HeaplessString<200>>` | `stage_description` | `Option<HeaplessString<200>>` | `stage_description` | `VARCHAR(200)` | Description of the stage | None |
| `status` | `DisbursementStatus` | `status` | `DisbursementStatus` | `status` | `disbursement_status` | Status of the disbursement | None |
| `created_at` | `DateTime<Utc>` | `created_at` | `DateTime<Utc>` | `created_at` | `TIMESTAMPTZ` | Creation timestamp | None |
| `last_updated_at` | `DateTime<Utc>` | `last_updated_at` | `DateTime<Utc>` | `last_updated_at` | `TIMESTAMPTZ` | Last update timestamp | None |
| `created_by_person_id` | `Uuid` | `created_by_person_id` | `Uuid` | `created_by_person_id` | `UUID` | Person who created the record | None |
| `updated_by_person_id` | `Uuid` | `updated_by_person_id` | `Uuid` | `updated_by_person_id` | `UUID` | Person who last updated the record | None |
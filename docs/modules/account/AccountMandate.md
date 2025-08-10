# Struct: AccountMandate

| Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---|
| `id` | `Uuid` | `id` | `Uuid` | `id` | `UUID` | Unique identifier | None |
| `account_id` | `Uuid` | `account_id` | `Uuid` | `account_id` | `UUID` | Account identifier | None |
| `grantee_customer_id` | `Uuid` | `grantee_customer_id` | `Uuid` | `grantee_customer_id` | `UUID` | Grantee customer identifier | None |
| `permission_type` | `PermissionType` | `permission_type` | `PermissionType` | `permission_type` | `permission_type` | Type of permission | None |
| `transaction_limit` | `Option<Decimal>` | `transaction_limit` | `Option<Decimal>` | `transaction_limit` | `DECIMAL(15,2)` | Transaction limit | None |
| `approver01_person_id` | `Option<Uuid>` | `approver01_person_id` | `Option<Uuid>` | `approver01_person_id` | `UUID` | Approver 1 | None |
| `approver02_person_id` | `Option<Uuid>` | `approver02_person_id` | `Option<Uuid>` | `approver02_person_id` | `UUID` | Approver 2 | None |
| `approver03_person_id` | `Option<Uuid>` | `approver03_person_id` | `Option<Uuid>` | `approver03_person_id` | `UUID` | Approver 3 | None |
| `approver04_person_id` | `Option<Uuid>` | `approver04_person_id` | `Option<Uuid>` | `approver04_person_id` | `UUID` | Approver 4 | None |
| `approver05_person_id` | `Option<Uuid>` | `approver05_person_id` | `Option<Uuid>` | `approver05_person_id` | `UUID` | Approver 5 | None |
| `approver06_person_id` | `Option<Uuid>` | `approver06_person_id` | `Option<Uuid>` | `approver06_person_id` | `UUID` | Approver 6 | None |
| `approver07_person_id` | `Option<Uuid>` | `approver07_person_id` | `Option<Uuid>` | `approver07_person_id` | `UUID` | Approver 7 | None |
| `required_signers_count` | `u8` | `required_signers_count` | `u8` | `required_signers_count` | `SMALLINT` | Number of required signers | None |
| `conditional_mandate_id` | `Option<Uuid>` | `conditional_mandate_id` | `Option<Uuid>` | `conditional_mandate_id` | `UUID` | Conditional mandate | None |
| `status` | `MandateStatus` | `status` | `MandateStatus` | `status` | `mandate_status` | Status of the mandate | None |
| `start_date` | `NaiveDate` | `start_date` | `NaiveDate` | `start_date` | `DATE` | Start date of the mandate | None |
| `end_date` | `Option<NaiveDate>` | `end_date` | `Option<NaiveDate>` | `end_date` | `DATE` | End date of the mandate | None |
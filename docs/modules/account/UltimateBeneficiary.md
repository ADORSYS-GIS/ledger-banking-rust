# Struct: UltimateBeneficiary

| Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---|
| `id` | `Uuid` | `id` | `Uuid` | `id` | `UUID` | Unique identifier | None |
| `corporate_customer_id` | `Uuid` | `corporate_customer_id` | `Uuid` | `corporate_customer_id` | `UUID` | Corporate customer identifier | None |
| `beneficiary_customer_id` | `Uuid` | `beneficiary_customer_id` | `Uuid` | `beneficiary_customer_id` | `UUID` | Beneficiary customer identifier | None |
| `ownership_percentage` | `Option<Decimal>` | `ownership_percentage` | `Option<Decimal>` | `ownership_percentage` | `DECIMAL(5,2)` | Percentage of ownership | None |
| `control_type` | `ControlType` | `control_type` | `ControlType` | `control_type` | `control_type` | Type of control | None |
| `description` | `Option<HeaplessString<200>>` | `description` | `Option<HeaplessString<200>>` | `description` | `VARCHAR(256)` | Description of control | None |
| `status` | `UboStatus` | `status` | `UboStatus` | `status` | `ubo_status` | Status of the UBO | None |
| `verification_status` | `VerificationStatus` | `verification_status` | `VerificationStatus` | `verification_status` | `verification_status` | Verification status | None |
| `created_at` | `DateTime<Utc>` | `created_at` | `DateTime<Utc>` | `created_at` | `TIMESTAMPTZ` | Creation timestamp | None |
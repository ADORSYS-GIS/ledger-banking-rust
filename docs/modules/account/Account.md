# Struct: Account

| Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---|
| `id` | `Uuid` | `id` | `Uuid` | `id` | `UUID` | Unique identifier for the account | None |
| `product_id` | `Uuid` | `product_id` | `Uuid` | `product_id` | `UUID` | Foreign key to the `products` table | None |
| `account_type` | `AccountType` | `account_type` | `AccountType` | `account_type` | `account_type` | Enum for account type (Savings, Current, Loan) | None |
| `account_status` | `AccountStatus` | `account_status` | `AccountStatus` | `account_status` | `account_status` | Enum for account status | None |
| `signing_condition` | `SigningCondition` | `signing_condition` | `SigningCondition` | `signing_condition` | `signing_condition` | Enum for signing conditions | None |
| `currency` | `HeaplessString<3>` | `currency` | `HeaplessString<3>` | `currency` | `VARCHAR(3)` | Currency of the account | None |
| `open_date` | `NaiveDate` | `open_date` | `NaiveDate` | `open_date` | `DATE` | Date the account was opened | None |
| `domicile_agency_branch_id` | `Uuid` | `domicile_agency_branch_id` | `Uuid` | `domicile_agency_branch_id` | `UUID` | Domicile branch | None |
| `current_balance` | `Decimal` | `current_balance` | `Decimal` | `current_balance` | `DECIMAL(15,2)` | Current balance | None |
| `available_balance` | `Decimal` | `available_balance` | `Decimal` | `available_balance` | `DECIMAL(15,2)` | Available balance | None |
| `accrued_interest` | `Decimal` | `accrued_interest` | `Decimal` | `accrued_interest` | `DECIMAL(15,2)` | Accrued interest | None |
| `overdraft_limit` | `Option<Decimal>` | `overdraft_limit` | `Option<Decimal>` | `overdraft_limit` | `DECIMAL(15,2)` | Overdraft limit | None |
| `original_principal` | `Option<Decimal>` | `original_principal` | `Option<Decimal>` | `original_principal` | `DECIMAL(15,2)` | Original loan principal | None |
| `outstanding_principal` | `Option<Decimal>` | `outstanding_principal` | `Option<Decimal>` | `outstanding_principal` | `DECIMAL(15,2)` | Outstanding loan principal | None |
| `loan_interest_rate` | `Option<Decimal>` | `loan_interest_rate` | `Option<Decimal>` | `loan_interest_rate` | `DECIMAL(8,6)` | Loan interest rate | None |
| `loan_term_months` | `Option<i32>` | `loan_term_months` | `Option<i32>` | `loan_term_months` | `INTEGER` | Loan term in months | None |
| `disbursement_date` | `Option<NaiveDate>` | `disbursement_date` | `Option<NaiveDate>` | `disbursement_date` | `DATE` | Loan disbursement date | None |
| `maturity_date` | `Option<NaiveDate>` | `maturity_date` | `Option<NaiveDate>` | `maturity_date` | `DATE` | Loan maturity date | None |
| `installment_amount` | `Option<Decimal>` | `installment_amount` | `Option<Decimal>` | `installment_amount` | `DECIMAL(15,2)` | Loan installment amount | None |
| `next_due_date` | `Option<NaiveDate>` | `next_due_date` | `Option<NaiveDate>` | `next_due_date` | `DATE` | Next loan payment due date | None |
| `penalty_rate` | `Option<Decimal>` | `penalty_rate` | `Option<Decimal>` | `penalty_rate` | `DECIMAL(8,6)` | Loan penalty rate | None |
| `collateral_id` | `Option<Uuid>` | `collateral_id` | `Option<Uuid>` | `collateral_id` | `UUID` | Reference to collateral | None |
| `loan_purpose_id` | `Option<Uuid>` | `loan_purpose_id` | `Option<Uuid>` | `loan_purpose_id` | `UUID` | Loan purpose | None |
| `close_date` | `Option<NaiveDate>` | `close_date` | `Option<NaiveDate>` | `close_date` | `DATE` | Date the account was closed | None |
| `last_activity_date` | `Option<NaiveDate>` | `last_activity_date` | `Option<NaiveDate>` | `last_activity_date` | `DATE` | Last activity date | None |
| `dormancy_threshold_days` | `Option<i32>` | `dormancy_threshold_days` | `Option<i32>` | `dormancy_threshold_days` | `INTEGER` | Days before account becomes dormant | None |
| `reactivation_required` | `bool` | `reactivation_required` | `bool` | `reactivation_required` | `BOOLEAN` | Whether reactivation is required | None |
| `pending_closure_reason_id` | `Option<Uuid>` | `pending_closure_reason_id` | `Option<Uuid>` | `pending_closure_reason_id` | `UUID` | Reason for pending closure | None |
| `last_disbursement_instruction_id` | `Option<Uuid>` | `last_disbursement_instruction_id` | `Option<Uuid>` | `last_disbursement_instruction_id` | `UUID` | Last disbursement instruction | None |
| `status_changed_by_person_id` | `Option<Uuid>` | `status_changed_by_person_id` | `Option<Uuid>` | `status_changed_by_person_id` | `UUID` | Person who last changed the status | None |
| `status_change_reason_id` | `Option<Uuid>` | `status_change_reason_id` | `Option<Uuid>` | `status_change_reason_id` | `UUID` | Reason for status change | None |
| `status_change_timestamp` | `Option<DateTime<Utc>>` | `status_change_timestamp` | `Option<DateTime<Utc>>` | `status_change_timestamp` | `TIMESTAMPTZ` | Timestamp of last status change | None |
| `most_significant_account_hold_id` | `Option<Uuid>` | `most_significant_account_hold_id` | `Option<Uuid>` | `most_significant_account_hold_id` | `UUID` | Most significant account hold | None |
| `account_ownership_id` | `Option<Uuid>` | `account_ownership_id` | `Option<Uuid>` | `account_ownership_id` | `UUID` | Account ownership record | None |
| `access01_account_relationship_id` | `Option<Uuid>` | `access01_account_relationship_id` | `Option<Uuid>` | `access01_account_relationship_id` | `UUID` | Access relationship 1 | None |
| `access02_account_relationship_id` | `Option<Uuid>` | `access02_account_relationship_id` | `Option<Uuid>` | `access02_account_relationship_id` | `UUID` | Access relationship 2 | None |
| `access03_account_relationship_id` | `Option<Uuid>` | `access03_account_relationship_id` | `Option<Uuid>` | `access03_account_relationship_id` | `UUID` | Access relationship 3 | None |
| `access04_account_relationship_id` | `Option<Uuid>` | `access04_account_relationship_id` | `Option<Uuid>` | `access04_account_relationship_id` | `UUID` | Access relationship 4 | None |
| `access05_account_relationship_id` | `Option<Uuid>` | `access05_account_relationship_id` | `Option<Uuid>` | `access05_account_relationship_id` | `UUID` | Access relationship 5 | None |
| `access06_account_relationship_id` | `Option<Uuid>` | `access06_account_relationship_id` | `Option<Uuid>` | `access06_account_relationship_id` | `UUID` | Access relationship 6 | None |
| `access07_account_relationship_id` | `Option<Uuid>` | `access07_account_relationship_id` | `Option<Uuid>` | `access07_account_relationship_id` | `UUID` | Access relationship 7 | None |
| `access11_account_mandate_id` | `Option<Uuid>` | `access11_account_mandate_id` | `Option<Uuid>` | `access11_account_mandate_id` | `UUID` | Access mandate 1 | None |
| `access12_account_mandate_id` | `Option<Uuid>` | `access12_account_mandate_id` | `Option<Uuid>` | `access12_account_mandate_id` | `UUID` | Access mandate 2 | None |
| `access13_account_mandate_id` | `Option<Uuid>` | `access13_account_mandate_id` | `Option<Uuid>` | `access13_account_mandate_id` | `UUID` | Access mandate 3 | None |
| `access14_account_mandate_id` | `Option<Uuid>` | `access14_account_mandate_id` | `Option<Uuid>` | `access14_account_mandate_id` | `UUID` | Access mandate 4 | None |
| `access15_account_mandate_id` | `Option<Uuid>` | `access15_account_mandate_id` | `Option<Uuid>` | `access15_account_mandate_id` | `UUID` | Access mandate 5 | None |
| `access16_account_mandate_id` | `Option<Uuid>` | `access16_account_mandate_id` | `Option<Uuid>` | `access16_account_mandate_id` | `UUID` | Access mandate 6 | None |
| `access17_account_mandate_id` | `Option<Uuid>` | `access17_account_mandate_id` | `Option<Uuid>` | `access17_account_mandate_id` | `UUID` | Access mandate 7 | None |
| `interest01_ultimate_beneficiary_id` | `Option<Uuid>` | `interest01_ultimate_beneficiary_id` | `Option<Uuid>` | `interest01_ultimate_beneficiary_id` | `UUID` | Ultimate beneficiary 1 | None |
| `interest02_ultimate_beneficiary_id` | `Option<Uuid>` | `interest02_ultimate_beneficiary_id` | `Option<Uuid>` | `interest02_ultimate_beneficiary_id` | `UUID` | Ultimate beneficiary 2 | None |
| `interest03_ultimate_beneficiary_id` | `Option<Uuid>` | `interest03_ultimate_beneficiary_id` | `Option<Uuid>` | `interest03_ultimate_beneficiary_id` | `UUID` | Ultimate beneficiary 3 | None |
| `interest04_ultimate_beneficiary_id` | `Option<Uuid>` | `interest04_ultimate_beneficiary_id` | `Option<Uuid>` | `interest04_ultimate_beneficiary_id` | `UUID` | Ultimate beneficiary 4 | None |
| `interest05_ultimate_beneficiary_id` | `Option<Uuid>` | `interest05_ultimate_beneficiary_id` | `Option<Uuid>` | `interest05_ultimate_beneficiary_id` | `UUID` | Ultimate beneficiary 5 | None |
| `interest06_ultimate_beneficiary_id` | `Option<Uuid>` | `interest06_ultimate_beneficiary_id` | `Option<Uuid>` | `interest06_ultimate_beneficiary_id` | `UUID` | Ultimate beneficiary 6 | None |
| `interest07_ultimate_beneficiary_id` | `Option<Uuid>` | `interest07_ultimate_beneficiary_id` | `Option<Uuid>` | `interest07_ultimate_beneficiary_id` | `UUID` | Ultimate beneficiary 7 | None |
| `created_at` | `DateTime<Utc>` | `created_at` | `DateTime<Utc>` | `created_at` | `TIMESTAMPTZ` | Creation timestamp | None |
| `last_updated_at` | `DateTime<Utc>` | `last_updated_at` | `DateTime<Utc>` | `last_updated_at` | `TIMESTAMPTZ` | Last update timestamp | None |
| `updated_by_person_id` | `Uuid` | `updated_by_person_id` | `Uuid` | `updated_by_person_id` | `UUID` | Person who last updated the record | None |
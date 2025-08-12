# Account Module: Domain-Database Mapping

This document outlines the alignment between the API domain models, database models, and database tables for the `account` module.

## Struct Mapping Summary

| Domain Struct (`banking-api/src/domain/account.rs`) | Database Model (`banking-db/src/models/account.rs`) | Database Table | Flattened / Details | Change to Perform |
|---|---|---|---|---|
| [`Account`](account/Account.md) | `AccountModel` | `accounts` | Direct mapping. | None |
| [`DisbursementInstructions`](account/DisbursementInstructions.md) | `DisbursementInstructionsModel` | `disbursement_instructions` | Direct mapping. | None |
| [`AccountStatusChangeRecord`](account/AccountStatusChangeRecord.md) | `AccountStatusHistoryModel` | `account_status_history` | Renamed model. | Align `AccountStatusHistoryModel` with `AccountStatusChangeRecord` and rename to `AccountStatusChangeRecordModel`. |
| [`AccountOwnership`](account/AccountOwnership.md) | `AccountOwnershipModel` | `account_ownership` | Direct mapping. | None |
| [`AccountRelationship`](account/AccountRelationship.md) | `AccountRelationshipModel` | `account_relationships` | Direct mapping. | None |
| [`AccountMandate`](account/AccountMandate.md) | `AccountMandateModel` | `account_mandates` | Direct mapping. | None |
| [`UltimateBeneficiary`](account/UltimateBeneficiary.md) | `UltimateBeneficiaryModel` | `ultimate_beneficial_owners` | Direct mapping. | None |
| *N/A* | `AccountFinalSettlementModel` | `final_settlements` | Exists only in DB layer. | Expose in the domain. |
| *N/A* | `StatusChangeRecordModel` | *N/A* | Duplicate of `AccountStatusHistoryModel`. | Remove `StatusChangeRecordModel`. |

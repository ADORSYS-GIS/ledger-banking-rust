# Account Hold Mappings

This document outlines the mapping between the domain structs, database models, and database tables for the `account_hold` module.

## Struct Mapping

| Domain Struct (`banking-api/src/domain/account_hold.rs`) | Database Model (`banking-db/src/models/account_hold.rs`) | Database Table (`banking-db-postgres/migrations/002_account_model_updates.sql`) | Flattened / Details | Changed |
|---|---|---|---|---|
| `AccountHold` | `AccountHoldModel` | `account_holds` | Direct mapping. | Yes |
| `AccountHoldReleaseRequest` | `AccountHoldReleaseRequestModel` | `AccountHoldReleaseRequest` | Direct mapping. | No |
| `AccountHoldExpiryJob` | `AccountHoldExpiryJobModel` | `AccountHoldExpiryJob` | Direct mapping. | No |
| `PlaceHoldRequest` | `PlaceHoldRequestModel` | `PlaceHoldRequest` | Direct mapping. | No |
| `AccountBalanceCalculation` | `AccountBalanceCalculationModel` | `AccountBalanceCalculation` | Direct mapping. | No |
| `AccountHoldSummary` | `AccountHoldSummaryModel` | `AccountHoldSummary` | Direct mapping. | No |

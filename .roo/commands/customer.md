In order to harmonize both domain and model, you will proceed with the following.

## Do not reuse domain structs in the database layer.
Define a copy of each of these structs in the @/banking-db/src/models/customer.rs 
- CustomerType
- IdentityType
- RiskRating
- CustomerStatus
- KycStatus

Modify references in the database layer to point to the new structs.

## Struct Missing in Domain Layer
Define a copy of following structs in the domai layer
- CustomerDocumentModel -> CustomerDocument
- CustomerAuditModel -> CustomerAudit

Also create a DocumentStatus in the domail layer.

## For enums, keep these serialization rules
- Domain: Uses standard Display and FromStr implementations
- Database: Uses custom serde serialization/deserialization functions for database compatibility

Provide service functions to CRUD struct added to domain layer

Implement mappers of enum and structs .

Correct database init file @/banking-db-postgres/migrations/001_initial_schema.sql if necessary

Use instructions in @/.roo/commands/enum-alignment.md  to include precision of the work to be done.
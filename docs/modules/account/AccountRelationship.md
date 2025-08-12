# Struct: AccountRelationship

| Domain Field | Domain Type | DB Field | DB Type | DB column | column type | Description | Change to Perform |
|---|---|---|---|---|---|---|---|
| `id` | `Uuid` | `id` | `Uuid` | `id` | `UUID` | Unique identifier | None |
| `account_id` | `Uuid` | `account_id` | `Uuid` | `account_id` | `UUID` | Account identifier | None |
| `person_id` | `Uuid` | `person_id` | `Uuid` | `person_id` | `UUID` | Person identifier | None |
| `entity_type` | `EntityType` | `entity_type` | `EntityType` | `entity_type` | `entity_type` | Type of entity | None |
| `relationship_type` | `RelationshipType` | `relationship_type` | `RelationshipType` | `relationship_type` | `relationship_type` | Type of relationship | None |
| `status` | `RelationshipStatus` | `status` | `RelationshipStatus` | `status` | `relationship_status` | Status of the relationship | None |
| `start_date` | `NaiveDate` | `start_date` | `NaiveDate` | `start_date` | `DATE` | Start date of the relationship | None |
| `end_date` | `Option<NaiveDate>` | `end_date` | `Option<NaiveDate>` | `end_date` | `DATE` | End date of the relationship | None |
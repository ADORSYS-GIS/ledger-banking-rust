---
description: Refactor foreign key field names to include target entity names for clarity
argument-hint: <entity>
---

# Foreign Key Reference Alignment

## Naming Convention Rules

### 1. Explicit Entity Reference Required
When the field name doesn't clearly indicate the target rust file, include the target struct name:
- `domicile_branch_id: Uuid` → `domicile_agency_branch_id: Uuid` (target: `AgencyBranch`)
- `updated_by_person_id: Uuid` → `updated_by_person_id: Uuid` (target: `Person`)
- `controlled_by`  → `controlled_by_person_id: Uuid` (target: `Person`)
The method comment might indicate the tartget struct:
-     /// References Person.person_id
-     `agent_user_id`   → `agent_person_id: Uuid` (target: `Person`)


### 2. Keep Existing When Clear
When the field name already matches or clearly indicates the target entity:
- `collateral_id: Option<Uuid>` → **KEEP AS IS** (target: `Collateral`)
- `customer_id: Uuid` → **KEEP AS IS** (target: `Customer`)

### 3. Compound Names Exception
For fields with compound qualifiers that don't have individual target structs:
- `loan_purpose_id: Option<Uuid>` → **KEEP AS IS** (target: `ReasonAndPurpose`, no `Purpose` struct exists)
- `transaction_reason_id: Option<Uuid>` → **KEEP AS IS** (target: `ReasonAndPurpose`, no `Reason` struct exists)

### 4. Person Reference Pattern
All `*_by` fields reference `Person` and should include explicit `_person_id` suffix:
- `created_by_person_id: Uuid` → **KEEP AS IS** (already follows correct pattern)
- `updated_by_person_id: Uuid` → **KEEP AS IS** (already follows correct pattern)
- `approved_by: Option<Uuid>` → `approved_by_person_id: Option<Uuid>`

### 5. Preserve Existing Correct Patterns
Fields that already follow the explicit entity reference pattern should not be modified:
- If field already includes target entity name (e.g., `created_by_person_id`, `updated_by_person_id`), keep unchanged
- Only modify fields that lack explicit entity reference (e.g., `approved_by` → `approved_by_person_id`, `imported_by` → `imported_by_person_id`)

## Workflow Steps

### Step 1: Analysis Phase
1. Analyze the domain struct file: `banking-api/src/domain/{entity}.rs`. Do not process other files if not explicitely requested.
2. Identify all fields ending with `_id: Uuid` or `_id: Option<Uuid>`
3. Identify all fields ending with `_by: Uuid` or `_by: Option<Uuid>`
4. For each field, determine:
   - Current field name
   - Target struct name and location
   - Whether renaming is needed based on rules above
   - Proposed new field name (if applicable)

### Step 2: Proposal Generation
Create/update `target/workbook.md` with:

```markdown
# Foreign Key Reference Analysis: {EntityName}

## Fields Requiring Changes
| Current Field | Source Struct |Target Struct | Target Location | Proposed Field | Reason |
|---------------|---------------|--------------|-----------------|----------------|---------|
| field_name | SourceStruct | TargetStruct | path/to/struct.rs | new_field_name | Rule explanation |

## Fields Keeping Current Names
| Current Field | Source Struct | Target Struct | Target Location | Reason to Keep |
|---------------|---------------|---------------|-----------------|----------------|
| field_name | SourceStruct | TargetStruct | path/to/struct.rs | Explanation |

## Summary
- Total fields analyzed: X
- Fields requiring changes: Y
- Fields keeping current names: Z
```

### Step 3: Review and Approval
1. Open `target/workbook.md` in editor for user review
2. Wait for user confirmation/corrections
3. Proceed only after explicit approval

### Step 4: Implementation Phase
For each approved field change, update in this order:

#### 4.1 Domain Model Update
- File: `banking-api/src/domain/{entity}.rs`
- Update field declarations in struct
- Update builder method parameters (if applicable)
- Update any struct methods referencing the field

#### 4.2 Service Trait Updates
- Files: `banking-api/src/service/*.rs`
- Update method signatures that reference the field
- Update trait documentation if field is mentioned

#### 4.3 Database Model Updates
- Files: `banking-db/src/models/{entity}.rs`
- Update corresponding model struct fields
- Update any model-specific methods

#### 4.4 Repository Trait Updates
- Files: `banking-db/src/repository/{entity}_repository.rs`
- Update method signatures and parameters
- Update trait documentation

#### 4.5 Mapper Updates
- Files: `banking-logic/src/mappers/{entity}_mapper.rs`
- Update domain-to-model and model-to-domain mappings
- Ensure field mappings are correct

#### 4.6 Service Implementation Updates
- Files: `banking-logic/src/services/{entity}_service_impl.rs`
- Update method implementations
- Update field references in business logic

#### 4.7 PostgreSQL Repository Updates
- Files: `banking-db-postgres/src/repository/{entity}_repository_impl.rs`
- Update SQL queries with new column names
- Update result mapping code
- Update any database-specific logic

#### 4.8 Database Schema Updates
- File: `banking-db-postgres/migrations/001_initial_schema.sql`
- Update column names in CREATE TABLE statements
- Update foreign key constraint names
- Update index definitions if they reference the columns

#### 4.9 Test Updates
- Update all test files that reference the changed fields:
  - `banking-logic/src/services/tests/*.rs`
  - `banking-db-postgres/tests/*.rs`
  - Any integration tests
- Update test data creation
- Update assertions and validations

### Step 5: Validation
1. Run `cargo check` to ensure compilation
2. Run `cargo clippy` to check for warnings
3. Run tests: `cargo test --features postgres_tests -- --test-threads=1`
4. Verify database schema is consistent
5. Check that all foreign key relationships are preserved

## Important Notes
1. **Preserve Relationships**: Ensure all foreign key relationships remain intact after renaming
2. **Database Consistency**: Column renames must be reflected in both schema and queries
3. **Test Coverage**: All tests must pass after changes to ensure functionality is preserved
4. **Incremental Approach**: Process one entity at a time to minimize complexity
5. **Backup Strategy**: Consider creating a branch before starting major refactoring
6. **Documentation**: Update any documentation that references the old field names
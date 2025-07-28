# Git Commit Template

This template provides a structured approach for creating comprehensive git commits in the ledger-banking-rust project.

## Commit Message Format

```
<type>(<scope>): <subject>

<body>

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

## Commit Types

- **feat**: New feature or capability
- **fix**: Bug fix or error correction
- **refactor**: Code restructuring without functionality changes
- **perf**: Performance improvements
- **style**: Code formatting, whitespace, style changes
- **docs**: Documentation updates
- **test**: Adding or updating tests
- **chore**: Maintenance tasks, dependency updates
- **build**: Build system or CI/CD changes
- **revert**: Reverting previous changes

## Scope Guidelines

Use specific component names:
- **api**: Changes to banking-api domain models
- **db**: Changes to banking-db models or repositories
- **logic**: Changes to banking-logic services or mappers
- **schema**: Database schema changes
- **migration**: Database migration files
- **workspace**: Multi-crate changes

## Subject Guidelines

- Use imperative mood ("add" not "added" or "adds")
- Start with lowercase letter
- No period at the end
- Maximum 50 characters
- Be specific and descriptive

## Body Guidelines

- Explain the **why** behind the change, not just what was changed
- Include context about the problem being solved
- List major changes in bullet points
- Reference issue numbers when applicable
- Use present tense
- Wrap lines at 72 characters

## Examples

### Feature Addition
```
feat(api): consolidate account relations into unified account domain

- Move AccountOwnership, AccountRelationship, AccountMandate, and UltimateBeneficiary from account_relations.rs to account.rs
- Remove duplicate account_relations module
- Update imports across all dependent modules
- Maintain backward compatibility for existing functionality

This change simplifies the domain structure by grouping related account entities together, improving code organization and reducing module complexity.

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

### Database Alignment
```
refactor(db): align account models with API domain using proper enums

- Convert String fields to strongly-typed enums (OwnershipType, EntityType, etc.)
- Add comprehensive serialization functions for database compatibility
- Update PostgreSQL schema with new enum types
- Enhance type safety and memory efficiency across data layers

This alignment eliminates runtime string validation errors and improves performance through enum usage instead of heap-allocated strings.

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

### Bug Fix
```
fix(logic): add missing disbursement_instructions field to test helpers

- Add disbursement_instructions field to create_test_account_model()
- Add disbursement_instructions field to create_test_loan_account_model()
- Initialize field as None to maintain existing test behavior

Resolves compilation errors in account service tests after AccountModel schema updates.

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

## Pre-Commit Checklist

- [ ] All tests pass (`cargo test --workspace`)
- [ ] Code compiles without warnings (`cargo check --workspace`)
- [ ] No sensitive information in commit
- [ ] Commit message follows template format
- [ ] Changes are logically grouped
- [ ] Documentation updated if needed

## Signing Guidelines

Always use both `-s` (sign-off) and `-S` (GPG sign) flags:
```bash
git commit -s -S -m "commit message"
```

- `-s`: Adds Signed-off-by line (DCO compliance)
- `-S`: GPG signature for authenticity verification

## Multi-Component Changes

For changes spanning multiple components, use the most significant scope and list all affected components in the body:

```
feat(workspace): implement comprehensive account domain alignment

Components affected:
- banking-api: Domain model consolidation
- banking-db: Enum-based type system
- banking-logic: Enhanced mappers and service compatibility
- banking-db-postgres: Schema updates with proper constraints

This change establishes consistent type safety across all layers of the banking system architecture.
```
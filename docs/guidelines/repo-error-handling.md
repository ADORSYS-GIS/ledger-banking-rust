# Repository Layer Error Handling Guidelines

## Overview

This document outlines the error handling patterns and best practices for the repository layer in the Ledger Banking Rust project. The repository layer uses domain-specific error types to provide meaningful context and improve debugging capabilities.

## Core Principles

### 1. Domain-Specific Error Types
- **Never expose raw database errors** directly to service layers
- **Create domain-specific error enums** that map database errors to business context
- **Use type aliases** for cleaner Result types in method signatures

### 2. Error Categories

Repository errors should be categorized into logical groups:

```rust
pub enum PersonDomainError {
    // Entity not found errors
    NotFound(String),
    
    // Validation and constraint errors
    InvalidHierarchy(String),
    DuplicateExternalId(String),
    OrganizationNotFound(Uuid),
    
    // Concurrency control errors
    StaleData {
        id: Uuid,
        expected_version: i32,
        actual_version: i32,
    },
    
    // Cascade operation errors
    CascadeDeleteBlocked(Vec<Uuid>),
    
    // Generic database errors (fallback)
    DatabaseError(String),
}
```

## Implementation Patterns

### 1. Error Mapping Functions

Each repository implementation should have a dedicated error mapping function:

```rust
impl PersonRepositoryImpl {
    fn map_sqlx_error(err: sqlx::Error, context: &str) -> PersonDomainError {
        match err {
            sqlx::Error::RowNotFound => {
                PersonDomainError::NotFound(context.to_string())
            }
            sqlx::Error::Database(db_err) => {
                // Check for specific constraint violations
                if let Some(constraint) = db_err.constraint() {
                    match constraint {
                        "person_external_id_unique" => {
                            PersonDomainError::DuplicateExternalId(context.to_string())
                        }
                        "person_organization_fk" => {
                            PersonDomainError::OrganizationNotFound(Uuid::nil())
                        }
                        _ => PersonDomainError::DatabaseError(db_err.to_string())
                    }
                } else {
                    PersonDomainError::DatabaseError(db_err.to_string())
                }
            }
            _ => PersonDomainError::DatabaseError(err.to_string())
        }
    }
}
```

### 2. Result Type Aliases

Define type aliases for cleaner method signatures:

```rust
pub type PersonResult<T> = Result<T, PersonDomainError>;
pub type LocationResult<T> = Result<T, LocationDomainError>;
pub type AuditResult<T> = Result<T, AuditDomainError>;
```

### 3. Method Error Documentation

Every repository method should document its possible error conditions:

```rust
/// Find person by ID
/// 
/// # Errors
/// - `PersonDomainError::NotFound` if person doesn't exist
async fn find_by_id(&self, id: Uuid) -> PersonResult<Person>;

/// Create a new person with validation
/// 
/// # Errors
/// - `PersonDomainError::InvalidHierarchy` if parent/organization relationship is invalid
/// - `PersonDomainError::DuplicateExternalId` if external_id already exists
/// - `PersonDomainError::OrganizationNotFound` if specified organization doesn't exist
async fn create(&self, person: &Person) -> PersonResult<Person>;
```

## Error Handling Best Practices

### 1. Early Validation
- Validate business rules before database operations
- Return specific errors for validation failures
- Use the cache layer for constraint checking when possible

```rust
// Validate hierarchy first
self.validate_hierarchy(person).await?;

// Check for duplicate external ID
if let Some(ref ext_id) = person.external_id {
    self.check_duplicate_external_id(ext_id, None).await?;
}

// Then perform the database operation
let result = sqlx::query_as::<_, Person>(...)
    .fetch_one(&self.pool)
    .await
    .map_err(|e| Self::map_sqlx_error(e, "Failed to create person"))?;
```

### 2. Contextual Error Messages
- Include relevant IDs and values in error messages
- Provide enough context to understand what failed
- Avoid exposing internal implementation details

```rust
PersonDomainError::NotFound(
    format!("Person with id {} not found", id)
)

PersonDomainError::DuplicateExternalId(
    format!("External ID '{}' is already in use", external_id)
)
```

### 3. Optimistic Locking Errors
- Check version numbers for concurrent updates
- Return specific error types for stale data
- Include both expected and actual versions

```rust
if current.version != person.version {
    return Err(PersonDomainError::StaleData {
        id: person.id,
        expected_version: person.version,
        actual_version: current.version,
    });
}
```

### 4. Cascade Operation Handling
- Check for dependent records before deletion
- Return lists of blocking IDs for cascade failures
- Provide clear information about what prevents the operation

```rust
let dependents: Vec<Uuid> = sqlx::query_scalar(
    "SELECT id FROM person WHERE parent_id = $1 LIMIT 10"
)
.bind(id)
.fetch_all(&self.pool)
.await?;

if !dependents.is_empty() {
    return Err(PersonDomainError::CascadeDeleteBlocked(dependents));
}
```

## Integration with Service Layer

### 1. Service Error Mapping
Services should map repository errors to their own error types:

```rust
impl From<PersonDomainError> for ServiceError {
    fn from(err: PersonDomainError) -> Self {
        match err {
            PersonDomainError::NotFound(msg) => {
                ServiceError::EntityNotFound(msg)
            }
            PersonDomainError::DuplicateExternalId(id) => {
                ServiceError::DuplicateResource(format!("External ID: {}", id))
            }
            PersonDomainError::StaleData { .. } => {
                ServiceError::ConcurrentModification
            }
            _ => ServiceError::InternalError(err.to_string())
        }
    }
}
```

### 2. Error Recovery Strategies
- Retry transient errors at the service layer
- Handle optimistic locking conflicts with retry logic
- Provide fallback behavior for non-critical operations

## Testing Error Conditions

### 1. Unit Tests for Error Mapping
```rust
#[test]
fn test_duplicate_key_error_mapping() {
    // Create a mock SQLx error with constraint violation
    let db_err = create_constraint_error("person_external_id_unique");
    let mapped = PersonRepositoryImpl::map_sqlx_error(
        sqlx::Error::Database(db_err),
        "test context"
    );
    
    assert!(matches!(
        mapped,
        PersonDomainError::DuplicateExternalId(_)
    ));
}
```

### 2. Integration Tests for Error Scenarios
```rust
#[tokio::test]
async fn test_cascade_delete_blocked() {
    let repo = create_test_repository().await;
    
    // Create parent and child persons
    let parent = create_test_person();
    let child = create_test_person()
        .with_parent_id(parent.id);
    
    repo.save(parent, audit_id).await.unwrap();
    repo.save(child, audit_id).await.unwrap();
    
    // Attempt to delete parent should fail
    let result = repo.delete(parent.id).await;
    
    assert!(matches!(
        result,
        Err(PersonDomainError::CascadeDeleteBlocked(deps)) 
        if deps.contains(&child.id)
    ));
}
```

## Error Monitoring and Logging

### 1. Structured Logging
```rust
match self.create(&person).await {
    Ok(p) => {
        tracing::info!(
            person_id = %p.id,
            "Person created successfully"
        );
        Ok(p)
    }
    Err(e) => {
        tracing::error!(
            error = %e,
            person_id = %person.id,
            "Failed to create person"
        );
        Err(e)
    }
}
```

### 2. Error Metrics
- Track error rates by error type
- Monitor specific constraint violations
- Alert on unusual error patterns

## Migration Strategy

When migrating existing repositories to use domain-specific errors:

1. **Create error types** in `banking-db/src/repository/errors.rs`
2. **Update repository traits** to use new Result types
3. **Implement error mapping** in repository implementations
4. **Update service layer** to handle new error types
5. **Add tests** for error conditions
6. **Update documentation** with error specifications

## References

- [`banking-db/src/repository/errors.rs`](../../banking-db/src/repository/errors.rs) - Error type definitions
- [`banking-db/src/repository/person_repository_enhanced.rs`](../../banking-db/src/repository/person_repository_enhanced.rs) - Example enhanced repository with domain errors
- [`banking-db-postgres/src/repository/person_person_repository_impl.rs`](../../banking-db-postgres/src/repository/person_person_repository_impl.rs) - Implementation example

## Summary

Domain-specific error handling in the repository layer provides:
- **Clear separation** between database and business errors
- **Better debugging** through contextual error messages
- **Type safety** preventing error type mixing
- **Improved maintainability** through centralized error logic
- **Enhanced testing** capabilities for error scenarios

Following these guidelines ensures consistent, meaningful error handling throughout the repository layer, improving both developer experience and system reliability.
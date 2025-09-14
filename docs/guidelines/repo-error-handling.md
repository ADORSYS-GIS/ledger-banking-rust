# Repository Layer Error Handling Guidelines

## Overview

This document outlines the error handling patterns and best practices for the repository layer in the Ledger Banking Rust project. The repository layer uses specific `RepositoryError` types for each domain to provide meaningful context, ensure type safety, and improve debugging capabilities.

## Core Principles

### 1. Domain-Specific Error Types
- **Never expose raw `sqlx::Error`** or other generic errors directly to service layers.
- **Create a specific error enum for each repository** (e.g., `PersonRepositoryError`, `LocationRepositoryError`) that maps database issues to business context.
- **Use type aliases** (e.g., `PersonResult<T>`) for cleaner `Result` types in method signatures.

### 2. Error Categories

Repository error enums should be structured to handle different categories of failures. A good `RepositoryError` enum includes variants for:
- **Not Found Errors**: For when a specific entity or a related dependency is not found.
- **Validation/Constraint Errors**: For business rule violations like duplicates or invalid foreign keys.
- **Generic Repository/Database Errors**: A fallback variant to wrap underlying database errors (e.g., `sqlx::Error`) or errors from other repositories.

```rust
// Example from banking-db/src/repository/person/location_repository.rs

pub enum LocationRepositoryError {
    /// Locality not found
    LocalityNotFound(Uuid),

    /// Invalid location type
    InvalidLocationType(String),

    /// Invalid coordinates
    InvalidCoordinates { latitude: f64, longitude: f64 },

    /// Duplicate location for same address
    DuplicateLocation {
        street: String,
        locality_id: Uuid,
    },

    /// Generic repository error, wrapping the source
    RepositoryError(Box<dyn Error + Send + Sync>),
}
```

## Implementation Patterns

### 1. Direct Error Mapping
For straightforward cases where a database operation can fail, use `.map_err()` to wrap the underlying error into your specific `RepositoryError` variant. This is the most common pattern.

```rust
// Example from banking-db-postgres/src/repository/person/location_repository_impl.rs

async fn load(&self, id: Uuid) -> LocationResult<LocationModel> {
    let query = sqlx::query("SELECT * FROM location WHERE id = $1").bind(id);

    let row = match &self.executor {
        Executor::Pool(pool) => query.fetch_one(&**pool).await
            .map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?,
        Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            query.fetch_one(&mut **tx).await
                .map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?
        }
    };

    LocationModel::try_from_row(&row)
        .map_err(|e| LocationRepositoryError::RepositoryError(e.into()))
}
```

### 2. Foreign Key and Pre-Condition Checks
Before performing an insert or update, validate the existence of foreign key dependencies. This provides clearer error messages than relying on database constraint violations.

```rust
// Example from banking-db-postgres/src/repository/person/person_repository_impl.rs

async fn save(&self, person: PersonModel, audit_log_id: Uuid) -> PersonResult<PersonModel> {
    if let Some(loc_id) = person.location_id {
        if !self.location_repository.exists_by_id(loc_id).await
            .map_err(|e| PersonRepositoryError::RepositoryError(e.into()))?
        {
            return Err(PersonRepositoryError::LocationNotFound(loc_id));
        }
    }
    // ... rest of the save logic
    Ok(person)
}
```

### 3. Result Type Aliases
Define type aliases in the repository trait file for cleaner method signatures.

```rust
// From banking-db/src/repository/person/person_repository.rs
pub type PersonResult<T> = Result<T, PersonRepositoryError>;

// From banking-db/src/repository/person/location_repository.rs
pub type LocationResult<T> = Result<T, LocationRepositoryError>;

// From banking-db/src/repository/person/messaging_repository.rs
pub type MessagingResult<T> = Result<T, MessagingRepositoryError>;

// From banking-db/src/repository/person/country_subdivision_repository.rs
pub type CountrySubdivisionResult<T> = Result<T, CountrySubdivisionRepositoryError>;
```

### 4. Method Error Documentation
Every repository method should document its possible error conditions in the trait definition.

```rust
/// Find person by ID.
///
/// # Errors
/// - `PersonRepositoryError::RepositoryError` if there is a database issue.
async fn load(&self, id: Uuid) -> PersonResult<PersonModel>;

/// Create a new person with validation.
///
/// # Errors
/// - `PersonRepositoryError::LocationNotFound` if the specified location doesn't exist.
/// - `PersonRepositoryError::OrganizationNotFound` if the specified organization doesn't exist.
async fn save(&self, person: PersonModel, audit_log_id: Uuid) -> PersonResult<PersonModel>;
```

## Integration with Service Layer

### Service Error Mapping
Services must map repository errors to their own service-specific error types. This mapping should be implemented within the service implementation file (e.g., in the `banking-logic` crate), not in the service trait definition (in the `banking-api` crate). This approach decouples the service layer from the data layer's error details and avoids creating cyclic dependencies between crates.

```rust
// Example from banking-logic/src/services/person/location_service_impl.rs

fn map_domain_error_to_service_error(error: LocationRepositoryError) -> LocationServiceError {
    match error {
        LocationRepositoryError::LocalityNotFound(id) => LocationServiceError::LocalityNotFound(id),
        LocationRepositoryError::InvalidLocationType(loc_type) => {
            LocationServiceError::InvalidLocationType(loc_type)
        }
        // ... other mappings
        LocationRepositoryError::RepositoryError(err) => LocationServiceError::RepositoryError(err),
    }
}

// Example from banking-logic/src/services/person/country_subdivision_service_impl.rs
fn map_domain_error_to_service_error(
    error: CountrySubdivisionRepositoryError,
) -> CountrySubdivisionServiceError {
    match error {
        CountrySubdivisionRepositoryError::CountryNotFound(id) => {
            CountrySubdivisionServiceError::CountryNotFound(id)
        }
        CountrySubdivisionRepositoryError::DuplicateCode { country_id, code } => {
            CountrySubdivisionServiceError::DuplicateCode { country_id, code }
        }
        CountrySubdivisionRepositoryError::RepositoryError(err) => {
            CountrySubdivisionServiceError::RepositoryError(err)
        }
    }
}
```

## Migration Strategy

When creating a new repository or refactoring an existing one:

1. **Define the error enum** (e.g., `<StructName>RepositoryError`) in the repository trait file (`banking-db/src/repository/person/<struct_name>_repository.rs`).
2. **Define the result type alias** (e.g., `pub type <StructName>Result<T> = Result<T, <StructName>RepositoryError>;`).
3. **Update repository trait methods** to use the new `Result` type.
4. **Implement error handling** in the repository implementation (`banking-db-postgres/src/repository/person/<struct_name>_repository_impl.rs`), mapping `sqlx::Error` and other errors to the new error type.
5. **Update the service layer** to handle the new repository error types and map them to service errors.
6. **Add tests** for the new error conditions.
7. **Update this documentation** with any new patterns or examples.

## References

- [`banking-db/src/repository/person/person_repository.rs`](../../banking-db/src/repository/person/person_repository.rs) - Trait and error definition example.
- [`banking-db-postgres/src/repository/person/person_repository_impl.rs`](../../banking-db-postgres/src/repository/person/person_repository_impl.rs) - Implementation example.
- [`banking-db/src/repository/person/location_repository.rs`](../../banking-db/src/repository/person/location_repository.rs) - Another trait and error definition example.
- [`banking-db-postgres/src/repository/person/location_repository_impl.rs`](../../banking-db-postgres/src/repository/person/location_repository_impl.rs) - Another implementation example.
- [`banking-db/src/repository/person/country_subdivision_repository.rs`](../../banking-db/src/repository/person/country_subdivision_repository.rs) - Another trait and error definition example.

## Summary

This domain-specific error handling approach provides:
- **Clear Separation**: Decouples database concerns from business logic.
- **Better Debugging**: Provides contextual, meaningful error messages.
- **Type Safety**: Prevents accidental propagation of raw database errors.
- **Improved Maintainability**: Centralizes error logic within each domain's repository.
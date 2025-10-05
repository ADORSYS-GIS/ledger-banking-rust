# Command: Refactor Domain Errors

**Objective**: Refactor the error handling for a service to improve modularity and align the service layer with the repository's domain-specific errors.

**Parameters**:
-   `<entity>`: The name of the service struct to refactor.

## Instructions

**Before you begin, you must read and follow the instructions outlined in `'docs/guidelines/repo-error-handling.md'`.**

Follow these steps precisely:

1.  **Define Domain-Specific Repository Errors** in `'banking-db/src/repository/person/<entity>_repository.rs'`.
2.  **Refactor Service-Level Errors** in `'banking-api/src/service/person/<entity>_service.rs'`.
3.  **Update Service Implementation** in `'banking-logic/src/services/person/<entity>_service_impl.rs'`.
4.  **Fix Affected Tests**.
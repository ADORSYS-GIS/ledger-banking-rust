Act as an expert Rust developer. Your task is to refactor the error handling for the `<StructName>` service to improve modularity and align the service layer with the repository's domain-specific errors.

**Before you begin, you must read and follow the instructions outlined in `'docs/guidelines/repo-error-handling.md'`.**

Follow these steps precisely:

1.  **Define Domain-Specific Repository Errors** in `'banking-db/src/repository/person/<struct_name>_repository.rs'`.
2.  **Refactor Service-Level Errors** in `'banking-api/src/service/person/<struct_name>_service.rs'`.
3.  **Update Service Implementation** in `'banking-logic/src/services/person/<struct_name>_service_impl.rs'`.
4.  **Fix Affected Tests**.
Act as an expert Rust developer. Your task is to refactor the error handling for the `<StructName>` service to improve modularity and align the service layer with the repository's domain-specific errors.

**Before you begin, you must read and follow the instructions outlined in `'docs/guidelines/repo-error-handling.md'`.**

Follow these steps precisely:

**1. Define Domain-Specific Repository Errors:**
   - In the repository trait file `'banking-db/src/repository/person/<struct_name>_repository.rs'`, define a comprehensive public enum named `<StructName>RepositoryError`.
   - Use the existing implementations in `person_repository.rs` and `location_repository.rs` as a reference for structure and style.
   - The enum should derive `Debug` and implement `std::fmt::Display` and `std::error::Error`.
   - Ensure the error variants are descriptive and cover all potential failures, including a `RepositoryError(Box<dyn Error + Send + Sync>)` variant to wrap underlying database or other repository errors.
   - Define a type alias: `pub type <StructName>Result<T> = Result<T, <StructName>RepositoryError>;`.

**2. Refactor Service-Level Errors in `'banking-api/src/service/person/<struct_name>_service.rs`:**
   - Define a new, public error enum named `<StructName>ServiceError`.
   - This enum must derive `thiserror::Error` and `Debug`.
   - Create variants in `<StructName>ServiceError` that semantically correspond to the variants in `<StructName>RepositoryError`.
   - Implement `From<<StructName>RepositoryError> for <StructName>ServiceError` to enable clean and automatic conversion.
   - Update all method signatures within the `<StructName>Service` trait to use the new error type in their return values, changing them to `Result<T, <StructName>ServiceError>`.

**3. Update Service Implementation in `'banking-logic/src/services/person/<struct_name>_service_impl.rs`:**
   - Modify the `impl <StructName>Service for <StructName>ServiceImpl` block to match the updated trait definition.
   - Within each method implementation, update the error handling logic. Use `map_err` or the `?` operator to transparently convert `<StructName>RepositoryError` instances into `<StructName>ServiceError` instances. Create a dedicated `map_domain_error_to_service_error` function if the mapping is complex.

**4. Fix Affected Tests:**
   - Search the entire codebase for unit and integration tests that interact with the `<StructName>Service`.
   - Update these tests to reflect the changes in the service methods' return types.
   - Modify assertions to correctly match against the new `<StructName>ServiceError` variants. Ensure all test cases, especially those for error conditions, pass successfully.
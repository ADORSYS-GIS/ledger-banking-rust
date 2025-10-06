# Module to Entity Layout Refactoring Guide

This guide outlines the process of refactoring a module to follow the entity-based file layout, using the `audit` module as an example. The goal is to organize files by entity, improving modularity and making the codebase easier to navigate.

## Refactoring Steps

The following steps will guide you through the refactoring process for the `audit` module across all relevant crates.

### 1. `banking-db`

- **Create the module directory**:
  ```bash
  mkdir -p banking-db/src/models/audit
  ```
- **Move the entity file**:
  ```bash
  mv banking-db/src/models/audit.rs banking-db/src/models/audit/mod.rs
  ```
- **Create the entity file**:
  ```bash
  touch banking-db/src/models/audit/audit_log.rs
  ```
- **Update `banking-db/src/models/audit/mod.rs`**:
  ```rust
  pub mod audit_log;
  pub use audit_log::*;
  ```
- **Update `banking-db/src/models/mod.rs`**:
  - Ensure the `audit` module is correctly declared:
    ```rust
    pub mod audit;
    pub use audit::*;
    ```

### 2. `banking-db-postgres`

- **Create the repository directory**:
  ```bash
  mkdir -p banking-db-postgres/src/repository/audit/audit_log_repository
  ```
- **Create the repository implementation file**:
  ```bash
  touch banking-db-postgres/src/repository/audit/audit_log_repository/repo_impl.rs
  ```
- **Create the module file**:
  ```bash
  touch banking-db-postgres/src/repository/audit/audit_log_repository/mod.rs
  ```
- **Update `banking-db-postgres/src/repository/audit/audit_log_repository/mod.rs`**:
  ```rust
  pub mod repo_impl;
  pub use repo_impl::*;
  ```
- **Update `banking-db-postgres/src/repository/audit/mod.rs`**:
  ```rust
  pub mod audit_log_repository;
  ```

### 3. `banking-logic`

- **Create the mappers directory**:
  ```bash
  mkdir -p banking-logic/src/mappers/audit
  ```
- **Move the mapper file**:
  ```bash
  mv banking-logic/src/mappers/audit_mapper.rs banking-logic/src/mappers/audit/audit_log_mapper.rs
  ```
- **Create the module file**:
  ```bash
  touch banking-logic/src/mappers/audit/mod.rs
  ```
- **Update `banking-logic/src/mappers/audit/mod.rs`**:
  ```rust
  pub mod audit_log_mapper;
  ```
- **Create the services directory**:
  ```bash
  mkdir -p banking-logic/src/services/audit
  ```
- **Move the service file**:
  ```bash
  mv banking-logic/src/services/audit_service_impl.rs banking-logic/src/services/audit/audit_log_service_impl.rs
  ```
- **Create the module file**:
  ```bash
  touch banking-logic/src/services/audit/mod.rs
  ```
- **Update `banking-logic/src/services/audit/mod.rs`**:
  ```rust
  pub mod audit_log_service_impl;
  ```

### 4. `banking-api`

- **Create the domain directory**:
  ```bash
  mkdir -p banking-api/src/domain/audit
  ```
- **Move the domain file**:
  ```bash
  mv banking-api/src/domain/audit.rs banking-api/src/domain/audit/audit_log.rs
  ```
- **Create the module file**:
  ```bash
  touch banking-api/src/domain/audit/mod.rs
  ```
- **Update `banking-api/src/domain/audit/mod.rs`**:
  ```rust
  pub mod audit_log;
  pub use audit_log::*;
  ```
- **Create the service directory**:
  ```bash
  mkdir -p banking-api/src/service/audit
  ```
- **Move the service file**:
  ```bash
  mv banking-api/src/service/audit_service.rs banking-api/src/service/audit/audit_log_service.rs
  ```
- **Create the module file**:
  ```bash
  touch banking-api/src/service/audit/mod.rs
  ```
- **Update `banking-api/src/service/audit/mod.rs`**:
  ```rust
  pub mod audit_log_service;
  ```

## Automation Script

To automate the file and directory creation, you can use the following shell script.

```bash
#!/bin/bash

MODULE_NAME="audit"
ENTITY_NAME="audit_log"

# banking-db
mkdir -p banking-db/src/models/$MODULE_NAME
mv banking-db/src/models/$MODULE_NAME.rs banking-db/src/models/$MODULE_NAME/mod.rs
echo "pub mod $ENTITY_NAME; pub use $ENTITY_NAME::*;" > banking-db/src/models/$MODULE_NAME/mod.rs
touch banking-db/src/models/$MODULE_NAME/$ENTITY_NAME.rs

# banking-db-postgres
mkdir -p banking-db-postgres/src/repository/$MODULE_NAME/${ENTITY_NAME}_repository
echo "pub mod repo_impl; pub use repo_impl::*;" > banking-db-postgres/src/repository/$MODULE_NAME/${ENTITY_NAME}_repository/mod.rs
touch banking-db-postgres/src/repository/$MODULE_NAME/${ENTITY_NAME}_repository/repo_impl.rs
echo "pub mod ${ENTITY_NAME}_repository;" > banking-db-postgres/src/repository/$MODULE_NAME/mod.rs

# banking-logic
mkdir -p banking-logic/src/mappers/$MODULE_NAME
mv banking-logic/src/mappers/${MODULE_NAME}_mapper.rs banking-logic/src/mappers/$MODULE_NAME/${ENTITY_NAME}_mapper.rs
echo "pub mod ${ENTITY_NAME}_mapper;" > banking-logic/src/mappers/$MODULE_NAME/mod.rs

mkdir -p banking-logic/src/services/$MODULE_NAME
mv banking-logic/src/services/${MODULE_NAME}_service_impl.rs banking-logic/src/services/$MODULE_NAME/${ENTITY_NAME}_service_impl.rs
echo "pub mod ${ENTITY_NAME}_service_impl;" > banking-logic/src/services/$MODULE_NAME/mod.rs

# banking-api
mkdir -p banking-api/src/domain/$MODULE_NAME
mv banking-api/src/domain/$MODULE_NAME.rs banking-api/src/domain/$MODULE_NAME/$ENTITY_NAME.rs
echo "pub mod $ENTITY_NAME; pub use $ENTITY_NAME::*;" > banking-api/src/domain/$MODULE_NAME/mod.rs

mkdir -p banking-api/src/service/$MODULE_NAME
mv banking-api/src/service/${MODULE_NAME}_service.rs banking-api/src/service/$MODULE_NAME/${ENTITY_NAME}_service.rs
echo "pub mod ${ENTITY_NAME}_service;" > banking-api/src/service/$MODULE_NAME/mod.rs

echo "Refactoring for module '$MODULE_NAME' is complete."
```

### How to Use the Script

1.  Save the script to a file named `refactor_module.sh`.
2.  Make the script executable: `chmod +x refactor_module.sh`.
3.  Run the script from the root of the project: `./refactor_module.sh`.

This will perform all the necessary file and directory operations for the `audit` module. You will still need to manually update the contents of the moved files to reflect the new module structure.
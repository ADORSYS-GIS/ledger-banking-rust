# Guideline: Repository with In-Memory Index Cache

This document provides a guideline for implementing a repository that uses an in-memory index cache for optimized query performance. This pattern is particularly useful for frequently accessed data where read performance is critical.

## 1. Overview

The core idea is to maintain a lightweight, in-memory cache of an "index model" (`*_idx`) that contains the primary key, version, a hash of the full model, and any other frequently queried fields (or their hashes). This cache is used to quickly satisfy finder methods and to optimize the `save` operation by avoiding unnecessary database reads.

## 2. Key Components

### 2.1. Data Models (`banking-db/src/models/`)

- **`Model`**: The main data model. It should **not** contain a `version` or `hash` field.
- **`ModelAudit`**: The audit model. It **must** contain `version` and `hash` fields.
- **`IdxModel`**: The index model. It **must** contain `version` and `hash` fields, along with the primary key and any other indexed fields.

### 2.2. Repository Trait (`banking-db/src/repository/`)

- **`load(id: Uuid) -> Result<Model, ...>`**: This is the *only* method that should return the full `Model`. It fetches the complete record from the database by its primary key.
- **`find_by_*` methods**: These methods should return `Option<IdxModel>` or `Vec<IdxModel>`. They should query the in-memory cache, not the database.
- **`exists_by_*` methods**: These should also query the cache and return a `bool`.
- **`save(model: Model) -> Result<Model, ...>`**: This method handles both creation and updates of records.

### 2.3. Repository Implementation (`banking-db-postgres/src/repository/`)

- **`struct RepositoryImpl`**: Must contain a `pool: Arc<PgPool>` and a `cache: Arc<RwLock<IdxModelCache>>`.
- **`new()`**: The constructor should pre-load all index records from the `*_idx` table into the cache upon instantiation.
- **`save()` Implementation**:
    - **Calculate Hashes**: Compute a hash of the incoming `Model` object (e.g., by serializing it to JSON and hashing the string). Also compute hashes for any other indexed fields.
    - **Check Cache First**: Before any database operation, check the cache to see if the record exists.
    - **Update Logic**:
        1.  If the record exists in the cache, it's an update.
        2.  Compare the hash of the incoming model with the hash stored in the cached `IdxModel`. If they are the same, no changes are needed, and you can return immediately.
        3.  If the hashes differ, increment the `version` from the cached `IdxModel`.
        4.  Create a `ModelAudit` record (with the new version and hash) and insert it into the `*_audit` table.
        5.  Perform a SQL `UPDATE` on the main table (note: the main table does not have a version or hash column).
        6.  Perform a SQL `UPDATE` on the `*_idx` table to persist the new version, hash, and any other indexed fields.
        7.  Create a new `IdxModel` with the updated information and update the cache.
    - **Insert Logic**:
        1.  If the record does not exist in the cache, it's a new record.
        2.  The `version` is `0`.
        3.  Create and insert a `ModelAudit` record (with version 0 and the new hash).
        4.  Perform a SQL `INSERT` into the main table.
        5.  Create and insert a new `IdxModel` record into the `*_idx` table.
        6.  Add the new `IdxModel` to the in-memory cache.
- **Finder Implementations (`find_by_*`, `exists_by_*`)**: These methods should *only* interact with the `RwLock` on the cache to perform read operations. They should not touch the database.

### 2.4. Thread Safety

- **Scoping Locks**: `RwLockReadGuard` and `RwLockWriteGuard` must be dropped *before* any `.await` calls to avoid `Send` trait compilation errors. Use block scopes to manage the lifetime of the guards.

```rust
// Correctly scoped read lock
let maybe_existing_idx = {
    let cache_read_guard = self.person_idx_cache.read().unwrap();
    cache_read_guard.get_by_primary(&person.id)
}; // guard is dropped here

// .await call is safe now
if let Some(existing_idx) = maybe_existing_idx {
    // ...
}
```

## 3. Example Flow: `save()` Method

1.  Calculate the hash of the incoming `Model`.
2.  Acquire a read lock on the cache to check if an `IdxModel` exists for the given ID. **Drop the lock immediately after the check.**
3.  **If `IdxModel` exists**:
    a. Compare the new hash with the hash in the cached `IdxModel`. If they match, return.
    b. Increment version from the cached `IdxModel`.
    c. `await` the insertion of a new `ModelAudit` record (with the new version and hash).
    d. `await` the `UPDATE` of the `Model` in the database.
    e. `await` the `UPDATE` of the `IdxModel` in the `*_idx` table.
    f. Acquire a write lock on the cache and update the `IdxModel` with the new version and hash.
4.  **If `IdxModel` does not exist**:
    a. `await` the insertion of a new `ModelAudit` record (with version 0 and the new hash).
    b. `await` the `INSERT` of the new `Model` into the database.
    c. `await` the `INSERT` of the new `IdxModel` into the `*_idx` table.
    d. Acquire a write lock on the cache and add the new `IdxModel`.
5.  Return the saved `Model`.
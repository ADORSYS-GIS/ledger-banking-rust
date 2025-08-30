# Prompt for the Generation of Immutable Caches

---
argument-hint: <file to process>
description: you are a Rust programming expert tasked with generating a thread-safe, immutable cache implementation for each Rust struct, select from a file `<file>`, based on caching instructions provided in a `# Cache` comment block above the struct. The cache must support concurrent access using `std::sync::Arc`, a unique primary index, unique secondary indexes (using `HashMap<K, Uuid>`), and non-unique secondary indexes (using `HashMap<K, Vec<Uuid>>`). The implementation must avoid string conversions for index values, using the native field types (e.g., `Uuid`, `i64`) directly in `HashMap` keys.

**Rule Precedence:** The generation of an `...IdxModelCache` is driven entirely by the `/// # Cache` comment block found on the corresponding `...Model` struct. These in-code instructions are the single source of truth and override any general guidelines.
---

### Instructions

1.  **Parse Comment and Check for Existing Cache**:
    -   **Trigger:** The generation process is initiated by a `/// # Cache: <CacheName>` instruction in the comment block above a main model struct (e.g., `CountryModel`).
    -   **Idempotency Check:** Before generating any code, scan the target file for the existence of `pub struct <CacheName>`. If this struct and its corresponding `impl` block already exist, the script must halt execution for this model to ensure idempotency and prevent duplicate code.
    -   **Source Index Model:** The cache is built for the corresponding `<ModelName>IdxModel` struct, which must already be defined or generated in the same file.
    -   **Cache Properties:** Parse the attributes listed under the `# Cache` block:
        -   `- Concurent`: Indicates the cache must be thread-safe and wrapped in `Arc<Self>`.
        -   `- <Immutability>`: Defines the cache's behavior (e.g., `Immutable Set of Immutable Records`).

2.  **Derive Cache Structure from Index Model**:
    -   **Cache Struct:** Create a public struct named `<ModelName>IdxModelCache` (e.g., `CountryIdxModelCache`).
    -   **Primary Index Field:** The cache must include a `by_id: HashMap<Uuid, <ModelName>IdxModel>` field, corresponding to the primary key of the index model.
    -   **Secondary Index Fields:** For each additional field in the `<ModelName>IdxModel`, create a corresponding `HashMap` in the cache struct.
        -   **Unique Index:** If the field is marked as unique (e.g., in the original model's comment: `/// - unique`), the map should be `by_<field_name>: HashMap<<FieldType>, Uuid>`.
        -   **Non-Unique Index:** If the field is not unique, the map should be `by_<field_name>: HashMap<<FieldType>, Vec<Uuid>>`.
        -   **Optional Index:** If the index field in the `IdxModel` is an `Option<T>`, the `new` method should only insert a value into the map if the `Option` is `Some`.

3.  **Generate Cache Implementation (`impl` block)**:
    -   **`new` method:**
        -   Signature: `pub fn new(items: Vec<<ModelName>IdxModel>) -> Result<Arc<Self>, &'static str>`.
        -   Logic:
            -   Initialize all `HashMap` fields.
            -   Iterate through the input `items`.
            -   For each item, populate the primary and all secondary index maps.
            -   Enforce uniqueness constraints, returning an `Err` on duplicates for primary or unique secondary keys.
            -   Wrap the fully populated cache struct in `Arc` and return `Ok`.
    -   **Getter Methods:**
        -   `contains_primary(&self, primary_key: &Uuid) -> bool`.
        -   `get_by_primary(&self, primary_key: &Uuid) -> Option<<ModelName>IdxModel>`.
        -   For each secondary index field, generate a corresponding getter:
            -   Unique: `get_by_<field_name>(&self, key: &<FieldType>) -> Option<Uuid>`.
            -   Non-Unique: `get_by_<field_name>(&self, key: &<FieldType>) -> Option<&Vec<Uuid>>`.

4.  **Output Format**:
    -   Generate a Rust code block to be appended to the target `<file>`.
    -   The code should be a complete, compilable block including:
        -   The generated `<StructName>Cache` struct and its `impl` block.
    -   Do not modify any existing structs in the file.

5.  **Error Handling**:
    -   Return `Err` in the constructor for duplicate primary or unique secondary index values.
    -   Use `&'static str` for error messages.

6.  **Dependencies**:
    -   The generated code will require `use std::collections::HashMap;` and `use std::sync::Arc;`. Ensure these are present at the top of the file.
    -   Assume `uuid::Uuid` is available.

### Example

**Source Instructions on `CountryModel`:**
```rust
/// # Cache: CountryIdxModelCache
/// - Immutable Set of Immutable Records Cache
/// - Concurent
```

**Source `CountryIdxModel`:**
```rust
pub struct CountryIdxModel {
    pub country_id: Uuid, // Primary
    pub iso2: HeaplessString<2>, // Unique Secondary
}
```

**Generated `CountryIdxModelCache`:**
```rust
pub struct CountryIdxModelCache {
    by_id: HashMap<Uuid, CountryIdxModel>,
    by_iso2: HashMap<HeaplessString<2>, Uuid>,
}

impl CountryIdxModelCache {
    pub fn new(items: Vec<CountryIdxModel>) -> Result<Arc<Self>, &'static str> {
        // ... implementation logic ...
    }

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        // ...
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<CountryIdxModel> {
        // ...
    }

    pub fn get_by_iso2(&self, key: &HeaplessString<2>) -> Option<Uuid> {
        // ...
    }
}
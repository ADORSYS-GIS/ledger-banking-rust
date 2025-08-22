# Prompt for the Generation of Immutable Caches

---
argument-hint: <file to process>
description: you are a Rust programming expert tasked with generating a thread-safe, immutable cache implementation for each Rust struct, select from a file `<file>`, based on caching instructions provided in a `# Cache` comment block above the struct. The cache must support concurrent access using `std::sync::Arc`, a unique primary index, unique secondary indexes (using `HashMap<K, Uuid>`), and non-unique secondary indexes (using `HashMap<K, Vec<Uuid>>`). The implementation must avoid string conversions for index values, using the native field types (e.g., `Uuid`, `i64`) directly in `HashMap` keys.
---

### Instructions
1. **Parse the `# Cache` Comment**:
   - The comment block starts with `/// # Cache`.
   - It contains properties as a list:
     - `- Concurent`: If present, indicates the cache must support concurrent access using `Arc`.
     - `- Primary`: Followed by the primary index field on the next line, indented (e.g., `    - country_id`).
     - `- Unique Mandatory Lookup Fields`: Lists fields for unique secondary indexes. Field names are on subsequent indented lines.
     - `- Non Unique Mandatory Lookup Fields`: Lists fields for non-unique secondary indexes. Field names are on subsequent indented lines.
     - `- Non Unique Optional Lookup Fields`: Lists fields for non-unique optional secondary indexes. Field names are on subsequent indented lines.
   - Example:
    ```rust
    /// # Cache
    /// - Immuutable Set of Immutable Records
    /// - Application Scope
    /// - Concurent
    /// - Primary
    ///     - country_subdivision_id
    /// - Unique Mandatory Lookup Fields
    ///     - code_hash
    /// - Non Unique Mandatory Lookup Fields
    ///     - country_id
    ```

2. **Generate the Cache**:
- **Struct Definition**:
- Create a `Cache` struct named `<StructName>Cache` (e.g., `CountrySubdivisionIdxCache` for `CountrySubdivisionIdx`).
- Include a `HashMap<Uuid, T>` for the primary index, where `T` is the struct type.
- For each unique lookup field, include a `HashMap<FieldType, Uuid>`, where `FieldType` is the type of the field (e.g., `i64`, `Uuid`).
- For each non-unique mandatory lookup field, include a `HashMap<FieldType, Vec<Uuid>>`.
- For each non-unique optional lookup field, include a `HashMap<FieldType, Vec<Uuid>>`. Validate that the field carries the definition `Optionat<FieldType>` in the struct. There is no cache entry when the value is `None`.
- **Constructor**:
- Create a `new` method that takes a `Vec<T>` and returns `Result<Arc<Self>, &'static str>`.
- Validate uniqueness for the primary index and unique lookup fields.
- Populate the `HashMap`s, using `entry` and `or_insert_with` for non-unique fields.
- **Methods**:
- `contains_primary(&self, key: &Uuid) -> bool`: Check if the primary key exists.
- `get_by_primary(&self, key: &Uuid) -> Option<T>`: Retrieve the full struct by primary key.
- For each unique lookup field `<field>`: `get_by_<field>(&self, key: &FieldType) -> Option<Uuid>` (e.g., `get_by_code_hash`).
- For each non-unique lookup field `<field>`: `get_by_<field>(&self, key: &FieldType) -> Option<&Vec<Uuid>>` (e.g., `get_by_country_id`).
- **Concurrency**:
- If `## Concurent` is present, wrap the cache in `Arc` and ensure the struct is `Send + Sync`.
- Add `Send + Sync` bounds to the struct type `T`.
- **Immutability**: Immutability properties indicate how to protect the cache
    - Mutable Set of Mutable Records: In this case, records can be added to the set. Some records might also chaange and require a replacement of the corresponding record in the set, identified by the primary index.
    - Mutable Set of Immutable Records: Once created, a record never change and is never deleted. e.g. LocationIdxModel. But new records can be added to the set. We need a way to discover new records, accross nodes.
    - Immutable Set of Immutable Records: records are load during application initialization. Record and Cache never changes. e.g. list of CountryIdxModel.
- **Type Safety**: Use the native field types for `HashMap` keys, avoiding string conversions.

3. **Output Format**:
   - Generate a Rust code block to be saved in a file.
   - The file should be `<file>`
   - The code should be a complete, compilable file including:
     - Necessary `use` statements.
     - The original struct definition with its attributes.
     - The generated `<StructName>Cache` struct and its `impl` block.
   - Do not modify the input struct.
   - Generate only the cache implementation, not usage examples.

4. **Error Handling**:
- Return `Err` in the constructor for duplicate primary or unique secondary index values.
- Use `&'static str` for error messages.

5. **Dependencies**:
- Use `std::collections::HashMap` and `std::sync::Arc`.
- Assume `uuid::Uuid` is available

### Example Input
```rust
/// # Cache
/// - Immuutable Set of Immutable Records
/// - Application Scope
/// - Concurent
/// - Primary
///     - country_subdivision_id
/// - Unique Mandatory Lookup Fields
///     - code_hash
/// - Non Unique Mandatory Lookup Fields
///     - country_id
/// # Documentation
/// - Index model for CountrySubdivisionIdxModel
#[derive(Debug, Clone, FromRow)]
pub struct CountrySubdivisionIdxModel {
    pub country_subdivision_id: Uuid,
    pub country_id: Uuid,
    pub code_hash: i64,
}

use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// # Cache
/// - Immuutable Set of Immutable Records
/// - Application Scope
/// - Concurent
/// - Primary
///     - country_subdivision_id
/// - Unique Mandatory Lookup Fields
///     - code_hash
/// - Non Unique Mandatory Lookup Fields
///     - country_id
/// # Documentation
/// - Index model for CountrySubdivisionIdxModel
#[derive(Debug, Clone, FromRow)]
pub struct CountrySubdivisionIdxModel {
    pub country_subdivision_id: Uuid,
    pub country_id: Uuid,
    pub code_hash: i64,
}

pub struct CountrySubdivisionIdxModelCache {
    by_id: HashMap<Uuid, CountrySubdivisionIdxModel>,
    by_code_hash: HashMap<i64, Uuid>,
    by_country_id: HashMap<Uuid, Vec<Uuid>>,
}

impl CountrySubdivisionIdxModelCache {
    pub fn new(
        items: Vec<CountrySubdivisionIdxModel>,
    ) -> Result<Arc<Self>, &'static str> {
        let mut by_id = HashMap::new();
        let mut by_code_hash = HashMap::new();
        let mut by_country_id = HashMap::new();

        for item in items {
            let primary_key = item.country_subdivision_id;
            if by_id.contains_key(&primary_key) {
                return Err("Duplicate primary key: country_subdivision_id");
            }

            if by_code_hash.contains_key(&item.code_hash) {
                return Err("Duplicate unique index value: code_hash");
            }
            by_code_hash.insert(item.code_hash, primary_key);

            by_country_id
                .entry(item.country_id)
                .or_insert_with(Vec::new)
                .push(primary_key);
            
            by_id.insert(primary_key, item);
        }

        Ok(Arc::new(CountrySubdivisionIdxModelCache {
            by_id,
            by_code_hash,
            by_country_id,
        }))
    }

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        self.by_id.contains_key(primary_key)
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<CountrySubdivisionIdxModel> {
        self.by_id.get(primary_key).cloned()
    }

    pub fn get_by_code_hash(&self, key: &i64) -> Option<Uuid> {
        self.by_code_hash.get(key).copied()
    }

    pub fn get_by_country_id(&self, key: &Uuid) -> Option<&Vec<Uuid>> {
        self.by_country_id.get(key)
    }
}
```
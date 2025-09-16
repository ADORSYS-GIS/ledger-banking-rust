use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use std::collections::HashMap;

/// # Repository Trait
/// - FQN: banking-db/src/repository/person/country_subdivision_repository.rs/CountrySubdivisionRepository
/// 
/// # Index: CountrySubdivisionIdxModel
/// ## Repository Trait
/// - FQN: banking-db/src/repository/person/country_subdivision_repository.rs/CountrySubdivisionRepository
/// ## Trait method
/// - create_idx
/// - load_idxes
/// ## Pg Trigger
/// - CREATE
/// ## Cache: CountrySubdivisionIdxModelCache
/// - Immutable Set of Immutable Records Cache
/// - Concurent
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CountrySubdivisionModel {
    /// # Trait methods
    /// - find_by_id
    /// - find_by_ids
    /// - exists_by_id
    /// 
    /// # Index: country_subdivision_id
    /// ## Nature
    /// - primary
    pub id: Uuid,
    /// # Trait method
    /// - find_ids_by_country_id
    /// - find_by_country_id
    /// 
    /// # Index
    /// ## Nature
    /// - secondary
    /// ## Constraint
    /// - exists(CountryModel.id)
    pub country_id: Uuid,

    /// # Documentation
    /// - if non existant the first 10 chars of the name_l1
    /// # Trait method
    /// - find_by_code
    ///     - code: self.code
    /// 
    /// # Index: code_hash: i64
    /// ## Nature
    /// - secondary
    /// - unique
    pub code: HeaplessString<10>,
    pub name_l1: HeaplessString<100>,
    pub name_l2: Option<HeaplessString<100>>,
    pub name_l3: Option<HeaplessString<100>>,
}

/// # Repository Trait
/// - FQN: banking-db/src/repository/person/country_subdivision_repository.rs/CountrySubdivisionRepository
/// # Trait method
/// - create_idx
/// - load_idxes
/// # Pg Trigger
/// - CREATE
/// # Cache: CountrySubdivisionIdxModelCache
/// - Immutable Set of Immutable Records Cache
/// - Concurent
#[derive(Debug, Clone, FromRow)]
pub struct CountrySubdivisionIdxModel {
    /// # Nature
    /// - primary
    pub country_subdivision_id: Uuid,

    /// # Nature
    /// - secondary
    pub country_id: Uuid,

    /// # Nature
    /// - secondary
    /// - unique
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
    ) -> Result<Self, &'static str> {
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

        Ok(CountrySubdivisionIdxModelCache {
            by_id,
            by_code_hash,
            by_country_id,
        })
    }

    pub fn add(&mut self, item: CountrySubdivisionIdxModel) {
        let primary_key = item.country_subdivision_id;
        if self.by_id.contains_key(&primary_key) {
            return;
        }

        self.by_code_hash.insert(item.code_hash, primary_key);
        self.by_country_id
            .entry(item.country_id)
            .or_default()
            .push(primary_key);
        self.by_id.insert(primary_key, item);
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
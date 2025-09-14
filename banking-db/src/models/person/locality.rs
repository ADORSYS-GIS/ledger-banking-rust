use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use std::collections::HashMap;

/// # Repository Trait
/// - FQN: banking-db/src/repository/person/locality_repository.rs/LocalityRepository
/// 
/// # Index: LocalityIdxModel
/// ## Repository Trait
/// - FQN: banking-db/src/repository/person/locality_repository.rs/LocalityRepository
/// ## Trait method
/// - create_idx
/// - load_idxes
/// ## Pg Trigger
/// - CREATE
/// ## Cache: LocalityIdxModelCache
/// - Mutable Set of Immutable Records
/// - Concurent
/// 
/// # Documentation
/// - Database model for Locality
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LocalityModel {
    /// # Trait methods
    /// - find_by_id
    /// - find_by_ids
    /// - exists_by_id
    /// 
    /// # Index: locality_id
    /// ## Nature
    /// - primary
    pub id: Uuid,

    /// # Trait method
    /// - find_ids_by_country_subdivision_id
    /// - find_by_country_subdivision_id
    /// 
    /// # Index
    /// ## Nature
    /// - secondary
    /// ## Constraint
    /// - exists(CountrySubdivisionModel.id)
    pub country_subdivision_id: Uuid,

    /// # Documentation
    /// - If non existant, country subdivision code '_' the first 10 chars of the name_l1
    /// 
    /// # Trait method
    /// - find_by_code
    ///     - code: self.code
    /// 
    /// # Index: code_hash: i64
    /// ## Nature
    /// - secondary
    /// - unique
    pub code: HeaplessString<50>,
    pub name_l1: HeaplessString<50>,
    pub name_l2: Option<HeaplessString<50>>,
    pub name_l3: Option<HeaplessString<50>>,
}

/// # Repository Trait
/// - FQN: banking-db/src/repository/person/locality_repository.rs/LocalityRepository
/// # Trait method
/// - create_idx
/// - load_idxes
/// # Pg Trigger
/// - CREATE
/// # Cache: LocationIdxModelCache
/// - Mutable Set of Immutable Records
/// - Concurent
#[derive(Debug, Clone, FromRow)]
pub struct LocalityIdxModel {
    /// # Nature
    /// - primary
    pub locality_id: Uuid,
    /// # Nature
    /// - secondary
    pub country_subdivision_id: Uuid,
    /// # Nature
    /// - secondary
    /// - unique
    pub code_hash: i64,
}

pub struct LocalityIdxModelCache {
    by_id: HashMap<Uuid, LocalityIdxModel>,
    by_code_hash: HashMap<i64, Uuid>,
    by_country_subdivision_id: HashMap<Uuid, Vec<Uuid>>,
}

impl LocalityIdxModelCache {
    pub fn new(items: Vec<LocalityIdxModel>) -> Result<Self, &'static str> {
        let mut by_id = HashMap::new();
        let mut by_code_hash = HashMap::new();
        let mut by_country_subdivision_id = HashMap::new();

        for item in items {
            let primary_key = item.locality_id;
            if by_id.contains_key(&primary_key) {
                return Err("Duplicate primary key: locality_id");
            }

            if by_code_hash.contains_key(&item.code_hash) {
                return Err("Duplicate unique index value: code_hash");
            }
            by_code_hash.insert(item.code_hash, primary_key);

            by_country_subdivision_id
                .entry(item.country_subdivision_id)
                .or_insert_with(Vec::new)
                .push(primary_key);

            by_id.insert(primary_key, item);
        }

        Ok(LocalityIdxModelCache {
            by_id,
            by_code_hash,
            by_country_subdivision_id,
        })
    }

    pub fn add(&mut self, item: LocalityIdxModel) {
        let primary_key = item.locality_id;
        if self.by_id.contains_key(&primary_key) {
            return;
        }

        self.by_code_hash.insert(item.code_hash, primary_key);
        self.by_country_subdivision_id
            .entry(item.country_subdivision_id)
            .or_default()
            .push(primary_key);
        self.by_id.insert(primary_key, item);
    }

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        self.by_id.contains_key(primary_key)
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<LocalityIdxModel> {
        self.by_id.get(primary_key).cloned()
    }

    pub fn get_by_code_hash(&self, key: &i64) -> Option<Uuid> {
        self.by_code_hash.get(key).copied()
    }

    pub fn get_by_country_subdivision_id(&self, key: &Uuid) -> Option<&Vec<Uuid>> {
        self.by_country_subdivision_id.get(key)
    }
}
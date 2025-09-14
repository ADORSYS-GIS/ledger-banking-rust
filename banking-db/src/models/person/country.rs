use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use std::collections::HashMap;

/// # Repository Trait
/// - FQN: banking-db/src/repository/person/country_repository.rs/CountryRepository
/// 
/// # Index: CountryIdxModel
/// ## Repository Trait
/// - FQN: banking-db/src/repository/person/country_repository.rs/CountryRepository
/// ## Trait method
/// - create_idx
/// - load_idxes
/// ## Pg Trigger
/// - CREATE
/// ## Cache: CountryIdxModelCache
/// - Immutable Set of Immutable Records Cache
/// - Concurent
/// 
/// # Documentation
/// - Country structure with ISO 3166-1 alpha-2 code
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CountryModel {
    /// # Trait method
    /// - find_by_id
    /// - find_by_ids
    /// - exists_by_id
    /// 
    /// # Index: country_id
    /// ## Nature
    /// - primary
    pub id: Uuid,
    
    /// # Documentation
    /// - ISO 3166-1 alpha-2 country code (e.g., "CM", "US", "GB")
    /// # Trait method
    /// - find_ids_by_iso2
    /// - find_by_iso2
    /// 
    /// # Index: iso2: HeaplessString<2>
    /// ## Nature
    /// - secondary
    /// - unique
    pub iso2: HeaplessString<2>,

    pub name_l1: HeaplessString<100>,
    pub name_l2: Option<HeaplessString<100>>,
    pub name_l3: Option<HeaplessString<100>>,
}

/// # Repository Trait
/// - FQN: banking-db/src/repository/person/country_repository.rs/CountryRepository
/// # Trait method
/// - create_idx
/// - load_idxes
/// # Pg Trigger
/// - CREATE
/// # Cache: CountryIdxModelCache
/// - Immutable Set of Immutable Records Cache
/// - Concurent
#[derive(Debug, Clone, FromRow)]
pub struct CountryIdxModel {
    /// # Nature
    /// - primary
    pub country_id: Uuid,

    /// # Nature
    /// - secondary
    /// - unique
    pub iso2: HeaplessString<2>,
}

pub struct CountryIdxModelCache {
    by_id: HashMap<Uuid, CountryIdxModel>,
    by_iso2: HashMap<HeaplessString<2>, Uuid>,
}

impl CountryIdxModelCache {
    pub fn new(
        items: Vec<CountryIdxModel>,
    ) -> Result<Self, &'static str> {
        let mut by_id = HashMap::new();
        let mut by_iso2 = HashMap::new();

        for item in items {
            let primary_key = item.country_id;
            if by_id.contains_key(&primary_key) {
                return Err("Duplicate primary key: country_id");
            }

            if by_iso2.contains_key(&item.iso2) {
                return Err("Duplicate unique index value: iso2");
            }
            by_iso2.insert(item.iso2.clone(), primary_key);
            
            by_id.insert(primary_key, item);
        }

        Ok(CountryIdxModelCache {
            by_id,
            by_iso2,
        })
    }

    pub fn add(&mut self, item: CountryIdxModel) {
        let primary_key = item.country_id;
        if self.by_id.contains_key(&primary_key) {
            return;
        }

        if self.by_iso2.contains_key(&item.iso2) {
            return;
        }
        self.by_iso2.insert(item.iso2.clone(), primary_key);
        
        self.by_id.insert(primary_key, item);
    }
    pub fn remove(&mut self, primary_key: &Uuid) -> Option<CountryIdxModel> {
        if let Some(item) = self.by_id.remove(primary_key) {
            self.by_iso2.remove(&item.iso2);
            Some(item)
        } else {
            None
        }
    }


    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        self.by_id.contains_key(primary_key)
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<CountryIdxModel> {
        self.by_id.get(primary_key).cloned()
    }

    pub fn get_by_iso2(&self, key: &HeaplessString<2>) -> Option<Uuid> {
        self.by_iso2.get(key).copied()
    }
}
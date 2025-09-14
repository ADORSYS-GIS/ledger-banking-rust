use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use std::collections::HashMap;
use super::common_enums::{MessagingType, serialize_messaging_type, deserialize_messaging_type};

/// # Repository Trait
/// - FQN: banking-db/src/repository/messaging_repository.rs/MessagingRepository
/// 
/// # Index: MessagingIdxModel
/// ## Repository Trait
/// - FQN: banking-db/src/repository/messaging_repository.rs/MessagingRepository
/// ## Trait method
/// - create_idx
/// - load_idxes
/// ## Pg Trigger
/// - CREATE
/// - UPDATE
/// ## Cache: MessagingIdxModelCache
/// - Mutable Set of Immutable Records
/// - Concurent
/// 
/// # Audit: MessagingAuditModel
/// ## Repository Trait
/// - FQN: banking-db/src/repository/messaging_repository.rs/MessagingRepository
/// ## Trait method
/// - create_audit
/// ## Additional field: `pub version: i32`
/// ### Nature
/// - composite-primary with self.id
/// ## Additional field: `pub hash: i64,`
/// ## Additional field: `pub audit_log_id: Uuid,`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MessagingModel {
    /// # Trait method
    /// - find_by_id
    /// - find_by_ids
    /// - exists_by_id
    /// 
    /// # Index: messaging_id
    /// ## Nature
    /// - primary
    /// 
    /// # Audit: messaging_id
    /// ## Nature
    /// - composite-primary with self.version
    /// ## Trait method
    /// - find_audits_by_id
    pub id: Uuid,

    /// # Documentation
    /// - Type of messaging/communication method
    #[serde(serialize_with = "serialize_messaging_type", deserialize_with = "deserialize_messaging_type")]
    pub messaging_type: MessagingType,
    
    /// # Documentation
    /// - The actual messaging identifier/location (email, phone, username, etc.)
    /// 
    /// # Trait method
    /// - find_ids_by_value
    /// 
    /// # Index: value_hash: i64
    /// ## Nature
    /// - secondary
    /// - unique
    /// 
    /// # Audit
    /// ## Trait method
    /// - find_audits_by_value
    pub value: HeaplessString<100>,

    /// # Documentation
    /// - Description of the messaging type when MessagingType::Other is used
    pub other_type: Option<HeaplessString<20>>,
}

/// # Repository Trait
/// - FQN: banking-db/src/repository/messaging_repository.rs/MessagingRepository
/// # Trait method
/// - create_audit
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MessagingAuditModel {
    /// # Nature
    /// - composite-primary with self.version
    /// # Trait method
    /// - find_audits_by_id
    pub messaging_id: Uuid,

    /// # Nature
    /// - composite-primary with self.id
    pub version: i32,

    pub hash: i64,

    /// # Documentation
    /// - Type of messaging/communication method
    #[serde(serialize_with = "serialize_messaging_type", deserialize_with = "deserialize_messaging_type")]
    pub messaging_type: MessagingType,
    
    /// # Trait method
    /// - find_audits_by_value
    pub value: HeaplessString<100>,

    /// # Documentation
    /// - Description of the messaging type when MessagingType::Other is used
    pub other_type: Option<HeaplessString<20>>,

    pub audit_log_id: Uuid,
}

/// # Repository Trait
/// - FQN: banking-db/src/repository/messaging_repository.rs/MessagingRepository
/// # Trait method
/// - create_idx
/// - load_idxes
/// # Pg Trigger
/// - CREATE
/// # Cache: MessagingIdxModelCache
/// - Concurent
/// - Mutable Set of Immutable Records
#[derive(Debug, Clone, FromRow)]
pub struct MessagingIdxModel {
    /// # Nature
    /// - primary
    pub messaging_id: Uuid,
    /// # Nature
    /// - secondary
    /// - unique
    pub value_hash: i64,
    pub version: i32,
    pub hash: i64,
}

pub struct MessagingIdxModelCache {
    by_id: HashMap<Uuid, MessagingIdxModel>,
    by_value_hash: HashMap<i64, Uuid>,
}

impl MessagingIdxModelCache {
    pub fn new(items: Vec<MessagingIdxModel>) -> Result<Self, &'static str> {
        let mut by_id = HashMap::new();
        let mut by_value_hash = HashMap::new();

        for item in items {
            let primary_key = item.messaging_id;
            if by_id.contains_key(&primary_key) {
                return Err("Duplicate primary key: messaging_id");
            }

            if by_value_hash.contains_key(&item.value_hash) {
                return Err("Duplicate unique index value: value_hash");
            }
            by_value_hash.insert(item.value_hash, primary_key);

            by_id.insert(primary_key, item);
        }

        Ok(MessagingIdxModelCache {
            by_id,
            by_value_hash,
        })
    }

    pub fn add(&mut self, item: MessagingIdxModel) {
        let primary_key = item.messaging_id;
        if self.by_id.contains_key(&primary_key) {
            self.update(item);
            return;
        }

        self.by_value_hash.insert(item.value_hash, primary_key);
        self.by_id.insert(primary_key, item);
    }

    pub fn remove(&mut self, messaging_id: &Uuid) -> Option<MessagingIdxModel> {
        if let Some(item) = self.by_id.remove(messaging_id) {
            self.by_value_hash.remove(&item.value_hash);
            return Some(item);
        }
        None
    }

    pub fn update(&mut self, item: MessagingIdxModel) {
        self.remove(&item.messaging_id);
        self.add(item);
    }

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        self.by_id.contains_key(primary_key)
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<MessagingIdxModel> {
        self.by_id.get(primary_key).cloned()
    }

    pub fn get_by_value_hash(&self, key: &i64) -> Option<Uuid> {
        self.by_value_hash.get(key).copied()
    }
}
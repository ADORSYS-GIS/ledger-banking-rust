use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use std::collections::HashMap;
use super::common_enums::{MessagingType, serialize_messaging_type_option, deserialize_messaging_type_option};

/// Database model for person type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "person_type", rename_all = "PascalCase")]
pub enum PersonType {
    Natural,
    Legal,
    System,
    Integration,
    Unknown,
}

impl std::fmt::Display for PersonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersonType::Natural => write!(f, "Natural"),
            PersonType::Legal => write!(f, "Legal"),
            PersonType::System => write!(f, "System"),
            PersonType::Integration => write!(f, "Integration"),
            PersonType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// # Repository Trait
/// - FQN: banking-db/src/repository/person/person_repository.rs/PersonRepository
/// # Documentation
/// - Database model for Person
/// - Represents a person throughout the system for bank audit and tracking purposes
/// 
/// # Index: PersonIdxModel
/// ## Repository Trait
/// - FQN: banking-db/src/repository/person/person_repository.rs/PersonRepository
/// ## Trait method
/// - create_idx
/// - load_idxes
/// - update_idx
/// ## Pg Trigger
/// - CREATE
/// - UPDATE
/// ## Cache: PersonIdxModel
/// - Mutable Set of Mutable Records
/// 
/// # Audit: PersonAuditModel
/// ## Repository Trait
/// - FQN: banking-db/src/repository/person/person_repository.rs/PersonRepository
/// ## Trait method
/// - create_audit
/// ## Additional field: `pub version: i32`
/// ### Nature
/// - composite-primary with self.id
/// ## Additional field: `pub hash: i64,`
/// ## Additional field: `pub audit_log_id: Uuid,`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PersonModel {
    /// # Trait method
    /// - find_by_id
    /// - find_by_ids
    /// - exists_by_id
    /// 
    /// # Index: person_id
    /// ## Nature
    /// - primary
    /// 
    /// # Audit: person_id
    /// ## Nature
    /// - composite-primary with self.version
    /// ## Trait method
    /// - find_audits_by_id
    pub id: Uuid,
    
    #[serde(serialize_with = "serialize_person_type", deserialize_with = "deserialize_person_type")]
    pub person_type: PersonType,
    
    pub display_name: HeaplessString<100>,

    /// # Documentation
    /// External identifier (e.g., employee ID, badge number, system ID)
    /// 
    /// # Trait method
    /// - get_ids_by_external_identifier
    /// - get_by_external_identifier
    /// 
    /// # Index: external_identifier_hash: Option<i64>
    /// ## Nature
    /// - secondary
    /// 
    /// # Audit
    /// ## Trait method
    /// - get_audits_by_external_identifier
    pub external_identifier: Option<HeaplessString<50>>,

    /// # Trait method
    /// - get_by_entity_reference
    /// 
    /// # Audit
    /// ## Trait method
    /// - get_audits_by_entity_reference
    pub entity_reference_count: i32,
    
    /// # Documentation
    /// References PersonModel.id for organizational hierarchy
    /// ## Constraint
    /// - exists(PersonModel.id)
    pub organization_person_id: Option<Uuid>,
    
    /// # Documentation
    /// References to MessagingModel.messaging_id (up to 5 messaging methods)
    pub messaging1_id: Option<Uuid>,
    #[serde(serialize_with = "serialize_messaging_type_option", deserialize_with = "deserialize_messaging_type_option")]
    pub messaging1_type: Option<MessagingType>,
    pub messaging2_id: Option<Uuid>,
    #[serde(serialize_with = "serialize_messaging_type_option", deserialize_with = "deserialize_messaging_type_option")]
    pub messaging2_type: Option<MessagingType>,
    pub messaging3_id: Option<Uuid>,
    #[serde(serialize_with = "serialize_messaging_type_option", deserialize_with = "deserialize_messaging_type_option")]
    pub messaging3_type: Option<MessagingType>,
    pub messaging4_id: Option<Uuid>,
    #[serde(serialize_with = "serialize_messaging_type_option", deserialize_with = "deserialize_messaging_type_option")]
    pub messaging4_type: Option<MessagingType>,
    pub messaging5_id: Option<Uuid>,
    #[serde(serialize_with = "serialize_messaging_type_option", deserialize_with = "deserialize_messaging_type_option")]
    pub messaging5_type: Option<MessagingType>,
    
    /// # Documentation
    /// Department within organization
    pub department: Option<HeaplessString<50>>,

    /// # Documentation
    /// References LocationModel.id for person's location
    /// ## Constraint
    /// - exists(LocationModel.id)
    pub location_id: Option<Uuid>,
    
    /// ## Constraint
    /// - exists(PersonModel.id)
    pub duplicate_of_person_id: Option<Uuid>,
}

/// # Repository Trait
/// - FQN: banking-db/src/repository/person/person_repository.rs/PersonRepository
/// # Trait method
/// - create_audit
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PersonAuditModel {
    /// # Nature
    /// - composite-primary with self.version
    /// # Trait method
    /// - find_audits_by_person_id
    pub person_id: Uuid,
    
    /// # Nature
    /// - composite-primary with self.person_id
    pub version: i32,

    pub hash: i64,

    #[serde(serialize_with = "serialize_person_type", deserialize_with = "deserialize_person_type")]
    pub person_type: PersonType,
    
    pub display_name: HeaplessString<100>,

    /// # Trait method
    /// - get_audits_by_external_identifier
    pub external_identifier: Option<HeaplessString<50>>,

    /// # Trait method
    /// - get_audits_by_entity_reference
    pub entity_reference_count: i32,
    
    pub organization_person_id: Option<Uuid>,
    
    pub messaging1_id: Option<Uuid>,
    #[serde(serialize_with = "serialize_messaging_type_option", deserialize_with = "deserialize_messaging_type_option")]
    pub messaging1_type: Option<MessagingType>,
    pub messaging2_id: Option<Uuid>,
    #[serde(serialize_with = "serialize_messaging_type_option", deserialize_with = "deserialize_messaging_type_option")]
    pub messaging2_type: Option<MessagingType>,
    pub messaging3_id: Option<Uuid>,
    #[serde(serialize_with = "serialize_messaging_type_option", deserialize_with = "deserialize_messaging_type_option")]
    pub messaging3_type: Option<MessagingType>,
    pub messaging4_id: Option<Uuid>,
    #[serde(serialize_with = "serialize_messaging_type_option", deserialize_with = "deserialize_messaging_type_option")]
    pub messaging4_type: Option<MessagingType>,
    pub messaging5_id: Option<Uuid>,
    #[serde(serialize_with = "serialize_messaging_type_option", deserialize_with = "deserialize_messaging_type_option")]
    pub messaging5_type: Option<MessagingType>,
    
    pub department: Option<HeaplessString<50>>,

    pub location_id: Option<Uuid>,
    
    pub duplicate_of_person_id: Option<Uuid>,

    pub audit_log_id: Uuid,
}

// Serialization functions for PersonType
pub fn serialize_person_type<S>(person_type: &PersonType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match person_type {
        PersonType::Natural => "Natural",
        PersonType::Legal => "Legal",
        PersonType::System => "System",
        PersonType::Integration => "Integration",
        PersonType::Unknown => "Unknown",
    };
    serializer.serialize_str(type_str)
}

pub fn deserialize_person_type<'de, D>(deserializer: D) -> Result<PersonType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Natural" => Ok(PersonType::Natural),
        "Legal" => Ok(PersonType::Legal),
        "System" => Ok(PersonType::System),
        "Integration" => Ok(PersonType::Integration),
        "unknown" => Ok(PersonType::Unknown),
        _ => Err(serde::de::Error::custom(format!("Unknown person type: {s}"))),
    }
}

/// # Repository Trait
/// - FQN: banking-db/src/repository/person/person_repository.rs/PersonRepository
/// # Trait method
/// - create_idx
/// - load_idxes
/// - update_idx
/// # Pg Trigger
/// - CREATE
/// - UPDATE
/// # Cache: PersonIdxModelCache
/// - Concurent
/// - Mutable Set of Mutable Records
#[derive(Debug, Clone, FromRow)]
pub struct PersonIdxModel {
    /// # Nature
    /// - primary
    pub person_id: Uuid,
    /// # Nature
    /// - secondary
    pub external_identifier_hash: Option<i64>,
    pub version: i32,
    pub hash: i64,
}

pub struct PersonIdxModelCache {
    by_id: HashMap<Uuid, PersonIdxModel>,
    by_external_identifier_hash: HashMap<i64, Vec<Uuid>>,
}

impl PersonIdxModelCache {
    pub fn new(items: Vec<PersonIdxModel>) -> Result<Self, &'static str> {
        let mut by_id = HashMap::new();
        let mut by_external_identifier_hash = HashMap::new();

        for item in items {
            let primary_key = item.person_id;
            if by_id.contains_key(&primary_key) {
                return Err("Duplicate primary key: person_id");
            }

            if let Some(hash) = item.external_identifier_hash {
                by_external_identifier_hash
                    .entry(hash)
                    .or_insert_with(Vec::new)
                    .push(primary_key);
            }

            by_id.insert(primary_key, item);
        }

        Ok(PersonIdxModelCache {
            by_id,
            by_external_identifier_hash,
        })
    }

    pub fn add(&mut self, item: PersonIdxModel) {
        let primary_key = item.person_id;
        if self.by_id.contains_key(&primary_key) {
            self.update(item);
            return;
        }

        if let Some(hash) = item.external_identifier_hash {
            self.by_external_identifier_hash
                .entry(hash)
                .or_default()
                .push(primary_key);
        }
        self.by_id.insert(primary_key, item);
    }

    pub fn remove(&mut self, person_id: &Uuid) -> Option<PersonIdxModel> {
        if let Some(item) = self.by_id.remove(person_id) {
            if let Some(hash) = item.external_identifier_hash {
                if let Some(ids) = self.by_external_identifier_hash.get_mut(&hash) {
                    ids.retain(|&id| id != *person_id);
                    if ids.is_empty() {
                        self.by_external_identifier_hash.remove(&hash);
                    }
                }
            }
            return Some(item);
        }
        None
    }

    pub fn update(&mut self, item: PersonIdxModel) {
        self.remove(&item.person_id);
        self.add(item);
    }

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        self.by_id.contains_key(primary_key)
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<PersonIdxModel> {
        self.by_id.get(primary_key).cloned()
    }

    pub fn get_by_external_identifier_hash(&self, key: &i64) -> Option<&Vec<Uuid>> {
        self.by_external_identifier_hash.get(key)
    }
}
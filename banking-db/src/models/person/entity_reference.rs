use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use std::collections::HashMap;

/// Database model for person entity type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "person_entity_type", rename_all = "PascalCase")]
pub enum RelationshipRole {
    Customer,
    Employee,
    Shareholder,
    Director,
    BeneficialOwner,
    Agent,
    Vendor,
    Partner,
    RegulatoryContact,
    EmergencyContact,
    SystemAdmin,
    Other,
}

/// # Repository Trait
/// - FQN: banking-db/src/repository/person/entity_reference_repository.rs/EntityReferenceRepository
/// # Documentation
/// - Entity reference table for managing person-to-entity relationships
/// - Database model for EntityReference
/// 
/// # Index: EntityReferenceIdxModel
/// ## Repository Trait
/// - FQN: banking-db/src/repository/person/entity_reference_repository.rs/EntityReferenceRepository
/// ## Trait method
/// - create_idx
/// - load_idxes
/// ## Pg Trigger
/// - CREATE
/// - UPDATE
/// ## Cache
/// - Mutable Set of Mutable Records
/// 
/// # Audit: EntityReferenceAuditModel
/// ## Repository Trait
/// - FQN: banking-db/src/repository/person/entity_reference_repository.rs/EntityReferenceRepository
/// ## Trait method
/// - create_audit
/// ## Additional field: `pub version: i32`
/// ### Nature
/// - composite-primary with self.id
/// ## Additional field: `pub hash: i64,`
/// ## Additional field: `pub audit_log_id: Uuid,`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EntityReferenceModel {
    /// # Trait method
    /// - find_by_id
    /// - find_by_ids
    /// - exists_by_id
    /// 
    /// # Index: entity_reference_id
    /// ## Nature
    /// - primary
    /// 
    /// # Audit: entity_reference_id
    /// ## Nature
    /// - composite-primary with self.version
    /// ## Trait method
    /// - find_audits_by_id
    pub id: Uuid,

    /// # Documentation
    /// - References PersonModel.person_id
    /// 
    /// # Trait method
    /// - find_ids_by_person_id
    /// - find_by_person_id
    /// 
    /// # Index
    /// ## Nature
    /// - secondary
    /// 
    /// # Audit
    /// ## Trait method
    /// - find_audits_by_person_id
    /// ## Constraint
    /// - exists(PersonModel.id)
    pub person_id: Uuid,

    /// # Documentation
    /// - Type of entity relationship
    #[serde(serialize_with = "serialize_person_entity_type", deserialize_with = "deserialize_person_entity_type")]
    pub entity_role: RelationshipRole,

    /// # Documentation
    /// - External identifier for the reference (e.g., customer ID, employee ID)
    /// 
    /// # Trait method
    /// - find_by_reference_external_id
    /// 
    /// # Audit
    /// ## Trait method
    /// - find_audits_by_reference_external_id
    pub reference_external_id: HeaplessString<50>,

    pub reference_details_l1: Option<HeaplessString<50>>,
    pub reference_details_l2: Option<HeaplessString<50>>,
    pub reference_details_l3: Option<HeaplessString<50>>,
}

/// # Repository Trait
/// - FQN: banking-db/src/repository/person/entity_reference_repository.rs/EntityReferenceRepository
/// # Trait method
/// - create_audit
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EntityReferenceAuditModel {
    /// # Nature
    /// - composite-primary with self.version
    /// # Trait method
    /// - find_audits_by_id
    pub entity_reference_id: Uuid,

    /// # Nature
    /// - composite-primary with self.id
    pub version: i32,

    pub hash: i64,

    /// # Trait method
    /// - find_audits_by_person_id
    pub person_id: Uuid,

    #[serde(serialize_with = "serialize_person_entity_type", deserialize_with = "deserialize_person_entity_type")]
    pub entity_role: RelationshipRole,

    /// # Trait method
    /// - find_audits_by_reference_external_id
    pub reference_external_id: HeaplessString<50>,

    pub reference_details_l1: Option<HeaplessString<50>>,
    pub reference_details_l2: Option<HeaplessString<50>>,
    pub reference_details_l3: Option<HeaplessString<50>>,

    pub audit_log_id: Uuid,
}

// Serialization functions for RelationshipRole
pub fn serialize_person_entity_type<S>(entity_role: &RelationshipRole, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match entity_role {
        RelationshipRole::Customer => "customer",
        RelationshipRole::Employee => "employee",
        RelationshipRole::Shareholder => "shareholder",
        RelationshipRole::Director => "director",
        RelationshipRole::BeneficialOwner => "beneficialowner",
        RelationshipRole::Agent => "agent",
        RelationshipRole::Vendor => "vendor",
        RelationshipRole::Partner => "partner",
        RelationshipRole::RegulatoryContact => "regulatorycontact",
        RelationshipRole::EmergencyContact => "emergencycontact",
        RelationshipRole::SystemAdmin => "systemadmin",
        RelationshipRole::Other => "other",
    };
    serializer.serialize_str(type_str)
}

pub fn deserialize_person_entity_type<'de, D>(deserializer: D) -> Result<RelationshipRole, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "customer" => Ok(RelationshipRole::Customer),
        "employee" => Ok(RelationshipRole::Employee),
        "shareholder" => Ok(RelationshipRole::Shareholder),
        "director" => Ok(RelationshipRole::Director),
        "beneficialowner" => Ok(RelationshipRole::BeneficialOwner),
        "agent" => Ok(RelationshipRole::Agent),
        "vendor" => Ok(RelationshipRole::Vendor),
        "partner" => Ok(RelationshipRole::Partner),
        "regulatorycontact" => Ok(RelationshipRole::RegulatoryContact),
        "emergencycontact" => Ok(RelationshipRole::EmergencyContact),
        "systemadmin" => Ok(RelationshipRole::SystemAdmin),
        "other" => Ok(RelationshipRole::Other),
        _ => Err(serde::de::Error::custom(format!("Unknown person entity type: {s}"))),
    }
}

/// # Repository Trait
/// - FQN: banking-db/src/repository/person/entity_reference_repository.rs/EntityReferenceRepository
/// # Trait method
/// - create_idx
/// - load_idxes
/// # Pg Trigger
/// - CREATE
/// - UPDATE
/// # Cache: EntityReferenceIdxModelCache
/// - Concurent
/// - Mutable Set of Mutable Records
#[derive(Debug, Clone, FromRow)]
pub struct EntityReferenceIdxModel {
    /// # Nature
    /// - primary
    pub entity_reference_id: Uuid,
    /// # Nature
    /// - secondary
    pub person_id: Uuid,
    /// # Nature
    /// - secondary
    pub reference_external_id_hash: i64,
    pub version: i32,
    pub hash: i64,
}

pub struct EntityReferenceIdxModelCache {
    by_id: HashMap<Uuid, EntityReferenceIdxModel>,
    by_person_id: HashMap<Uuid, Vec<Uuid>>,
    by_reference_external_id_hash: HashMap<i64, Vec<Uuid>>,
}

impl EntityReferenceIdxModelCache {
    pub fn new(items: Vec<EntityReferenceIdxModel>) -> Result<Self, &'static str> {
        let mut by_id = HashMap::new();
        let mut by_person_id: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        let mut by_reference_external_id_hash: HashMap<i64, Vec<Uuid>> = HashMap::new();

        for item in items {
            let primary_key = item.entity_reference_id;
            if by_id.contains_key(&primary_key) {
                return Err("Duplicate primary key: entity_reference_id");
            }

            by_person_id
                .entry(item.person_id)
                .or_default()
                .push(primary_key);

            by_reference_external_id_hash
                .entry(item.reference_external_id_hash)
                .or_default()
                .push(primary_key);

            by_id.insert(primary_key, item);
        }

        Ok(EntityReferenceIdxModelCache {
            by_id,
            by_person_id,
            by_reference_external_id_hash,
        })
    }

    pub fn add(&mut self, item: EntityReferenceIdxModel) {
        let primary_key = item.entity_reference_id;
        if self.by_id.contains_key(&primary_key) {
            self.update(item);
            return;
        }

        self.by_person_id
            .entry(item.person_id)
            .or_default()
            .push(primary_key);

        self.by_reference_external_id_hash
            .entry(item.reference_external_id_hash)
            .or_default()
            .push(primary_key);

        self.by_id.insert(primary_key, item);
    }

    pub fn remove(&mut self, entity_reference_id: &Uuid) -> Option<EntityReferenceIdxModel> {
        if let Some(item) = self.by_id.remove(entity_reference_id) {
            if let Some(ids) = self.by_person_id.get_mut(&item.person_id) {
                ids.retain(|&id| id != *entity_reference_id);
                if ids.is_empty() {
                    self.by_person_id.remove(&item.person_id);
                }
            }
            if let Some(ids) = self
                .by_reference_external_id_hash
                .get_mut(&item.reference_external_id_hash)
            {
                ids.retain(|&id| id != *entity_reference_id);
                if ids.is_empty() {
                    self.by_reference_external_id_hash
                        .remove(&item.reference_external_id_hash);
                }
            }
            return Some(item);
        }
        None
    }

    pub fn update(&mut self, item: EntityReferenceIdxModel) {
        self.remove(&item.entity_reference_id);
        self.add(item);
    }

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        self.by_id.contains_key(primary_key)
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<EntityReferenceIdxModel> {
        self.by_id.get(primary_key).cloned()
    }

    pub fn get_by_person_id(&self, key: &Uuid) -> Option<&Vec<Uuid>> {
        self.by_person_id.get(key)
    }

    pub fn get_by_reference_external_id_hash(&self, key: &i64) -> Option<&Vec<Uuid>> {
        self.by_reference_external_id_hash.get(key)
    }
}
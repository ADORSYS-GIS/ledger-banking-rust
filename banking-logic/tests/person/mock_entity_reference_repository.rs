use async_trait::async_trait;
use banking_api::domain::person::{EntityReference, RelationshipRole};
use banking_db::models::person::{
    EntityReferenceAuditModel, EntityReferenceIdxModel, EntityReferenceModel,
};
use banking_db::repository::person::entity_reference_repository::{EntityReferenceRepository, EntityReferenceRepositoryError, EntityReferenceResult};
use banking_db::repository::PersonRepository;
use heapless::String as HeaplessString;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use sqlx::Postgres;
use crate::person::mock_person_repository::MockPersonRepository;

#[derive(Default)]
pub struct MockEntityReferenceRepository {
    entities: Mutex<Vec<EntityReferenceModel>>,
    entity_ixes: Mutex<Vec<EntityReferenceIdxModel>>,
    entity_audits: Mutex<Vec<EntityReferenceAuditModel>>,
    person_repository: Arc<MockPersonRepository>,
}

impl MockEntityReferenceRepository {
    pub fn new(person_repository: Arc<MockPersonRepository>) -> Self {
        Self {
            entities: Mutex::new(Vec::new()),
            entity_ixes: Mutex::new(Vec::new()),
            entity_audits: Mutex::new(Vec::new()),
            person_repository,
        }
    }
}

#[async_trait]
impl EntityReferenceRepository<Postgres> for MockEntityReferenceRepository {
    async fn save(
        &self,
        entity_ref: EntityReferenceModel,
        audit_log_id: Uuid,
    ) -> EntityReferenceResult<EntityReferenceModel> {
        if !self.person_repository.exists_by_id(entity_ref.person_id).await.unwrap() {
            return Err(EntityReferenceRepositoryError::PersonNotFound(entity_ref.person_id));
        }

        self.entities.lock().unwrap().push(entity_ref.clone());
        let entity_idx = EntityReferenceIdxModel {
            entity_reference_id: entity_ref.id,
            person_id: entity_ref.person_id,
            reference_external_id_hash: 0, // dummy hash
            version: 0,
            hash: 0,
        };
        self.entity_ixes.lock().unwrap().push(entity_idx);

        let entity_audit = EntityReferenceAuditModel {
            entity_reference_id: entity_ref.id,
            version: 0,
            hash: 0,
            person_id: entity_ref.person_id,
            entity_role: entity_ref.entity_role,
            reference_external_id: entity_ref.reference_external_id.clone(),
            reference_details_l1: entity_ref.reference_details_l1.clone(),
            reference_details_l2: entity_ref.reference_details_l2.clone(),
            reference_details_l3: entity_ref.reference_details_l3.clone(),
            audit_log_id,
        };
        self.entity_audits.lock().unwrap().push(entity_audit);

        Ok(entity_ref)
    }

    async fn load(&self, id: Uuid) -> EntityReferenceResult<EntityReferenceModel> {
        match self.entities.lock().unwrap().iter().find(|p| p.id == id).cloned() {
            Some(e) => Ok(e),
            None => Err(EntityReferenceRepositoryError::RepositoryError("Not found".into()))
        }
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> EntityReferenceResult<Option<EntityReferenceIdxModel>> {
        Ok(self
            .entity_ixes
            .lock()
            .unwrap()
            .iter()
            .find(|e| e.entity_reference_id == id)
            .cloned())
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> EntityReferenceResult<Vec<EntityReferenceIdxModel>> {
        let entities = self
            .entity_ixes
            .lock()
            .unwrap()
            .iter()
            .filter(|e| ids.contains(&e.entity_reference_id))
            .cloned()
            .collect();
        Ok(entities)
    }

    async fn exists_by_id(&self, id: Uuid) -> EntityReferenceResult<bool> {
        Ok(self.entities.lock().unwrap().iter().any(|e| e.id == id))
    }

    async fn find_ids_by_person_id(
        &self,
        person_id: Uuid,
    ) -> EntityReferenceResult<Vec<Uuid>> {
        let ids = self
            .entities
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.person_id == person_id)
            .map(|e| e.id)
            .collect();
        Ok(ids)
    }

    async fn find_by_person_id(
        &self,
        person_id: Uuid,
        _page: i32,
        _page_size: i32,
    ) -> EntityReferenceResult<Vec<EntityReferenceIdxModel>> {
        let entities = self
            .entity_ixes
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.person_id == person_id)
            .cloned()
            .collect();
        Ok(entities)
    }

    async fn find_by_reference_external_id(
        &self,
        reference_external_id: &str,
        _page: i32,
        _page_size: i32,
    ) -> EntityReferenceResult<Vec<EntityReferenceIdxModel>> {
        let entities = self.entities.lock().unwrap();
        let entity_ixes = self.entity_ixes.lock().unwrap();
        let ids: Vec<Uuid> = entities
            .iter()
            .filter(|e| e.reference_external_id.as_str() == reference_external_id)
            .map(|e| e.id)
            .collect();
        let result = entity_ixes
            .iter()
            .filter(|e| ids.contains(&e.entity_reference_id))
            .cloned()
            .collect();
        Ok(result)
    }
}

pub fn create_test_entity_reference(person_id: Uuid) -> EntityReference {
    EntityReference {
        id: Uuid::new_v4(),
        person_id,
        entity_role: RelationshipRole::Customer,
        reference_external_id: HeaplessString::new(),
        reference_details_l1: None,
        reference_details_l2: None,
        reference_details_l3: None,
    }
}
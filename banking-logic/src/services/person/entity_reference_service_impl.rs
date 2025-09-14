use async_trait::async_trait;
use banking_api::domain::person::EntityReference;
use banking_api::service::EntityReferenceService;
use banking_api::BankingResult;
use heapless::String as HeaplessString;
use sqlx::Database;
use uuid::Uuid;

use crate::mappers::person_mapper::{ToDomain, ToModel};
use crate::services::repositories::Repositories;

pub struct EntityReferenceServiceImpl<DB: Database> {
    repositories: Repositories<DB>,
}

impl<DB: Database> EntityReferenceServiceImpl<DB> {
    pub fn new(repositories: Repositories<DB>) -> Self {
        Self { repositories }
    }
}

#[async_trait]
impl<DB: Database + Send + Sync> EntityReferenceService
    for EntityReferenceServiceImpl<DB>
{
    async fn create_entity_reference(
        &self,
        entity_reference: EntityReference,
        audit_log: banking_api::domain::AuditLog,
    ) -> BankingResult<EntityReference> {
        let model = entity_reference.to_model();
        let saved_model = self
            .repositories
            .entity_reference_repository
            .save(model, audit_log.id)
            .await?;
        Ok(saved_model.to_domain())
    }

    async fn fix_entity_reference(
        &self,
        entity_reference: EntityReference,
    ) -> BankingResult<EntityReference> {
        let audit_log = banking_api::domain::AuditLog {
            id: Uuid::new_v4(),
            updated_at: chrono::Utc::now(),
            updated_by_person_id: Uuid::new_v4(), // Placeholder
        };
        self.create_entity_reference(entity_reference, audit_log)
            .await
    }

    async fn find_entity_reference_by_id(
        &self,
        id: Uuid,
    ) -> BankingResult<Option<EntityReference>> {
        let model_idx = self
            .repositories
            .entity_reference_repository
            .find_by_id(id)
            .await?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .entity_reference_repository
                .load(idx.entity_reference_id)
                .await?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn find_entity_references_by_person_id(
        &self,
        person_id: Uuid,
    ) -> BankingResult<Vec<EntityReference>> {
        let model_ixes = self
            .repositories
            .entity_reference_repository
            .find_by_person_id(person_id, 1, 1000)
            .await?;
        let mut refs = Vec::new();
        for idx in model_ixes {
            let ref_model = self
                .repositories
                .entity_reference_repository
                .load(idx.entity_reference_id)
                .await?;
            refs.push(ref_model.to_domain());
        }
        Ok(refs)
    }

    async fn find_entity_references_by_reference_external_id(
        &self,
        reference_external_id: HeaplessString<50>,
    ) -> BankingResult<Vec<EntityReference>> {
        let model_ixes = self
            .repositories
            .entity_reference_repository
            .find_by_reference_external_id(reference_external_id.as_str(), 1, 1000)
            .await?;
        let mut refs = Vec::new();
        for idx in model_ixes {
            let ref_model = self
                .repositories
                .entity_reference_repository
                .load(idx.entity_reference_id)
                .await?;
            refs.push(ref_model.to_domain());
        }
        Ok(refs)
    }
}
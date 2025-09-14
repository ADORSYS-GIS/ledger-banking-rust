use crate::domain::person::EntityReference;
use crate::domain::AuditLog;
use crate::BankingResult;
use async_trait::async_trait;
use heapless::String as HeaplessString;
use uuid::Uuid;

#[async_trait]
pub trait EntityReferenceService: Send + Sync {
    async fn create_entity_reference(
        &self,
        entity_reference: EntityReference,
        audit_log: AuditLog,
    ) -> BankingResult<EntityReference>;
    async fn fix_entity_reference(
        &self,
        entity_reference: EntityReference,
    ) -> BankingResult<EntityReference>;
    async fn find_entity_reference_by_id(&self, id: Uuid) -> BankingResult<Option<EntityReference>>;
    async fn find_entity_references_by_person_id(
        &self,
        person_id: Uuid,
    ) -> BankingResult<Vec<EntityReference>>;
    async fn find_entity_references_by_reference_external_id(
        &self,
        reference_external_id: HeaplessString<50>,
    ) -> BankingResult<Vec<EntityReference>>;
}
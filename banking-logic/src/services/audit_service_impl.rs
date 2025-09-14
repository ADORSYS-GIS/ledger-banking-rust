use crate::mappers::audit_mapper;
use async_trait::async_trait;
use banking_api::{
    domain::audit::AuditLog,
    service::audit_service::{AuditService, AuditServiceError, AuditServiceResult},
};
use banking_db::{
    models::audit::AuditLogModel,
    repository::audit_repository::{AuditDomainError, AuditLogRepository},
};
use sqlx::Postgres;
use std::sync::Arc;
use uuid::Uuid;

pub struct AuditServiceImpl {
    audit_log_repository: Arc<dyn AuditLogRepository<Postgres>>,
}

impl AuditServiceImpl {
    pub fn new(audit_log_repository: Arc<dyn AuditLogRepository<Postgres>>) -> Self {
        Self {
            audit_log_repository,
        }
    }

    fn map_domain_error(err: AuditDomainError) -> AuditServiceError {
        match err {
            AuditDomainError::InvalidAuditType(audit_type) => {
                AuditServiceError::InvalidAuditType(audit_type)
            }
            AuditDomainError::DuplicateAuditRecord { entity_id, version } => {
                AuditServiceError::DuplicateAuditRecord { entity_id, version }
            }
            AuditDomainError::InvalidVersionSequence { expected, actual } => {
                AuditServiceError::InvalidVersionSequence { expected, actual }
            }
            AuditDomainError::HashMismatch { entity_id, version } => {
                AuditServiceError::HashMismatch { entity_id, version }
            }
            AuditDomainError::RepositoryError(err) => {
                AuditServiceError::RepositoryError(err.to_string())
            }
        }
    }
}

#[async_trait]
impl AuditService for AuditServiceImpl {
    async fn create_audit_log(
        &self,
        updated_by_person_id: Uuid,
    ) -> AuditServiceResult<AuditLog> {
        let audit_log = AuditLogModel {
            id: Uuid::new_v4(),
            updated_at: chrono::Utc::now(),
            updated_by_person_id,
        };
        let created_audit_log = self
            .audit_log_repository
            .create(&audit_log)
            .await
            .map_err(Self::map_domain_error)?;
        Ok(audit_mapper::map_to_domain(&created_audit_log))
    }

    async fn find_audit_log_by_id(&self, id: Uuid) -> AuditServiceResult<Option<AuditLog>> {
        let audit_log = self
            .audit_log_repository
            .find_by_id(id)
            .await
            .map_err(Self::map_domain_error)?;
        Ok(audit_log.map(|log| audit_mapper::map_to_domain(&log)))
    }
}
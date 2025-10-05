use crate::mappers::audit::audit_log_mapper;
use async_trait::async_trait;
use banking_api::{
    domain::audit::AuditLog,
    service::audit::audit_log_service::{AuditLogService, AuditLogServiceError, AuditLogServiceResult},
};
use banking_db::{
    models::audit::AuditLogModel,
    repository::audit_repository::{AuditLogRepository, AuditLogRepositoryError},
};
use sqlx::Postgres;
use std::sync::Arc;
use uuid::Uuid;

pub struct AuditLogServiceImpl {
    audit_log_repository: Arc<dyn AuditLogRepository<Postgres>>,
}

impl AuditLogServiceImpl {
    pub fn new(audit_log_repository: Arc<dyn AuditLogRepository<Postgres>>) -> Self {
        Self {
            audit_log_repository,
        }
    }

    fn map_domain_error(err: AuditLogRepositoryError) -> AuditLogServiceError {
        match err {
            AuditLogRepositoryError::RepositoryError(err) => {
                AuditLogServiceError::RepositoryError(err.to_string())
            }
        }
    }
}

#[async_trait]
impl AuditLogService for AuditLogServiceImpl {
    async fn create_audit_log(
        &self,
        updated_by_person_id: Uuid,
    ) -> AuditLogServiceResult<AuditLog> {
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
        Ok(audit_log_mapper::map_to_domain(&created_audit_log))
    }

    async fn find_audit_log_by_id(&self, id: Uuid) -> AuditLogServiceResult<Option<AuditLog>> {
        let audit_log = self
            .audit_log_repository
            .find_by_id(id)
            .await
            .map_err(Self::map_domain_error)?;
        Ok(audit_log.map(|log| audit_log_mapper::map_to_domain(&log)))
    }
}
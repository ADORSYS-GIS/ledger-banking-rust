use async_trait::async_trait;
use banking_api::{domain::audit::AuditLog, service::audit_service::AuditService};
use banking_db::{repository::audit_repository::AuditLogRepository, models::audit::AuditLogModel};
use std::sync::Arc;
use uuid::Uuid;
use crate::mappers::audit_mapper;

pub struct AuditServiceImpl {
    audit_log_repository: Arc<dyn AuditLogRepository>,
}

impl AuditServiceImpl {
    pub fn new(audit_log_repository: Arc<dyn AuditLogRepository>) -> Self {
        Self {
            audit_log_repository,
        }
    }
}

#[async_trait]
impl AuditService for AuditServiceImpl {
    async fn create_audit_log(&self, updated_by_person_id: Uuid) -> Result<AuditLog, Box<dyn std::error::Error + Send + Sync>> {
        let audit_log = AuditLogModel {
            id: Uuid::new_v4(),
            updated_at: chrono::Utc::now(),
            updated_by_person_id,
        };
        let created_audit_log = self.audit_log_repository.create(&audit_log).await?;
        Ok(audit_mapper::map_to_domain(&created_audit_log))
    }

    async fn find_audit_log_by_id(&self, id: Uuid) -> Result<Option<AuditLog>, Box<dyn std::error::Error + Send + Sync>> {
        let audit_log = self.audit_log_repository.find_by_id(id).await?;
        Ok(audit_log.map(|log| audit_mapper::map_to_domain(&log)))
    }
}
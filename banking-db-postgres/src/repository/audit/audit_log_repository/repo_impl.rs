use async_trait::async_trait;
use banking_db::{
    models::audit::AuditLogModel,
    repository::audit_repository::{AuditLogRepository, AuditLogResult},
};
use sqlx::{Postgres};
use uuid::Uuid;

// Import the new Executor enum
use crate::repository::executor::Executor;

pub struct AuditLogRepositoryImpl {
    // The struct now holds our generic Executor
    pub(crate) executor: Executor,
}

impl AuditLogRepositoryImpl {
    // The constructor now accepts the Executor
    pub fn new(executor: Executor) -> Self {
        Self { executor }
    }
}

#[async_trait]
impl AuditLogRepository<Postgres> for AuditLogRepositoryImpl {
    async fn create(&self, audit_log: &AuditLogModel) -> AuditLogResult<AuditLogModel> {
        super::create::create(&self.executor, audit_log).await
    }

    async fn find_by_id(&self, id: Uuid) -> AuditLogResult<Option<AuditLogModel>> {
        super::find_by_id::find_by_id(&self.executor, id).await
    }
}
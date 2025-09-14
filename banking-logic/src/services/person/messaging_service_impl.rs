use async_trait::async_trait;
use banking_api::domain::person::Messaging;
use banking_api::service::MessagingService;
use banking_api::BankingResult;
use heapless::String as HeaplessString;
use sqlx::Database;
use uuid::Uuid;

use crate::mappers::person_mapper::{ToDomain, ToModel};
use crate::services::repositories::Repositories;

pub struct MessagingServiceImpl<DB: Database> {
    repositories: Repositories<DB>,
}

impl<DB: Database> MessagingServiceImpl<DB> {
    pub fn new(repositories: Repositories<DB>) -> Self {
        Self { repositories }
    }
}

#[async_trait]
impl<DB: Database + Send + Sync> MessagingService for MessagingServiceImpl<DB> {
    async fn create_messaging(
        &self,
        messaging: Messaging,
        audit_log: banking_api::domain::AuditLog,
    ) -> BankingResult<Messaging> {
        let model = messaging.to_model();
        let saved_model = self
            .repositories
            .messaging_repository
            .save(model, audit_log.id)
            .await?;
        Ok(saved_model.to_domain())
    }

    async fn fix_messaging(&self, messaging: Messaging) -> BankingResult<Messaging> {
        let audit_log = banking_api::domain::AuditLog {
            id: Uuid::new_v4(),
            updated_at: chrono::Utc::now(),
            updated_by_person_id: Uuid::new_v4(), // Placeholder
        };
        self.create_messaging(messaging, audit_log).await
    }

    async fn find_messaging_by_id(&self, id: Uuid) -> BankingResult<Option<Messaging>> {
        let model_idx = self.repositories.messaging_repository.find_by_id(id).await?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .messaging_repository
                .load(idx.messaging_id)
                .await?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn find_messaging_by_value(
        &self,
        value: HeaplessString<100>,
    ) -> BankingResult<Option<Messaging>> {
        let ids = self
            .repositories
            .messaging_repository
            .find_ids_by_value(value.as_str())
            .await?;
        if let Some(id) = ids.first() {
            let model_idx = self.repositories.messaging_repository.find_by_id(*id).await?;
            if let Some(idx) = model_idx {
                let model = self
                    .repositories
                    .messaging_repository
                    .load(idx.messaging_id)
                    .await?;
                Ok(Some(model.to_domain()))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
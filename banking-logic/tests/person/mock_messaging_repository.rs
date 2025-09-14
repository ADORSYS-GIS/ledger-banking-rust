use async_trait::async_trait;
use banking_api::domain::person::Messaging;
use banking_db::models::person::{MessagingAuditModel, MessagingIdxModel, MessagingModel};
use banking_db::repository::person::messaging_repository::{
    MessagingRepository, MessagingRepositoryError, MessagingResult,
};
use heapless::String as HeaplessString;
use std::sync::Mutex;
use uuid::Uuid;
use sqlx::Postgres;

#[derive(Default)]
pub struct MockMessagingRepository {
    messages: Mutex<Vec<MessagingModel>>,
    message_ixes: Mutex<Vec<MessagingIdxModel>>,
    message_audits: Mutex<Vec<MessagingAuditModel>>,
}

#[async_trait]
impl MessagingRepository<Postgres> for MockMessagingRepository {
    async fn save(
        &self,
        messaging: MessagingModel,
        audit_log_id: Uuid,
    ) -> MessagingResult<MessagingModel> {
        self.messages.lock().unwrap().push(messaging.clone());
        let msg_idx = MessagingIdxModel {
            messaging_id: messaging.id,
            value_hash: 0, // dummy hash
            version: 0,
            hash: 0,
        };
        self.message_ixes.lock().unwrap().push(msg_idx);

        let msg_audit = MessagingAuditModel {
            messaging_id: messaging.id,
            version: 0,
            hash: 0,
            messaging_type: messaging.messaging_type,
            value: messaging.value.clone(),
            other_type: messaging.other_type.clone(),
            audit_log_id,
        };
        self.message_audits.lock().unwrap().push(msg_audit);

        Ok(messaging)
    }

    async fn load(&self, id: Uuid) -> MessagingResult<MessagingModel> {
        match self.messages.lock().unwrap().iter().find(|p| p.id == id).cloned() {
            Some(messaging) => Ok(messaging),
            None => Err(MessagingRepositoryError::NotFound(id)),
        }
    }

    async fn find_by_id(&self, id: Uuid) -> MessagingResult<Option<MessagingIdxModel>> {
        Ok(self
            .message_ixes
            .lock()
            .unwrap()
            .iter()
            .find(|m| m.messaging_id == id)
            .cloned())
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> MessagingResult<Vec<MessagingIdxModel>> {
        let messages = self
            .message_ixes
            .lock()
            .unwrap()
            .iter()
            .filter(|m| ids.contains(&m.messaging_id))
            .cloned()
            .collect();
        Ok(messages)
    }

    async fn exists_by_id(&self, id: Uuid) -> MessagingResult<bool> {
        Ok(self
            .message_ixes
            .lock()
            .unwrap()
            .iter()
            .any(|m| m.messaging_id == id))
    }

    async fn find_ids_by_value(&self, value: &str) -> MessagingResult<Vec<Uuid>> {
        let ids = self
            .messages
            .lock()
            .unwrap()
            .iter()
            .filter(|m| m.value.as_str() == value)
            .map(|m| m.id)
            .collect();
        Ok(ids)
    }
}

pub fn create_test_messaging() -> Messaging {
    Messaging {
        id: Uuid::new_v4(),
        messaging_type: banking_api::domain::person::MessagingType::Email,
        value: HeaplessString::try_from("test@example.com").unwrap(),
        other_type: None,
    }
}
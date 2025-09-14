use crate::domain::person::Messaging;
use crate::domain::AuditLog;
use crate::BankingResult;
use async_trait::async_trait;
use heapless::String as HeaplessString;
use uuid::Uuid;

#[async_trait]
pub trait MessagingService: Send + Sync {
    async fn create_messaging(&self, messaging: Messaging, audit_log: AuditLog) -> BankingResult<Messaging>;
    async fn fix_messaging(&self, messaging: Messaging) -> BankingResult<Messaging>;
    async fn find_messaging_by_id(&self, id: Uuid) -> BankingResult<Option<Messaging>>;
    async fn find_messaging_by_value(
        &self,
        value: HeaplessString<100>,
    ) -> BankingResult<Option<Messaging>>;
}
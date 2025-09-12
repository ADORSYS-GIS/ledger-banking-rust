use async_trait::async_trait;
use banking_api::BankingResult;

#[async_trait]
pub trait TransactionAware: Send + Sync {
    async fn on_commit(&self) -> BankingResult<()>;
    async fn on_rollback(&self) -> BankingResult<()>;
}
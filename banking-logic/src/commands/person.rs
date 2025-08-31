use banking_api::command::person::{PersonCommand, PersonCommandExecutor};
use banking_api::command::{person::Services, Command, CommandResult};
use banking_api::error::BankingError;
use banking_db::repository::unit_of_work::{UnitOfWork, UnitOfWorkSession};
use sqlx::Database;
use std::any::Any;
use std::marker::PhantomData;
use std::sync::Arc;

/// A factory for creating services within a transactional context.
/// The concrete implementation of this trait will be provided in the
/// application's composition root (e.g., `main.rs`), where the
/// concrete repository types are known.
pub trait ServiceFactory<DB: Database, S: UnitOfWorkSession<DB>>: Send + Sync {
    fn build_services(&self, session: &S) -> Services;
}

pub struct CommandExecutorImpl<DB: Database, F, UoW: UnitOfWork<DB>> {
    service_factory: F,
    uow: Arc<UoW>,
    _marker: PhantomData<DB>,
}

impl<DB: Database, F, UoW: UnitOfWork<DB>> CommandExecutorImpl<DB, F, UoW> {
    pub fn new(service_factory: F, uow: Arc<UoW>) -> Self {
        Self {
            service_factory,
            uow,
            _marker: PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<DB, F, UoW> PersonCommandExecutor for CommandExecutorImpl<DB, F, UoW>
where
    DB: Database + Send + Sync,
    F: ServiceFactory<DB, <UoW as UnitOfWork<DB>>::Session> + Send + Sync,
    UoW: UnitOfWork<DB> + Send + Sync,
{
    async fn execute(
        &self,
        command: PersonCommand,
    ) -> Result<CommandResult, BankingError> {
        let session = self.uow.begin().await?;
        let services = self.service_factory.build_services(&session);

        let result = match command {
            PersonCommand::AddPersonOfInterest(cmd) => cmd
                .execute(&services)
                .await
                .map(|r| Box::new(r) as Box<dyn Any + Send>),
            PersonCommand::PopulateGeoData(cmd) => cmd
                .execute(&services)
                .await
                .map(|r| Box::new(r) as Box<dyn Any + Send>),
        };

        match result {
            Ok(res) => {
                session.commit().await?;
                Ok(res)
            }
            Err(e) => {
                session.rollback().await?;
                Err(e)
            }
        }
    }
}
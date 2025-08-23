use banking_api::command::{AppCommand, Command, CommandExecutor, CommandResult};
use banking_api::error::BankingError;
use banking_api::service::person_service::PersonService;
use std::any::Any;
use std::sync::Arc;

pub struct CommandExecutorImpl {
    person_service: Arc<dyn PersonService>,
    // Add other services here as needed
}

impl CommandExecutorImpl {
    pub fn new(person_service: Arc<dyn PersonService>) -> Self {
        Self { person_service }
    }
}

#[async_trait::async_trait]
impl CommandExecutor for CommandExecutorImpl {
    async fn execute(&self, command: AppCommand) -> Result<CommandResult, BankingError> {
        match command {
            AppCommand::AddPersonOfInterest(cmd) => {
                let result = cmd.execute(&self.person_service).await?;
                Ok(Box::new(result) as Box<dyn Any + Send>)
            }
            AppCommand::PopulateGeoData(cmd) => {
                let result = cmd.execute(&self.person_service).await?;
                Ok(Box::new(result) as Box<dyn Any + Send>)
            }
        }
    }
}
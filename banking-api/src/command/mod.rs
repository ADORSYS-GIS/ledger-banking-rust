pub mod person;

use crate::domain::audit::AuditLog;
use crate::domain::person::Person;
use crate::error::BankingError;
use crate::service::person_service::PersonService;
use async_trait::async_trait;
use std::any::Any;
use std::sync::Arc;

/// A command that can be executed to perform an action and return a result.
#[async_trait]
pub trait Command: Send + Sync {
    /// The context required to execute the command (e.g., services, repositories).
    type Context;
    /// The result of the command execution.
    type Result: Send + 'static;

    /// Executes the command with the given context.
    async fn execute(&self, context: &Self::Context) -> Result<Self::Result, BankingError>;
}

// #############################################################################
// # Command: Add Person Of Interest
// #############################################################################

/// Command to create a new person of interest.
pub struct AddPersonOfInterestCommand {
    pub person_data: Person,
    pub audit_log: AuditLog,
}

#[async_trait]
impl Command for AddPersonOfInterestCommand {
    type Context = Arc<dyn PersonService>;
    type Result = Person;

    async fn execute(&self, context: &Self::Context) -> Result<Self::Result, BankingError> {
        context.create_person(self.person_data.clone()).await
    }
}

// #############################################################################
// # Application Command Enum
// #############################################################################

/// An enum to represent all possible commands in the application.
/// This will be the primary type used by the command executor.
pub enum AppCommand {
    AddPersonOfInterest(Box<AddPersonOfInterestCommand>),
    PopulateGeoData(person::PopulateGeoDataCommand),
    // Add other commands here
}

/// A type-erased result for the command executor.
pub type CommandResult = Box<dyn Any + Send>;

/// A generic command executor trait.
/// The implementation will live in the `banking-logic` crate.
#[async_trait]
pub trait CommandExecutor: Send + Sync {
    async fn execute(&self, command: AppCommand) -> Result<CommandResult, BankingError>;
}

pub use person::PopulateGeoDataCommand;
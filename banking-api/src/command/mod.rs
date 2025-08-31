pub mod person;

use crate::{error::BankingError};
use async_trait::async_trait;
use std::any::Any;

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
// # Generic Command Executor
// #############################################################################
/// A type-erased result for the command executor.
pub type CommandResult = Box<dyn Any + Send>;


use crate::domain::audit::AuditLog;
use crate::domain::person::{Country, CountrySubdivision, Locality, Person};
use crate::error::BankingError;
use async_trait::async_trait;
use serde::Deserialize;
use std::sync::Arc;
use crate::{service::person_service::PersonService};

use super::{Command, CommandResult};

pub struct Services {
    pub person_service: Arc<dyn PersonService>,
}

#[derive(Deserialize)]
struct GeoData {
    countries: Vec<Country>,
    subdivisions: Vec<CountrySubdivision>,
    localities: Vec<Locality>,
}

/// Command to populate geographical data from a JSON string.
pub struct PopulateGeoDataCommand {
    pub json_data: String,
}

#[async_trait]
impl Command for PopulateGeoDataCommand {
    type Context = Services;
    type Result = ();

    async fn execute(&self, context: &Self::Context) -> Result<Self::Result, BankingError> {
        let data: GeoData = serde_json::from_str(&self.json_data).map_err(|e| {
            BankingError::ValidationFailed(format!("Failed to parse JSON data: {e}"))
        })?;

        for country in data.countries {
            context.person_service.create_country(country).await?;
        }

        for subdivision in data.subdivisions {
            context
                .person_service
                .create_country_subdivision(subdivision)
                .await?;
        }

        for locality in data.localities {
            context.person_service.create_locality(locality).await?;
        }

        Ok(())
    }
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
    type Context = Services;
    type Result = Person;

    async fn execute(&self, context: &Self::Context) -> Result<Self::Result, BankingError> {
        context
            .person_service
            .create_person(self.person_data.clone(), self.audit_log.clone())
            .await
    }
}

// #############################################################################
// # Application Command Enum
// #############################################################################

/// An enum to represent all possible commands in the application.
/// This will be the primary type used by the command executor.
pub enum PersonCommand {
    AddPersonOfInterest(Box<AddPersonOfInterestCommand>),
    PopulateGeoData(PopulateGeoDataCommand),
    // Add other commands here
}

/// A generic command executor trait.
/// The implementation will live in the `banking-logic` crate.
#[async_trait]
pub trait PersonCommandExecutor: Send + Sync {
    async fn execute(
        &self,
        command: PersonCommand,
    ) -> Result<CommandResult, BankingError>;
}
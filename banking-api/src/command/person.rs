use crate::domain::person::{Country, CountrySubdivision, Locality};
use crate::error::BankingError;
use crate::service::person_service::PersonService;
use async_trait::async_trait;
use serde::Deserialize;
use std::sync::Arc;

use super::Command;

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
    type Context = Arc<dyn PersonService>;
    type Result = ();

    async fn execute(&self, context: &Self::Context) -> Result<Self::Result, BankingError> {
        let data: GeoData = serde_json::from_str(&self.json_data).map_err(|e| {
            BankingError::ValidationFailed(format!("Failed to parse JSON data: {e}"))
        })?;

        for country in data.countries {
            context.create_country(country).await?;
        }

        for subdivision in data.subdivisions {
            context.create_country_subdivision(subdivision).await?;
        }

        for locality in data.localities {
            context.create_locality(locality).await?;
        }

        Ok(())
    }
}
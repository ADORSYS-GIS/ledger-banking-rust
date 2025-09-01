use anyhow::Result;
use banking_api::service::person_service::PersonService;
use banking_db_postgres::test_utils::commons::{cleanup_database, establish_connection};
use uuid::Uuid;
use std::fs;
use std::sync::Arc;
use banking_logic::services::person_service_impl::PersonServiceImpl;
use banking_db_postgres::{
    PostgresRepositories};
use banking_api::domain::person::{Country, CountrySubdivision, Locality};
use serde_json;

#[tokio::test]
async fn test_populate_geo_data_directly() -> Result<()> {
    let pool = Arc::new(establish_connection().await);
    cleanup_database(&pool).await;
    let postgres_repos = PostgresRepositories::new(pool.clone());
    let repos = postgres_repos.create_person_service_repositories().await;
    let country_repo = repos.country_repository.clone();
    let country_subdivision_repo = repos.country_subdivision_repository.clone();
    let locality_repo = repos.locality_repository.clone();
    let person_service = Arc::new(PersonServiceImpl::new(repos));

    let json_data = fs::read_to_string("tests/fixtures/geo_data.json")?;
    
    #[derive(serde::Deserialize)]
    struct GeoData {
        countries: Vec<Country>,
        subdivisions: Vec<CountrySubdivision>,
        localities: Vec<Locality>,
    }

    let data: GeoData = serde_json::from_str(&json_data)?;

    for country in data.countries {
        person_service.create_country(country).await?;
    }

    for subdivision in data.subdivisions {
        person_service.create_country_subdivision(subdivision).await?;
    }

    for locality in data.localities {
        person_service.create_locality(locality).await?;
    }

    // Verify the data was inserted
    let uuid = Uuid::parse_str("a7a5b7a0-3b7e-4b0e-8c1a-2b0a9b8a7a5b").expect("Failed to parse uuid");
    let countries = country_repo.find_by_ids(&[uuid]).await?;
    assert_eq!(countries.len(), 1, "Should be 1 country but is {}", countries.len());

    let country_id = countries[0].country_id;
    let subdivisions = country_subdivision_repo.find_by_country_id(country_id, 1, 20).await?;
    assert_eq!(subdivisions.len(), 4, "Should be 4 subdivisions");

    let mut total_localities = 0;
    for subdivision in &subdivisions {
        let localities = locality_repo.find_by_country_subdivision_id(subdivision.country_subdivision_id, 1, 20).await?;
        total_localities += localities.len();
    }
    assert_eq!(total_localities, 12, "Should be 12 localities in total");

    Ok(())
}
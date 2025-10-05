use crate::repository::person::location_repository::LocationRepositoryImpl;
use crate::utils::TryFromRow;
use banking_db::models::person::LocationModel;
use std::collections::HashMap;
use std::error::Error;
use uuid::Uuid;

pub async fn load_batch(
    repo: &LocationRepositoryImpl,
    ids: &[Uuid],
) -> Result<Vec<Option<LocationModel>>, Box<dyn Error + Send + Sync>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let query = r#"SELECT * FROM location WHERE id = ANY($1)"#;
    let rows = match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(query).bind(ids).fetch_all(&**pool).await?
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(query).bind(ids).fetch_all(&mut **tx).await?
        }
    };
    let mut item_map = HashMap::new();
    for row in rows {
        let item = LocationModel::try_from_row(&row)?;
        item_map.insert(item.id, item);
    }
    let mut result = Vec::with_capacity(ids.len());
    for id in ids {
        result.push(item_map.remove(id));
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    use banking_db::models::person::{LocationModel, LocationType};
    use banking_db::repository::{
        BatchRepository, CountryRepository, CountrySubdivisionRepository,
        LocalityRepository, PersonRepos,
    };
    use heapless::String as HeaplessString;
    use rust_decimal::Decimal;
    use std::str::FromStr;
    use uuid::Uuid;

    use crate::repository::person::test_helpers::{
        create_test_country_model, create_test_country_subdivision_model,
        create_test_locality_model,
    };
    use crate::test_helper::setup_test_context;

    pub fn setup_test_location(locality_id: Uuid) -> LocationModel {
        LocationModel {
            id: Uuid::new_v4(),
            locality_id,
            street_line1: HeaplessString::try_from("123 Main St").unwrap(),
            street_line2: None,
            street_line3: None,
            street_line4: None,
            postal_code: Some(HeaplessString::try_from("12345").unwrap()),
            latitude: Some(Decimal::from_str("34.0522").unwrap()),
            longitude: Some(Decimal::from_str("-118.2437").unwrap()),
            accuracy_meters: Some(10.0),
            location_type: LocationType::Residential,
        }
    }

    #[tokio::test]
    async fn test_load_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let person_repos = ctx.person_repos();
        let country_repo = person_repos.countries();
        let subdivision_repo = person_repos.country_subdivisions();
        let locality_repo = person_repos.localities();
        let location_repo = person_repos.locations();

        let unique_iso2 = format!("O{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
        let country = create_test_country_model(&unique_iso2, "Test Country");
        country_repo.save(country.clone()).await?;

        let unique_subdivision_code =
            format!("OS{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
        let subdivision =
            create_test_country_subdivision_model(country.id, &unique_subdivision_code, "Test Subdivision");
        subdivision_repo.save(subdivision.clone()).await?;

        let unique_locality_code = format!("OL{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
        let locality =
            create_test_locality_model(subdivision.id, &unique_locality_code, "Test Locality");
        locality_repo.save(locality.clone()).await?;

        let mut locations = Vec::new();
        for i in 0..5 {
            let mut location = setup_test_location(locality.id);
            location.street_line1 =
                HeaplessString::try_from(format!("Street {i}").as_str()).unwrap();
            locations.push(location);
        }

        let audit_log_id = Uuid::new_v4();

        let saved_locations = location_repo
            .create_batch(locations.clone(), audit_log_id)
            .await?;
        assert_eq!(saved_locations.len(), 5);

        let ids: Vec<Uuid> = saved_locations.iter().map(|l| l.id).collect();
        let loaded_locations = location_repo.load_batch(&ids).await?;

        assert_eq!(loaded_locations.len(), 5);
        for (i, location) in loaded_locations.iter().enumerate() {
            assert_eq!(location.as_ref().unwrap().id, saved_locations[i].id);
        }

        Ok(())
    }
}
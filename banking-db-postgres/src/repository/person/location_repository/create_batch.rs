use crate::repository::person::location_repository::LocationRepositoryImpl;
use banking_db::models::person::{LocationIdxModel, LocationModel};
use banking_db::repository::{LocationRepository, LocationRepositoryError};
use std::error::Error;
use std::hash::Hasher;
use twox_hash::XxHash64;
use uuid::Uuid;

pub async fn create_batch(
    repo: &LocationRepositoryImpl,
    items: Vec<LocationModel>,
    audit_log_id: Uuid,
) -> Result<Vec<LocationModel>, Box<dyn Error + Send + Sync>> {
    if items.is_empty() {
        return Ok(Vec::new());
    }

    let ids: Vec<Uuid> = items.iter().map(|p| p.id).collect();
    let existing_check = repo.exist_by_ids(&ids).await?;
    let truly_existing_ids: Vec<Uuid> = existing_check
        .into_iter()
        .filter_map(|(id, exists)| if exists { Some(id) } else { None })
        .collect();

    if !truly_existing_ids.is_empty() {
        return Err(Box::new(LocationRepositoryError::ManyLocationsExist(
            truly_existing_ids,
        )));
    }

    let cache = repo.location_idx_cache.read().await;
    for item in &items {
        let mut hasher = XxHash64::with_seed(0);
        let mut cbor = Vec::new();
        ciborium::ser::into_writer(item, &mut cbor).unwrap();
        hasher.write(&cbor);
        let hash = hasher.finish() as i64;

        let idx_model = LocationIdxModel {
            location_id: item.id,
            locality_id: item.locality_id,
            version: 0,
            hash,
        };
        cache.add(idx_model);
    }

    let mut location_values = Vec::new();
    let mut location_idx_values = Vec::new();
    let mut location_audit_values = Vec::new();
    let mut saved_items = Vec::new();

    for item in items {
        let idx_model = cache.get_by_primary(&item.id).unwrap();

        location_values.push((
            item.id,
            item.street_line1.to_string(),
            item.street_line2.as_ref().map(|s| s.to_string()),
            item.street_line3.as_ref().map(|s| s.to_string()),
            item.street_line4.as_ref().map(|s| s.to_string()),
            item.locality_id,
            item.postal_code.as_ref().map(|s| s.to_string()),
            item.latitude,
            item.longitude,
            item.accuracy_meters,
            item.location_type,
        ));

        location_idx_values.push((item.id, item.locality_id, 0i32, idx_model.hash));

        location_audit_values.push((
            item.id,
            0i32,
            idx_model.hash,
            item.street_line1.to_string(),
            item.street_line2.as_ref().map(|s| s.to_string()),
            item.street_line3.as_ref().map(|s| s.to_string()),
            item.street_line4.as_ref().map(|s| s.to_string()),
            item.locality_id,
            item.postal_code.as_ref().map(|s| s.to_string()),
            item.latitude,
            item.longitude,
            item.accuracy_meters,
            item.location_type,
            audit_log_id,
        ));
        saved_items.push(item);
    }

    if !location_values.is_empty() {
        repo.execute_location_insert(location_values).await?;
        repo.execute_location_idx_insert(location_idx_values)
            .await?;
        repo.execute_location_audit_insert(location_audit_values)
            .await?;
    }

    Ok(saved_items)
}
#[cfg(test)]
mod tests {
    use banking_db::models::person::{LocationModel, LocationType};
    use banking_db::repository::{
        BatchRepository, CountryRepository, CountrySubdivisionRepository, LocationRepository,
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
    async fn test_create_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

        for location in &saved_locations {
            assert!(location_repo.exists_by_id(location.id).await?);
        }

        Ok(())
    }
}
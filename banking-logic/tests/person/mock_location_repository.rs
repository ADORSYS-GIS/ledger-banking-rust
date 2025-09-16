use async_trait::async_trait;
use banking_api::domain::person::{Location, LocationType};
use banking_db::models::person::{LocationAuditModel, LocationIdxModel, LocationModel};
use banking_db::repository::location_repository::{LocationRepository, LocationRepositoryError};
use heapless::String as HeaplessString;
use std::sync::Mutex;
use uuid::Uuid;
use sqlx::Postgres;

#[derive(Default)]
pub struct MockLocationRepository {
    locations: Mutex<Vec<LocationModel>>,
    location_ixes: Mutex<Vec<LocationIdxModel>>,
    location_audits: Mutex<Vec<LocationAuditModel>>,
}

#[async_trait]
impl LocationRepository<Postgres> for MockLocationRepository {
    async fn save(
        &self,
        location: LocationModel,
        audit_log_id: Uuid,
    ) -> Result<LocationModel, LocationRepositoryError> {
        self.locations.lock().unwrap().push(location.clone());
        let location_idx = LocationIdxModel {
            location_id: location.id,
            locality_id: location.locality_id,
            version: 0,
            hash: 0,
        };
        self.location_ixes.lock().unwrap().push(location_idx);

        let location_audit = LocationAuditModel {
            location_id: location.id,
            version: 0,
            hash: 0,
            street_line1: location.street_line1.clone(),
            street_line2: location.street_line2.clone(),
            street_line3: location.street_line3.clone(),
            street_line4: location.street_line4.clone(),
            locality_id: location.locality_id,
            postal_code: location.postal_code.clone(),
            latitude: location.latitude,
            longitude: location.longitude,
            accuracy_meters: location.accuracy_meters,
            location_type: location.location_type,
            audit_log_id,
        };
        self.location_audits.lock().unwrap().push(location_audit);

        Ok(location)
    }

    async fn load(&self, id: Uuid) -> Result<LocationModel, LocationRepositoryError> {
        match self
            .locations
            .lock()
            .unwrap()
            .iter()
            .find(|a| a.id == id)
            .cloned()
        {
            Some(location) => Ok(location),
            None => Err(LocationRepositoryError::RepositoryError(Box::new(
                sqlx::Error::RowNotFound,
            ))),
        }
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<LocationIdxModel>, LocationRepositoryError> {
        Ok(self
            .location_ixes
            .lock()
            .unwrap()
            .iter()
            .find(|a| a.location_id == id)
            .cloned())
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<LocationIdxModel>, LocationRepositoryError> {
        let locations = self
            .location_ixes
            .lock()
            .unwrap()
            .iter()
            .filter(|a| ids.contains(&a.location_id))
            .cloned()
            .collect();
        Ok(locations)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, LocationRepositoryError> {
        Ok(self
            .location_ixes
            .lock()
            .unwrap()
            .iter()
            .any(|l| l.location_id == id))
    }

    async fn find_ids_by_locality_id(
        &self,
        locality_id: Uuid,
    ) -> Result<Vec<Uuid>, LocationRepositoryError> {
        let ids = self
            .locations
            .lock()
            .unwrap()
            .iter()
            .filter(|l| l.locality_id == locality_id)
            .map(|l| l.id)
            .collect();
        Ok(ids)
    }

    async fn find_by_locality_id(
        &self,
        locality_id: Uuid,
        _page: i32,
        _page_size: i32,
    ) -> Result<Vec<LocationIdxModel>, LocationRepositoryError> {
        let locations = self
            .location_ixes
            .lock()
            .unwrap()
            .iter()
            .filter(|a| a.locality_id == locality_id)
            .cloned()
            .collect();
        Ok(locations)
    }
}

pub fn create_test_location(locality_id: Uuid) -> Location {
    Location {
        id: Uuid::new_v4(),
        location_type: LocationType::Residential,
        street_line1: HeaplessString::try_from("123 Main St").unwrap(),
        street_line2: None,
        street_line3: None,
        street_line4: None,
        locality_id,
        postal_code: None,
        latitude: None,
        longitude: None,
        accuracy_meters: None,
    }
}
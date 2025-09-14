use async_trait::async_trait;
use banking_api::domain::person::Locality;
use banking_db::models::person::{LocalityIdxModel, LocalityModel};
use banking_db::repository::LocalityRepository;
use heapless::String as HeaplessString;
use std::error::Error;
use std::sync::Mutex;
use uuid::Uuid;
use sqlx::Postgres;

#[derive(Default)]
pub struct MockLocalityRepository {
    localities: Mutex<Vec<LocalityModel>>,
    locality_ixes: Mutex<Vec<LocalityIdxModel>>,
}

#[async_trait]
impl LocalityRepository<Postgres> for MockLocalityRepository {
    async fn save(&self, locality: LocalityModel) -> Result<LocalityModel, sqlx::Error> {
        self.localities.lock().unwrap().push(locality.clone());
        let locality_idx = LocalityIdxModel {
            locality_id: locality.id,
            country_subdivision_id: locality.country_subdivision_id,
            code_hash: 0, // dummy hash
        };
        self.locality_ixes.lock().unwrap().push(locality_idx);
        Ok(locality)
    }

    async fn load(&self, id: Uuid) -> Result<LocalityModel, sqlx::Error> {
        Ok(self
            .localities
            .lock()
            .unwrap()
            .iter()
            .find(|c| c.id == id)
            .cloned()
            .unwrap())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<LocalityIdxModel>, sqlx::Error> {
        Ok(self
            .locality_ixes
            .lock()
            .unwrap()
            .iter()
            .find(|c| c.locality_id == id)
            .cloned())
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<LocalityIdxModel>, Box<dyn Error + Send + Sync>> {
        let localities = self
            .locality_ixes
            .lock()
            .unwrap()
            .iter()
            .filter(|l| ids.contains(&l.locality_id))
            .cloned()
            .collect();
        Ok(localities)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(self
            .locality_ixes
            .lock()
            .unwrap()
            .iter()
            .any(|l| l.locality_id == id))
    }

    async fn find_ids_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = self
            .localities
            .lock()
            .unwrap()
            .iter()
            .filter(|l| l.country_subdivision_id == country_subdivision_id)
            .map(|l| l.id)
            .collect();
        Ok(ids)
    }

    async fn find_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
        _page: i32,
        _page_size: i32,
    ) -> Result<Vec<LocalityIdxModel>, sqlx::Error> {
        let localities = self
            .locality_ixes
            .lock()
            .unwrap()
            .iter()
            .filter(|c| c.country_subdivision_id == country_subdivision_id)
            .cloned()
            .collect();
        Ok(localities)
    }

    async fn find_by_code(
        &self,
        country_subdivision_id: Uuid,
        code: &str,
    ) -> Result<Option<LocalityIdxModel>, sqlx::Error> {
        let locality = self
            .localities
            .lock()
            .unwrap()
            .iter()
            .find(|c| c.country_subdivision_id == country_subdivision_id && c.code.as_str() == code)
            .cloned();

        if let Some(loc) = locality {
            Ok(self
                .locality_ixes
                .lock()
                .unwrap()
                .iter()
                .find(|l| l.locality_id == loc.id)
                .cloned())
        } else {
            Ok(None)
        }
    }
}

pub fn create_test_locality(country_subdivision_id: Uuid) -> Locality {
    Locality {
        id: Uuid::new_v4(),
        country_subdivision_id,
        code: HeaplessString::try_from("LA").unwrap(),
        name_l1: HeaplessString::try_from("Los Angeles").unwrap(),
        name_l2: None,
        name_l3: None,
    }
}
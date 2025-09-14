use async_trait::async_trait;
use banking_api::domain::person::CountrySubdivision;
use banking_db::models::person::{CountrySubdivisionIdxModel, CountrySubdivisionModel};
use banking_db::repository::CountrySubdivisionRepository;
use heapless::String as HeaplessString;
use std::error::Error;
use std::sync::Mutex;
use uuid::Uuid;
use sqlx::Postgres;

#[derive(Default)]
pub struct MockCountrySubdivisionRepository {
    country_subdivisions: Mutex<Vec<CountrySubdivisionModel>>,
    country_subdivision_ixes: Mutex<Vec<CountrySubdivisionIdxModel>>,
}

#[async_trait]
impl CountrySubdivisionRepository<Postgres> for MockCountrySubdivisionRepository {
    async fn save(
        &self,
        country_subdivision: CountrySubdivisionModel,
    ) -> Result<CountrySubdivisionModel, sqlx::Error> {
        self.country_subdivisions.lock().unwrap().push(country_subdivision.clone());
        let country_subdivision_idx = CountrySubdivisionIdxModel {
            country_subdivision_id: country_subdivision.id,
            country_id: country_subdivision.country_id,
            code_hash: 0, // dummy hash
        };
        self.country_subdivision_ixes
            .lock()
            .unwrap()
            .push(country_subdivision_idx);
        Ok(country_subdivision)
    }

    async fn load(&self, id: Uuid) -> Result<CountrySubdivisionModel, sqlx::Error> {
        Ok(self
            .country_subdivisions
            .lock()
            .unwrap()
            .iter()
            .find(|s| s.id == id)
            .cloned()
            .unwrap())
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<CountrySubdivisionIdxModel>, sqlx::Error> {
        Ok(self
            .country_subdivision_ixes
            .lock()
            .unwrap()
            .iter()
            .find(|s| s.country_subdivision_id == id)
            .cloned())
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<CountrySubdivisionIdxModel>, Box<dyn Error + Send + Sync>> {
        let subdivisions = self
            .country_subdivision_ixes
            .lock()
            .unwrap()
            .iter()
            .filter(|s| ids.contains(&s.country_subdivision_id))
            .cloned()
            .collect();
        Ok(subdivisions)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(self
            .country_subdivision_ixes
            .lock()
            .unwrap()
            .iter()
            .any(|s| s.country_subdivision_id == id))
    }

    async fn find_ids_by_country_id(
        &self,
        country_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = self
            .country_subdivisions
            .lock()
            .unwrap()
            .iter()
            .filter(|s| s.country_id == country_id)
            .map(|s| s.id)
            .collect();
        Ok(ids)
    }

    async fn find_by_country_id(
        &self,
        country_id: Uuid,
        _page: i32,
        _page_size: i32,
    ) -> Result<Vec<CountrySubdivisionIdxModel>, sqlx::Error> {
        let country_subdivisions = self
            .country_subdivision_ixes
            .lock()
            .unwrap()
            .iter()
            .filter(|s| s.country_id == country_id)
            .cloned()
            .collect();
        Ok(country_subdivisions)
    }

    async fn find_by_code(
        &self,
        country_id: Uuid,
        code: &str,
    ) -> Result<Option<CountrySubdivisionIdxModel>, sqlx::Error> {
        let subdivision = self
            .country_subdivisions
            .lock()
            .unwrap()
            .iter()
            .find(|s| s.country_id == country_id && s.code.as_str() == code)
            .cloned();

        if let Some(sub) = subdivision {
            Ok(self
                .country_subdivision_ixes
                .lock()
                .unwrap()
                .iter()
                .find(|s| s.country_subdivision_id == sub.id)
                .cloned())
        } else {
            Ok(None)
        }
    }
}

pub fn create_test_country_subdivision(country_id: Uuid) -> CountrySubdivision {
    CountrySubdivision {
        id: Uuid::new_v4(),
        country_id,
        code: HeaplessString::try_from("CA").unwrap(),
        name_l1: HeaplessString::try_from("California").unwrap(),
        name_l2: None,
        name_l3: None,
    }
}
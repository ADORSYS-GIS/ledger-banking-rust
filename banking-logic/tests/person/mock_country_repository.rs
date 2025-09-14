use async_trait::async_trait;
use banking_api::domain::person::Country;
use banking_db::models::person::{CountryIdxModel, CountryModel};
use banking_db::repository::person::country_repository::{CountryRepository, CountryRepositoryError, CountryResult};
use heapless::String as HeaplessString;
use std::sync::Mutex;
use uuid::Uuid;
use sqlx::Postgres;

#[derive(Default)]
pub struct MockCountryRepository {
    countries: Mutex<Vec<CountryModel>>,
    country_ixes: Mutex<Vec<CountryIdxModel>>,
}

#[async_trait]
impl CountryRepository<Postgres> for MockCountryRepository {
    async fn save(&self, country: CountryModel) -> CountryResult<CountryModel> {
        let mut countries = self.countries.lock().unwrap();
        if countries.iter().any(|c| c.iso2 == country.iso2) {
            return Err(CountryRepositoryError::DuplicateCountryISO2(
                country.iso2.to_string(),
            ));
        }
        countries.push(country.clone());
        let country_idx = CountryIdxModel {
            country_id: country.id,
            iso2: country.iso2.clone(),
        };
        self.country_ixes.lock().unwrap().push(country_idx);
        Ok(country)
    }

    async fn load(&self, id: Uuid) -> CountryResult<CountryModel> {
        self.countries
            .lock()
            .unwrap()
            .iter()
            .find(|c| c.id == id)
            .cloned()
            .ok_or(CountryRepositoryError::CountryNotFound(id))
    }

    async fn find_by_id(&self, id: Uuid) -> CountryResult<Option<CountryIdxModel>> {
        Ok(self
            .country_ixes
            .lock()
            .unwrap()
            .iter()
            .find(|c| c.country_id == id)
            .cloned())
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> CountryResult<Vec<CountryIdxModel>> {
        let countries = self
            .country_ixes
            .lock()
            .unwrap()
            .iter()
            .filter(|c| ids.contains(&c.country_id))
            .cloned()
            .collect();
        Ok(countries)
    }

    async fn exists_by_id(&self, id: Uuid) -> CountryResult<bool> {
        Ok(self
            .country_ixes
            .lock()
            .unwrap()
            .iter()
            .any(|c| c.country_id == id))
    }

    async fn find_ids_by_iso2(&self, iso2: &str) -> CountryResult<Vec<Uuid>> {
        let ids = self
            .countries
            .lock()
            .unwrap()
            .iter()
            .filter(|c| c.iso2.as_str() == iso2)
            .map(|c| c.id)
            .collect();
        Ok(ids)
    }

    async fn find_by_iso2(
        &self,
        iso2: &str,
        _page: i32,
        _page_size: i32,
    ) -> CountryResult<Vec<CountryIdxModel>> {
        let countries = self
            .country_ixes
            .lock()
            .unwrap()
            .iter()
            .filter(|c| c.iso2.as_str() == iso2)
            .cloned()
            .collect();
        Ok(countries)
    }
}

// Helper functions for creating test data
pub fn create_test_country() -> Country {
    Country {
        id: Uuid::new_v4(),
        iso2: HeaplessString::try_from("US").unwrap(),
        name_l1: HeaplessString::try_from("United States").unwrap(),
        name_l2: None,
        name_l3: None,
    }
}
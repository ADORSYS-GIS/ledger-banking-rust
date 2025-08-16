use async_trait::async_trait;
use banking_api::BankingError;
use banking_db::models::person::{
    AddressModel, CityModel, CountryModel, EntityReferenceModel, MessagingModel, PersonModel,
    PersonType, RelationshipRole, StateProvinceModel, AddressType, MessagingType,
};
use banking_db::repository::{
    AddressRepository, CityRepository, CountryRepository, EntityReferenceRepository,
    MessagingRepository, PersonRepository, StateProvinceRepository,
};
use sqlx::{postgres::PgRow, PgPool, Row};
use std::error::Error;
use std::hash::Hasher;
use std::str::FromStr;
use std::sync::Arc;
use twox_hash::XxHash64;
use uuid::Uuid;

use crate::utils::{get_heapless_string, get_optional_heapless_string};

trait TryFromRow<R> {
    fn try_from_row(row: &R) -> Result<Self, Box<dyn Error + Send + Sync>>
    where
        Self: Sized;
}

pub struct PersonRepositoryImpl {
    pool: Arc<PgPool>,
}

impl PersonRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PersonRepository for PersonRepositoryImpl {
    async fn save(&self, person: PersonModel) -> Result<PersonModel, Box<dyn Error + Send + Sync>> {
        sqlx::query(
            r#"
            INSERT INTO person (
                id, person_type, display_name, external_identifier, organization_person_id,
                messaging1_id, messaging1_type, messaging2_id, messaging2_type, messaging3_id, messaging3_type,
                messaging4_id, messaging4_type, messaging5_id, messaging5_type,
                department, location_address_id, duplicate_of_person_id, is_active, created_at, updated_at
            )
            VALUES ($1, $2::person_type, $3, $4, $5, $6, $7::messaging_type, $8, $9::messaging_type, $10, $11::messaging_type, $12, $13::messaging_type, $14, $15::messaging_type, $16, $17, $18, $19, $20, $21)
           "#,
        )
        .bind(person.id)
        .bind(person.person_type)
        .bind(person.display_name.as_str())
        .bind(person.external_identifier.as_ref().map(|s| s.as_str()))
        .bind(person.organization_person_id)
        .bind(person.messaging1_id)
        .bind(person.messaging1_type)
        .bind(person.messaging2_id)
        .bind(person.messaging2_type)
        .bind(person.messaging3_id)
        .bind(person.messaging3_type)
        .bind(person.messaging4_id)
        .bind(person.messaging4_type)
        .bind(person.messaging5_id)
        .bind(person.messaging5_type)
        .bind(person.department.as_ref().map(|s| s.as_str()))
        .bind(person.location_address_id)
        .bind(person.duplicate_of_person_id)
        .bind(person.is_active)
        .bind(person.created_at)
        .bind(person.updated_at)
        .execute(&*self.pool)
        .await?;

        let external_identifier_hash = person.external_identifier.as_ref().map(|s| {
            let mut hasher = XxHash64::with_seed(0);
            hasher.write(s.as_bytes());
            hasher.finish() as i64
        });

        sqlx::query(
            r#"
            INSERT INTO person_idx (person_id, person_type, external_identifier_hash, is_active)
            VALUES ($1, $2::person_type, $3, $4)
            "#,
        )
        .bind(person.id)
        .bind(person.person_type)
        .bind(external_identifier_hash)
        .bind(person.is_active)
        .execute(&*self.pool)
        .await?;

        Ok(person)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<PersonModel>, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query(
            r#"
            SELECT * FROM person WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(PersonModel::try_from_row(&row)?)),
            None => Ok(None),
        }
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM person WHERE id = ANY($1)
            "#,
        )
        .bind(ids)
        .fetch_all(&*self.pool)
        .await?;

        let mut persons = Vec::new();
        for row in rows {
            persons.push(PersonModel::try_from_row(&row)?);
        }
        Ok(persons)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM person WHERE id = $1)",
            id
        )
        .fetch_one(&*self.pool)
        .await?;
        Ok(exists.unwrap_or(false))
    }

    async fn find_ids_by_person_type(
        &self,
        person_type: PersonType,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = sqlx::query_scalar(
            "SELECT person_id FROM person_idx WHERE person_type = $1::person_type"
        )
        .bind(person_type)
        .fetch_all(&*self.pool)
        .await?;
        Ok(ids)
    }

    async fn find_by_person_type(
        &self,
        person_type: PersonType,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        let ids = self.find_ids_by_person_type(person_type).await?;
        let start = (page.saturating_sub(1) * page_size) as usize;
        let end = (start + page_size as usize).min(ids.len());
        if start >= end {
            return Ok(vec![]);
        }
        self.find_by_ids(&ids[start..end]).await
    }

    async fn find_ids_by_is_active(
        &self,
        is_active: bool,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = sqlx::query_scalar(
            "SELECT person_id FROM person_idx WHERE is_active = $1"
        )
        .bind(is_active)
        .fetch_all(&*self.pool)
        .await?;
        Ok(ids)
    }

    async fn find_by_is_active(
        &self,
        is_active: bool,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        let ids = self.find_ids_by_is_active(is_active).await?;
        let start = (page.saturating_sub(1) * page_size) as usize;
        let end = (start + page_size as usize).min(ids.len());
        if start >= end {
            return Ok(vec![]);
        }
        self.find_by_ids(&ids[start..end]).await
    }

    async fn get_ids_by_external_identifier(
        &self,
        identifier: &str,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let mut hasher = XxHash64::with_seed(0);
        hasher.write(identifier.as_bytes());
        let hash = hasher.finish() as i64;

        let ids = sqlx::query_scalar(
            "SELECT person_id FROM person_idx WHERE external_identifier_hash = $1",
        )
        .bind(hash)
        .fetch_all(&*self.pool)
        .await?;
        Ok(ids)
    }

    async fn get_by_external_identifier(
        &self,
        identifier: &str,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        let ids = self.get_ids_by_external_identifier(identifier).await?;
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let persons = self.find_by_ids(&ids).await?;
        let filtered_persons = persons
            .into_iter()
            .filter(|p| p.external_identifier.as_deref() == Some(identifier))
            .collect();
        Ok(filtered_persons)
    }

    async fn get_by_entity_reference(
        &self,
        entity_id: Uuid,
        entity_type: RelationshipRole,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query(
            r#"
            SELECT p.*
            FROM person p
            INNER JOIN entity_reference er ON p.id = er.person_id
            WHERE er.id = $1 AND er.entity_role:relationship_role = $2
            "#,
        )
        .bind(entity_id)
        .bind(entity_type)
        .fetch_all(&*self.pool)
        .await?;

        let mut persons = Vec::new();
        for row in rows {
            persons.push(PersonModel::try_from_row(&row)?);
        }
        Ok(persons)
    }

    async fn create(
        &self,
        display_name: &str,
        person_type: PersonType,
        external_identifier: Option<&str>,
    ) -> Result<PersonModel, Box<dyn Error + Send + Sync>> {
        if let Some(ext_id) = external_identifier {
            let existing = self.get_by_external_identifier(ext_id).await?;
            if !existing.is_empty() {
                return Err(Box::new(BankingError::DuplicatePerson(
                    "Person with the same external identifier already exists".to_string(),
                )));
            }
        }

        let new_person = PersonModel {
            id: Uuid::new_v4(),
            person_type,
            display_name: display_name.try_into().map_err(|_| "Display name too long")?,
            external_identifier: external_identifier
                .map(|s| s.try_into())
                .transpose()
                .map_err(|_| "External identifier too long")?,
            organization_person_id: None,
            messaging1_id: None,
            messaging1_type: None,
            messaging2_id: None,
            messaging2_type: None,
            messaging3_id: None,
            messaging3_type: None,
            messaging4_id: None,
            messaging4_type: None,
            messaging5_id: None,
            messaging5_type: None,
            department: None,
            location_address_id: None,
            duplicate_of_person_id: None,
            is_active: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.save(new_person).await
    }

    async fn mark_as_duplicate(
        &self,
        person_id: Uuid,
        duplicate_of_person_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query!(
            "UPDATE person SET duplicate_of_person_id = $2, is_active = false WHERE id = $1",
            person_id,
            duplicate_of_person_id
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    async fn search_by_name(
        &self,
        query: &str,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM person WHERE display_name ILIKE $1
            "#,
        )
        .bind(format!("%{query}%"))
        .fetch_all(&*self.pool)
        .await?;

        let mut persons = Vec::new();
        for row in rows {
            persons.push(PersonModel::try_from_row(&row)?);
        }
        Ok(persons)
    }

    async fn batch_create(
        &self,
        persons: Vec<PersonModel>,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        let mut created_persons = Vec::new();
        for person in persons {
            created_persons.push(self.save(person).await?);
        }
        Ok(created_persons)
    }
}

pub struct CountryRepositoryImpl {
    pool: Arc<PgPool>,
}
impl CountryRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl CountryRepository for CountryRepositoryImpl {
    async fn save(
        &self,
        _country: CountryModel,
    ) -> Result<CountryModel, Box<dyn Error + Send + Sync>> {
        // Implementation for save
        todo!()
    }
    async fn find_by_id(
        &self,
        _id: Uuid,
    ) -> Result<Option<CountryModel>, Box<dyn Error + Send + Sync>> {
        // Implementation for find_by_id
        todo!()
    }
    async fn find_by_ids(
        &self,
        _ids: &[Uuid],
    ) -> Result<Vec<CountryModel>, Box<dyn Error + Send + Sync>> {
        // Implementation for find_by_ids
        todo!()
    }
    async fn exists_by_id(&self, _id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        // Implementation for exists_by_id
        todo!()
    }
    async fn find_ids_by_iso2(&self, _iso2: &str) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        // Implementation for find_ids_by_iso2
        todo!()
    }
    async fn find_by_iso2(
        &self,
        _iso2: &str,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<CountryModel>, Box<dyn Error + Send + Sync>> {
        // Implementation for find_by_iso2
        todo!()
    }
    async fn find_ids_by_is_active(
        &self,
        _is_active: bool,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        // Implementation for find_ids_by_is_active
        todo!()
    }
    async fn find_by_is_active(
        &self,
        _is_active: bool,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<CountryModel>, Box<dyn Error + Send + Sync>> {
        // Implementation for find_by_is_active
        todo!()
    }
}

pub struct StateProvinceRepositoryImpl {
    pool: Arc<PgPool>,
}
impl StateProvinceRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl StateProvinceRepository for StateProvinceRepositoryImpl {
    async fn save(
        &self,
        state: StateProvinceModel,
    ) -> Result<StateProvinceModel, Box<dyn Error + Send + Sync>> {
        sqlx::query(
            r#"
            INSERT INTO state_province (id, country_id, state_province_code, name_l1, name_l2, name_l3, is_active, created_at, updated_at, created_by_person_id, updated_by_person_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(state.id)
        .bind(state.country_id)
        .bind(state.state_province_code.as_str())
        .bind(state.name_l1.as_str())
        .bind(state.name_l2.as_ref().map(|s| s.as_str()))
        .bind(state.name_l3.as_ref().map(|s| s.as_str()))
        .bind(state.is_active)
        .bind(state.created_at)
        .bind(state.updated_at)
        .bind(state.created_by_person_id)
        .bind(state.updated_by_person_id)
        .execute(&*self.pool)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO state_province_idx (state_province_id, country_id, state_province_code, is_active)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(state.id)
        .bind(state.country_id)
        .bind(state.state_province_code.as_str())
        .bind(state.is_active)
        .execute(&*self.pool)
        .await?;

        Ok(state)
    }
    async fn find_by_id(
        &self,
        _id: Uuid,
    ) -> Result<Option<StateProvinceModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_by_ids(
        &self,
        _ids: &[Uuid],
    ) -> Result<Vec<StateProvinceModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn exists_by_id(&self, _id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_ids_by_country_id(
        &self,
        _country_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_by_country_id(
        &self,
        _country_id: Uuid,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<StateProvinceModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_state_province_by_state_province_code(
        &self,
        country_id: Uuid,
        state_province_code: &str,
    ) -> Result<Option<StateProvinceModel>, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query(
            r#"
            SELECT * FROM state_province WHERE country_id = $1 AND state_province_code = $2
            "#,
        )
        .bind(country_id)
        .bind(state_province_code)
        .fetch_optional(&*self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(StateProvinceModel::try_from_row(&row)?)),
            None => Ok(None),
        }
    }
    async fn find_ids_by_is_active(
        &self,
        _is_active: bool,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_by_is_active(
        &self,
        _is_active: bool,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<StateProvinceModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
}

pub struct CityRepositoryImpl {
    pool: Arc<PgPool>,
}
impl CityRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl CityRepository for CityRepositoryImpl {
    async fn save(&self, city: CityModel) -> Result<CityModel, Box<dyn Error + Send + Sync>> {
        sqlx::query(
            r#"
            INSERT INTO city (id, country_id, state_id, city_code, name_l1, name_l2, name_l3, is_active, created_at, updated_at, created_by_person_id, updated_by_person_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
        )
        .bind(city.id)
        .bind(city.country_id)
        .bind(city.state_id)
        .bind(city.city_code.as_str())
        .bind(city.name_l1.as_str())
        .bind(city.name_l2.as_ref().map(|s| s.as_str()))
        .bind(city.name_l3.as_ref().map(|s| s.as_str()))
        .bind(city.is_active)
        .bind(city.created_at)
        .bind(city.updated_at)
        .bind(city.created_by_person_id)
        .bind(city.updated_by_person_id)
        .execute(&*self.pool)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO city_idx (city_id, country_id, state_id, city_code, is_active)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(city.id)
        .bind(city.country_id)
        .bind(city.state_id)
        .bind(city.city_code.as_str())
        .bind(city.is_active)
        .execute(&*self.pool)
        .await?;

        Ok(city)
    }
    async fn find_by_id(&self, _id: Uuid) -> Result<Option<CityModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_by_ids(&self, _ids: &[Uuid]) -> Result<Vec<CityModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn exists_by_id(&self, _id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_ids_by_country_id(
        &self,
        _country_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_by_country_id(
        &self,
        _country_id: Uuid,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<CityModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_ids_by_state_id(&self, _state_id: Uuid) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_by_state_id(
        &self,
        _state_id: Uuid,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<CityModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_city_by_city_code(
        &self,
        country_id: Uuid,
        city_code: &str,
    ) -> Result<Option<CityModel>, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query(
            r#"
            SELECT * FROM city WHERE country_id = $1 AND city_code = $2
            "#,
        )
        .bind(country_id)
        .bind(city_code)
        .fetch_optional(&*self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(CityModel::try_from_row(&row)?)),
            None => Ok(None),
        }
    }
    async fn find_ids_by_is_active(
        &self,
        _is_active: bool,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_by_is_active(
        &self,
        _is_active: bool,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<CityModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
}

pub struct AddressRepositoryImpl {
    pool: Arc<PgPool>,
}
impl AddressRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl AddressRepository for AddressRepositoryImpl {
    async fn save(&self, _address: AddressModel) -> Result<AddressModel, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_by_id(&self, _id: Uuid) -> Result<Option<AddressModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_by_ids(&self, _ids: &[Uuid]) -> Result<Vec<AddressModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn exists_by_id(&self, _id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_ids_by_address_type(
        &self,
        _address_type: AddressType,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_by_address_type(
        &self,
        _address_type: AddressType,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<AddressModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_ids_by_city_id(&self, _city_id: Uuid) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_by_city_id(
        &self,
        _city_id: Uuid,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<AddressModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_ids_by_is_active(&self, _is_active: bool) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_by_is_active(
        &self,
        _is_active: bool,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<AddressModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
}

impl TryFromRow<PgRow> for AddressModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(AddressModel {
            id: row.get("id"),
            address_type: row.get("address_type"),
            street_line1: get_heapless_string(row, "street_line1")?,
            street_line2: get_optional_heapless_string(row, "street_line2")?,
            street_line3: get_optional_heapless_string(row, "street_line3")?,
            street_line4: get_optional_heapless_string(row, "street_line4")?,
            city_id: row.get("city_id"),
            postal_code: get_optional_heapless_string(row, "postal_code")?,
            latitude: row.get("latitude"),
            longitude: row.get("longitude"),
            accuracy_meters: row.get("accuracy_meters"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            created_by_person_id: row.get("created_by_person_id"),
            updated_by_person_id: row.get("updated_by_person_id"),
        })
    }
}

impl TryFromRow<PgRow> for PersonModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(PersonModel {
            id: row.get("id"),
            person_type: row.get("person_type"),
            display_name: get_heapless_string(row, "display_name")?,
            external_identifier: get_optional_heapless_string(row, "external_identifier")?,
            organization_person_id: row.get("organization_person_id"),
            messaging1_id: row.get("messaging1_id"),
            messaging1_type: row.get("messaging1_type"),
            messaging2_id: row.get("messaging2_id"),
            messaging2_type: row.get("messaging2_type"),
            messaging3_id: row.get("messaging3_id"),
            messaging3_type: row.get("messaging3_type"),
            messaging4_id: row.get("messaging4_id"),
            messaging4_type: row.get("messaging4_type"),
            messaging5_id: row.get("messaging5_id"),
            messaging5_type: row.get("messaging5_type"),
            department: get_optional_heapless_string(row, "department")?,
            location_address_id: row.get("location_address_id"),
            duplicate_of_person_id: row.get("duplicate_of_person_id"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

impl TryFromRow<PgRow> for CountryModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(CountryModel {
            id: row.get("id"),
            iso2: get_heapless_string(row, "iso2")?,
            name_l1: get_heapless_string(row, "name_l1")?,
            name_l2: get_optional_heapless_string(row, "name_l2")?,
            name_l3: get_optional_heapless_string(row, "name_l3")?,
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            created_by_person_id: row.get("created_by_person_id"),
            updated_by_person_id: row.get("updated_by_person_id"),
        })
    }
}

impl TryFromRow<PgRow> for StateProvinceModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(StateProvinceModel {
            id: row.get("id"),
            country_id: row.get("country_id"),
            state_province_code: get_heapless_string(row, "state_province_code")?,
            name_l1: get_heapless_string(row, "name_l1")?,
            name_l2: get_optional_heapless_string(row, "name_l2")?,
            name_l3: get_optional_heapless_string(row, "name_l3")?,
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            created_by_person_id: row.get("created_by_person_id"),
            updated_by_person_id: row.get("updated_by_person_id"),
        })
    }
}

impl TryFromRow<PgRow> for CityModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(CityModel {
            id: row.get("id"),
            country_id: row.get("country_id"),
            state_id: row.get("state_id"),
            city_code: get_heapless_string(row, "city_code")?,
            name_l1: get_heapless_string(row, "name_l1")?,
            name_l2: get_optional_heapless_string(row, "name_l2")?,
            name_l3: get_optional_heapless_string(row, "name_l3")?,
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            created_by_person_id: row.get("created_by_person_id"),
            updated_by_person_id: row.get("updated_by_person_id"),
        })
    }
}

pub struct MessagingRepositoryImpl {
    pool: Arc<PgPool>,
}
impl MessagingRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl MessagingRepository for MessagingRepositoryImpl {
    async fn save(
        &self,
        _messaging: MessagingModel,
    ) -> Result<MessagingModel, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_by_id(
        &self,
        _id: Uuid,
    ) -> Result<Option<MessagingModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_by_ids(
        &self,
        _ids: &[Uuid],
    ) -> Result<Vec<MessagingModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn exists_by_id(&self, _id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_ids_by_messaging_type(
        &self,
        _messaging_type: MessagingType,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_by_messaging_type(
        &self,
        _messaging_type: MessagingType,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<MessagingModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_ids_by_is_active(&self, _is_active: bool) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
    async fn find_by_is_active(
        &self,
        _is_active: bool,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<MessagingModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
}

impl TryFromRow<PgRow> for MessagingModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(MessagingModel {
            id: row.get("id"),
            messaging_type: row.get("messaging_type"),
            value: get_heapless_string(row, "value")?,
            other_type: get_optional_heapless_string(row, "other_type")?,
            is_active: row.get("is_active"),
            priority: row.try_get::<Option<i16>, _>("priority")?.map(|p| p as u8),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}
use async_trait::async_trait;
use banking_db::models::person::{
    CountryModel, CountrySubdivisionModel, EntityReferenceModel, LocationModel, LocationType,
    LocalityModel, MessagingModel, PersonModel, RelationshipRole,
};
use banking_db::repository::{
    CountryRepository, CountrySubdivisionRepository, EntityReferenceRepository, LocationRepository,
    LocalityRepository, MessagingRepository, PersonRepository,
};
use sqlx::{postgres::PgRow, PgPool, Postgres, Row};
use std::error::Error;
use std::hash::Hasher;
use std::sync::Arc;
use twox_hash::XxHash64;
use uuid::Uuid;

use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};

pub struct PersonRepositoryImpl {
    pool: Arc<PgPool>,
}

impl PersonRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PersonRepository<Postgres> for PersonRepositoryImpl {
    async fn save(&self, person: PersonModel) -> Result<PersonModel, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO person (
                id, version, person_type, display_name, external_identifier, organization_person_id,
                messaging1_id, messaging1_type, messaging2_id, messaging2_type, messaging3_id, messaging3_type,
                messaging4_id, messaging4_type, messaging5_id, messaging5_type,
                department, location_id, duplicate_of_person_id, entity_reference_count, audit_log_id
            )
            VALUES ($1, $2, $3::person_type, $4, $5, $6, $7, $8::messaging_type, $9, $10::messaging_type, $11, $12::messaging_type, $13, $14::messaging_type, $15, $16, $17, $18, $19, $20, $21)
           "#,
        )
        .bind(person.id)
        .bind(person.version)
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
        .bind(person.location_id)
        .bind(person.duplicate_of_person_id)
        .bind(person.entity_reference_count)
        .bind(person.audit_log_id)
        .execute(&*self.pool)
        .await?;

        let hash = person.external_identifier.as_ref().map(|s| {
            let mut hasher = XxHash64::with_seed(0);
            hasher.write(s.as_bytes());
            hasher.finish() as i64
        });

        sqlx::query(
            r#"
            INSERT INTO person_idx (person_id, external_identifier_hash)
            VALUES ($1, $2)
            "#,
        )
        .bind(person.id)
        .bind(hash)
        .execute(&*self.pool)
        .await?;

        Ok(person)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<PersonModel>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT * FROM person WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(
                PersonModel::try_from_row(&row).map_err(|e| sqlx::Error::Decode(e))?,
            )),
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
            WHERE er.reference_external_id = $1 AND er.entity_role = $2
            "#,
        )
        .bind(entity_id.to_string())
        .bind(entity_type)
        .fetch_all(&*self.pool)
        .await?;

        let mut persons = Vec::new();
        for row in rows {
            persons.push(PersonModel::try_from_row(&row)?);
        }
        Ok(persons)
    }


    async fn mark_as_duplicate(
        &self,
        _person_id: Uuid,
        _duplicate_of_person_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn search_by_name(
        &self,
        _query: &str,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn batch_create(
        &self,
        _persons: Vec<PersonModel>,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
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
impl CountryRepository<Postgres> for CountryRepositoryImpl {
    async fn save(&self, country: CountryModel) -> Result<CountryModel, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO country (id, iso2, name_l1, name_l2, name_l3)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(country.id)
        .bind(country.iso2.as_str())
        .bind(country.name_l1.as_str())
        .bind(country.name_l2.as_ref().map(|s| s.as_str()))
        .bind(country.name_l3.as_ref().map(|s| s.as_str()))
        .execute(&*self.pool)
        .await?;

        Ok(country)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<CountryModel>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT * FROM country WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(
                CountryModel::try_from_row(&row).map_err(|e| sqlx::Error::Decode(e))?,
            )),
            None => Ok(None),
        }
    }

    async fn find_by_iso2(
        &self,
        iso2: &str,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<CountryModel>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM country WHERE iso2 = $1
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(iso2)
        .bind(page_size)
        .bind((page - 1) * page_size)
        .fetch_all(&*self.pool)
        .await?;

        let mut countries = Vec::new();
        for row in rows {
            countries.push(CountryModel::try_from_row(&row).map_err(|e| sqlx::Error::Decode(e))?);
        }
        Ok(countries)
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<CountryModel>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM country WHERE id = ANY($1)
            "#,
        )
        .bind(ids)
        .fetch_all(&*self.pool)
        .await?;

        let mut countries = Vec::new();
        for row in rows {
            countries.push(CountryModel::try_from_row(&row).map_err(|e| sqlx::Error::Decode(e))?);
        }
        Ok(countries)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM country WHERE id = $1)",
            id
        )
        .fetch_one(&*self.pool)
        .await?;
        Ok(exists.unwrap_or(false))
    }

    async fn find_ids_by_iso2(
        &self,
        iso2: &str,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = sqlx::query_scalar(
            r#"
            SELECT id FROM country WHERE iso2 = $1
            "#,
        )
        .bind(iso2)
        .fetch_all(&*self.pool)
        .await?;
        Ok(ids)
    }
}

pub struct CountrySubdivisionRepositoryImpl {
    pool: Arc<PgPool>,
}

impl CountrySubdivisionRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CountrySubdivisionRepository<Postgres> for CountrySubdivisionRepositoryImpl {
    async fn save(
        &self,
        country_subdivision: CountrySubdivisionModel,
    ) -> Result<CountrySubdivisionModel, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO country_subdivision (id, country_id, code, name_l1, name_l2, name_l3)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(country_subdivision.id)
        .bind(country_subdivision.country_id)
        .bind(country_subdivision.code.as_str())
        .bind(country_subdivision.name_l1.as_str())
        .bind(country_subdivision.name_l2.as_ref().map(|s| s.as_str()))
        .bind(country_subdivision.name_l3.as_ref().map(|s| s.as_str()))
        .execute(&*self.pool)
        .await?;

        Ok(country_subdivision)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<CountrySubdivisionModel>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT * FROM country_subdivision WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(
                CountrySubdivisionModel::try_from_row(&row)
                    .map_err(|e| sqlx::Error::Decode(e))?,
            )),
            None => Ok(None),
        }
    }

    async fn find_by_country_id(
        &self,
        country_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<CountrySubdivisionModel>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM country_subdivision WHERE country_id = $1
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(country_id)
        .bind(page_size)
        .bind((page - 1) * page_size)
        .fetch_all(&*self.pool)
        .await?;

        let mut subdivisions = Vec::new();
        for row in rows {
            subdivisions.push(
                CountrySubdivisionModel::try_from_row(&row)
                    .map_err(|e| sqlx::Error::Decode(e))?,
            );
        }
        Ok(subdivisions)
    }

    async fn find_by_code(
        &self,
        country_id: Uuid,
        code: &str,
    ) -> Result<Option<CountrySubdivisionModel>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT * FROM country_subdivision WHERE country_id = $1 AND code = $2
            "#,
        )
        .bind(country_id)
        .bind(code)
        .fetch_optional(&*self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(
                CountrySubdivisionModel::try_from_row(&row)
                    .map_err(|e| sqlx::Error::Decode(e))?,
            )),
            None => Ok(None),
        }
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<CountrySubdivisionModel>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM country_subdivision WHERE id = ANY($1)
            "#,
        )
        .bind(ids)
        .fetch_all(&*self.pool)
        .await?;

        let mut subdivisions = Vec::new();
        for row in rows {
            subdivisions.push(CountrySubdivisionModel::try_from_row(&row)?);
        }
        Ok(subdivisions)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM country_subdivision WHERE id = $1)",
            id
        )
        .fetch_one(&*self.pool)
        .await?;
        Ok(exists.unwrap_or(false))
    }

    async fn find_ids_by_country_id(
        &self,
        country_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = sqlx::query_scalar(
            r#"
            SELECT id FROM country_subdivision WHERE country_id = $1
            "#,
        )
        .bind(country_id)
        .fetch_all(&*self.pool)
        .await?;
        Ok(ids)
    }
}

pub struct LocalityRepositoryImpl {
    pool: Arc<PgPool>,
}

impl LocalityRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LocalityRepository<Postgres> for LocalityRepositoryImpl {
    async fn save(&self, locality: LocalityModel) -> Result<LocalityModel, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO locality (id, country_subdivision_id, code, name_l1, name_l2, name_l3)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(locality.id)
        .bind(locality.country_subdivision_id)
        .bind(locality.code.as_str())
        .bind(locality.name_l1.as_str())
        .bind(locality.name_l2.as_ref().map(|s| s.as_str()))
        .bind(locality.name_l3.as_ref().map(|s| s.as_str()))
        .execute(&*self.pool)
        .await?;

        Ok(locality)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<LocalityModel>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT * FROM locality WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(
                LocalityModel::try_from_row(&row).map_err(|e| sqlx::Error::Decode(e))?,
            )),
            None => Ok(None),
        }
    }

    async fn find_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<LocalityModel>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM locality WHERE country_subdivision_id = $1
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(country_subdivision_id)
        .bind(page_size)
        .bind((page - 1) * page_size)
        .fetch_all(&*self.pool)
        .await?;

        let mut localities = Vec::new();
        for row in rows {
            localities
                .push(LocalityModel::try_from_row(&row).map_err(|e| sqlx::Error::Decode(e))?);
        }
        Ok(localities)
    }

    async fn find_by_code(
        &self,
        country_id: Uuid,
        code: &str,
    ) -> Result<Option<LocalityModel>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT l.* FROM locality l
            INNER JOIN country_subdivision cs ON l.country_subdivision_id = cs.id
            WHERE cs.country_id = $1 AND l.code = $2
            "#,
        )
        .bind(country_id)
        .bind(code)
        .fetch_optional(&*self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(
                LocalityModel::try_from_row(&row).map_err(|e| sqlx::Error::Decode(e))?,
            )),
            None => Ok(None),
        }
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<LocalityModel>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM locality WHERE id = ANY($1)
            "#,
        )
        .bind(ids)
        .fetch_all(&*self.pool)
        .await?;

        let mut localities = Vec::new();
        for row in rows {
            localities.push(LocalityModel::try_from_row(&row)?);
        }
        Ok(localities)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM locality WHERE id = $1)",
            id
        )
        .fetch_one(&*self.pool)
        .await?;
        Ok(exists.unwrap_or(false))
    }

    async fn find_ids_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = sqlx::query_scalar(
            r#"
            SELECT id FROM locality WHERE country_subdivision_id = $1
            "#,
        )
        .bind(country_subdivision_id)
        .fetch_all(&*self.pool)
        .await?;
        Ok(ids)
    }
}

pub struct LocationRepositoryImpl {
    pool: Arc<PgPool>,
}

impl LocationRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LocationRepository<Postgres> for LocationRepositoryImpl {
    async fn save(&self, location: LocationModel) -> Result<LocationModel, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO location (id, version, street_line1, street_line2, street_line3, street_line4, locality_id, postal_code, latitude, longitude, accuracy_meters, location_type, audit_log_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12::location_type, $13)
            "#,
        )
        .bind(location.id)
        .bind(location.version)
        .bind(location.street_line1.as_str())
        .bind(location.street_line2.as_ref().map(|s| s.as_str()))
        .bind(location.street_line3.as_ref().map(|s| s.as_str()))
        .bind(location.street_line4.as_ref().map(|s| s.as_str()))
        .bind(location.locality_id)
        .bind(location.postal_code.as_ref().map(|s| s.as_str()))
        .bind(location.latitude)
        .bind(location.longitude)
        .bind(location.accuracy_meters)
        .bind(location.location_type)
        .bind(location.audit_log_id)
        .execute(&*self.pool)
        .await?;

        Ok(location)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<LocationModel>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT * FROM location WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(
                LocationModel::try_from_row(&row).map_err(|e| sqlx::Error::Decode(e))?,
            )),
            None => Ok(None),
        }
    }

    async fn find_ids_by_street_line1(
        &self,
        street_line1: &str,
    ) -> Result<Vec<Uuid>, sqlx::Error> {
        let ids = sqlx::query_scalar(
            r#"
            SELECT id FROM location WHERE street_line1 = $1
            "#,
        )
        .bind(street_line1)
        .fetch_all(&*self.pool)
        .await?;
        Ok(ids)
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<LocationModel>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM location WHERE id = ANY($1)
            "#,
        )
        .bind(ids)
        .fetch_all(&*self.pool)
        .await?;

        let mut locations = Vec::new();
        for row in rows {
            locations
                .push(LocationModel::try_from_row(&row).map_err(|e| sqlx::Error::Decode(e))?);
        }
        Ok(locations)
    }

    async fn find_by_locality_id(
        &self,
        locality_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<LocationModel>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM location WHERE locality_id = $1
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(locality_id)
        .bind(page_size)
        .bind((page - 1) * page_size)
        .fetch_all(&*self.pool)
        .await?;

        let mut locations = Vec::new();
        for row in rows {
            locations
                .push(LocationModel::try_from_row(&row).map_err(|e| sqlx::Error::Decode(e))?);
        }
        Ok(locations)
    }

    async fn find_by_type_and_locality(
        &self,
        location_type: LocationType,
        locality_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<LocationModel>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM location WHERE location_type = $1 AND locality_id = $2
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(location_type)
        .bind(locality_id)
        .bind(page_size)
        .bind((page - 1) * page_size)
        .fetch_all(&*self.pool)
        .await?;

        let mut locations = Vec::new();
        for row in rows {
            locations
                .push(LocationModel::try_from_row(&row).map_err(|e| sqlx::Error::Decode(e))?);
        }
        Ok(locations)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM location WHERE id = $1)",
            id
        )
        .fetch_one(&*self.pool)
        .await?;
        Ok(exists.unwrap_or(false))
    }

    async fn find_ids_by_location_type(
        &self,
        location_type: LocationType,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = sqlx::query_scalar(
            r#"
            SELECT id FROM location WHERE location_type = $1
            "#,
        )
        .bind(location_type)
        .fetch_all(&*self.pool)
        .await?;
        Ok(ids)
    }

    async fn find_ids_by_locality_id(
        &self,
        locality_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = sqlx::query_scalar(
            r#"
            SELECT id FROM location WHERE locality_id = $1
            "#,
        )
        .bind(locality_id)
        .fetch_all(&*self.pool)
        .await?;
        Ok(ids)
    }
}

impl TryFromRow<PgRow> for LocationModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(LocationModel {
            id: row.get("id"),
            location_type: row.get("location_type"),
            street_line1: get_heapless_string(row, "street_line1")?,
            street_line2: get_optional_heapless_string(row, "street_line2")?,
            street_line3: get_optional_heapless_string(row, "street_line3")?,
            street_line4: get_optional_heapless_string(row, "street_line4")?,
            locality_id: row.get("locality_id"),
            postal_code: get_optional_heapless_string(row, "postal_code")?,
            latitude: row.get("latitude"),
            longitude: row.get("longitude"),
            accuracy_meters: row.get("accuracy_meters"),
            version: row.get("version"),
            audit_log_id: row.get("audit_log_id"),
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
            location_id: row.get("location_id"),
            duplicate_of_person_id: row.get("duplicate_of_person_id"),
            version: row.get("version"),
            entity_reference_count: row.get("entity_reference_count"),
            audit_log_id: row.get("audit_log_id"),
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
        })
    }
}

impl TryFromRow<PgRow> for CountrySubdivisionModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(CountrySubdivisionModel {
            id: row.get("id"),
            country_id: row.get("country_id"),
            code: get_heapless_string(row, "code")?,
            name_l1: get_heapless_string(row, "name_l1")?,
            name_l2: get_optional_heapless_string(row, "name_l2")?,
            name_l3: get_optional_heapless_string(row, "name_l3")?,
        })
    }
}

impl TryFromRow<PgRow> for LocalityModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(LocalityModel {
            id: row.get("id"),
            country_subdivision_id: row.get("country_subdivision_id"),
            code: get_heapless_string(row, "code")?,
            name_l1: get_heapless_string(row, "name_l1")?,
            name_l2: get_optional_heapless_string(row, "name_l2")?,
            name_l3: get_optional_heapless_string(row, "name_l3")?,
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
impl MessagingRepository<Postgres> for MessagingRepositoryImpl {
    async fn save(
        &self,
        messaging: MessagingModel,
    ) -> Result<MessagingModel, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO messaging (id, version, messaging_type, value, other_type, audit_log_id)
            VALUES ($1, $2, $3::messaging_type, $4, $5, $6)
            "#,
        )
        .bind(messaging.id)
        .bind(messaging.version)
        .bind(messaging.messaging_type)
        .bind(messaging.value.as_str())
        .bind(messaging.other_type.as_ref().map(|s| s.as_str()))
        .bind(messaging.audit_log_id)
        .execute(&*self.pool)
        .await?;

        let mut hasher = XxHash64::with_seed(0);
        hasher.write(messaging.value.as_bytes());
        let hash = hasher.finish() as i64;

        sqlx::query(
            r#"
            INSERT INTO messaging_idx (messaging_id, value_hash)
            VALUES ($1, $2)
            "#,
        )
        .bind(messaging.id)
        .bind(hash)
        .execute(&*self.pool)
        .await?;

        Ok(messaging)
    }
    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<MessagingModel>, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query("SELECT * FROM messaging WHERE id = $1")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await?;
        match row {
            Some(row) => Ok(Some(MessagingModel::try_from_row(&row)?)),
            None => Ok(None),
        }
    }
    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<MessagingModel>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query("SELECT * FROM messaging WHERE id = ANY($1)")
            .bind(ids)
            .fetch_all(&*self.pool)
            .await?;
        let mut messagings = Vec::new();
        for row in rows {
            messagings.push(MessagingModel::try_from_row(&row)?);
        }
        Ok(messagings)
    }
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let exists = sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM messaging WHERE id = $1)", id)
            .fetch_one(&*self.pool)
            .await?;
        Ok(exists.unwrap_or(false))
    }
    async fn find_ids_by_value(
        &self,
        value: &str,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let mut hasher = XxHash64::with_seed(0);
        hasher.write(value.as_bytes());
        let hash = hasher.finish() as i64;

        let ids = sqlx::query_scalar("SELECT messaging_id FROM messaging_idx WHERE value_hash = $1")
            .bind(hash)
            .fetch_all(&*self.pool)
            .await?;
        Ok(ids)
    }
}

impl TryFromRow<PgRow> for MessagingModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(MessagingModel {
            id: row.get("id"),
            messaging_type: row.get("messaging_type"),
            value: get_heapless_string(row, "value")?,
            other_type: get_optional_heapless_string(row, "other_type")?,
            version: row.get("version"),
            audit_log_id: row.get("audit_log_id"),
        })
    }
}

pub struct EntityReferenceRepositoryImpl {
    pool: Arc<PgPool>,
}

impl EntityReferenceRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EntityReferenceRepository<Postgres> for EntityReferenceRepositoryImpl {
    async fn save(
        &self,
        entity_ref: EntityReferenceModel,
    ) -> Result<EntityReferenceModel, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO entity_reference (id, person_id, entity_role, reference_external_id, reference_details_l1, reference_details_l2, reference_details_l3)
            VALUES ($1, $2, $3::person_entity_type, $4, $5, $6, $7)
            "#,
        )
        .bind(entity_ref.id)
        .bind(entity_ref.person_id)
        .bind(entity_ref.entity_role)
        .bind(entity_ref.reference_external_id.as_str())
        .bind(
            entity_ref
                .reference_details_l1
                .as_ref()
                .map(|s| s.as_str()),
        )
        .bind(
            entity_ref
                .reference_details_l2
                .as_ref()
                .map(|s| s.as_str()),
        )
        .bind(
            entity_ref
                .reference_details_l3
                .as_ref()
                .map(|s| s.as_str()),
        )
        .execute(&*self.pool)
        .await?;

        Ok(entity_ref)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<EntityReferenceModel>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT * FROM entity_reference WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(
                EntityReferenceModel::try_from_row(&row)
                    .map_err(|e| sqlx::Error::Decode(e))?,
            )),
            None => Ok(None),
        }
    }

    async fn find_by_person_id(
        &self,
        person_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<EntityReferenceModel>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM entity_reference WHERE person_id = $1
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(person_id)
        .bind(page_size)
        .bind((page - 1) * page_size)
        .fetch_all(&*self.pool)
        .await?;

        let mut refs = Vec::new();
        for row in rows {
            refs.push(
                EntityReferenceModel::try_from_row(&row)
                    .map_err(|e| sqlx::Error::Decode(e))?,
            );
        }
        Ok(refs)
    }

    async fn find_by_reference_external_id(
        &self,
        reference_external_id: &str,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<EntityReferenceModel>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM entity_reference WHERE reference_external_id = $1
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(reference_external_id)
        .bind(page_size)
        .bind((page - 1) * page_size)
        .fetch_all(&*self.pool)
        .await?;

        let mut refs = Vec::new();
        for row in rows {
            refs.push(
                EntityReferenceModel::try_from_row(&row)
                    .map_err(|e| sqlx::Error::Decode(e))?,
            );
        }
        Ok(refs)
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<EntityReferenceModel>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM entity_reference WHERE id = ANY($1)
            "#,
        )
        .bind(ids)
        .fetch_all(&*self.pool)
        .await?;

        let mut refs = Vec::new();
        for row in rows {
            refs.push(EntityReferenceModel::try_from_row(&row)?);
        }
        Ok(refs)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM entity_reference WHERE id = $1)",
            id
        )
        .fetch_one(&*self.pool)
        .await?;
        Ok(exists.unwrap_or(false))
    }

    async fn find_ids_by_person_id(
        &self,
        person_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = sqlx::query_scalar(
            r#"
            SELECT id FROM entity_reference WHERE person_id = $1
            "#,
        )
        .bind(person_id)
        .fetch_all(&*self.pool)
        .await?;
        Ok(ids)
    }
}

impl TryFromRow<PgRow> for EntityReferenceModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(EntityReferenceModel {
            id: row.get("id"),
            version: row.get("version"),
            person_id: row.get("person_id"),
            entity_role: row.get("entity_role"),
            reference_external_id: get_heapless_string(row, "reference_external_id")?,
            reference_details_l1: get_optional_heapless_string(row, "reference_details_l1")?,
            reference_details_l2: get_optional_heapless_string(row, "reference_details_l2")?,
            reference_details_l3: get_optional_heapless_string(row, "reference_details_l3")?,
            audit_log_id: row.get("audit_log_id"),
        })
    }
}
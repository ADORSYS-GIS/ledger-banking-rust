use async_trait::async_trait;
use banking_db::models::person::{
    AddressModel, AddressType, CityModel, CountryModel, EntityReferenceModel, MessagingModel,
    MessagingType, PersonModel, PersonType, RelationshipRole, StateProvinceModel,
};
use banking_db::repository::{
    AddressRepository, CityRepository, CountryRepository, EntityReferenceRepository,
    MessagingRepository, PersonRepository, StateProvinceRepository,
};
use crate::simple_models::PersonModelSqlx;
use sqlx::{PgPool, Row};
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

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
        sqlx::query!(
            r#"
            INSERT INTO person (
                id, person_type, display_name, external_identifier, organization_person_id,
                messaging1_id, messaging1_type, messaging2_id, messaging2_type, messaging3_id, messaging3_type,
                messaging4_id, messaging4_type, messaging5_id, messaging5_type,
                department, location_address_id, duplicate_of_person_id, is_active, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)
            ON CONFLICT (id) DO UPDATE SET
                person_type = EXCLUDED.person_type,
                display_name = EXCLUDED.display_name,
                external_identifier = EXCLUDED.external_identifier,
                organization_person_id = EXCLUDED.organization_person_id,
                messaging1_id = EXCLUDED.messaging1_id,
                messaging1_type = EXCLUDED.messaging1_type,
                messaging2_id = EXCLUDED.messaging2_id,
                messaging2_type = EXCLUDED.messaging2_type,
                messaging3_id = EXCLUDED.messaging3_id,
                messaging3_type = EXCLUDED.messaging3_type,
                messaging4_id = EXCLUDED.messaging4_id,
                messaging4_type = EXCLUDED.messaging4_type,
                messaging5_id = EXCLUDED.messaging5_id,
                messaging5_type = EXCLUDED.messaging5_type,
                department = EXCLUDED.department,
                location_address_id = EXCLUDED.location_address_id,
                duplicate_of_person_id = EXCLUDED.duplicate_of_person_id,
                is_active = EXCLUDED.is_active,
                updated_at = EXCLUDED.updated_at
            "#,
            person.id,
            person.person_type as _,
            person.display_name.as_str(),
            person.external_identifier.as_ref().map(|s| s.as_str()),
            person.organization_person_id,
            person.messaging1_id,
            person.messaging1_type as _,
            person.messaging2_id,
            person.messaging2_type as _,
            person.messaging3_id,
            person.messaging3_type as _,
            person.messaging4_id,
            person.messaging4_type as _,
            person.messaging5_id,
            person.messaging5_type as _,
            person.department.as_ref().map(|s| s.as_str()),
            person.location_address_id,
            person.duplicate_of_person_id,
            person.is_active,
            person.created_at,
            person.updated_at
        )
        .execute(&*self.pool)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO person_idx (person_id, person_type, is_active)
            VALUES ($1, $2, $3)
            ON CONFLICT (person_id) DO UPDATE SET
                person_type = EXCLUDED.person_type,
                is_active = EXCLUDED.is_active
            "#,
            person.id,
            person.person_type as _,
            person.is_active
        )
        .execute(&*self.pool)
        .await?;

        Ok(person)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<PersonModel>, Box<dyn Error + Send + Sync>> {
        let person_sqlx = sqlx::query_as!(
            PersonModelSqlx,
            r#"
            SELECT id, person_type as "person_type: _", display_name, external_identifier, organization_person_id,
                messaging1_id, messaging1_type as "messaging1_type: _", messaging2_id, messaging2_type as "messaging2_type: _",
                messaging3_id, messaging3_type as "messaging3_type: _", messaging4_id, messaging4_type as "messaging4_type: _",
                messaging5_id, messaging5_type as "messaging5_type: _", department, location_address_id,
                duplicate_of_person_id, is_active, created_at, updated_at
            FROM person WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*self.pool)
        .await?;

        if let Some(p) = person_sqlx {
            Ok(Some(PersonModel {
                id: p.id,
                person_type: p.person_type,
                display_name: heapless::String::from_str(&p.display_name).unwrap(),
                external_identifier: p
                    .external_identifier
                    .map(|s| heapless::String::from_str(&s).unwrap()),
                organization_person_id: p.organization_person_id,
                messaging1_id: p.messaging1_id,
                messaging1_type: p.messaging1_type,
                messaging2_id: p.messaging2_id,
                messaging2_type: p.messaging2_type,
                messaging3_id: p.messaging3_id,
                messaging3_type: p.messaging3_type,
                messaging4_id: p.messaging4_id,
                messaging4_type: p.messaging4_type,
                messaging5_id: p.messaging5_id,
                messaging5_type: p.messaging5_type,
                department: p.department.map(|s| heapless::String::from_str(&s).unwrap()),
                location_address_id: p.location_address_id,
                duplicate_of_person_id: p.duplicate_of_person_id,
                is_active: p.is_active,
                created_at: p.created_at,
                updated_at: p.updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        let persons_sqlx = sqlx::query_as!(
            PersonModelSqlx,
            r#"
            SELECT id, person_type as "person_type: _", display_name, external_identifier, organization_person_id,
                messaging1_id, messaging1_type as "messaging1_type: _", messaging2_id, messaging2_type as "messaging2_type: _",
                messaging3_id, messaging3_type as "messaging3_type: _", messaging4_id, messaging4_type as "messaging4_type: _",
                messaging5_id, messaging5_type as "messaging5_type: _", department, location_address_id,
                duplicate_of_person_id, is_active, created_at, updated_at
            FROM person WHERE id = ANY($1)
            "#,
            ids
        )
        .fetch_all(&*self.pool)
        .await?;

        let persons = persons_sqlx
            .into_iter()
            .map(|p| PersonModel {
                id: p.id,
                person_type: p.person_type,
                display_name: heapless::String::from_str(&p.display_name).unwrap(),
                external_identifier: p
                    .external_identifier
                    .map(|s| heapless::String::from_str(&s).unwrap()),
                organization_person_id: p.organization_person_id,
                messaging1_id: p.messaging1_id,
                messaging1_type: p.messaging1_type,
                messaging2_id: p.messaging2_id,
                messaging2_type: p.messaging2_type,
                messaging3_id: p.messaging3_id,
                messaging3_type: p.messaging3_type,
                messaging4_id: p.messaging4_id,
                messaging4_type: p.messaging4_type,
                messaging5_id: p.messaging5_id,
                messaging5_type: p.messaging5_type,
                department: p.department.map(|s| heapless::String::from_str(&s).unwrap()),
                location_address_id: p.location_address_id,
                duplicate_of_person_id: p.duplicate_of_person_id,
                is_active: p.is_active,
                created_at: p.created_at,
                updated_at: p.updated_at,
            })
            .collect();

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
        let ids = sqlx::query_scalar!(
            "SELECT person_id FROM person_idx WHERE person_type = $1",
            person_type as _
        )
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
        let ids = sqlx::query_scalar!(
            "SELECT person_id FROM person_idx WHERE is_active = $1",
            is_active
        )
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

    async fn get_by_external_identifier(
        &self,
        identifier: &str,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        let persons_sqlx = sqlx::query_as!(
            PersonModelSqlx,
            r#"
            SELECT id, person_type as "person_type: _", display_name, external_identifier, organization_person_id,
                messaging1_id, messaging1_type as "messaging1_type: _", messaging2_id, messaging2_type as "messaging2_type: _",
                messaging3_id, messaging3_type as "messaging3_type: _", messaging4_id, messaging4_type as "messaging4_type: _",
                messaging5_id, messaging5_type as "messaging5_type: _", department, location_address_id,
                duplicate_of_person_id, is_active, created_at, updated_at
            FROM person WHERE external_identifier = $1
            "#,
            identifier
        )
        .fetch_all(&*self.pool)
        .await?;

        let persons = persons_sqlx
            .into_iter()
            .map(|p| PersonModel {
                id: p.id,
                person_type: p.person_type,
                display_name: heapless::String::from_str(&p.display_name).unwrap(),
                external_identifier: p
                    .external_identifier
                    .map(|s| heapless::String::from_str(&s).unwrap()),
                organization_person_id: p.organization_person_id,
                messaging1_id: p.messaging1_id,
                messaging1_type: p.messaging1_type,
                messaging2_id: p.messaging2_id,
                messaging2_type: p.messaging2_type,
                messaging3_id: p.messaging3_id,
                messaging3_type: p.messaging3_type,
                messaging4_id: p.messaging4_id,
                messaging4_type: p.messaging4_type,
                messaging5_id: p.messaging5_id,
                messaging5_type: p.messaging5_type,
                department: p.department.map(|s| heapless::String::from_str(&s).unwrap()),
                location_address_id: p.location_address_id,
                duplicate_of_person_id: p.duplicate_of_person_id,
                is_active: p.is_active,
                created_at: p.created_at,
                updated_at: p.updated_at,
            })
            .collect();

        Ok(persons)
    }

    async fn get_by_entity_reference(
        &self,
        entity_id: Uuid,
        entity_type: RelationshipRole,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        let persons_sqlx = sqlx::query_as!(
            PersonModelSqlx,
            r#"
            SELECT p.id, p.person_type as "person_type: _", p.display_name, p.external_identifier, p.organization_person_id,
                p.messaging1_id, p.messaging1_type as "messaging1_type: _", p.messaging2_id, p.messaging2_type as "messaging2_type: _",
                p.messaging3_id, p.messaging3_type as "messaging3_type: _", p.messaging4_id, p.messaging4_type as "messaging4_type: _",
                p.messaging5_id, p.messaging5_type as "messaging5_type: _", p.department, p.location_address_id,
                p.duplicate_of_person_id, p.is_active, p.created_at, p.updated_at
            FROM person p
            INNER JOIN entity_reference er ON p.id = er.person_id
            WHERE er.id = $1 AND er.entity_role = $2
            "#,
            entity_id,
            entity_type as _
        )
        .fetch_all(&*self.pool)
        .await?;

        let persons = persons_sqlx
            .into_iter()
            .map(|p| PersonModel {
                id: p.id,
                person_type: p.person_type,
                display_name: heapless::String::from_str(&p.display_name).unwrap(),
                external_identifier: p
                    .external_identifier
                    .map(|s| heapless::String::from_str(&s).unwrap()),
                organization_person_id: p.organization_person_id,
                messaging1_id: p.messaging1_id,
                messaging1_type: p.messaging1_type,
                messaging2_id: p.messaging2_id,
                messaging2_type: p.messaging2_type,
                messaging3_id: p.messaging3_id,
                messaging3_type: p.messaging3_type,
                messaging4_id: p.messaging4_id,
                messaging4_type: p.messaging4_type,
                messaging5_id: p.messaging5_id,
                messaging5_type: p.messaging5_type,
                department: p.department.map(|s| heapless::String::from_str(&s).unwrap()),
                location_address_id: p.location_address_id,
                duplicate_of_person_id: p.duplicate_of_person_id,
                is_active: p.is_active,
                created_at: p.created_at,
                updated_at: p.updated_at,
            })
            .collect();

        Ok(persons)
    }

    async fn find_or_create(
        &self,
        display_name: &str,
        person_type: PersonType,
        external_identifier: Option<&str>,
    ) -> Result<PersonModel, Box<dyn Error + Send + Sync>> {
        if let Some(ext_id) = external_identifier {
            let existing = self.get_by_external_identifier(ext_id).await?;
            if let Some(person) = existing.into_iter().find(|p| p.person_type == person_type) {
                return Ok(person);
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
        let persons_sqlx = sqlx::query_as!(
            PersonModelSqlx,
            r#"
            SELECT id, person_type as "person_type: _", display_name, external_identifier, organization_person_id,
                messaging1_id, messaging1_type as "messaging1_type: _", messaging2_id, messaging2_type as "messaging2_type: _",
                messaging3_id, messaging3_type as "messaging3_type: _", messaging4_id, messaging4_type as "messaging4_type: _",
                messaging5_id, messaging5_type as "messaging5_type: _", department, location_address_id,
                duplicate_of_person_id, is_active, created_at, updated_at
            FROM person WHERE display_name ILIKE $1
            "#,
            format!("%{query}%")
        )
        .fetch_all(&*self.pool)
        .await?;

        let persons = persons_sqlx
            .into_iter()
            .map(|p| PersonModel {
                id: p.id,
                person_type: p.person_type,
                display_name: heapless::String::from_str(&p.display_name).unwrap(),
                external_identifier: p
                    .external_identifier
                    .map(|s| heapless::String::from_str(&s).unwrap()),
                organization_person_id: p.organization_person_id,
                messaging1_id: p.messaging1_id,
                messaging1_type: p.messaging1_type,
                messaging2_id: p.messaging2_id,
                messaging2_type: p.messaging2_type,
                messaging3_id: p.messaging3_id,
                messaging3_type: p.messaging3_type,
                messaging4_id: p.messaging4_id,
                messaging4_type: p.messaging4_type,
                messaging5_id: p.messaging5_id,
                messaging5_type: p.messaging5_type,
                department: p.department.map(|s| heapless::String::from_str(&s).unwrap()),
                location_address_id: p.location_address_id,
                duplicate_of_person_id: p.duplicate_of_person_id,
                is_active: p.is_active,
                created_at: p.created_at,
                updated_at: p.updated_at,
            })
            .collect();

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
        country: CountryModel,
    ) -> Result<CountryModel, Box<dyn Error + Send + Sync>> {
        // Implementation for save
        todo!()
    }
    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<CountryModel>, Box<dyn Error + Send + Sync>> {
        // Implementation for find_by_id
        todo!()
    }
    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<CountryModel>, Box<dyn Error + Send + Sync>> {
        // Implementation for find_by_ids
        todo!()
    }
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        // Implementation for exists_by_id
        todo!()
    }
    async fn find_ids_by_iso2(&self, iso2: &str) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        // Implementation for find_ids_by_iso2
        todo!()
    }
    async fn find_by_iso2(
        &self,
        iso2: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<CountryModel>, Box<dyn Error + Send + Sync>> {
        // Implementation for find_by_iso2
        todo!()
    }
    async fn find_ids_by_is_active(
        &self,
        is_active: bool,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        // Implementation for find_ids_by_is_active
        todo!()
    }
    async fn find_by_is_active(
        &self,
        is_active: bool,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<CountryModel>, Box<dyn Error + Send + Sync>> {
        // Implementation for find_by_is_active
        todo!()
    }
}
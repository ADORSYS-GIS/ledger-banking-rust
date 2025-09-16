use async_trait::async_trait;
use banking_api::domain::person::{Person, PersonType};
use banking_db::models::person::{PersonAuditModel, PersonIdxModel, PersonModel};
use banking_db::repository::person::person_repository::{PersonRepository, PersonRepositoryError, PersonResult};
use heapless::String as HeaplessString;
use std::sync::Mutex;
use uuid::Uuid;
use sqlx::Postgres;

#[derive(Default)]
pub struct MockPersonRepository {
    persons: Mutex<Vec<PersonModel>>,
    person_ixes: Mutex<Vec<PersonIdxModel>>,
    person_audits: Mutex<Vec<PersonAuditModel>>,
}

#[async_trait]
impl PersonRepository<Postgres> for MockPersonRepository {
    async fn save(&self, person: PersonModel, audit_log_id: Uuid) -> PersonResult<PersonModel> {
        self.persons.lock().unwrap().push(person.clone());
        // In a real scenario, we'd create a proper hash and version.
        let person_idx = PersonIdxModel {
            person_id: person.id,
            external_identifier_hash: None,
            organization_person_id: person.organization_person_id,
            duplicate_of_person_id: person.duplicate_of_person_id,
            version: 0,
            hash: 0,
        };
        self.person_ixes.lock().unwrap().push(person_idx);

        let person_audit = PersonAuditModel {
            person_id: person.id,
            version: 0,
            hash: 0,
            person_type: person.person_type,
            display_name: person.display_name.clone(),
            external_identifier: person.external_identifier.clone(),
            entity_reference_count: person.entity_reference_count,
            organization_person_id: person.organization_person_id,
            messaging1_id: person.messaging1_id,
            messaging1_type: person.messaging1_type,
            messaging2_id: person.messaging2_id,
            messaging2_type: person.messaging2_type,
            messaging3_id: person.messaging3_id,
            messaging3_type: person.messaging3_type,
            messaging4_id: person.messaging4_id,
            messaging4_type: person.messaging4_type,
            messaging5_id: person.messaging5_id,
            messaging5_type: person.messaging5_type,
            department: person.department.clone(),
            location_id: person.location_id,
            duplicate_of_person_id: person.duplicate_of_person_id,
            audit_log_id,
        };
        self.person_audits.lock().unwrap().push(person_audit);

        Ok(person)
    }

    async fn load(&self, id: Uuid) -> PersonResult<PersonModel> {
        match self
            .persons
            .lock()
            .unwrap()
            .iter()
            .find(|p| p.id == id)
            .cloned()
        {
            Some(person) => Ok(person),
            None => Err(PersonRepositoryError::from(sqlx::Error::RowNotFound)),
        }
    }

    async fn find_by_id(&self, id: Uuid) -> PersonResult<Option<PersonIdxModel>> {
        Ok(self
            .person_ixes
            .lock()
            .unwrap()
            .iter()
            .find(|p| p.person_id == id)
            .cloned())
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> PersonResult<Vec<PersonIdxModel>> {
        let person_ixes = self
            .person_ixes
            .lock()
            .unwrap()
            .iter()
            .filter(|p| ids.contains(&p.person_id))
            .cloned()
            .collect();
        Ok(person_ixes)
    }

    async fn exists_by_id(&self, id: Uuid) -> PersonResult<bool> {
        Ok(self
            .person_ixes
            .lock()
            .unwrap()
            .iter()
            .any(|p| p.person_id == id))
    }

    async fn exist_by_ids(&self, ids: &[Uuid]) -> PersonResult<Vec<(Uuid, bool)>> {
        let person_ixes = self.person_ixes.lock().unwrap();
        let result = ids
            .iter()
            .map(|id| {
                (
                    *id,
                    person_ixes.iter().any(|p_idx| &p_idx.person_id == id),
                )
            })
            .collect();
        Ok(result)
    }

    async fn get_ids_by_external_identifier(&self, identifier: &str) -> PersonResult<Vec<Uuid>> {
        let persons = self.persons.lock().unwrap();
        let ids = persons
            .iter()
            .filter(|p| p.external_identifier.as_deref() == Some(identifier))
            .map(|p| p.id)
            .collect();
        Ok(ids)
    }

    async fn get_by_external_identifier(
        &self,
        identifier: &str,
    ) -> PersonResult<Vec<PersonIdxModel>> {
        let persons = self.persons.lock().unwrap();
        let person_ixes = self.person_ixes.lock().unwrap();
        let ids: Vec<Uuid> = persons
            .iter()
            .filter(|p| p.external_identifier.as_deref() == Some(identifier))
            .map(|p| p.id)
            .collect();
        let result = person_ixes
            .iter()
            .filter(|p| ids.contains(&p.person_id))
            .cloned()
            .collect();
        Ok(result)
    }

    async fn find_by_duplicate_of_person_id(
        &self,
        person_id: Uuid,
    ) -> PersonResult<Vec<PersonIdxModel>> {
        let person_ixes = self.person_ixes.lock().unwrap();
        let result = person_ixes
            .iter()
            .filter(|p| p.duplicate_of_person_id == Some(person_id))
            .cloned()
            .collect();
        Ok(result)
    }

    async fn find_by_organization_person_id(
        &self,
        person_id: Uuid,
    ) -> PersonResult<Vec<PersonIdxModel>> {
        let person_ixes = self.person_ixes.lock().unwrap();
        let result = person_ixes
            .iter()
            .filter(|p| p.organization_person_id == Some(person_id))
            .cloned()
            .collect();
        Ok(result)
    }
}

pub fn create_test_person() -> Person {
    Person {
        id: Uuid::new_v4(),
        person_type: PersonType::Natural,
        display_name: HeaplessString::try_from("John Doe").unwrap(),
        external_identifier: Some(HeaplessString::try_from("JD001").unwrap()),
        entity_reference_count: 0,
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
        location_id: None,
        duplicate_of_person_id: None,
    }
}
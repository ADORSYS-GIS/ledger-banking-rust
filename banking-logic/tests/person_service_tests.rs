use async_trait::async_trait;
use banking_api::domain::person::{
    Location, LocationType, Locality, Country, EntityReference, Messaging, Person, PersonType,
    RelationshipRole, CountrySubdivision,
};
use banking_api::service::{
    CountryService, CountrySubdivisionService, EntityReferenceService,
    LocalityService, LocationService, MessagingService, PersonService,
};
use banking_db::models::person::{
    LocationModel, LocationIdxModel, LocationAuditModel,
    LocalityModel, LocalityIdxModel, CountryModel, CountryIdxModel, EntityReferenceModel,
    EntityReferenceIdxModel, EntityReferenceAuditModel, MessagingModel, MessagingIdxModel,
    MessagingAuditModel, PersonModel, PersonIdxModel, PersonAuditModel, CountrySubdivisionModel,
    CountrySubdivisionIdxModel,
};
use banking_db::models::audit::AuditLogModel;

use banking_db::repository::{
    LocationRepository, LocalityRepository, CountryRepository, EntityReferenceRepository,
    MessagingRepository, CountrySubdivisionRepository, PersonRepository, AuditLogRepository,
};
use banking_db::repository::{
    audit_repository::AuditDomainError,
    location_repository::LocationDomainError,
    person::person_repository::{PersonDomainError, PersonResult},
};
use banking_logic::services::{
    CountryServiceImpl, CountrySubdivisionServiceImpl,
    EntityReferenceServiceImpl, LocalityServiceImpl, LocationServiceImpl,
    MessagingServiceImpl, PersonServiceImpl,
};
use banking_logic::services::repositories::Repositories;
use heapless::String as HeaplessString;
use std::error::Error;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use sqlx::Postgres;

// Mock Repositories
#[derive(Default)]
struct MockPersonRepository {
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
            None => Err(PersonDomainError::from(sqlx::Error::RowNotFound)),
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
}

#[derive(Default)]
struct MockCountryRepository {
    countries: Mutex<Vec<CountryModel>>,
    country_ixes: Mutex<Vec<CountryIdxModel>>,
}

#[async_trait]
impl CountryRepository<Postgres> for MockCountryRepository {
    async fn save(
        &self,
        country: CountryModel,
    ) -> Result<CountryModel, sqlx::Error> {
        self.countries.lock().unwrap().push(country.clone());
        let country_idx = CountryIdxModel {
            country_id: country.id,
            iso2: country.iso2.clone(),
        };
        self.country_ixes.lock().unwrap().push(country_idx);
        Ok(country)
    }

    async fn load(&self, id: Uuid) -> Result<CountryModel, sqlx::Error> {
        Ok(self
            .countries
            .lock()
            .unwrap()
            .iter()
            .find(|c| c.id == id)
            .cloned()
            .unwrap())
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<CountryIdxModel>, sqlx::Error> {
        Ok(self
            .country_ixes
            .lock()
            .unwrap()
            .iter()
            .find(|c| c.country_id == id)
            .cloned())
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<CountryIdxModel>, sqlx::Error> {
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

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(self
            .country_ixes
            .lock()
            .unwrap()
            .iter()
            .any(|c| c.country_id == id))
    }

    async fn find_ids_by_iso2(
        &self,
        iso2: &str,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
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
    ) -> Result<Vec<CountryIdxModel>, sqlx::Error> {
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

#[derive(Default)]
struct MockCountrySubdivisionRepository {
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

#[derive(Default)]
struct MockLocalityRepository {
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

#[derive(Default)]
struct MockLocationRepository {
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
    ) -> Result<LocationModel, LocationDomainError> {
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

    async fn load(&self, id: Uuid) -> Result<LocationModel, LocationDomainError> {
        match self
            .locations
            .lock()
            .unwrap()
            .iter()
            .find(|a| a.id == id)
            .cloned()
        {
            Some(location) => Ok(location),
            None => Err(LocationDomainError::RepositoryError(Box::new(
                sqlx::Error::RowNotFound,
            ))),
        }
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<LocationIdxModel>, LocationDomainError> {
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
    ) -> Result<Vec<LocationIdxModel>, LocationDomainError> {
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

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, LocationDomainError> {
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
    ) -> Result<Vec<Uuid>, LocationDomainError> {
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
    ) -> Result<Vec<LocationIdxModel>, LocationDomainError> {
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


#[derive(Default)]
struct MockMessagingRepository {
    messages: Mutex<Vec<MessagingModel>>,
    message_ixes: Mutex<Vec<MessagingIdxModel>>,
    message_audits: Mutex<Vec<MessagingAuditModel>>,
}

#[async_trait]
impl MessagingRepository<Postgres> for MockMessagingRepository {
    async fn save(
        &self,
        messaging: MessagingModel,
        audit_log_id: Uuid,
    ) -> Result<MessagingModel, sqlx::Error> {
        self.messages.lock().unwrap().push(messaging.clone());
        let msg_idx = MessagingIdxModel {
            messaging_id: messaging.id,
            value_hash: 0, // dummy hash
            version: 0,
            hash: 0,
        };
        self.message_ixes.lock().unwrap().push(msg_idx);

        let msg_audit = MessagingAuditModel {
            messaging_id: messaging.id,
            version: 0,
            hash: 0,
            messaging_type: messaging.messaging_type,
            value: messaging.value.clone(),
            other_type: messaging.other_type.clone(),
            audit_log_id,
        };
        self.message_audits.lock().unwrap().push(msg_audit);

        Ok(messaging)
    }

    async fn load(&self, id: Uuid) -> Result<MessagingModel, sqlx::Error> {
        Ok(self.messages.lock().unwrap().iter().find(|p| p.id == id).cloned().unwrap())
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<MessagingIdxModel>, Box<dyn Error + Send + Sync>> {
        Ok(self
            .message_ixes
            .lock()
            .unwrap()
            .iter()
            .find(|m| m.messaging_id == id)
            .cloned())
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<MessagingIdxModel>, Box<dyn Error + Send + Sync>> {
        let messages = self
            .message_ixes
            .lock()
            .unwrap()
            .iter()
            .filter(|m| ids.contains(&m.messaging_id))
            .cloned()
            .collect();
        Ok(messages)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(self
            .message_ixes
            .lock()
            .unwrap()
            .iter()
            .any(|m| m.messaging_id == id))
    }

    async fn find_ids_by_value(
        &self,
        value: &str,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = self
            .messages
            .lock()
            .unwrap()
            .iter()
            .filter(|m| m.value.as_str() == value)
            .map(|m| m.id)
            .collect();
        Ok(ids)
    }
}

#[derive(Default)]
struct MockEntityReferenceRepository {
    entities: Mutex<Vec<EntityReferenceModel>>,
    entity_ixes: Mutex<Vec<EntityReferenceIdxModel>>,
    entity_audits: Mutex<Vec<EntityReferenceAuditModel>>,
}

#[async_trait]
impl EntityReferenceRepository<Postgres> for MockEntityReferenceRepository {
    async fn save(
        &self,
        entity_ref: EntityReferenceModel,
        audit_log_id: Uuid,
    ) -> Result<EntityReferenceModel, sqlx::Error> {
        self.entities.lock().unwrap().push(entity_ref.clone());
        let entity_idx = EntityReferenceIdxModel {
            entity_reference_id: entity_ref.id,
            person_id: entity_ref.person_id,
            reference_external_id_hash: 0, // dummy hash
            version: 0,
            hash: 0,
        };
        self.entity_ixes.lock().unwrap().push(entity_idx);

        let entity_audit = EntityReferenceAuditModel {
            entity_reference_id: entity_ref.id,
            version: 0,
            hash: 0,
            person_id: entity_ref.person_id,
            entity_role: entity_ref.entity_role,
            reference_external_id: entity_ref.reference_external_id.clone(),
            reference_details_l1: entity_ref.reference_details_l1.clone(),
            reference_details_l2: entity_ref.reference_details_l2.clone(),
            reference_details_l3: entity_ref.reference_details_l3.clone(),
            audit_log_id,
        };
        self.entity_audits.lock().unwrap().push(entity_audit);

        Ok(entity_ref)
    }

    async fn load(&self, id: Uuid) -> Result<EntityReferenceModel, sqlx::Error> {
        Ok(self.entities.lock().unwrap().iter().find(|p| p.id == id).cloned().unwrap())
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<EntityReferenceIdxModel>, Box<dyn Error + Send + Sync>> {
        Ok(self
            .entity_ixes
            .lock()
            .unwrap()
            .iter()
            .find(|e| e.entity_reference_id == id)
            .cloned())
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<EntityReferenceIdxModel>, Box<dyn Error + Send + Sync>> {
        let entities = self
            .entity_ixes
            .lock()
            .unwrap()
            .iter()
            .filter(|e| ids.contains(&e.entity_reference_id))
            .cloned()
            .collect();
        Ok(entities)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(self.entities.lock().unwrap().iter().any(|e| e.id == id))
    }

    async fn find_ids_by_person_id(
        &self,
        person_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = self
            .entities
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.person_id == person_id)
            .map(|e| e.id)
            .collect();
        Ok(ids)
    }

    async fn find_by_person_id(
        &self,
        person_id: Uuid,
        _page: i32,
        _page_size: i32,
    ) -> Result<Vec<EntityReferenceIdxModel>, sqlx::Error> {
        let entities = self
            .entity_ixes
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.person_id == person_id)
            .cloned()
            .collect();
        Ok(entities)
    }

    async fn find_by_reference_external_id(
        &self,
        reference_external_id: &str,
        _page: i32,
        _page_size: i32,
    ) -> Result<Vec<EntityReferenceIdxModel>, sqlx::Error> {
        let entities = self.entities.lock().unwrap();
        let entity_ixes = self.entity_ixes.lock().unwrap();
        let ids: Vec<Uuid> = entities
            .iter()
            .filter(|e| e.reference_external_id.as_str() == reference_external_id)
            .map(|e| e.id)
            .collect();
        let result = entity_ixes
            .iter()
            .filter(|e| ids.contains(&e.entity_reference_id))
            .cloned()
            .collect();
        Ok(result)
    }
}

#[derive(Default)]
struct MockAuditLogRepository {
    audit_logs: Mutex<Vec<AuditLogModel>>,
}

#[async_trait]
impl AuditLogRepository<Postgres> for MockAuditLogRepository {
    async fn create(&self, audit_log: &AuditLogModel) -> Result<AuditLogModel, AuditDomainError> {
        self.audit_logs.lock().unwrap().push(audit_log.clone());
        Ok(audit_log.clone())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<AuditLogModel>, AuditDomainError> {
        Ok(self
            .audit_logs
            .lock()
            .unwrap()
            .iter()
            .find(|a| a.id == id)
            .cloned())
    }
}

struct TestServices {
    country_service: CountryServiceImpl<Postgres>,
    country_subdivision_service: CountrySubdivisionServiceImpl<Postgres>,
    locality_service: LocalityServiceImpl<Postgres>,
    location_service: LocationServiceImpl<Postgres>,
    messaging_service: MessagingServiceImpl<Postgres>,
    entity_reference_service: EntityReferenceServiceImpl<Postgres>,
    person_service: PersonServiceImpl<Postgres>,
}

fn create_test_services() -> TestServices {
    let repositories = Repositories {
        person_repository: Arc::new(MockPersonRepository::default()),
        audit_log_repository: Arc::new(MockAuditLogRepository::default()),
        country_repository: Arc::new(MockCountryRepository::default()),
        country_subdivision_repository: Arc::new(MockCountrySubdivisionRepository::default()),
        locality_repository: Arc::new(MockLocalityRepository::default()),
        location_repository: Arc::new(MockLocationRepository::default()),
        messaging_repository: Arc::new(MockMessagingRepository::default()),
        entity_reference_repository: Arc::new(MockEntityReferenceRepository::default()),
    };
    TestServices {
        country_service: CountryServiceImpl::new(repositories.clone()),
        country_subdivision_service: CountrySubdivisionServiceImpl::new(
            repositories.clone(),
        ),
        locality_service: LocalityServiceImpl::new(repositories.clone()),
        location_service: LocationServiceImpl::new(repositories.clone()),
        messaging_service: MessagingServiceImpl::new(repositories.clone()),
        entity_reference_service: EntityReferenceServiceImpl::new(
            repositories.clone(),
        ),
        person_service: PersonServiceImpl::new(repositories),
    }
}

fn create_test_audit_log() -> banking_api::domain::AuditLog {
    banking_api::domain::AuditLog {
        id: Uuid::new_v4(),
        updated_at: chrono::Utc::now(),
        updated_by_person_id: Uuid::new_v4(),
    }
}

// Helper functions for creating test data
fn create_test_country() -> Country {
    Country {
        id: Uuid::new_v4(),
        iso2: HeaplessString::try_from("US").unwrap(),
        name_l1: HeaplessString::try_from("United States").unwrap(),
        name_l2: None,
        name_l3: None,
    }
}

fn create_test_country_subdivision(country_id: Uuid) -> CountrySubdivision {
    CountrySubdivision {
        id: Uuid::new_v4(),
        country_id,
        code: HeaplessString::try_from("CA").unwrap(),
        name_l1: HeaplessString::try_from("California").unwrap(),
        name_l2: None,
        name_l3: None,
    }
}

fn create_test_locality(country_subdivision_id: Uuid) -> Locality {
    Locality {
        id: Uuid::new_v4(),
        country_subdivision_id,
        code: HeaplessString::try_from("LA").unwrap(),
        name_l1: HeaplessString::try_from("Los Angeles").unwrap(),
        name_l2: None,
        name_l3: None,
    }
}

fn create_test_location(locality_id: Uuid) -> Location {
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

fn create_test_messaging() -> Messaging {
    Messaging {
        id: Uuid::new_v4(),
        messaging_type: banking_api::domain::person::MessagingType::Email,
        value: HeaplessString::try_from("test@example.com").unwrap(),
        other_type: None,
    }
}

fn create_test_entity_reference(person_id: Uuid) -> EntityReference {
    EntityReference {
        id: Uuid::new_v4(),
        person_id,
        entity_role: RelationshipRole::Customer,
        reference_external_id: HeaplessString::new(),
        reference_details_l1: None,
        reference_details_l2: None,
        reference_details_l3: None,
    }
}

fn create_test_person() -> Person {
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

#[tokio::test]
async fn test_create_country() {
    let services = create_test_services();
    let country = create_test_country();
    let created_country = services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    assert_eq!(country.id, created_country.id);
}

#[tokio::test]
async fn test_find_country_by_id() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let found_country = services
        .country_service
        .find_country_by_id(country.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(country.id, found_country.id);
}

#[tokio::test]
async fn test_find_country_by_iso2() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let found_country = services
        .country_service
        .find_country_by_iso2(country.iso2.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(country.id, found_country.id);
}

#[tokio::test]
async fn test_get_all_countries() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let countries = services
        .country_service
        .get_all_countries()
        .await
        .unwrap();
    assert!(countries.is_empty());
}

#[tokio::test]
async fn test_create_country_subdivision() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    let created_country_subdivision = services
        .country_subdivision_service
        .create_country_subdivision(country_subdivision.clone())
        .await
        .unwrap();
    assert_eq!(country_subdivision.id, created_country_subdivision.id);
}

#[tokio::test]
async fn test_find_country_subdivision_by_id() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    services
        .country_subdivision_service
        .create_country_subdivision(country_subdivision.clone())
        .await
        .unwrap();
    let found_country_subdivision = services
        .country_subdivision_service
        .find_country_subdivision_by_id(country_subdivision.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(country_subdivision.id, found_country_subdivision.id);
}

#[tokio::test]
async fn test_find_country_subdivisions_by_country_id() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    services
        .country_subdivision_service
        .create_country_subdivision(country_subdivision.clone())
        .await
        .unwrap();
    let country_subdivisions = services
        .country_subdivision_service
        .find_country_subdivisions_by_country_id(country.id)
        .await
        .unwrap();
    assert!(!country_subdivisions.is_empty());
}

#[tokio::test]
async fn test_find_country_subdivision_by_code() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    services
        .country_subdivision_service
        .create_country_subdivision(country_subdivision.clone())
        .await
        .unwrap();
    let found_country_subdivision = services
        .country_subdivision_service
        .find_country_subdivision_by_code(country.id, country_subdivision.code.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(country_subdivision.id, found_country_subdivision.id);
}

#[tokio::test]
async fn test_create_locality() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    services
        .country_subdivision_service
        .create_country_subdivision(country_subdivision.clone())
        .await
        .unwrap();
    let locality = create_test_locality(country_subdivision.id);
    let created_locality = services
        .locality_service
        .create_locality(locality.clone())
        .await
        .unwrap();
    assert_eq!(locality.id, created_locality.id);
}

#[tokio::test]
async fn test_find_locality_by_id() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    services
        .country_subdivision_service
        .create_country_subdivision(country_subdivision.clone())
        .await
        .unwrap();
    let locality = create_test_locality(country_subdivision.id);
    services
        .locality_service
        .create_locality(locality.clone())
        .await
        .unwrap();
    let found_locality = services
        .locality_service
        .find_locality_by_id(locality.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(locality.id, found_locality.id);
}

#[tokio::test]
async fn test_find_localities_by_country_subdivision_id() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    services
        .country_subdivision_service
        .create_country_subdivision(country_subdivision.clone())
        .await
        .unwrap();
    let locality = create_test_locality(country_subdivision.id);
    services
        .locality_service
        .create_locality(locality.clone())
        .await
        .unwrap();
    let localities = services
        .locality_service
        .find_localities_by_country_subdivision_id(country_subdivision.id)
        .await
        .unwrap();
    assert!(!localities.is_empty());
}

#[tokio::test]
async fn test_find_locality_by_code() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    services
        .country_subdivision_service
        .create_country_subdivision(country_subdivision.clone())
        .await
        .unwrap();
    let locality = create_test_locality(country_subdivision.id);
    services
        .locality_service
        .create_locality(locality.clone())
        .await
        .unwrap();
    let found_locality = services
        .locality_service
        .find_locality_by_code(country_subdivision.id, locality.code.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(locality.id, found_locality.id);
}

#[tokio::test]
async fn test_create_location() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    services
        .country_subdivision_service
        .create_country_subdivision(country_subdivision.clone())
        .await
        .unwrap();
    let locality = create_test_locality(country_subdivision.id);
    services
        .locality_service
        .create_locality(locality.clone())
        .await
        .unwrap();
    let location = create_test_location(locality.id);
    let audit_log = create_test_audit_log();
    let created_location = services
        .location_service
        .create_location(location.clone(), audit_log)
        .await
        .unwrap();
    assert_eq!(location.id, created_location.id);
}

#[tokio::test]
async fn test_find_location_by_id() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    services
        .country_subdivision_service
        .create_country_subdivision(country_subdivision.clone())
        .await
        .unwrap();
    let locality = create_test_locality(country_subdivision.id);
    services
        .locality_service
        .create_locality(locality.clone())
        .await
        .unwrap();
    let location = create_test_location(locality.id);
    services
        .location_service
        .create_location(location.clone(), create_test_audit_log())
        .await
        .unwrap();
    let found_location = services
        .location_service
        .find_location_by_id(location.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(location.id, found_location.id);
}

#[tokio::test]
async fn test_find_locations_by_locality_id() {
    let services = create_test_services();
    let country = create_test_country();
    services
        .country_service
        .create_country(country.clone())
        .await
        .unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    services
        .country_subdivision_service
        .create_country_subdivision(country_subdivision.clone())
        .await
        .unwrap();
    let locality = create_test_locality(country_subdivision.id);
    services
        .locality_service
        .create_locality(locality.clone())
        .await
        .unwrap();
    let location = create_test_location(locality.id);
    services
        .location_service
        .create_location(location.clone(), create_test_audit_log())
        .await
        .unwrap();
    let locations = services
        .location_service
        .find_locations_by_locality_id(locality.id)
        .await
        .unwrap();
    assert!(!locations.is_empty());
}

#[tokio::test]
async fn test_create_messaging() {
    let services = create_test_services();
    let messaging = create_test_messaging();
    let created_messaging = services
        .messaging_service
        .create_messaging(messaging.clone(), create_test_audit_log())
        .await
        .unwrap();
    assert_eq!(messaging.id, created_messaging.id);
}

#[tokio::test]
async fn test_find_messaging_by_id() {
    let services = create_test_services();
    let messaging = create_test_messaging();
    services
        .messaging_service
        .create_messaging(messaging.clone(), create_test_audit_log())
        .await
        .unwrap();
    let found_messaging = services
        .messaging_service
        .find_messaging_by_id(messaging.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(messaging.id, found_messaging.id);
}

#[tokio::test]
async fn test_find_messaging_by_value() {
    let services = create_test_services();
    let messaging = create_test_messaging();
    services
        .messaging_service
        .create_messaging(messaging.clone(), create_test_audit_log())
        .await
        .unwrap();
    let found_messaging = services
        .messaging_service
        .find_messaging_by_value(messaging.value.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(messaging.id, found_messaging.id);
}

#[tokio::test]
async fn test_create_entity_reference() {
    let services = create_test_services();
    let person = create_test_person();
    services
        .person_service
        .create_person(person.clone(), create_test_audit_log())
        .await
        .unwrap();
    let entity_ref = create_test_entity_reference(person.id);
    let created_entity_ref = services
        .entity_reference_service
        .create_entity_reference(entity_ref.clone(), create_test_audit_log())
        .await
        .unwrap();
    assert_eq!(entity_ref.id, created_entity_ref.id);
}

#[tokio::test]
async fn test_find_entity_reference_by_id() {
    let services = create_test_services();
    let person = create_test_person();
    services
        .person_service
        .create_person(person.clone(), create_test_audit_log())
        .await
        .unwrap();
    let entity_ref = create_test_entity_reference(person.id);
    services
        .entity_reference_service
        .create_entity_reference(entity_ref.clone(), create_test_audit_log())
        .await
        .unwrap();
    let found_entity_ref = services
        .entity_reference_service
        .find_entity_reference_by_id(entity_ref.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(entity_ref.id, found_entity_ref.id);
}

#[tokio::test]
async fn test_find_entity_references_by_person_id() {
    let services = create_test_services();
    let person = create_test_person();
    services
        .person_service
        .create_person(person.clone(), create_test_audit_log())
        .await
        .unwrap();
    let entity_ref = create_test_entity_reference(person.id);
    services
        .entity_reference_service
        .create_entity_reference(entity_ref.clone(), create_test_audit_log())
        .await
        .unwrap();
    let entity_refs = services
        .entity_reference_service
        .find_entity_references_by_person_id(person.id)
        .await
        .unwrap();
    assert!(!entity_refs.is_empty());
}

#[tokio::test]
async fn test_create_person() {
    let services = create_test_services();
    let person = create_test_person();
    let created_person = services
        .person_service
        .create_person(person.clone(), create_test_audit_log())
        .await
        .unwrap();
    assert_eq!(person.id, created_person.id);
}

#[tokio::test]
async fn test_find_person_by_id() {
    let services = create_test_services();
    let person = create_test_person();
    services
        .person_service
        .create_person(person.clone(), create_test_audit_log())
        .await
        .unwrap();
    let found_person = services
        .person_service
        .find_person_by_id(person.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(person.id, found_person.id);
}

#[tokio::test]
async fn test_get_person_by_external_identifier() {
    let services = create_test_services();
    let person = create_test_person();
    services
        .person_service
        .create_person(person.clone(), create_test_audit_log())
        .await
        .unwrap();
    let found_person = services
        .person_service
        .get_persons_by_external_identifier(person.external_identifier.clone().unwrap())
        .await
        .unwrap();
    assert_eq!(person.id, found_person[0].id);
}

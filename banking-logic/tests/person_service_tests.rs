use async_trait::async_trait;
use banking_api::domain::person::{
    Location, LocationType, Locality, Country, EntityReference, Messaging, Person, PersonType,
    RelationshipRole, CountrySubdivision,
};
use banking_api::service::PersonService;
use banking_db::models::person::{
    LocationModel, LocationType as LocationTypeModel, LocalityModel, CountryModel, EntityReferenceModel,
    MessagingModel, PersonModel, PersonIdxModel,
    CountrySubdivisionModel,
};
use banking_db::models::audit::AuditLogModel;

use banking_db::repository::{
    LocationRepository, LocalityRepository, CountryRepository, EntityReferenceRepository,
    MessagingRepository, CountrySubdivisionRepository, PersonRepository, AuditLogRepository
};
use banking_logic::services::person_service_impl::PersonServiceImpl;
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
}

#[async_trait]
impl PersonRepository<Postgres> for MockPersonRepository {
    async fn save(&self, person: PersonModel) -> Result<PersonModel, sqlx::Error> {
        self.persons.lock().unwrap().push(person.clone());
        // In a real scenario, we'd create a proper hash and version.
        let person_idx = PersonIdxModel {
            person_id: person.id,
            external_identifier_hash: None,
            version: 0,
            hash: 0,
        };
        self.person_ixes.lock().unwrap().push(person_idx);
        Ok(person)
    }

    async fn load(&self, id: Uuid) -> Result<PersonModel, sqlx::Error> {
        Ok(self.persons.lock().unwrap().iter().find(|p| p.id == id).cloned().unwrap())
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<PersonIdxModel>, Box<dyn Error + Send + Sync>> {
        Ok(self.person_ixes.lock().unwrap().iter().find(|p| p.person_id == id).cloned())
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<PersonIdxModel>, Box<dyn Error + Send + Sync>> {
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

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(self.person_ixes.lock().unwrap().iter().any(|p| p.person_id == id))
    }

    async fn get_ids_by_external_identifier(
        &self,
        identifier: &str,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
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
    ) -> Result<Vec<PersonIdxModel>, Box<dyn Error + Send + Sync>> {
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
}

#[async_trait]
impl CountryRepository<Postgres> for MockCountryRepository {
    async fn save(
        &self,
        country: CountryModel,
    ) -> Result<CountryModel, sqlx::Error> {
        self.countries.lock().unwrap().push(country.clone());
        Ok(country)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<CountryModel>, sqlx::Error> {
        Ok(self
            .countries
            .lock()
            .unwrap()
            .iter()
            .find(|c| c.id == id)
            .cloned())
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<CountryModel>, sqlx::Error> {
        let countries = self
            .countries
            .lock()
            .unwrap()
            .iter()
            .filter(|c| ids.contains(&c.id))
            .cloned()
            .collect();
        Ok(countries)
    }

    async fn exists_by_id(&self, _id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_ids_by_iso2(
        &self,
        _iso2: &str,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_by_iso2(
        &self,
        iso2: &str,
        _page: i32,
        _page_size: i32,
    ) -> Result<Vec<CountryModel>, sqlx::Error> {
        let countries = self
            .countries
            .lock()
            .unwrap()
            .iter()
            .filter(|c| c.iso2 == iso2)
            .cloned()
            .collect();
        Ok(countries)
    }
}

#[derive(Default)]
struct MockCountrySubdivisionRepository {
    country_subdivisions: Mutex<Vec<CountrySubdivisionModel>>,
}

#[async_trait]
impl CountrySubdivisionRepository<Postgres> for MockCountrySubdivisionRepository {
    async fn save(
        &self,
        country_subdivision: CountrySubdivisionModel,
    ) -> Result<CountrySubdivisionModel, sqlx::Error> {
        self.country_subdivisions.lock().unwrap().push(country_subdivision.clone());
        Ok(country_subdivision)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<CountrySubdivisionModel>, sqlx::Error> {
        Ok(self.country_subdivisions.lock().unwrap().iter().find(|s| s.id == id).cloned())
    }

    async fn find_by_ids(
        &self,
        _ids: &[Uuid],
    ) -> Result<Vec<CountrySubdivisionModel>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn exists_by_id(&self, _id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_ids_by_country_id(
        &self,
        _country_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_by_country_id(
        &self,
        country_id: Uuid,
        _page: i32,
        _page_size: i32,
    ) -> Result<Vec<CountrySubdivisionModel>, sqlx::Error> {
        let country_subdivisions = self
            .country_subdivisions
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
    ) -> Result<Option<CountrySubdivisionModel>, sqlx::Error> {
        Ok(self
            .country_subdivisions
            .lock()
            .unwrap()
            .iter()
            .find(|s| s.country_id == country_id && s.code == code)
            .cloned())
    }
}

#[derive(Default)]
struct MockLocalityRepository {
    localities: Mutex<Vec<LocalityModel>>,
}

#[async_trait]
impl LocalityRepository<Postgres> for MockLocalityRepository {
    async fn save(&self, locality: LocalityModel) -> Result<LocalityModel, sqlx::Error> {
        self.localities.lock().unwrap().push(locality.clone());
        Ok(locality)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<LocalityModel>, sqlx::Error> {
        Ok(self.localities.lock().unwrap().iter().find(|c| c.id == id).cloned())
    }

    async fn find_by_ids(
        &self,
        _ids: &[Uuid],
    ) -> Result<Vec<LocalityModel>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn exists_by_id(&self, _id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_ids_by_country_subdivision_id(
        &self,
        _country_subdivision_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
        _page: i32,
        _page_size: i32,
    ) -> Result<Vec<LocalityModel>, sqlx::Error> {
        let localities = self
            .localities
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
    ) -> Result<Option<LocalityModel>, sqlx::Error> {
        Ok(self
            .localities
            .lock()
            .unwrap()
            .iter()
            .find(|c| c.country_subdivision_id == country_subdivision_id && c.code == code)
            .cloned())
    }
}

#[derive(Default)]
struct MockLocationRepository {
    locations: Mutex<Vec<LocationModel>>,
}

#[async_trait]
impl LocationRepository<Postgres> for MockLocationRepository {
    async fn save(&self, location: LocationModel) -> Result<LocationModel, sqlx::Error> {
        self.locations.lock().unwrap().push(location.clone());
        Ok(location)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<LocationModel>, sqlx::Error> {
        Ok(self
            .locations
            .lock()
            .unwrap()
            .iter()
            .find(|a| a.id == id)
            .cloned())
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<LocationModel>, sqlx::Error> {
        let locations = self
            .locations
            .lock()
            .unwrap()
            .iter()
            .filter(|a| ids.contains(&a.id))
            .cloned()
            .collect();
        Ok(locations)
    }

    async fn exists_by_id(&self, _id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_ids_by_location_type(
        &self,
        _location_type: LocationTypeModel,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_ids_by_locality_id(
        &self,
        _locality_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_by_locality_id(
        &self,
        locality_id: Uuid,
        _page: i32,
        _page_size: i32,
    ) -> Result<Vec<LocationModel>, sqlx::Error> {
        let locations = self
            .locations
            .lock()
            .unwrap()
            .iter()
            .filter(|a| a.locality_id == locality_id)
            .cloned()
            .collect();
        Ok(locations)
    }

    async fn find_ids_by_street_line1(
        &self,
        street_line1: &str,
    ) -> Result<Vec<Uuid>, sqlx::Error> {
        let ids = self
            .locations
            .lock()
            .unwrap()
            .iter()
            .filter(|a| a.street_line1 == street_line1)
            .map(|a| a.id)
            .collect();
        Ok(ids)
    }

    async fn find_by_type_and_locality(
        &self,
        location_type: LocationTypeModel,
        locality_id: Uuid,
        _page: i32,
        _page_size: i32,
    ) -> Result<Vec<LocationModel>, sqlx::Error> {
        let locations = self
            .locations
            .lock()
            .unwrap()
            .iter()
            .filter(|l| l.location_type == location_type && l.locality_id == locality_id)
            .cloned()
            .collect();
        Ok(locations)
    }
}

#[derive(Default)]
struct MockMessagingRepository {
    messages: Mutex<Vec<MessagingModel>>,
}

#[async_trait]
impl MessagingRepository<Postgres> for MockMessagingRepository {
    async fn save(
        &self,
        messaging: MessagingModel,
    ) -> Result<MessagingModel, sqlx::Error> {
        self.messages.lock().unwrap().push(messaging.clone());
        Ok(messaging)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<MessagingModel>, Box<dyn Error + Send + Sync>> {
        Ok(self
            .messages
            .lock()
            .unwrap()
            .iter()
            .find(|m| m.id == id)
            .cloned())
    }

    async fn find_by_ids(
        &self,
        _ids: &[Uuid],
    ) -> Result<Vec<MessagingModel>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn exists_by_id(&self, _id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        unimplemented!()
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
            .filter(|m| m.value == value)
            .map(|m| m.id)
            .collect();
        Ok(ids)
    }
}

#[derive(Default)]
struct MockEntityReferenceRepository {
    entities: Mutex<Vec<EntityReferenceModel>>,
}

#[async_trait]
impl EntityReferenceRepository<Postgres> for MockEntityReferenceRepository {
    async fn save(
        &self,
        entity_ref: EntityReferenceModel,
    ) -> Result<EntityReferenceModel, sqlx::Error> {
        self.entities.lock().unwrap().push(entity_ref.clone());
        Ok(entity_ref)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<EntityReferenceModel>, sqlx::Error> {
        Ok(self
            .entities
            .lock()
            .unwrap()
            .iter()
            .find(|e| e.id == id)
            .cloned())
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<EntityReferenceModel>, Box<dyn Error + Send + Sync>> {
        let entities = self
            .entities
            .lock()
            .unwrap()
            .iter()
            .filter(|e| ids.contains(&e.id))
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
    ) -> Result<Vec<EntityReferenceModel>, sqlx::Error> {
        let entities = self
            .entities
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
        _reference_external_id: &str,
        _page: i32,
        _page_size: i32,
    ) -> Result<Vec<EntityReferenceModel>, sqlx::Error> {
        unimplemented!()
    }
}

#[derive(Default)]
struct MockAuditLogRepository {
    audit_logs: Mutex<Vec<AuditLogModel>>,
}

#[async_trait]
impl AuditLogRepository<Postgres> for MockAuditLogRepository {
    async fn create(&self, audit_log: &AuditLogModel) -> Result<AuditLogModel, sqlx::Error> {
        self.audit_logs.lock().unwrap().push(audit_log.clone());
        Ok(audit_log.clone())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<AuditLogModel>, sqlx::Error> {
        Ok(self
            .audit_logs
            .lock()
            .unwrap()
            .iter()
            .find(|a| a.id == id)
            .cloned())
    }
}

fn create_test_service() -> PersonServiceImpl {
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
    PersonServiceImpl::new(repositories)
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
        version: 1,
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
        audit_log_id: Uuid::new_v4(),
    }
}

fn create_test_messaging() -> Messaging {
    Messaging {
        id: Uuid::new_v4(),
        version: 1,
        messaging_type: banking_api::domain::person::MessagingType::Email,
        value: HeaplessString::try_from("test@example.com").unwrap(),
        other_type: None,
        audit_log_id: Uuid::new_v4(),
    }
}

fn create_test_entity_reference(person_id: Uuid) -> EntityReference {
    EntityReference {
        id: Uuid::new_v4(),
        version: 1,
        person_id,
        entity_role: RelationshipRole::Customer,
        reference_external_id: HeaplessString::new(),
        reference_details_l1: None,
        reference_details_l2: None,
        reference_details_l3: None,
        audit_log_id: Uuid::new_v4(),
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
        audit_log_id: Uuid::new_v4(),
    }
}

#[tokio::test]
async fn test_create_country() {
    let service = create_test_service();
    let country = create_test_country();
    let created_country = service.create_country(country.clone()).await.unwrap();
    assert_eq!(country.id, created_country.id);
}

#[tokio::test]
async fn test_find_country_by_id() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let found_country = service.find_country_by_id(country.id).await.unwrap().unwrap();
    assert_eq!(country.id, found_country.id);
}

#[tokio::test]
async fn test_find_country_by_iso2() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let found_country = service
        .find_country_by_iso2(country.iso2.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(country.id, found_country.id);
}

#[tokio::test]
async fn test_get_all_countries() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let countries = service.get_all_countries().await.unwrap();
    assert!(countries.is_empty());
}

#[tokio::test]
async fn test_create_country_subdivision() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    let created_country_subdivision = service.create_country_subdivision(country_subdivision.clone()).await.unwrap();
    assert_eq!(country_subdivision.id, created_country_subdivision.id);
}

#[tokio::test]
async fn test_find_country_subdivision_by_id() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    service.create_country_subdivision(country_subdivision.clone()).await.unwrap();
    let found_country_subdivision = service
        .find_country_subdivision_by_id(country_subdivision.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(country_subdivision.id, found_country_subdivision.id);
}

#[tokio::test]
async fn test_find_country_subdivisions_by_country_id() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    service.create_country_subdivision(country_subdivision.clone()).await.unwrap();
    let country_subdivisions = service
        .find_country_subdivisions_by_country_id(country.id)
        .await
        .unwrap();
    assert!(!country_subdivisions.is_empty());
}

#[tokio::test]
async fn test_find_country_subdivision_by_code() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    service.create_country_subdivision(country_subdivision.clone()).await.unwrap();
    let found_country_subdivision = service
        .find_country_subdivision_by_code(country.id, country_subdivision.code.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(country_subdivision.id, found_country_subdivision.id);
}

#[tokio::test]
async fn test_create_locality() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    service.create_country_subdivision(country_subdivision.clone()).await.unwrap();
    let locality = create_test_locality(country_subdivision.id);
    let created_locality = service.create_locality(locality.clone()).await.unwrap();
    assert_eq!(locality.id, created_locality.id);
}

#[tokio::test]
async fn test_find_locality_by_id() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    service.create_country_subdivision(country_subdivision.clone()).await.unwrap();
    let locality = create_test_locality(country_subdivision.id);
    service.create_locality(locality.clone()).await.unwrap();
    let found_locality = service.find_locality_by_id(locality.id).await.unwrap().unwrap();
    assert_eq!(locality.id, found_locality.id);
}

#[tokio::test]
async fn test_find_localities_by_country_subdivision_id() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    service.create_country_subdivision(country_subdivision.clone()).await.unwrap();
    let locality = create_test_locality(country_subdivision.id);
    service.create_locality(locality.clone()).await.unwrap();
    let localities = service.find_localities_by_country_subdivision_id(country_subdivision.id).await.unwrap();
    assert!(!localities.is_empty());
}

#[tokio::test]
async fn test_find_locality_by_code() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    service.create_country_subdivision(country_subdivision.clone()).await.unwrap();
    let locality = create_test_locality(country_subdivision.id);
    service.create_locality(locality.clone()).await.unwrap();
    let found_locality = service
        .find_locality_by_code(country_subdivision.id, locality.code.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(locality.id, found_locality.id);
}

#[tokio::test]
async fn test_create_location() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    service.create_country_subdivision(country_subdivision.clone()).await.unwrap();
    let locality = create_test_locality(country_subdivision.id);
    service.create_locality(locality.clone()).await.unwrap();
    let location = create_test_location(locality.id);
    let created_location = service.create_location(location.clone()).await.unwrap();
    assert_eq!(location.id, created_location.id);
}

#[tokio::test]
async fn test_find_location_by_id() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    service.create_country_subdivision(country_subdivision.clone()).await.unwrap();
    let locality = create_test_locality(country_subdivision.id);
    service.create_locality(locality.clone()).await.unwrap();
    let location = create_test_location(locality.id);
    service.create_location(location.clone()).await.unwrap();
    let found_location = service
        .find_location_by_id(location.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(location.id, found_location.id);
}

#[tokio::test]
async fn test_find_locations_by_street_line1() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    service.create_country_subdivision(country_subdivision.clone()).await.unwrap();
    let locality = create_test_locality(country_subdivision.id);
    service.create_locality(locality.clone()).await.unwrap();
    let location = create_test_location(locality.id);
    service.create_location(location.clone()).await.unwrap();
    let locations = service
        .find_locations_by_street_line1(location.street_line1.clone())
        .await
        .unwrap();
    assert!(!locations.is_empty());
}

#[tokio::test]
async fn test_find_locations_by_locality_id() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    service.create_country_subdivision(country_subdivision.clone()).await.unwrap();
    let locality = create_test_locality(country_subdivision.id);
    service.create_locality(locality.clone()).await.unwrap();
    let location = create_test_location(locality.id);
    service.create_location(location.clone()).await.unwrap();
    let locations = service.find_locations_by_locality_id(locality.id).await.unwrap();
    assert!(!locations.is_empty());
}

#[tokio::test]
async fn test_find_locations_by_type_and_locality() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let country_subdivision = create_test_country_subdivision(country.id);
    service.create_country_subdivision(country_subdivision.clone()).await.unwrap();
    let locality = create_test_locality(country_subdivision.id);
    service.create_locality(locality.clone()).await.unwrap();
    let location = create_test_location(locality.id);
    service.create_location(location.clone()).await.unwrap();
    let locations = service
        .find_locations_by_type_and_locality(location.location_type, locality.id)
        .await
        .unwrap();
    assert!(!locations.is_empty());
}

#[tokio::test]
async fn test_create_messaging() {
    let service = create_test_service();
    let messaging = create_test_messaging();
    let created_messaging = service.create_messaging(messaging.clone()).await.unwrap();
    assert_eq!(messaging.id, created_messaging.id);
}

#[tokio::test]
async fn test_find_messaging_by_id() {
    let service = create_test_service();
    let messaging = create_test_messaging();
    service.create_messaging(messaging.clone()).await.unwrap();
    let found_messaging = service
        .find_messaging_by_id(messaging.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(messaging.id, found_messaging.id);
}

#[tokio::test]
async fn test_find_messaging_by_value() {
    let service = create_test_service();
    let messaging = create_test_messaging();
    service.create_messaging(messaging.clone()).await.unwrap();
    let found_messaging = service
        .find_messaging_by_value(messaging.value.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(messaging.id, found_messaging.id);
}

#[tokio::test]
async fn test_create_entity_reference() {
    let service = create_test_service();
    let person = create_test_person();
    service.create_person(person.clone()).await.unwrap();
    let entity_ref = create_test_entity_reference(person.id);
    let created_entity_ref = service
        .create_entity_reference(entity_ref.clone(), banking_api::domain::AuditLog { id: Uuid::new_v4(), updated_at: chrono::Utc::now(), updated_by_person_id: Uuid::new_v4() })
        .await
        .unwrap();
    assert_eq!(entity_ref.id, created_entity_ref.id);
}

#[tokio::test]
async fn test_find_entity_reference_by_id() {
    let service = create_test_service();
    let person = create_test_person();
    service.create_person(person.clone()).await.unwrap();
    let entity_ref = create_test_entity_reference(person.id);
    service
        .create_entity_reference(entity_ref.clone(), banking_api::domain::AuditLog { id: Uuid::new_v4(), updated_at: chrono::Utc::now(), updated_by_person_id: Uuid::new_v4() })
        .await
        .unwrap();
    let found_entity_ref = service
        .find_entity_reference_by_id(entity_ref.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(entity_ref.id, found_entity_ref.id);
}

#[tokio::test]
async fn test_find_entity_references_by_person_id() {
    let service = create_test_service();
    let person = create_test_person();
    service.create_person(person.clone()).await.unwrap();
    let entity_ref = create_test_entity_reference(person.id);
    service
        .create_entity_reference(entity_ref.clone(), banking_api::domain::AuditLog { id: Uuid::new_v4(), updated_at: chrono::Utc::now(), updated_by_person_id: Uuid::new_v4() })
        .await
        .unwrap();
    let entity_refs = service
        .find_entity_references_by_person_id(person.id)
        .await
        .unwrap();
    assert!(!entity_refs.is_empty());
}

#[tokio::test]
async fn test_create_person() {
    let service = create_test_service();
    let person = create_test_person();
    let created_person = service.create_person(person.clone()).await.unwrap();
    assert_eq!(person.id, created_person.id);
}

#[tokio::test]
async fn test_find_person_by_id() {
    let service = create_test_service();
    let person = create_test_person();
    service.create_person(person.clone()).await.unwrap();
    let found_person = service.find_person_by_id(person.id).await.unwrap().unwrap();
    assert_eq!(person.id, found_person.id);
}

#[tokio::test]
async fn test_get_person_by_external_identifier() {
    let service = create_test_service();
    let person = create_test_person();
    service.create_person(person.clone()).await.unwrap();
    let found_person = service
        .get_persons_by_external_identifier(person.external_identifier.clone().unwrap())
        .await
        .unwrap();
    assert_eq!(person.id, found_person[0].id);
}
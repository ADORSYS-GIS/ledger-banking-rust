use async_trait::async_trait;
use banking_api::domain::person::{
    Address, AddressType, City, Country, EntityReference, Messaging, Person, PersonType,
    RelationshipRole, StateProvince,
};
use banking_api::service::PersonService;
use banking_db::models::person::{
    AddressModel, AddressType as AddressTypeModel, CityModel, CountryModel, EntityReferenceModel,
    MessagingModel, MessagingType as MessagingTypeModel, PersonModel,
    PersonType as PersonTypeModel, RelationshipRole as RelationshipRoleModel, StateProvinceModel,
};
use banking_db::repository::{
    AddressRepository, CityRepository, CountryRepository, EntityReferenceRepository,
    MessagingRepository, PersonRepository, StateProvinceRepository,
};
use banking_logic::services::person_service_impl::PersonServiceImpl;
use heapless::String as HeaplessString;
use std::error::Error;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// Mock Repositories

#[derive(Default)]
struct MockPersonRepository {
    persons: Mutex<Vec<PersonModel>>,
}

#[async_trait]
impl PersonRepository for MockPersonRepository {
    async fn save(&self, person: PersonModel) -> Result<PersonModel, Box<dyn Error + Send + Sync>> {
        self.persons.lock().unwrap().push(person.clone());
        Ok(person)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<PersonModel>, Box<dyn Error + Send + Sync>> {
        Ok(self.persons.lock().unwrap().iter().find(|p| p.id == id).cloned())
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        let persons = self
            .persons
            .lock()
            .unwrap()
            .iter()
            .filter(|p| ids.contains(&p.id))
            .cloned()
            .collect();
        Ok(persons)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(self.persons.lock().unwrap().iter().any(|p| p.id == id))
    }

    async fn find_ids_by_person_type(
        &self,
        person_type: PersonTypeModel,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = self
            .persons
            .lock()
            .unwrap()
            .iter()
            .filter(|p| p.person_type == person_type)
            .map(|p| p.id)
            .collect();
        Ok(ids)
    }

    async fn find_by_person_type(
        &self,
        person_type: PersonTypeModel,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        let persons = self
            .persons
            .lock()
            .unwrap()
            .iter()
            .filter(|p| p.person_type == person_type)
            .cloned()
            .collect();
        Ok(persons)
    }

    async fn find_ids_by_is_active(
        &self,
        is_active: bool,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = self
            .persons
            .lock()
            .unwrap()
            .iter()
            .filter(|p| p.is_active == is_active)
            .map(|p| p.id)
            .collect();
        Ok(ids)
    }

    async fn find_by_is_active(
        &self,
        is_active: bool,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        let persons = self
            .persons
            .lock()
            .unwrap()
            .iter()
            .filter(|p| p.is_active == is_active)
            .cloned()
            .collect();
        Ok(persons)
    }

    async fn get_ids_by_external_identifier(
        &self,
        identifier: &str,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = self
            .persons
            .lock()
            .unwrap()
            .iter()
            .filter(|p| p.external_identifier.as_deref() == Some(identifier))
            .map(|p| p.id)
            .collect();
        Ok(ids)
    }

    async fn get_by_external_identifier(
        &self,
        identifier: &str,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        let persons = self
            .persons
            .lock()
            .unwrap()
            .iter()
            .filter(|p| p.external_identifier.as_deref() == Some(identifier))
            .cloned()
            .collect();
        Ok(persons)
    }

    async fn get_by_entity_reference(
        &self,
        _entity_id: Uuid,
        _entity_type: banking_db::models::person::RelationshipRole,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        Ok(vec![])
    }

    async fn create(
        &self,
        _display_name: &str,
        _person_type: PersonTypeModel,
        _external_identifier: Option<&str>,
    ) -> Result<PersonModel, Box<dyn Error + Send + Sync>> {
        unimplemented!()
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

#[derive(Default)]
struct MockCountryRepository {
    countries: Mutex<Vec<CountryModel>>,
}

#[async_trait]
impl CountryRepository for MockCountryRepository {
    async fn save(
        &self,
        country: CountryModel,
    ) -> Result<CountryModel, Box<dyn Error + Send + Sync>> {
        self.countries.lock().unwrap().push(country.clone());
        Ok(country)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<CountryModel>, Box<dyn Error + Send + Sync>> {
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
        _ids: &[Uuid],
    ) -> Result<Vec<CountryModel>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
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
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<CountryModel>, Box<dyn Error + Send + Sync>> {
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

    async fn find_ids_by_is_active(
        &self,
        _is_active: bool,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_by_is_active(
        &self,
        is_active: bool,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<CountryModel>, Box<dyn Error + Send + Sync>> {
        let countries = self
            .countries
            .lock()
            .unwrap()
            .iter()
            .filter(|c| c.is_active == is_active)
            .cloned()
            .collect();
        Ok(countries)
    }
}

#[derive(Default)]
struct MockStateProvinceRepository {
    states: Mutex<Vec<StateProvinceModel>>,
}

#[async_trait]
impl StateProvinceRepository for MockStateProvinceRepository {
    async fn save(
        &self,
        state: StateProvinceModel,
    ) -> Result<StateProvinceModel, Box<dyn Error + Send + Sync>> {
        self.states.lock().unwrap().push(state.clone());
        Ok(state)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<StateProvinceModel>, Box<dyn Error + Send + Sync>> {
        Ok(self.states.lock().unwrap().iter().find(|s| s.id == id).cloned())
    }

    async fn find_by_ids(
        &self,
        _ids: &[Uuid],
    ) -> Result<Vec<StateProvinceModel>, Box<dyn Error + Send + Sync>> {
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
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<StateProvinceModel>, Box<dyn Error + Send + Sync>> {
        let states = self
            .states
            .lock()
            .unwrap()
            .iter()
            .filter(|s| s.country_id == country_id)
            .cloned()
            .collect();
        Ok(states)
    }

    async fn find_state_province_by_state_province_code(
        &self,
        country_id: Uuid,
        state_province_code: &str,
    ) -> Result<Option<StateProvinceModel>, Box<dyn Error + Send + Sync>> {
        Ok(self
            .states
            .lock()
            .unwrap()
            .iter()
            .find(|s| s.country_id == country_id && s.state_province_code == state_province_code)
            .cloned())
    }

    async fn find_ids_by_is_active(
        &self,
        _is_active: bool,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_by_is_active(
        &self,
        _is_active: bool,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<StateProvinceModel>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }
}

#[derive(Default)]
struct MockCityRepository {
    cities: Mutex<Vec<CityModel>>,
}

#[async_trait]
impl CityRepository for MockCityRepository {
    async fn save(&self, city: CityModel) -> Result<CityModel, Box<dyn Error + Send + Sync>> {
        self.cities.lock().unwrap().push(city.clone());
        Ok(city)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<CityModel>, Box<dyn Error + Send + Sync>> {
        Ok(self.cities.lock().unwrap().iter().find(|c| c.id == id).cloned())
    }

    async fn find_by_ids(
        &self,
        _ids: &[Uuid],
    ) -> Result<Vec<CityModel>, Box<dyn Error + Send + Sync>> {
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
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<CityModel>, Box<dyn Error + Send + Sync>> {
        let cities = self
            .cities
            .lock()
            .unwrap()
            .iter()
            .filter(|c| c.country_id == country_id)
            .cloned()
            .collect();
        Ok(cities)
    }

    async fn find_ids_by_state_id(
        &self,
        _state_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_by_state_id(
        &self,
        state_id: Uuid,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<CityModel>, Box<dyn Error + Send + Sync>> {
        let cities = self
            .cities
            .lock()
            .unwrap()
            .iter()
            .filter(|c| c.state_id == Some(state_id))
            .cloned()
            .collect();
        Ok(cities)
    }

    async fn find_city_by_city_code(
        &self,
        country_id: Uuid,
        city_code: &str,
    ) -> Result<Option<CityModel>, Box<dyn Error + Send + Sync>> {
        Ok(self
            .cities
            .lock()
            .unwrap()
            .iter()
            .find(|c| c.country_id == country_id && c.city_code == city_code)
            .cloned())
    }

    async fn find_ids_by_is_active(
        &self,
        _is_active: bool,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_by_is_active(
        &self,
        _is_active: bool,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<CityModel>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }
}

#[derive(Default)]
struct MockAddressRepository {
    addresses: Mutex<Vec<AddressModel>>,
}

#[async_trait]
impl AddressRepository for MockAddressRepository {
    async fn save(&self, address: AddressModel) -> Result<AddressModel, Box<dyn Error + Send + Sync>> {
        self.addresses.lock().unwrap().push(address.clone());
        Ok(address)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<AddressModel>, Box<dyn Error + Send + Sync>> {
        Ok(self
            .addresses
            .lock()
            .unwrap()
            .iter()
            .find(|a| a.id == id)
            .cloned())
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<AddressModel>, Box<dyn Error + Send + Sync>> {
        let addresses = self
            .addresses
            .lock()
            .unwrap()
            .iter()
            .filter(|a| ids.contains(&a.id))
            .cloned()
            .collect();
        Ok(addresses)
    }

    async fn exists_by_id(&self, _id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_ids_by_address_type(
        &self,
        _address_type: AddressTypeModel,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_by_address_type(
        &self,
        address_type: AddressTypeModel,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<AddressModel>, Box<dyn Error + Send + Sync>> {
        let addresses = self
            .addresses
            .lock()
            .unwrap()
            .iter()
            .filter(|a| a.address_type == address_type)
            .cloned()
            .collect();
        Ok(addresses)
    }

    async fn find_ids_by_city_id(
        &self,
        _city_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_by_city_id(
        &self,
        city_id: Uuid,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<AddressModel>, Box<dyn Error + Send + Sync>> {
        let addresses = self
            .addresses
            .lock()
            .unwrap()
            .iter()
            .filter(|a| a.city_id == city_id)
            .cloned()
            .collect();
        Ok(addresses)
    }

    async fn find_ids_by_is_active(
        &self,
        _is_active: bool,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_by_is_active(
        &self,
        _is_active: bool,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<AddressModel>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_ids_by_street_line1(
        &self,
        street_line1: &str,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = self
            .addresses
            .lock()
            .unwrap()
            .iter()
            .filter(|a| a.street_line1 == street_line1)
            .map(|a| a.id)
            .collect();
        Ok(ids)
    }
}

#[derive(Default)]
struct MockMessagingRepository {
    messages: Mutex<Vec<MessagingModel>>,
}

#[async_trait]
impl MessagingRepository for MockMessagingRepository {
    async fn save(
        &self,
        messaging: MessagingModel,
    ) -> Result<MessagingModel, Box<dyn Error + Send + Sync>> {
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

    async fn find_ids_by_messaging_type(
        &self,
        _messaging_type: MessagingTypeModel,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_by_messaging_type(
        &self,
        _messaging_type: MessagingTypeModel,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<MessagingModel>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_ids_by_is_active(
        &self,
        _is_active: bool,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }

    async fn find_by_is_active(
        &self,
        _is_active: bool,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<MessagingModel>, Box<dyn Error + Send + Sync>> {
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
impl EntityReferenceRepository for MockEntityReferenceRepository {
    async fn save(
        &self,
        entity_ref: EntityReferenceModel,
    ) -> Result<EntityReferenceModel, Box<dyn Error + Send + Sync>> {
        self.entities.lock().unwrap().push(entity_ref.clone());
        Ok(entity_ref)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<EntityReferenceModel>, Box<dyn Error + Send + Sync>> {
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
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<EntityReferenceModel>, Box<dyn Error + Send + Sync>> {
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

    async fn find_ids_by_entity_role(
        &self,
        entity_role: RelationshipRoleModel,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = self
            .entities
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.entity_role == entity_role)
            .map(|e| e.id)
            .collect();
        Ok(ids)
    }

    async fn find_by_entity_role(
        &self,
        entity_role: RelationshipRoleModel,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<EntityReferenceModel>, Box<dyn Error + Send + Sync>> {
        let entities = self
            .entities
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.entity_role == entity_role)
            .cloned()
            .collect();
        Ok(entities)
    }

    async fn find_ids_by_is_active(
        &self,
        is_active: bool,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = self
            .entities
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.is_active == is_active)
            .map(|e| e.id)
            .collect();
        Ok(ids)
    }

    async fn find_by_is_active(
        &self,
        is_active: bool,
        _page: u32,
        _page_size: u32,
    ) -> Result<Vec<EntityReferenceModel>, Box<dyn Error + Send + Sync>> {
        let entities = self
            .entities
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.is_active == is_active)
            .cloned()
            .collect();
        Ok(entities)
    }
}

fn create_test_service() -> PersonServiceImpl {
    PersonServiceImpl::new(
        Arc::new(MockPersonRepository::default()),
        Arc::new(MockCountryRepository::default()),
        Arc::new(MockStateProvinceRepository::default()),
        Arc::new(MockCityRepository::default()),
        Arc::new(MockAddressRepository::default()),
        Arc::new(MockMessagingRepository::default()),
        Arc::new(MockEntityReferenceRepository::default()),
    )
}

// Helper functions for creating test data
fn create_test_country() -> Country {
    let person_id = Uuid::new_v4();
    Country {
        id: Uuid::new_v4(),
        iso2: HeaplessString::try_from("US").unwrap(),
        name_l1: HeaplessString::try_from("United States").unwrap(),
        name_l2: None,
        name_l3: None,
        is_active: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        created_by_person_id: person_id,
        updated_by_person_id: person_id,
    }
}

fn create_test_state(country_id: Uuid) -> StateProvince {
    let person_id = Uuid::new_v4();
    StateProvince {
        id: Uuid::new_v4(),
        country_id,
        state_province_code: HeaplessString::try_from("CA").unwrap(),
        name_l1: HeaplessString::try_from("California").unwrap(),
        name_l2: None,
        name_l3: None,
        is_active: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        created_by_person_id: person_id,
        updated_by_person_id: person_id,
    }
}

fn create_test_city(country_id: Uuid, state_id: Uuid) -> City {
    let person_id = Uuid::new_v4();
    City {
        id: Uuid::new_v4(),
        country_id,
        state_id: Some(state_id),
        city_code: HeaplessString::try_from("LA").unwrap(),
        name_l1: HeaplessString::try_from("Los Angeles").unwrap(),
        name_l2: None,
        name_l3: None,
        is_active: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        created_by_person_id: person_id,
        updated_by_person_id: person_id,
    }
}

fn create_test_address(city_id: Uuid) -> Address {
    let person_id = Uuid::new_v4();
    Address {
        id: Uuid::new_v4(),
        address_type: AddressType::Residential,
        street_line1: HeaplessString::try_from("123 Main St").unwrap(),
        street_line2: None,
        street_line3: None,
        street_line4: None,
        city_id,
        postal_code: None,
        latitude: None,
        longitude: None,
        accuracy_meters: None,
        is_active: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        created_by_person_id: person_id,
        updated_by_person_id: person_id,
    }
}

fn create_test_messaging() -> Messaging {
    Messaging {
        id: Uuid::new_v4(),
        messaging_type: banking_api::domain::person::MessagingType::Email,
        value: HeaplessString::try_from("test@example.com").unwrap(),
        other_type: None,
        is_active: true,
        priority: Some(1),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }
}

fn create_test_entity_reference(person_id: Uuid) -> EntityReference {
    let creator_id = Uuid::new_v4();
    EntityReference {
        id: Uuid::new_v4(),
        person_id,
        entity_role: RelationshipRole::Customer,
        reference_external_id: None,
        reference_details_l1: None,
        reference_details_l2: None,
        reference_details_l3: None,
        is_active: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        created_by_person_id: creator_id,
        updated_by_person_id: creator_id,
    }
}

fn create_test_person() -> Person {
    Person {
        id: Uuid::new_v4(),
        person_type: PersonType::Natural,
        display_name: HeaplessString::try_from("John Doe").unwrap(),
        external_identifier: Some(HeaplessString::try_from("JD001").unwrap()),
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
    assert!(!countries.is_empty());
}

#[tokio::test]
async fn test_create_state_province() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let state = create_test_state(country.id);
    let created_state = service.create_state_province(state.clone()).await.unwrap();
    assert_eq!(state.id, created_state.id);
}

#[tokio::test]
async fn test_find_state_province_by_id() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let state = create_test_state(country.id);
    service.create_state_province(state.clone()).await.unwrap();
    let found_state = service
        .find_state_province_by_id(state.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(state.id, found_state.id);
}

#[tokio::test]
async fn test_find_state_provinces_by_country_id() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let state = create_test_state(country.id);
    service.create_state_province(state.clone()).await.unwrap();
    let states = service
        .find_state_provinces_by_country_id(country.id)
        .await
        .unwrap();
    assert!(!states.is_empty());
}

#[tokio::test]
async fn test_find_state_province_by_code() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let state = create_test_state(country.id);
    service.create_state_province(state.clone()).await.unwrap();
    let found_state = service
        .find_state_province_by_code(country.id, state.state_province_code.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(state.id, found_state.id);
}

#[tokio::test]
async fn test_create_city() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let state = create_test_state(country.id);
    service.create_state_province(state.clone()).await.unwrap();
    let city = create_test_city(country.id, state.id);
    let created_city = service.create_city(city.clone()).await.unwrap();
    assert_eq!(city.id, created_city.id);
}

#[tokio::test]
async fn test_find_city_by_id() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let state = create_test_state(country.id);
    service.create_state_province(state.clone()).await.unwrap();
    let city = create_test_city(country.id, state.id);
    service.create_city(city.clone()).await.unwrap();
    let found_city = service.find_city_by_id(city.id).await.unwrap().unwrap();
    assert_eq!(city.id, found_city.id);
}

#[tokio::test]
async fn test_find_cities_by_country_id() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let state = create_test_state(country.id);
    service.create_state_province(state.clone()).await.unwrap();
    let city = create_test_city(country.id, state.id);
    service.create_city(city.clone()).await.unwrap();
    let cities = service.find_cities_by_country_id(country.id).await.unwrap();
    assert!(!cities.is_empty());
}

#[tokio::test]
async fn test_find_cities_by_state_id() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let state = create_test_state(country.id);
    service.create_state_province(state.clone()).await.unwrap();
    let city = create_test_city(country.id, state.id);
    service.create_city(city.clone()).await.unwrap();
    let cities = service.find_cities_by_state_id(state.id).await.unwrap();
    assert!(!cities.is_empty());
}

#[tokio::test]
async fn test_find_city_by_code() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let state = create_test_state(country.id);
    service.create_state_province(state.clone()).await.unwrap();
    let city = create_test_city(country.id, state.id);
    service.create_city(city.clone()).await.unwrap();
    let found_city = service
        .find_city_by_code(country.id, city.city_code.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(city.id, found_city.id);
}

#[tokio::test]
async fn test_create_address() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let state = create_test_state(country.id);
    service.create_state_province(state.clone()).await.unwrap();
    let city = create_test_city(country.id, state.id);
    service.create_city(city.clone()).await.unwrap();
    let address = create_test_address(city.id);
    let created_address = service.create_address(address.clone()).await.unwrap();
    assert_eq!(address.id, created_address.id);
}

#[tokio::test]
async fn test_find_address_by_id() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let state = create_test_state(country.id);
    service.create_state_province(state.clone()).await.unwrap();
    let city = create_test_city(country.id, state.id);
    service.create_city(city.clone()).await.unwrap();
    let address = create_test_address(city.id);
    service.create_address(address.clone()).await.unwrap();
    let found_address = service
        .find_address_by_id(address.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(address.id, found_address.id);
}

#[tokio::test]
async fn test_find_addresses_by_street_line1() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let state = create_test_state(country.id);
    service.create_state_province(state.clone()).await.unwrap();
    let city = create_test_city(country.id, state.id);
    service.create_city(city.clone()).await.unwrap();
    let address = create_test_address(city.id);
    service.create_address(address.clone()).await.unwrap();
    let addresses = service
        .find_addresses_by_street_line1(address.street_line1.clone())
        .await
        .unwrap();
    assert!(!addresses.is_empty());
}

#[tokio::test]
async fn test_find_addresses_by_city_id() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let state = create_test_state(country.id);
    service.create_state_province(state.clone()).await.unwrap();
    let city = create_test_city(country.id, state.id);
    service.create_city(city.clone()).await.unwrap();
    let address = create_test_address(city.id);
    service.create_address(address.clone()).await.unwrap();
    let addresses = service.find_addresses_by_city_id(city.id).await.unwrap();
    assert!(!addresses.is_empty());
}

#[tokio::test]
async fn test_find_addresses_by_type_and_city() {
    let service = create_test_service();
    let country = create_test_country();
    service.create_country(country.clone()).await.unwrap();
    let state = create_test_state(country.id);
    service.create_state_province(state.clone()).await.unwrap();
    let city = create_test_city(country.id, state.id);
    service.create_city(city.clone()).await.unwrap();
    let address = create_test_address(city.id);
    service.create_address(address.clone()).await.unwrap();
    let addresses = service
        .find_addresses_by_type_and_city(address.address_type, city.id)
        .await
        .unwrap();
    assert!(!addresses.is_empty());
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
        .create_entity_reference(entity_ref.clone())
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
        .create_entity_reference(entity_ref.clone())
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
        .create_entity_reference(entity_ref.clone())
        .await
        .unwrap();
    let entity_refs = service
        .find_entity_references_by_person_id(person.id)
        .await
        .unwrap();
    assert!(!entity_refs.is_empty());
}

#[tokio::test]
async fn test_find_entity_reference_by_person_and_role() {
    let service = create_test_service();
    let person = create_test_person();
    service.create_person(person.clone()).await.unwrap();
    let entity_ref = create_test_entity_reference(person.id);
    service
        .create_entity_reference(entity_ref.clone())
        .await
        .unwrap();
    let found_entity_ref = service
        .find_entity_reference_by_person_and_role(person.id, entity_ref.entity_role)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(entity_ref.id, found_entity_ref.id);
}

#[tokio::test]
async fn test_find_active_entity_references() {
    let service = create_test_service();
    let person = create_test_person();
    service.create_person(person.clone()).await.unwrap();
    let mut entity_ref = create_test_entity_reference(person.id);
    entity_ref.is_active = true;
    service
        .create_entity_reference(entity_ref.clone())
        .await
        .unwrap();
    let entity_refs = service.find_active_entity_references().await.unwrap();
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
async fn test_find_persons_by_type() {
    let service = create_test_service();
    let person = create_test_person();
    service.create_person(person.clone()).await.unwrap();
    let persons = service
        .find_persons_by_type(person.person_type)
        .await
        .unwrap();
    assert!(!persons.is_empty());
}

#[tokio::test]
async fn test_get_person_by_external_identifier() {
    let service = create_test_service();
    let person = create_test_person();
    service.create_person(person.clone()).await.unwrap();
    let found_person = service
        .get_person_by_external_identifier(person.external_identifier.clone().unwrap())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(person.id, found_person.id);
}
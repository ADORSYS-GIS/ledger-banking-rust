use async_trait::async_trait;
use banking_api::domain::person::{
    Address, AddressType, City, Country, EntityReference, Messaging, Person, PersonType,
    RelationshipRole, StateProvince,
};
use banking_api::service::PersonService;
use banking_api::BankingResult;
use banking_db::repository::{
    AddressRepository, CityRepository, CountryRepository, EntityReferenceRepository,
    MessagingRepository, PersonRepository, StateProvinceRepository,
};
use heapless::String as HeaplessString;
use std::sync::Arc;
use uuid::Uuid;

use crate::mappers::person_mapper::{ToDomain, ToModel};

pub struct PersonServiceImpl {
    person_repository: Arc<dyn PersonRepository>,
    country_repository: Arc<dyn CountryRepository>,
    state_province_repository: Arc<dyn StateProvinceRepository>,
    city_repository: Arc<dyn CityRepository>,
    address_repository: Arc<dyn AddressRepository>,
    messaging_repository: Arc<dyn MessagingRepository>,
    entity_reference_repository: Arc<dyn EntityReferenceRepository>,
}

impl PersonServiceImpl {
    pub fn new(
        person_repository: Arc<dyn PersonRepository>,
        country_repository: Arc<dyn CountryRepository>,
        state_province_repository: Arc<dyn StateProvinceRepository>,
        city_repository: Arc<dyn CityRepository>,
        address_repository: Arc<dyn AddressRepository>,
        messaging_repository: Arc<dyn MessagingRepository>,
        entity_reference_repository: Arc<dyn EntityReferenceRepository>,
    ) -> Self {
        Self {
            person_repository,
            country_repository,
            state_province_repository,
            city_repository,
            address_repository,
            messaging_repository,
            entity_reference_repository,
        }
    }
}

#[async_trait]
impl PersonService for PersonServiceImpl {
    async fn create_country(&self, country: Country) -> BankingResult<Country> {
        let model = country.to_model();
        let saved_model = self.country_repository.save(model).await?;
        Ok(saved_model.to_domain())
    }

    async fn find_country_by_id(&self, id: Uuid) -> BankingResult<Option<Country>> {
        let model = self.country_repository.find_by_id(id).await?;
        Ok(model.map(|m| m.to_domain()))
    }

    async fn find_country_by_iso2(&self, iso2: HeaplessString<2>) -> BankingResult<Option<Country>> {
        let models = self.country_repository.find_by_iso2(iso2.as_str(), 1, 1).await?;
        Ok(models.into_iter().next().map(|m| m.to_domain()))
    }

    async fn get_all_countries(&self) -> BankingResult<Vec<Country>> {
        let models = self.country_repository.find_by_is_active(true, 1, 1000).await?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn create_state_province(&self, state_province: StateProvince) -> BankingResult<StateProvince> {
        let model = state_province.to_model();
        let saved_model = self.state_province_repository.save(model).await?;
        Ok(saved_model.to_domain())
    }

    async fn find_state_province_by_id(&self, id: Uuid) -> BankingResult<Option<StateProvince>> {
        let model = self.state_province_repository.find_by_id(id).await?;
        Ok(model.map(|m| m.to_domain()))
    }

    async fn find_state_provinces_by_country_id(&self, country_id: Uuid) -> BankingResult<Vec<StateProvince>> {
        let models = self.state_province_repository.find_by_country_id(country_id, 1, 1000).await?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn find_state_province_by_code(
        &self,
        country_id: Uuid,
        state_province_code: HeaplessString<10>,
    ) -> BankingResult<Option<StateProvince>> {
        let model = self.state_province_repository.find_state_province_by_state_province_code(country_id, state_province_code.as_str()).await?;
        Ok(model.map(|m| m.to_domain()))
    }

    async fn create_city(&self, city: City) -> BankingResult<City> {
        let model = city.to_model();
        let saved_model = self.city_repository.save(model).await?;
        Ok(saved_model.to_domain())
    }

    async fn find_city_by_id(&self, id: Uuid) -> BankingResult<Option<City>> {
        let model = self.city_repository.find_by_id(id).await?;
        Ok(model.map(|m| m.to_domain()))
    }

    async fn find_cities_by_country_id(&self, country_id: Uuid) -> BankingResult<Vec<City>> {
        let models = self.city_repository.find_by_country_id(country_id, 1, 1000).await?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn find_cities_by_state_id(&self, state_id: Uuid) -> BankingResult<Vec<City>> {
        let models = self.city_repository.find_by_state_id(state_id, 1, 1000).await?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn find_city_by_code(
        &self,
        country_id: Uuid,
        city_code: HeaplessString<50>,
    ) -> BankingResult<Option<City>> {
        let model = self.city_repository.find_city_by_city_code(country_id, city_code.as_str()).await?;
        Ok(model.map(|m| m.to_domain()))
    }

    async fn create_address(&self, address: Address) -> BankingResult<Address> {
        let model = address.to_model();
        let saved_model = self.address_repository.save(model).await?;
        Ok(saved_model.to_domain())
    }

    async fn find_address_by_id(&self, id: Uuid) -> BankingResult<Option<Address>> {
        let model = self.address_repository.find_by_id(id).await?;
        Ok(model.map(|m| m.to_domain()))
    }

    async fn find_addresses_by_street_line1(
        &self,
        street_line1: HeaplessString<50>,
    ) -> BankingResult<Vec<Address>> {
        let ids = self
            .address_repository
            .find_ids_by_street_line1(street_line1.as_str())
            .await?;
        let models = self.address_repository.find_by_ids(&ids).await?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn find_addresses_by_city_id(&self, city_id: Uuid) -> BankingResult<Vec<Address>> {
        let models = self.address_repository.find_by_city_id(city_id, 1, 1000).await?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn find_addresses_by_type_and_city(
        &self,
        address_type: AddressType,
        city_id: Uuid,
    ) -> BankingResult<Vec<Address>> {
        let models = self.address_repository.find_by_address_type(address_type.to_model(), 1, 1000).await?;
        let filtered_models = models.into_iter().filter(|m| m.city_id == city_id).collect::<Vec<_>>();
        Ok(filtered_models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn create_messaging(&self, messaging: Messaging) -> BankingResult<Messaging> {
        let model = messaging.to_model();
        let saved_model = self.messaging_repository.save(model).await?;
        Ok(saved_model.to_domain())
    }

    async fn find_messaging_by_id(&self, id: Uuid) -> BankingResult<Option<Messaging>> {
        let model = self.messaging_repository.find_by_id(id).await?;
        Ok(model.map(|m| m.to_domain()))
    }

    async fn find_messaging_by_value(
        &self,
        value: HeaplessString<100>,
    ) -> BankingResult<Option<Messaging>> {
        let ids = self
            .messaging_repository
            .find_ids_by_value(value.as_str())
            .await?;
        if let Some(id) = ids.first() {
            let model = self.messaging_repository.find_by_id(*id).await?;
            Ok(model.map(|m| m.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn create_entity_reference(&self, entity_reference: EntityReference) -> BankingResult<EntityReference> {
        let model = entity_reference.to_model();
        let saved_model = self.entity_reference_repository.save(model).await?;
        Ok(saved_model.to_domain())
    }

    async fn find_entity_reference_by_id(&self, id: Uuid) -> BankingResult<Option<EntityReference>> {
        let model = self.entity_reference_repository.find_by_id(id).await?;
        Ok(model.map(|m| m.to_domain()))
    }

    async fn find_entity_references_by_person_id(&self, person_id: Uuid) -> BankingResult<Vec<EntityReference>> {
        let models = self.entity_reference_repository.find_by_person_id(person_id, 1, 1000).await?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn find_entity_reference_by_person_and_role(
        &self,
        person_id: Uuid,
        entity_role: RelationshipRole,
    ) -> BankingResult<Option<EntityReference>> {
        let models = self.entity_reference_repository.find_by_entity_role(entity_role.to_model(), 1, 1000).await?;
        let filtered_model = models.into_iter().find(|m| m.person_id == person_id);
        Ok(filtered_model.map(|m| m.to_domain()))
    }

    async fn find_active_entity_references(&self) -> BankingResult<Vec<EntityReference>> {
        let models = self.entity_reference_repository.find_by_is_active(true, 1, 1000).await?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn create_person(&self, person: Person) -> BankingResult<Person> {
        let model = person.to_model();
        let saved_model = self.person_repository.save(model).await?;
        Ok(saved_model.to_domain())
    }

    async fn find_person_by_id(&self, id: Uuid) -> BankingResult<Option<Person>> {
        let model = self.person_repository.find_by_id(id).await?;
        Ok(model.map(|m| m.to_domain()))
    }

    async fn find_persons_by_type(&self, person_type: PersonType) -> BankingResult<Vec<Person>> {
        let models = self.person_repository.find_by_person_type(person_type.to_model(), 1, 1000).await?;
        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn get_person_by_external_identifier(
        &self,
        external_identifier: HeaplessString<50>,
    ) -> BankingResult<Option<Person>> {
        let models = self.person_repository.get_by_external_identifier(external_identifier.as_str()).await?;
        Ok(models.into_iter().next().map(|m| m.to_domain()))
    }
}
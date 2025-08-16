use crate::domain::person::{
    Address, AddressType, City, Country, EntityReference, Messaging, Person, PersonType,
    RelationshipRole, StateProvince,
};
use crate::BankingResult;
use async_trait::async_trait;
use heapless::String as HeaplessString;
use uuid::Uuid;

#[async_trait]
pub trait PersonService: Send + Sync {
    // Country methods
    async fn create_country(&self, country: Country) -> BankingResult<Country>;
    async fn find_country_by_id(&self, id: Uuid) -> BankingResult<Option<Country>>;
    async fn find_country_by_iso2(&self, iso2: HeaplessString<2>) -> BankingResult<Option<Country>>;
    async fn get_all_countries(&self) -> BankingResult<Vec<Country>>;

    // StateProvince methods
    async fn create_state_province(&self, state_province: StateProvince) -> BankingResult<StateProvince>;
    async fn find_state_province_by_id(&self, id: Uuid) -> BankingResult<Option<StateProvince>>;
    async fn find_state_provinces_by_country_id(&self, country_id: Uuid) -> BankingResult<Vec<StateProvince>>;
    async fn find_state_province_by_code(
        &self,
        country_id: Uuid,
        state_province_code: HeaplessString<10>,
    ) -> BankingResult<Option<StateProvince>>;

    // City methods
    async fn create_city(&self, city: City) -> BankingResult<City>;
    async fn find_city_by_id(&self, id: Uuid) -> BankingResult<Option<City>>;
    async fn find_cities_by_country_id(&self, country_id: Uuid) -> BankingResult<Vec<City>>;
    async fn find_cities_by_state_id(&self, state_id: Uuid) -> BankingResult<Vec<City>>;
    async fn find_city_by_code(
        &self,
        country_id: Uuid,
        city_code: HeaplessString<50>,
    ) -> BankingResult<Option<City>>;

    // Address methods
    async fn create_address(&self, address: Address) -> BankingResult<Address>;
    async fn find_address_by_id(&self, id: Uuid) -> BankingResult<Option<Address>>;
    async fn find_addresses_by_street_line1(
        &self,
        street_line1: HeaplessString<50>,
    ) -> BankingResult<Vec<Address>>;
    async fn find_addresses_by_city_id(&self, city_id: Uuid) -> BankingResult<Vec<Address>>;
    async fn find_addresses_by_type_and_city(
        &self,
        address_type: AddressType,
        city_id: Uuid,
    ) -> BankingResult<Vec<Address>>;

    // Messaging methods
    async fn create_messaging(&self, messaging: Messaging) -> BankingResult<Messaging>;
    async fn find_messaging_by_id(&self, id: Uuid) -> BankingResult<Option<Messaging>>;
    async fn find_messaging_by_value(
        &self,
        value: HeaplessString<100>,
    ) -> BankingResult<Option<Messaging>>;

    // EntityReference methods
    async fn create_entity_reference(&self, entity_reference: EntityReference) -> BankingResult<EntityReference>;
    async fn find_entity_reference_by_id(&self, id: Uuid) -> BankingResult<Option<EntityReference>>;
    async fn find_entity_references_by_person_id(&self, person_id: Uuid) -> BankingResult<Vec<EntityReference>>;
    async fn find_entity_reference_by_person_and_role(
        &self,
        person_id: Uuid,
        entity_role: RelationshipRole,
    ) -> BankingResult<Option<EntityReference>>;
    async fn find_active_entity_references(&self) -> BankingResult<Vec<EntityReference>>;

    // Person methods
    async fn create_person(&self, person: Person) -> BankingResult<Person>;
    async fn find_person_by_id(&self, id: Uuid) -> BankingResult<Option<Person>>;
    async fn find_persons_by_type(&self, person_type: PersonType) -> BankingResult<Vec<Person>>;
    async fn get_person_by_external_identifier(
        &self,
        external_identifier: HeaplessString<50>,
    ) -> BankingResult<Option<Person>>;
}
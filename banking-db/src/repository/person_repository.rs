use async_trait::async_trait;
use std::error::Error;
use uuid::Uuid;

use crate::models::person::{
    AddressModel, AddressType, CityModel, CountryModel, EntityReferenceModel, MessagingModel,
    MessagingType, PersonModel, PersonType, RelationshipRole, StateProvinceModel,
};

#[async_trait]
pub trait PersonRepository: Send + Sync {
    /// Saves a new or updated person model.
    async fn save(&self, person: PersonModel) -> Result<PersonModel, Box<dyn Error + Send + Sync>>;

    /// Finds a single person by its primary key.
    async fn find_by_id(&self, id: Uuid) -> Result<Option<PersonModel>, Box<dyn Error + Send + Sync>>;

    /// Finds multiple persons by a list of primary keys.
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>>;

    /// Checks for the existence of a person by its primary key.
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>>;

    /// INDEX FINDER: Finds all person IDs for a given person type.
    async fn find_ids_by_person_type(
        &self,
        person_type: PersonType,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;

    /// PUBLIC FINDER: Finds a paginated list of persons for a given person type.
    async fn find_by_person_type(
        &self,
        person_type: PersonType,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>>;

    /// INDEX FINDER: Finds all person IDs by active status.
    async fn find_ids_by_is_active(&self, is_active: bool) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;

    /// PUBLIC FINDER: Finds a paginated list of persons by active status.
    async fn find_by_is_active(
        &self,
        is_active: bool,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>>;

    /// INDEX FINDER: Finds person IDs by external identifier hash.
    async fn get_ids_by_external_identifier(
        &self,
        identifier: &str,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;

    /// Get persons by external identifier
    async fn get_by_external_identifier(
        &self,
        identifier: &str,
    ) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>>;

    /// Get persons by entity reference
    async fn get_by_entity_reference(
        &self,
        entity_id: Uuid,
        entity_type: RelationshipRole,
    ) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>>;

    /// Create a person
    async fn create(
        &self,
        display_name: &str,
        person_type: crate::models::person::PersonType,
        external_identifier: Option<&str>,
    ) -> Result<PersonModel, Box<dyn std::error::Error + Send + Sync>>;

    /// Mark a person as duplicate of another
    async fn mark_as_duplicate(
        &self,
        person_id: Uuid,
        duplicate_of_person_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// Search persons by display name
    async fn search_by_name(
        &self,
        query: &str,
    ) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>>;

    /// Batch create persons
    async fn batch_create(
        &self,
        persons: Vec<PersonModel>,
    ) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait CountryRepository: Send + Sync {
    async fn save(&self, country: CountryModel) -> Result<CountryModel, Box<dyn Error + Send + Sync>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CountryModel>, Box<dyn Error + Send + Sync>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<CountryModel>, Box<dyn Error + Send + Sync>>;
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_iso2(&self, iso2: &str) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
    async fn find_by_iso2(
        &self,
        iso2: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<CountryModel>, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_is_active(&self, is_active: bool) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
    async fn find_by_is_active(
        &self,
        is_active: bool,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<CountryModel>, Box<dyn Error + Send + Sync>>;
}

#[async_trait]
pub trait StateProvinceRepository: Send + Sync {
    async fn save(
        &self,
        state: StateProvinceModel,
    ) -> Result<StateProvinceModel, Box<dyn Error + Send + Sync>>;
    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<StateProvinceModel>, Box<dyn Error + Send + Sync>>;
    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<StateProvinceModel>, Box<dyn Error + Send + Sync>>;
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_country_id(
        &self,
        country_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
    async fn find_by_country_id(
        &self,
        country_id: Uuid,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<StateProvinceModel>, Box<dyn Error + Send + Sync>>;
    async fn find_state_province_by_state_province_code(
        &self,
        country_id: Uuid,
        state_province_code: &str,
    ) -> Result<Option<StateProvinceModel>, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_is_active(&self, is_active: bool) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
    async fn find_by_is_active(
        &self,
        is_active: bool,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<StateProvinceModel>, Box<dyn Error + Send + Sync>>;
}

#[async_trait]
pub trait CityRepository: Send + Sync {
    async fn save(&self, city: CityModel) -> Result<CityModel, Box<dyn Error + Send + Sync>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CityModel>, Box<dyn Error + Send + Sync>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<CityModel>, Box<dyn Error + Send + Sync>>;
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_country_id(
        &self,
        country_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
    async fn find_by_country_id(
        &self,
        country_id: Uuid,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<CityModel>, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_state_id(&self, state_id: Uuid) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
    async fn find_by_state_id(
        &self,
        state_id: Uuid,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<CityModel>, Box<dyn Error + Send + Sync>>;
    async fn find_city_by_city_code(
        &self,
        country_id: Uuid,
        city_code: &str,
    ) -> Result<Option<CityModel>, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_is_active(&self, is_active: bool) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
    async fn find_by_is_active(
        &self,
        is_active: bool,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<CityModel>, Box<dyn Error + Send + Sync>>;
}

#[async_trait]
pub trait AddressRepository: Send + Sync {
    async fn save(&self, address: AddressModel) -> Result<AddressModel, Box<dyn Error + Send + Sync>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AddressModel>, Box<dyn Error + Send + Sync>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<AddressModel>, Box<dyn Error + Send + Sync>>;
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_address_type(
        &self,
        address_type: AddressType,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
    async fn find_by_address_type(
        &self,
        address_type: AddressType,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<AddressModel>, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_city_id(&self, city_id: Uuid) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
    async fn find_by_city_id(
        &self,
        city_id: Uuid,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<AddressModel>, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_is_active(&self, is_active: bool) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
    async fn find_by_is_active(
        &self,
        is_active: bool,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<AddressModel>, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_street_line1(
        &self,
        street_line1: &str,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
}

#[async_trait]
pub trait MessagingRepository: Send + Sync {
    async fn save(
        &self,
        messaging: MessagingModel,
    ) -> Result<MessagingModel, Box<dyn Error + Send + Sync>>;
    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<MessagingModel>, Box<dyn Error + Send + Sync>>;
    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<MessagingModel>, Box<dyn Error + Send + Sync>>;
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_messaging_type(
        &self,
        messaging_type: MessagingType,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
    async fn find_by_messaging_type(
        &self,
        messaging_type: MessagingType,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<MessagingModel>, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_is_active(&self, is_active: bool) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
    async fn find_by_is_active(
        &self,
        is_active: bool,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<MessagingModel>, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_value(
        &self,
        value: &str,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
}

#[async_trait]
pub trait EntityReferenceRepository: Send + Sync {
    async fn save(
        &self,
        entity_ref: EntityReferenceModel,
    ) -> Result<EntityReferenceModel, Box<dyn Error + Send + Sync>>;
    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<EntityReferenceModel>, Box<dyn Error + Send + Sync>>;
    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<EntityReferenceModel>, Box<dyn Error + Send + Sync>>;
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_person_id(
        &self,
        person_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
    async fn find_by_person_id(
        &self,
        person_id: Uuid,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<EntityReferenceModel>, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_entity_role(
        &self,
        entity_role: RelationshipRole,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
    async fn find_by_entity_role(
        &self,
        entity_role: RelationshipRole,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<EntityReferenceModel>, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_is_active(&self, is_active: bool) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
    async fn find_by_is_active(
        &self,
        is_active: bool,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<EntityReferenceModel>, Box<dyn Error + Send + Sync>>;
}
use async_trait::async_trait;
use std::error::Error;
use uuid::Uuid;

use crate::models::person::{
    LocationModel, LocationType, LocalityModel, CountryModel, EntityReferenceModel, MessagingModel,
    PersonModel, RelationshipRole, CountrySubdivisionModel,
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
}

#[async_trait]
pub trait CountrySubdivisionRepository: Send + Sync {
    async fn save(
        &self,
        country_subdivision: CountrySubdivisionModel,
    ) -> Result<CountrySubdivisionModel, Box<dyn Error + Send + Sync>>;
    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<CountrySubdivisionModel>, Box<dyn Error + Send + Sync>>;
    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<CountrySubdivisionModel>, Box<dyn Error + Send + Sync>>;
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
    ) -> Result<Vec<CountrySubdivisionModel>, Box<dyn Error + Send + Sync>>;
    async fn find_by_code(
        &self,
        country_id: Uuid,
        code: &str,
    ) -> Result<Option<CountrySubdivisionModel>, Box<dyn Error + Send + Sync>>;
}

#[async_trait]
pub trait LocalityRepository: Send + Sync {
    async fn save(&self, locality: LocalityModel) -> Result<LocalityModel, Box<dyn Error + Send + Sync>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<LocalityModel>, Box<dyn Error + Send + Sync>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<LocalityModel>, Box<dyn Error + Send + Sync>>;
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_country_subdivision_id(&self, country_subdivision_id: Uuid) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
    async fn find_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<LocalityModel>, Box<dyn Error + Send + Sync>>;
    async fn find_by_code(
        &self,
        country_id: Uuid,
        code: &str,
    ) -> Result<Option<LocalityModel>, Box<dyn Error + Send + Sync>>;
}

#[async_trait]
pub trait LocationRepository: Send + Sync {
    async fn save(&self, location: LocationModel) -> Result<LocationModel, Box<dyn Error + Send + Sync>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<LocationModel>, Box<dyn Error + Send + Sync>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<LocationModel>, Box<dyn Error + Send + Sync>>;
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_location_type(
        &self,
        location_type: LocationType,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
    async fn find_by_type_and_locality(
        &self,
        location_type: LocationType,
        locality_id: Uuid,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<LocationModel>, Box<dyn Error + Send + Sync>>;
    async fn find_ids_by_locality_id(&self, locality_id: Uuid) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
    async fn find_by_locality_id(
        &self,
        locality_id: Uuid,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<LocationModel>, Box<dyn Error + Send + Sync>>;
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
    async fn find_by_reference_external_id(
        &self,
        reference_external_id: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<EntityReferenceModel>, Box<dyn Error + Send + Sync>>;
}
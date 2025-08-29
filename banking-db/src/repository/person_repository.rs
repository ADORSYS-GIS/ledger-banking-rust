use async_trait::async_trait;
use sqlx::Database;
use uuid::Uuid;

use crate::models::person::{
    CountryModel, CountrySubdivisionModel, EntityReferenceModel, LocationModel, LocationType,
    LocalityModel, MessagingModel, PersonModel, PersonType, RelationshipRole,
};

#[async_trait]
pub trait PersonRepository<DB: Database>: Send + Sync {
    async fn save(&self, person: PersonModel) -> Result<PersonModel, sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<PersonModel>, sqlx::Error>;
    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>>;

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;

    async fn get_ids_by_external_identifier(
        &self,
        identifier: &str,
    ) -> Result<Vec<Uuid>, Box<dyn std::error::Error + Send + Sync>>;

    async fn get_by_external_identifier(
        &self,
        identifier: &str,
    ) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>>;

    async fn get_by_entity_reference(
        &self,
        entity_id: Uuid,
        entity_type: RelationshipRole,
    ) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>>;

    async fn create(
        &self,
        display_name: &str,
        person_type: PersonType,
        external_identifier: Option<&str>,
    ) -> Result<PersonModel, Box<dyn std::error::Error + Send + Sync>>;
    async fn mark_as_duplicate(
        &self,
        person_id: Uuid,
        duplicate_of_person_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn search_by_name(
        &self,
        query: &str,
    ) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>>;
    async fn batch_create(
        &self,
        persons: Vec<PersonModel>,
    ) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait CountryRepository<DB: Database>: Send + Sync {
    async fn save(&self, country: CountryModel) -> Result<CountryModel, sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CountryModel>, sqlx::Error>;
    async fn find_by_iso2(
        &self,
        iso2: &str,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<CountryModel>, sqlx::Error>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<CountryModel>, sqlx::Error>;
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_ids_by_iso2(
        &self,
        iso2: &str,
    ) -> Result<Vec<Uuid>, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait CountrySubdivisionRepository<DB: Database>: Send + Sync {
    async fn save(
        &self,
        country_subdivision: CountrySubdivisionModel,
    ) -> Result<CountrySubdivisionModel, sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CountrySubdivisionModel>, sqlx::Error>;
    async fn find_by_country_id(
        &self,
        country_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<CountrySubdivisionModel>, sqlx::Error>;
    async fn find_by_code(
        &self,
        country_id: Uuid,
        code: &str,
    ) -> Result<Option<CountrySubdivisionModel>, sqlx::Error>;
    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<CountrySubdivisionModel>, Box<dyn std::error::Error + Send + Sync>>;
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_ids_by_country_id(
        &self,
        country_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait LocalityRepository<DB: Database>: Send + Sync {
    async fn save(&self, locality: LocalityModel) -> Result<LocalityModel, sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<LocalityModel>, sqlx::Error>;
    async fn find_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<LocalityModel>, sqlx::Error>;
    async fn find_by_code(
        &self,
        country_id: Uuid,
        code: &str,
    ) -> Result<Option<LocalityModel>, sqlx::Error>;
    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<LocalityModel>, Box<dyn std::error::Error + Send + Sync>>;
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_ids_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait LocationRepository<DB: Database>: Send + Sync {
    async fn save(&self, location: LocationModel) -> Result<LocationModel, sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<LocationModel>, sqlx::Error>;
    async fn find_ids_by_street_line1(&self, street_line1: &str) -> Result<Vec<Uuid>, sqlx::Error>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<LocationModel>, sqlx::Error>;
    async fn find_by_locality_id(
        &self,
        locality_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<LocationModel>, sqlx::Error>;
    async fn find_by_type_and_locality(
        &self,
        location_type: LocationType,
        locality_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<LocationModel>, sqlx::Error>;
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_ids_by_location_type(
        &self,
        location_type: LocationType,
    ) -> Result<Vec<Uuid>, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_ids_by_locality_id(
        &self,
        locality_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait MessagingRepository<DB: Database>: Send + Sync {
    async fn save(
        &self,
        messaging: MessagingModel,
    ) -> Result<MessagingModel, sqlx::Error>;
    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<MessagingModel>, Box<dyn std::error::Error + Send + Sync>>;

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<MessagingModel>, Box<dyn std::error::Error + Send + Sync>>;

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;

    async fn find_ids_by_value(
        &self,
        value: &str,
    ) -> Result<Vec<Uuid>, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait EntityReferenceRepository<DB: Database>: Send + Sync {
    async fn save(
        &self,
        entity_ref: EntityReferenceModel,
    ) -> Result<EntityReferenceModel, sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<EntityReferenceModel>, sqlx::Error>;
    async fn find_by_person_id(
        &self,
        person_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<EntityReferenceModel>, sqlx::Error>;
    async fn find_by_reference_external_id(
        &self,
        reference_external_id: &str,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<EntityReferenceModel>, sqlx::Error>;
    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<EntityReferenceModel>, Box<dyn std::error::Error + Send + Sync>>;
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_ids_by_person_id(
        &self,
        person_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn std::error::Error + Send + Sync>>;
}
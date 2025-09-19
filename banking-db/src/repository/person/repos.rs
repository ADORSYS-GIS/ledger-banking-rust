use crate::repository::{country_repository::CountryRepository, country_subdivision_repository::CountrySubdivisionRepository, entity_reference_repository::EntityReferenceRepository, location_repository::LocationRepository, locality_repository::LocalityRepository, person_repository::PersonRepository};
use sqlx::Database;

pub trait PersonRepos<DB: Database>: Send + Sync {
    type PersonRepo: PersonRepository<DB> + Send + Sync;
    type CountryRepo: CountryRepository<DB> + Send + Sync;
    type CountrySubdivisionRepo: CountrySubdivisionRepository<DB> + Send + Sync;
    type LocalityRepo: LocalityRepository<DB> + Send + Sync;
    type LocationRepo: LocationRepository<DB> + Send + Sync;
    type EntityReferenceRepo: EntityReferenceRepository<DB> + Send + Sync;

    fn persons(&self) -> &Self::PersonRepo;
    fn countries(&self) -> &Self::CountryRepo;
    fn country_subdivisions(&self) -> &Self::CountrySubdivisionRepo;
    fn localities(&self) -> &Self::LocalityRepo;
    fn locations(&self) -> &Self::LocationRepo;
    fn entity_references(&self) -> &Self::EntityReferenceRepo;
}
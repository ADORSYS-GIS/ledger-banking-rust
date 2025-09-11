use crate::repository::{
    CountryRepository, CountrySubdivisionRepository, EntityReferenceRepository, LocationRepository,
    LocalityRepository, MessagingRepository, PersonRepository,
};
use sqlx::Database;

pub trait PersonRepos<DB: Database>: Send + Sync {
    type PersonRepo: PersonRepository<DB> + Send + Sync;
    type CountryRepo: CountryRepository<DB> + Send + Sync;
    type CountrySubdivisionRepo: CountrySubdivisionRepository<DB> + Send + Sync;
    type LocalityRepo: LocalityRepository<DB> + Send + Sync;
    type LocationRepo: LocationRepository<DB> + Send + Sync;
    type MessagingRepo: MessagingRepository<DB> + Send + Sync;
    type EntityReferenceRepo: EntityReferenceRepository<DB> + Send + Sync;

    fn persons(&self) -> &Self::PersonRepo;
    fn countries(&self) -> &Self::CountryRepo;
    fn country_subdivisions(&self) -> &Self::CountrySubdivisionRepo;
    fn localities(&self) -> &Self::LocalityRepo;
    fn locations(&self) -> &Self::LocationRepo;
    fn messagings(&self) -> &Self::MessagingRepo;
    fn entity_references(&self) -> &Self::EntityReferenceRepo;
}
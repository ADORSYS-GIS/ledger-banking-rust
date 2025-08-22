use crate::domain::person::{
    Location, LocationType, Locality, Country, EntityReference, Messaging, Person, CountrySubdivision,
};
use crate::domain::AuditLog;
use crate::{BankingResult, PersonAudit};
use async_trait::async_trait;
use heapless::String as HeaplessString;
use uuid::Uuid;

#[async_trait]
pub trait PersonService: Send + Sync {
    // Country methods
    async fn create_country(&self, country: Country) -> BankingResult<Country>;
    async fn fix_country(&self, country: Country) -> BankingResult<Country>;
    async fn find_country_by_id(&self, id: Uuid) -> BankingResult<Option<Country>>;
    async fn find_country_by_iso2(&self, iso2: HeaplessString<2>) -> BankingResult<Option<Country>>;
    async fn get_all_countries(&self) -> BankingResult<Vec<Country>>;

    // CountrySubdivision methods
    async fn create_country_subdivision(&self, country_subdivision: CountrySubdivision) -> BankingResult<CountrySubdivision>;
    async fn fix_country_subdivision(&self, country_subdivision: CountrySubdivision) -> BankingResult<CountrySubdivision>;
    async fn find_country_subdivision_by_id(&self, id: Uuid) -> BankingResult<Option<CountrySubdivision>>;
    async fn find_country_subdivisions_by_country_id(&self, country_id: Uuid) -> BankingResult<Vec<CountrySubdivision>>;
    async fn find_country_subdivision_by_code(
        &self,
        country_id: Uuid,
        code: HeaplessString<10>,
    ) -> BankingResult<Option<CountrySubdivision>>;

    // Locality methods
    async fn create_locality(&self, locality: Locality) -> BankingResult<Locality>;
    async fn fix_locality(&self, locality: Locality) -> BankingResult<Locality>;
    async fn find_locality_by_id(&self, id: Uuid) -> BankingResult<Option<Locality>>;
    async fn find_localities_by_country_subdivision_id(&self, country_subdivision_id: Uuid) -> BankingResult<Vec<Locality>>;
    async fn find_locality_by_code(
        &self,
        country_id: Uuid,
        code: HeaplessString<50>,
    ) -> BankingResult<Option<Locality>>;

    // Location methods
    async fn create_location(&self, location: Location) -> BankingResult<Location>;
    async fn fix_location(&self, location: Location) -> BankingResult<Location>;
    async fn find_location_by_id(&self, id: Uuid) -> BankingResult<Option<Location>>;
    async fn find_locations_by_street_line1(
        &self,
        street_line1: HeaplessString<50>,
    ) -> BankingResult<Vec<Location>>;
    async fn find_locations_by_locality_id(&self, locality_id: Uuid) -> BankingResult<Vec<Location>>;
    async fn find_locations_by_type_and_locality(
        &self,
        location_type: LocationType,
        locality_id: Uuid,
    ) -> BankingResult<Vec<Location>>;

    // Messaging methods
    async fn create_messaging(&self, messaging: Messaging) -> BankingResult<Messaging>;
    async fn fix_messaging(&self, messaging: Messaging) -> BankingResult<Messaging>;
    async fn find_messaging_by_id(&self, id: Uuid) -> BankingResult<Option<Messaging>>;
    async fn find_messaging_by_value(
        &self,
        value: HeaplessString<100>,
    ) -> BankingResult<Option<Messaging>>;

    // EntityReference methods
    async fn create_entity_reference(&self, entity_reference: EntityReference, audit_log: AuditLog) -> BankingResult<EntityReference>;
    async fn fix_entity_reference(&self, entity_reference: EntityReference) -> BankingResult<EntityReference>;
    async fn find_entity_reference_by_id(&self, id: Uuid) -> BankingResult<Option<EntityReference>>;
    async fn find_entity_references_by_person_id(&self, person_id: Uuid) -> BankingResult<Vec<EntityReference>>;
    async fn find_entity_references_by_reference_external_id(
        &self,
        reference_external_id: HeaplessString<50>,
    ) -> BankingResult<Vec<EntityReference>>;

    // EntityReferenceAudit methods
    async fn find_entity_reference_audits_by_id(&self, id: Uuid) -> BankingResult<Vec<EntityReference>>;
    async fn find_entity_reference_audits_by_person_id(&self, person_id: Uuid) -> BankingResult<Vec<EntityReference>>;

    // Person methods
    async fn create_person(&self, person: Person) -> BankingResult<Person>;
    async fn find_person_by_id(&self, id: Uuid) -> BankingResult<Option<Person>>;
    async fn get_persons_by_external_identifier(
        &self,
        external_identifier: HeaplessString<50>,
    ) -> BankingResult<Vec<Person>>;

    // Person methods
    async fn create_person_audit(&self, person: PersonAudit) -> BankingResult<Person>;
    async fn find_person_audit_by_id(&self, id: Uuid) -> BankingResult<Option<Person>>;
    async fn get_persons_audit_by_external_identifier(
        &self,
        external_identifier: HeaplessString<50>,
    ) -> BankingResult<Vec<Person>>;
}
use async_trait::async_trait;
use banking_api::domain::person::{
    Location, LocationType, Locality, Country, EntityReference, Messaging, Person, CountrySubdivision,
};
use banking_api::service::PersonService;
use banking_api::BankingResult;
// use banking_db::repository::{
//     LocationRepository, LocalityRepository, CountryRepository, EntityReferenceRepository,
//     MessagingRepository, PersonRepository, CountrySubdivisionRepository,
// };
use heapless::String as HeaplessString;
use uuid::Uuid;

use crate::mappers::person_mapper::{ToDomain, ToModel};
use crate::services::repositories::Repositories;
use sqlx::Database;

pub struct PersonServiceImpl<DB: Database> {
    repositories: Repositories<DB>,
}

impl<DB: Database> PersonServiceImpl<DB> {
    pub fn new(repositories: Repositories<DB>) -> Self {
        Self { repositories }
    }
}

#[async_trait]
impl<DB: Database + Send + Sync> PersonService for PersonServiceImpl<DB> {
    async fn create_country(&self, country: Country) -> BankingResult<Country> {
        let model = country.to_model();
        let saved_model = self.repositories.country_repository.save(model).await?;
        Ok(saved_model.to_domain())
    }

    async fn fix_country(&self, country: Country) -> BankingResult<Country> {
        self.create_country(country).await
    }

    async fn find_country_by_id(&self, id: Uuid) -> BankingResult<Option<Country>> {
        let model_idx = self.repositories.country_repository.find_by_id(id).await?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .country_repository
                .load(idx.country_id)
                .await?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn find_country_by_iso2(&self, iso2: HeaplessString<2>) -> BankingResult<Option<Country>> {
        let model_ixes = self
            .repositories
            .country_repository
            .find_by_iso2(iso2.as_str(), 1, 1)
            .await?;
        if let Some(idx) = model_ixes.into_iter().next() {
            let model = self
                .repositories
                .country_repository
                .load(idx.country_id)
                .await?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn get_all_countries(&self) -> BankingResult<Vec<Country>> {
        let model_ixes = self.repositories.country_repository.find_by_ids(&[]).await?;
        let mut countries = Vec::new();
        for idx in model_ixes {
            let country_model = self
                .repositories
                .country_repository
                .load(idx.country_id)
                .await?;
            countries.push(country_model.to_domain());
        }
        Ok(countries)
    }

    async fn create_country_subdivision(&self, country_subdivision: CountrySubdivision) -> BankingResult<CountrySubdivision> {
        let model = country_subdivision.to_model();
        let saved_model = self
            .repositories
            .country_subdivision_repository
            .save(model)
            .await?;
        Ok(saved_model.to_domain())
    }

    async fn fix_country_subdivision(&self, country_subdivision: CountrySubdivision) -> BankingResult<CountrySubdivision> {
        self.create_country_subdivision(country_subdivision).await
    }

    async fn find_country_subdivision_by_id(&self, id: Uuid) -> BankingResult<Option<CountrySubdivision>> {
        let model_idx = self
            .repositories
            .country_subdivision_repository
            .find_by_id(id)
            .await?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .country_subdivision_repository
                .load(idx.country_subdivision_id)
                .await?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn find_country_subdivisions_by_country_id(&self, country_id: Uuid) -> BankingResult<Vec<CountrySubdivision>> {
        let model_ixes = self
            .repositories
            .country_subdivision_repository
            .find_by_country_id(country_id, 1, 1000)
            .await?;
        let mut subdivisions = Vec::new();
        for idx in model_ixes {
            let subdivision_model = self
                .repositories
                .country_subdivision_repository
                .load(idx.country_subdivision_id)
                .await?;
            subdivisions.push(subdivision_model.to_domain());
        }
        Ok(subdivisions)
    }

    async fn find_country_subdivision_by_code(
        &self,
        country_id: Uuid,
        code: HeaplessString<10>,
    ) -> BankingResult<Option<CountrySubdivision>> {
        let model_idx = self
            .repositories
            .country_subdivision_repository
            .find_by_code(country_id, code.as_str())
            .await?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .country_subdivision_repository
                .load(idx.country_subdivision_id)
                .await?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn create_locality(&self, locality: Locality) -> BankingResult<Locality> {
        let model = locality.to_model();
        let saved_model = self.repositories.locality_repository.save(model).await?;
        Ok(saved_model.to_domain())
    }

    async fn fix_locality(&self, locality: Locality) -> BankingResult<Locality> {
        self.create_locality(locality).await
    }

    async fn find_locality_by_id(&self, id: Uuid) -> BankingResult<Option<Locality>> {
        let model_idx = self.repositories.locality_repository.find_by_id(id).await?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .locality_repository
                .load(idx.locality_id)
                .await?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn find_localities_by_country_subdivision_id(&self, country_subdivision_id: Uuid) -> BankingResult<Vec<Locality>> {
        let model_ixes = self
            .repositories
            .locality_repository
            .find_by_country_subdivision_id(country_subdivision_id, 1, 1000)
            .await?;
        let mut localities = Vec::new();
        for idx in model_ixes {
            let locality_model = self
                .repositories
                .locality_repository
                .load(idx.locality_id)
                .await?;
            localities.push(locality_model.to_domain());
        }
        Ok(localities)
    }

    async fn find_locality_by_code(
        &self,
        country_id: Uuid,
        code: HeaplessString<50>,
    ) -> BankingResult<Option<Locality>> {
        let model_idx = self
            .repositories
            .locality_repository
            .find_by_code(country_id, code.as_str())
            .await?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .locality_repository
                .load(idx.locality_id)
                .await?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn create_location(&self, location: Location, audit_log: banking_api::domain::AuditLog) -> BankingResult<Location> {
        let model = location.to_model();
        let saved_model = self
            .repositories
            .location_repository
            .save(model, audit_log.id)
            .await?;
        Ok(saved_model.to_domain())
    }

    async fn fix_location(&self, location: Location) -> BankingResult<Location> {
        let audit_log = banking_api::domain::AuditLog {
            id: Uuid::new_v4(),
            updated_at: chrono::Utc::now(),
            updated_by_person_id: Uuid::new_v4(), // Placeholder
        };
        self.create_location(location, audit_log).await
    }

    async fn find_location_by_id(&self, id: Uuid) -> BankingResult<Option<Location>> {
        let model_idx = self.repositories.location_repository.find_by_id(id).await?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .location_repository
                .load(idx.location_id)
                .await?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn find_locations_by_street_line1(
        &self,
        street_line1: HeaplessString<50>,
    ) -> BankingResult<Vec<Location>> {
        let ids = self
            .repositories
            .location_repository
            .find_ids_by_street_line1(street_line1.as_str())
            .await?;
        let model_ixes = self.repositories.location_repository.find_by_ids(&ids).await?;
        let mut locations = Vec::new();
        for idx in model_ixes {
            let location_model = self
                .repositories
                .location_repository
                .load(idx.location_id)
                .await?;
            locations.push(location_model.to_domain());
        }
        Ok(locations)
    }

    async fn find_locations_by_locality_id(&self, locality_id: Uuid) -> BankingResult<Vec<Location>> {
        let model_ixes = self
            .repositories
            .location_repository
            .find_by_locality_id(locality_id, 1, 1000)
            .await?;
        let mut locations = Vec::new();
        for idx in model_ixes {
            let location_model = self
                .repositories
                .location_repository
                .load(idx.location_id)
                .await?;
            locations.push(location_model.to_domain());
        }
        Ok(locations)
    }

    async fn find_locations_by_type_and_locality(
        &self,
        location_type: LocationType,
        locality_id: Uuid,
    ) -> BankingResult<Vec<Location>> {
        let model_ixes = self
            .repositories
            .location_repository
            .find_by_type_and_locality(location_type.to_model(), locality_id, 1, 1000)
            .await?;
        let mut locations = Vec::new();
        for idx in model_ixes {
            let location_model = self
                .repositories
                .location_repository
                .load(idx.location_id)
                .await?;
            locations.push(location_model.to_domain());
        }
        Ok(locations)
    }

    async fn create_messaging(&self, messaging: Messaging, audit_log: banking_api::domain::AuditLog) -> BankingResult<Messaging> {
        let model = messaging.to_model();
        let saved_model = self
            .repositories
            .messaging_repository
            .save(model, audit_log.id)
            .await?;
        Ok(saved_model.to_domain())
    }

    async fn fix_messaging(&self, messaging: Messaging) -> BankingResult<Messaging> {
        let audit_log = banking_api::domain::AuditLog {
            id: Uuid::new_v4(),
            updated_at: chrono::Utc::now(),
            updated_by_person_id: Uuid::new_v4(), // Placeholder
        };
        self.create_messaging(messaging, audit_log).await
    }

    async fn find_messaging_by_id(&self, id: Uuid) -> BankingResult<Option<Messaging>> {
        let model_idx = self.repositories.messaging_repository.find_by_id(id).await?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .messaging_repository
                .load(idx.messaging_id)
                .await?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn find_messaging_by_value(
        &self,
        value: HeaplessString<100>,
    ) -> BankingResult<Option<Messaging>> {
        let ids = self
            .repositories
            .messaging_repository
            .find_ids_by_value(value.as_str())
            .await?;
        if let Some(id) = ids.first() {
            let model_idx = self.repositories.messaging_repository.find_by_id(*id).await?;
            if let Some(idx) = model_idx {
                let model = self
                    .repositories
                    .messaging_repository
                    .load(idx.messaging_id)
                    .await?;
                Ok(Some(model.to_domain()))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    async fn create_entity_reference(&self, entity_reference: EntityReference, audit_log: banking_api::domain::AuditLog) -> BankingResult<EntityReference> {
        let model = entity_reference.to_model();
        let saved_model = self
            .repositories
            .entity_reference_repository
            .save(model, audit_log.id)
            .await?;
        Ok(saved_model.to_domain())
    }

    async fn fix_entity_reference(&self, entity_reference: EntityReference) -> BankingResult<EntityReference> {
        let audit_log = banking_api::domain::AuditLog {
            id: Uuid::new_v4(),
            updated_at: chrono::Utc::now(),
            updated_by_person_id: Uuid::new_v4(), // Placeholder
        };
        self.create_entity_reference(entity_reference, audit_log).await
    }

    async fn find_entity_reference_by_id(&self, id: Uuid) -> BankingResult<Option<EntityReference>> {
        let model_idx = self
            .repositories
            .entity_reference_repository
            .find_by_id(id)
            .await?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .entity_reference_repository
                .load(idx.entity_reference_id)
                .await?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn find_entity_references_by_person_id(&self, person_id: Uuid) -> BankingResult<Vec<EntityReference>> {
        let model_ixes = self
            .repositories
            .entity_reference_repository
            .find_by_person_id(person_id, 1, 1000)
            .await?;
        let mut refs = Vec::new();
        for idx in model_ixes {
            let ref_model = self
                .repositories
                .entity_reference_repository
                .load(idx.entity_reference_id)
                .await?;
            refs.push(ref_model.to_domain());
        }
        Ok(refs)
    }

    async fn find_entity_references_by_reference_external_id(
        &self,
        reference_external_id: HeaplessString<50>,
    ) -> BankingResult<Vec<EntityReference>> {
        let model_ixes = self
            .repositories
            .entity_reference_repository
            .find_by_reference_external_id(reference_external_id.as_str(), 1, 1000)
            .await?;
        let mut refs = Vec::new();
        for idx in model_ixes {
            let ref_model = self
                .repositories
                .entity_reference_repository
                .load(idx.entity_reference_id)
                .await?;
            refs.push(ref_model.to_domain());
        }
        Ok(refs)
    }

    async fn create_person(&self, person: Person, audit_log: banking_api::domain::AuditLog) -> BankingResult<Person> {
        let model = person.to_model();
        let saved_model = self
            .repositories
            .person_repository
            .save(model, audit_log.id)
            .await?;
        Ok(saved_model.to_domain())
    }

    async fn find_person_by_id(&self, id: Uuid) -> BankingResult<Option<Person>> {
        let model_idx = self.repositories.person_repository.find_by_id(id).await?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .person_repository
                .load(idx.person_id)
                .await?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn get_persons_by_external_identifier(
        &self,
        external_identifier: HeaplessString<50>,
    ) -> BankingResult<Vec<Person>> {
        let model_ixes = self
            .repositories
            .person_repository
            .get_by_external_identifier(external_identifier.as_str())
            .await?;
        let mut persons = Vec::new();
        for idx in model_ixes {
            let person_model = self
                .repositories
                .person_repository
                .load(idx.person_id)
                .await?;
            persons.push(person_model.to_domain());
        }
        Ok(persons)
    }
}
use banking_api::domain::person::{
    Location, LocationType, Locality, Country, EntityReference, Person,
    PersonType, RelationshipRole, CountrySubdivision,
};
use banking_db::models::person::{
    LocationModel, LocalityModel, CountryModel, EntityReferenceModel, PersonModel,
    CountrySubdivisionModel,
};

pub trait ToDomain<D> {
    fn to_domain(self) -> D;
}

pub trait ToModel<M> {
    fn to_model(self) -> M;
}

impl ToDomain<Country> for CountryModel {
    fn to_domain(self) -> Country {
        Country {
            id: self.id,
            iso2: self.iso2,
            name_l1: self.name_l1,
            name_l2: self.name_l2,
            name_l3: self.name_l3,
        }
    }
}

impl ToModel<CountryModel> for Country {
    fn to_model(self) -> CountryModel {
        CountryModel {
            id: self.id,
            iso2: self.iso2,
            name_l1: self.name_l1,
            name_l2: self.name_l2,
            name_l3: self.name_l3,
        }
    }
}

impl ToDomain<CountrySubdivision> for CountrySubdivisionModel {
    fn to_domain(self) -> CountrySubdivision {
        CountrySubdivision {
            id: self.id,
            country_id: self.country_id,
            code: self.code,
            name_l1: self.name_l1,
            name_l2: self.name_l2,
            name_l3: self.name_l3,
        }
    }
}

impl ToModel<CountrySubdivisionModel> for CountrySubdivision {
    fn to_model(self) -> CountrySubdivisionModel {
        CountrySubdivisionModel {
            id: self.id,
            country_id: self.country_id,
            code: self.code,
            name_l1: self.name_l1,
            name_l2: self.name_l2,
            name_l3: self.name_l3,
        }
    }
}

impl ToDomain<Locality> for LocalityModel {
    fn to_domain(self) -> Locality {
        Locality {
            id: self.id,
            country_subdivision_id: self.country_subdivision_id,
            code: self.code,
            name_l1: self.name_l1,
            name_l2: self.name_l2,
            name_l3: self.name_l3,
        }
    }
}

impl ToModel<LocalityModel> for Locality {
    fn to_model(self) -> LocalityModel {
        LocalityModel {
            id: self.id,
            country_subdivision_id: self.country_subdivision_id,
            code: self.code,
            name_l1: self.name_l1,
            name_l2: self.name_l2,
            name_l3: self.name_l3,
        }
    }
}

impl ToDomain<Location> for LocationModel {
    fn to_domain(self) -> Location {
        Location {
            id: self.id,
            street_line1: self.street_line1,
            street_line2: self.street_line2,
            street_line3: self.street_line3,
            street_line4: self.street_line4,
            locality_id: self.locality_id,
            postal_code: self.postal_code,
            latitude: self.latitude,
            longitude: self.longitude,
            accuracy_meters: self.accuracy_meters,
            location_type: self.location_type.to_domain(),
        }
    }
}

impl ToModel<LocationModel> for Location {
    fn to_model(self) -> LocationModel {
        LocationModel {
            id: self.id,
            street_line1: self.street_line1,
            street_line2: self.street_line2,
            street_line3: self.street_line3,
            street_line4: self.street_line4,
            locality_id: self.locality_id,
            postal_code: self.postal_code,
            latitude: self.latitude,
            longitude: self.longitude,
            accuracy_meters: self.accuracy_meters,
            location_type: self.location_type.to_model(),
        }
    }
}


impl ToDomain<EntityReference> for EntityReferenceModel {
    fn to_domain(self) -> EntityReference {
        EntityReference {
            id: self.id,
            person_id: self.person_id,
            entity_role: self.entity_role.to_domain(),
            reference_external_id: self.reference_external_id,
            reference_details_l1: self.reference_details_l1,
            reference_details_l2: self.reference_details_l2,
            reference_details_l3: self.reference_details_l3,
        }
    }
}

impl ToModel<EntityReferenceModel> for EntityReference {
    fn to_model(self) -> EntityReferenceModel {
        EntityReferenceModel {
            id: self.id,
            person_id: self.person_id,
            entity_role: self.entity_role.to_model(),
            reference_external_id: self.reference_external_id,
            reference_details_l1: self.reference_details_l1,
            reference_details_l2: self.reference_details_l2,
            reference_details_l3: self.reference_details_l3,
        }
    }
}

impl ToDomain<Person> for PersonModel {
    fn to_domain(self) -> Person {
        Person {
            id: self.id,
            person_type: self.person_type.to_domain(),
            display_name: self.display_name,
            external_identifier: self.external_identifier,
            organization_person_id: self.organization_person_id,
            entity_reference_count: self.entity_reference_count,
            messaging_info1: self.messaging_info1,
            messaging_info2: self.messaging_info2,
            messaging_info3: self.messaging_info3,
            messaging_info4: self.messaging_info4,
            messaging_info5: self.messaging_info5,
            department: self.department,
            location_id: self.location_id,
            duplicate_of_person_id: self.duplicate_of_person_id,
        }
    }
}

impl ToModel<PersonModel> for Person {
    fn to_model(self) -> PersonModel {
        PersonModel {
            id: self.id,
            person_type: self.person_type.to_model(),
            display_name: self.display_name,
            external_identifier: self.external_identifier,
            organization_person_id: self.organization_person_id,
            entity_reference_count: self.entity_reference_count,
            messaging_info1: self.messaging_info1,
            messaging_info2: self.messaging_info2,
            messaging_info3: self.messaging_info3,
            messaging_info4: self.messaging_info4,
            messaging_info5: self.messaging_info5,
            department: self.department,
            location_id: self.location_id,
            duplicate_of_person_id: self.duplicate_of_person_id,
        }
    }
}

// Enum conversions
impl ToDomain<LocationType> for banking_db::models::person::LocationType {
    fn to_domain(self) -> LocationType {
        match self {
            banking_db::models::person::LocationType::Residential => LocationType::Residential,
            banking_db::models::person::LocationType::Business => LocationType::Business,
            banking_db::models::person::LocationType::Mailing => LocationType::Mailing,
            banking_db::models::person::LocationType::Temporary => LocationType::Temporary,
            banking_db::models::person::LocationType::Branch => LocationType::Branch,
            banking_db::models::person::LocationType::Community => LocationType::Community,
            banking_db::models::person::LocationType::Other => LocationType::Other,
        }
    }
}

impl ToModel<banking_db::models::person::LocationType> for LocationType {
    fn to_model(self) -> banking_db::models::person::LocationType {
        match self {
            LocationType::Residential => banking_db::models::person::LocationType::Residential,
            LocationType::Business => banking_db::models::person::LocationType::Business,
            LocationType::Mailing => banking_db::models::person::LocationType::Mailing,
            LocationType::Temporary => banking_db::models::person::LocationType::Temporary,
            LocationType::Branch => banking_db::models::person::LocationType::Branch,
            LocationType::Community => banking_db::models::person::LocationType::Community,
            LocationType::Other => banking_db::models::person::LocationType::Other,
        }
    }
}


impl ToDomain<PersonType> for banking_db::models::person::PersonType {
    fn to_domain(self) -> PersonType {
        match self {
            banking_db::models::person::PersonType::Natural => PersonType::Natural,
            banking_db::models::person::PersonType::Legal => PersonType::Legal,
            banking_db::models::person::PersonType::System => PersonType::System,
            banking_db::models::person::PersonType::Integration => PersonType::Integration,
            banking_db::models::person::PersonType::Unknown => PersonType::Unknown,
        }
    }
}

impl ToModel<banking_db::models::person::PersonType> for PersonType {
    fn to_model(self) -> banking_db::models::person::PersonType {
        match self {
            PersonType::Natural => banking_db::models::person::PersonType::Natural,
            PersonType::Legal => banking_db::models::person::PersonType::Legal,
            PersonType::System => banking_db::models::person::PersonType::System,
            PersonType::Integration => banking_db::models::person::PersonType::Integration,
            PersonType::Unknown => banking_db::models::person::PersonType::Unknown,
        }
    }
}

impl ToDomain<RelationshipRole> for banking_db::models::person::RelationshipRole {
    fn to_domain(self) -> RelationshipRole {
        match self {
            banking_db::models::person::RelationshipRole::Customer => RelationshipRole::Customer,
            banking_db::models::person::RelationshipRole::Employee => RelationshipRole::Employee,
            banking_db::models::person::RelationshipRole::Shareholder => RelationshipRole::Shareholder,
            banking_db::models::person::RelationshipRole::Director => RelationshipRole::Director,
            banking_db::models::person::RelationshipRole::BeneficialOwner => RelationshipRole::BeneficialOwner,
            banking_db::models::person::RelationshipRole::Agent => RelationshipRole::Agent,
            banking_db::models::person::RelationshipRole::Vendor => RelationshipRole::Vendor,
            banking_db::models::person::RelationshipRole::Partner => RelationshipRole::Partner,
            banking_db::models::person::RelationshipRole::RegulatoryContact => RelationshipRole::RegulatoryContact,
            banking_db::models::person::RelationshipRole::EmergencyContact => RelationshipRole::EmergencyContact,
            banking_db::models::person::RelationshipRole::SystemAdmin => RelationshipRole::SystemAdmin,
            banking_db::models::person::RelationshipRole::Other => RelationshipRole::Other,
        }
    }
}

impl ToModel<banking_db::models::person::RelationshipRole> for RelationshipRole {
    fn to_model(self) -> banking_db::models::person::RelationshipRole {
        match self {
            RelationshipRole::Customer => banking_db::models::person::RelationshipRole::Customer,
            RelationshipRole::Employee => banking_db::models::person::RelationshipRole::Employee,
            RelationshipRole::Shareholder => banking_db::models::person::RelationshipRole::Shareholder,
            RelationshipRole::Director => banking_db::models::person::RelationshipRole::Director,
            RelationshipRole::BeneficialOwner => banking_db::models::person::RelationshipRole::BeneficialOwner,
            RelationshipRole::Agent => banking_db::models::person::RelationshipRole::Agent,
            RelationshipRole::Vendor => banking_db::models::person::RelationshipRole::Vendor,
            RelationshipRole::Partner => banking_db::models::person::RelationshipRole::Partner,
            RelationshipRole::RegulatoryContact => banking_db::models::person::RelationshipRole::RegulatoryContact,
            RelationshipRole::EmergencyContact => banking_db::models::person::RelationshipRole::EmergencyContact,
            RelationshipRole::SystemAdmin => banking_db::models::person::RelationshipRole::SystemAdmin,
            RelationshipRole::Other => banking_db::models::person::RelationshipRole::Other,
        }
    }
}
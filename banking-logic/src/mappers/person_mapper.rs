use banking_api::domain::person::{
    Location, LocationType, Locality, Country, EntityReference, Messaging, MessagingType, Person,
    PersonType, RelationshipRole, CountrySubdivision,
};
use banking_db::models::person::{
    LocationModel, LocalityModel, CountryModel, EntityReferenceModel, MessagingModel, PersonModel,
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
            version: self.version,
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
            audit_log_id: self.audit_log_id,
        }
    }
}

impl ToModel<LocationModel> for Location {
    fn to_model(self) -> LocationModel {
        LocationModel {
            id: self.id,
            version: self.version,
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
            audit_log_id: self.audit_log_id,
        }
    }
}

impl ToDomain<Messaging> for MessagingModel {
    fn to_domain(self) -> Messaging {
        Messaging {
            id: self.id,
            version: self.version,
            messaging_type: self.messaging_type.to_domain(),
            value: self.value,
            other_type: self.other_type,
            audit_log_id: self.audit_log_id,
        }
    }
}

impl ToModel<MessagingModel> for Messaging {
    fn to_model(self) -> MessagingModel {
        MessagingModel {
            id: self.id,
            version: self.version,
            messaging_type: self.messaging_type.to_model(),
            value: self.value,
            other_type: self.other_type,
            audit_log_id: self.audit_log_id,
        }
    }
}

impl ToDomain<EntityReference> for EntityReferenceModel {
    fn to_domain(self) -> EntityReference {
        EntityReference {
            id: self.id,
            version: self.version,
            person_id: self.person_id,
            entity_role: self.entity_role.to_domain(),
            reference_external_id: self.reference_external_id,
            reference_details_l1: self.reference_details_l1,
            reference_details_l2: self.reference_details_l2,
            reference_details_l3: self.reference_details_l3,
            audit_log_id: self.audit_log_id,
        }
    }
}

impl ToModel<EntityReferenceModel> for EntityReference {
    fn to_model(self) -> EntityReferenceModel {
        EntityReferenceModel {
            id: self.id,
            version: self.version,
            person_id: self.person_id,
            entity_role: self.entity_role.to_model(),
            reference_external_id: self.reference_external_id,
            reference_details_l1: self.reference_details_l1,
            reference_details_l2: self.reference_details_l2,
            reference_details_l3: self.reference_details_l3,
            audit_log_id: self.audit_log_id,
        }
    }
}

impl ToDomain<Person> for PersonModel {
    fn to_domain(self) -> Person {
        Person {
            id: self.id,
            version: self.version,
            person_type: self.person_type.to_domain(),
            display_name: self.display_name,
            external_identifier: self.external_identifier,
            organization_person_id: self.organization_person_id,
            entity_reference_count: self.entity_reference_count,
            messaging1_id: self.messaging1_id,
            messaging1_type: self.messaging1_type.map(|t| t.to_domain()),
            messaging2_id: self.messaging2_id,
            messaging2_type: self.messaging2_type.map(|t| t.to_domain()),
            messaging3_id: self.messaging3_id,
            messaging3_type: self.messaging3_type.map(|t| t.to_domain()),
            messaging4_id: self.messaging4_id,
            messaging4_type: self.messaging4_type.map(|t| t.to_domain()),
            messaging5_id: self.messaging5_id,
            messaging5_type: self.messaging5_type.map(|t| t.to_domain()),
            department: self.department,
            location_id: self.location_id,
            duplicate_of_person_id: self.duplicate_of_person_id,
            audit_log_id: self.audit_log_id,
        }
    }
}

impl ToModel<PersonModel> for Person {
    fn to_model(self) -> PersonModel {
        PersonModel {
            id: self.id,
            version: self.version,
            person_type: self.person_type.to_model(),
            display_name: self.display_name,
            external_identifier: self.external_identifier,
            organization_person_id: self.organization_person_id,
            entity_reference_count: self.entity_reference_count,
            messaging1_id: self.messaging1_id,
            messaging1_type: self.messaging1_type.map(|t| t.to_model()),
            messaging2_id: self.messaging2_id,
            messaging2_type: self.messaging2_type.map(|t| t.to_model()),
            messaging3_id: self.messaging3_id,
            messaging3_type: self.messaging3_type.map(|t| t.to_model()),
            messaging4_id: self.messaging4_id,
            messaging4_type: self.messaging4_type.map(|t| t.to_model()),
            messaging5_id: self.messaging5_id,
            messaging5_type: self.messaging5_type.map(|t| t.to_model()),
            department: self.department,
            location_id: self.location_id,
            duplicate_of_person_id: self.duplicate_of_person_id,
            audit_log_id: self.audit_log_id,
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

impl ToDomain<MessagingType> for banking_db::models::person::MessagingType {
    fn to_domain(self) -> MessagingType {
        match self {
            banking_db::models::person::MessagingType::Email => MessagingType::Email,
            banking_db::models::person::MessagingType::Phone => MessagingType::Phone,
            banking_db::models::person::MessagingType::Sms => MessagingType::Sms,
            banking_db::models::person::MessagingType::WhatsApp => MessagingType::WhatsApp,
            banking_db::models::person::MessagingType::Telegram => MessagingType::Telegram,
            banking_db::models::person::MessagingType::Skype => MessagingType::Skype,
            banking_db::models::person::MessagingType::Teams => MessagingType::Teams,
            banking_db::models::person::MessagingType::Signal => MessagingType::Signal,
            banking_db::models::person::MessagingType::WeChat => MessagingType::WeChat,
            banking_db::models::person::MessagingType::Viber => MessagingType::Viber,
            banking_db::models::person::MessagingType::Messenger => MessagingType::Messenger,
            banking_db::models::person::MessagingType::LinkedIn => MessagingType::LinkedIn,
            banking_db::models::person::MessagingType::Slack => MessagingType::Slack,
            banking_db::models::person::MessagingType::Discord => MessagingType::Discord,
            banking_db::models::person::MessagingType::Other => MessagingType::Other,
        }
    }
}

impl ToModel<banking_db::models::person::MessagingType> for MessagingType {
    fn to_model(self) -> banking_db::models::person::MessagingType {
        match self {
            MessagingType::Email => banking_db::models::person::MessagingType::Email,
            MessagingType::Phone => banking_db::models::person::MessagingType::Phone,
            MessagingType::Sms => banking_db::models::person::MessagingType::Sms,
            MessagingType::WhatsApp => banking_db::models::person::MessagingType::WhatsApp,
            MessagingType::Telegram => banking_db::models::person::MessagingType::Telegram,
            MessagingType::Skype => banking_db::models::person::MessagingType::Skype,
            MessagingType::Teams => banking_db::models::person::MessagingType::Teams,
            MessagingType::Signal => banking_db::models::person::MessagingType::Signal,
            MessagingType::WeChat => banking_db::models::person::MessagingType::WeChat,
            MessagingType::Viber => banking_db::models::person::MessagingType::Viber,
            MessagingType::Messenger => banking_db::models::person::MessagingType::Messenger,
            MessagingType::LinkedIn => banking_db::models::person::MessagingType::LinkedIn,
            MessagingType::Slack => banking_db::models::person::MessagingType::Slack,
            MessagingType::Discord => banking_db::models::person::MessagingType::Discord,
            MessagingType::Other => banking_db::models::person::MessagingType::Other,
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
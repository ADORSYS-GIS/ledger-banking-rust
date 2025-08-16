use banking_api::domain::person::{
    Address, AddressType, City, Country, EntityReference, Messaging, MessagingType, Person,
    PersonType, RelationshipRole, StateProvince,
};
use banking_db::models::person::{
    AddressModel, CityModel, CountryModel, EntityReferenceModel, MessagingModel, PersonModel,
    StateProvinceModel,
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
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
            created_by_person_id: self.created_by_person_id,
            updated_by_person_id: self.updated_by_person_id,
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
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
            created_by_person_id: self.created_by_person_id,
            updated_by_person_id: self.updated_by_person_id,
        }
    }
}

impl ToDomain<StateProvince> for StateProvinceModel {
    fn to_domain(self) -> StateProvince {
        StateProvince {
            id: self.id,
            country_id: self.country_id,
            state_province_code: self.state_province_code,
            name_l1: self.name_l1,
            name_l2: self.name_l2,
            name_l3: self.name_l3,
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
            created_by_person_id: self.created_by_person_id,
            updated_by_person_id: self.updated_by_person_id,
        }
    }
}

impl ToModel<StateProvinceModel> for StateProvince {
    fn to_model(self) -> StateProvinceModel {
        StateProvinceModel {
            id: self.id,
            country_id: self.country_id,
            state_province_code: self.state_province_code,
            name_l1: self.name_l1,
            name_l2: self.name_l2,
            name_l3: self.name_l3,
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
            created_by_person_id: self.created_by_person_id,
            updated_by_person_id: self.updated_by_person_id,
        }
    }
}

impl ToDomain<City> for CityModel {
    fn to_domain(self) -> City {
        City {
            id: self.id,
            country_id: self.country_id,
            state_id: self.state_id,
            city_code: self.city_code,
            name_l1: self.name_l1,
            name_l2: self.name_l2,
            name_l3: self.name_l3,
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
            created_by_person_id: self.created_by_person_id,
            updated_by_person_id: self.updated_by_person_id,
        }
    }
}

impl ToModel<CityModel> for City {
    fn to_model(self) -> CityModel {
        CityModel {
            id: self.id,
            country_id: self.country_id,
            state_id: self.state_id,
            city_code: self.city_code,
            name_l1: self.name_l1,
            name_l2: self.name_l2,
            name_l3: self.name_l3,
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
            created_by_person_id: self.created_by_person_id,
            updated_by_person_id: self.updated_by_person_id,
        }
    }
}

impl ToDomain<Address> for AddressModel {
    fn to_domain(self) -> Address {
        Address {
            id: self.id,
            street_line1: self.street_line1,
            street_line2: self.street_line2,
            street_line3: self.street_line3,
            street_line4: self.street_line4,
            city_id: self.city_id,
            postal_code: self.postal_code,
            latitude: self.latitude,
            longitude: self.longitude,
            accuracy_meters: self.accuracy_meters,
            address_type: self.address_type.to_domain(),
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
            created_by_person_id: self.created_by_person_id,
            updated_by_person_id: self.updated_by_person_id,
        }
    }
}

impl ToModel<AddressModel> for Address {
    fn to_model(self) -> AddressModel {
        AddressModel {
            id: self.id,
            street_line1: self.street_line1,
            street_line2: self.street_line2,
            street_line3: self.street_line3,
            street_line4: self.street_line4,
            city_id: self.city_id,
            postal_code: self.postal_code,
            latitude: self.latitude,
            longitude: self.longitude,
            accuracy_meters: self.accuracy_meters,
            address_type: self.address_type.to_model(),
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
            created_by_person_id: self.created_by_person_id,
            updated_by_person_id: self.updated_by_person_id,
        }
    }
}

impl ToDomain<Messaging> for MessagingModel {
    fn to_domain(self) -> Messaging {
        Messaging {
            id: self.id,
            messaging_type: self.messaging_type.to_domain(),
            value: self.value,
            other_type: self.other_type,
            is_active: self.is_active,
            priority: self.priority,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl ToModel<MessagingModel> for Messaging {
    fn to_model(self) -> MessagingModel {
        MessagingModel {
            id: self.id,
            messaging_type: self.messaging_type.to_model(),
            value: self.value,
            other_type: self.other_type,
            is_active: self.is_active,
            priority: self.priority,
            created_at: self.created_at,
            updated_at: self.updated_at,
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
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
            created_by_person_id: self.created_by_person_id,
            updated_by_person_id: self.updated_by_person_id,
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
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
            created_by_person_id: self.created_by_person_id,
            updated_by_person_id: self.updated_by_person_id,
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
            location_address_id: self.location_address_id,
            duplicate_of_person_id: self.duplicate_of_person_id,
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
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
            location_address_id: self.location_address_id,
            duplicate_of_person_id: self.duplicate_of_person_id,
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

// Enum conversions
impl ToDomain<AddressType> for banking_db::models::person::AddressType {
    fn to_domain(self) -> AddressType {
        match self {
            banking_db::models::person::AddressType::Residential => AddressType::Residential,
            banking_db::models::person::AddressType::Business => AddressType::Business,
            banking_db::models::person::AddressType::Mailing => AddressType::Mailing,
            banking_db::models::person::AddressType::Temporary => AddressType::Temporary,
            banking_db::models::person::AddressType::Branch => AddressType::Branch,
            banking_db::models::person::AddressType::Community => AddressType::Community,
            banking_db::models::person::AddressType::Other => AddressType::Other,
        }
    }
}

impl ToModel<banking_db::models::person::AddressType> for AddressType {
    fn to_model(self) -> banking_db::models::person::AddressType {
        match self {
            AddressType::Residential => banking_db::models::person::AddressType::Residential,
            AddressType::Business => banking_db::models::person::AddressType::Business,
            AddressType::Mailing => banking_db::models::person::AddressType::Mailing,
            AddressType::Temporary => banking_db::models::person::AddressType::Temporary,
            AddressType::Branch => banking_db::models::person::AddressType::Branch,
            AddressType::Community => banking_db::models::person::AddressType::Community,
            AddressType::Other => banking_db::models::person::AddressType::Other,
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
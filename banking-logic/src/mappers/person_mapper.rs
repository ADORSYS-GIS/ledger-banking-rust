use banking_api::domain;
use banking_api::domain::person::MessagingType as DomainMessagingType;
use banking_db::models::person::{PersonType, PersonModel, RelationshipRole, EntityReferenceModel, MessagingType as DbMessagingType};

#[cfg(test)]
use heapless::String as HeaplessString;

/// Mapper for converting between domain and database models for Person
pub struct PersonMapper;

impl PersonMapper {
    /// Convert domain PersonType to database PersonType
    pub fn person_type_to_model(person_type: domain::PersonType) -> PersonType {
        match person_type {
            domain::PersonType::Natural => PersonType::Natural,
            domain::PersonType::Legal => PersonType::Legal,
            domain::PersonType::System => PersonType::System,
            domain::PersonType::Integration => PersonType::Integration,
            domain::PersonType::Unknown => PersonType::Unknown,
        }
    }

    /// Convert database PersonType to domain PersonType
    pub fn person_type_from_model(model: PersonType) -> domain::PersonType {
        match model {
            PersonType::Natural => domain::PersonType::Natural,
            PersonType::Legal => domain::PersonType::Legal,
            PersonType::System => domain::PersonType::System,
            PersonType::Integration => domain::PersonType::Integration,
            PersonType::Unknown => domain::PersonType::Unknown,
        }
    }

    /// Convert domain Person to database PersonModel
    pub fn to_model(person: domain::Person) -> PersonModel {
        PersonModel {
            person_id: person.person_id,
            person_type: Self::person_type_to_model(person.person_type),
            display_name: person.display_name,
            external_identifier: person.external_identifier,
            organization: person.organization,
            // Individual messaging fields
            messaging1_id: person.messaging1_id,
            messaging1_type: person.messaging1_type.map(Self::messaging_type_to_db),
            messaging2_id: person.messaging2_id,
            messaging2_type: person.messaging2_type.map(Self::messaging_type_to_db),
            messaging3_id: person.messaging3_id,
            messaging3_type: person.messaging3_type.map(Self::messaging_type_to_db),
            messaging4_id: person.messaging4_id,
            messaging4_type: person.messaging4_type.map(Self::messaging_type_to_db),
            messaging5_id: person.messaging5_id,
            messaging5_type: person.messaging5_type.map(Self::messaging_type_to_db),
            department: person.department,
            location: person.location,
            duplicate_of: person.duplicate_of,
            // Note: entity_reference and entity_type fields removed from Person domain model
            is_active: person.is_active,
            created_at: person.created_at,
            updated_at: person.updated_at,
        }
    }

    /// Convert database PersonModel to domain Person
    pub fn from_model(model: PersonModel) -> domain::Person {
        domain::Person {
            person_id: model.person_id,
            person_type: Self::person_type_from_model(model.person_type),
            display_name: model.display_name,
            external_identifier: model.external_identifier,
            organization: model.organization,
            // Individual messaging fields
            messaging1_id: model.messaging1_id,
            messaging1_type: model.messaging1_type.map(Self::messaging_type_from_db),
            messaging2_id: model.messaging2_id,
            messaging2_type: model.messaging2_type.map(Self::messaging_type_from_db),
            messaging3_id: model.messaging3_id,
            messaging3_type: model.messaging3_type.map(Self::messaging_type_from_db),
            messaging4_id: model.messaging4_id,
            messaging4_type: model.messaging4_type.map(Self::messaging_type_from_db),
            messaging5_id: model.messaging5_id,
            messaging5_type: model.messaging5_type.map(Self::messaging_type_from_db),
            department: model.department,
            location: model.location,
            duplicate_of: model.duplicate_of,
            // Note: entity_reference and entity_type fields removed from Person domain model
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }

    /// Convert a vector of database models to domain models
    pub fn from_models(models: Vec<PersonModel>) -> Vec<domain::Person> {
        models.into_iter().map(Self::from_model).collect()
    }

    /// Convert a vector of domain models to database models
    pub fn to_models(persons: Vec<domain::Person>) -> Vec<PersonModel> {
        persons.into_iter().map(Self::to_model).collect()
    }

    /// Convert domain MessagingType to database MessagingType
    fn messaging_type_to_db(messaging_type: DomainMessagingType) -> DbMessagingType {
        match messaging_type {
            DomainMessagingType::Email => DbMessagingType::Email,
            DomainMessagingType::Phone => DbMessagingType::Phone,
            DomainMessagingType::Sms => DbMessagingType::Sms,
            DomainMessagingType::WhatsApp => DbMessagingType::WhatsApp,
            DomainMessagingType::Telegram => DbMessagingType::Telegram,
            DomainMessagingType::Skype => DbMessagingType::Skype,
            DomainMessagingType::Teams => DbMessagingType::Teams,
            DomainMessagingType::Signal => DbMessagingType::Signal,
            DomainMessagingType::WeChat => DbMessagingType::WeChat,
            DomainMessagingType::Viber => DbMessagingType::Viber,
            DomainMessagingType::Messenger => DbMessagingType::Messenger,
            DomainMessagingType::LinkedIn => DbMessagingType::LinkedIn,
            DomainMessagingType::Slack => DbMessagingType::Slack,
            DomainMessagingType::Discord => DbMessagingType::Discord,
            DomainMessagingType::Other => DbMessagingType::Other,
        }
    }

    /// Convert database MessagingType to domain MessagingType
    fn messaging_type_from_db(db_type: DbMessagingType) -> DomainMessagingType {
        match db_type {
            DbMessagingType::Email => DomainMessagingType::Email,
            DbMessagingType::Phone => DomainMessagingType::Phone,
            DbMessagingType::Sms => DomainMessagingType::Sms,
            DbMessagingType::WhatsApp => DomainMessagingType::WhatsApp,
            DbMessagingType::Telegram => DomainMessagingType::Telegram,
            DbMessagingType::Skype => DomainMessagingType::Skype,
            DbMessagingType::Teams => DomainMessagingType::Teams,
            DbMessagingType::Signal => DomainMessagingType::Signal,
            DbMessagingType::WeChat => DomainMessagingType::WeChat,
            DbMessagingType::Viber => DomainMessagingType::Viber,
            DbMessagingType::Messenger => DomainMessagingType::Messenger,
            DbMessagingType::LinkedIn => DomainMessagingType::LinkedIn,
            DbMessagingType::Slack => DomainMessagingType::Slack,
            DbMessagingType::Discord => DomainMessagingType::Discord,
            DbMessagingType::Other => DomainMessagingType::Other,
        }
    }

    // EntityReference mappers (for the new separate entity reference table)

    /// Convert domain RelationshipRole to database RelationshipRole
    pub fn relationship_role_to_model(entity_role: domain::RelationshipRole) -> RelationshipRole {
        match entity_role {
            domain::RelationshipRole::Customer => RelationshipRole::Customer,
            domain::RelationshipRole::Employee => RelationshipRole::Employee,
            domain::RelationshipRole::Shareholder => RelationshipRole::Shareholder,
            domain::RelationshipRole::Director => RelationshipRole::Director,
            domain::RelationshipRole::BeneficialOwner => RelationshipRole::BeneficialOwner,
            domain::RelationshipRole::Agent => RelationshipRole::Agent,
            domain::RelationshipRole::Vendor => RelationshipRole::Vendor,
            domain::RelationshipRole::Partner => RelationshipRole::Partner,
            domain::RelationshipRole::RegulatoryContact => RelationshipRole::RegulatoryContact,
            domain::RelationshipRole::EmergencyContact => RelationshipRole::EmergencyContact,
            domain::RelationshipRole::SystemAdmin => RelationshipRole::SystemAdmin,
            domain::RelationshipRole::Other => RelationshipRole::Other,
        }
    }

    /// Convert database RelationshipRole to domain RelationshipRole
    pub fn relationship_role_from_model(model: RelationshipRole) -> domain::RelationshipRole {
        match model {
            RelationshipRole::Customer => domain::RelationshipRole::Customer,
            RelationshipRole::Employee => domain::RelationshipRole::Employee,
            RelationshipRole::Shareholder => domain::RelationshipRole::Shareholder,
            RelationshipRole::Director => domain::RelationshipRole::Director,
            RelationshipRole::BeneficialOwner => domain::RelationshipRole::BeneficialOwner,
            RelationshipRole::Agent => domain::RelationshipRole::Agent,
            RelationshipRole::Vendor => domain::RelationshipRole::Vendor,
            RelationshipRole::Partner => domain::RelationshipRole::Partner,
            RelationshipRole::RegulatoryContact => domain::RelationshipRole::RegulatoryContact,
            RelationshipRole::EmergencyContact => domain::RelationshipRole::EmergencyContact,
            RelationshipRole::SystemAdmin => domain::RelationshipRole::SystemAdmin,
            RelationshipRole::Other => domain::RelationshipRole::Other,
        }
    }

    /// Placeholder mapper for EntityReferenceModel (until domain model exists)
    pub fn entity_reference_from_model(model: EntityReferenceModel) -> (uuid::Uuid, uuid::Uuid, RelationshipRole, Option<String>) {
        (
            model.id,
            model.person_id,
            model.entity_role,
            model.reference_external_id.map(|s| s.to_string())
        )
    }

    /// Placeholder mapper to EntityReferenceModel (until domain model exists)
    pub fn entity_reference_to_model(
        id: uuid::Uuid,
        person_id: uuid::Uuid,
        entity_role: RelationshipRole,
        reference_external_id: Option<&str>,
        created_by: uuid::Uuid,
    ) -> EntityReferenceModel {
        use chrono::Utc;
        use heapless::String as HeaplessString;
        
        EntityReferenceModel {
            id,
            person_id,
            entity_role,
            reference_external_id: reference_external_id.map(|s| HeaplessString::try_from(s).unwrap_or_default()),
            reference_details_l1: None,
            reference_details_l2: None,
            reference_details_l3: None,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by,
            updated_by: created_by,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use chrono::Utc;


    #[test]
    fn test_person_type_mapping() {
        let domain_types = vec![
            domain::PersonType::Natural,
            domain::PersonType::Legal,
            domain::PersonType::System,
            domain::PersonType::Integration,
            domain::PersonType::Unknown,
        ];

        for domain_type in domain_types {
            let model_type = PersonMapper::person_type_to_model(domain_type);
            let back_to_domain = PersonMapper::person_type_from_model(model_type);
            assert_eq!(domain_type, back_to_domain);
        }
    }

    #[test]
    fn test_person_mapping() {
        let person_id = Uuid::new_v4();
        let now = Utc::now();
        
        let domain_person = domain::Person {
            person_id,
            person_type: domain::PersonType::Natural,
            display_name: HeaplessString::try_from("John Doe").unwrap(),
            external_identifier: Some(HeaplessString::try_from("EMP001").unwrap()),
            organization: Some(Uuid::new_v4()), // Changed to UUID reference
            // Individual messaging fields
            messaging1_id: Some(Uuid::new_v4()),
            messaging1_type: Some(DomainMessagingType::Email),
            messaging2_id: None,
            messaging2_type: None,
            messaging3_id: None,
            messaging3_type: None,
            messaging4_id: None,
            messaging4_type: None,
            messaging5_id: None,
            messaging5_type: None,
            department: Some(HeaplessString::try_from("Engineering").unwrap()),
            location: Some(Uuid::new_v4()), // Use correct field name for address reference
            duplicate_of: None,
            // Note: entity_reference and entity_type fields removed
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        let model = PersonMapper::to_model(domain_person.clone());
        let back_to_domain = PersonMapper::from_model(model);

        assert_eq!(domain_person.person_id, back_to_domain.person_id);
        assert_eq!(domain_person.person_type, back_to_domain.person_type);
        assert_eq!(domain_person.display_name, back_to_domain.display_name);
        assert_eq!(domain_person.external_identifier, back_to_domain.external_identifier);
        assert_eq!(domain_person.organization, back_to_domain.organization);
        assert_eq!(domain_person.is_active, back_to_domain.is_active);
        assert_eq!(domain_person.created_at, back_to_domain.created_at);
        assert_eq!(domain_person.updated_at, back_to_domain.updated_at);
    }
}
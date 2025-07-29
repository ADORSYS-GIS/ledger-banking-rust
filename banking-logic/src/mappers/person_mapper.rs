use banking_api::domain;
use banking_db::models::person::{PersonType, PersonModel};
use heapless::Vec as HeaplessVec;

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
            messaging: serde_json::to_value(&person.messaging).unwrap_or(serde_json::Value::Array(vec![])),
            department: person.department,
            location: person.location,
            duplicate_of: person.duplicate_of,
            entity_reference: person.entity_reference,
            entity_type: person.entity_type,
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
            messaging: serde_json::from_value(model.messaging).unwrap_or_else(|_| HeaplessVec::new()),
            department: model.department,
            location: model.location,
            duplicate_of: model.duplicate_of,
            entity_reference: model.entity_reference,
            entity_type: model.entity_type,
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
            messaging: HeaplessVec::new(), // Use correct field name
            department: Some(HeaplessString::try_from("Engineering").unwrap()),
            location: Some(Uuid::new_v4()), // Use correct field name for address reference
            duplicate_of: None,
            entity_reference: Some(Uuid::new_v4()),
            entity_type: Some(HeaplessString::try_from("employee").unwrap()),
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
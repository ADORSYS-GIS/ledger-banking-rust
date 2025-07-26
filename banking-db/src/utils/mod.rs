use serde::{Deserialize, Deserializer, Serializer};
use crate::models::referenced_person::PersonType;

/// Serialize PersonType for database compatibility
pub fn serialize_person_type<S>(person_type: &PersonType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let person_type_str = match person_type {
        PersonType::Natural => "Natural",
        PersonType::Legal => "Legal",
        PersonType::System => "System",
        PersonType::Integration => "Integration",
        PersonType::Unknown => "Unknown",
    };
    serializer.serialize_str(person_type_str)
}

/// Deserialize PersonType from database string
pub fn deserialize_person_type<'de, D>(deserializer: D) -> Result<PersonType, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Natural" => Ok(PersonType::Natural),
        "Legal" => Ok(PersonType::Legal),
        "System" => Ok(PersonType::System),
        "Integration" => Ok(PersonType::Integration),
        "Unknown" => Ok(PersonType::Unknown),
        _ => Err(serde::de::Error::custom(format!("Invalid person type: {}", s))),
    }
}
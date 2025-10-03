use banking_db::models::person::{
    CountryModel, CountrySubdivisionModel, LocalityModel, LocationModel, LocationType,
    PersonModel, PersonType,
};
use heapless::String as HeaplessString;
use uuid::Uuid;

pub async fn setup_test_country() -> CountryModel {
    create_test_country_model("US", "United States")
}

pub fn create_test_person_model(name: &str) -> PersonModel {
    PersonModel {
        id: Uuid::new_v4(),
        person_type: PersonType::Natural,
        // "John Doe"
        display_name: HeaplessString::try_from(name).unwrap(),
        external_identifier: Some(
            HeaplessString::try_from(Uuid::new_v4().to_string().as_str()).unwrap(),
        ),
        entity_reference_count: 0,
        organization_person_id: None,
        messaging_info1: None,
        messaging_info2: None,
        messaging_info3: None,
        messaging_info4: None,
        messaging_info5: None,
        department: None,
        location_id: None,
        duplicate_of_person_id: None,
    }
}

pub fn create_test_country_model(iso2: &str, name_l1: &str) -> CountryModel {
    CountryModel {
        id: Uuid::new_v4(),
        // US
        iso2: HeaplessString::try_from(iso2).unwrap(),
        // "United States"
        name_l1: HeaplessString::try_from(name_l1).unwrap(),
        name_l2: None,
        name_l3: None,
    }
}

pub fn create_test_country_subdivision_model(
    country_id: Uuid,
    code: &str,
    name_l1: &str,
) -> CountrySubdivisionModel {
    CountrySubdivisionModel {
        id: Uuid::new_v4(),
        country_id,
        // "CA"
        code: HeaplessString::try_from(code).unwrap(),
        // "California"
        name_l1: HeaplessString::try_from(name_l1).unwrap(),
        name_l2: None,
        name_l3: None,
    }
}

pub fn create_test_locality_model(
    country_subdivision_id: Uuid,
    code: &str,
    name_l1: &str,
) -> LocalityModel {
    LocalityModel {
        id: Uuid::new_v4(),
        country_subdivision_id,
        // "LA"
        code: HeaplessString::try_from(code).unwrap(),
        // "Los Angeles"
        name_l1: HeaplessString::try_from(name_l1).unwrap(),
        name_l2: None,
        name_l3: None,
    }
}

pub fn create_test_location_model(
    locality_id: Uuid,
    street_line1: &str,
    postal_code: &str,
) -> LocationModel {
    LocationModel {
        id: Uuid::new_v4(),
        location_type: LocationType::Residential,
        // format!("123 Main St {}", Uuid::new_v4()).as_str()
        street_line1: HeaplessString::try_from(street_line1).unwrap(),
        street_line2: None,
        street_line3: None,
        street_line4: None,
        locality_id,
        // "90210"
        postal_code: Some(HeaplessString::try_from(postal_code).unwrap()),
        latitude: None,
        longitude: None,
        accuracy_meters: None,
    }
}

use banking_db::models::person::{EntityReferenceModel, RelationshipRole};

pub fn create_test_entity_reference_model(
    person_id: Uuid,
    entity_role: RelationshipRole,
    reference_external_id: &str,
) -> EntityReferenceModel {
    EntityReferenceModel {
        id: Uuid::new_v4(),
        person_id,
        entity_role,
        reference_external_id: HeaplessString::try_from(reference_external_id).unwrap(),
        reference_details_l1: None,
        reference_details_l2: None,
        reference_details_l3: None,
    }
}
// FILE: banking-db-postgres/src/repository/person/country_repository/test_helpers.rs

#![cfg(test)]

use banking_db::models::person::CountryModel;
use heapless::String as HeaplessString;
use uuid::Uuid;

pub(crate) async fn setup_test_country() -> CountryModel {
    CountryModel {
        id: Uuid::new_v4(),
        iso2: HeaplessString::try_from("CM").unwrap(),
        name_l1: HeaplessString::try_from("Cameroon").unwrap(),
        name_l2: Some(HeaplessString::try_from("Cameroun").unwrap()),
        name_l3: None,
    }
}
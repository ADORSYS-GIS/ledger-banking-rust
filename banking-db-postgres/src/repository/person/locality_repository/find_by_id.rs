use banking_db::models::person::LocalityIdxModel;
use banking_db::repository::LocalityResult;
use crate::repository::person::locality_repository::LocalityRepositoryImpl;
use uuid::Uuid;

pub async fn find_by_id(repo: &LocalityRepositoryImpl, id: Uuid) -> LocalityResult<Option<LocalityIdxModel>> {
    Ok(repo.locality_idx_cache.read().await.get_by_primary(&id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use banking_db::repository::{CountryRepository, CountrySubdivisionRepository, LocalityRepository, PersonRepos};
    use crate::repository::person::test_helpers::{
        create_test_country_model, create_test_country_subdivision_model, create_test_locality_model,
    };
    use crate::test_helper::setup_test_context;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_find_by_id() {
        let ctx = setup_test_context().await.unwrap();
        let country_repo = ctx.person_repos().countries();
        let country_subdivision_repo = ctx.person_repos().country_subdivisions();
        let repo = ctx.person_repos().localities();

        let unique_iso2 = format!("L{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
        let country = create_test_country_model(&unique_iso2, "Test Country");
        country_repo.save(country.clone()).await.unwrap();

        let unique_subdivision_code = format!("LS{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
        let country_subdivision = create_test_country_subdivision_model(country.id, &unique_subdivision_code, "Test Subdivision");
        country_subdivision_repo
            .save(country_subdivision.clone())
            .await
            .unwrap();

        let unique_locality_code = format!("LC{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
        let new_locality = create_test_locality_model(country_subdivision.id, &unique_locality_code, "Test Locality");
        repo.save(new_locality.clone()).await.unwrap();

        let found_locality = repo.find_by_id(new_locality.id).await.unwrap().unwrap();
        assert_eq!(new_locality.id, found_locality.locality_id);
    }
}
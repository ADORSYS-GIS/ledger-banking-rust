use banking_db::models::person::{LocalityIdxModel, LocalityModel};
use banking_db::repository::{
    CountrySubdivisionRepository, LocalityRepositoryError, LocalityResult,
};
use crate::repository::executor::Executor;
use crate::repository::person::locality_repository::LocalityRepositoryImpl;
use std::hash::Hasher;

pub async fn save(repo: &LocalityRepositoryImpl, locality: LocalityModel) -> LocalityResult<LocalityModel> {
    if !repo
        .country_subdivision_repository
        .exists_by_id(locality.country_subdivision_id)
        .await
        .map_err(|e| LocalityRepositoryError::RepositoryError(e.into()))?
    {
        return Err(LocalityRepositoryError::CountrySubdivisionNotFound(
            locality.country_subdivision_id,
        ));
    }

    let query1 = sqlx::query(
        r#"
        INSERT INTO locality (id, country_subdivision_id, code, name_l1, name_l2, name_l3)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(locality.id)
    .bind(locality.country_subdivision_id)
    .bind(locality.code.as_str())
    .bind(locality.name_l1.as_str())
    .bind(locality.name_l2.as_ref().map(|s| s.as_str()))
    .bind(locality.name_l3.as_ref().map(|s| s.as_str()));

    let mut hasher = twox_hash::XxHash64::with_seed(0);
    hasher.write(locality.code.as_bytes());
    let code_hash = hasher.finish() as i64;

    let query2 = sqlx::query(
        r#"
        INSERT INTO locality_idx (locality_id, country_subdivision_id, code_hash)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(locality.id)
    .bind(locality.country_subdivision_id)
    .bind(code_hash);

    match &repo.executor {
        Executor::Pool(pool) => {
            query1
                .execute(&**pool)
                .await
                .map_err(|e| LocalityRepositoryError::RepositoryError(e.into()))?;
            query2
                .execute(&**pool)
                .await
                .map_err(|e| LocalityRepositoryError::RepositoryError(e.into()))?;
        }
        Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            query1
                .execute(&mut **tx)
                .await
                .map_err(|e| LocalityRepositoryError::RepositoryError(e.into()))?;
            query2
                .execute(&mut **tx)
                .await
                .map_err(|e| LocalityRepositoryError::RepositoryError(e.into()))?;
        }
    }

    let new_idx = LocalityIdxModel {
        locality_id: locality.id,
        country_subdivision_id: locality.country_subdivision_id,
        code_hash,
    };
    repo.locality_idx_cache.read().await.add(new_idx);

    Ok(locality)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::person::test_helpers::{
        create_test_country_model, create_test_country_subdivision_model, create_test_locality_model,
    };
    use crate::test_helper::setup_test_context;
    use banking_db::repository::{CountryRepository, CountrySubdivisionRepository, LocalityRepository, PersonRepos};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_save_locality() {
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
        let saved_locality = repo.save(new_locality.clone()).await.unwrap();
        assert_eq!(new_locality.id, saved_locality.id);
    }
}
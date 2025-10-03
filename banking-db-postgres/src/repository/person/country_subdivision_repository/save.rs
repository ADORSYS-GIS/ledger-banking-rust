use banking_db::models::person::{CountrySubdivisionIdxModel, CountrySubdivisionModel};
use banking_db::repository::{
    CountryRepository, CountrySubdivisionRepositoryError, CountrySubdivisionResult,
};
use std::hash::Hasher;

use crate::repository::executor::Executor;

use super::repo_impl::CountrySubdivisionRepositoryImpl;

pub async fn save(
    repo: &CountrySubdivisionRepositoryImpl,
    country_subdivision: CountrySubdivisionModel,
) -> CountrySubdivisionResult<CountrySubdivisionModel> {
    if !repo
        .country_repository
        .exists_by_id(country_subdivision.country_id)
        .await
        .map_err(|e| CountrySubdivisionRepositoryError::RepositoryError(e.into()))?
    {
        return Err(CountrySubdivisionRepositoryError::CountryNotFound(
            country_subdivision.country_id,
        ));
    }

    let query1 = sqlx::query(
        r#"
        INSERT INTO country_subdivision (id, country_id, code, name_l1, name_l2, name_l3)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(country_subdivision.id)
    .bind(country_subdivision.country_id)
    .bind(country_subdivision.code.as_str())
    .bind(country_subdivision.name_l1.as_str())
    .bind(
        country_subdivision
            .name_l2
            .as_ref()
            .map(|s| s.as_str()),
    )
    .bind(
        country_subdivision
            .name_l3
            .as_ref()
            .map(|s| s.as_str()),
    );

    let mut hasher = twox_hash::XxHash64::with_seed(0);
    hasher.write(country_subdivision.code.as_bytes());
    let code_hash = hasher.finish() as i64;

    let query2 = sqlx::query(
        r#"
        INSERT INTO country_subdivision_idx (country_subdivision_id, country_id, code_hash)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(country_subdivision.id)
    .bind(country_subdivision.country_id)
    .bind(code_hash);

    match &repo.executor {
        Executor::Pool(pool) => {
            query1
                .execute(&**pool)
                .await
                .map_err(|e| CountrySubdivisionRepositoryError::RepositoryError(e.into()))?;
            query2
                .execute(&**pool)
                .await
                .map_err(|e| CountrySubdivisionRepositoryError::RepositoryError(e.into()))?;
        }
        Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            query1
                .execute(&mut **tx)
                .await
                .map_err(|e| CountrySubdivisionRepositoryError::RepositoryError(e.into()))?;
            query2
                .execute(&mut **tx)
                .await
                .map_err(|e| CountrySubdivisionRepositoryError::RepositoryError(e.into()))?;
        }
    }

    let idx_model = CountrySubdivisionIdxModel {
        country_subdivision_id: country_subdivision.id,
        country_id: country_subdivision.country_id,
        code_hash,
    };
    repo.country_subdivision_idx_cache
        .read()
        .await
        .add(idx_model);

    Ok(country_subdivision)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::person::test_helpers::{
        create_test_country_model, create_test_country_subdivision_model,
    };
    use crate::test_helper::setup_test_context;
    use banking_db::repository::{CountryRepository, CountrySubdivisionRepository, PersonRepos};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_save_country_subdivision() {
        let ctx = setup_test_context().await.unwrap();
        let country_repo = ctx.person_repos().countries();
        let subdivision_repo = ctx.person_repos().country_subdivisions();

        let unique_iso2 = format!("C{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
        let country = create_test_country_model(&unique_iso2, "Test Country");
        country_repo.save(country.clone()).await.unwrap();

        let unique_code = format!("S{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
        let new_subdivision =
            create_test_country_subdivision_model(country.id, &unique_code, "Test Subdivision");
        let saved_subdivision = subdivision_repo
            .save(new_subdivision.clone())
            .await
            .unwrap();

        assert_eq!(new_subdivision.id, saved_subdivision.id);
    }
}
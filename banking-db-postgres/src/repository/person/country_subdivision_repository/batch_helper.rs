use crate::repository::person::country_subdivision_repository::CountrySubdivisionRepositoryImpl;
use std::error::Error;
use uuid::Uuid;

type CountrySubdivisionTuple = (
    Uuid,
    Uuid,
    String,
    String,
    Option<String>,
    Option<String>,
);

type CountrySubdivisionIdxTuple = (
    Uuid,
    Uuid,
    i64,
);

pub async fn execute_country_subdivision_insert(
    repo: &CountrySubdivisionRepositoryImpl,
    values: Vec<CountrySubdivisionTuple>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut ids = Vec::new();
    let mut country_ids = Vec::new();
    let mut codes = Vec::new();
    let mut names_l1 = Vec::new();
    let mut names_l2 = Vec::new();
    let mut names_l3 = Vec::new();

    for v in values {
        ids.push(v.0);
        country_ids.push(v.1);
        codes.push(v.2);
        names_l1.push(v.3);
        names_l2.push(v.4);
        names_l3.push(v.5);
    }

    let query = r#"
        INSERT INTO country_subdivision (id, country_id, code, name_l1, name_l2, name_l3)
        SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::text[], $4::text[], $5::text[], $6::text[])
    "#;

    match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(query)
                .bind(&ids)
                .bind(&country_ids)
                .bind(&codes)
                .bind(&names_l1)
                .bind(&names_l2)
                .bind(&names_l3)
                .execute(&**pool)
                .await?;
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(query)
                .bind(&ids)
                .bind(&country_ids)
                .bind(&codes)
                .bind(&names_l1)
                .bind(&names_l2)
                .bind(&names_l3)
                .execute(&mut **tx)
                .await?;
        }
    }
    Ok(())
}

pub async fn execute_country_subdivision_idx_insert(
    repo: &CountrySubdivisionRepositoryImpl,
    values: Vec<CountrySubdivisionIdxTuple>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut country_subdivision_ids = Vec::new();
    let mut country_ids = Vec::new();
    let mut code_hashes = Vec::new();

    for v in values {
        country_subdivision_ids.push(v.0);
        country_ids.push(v.1);
        code_hashes.push(v.2);
    }

    let query = r#"
        INSERT INTO country_subdivision_idx (country_subdivision_id, country_id, code_hash)
        SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::bigint[])
    "#;

    match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(query)
                .bind(&country_subdivision_ids)
                .bind(&country_ids)
                .bind(&code_hashes)
                .execute(&**pool)
                .await?;
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(query)
                .bind(&country_subdivision_ids)
                .bind(&country_ids)
                .bind(&code_hashes)
                .execute(&mut **tx)
                .await?;
        }
    }
    Ok(())
}

pub async fn execute_country_subdivision_update(
    repo: &CountrySubdivisionRepositoryImpl,
    values: Vec<CountrySubdivisionTuple>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut ids = Vec::new();
    let mut country_ids = Vec::new();
    let mut codes = Vec::new();
    let mut names_l1 = Vec::new();
    let mut names_l2 = Vec::new();
    let mut names_l3 = Vec::new();

    for v in values {
        ids.push(v.0);
        country_ids.push(v.1);
        codes.push(v.2);
        names_l1.push(v.3);
        names_l2.push(v.4);
        names_l3.push(v.5);
    }

    let query = r#"
        UPDATE country_subdivision SET
            country_id = u.country_id,
            code = u.code,
            name_l1 = u.name_l1,
            name_l2 = u.name_l2,
            name_l3 = u.name_l3
        FROM (SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::text[], $4::text[], $5::text[], $6::text[]))
        AS u(id, country_id, code, name_l1, name_l2, name_l3)
        WHERE country_subdivision.id = u.id
    "#;

    match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(query)
                .bind(&ids)
                .bind(&country_ids)
                .bind(&codes)
                .bind(&names_l1)
                .bind(&names_l2)
                .bind(&names_l3)
                .execute(&**pool)
                .await?;
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(query)
                .bind(&ids)
                .bind(&country_ids)
                .bind(&codes)
                .bind(&names_l1)
                .bind(&names_l2)
                .bind(&names_l3)
                .execute(&mut **tx)
                .await?;
        }
    }
    Ok(())
}

pub async fn execute_country_subdivision_idx_update(
    repo: &CountrySubdivisionRepositoryImpl,
    values: Vec<CountrySubdivisionIdxTuple>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut country_subdivision_ids = Vec::new();
    let mut country_ids = Vec::new();
    let mut code_hashes = Vec::new();

    for v in values {
        country_subdivision_ids.push(v.0);
        country_ids.push(v.1);
        code_hashes.push(v.2);
    }

    let query = r#"
        UPDATE country_subdivision_idx SET
            country_id = u.country_id,
            code_hash = u.code_hash
        FROM (SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::bigint[]))
        AS u(country_subdivision_id, country_id, code_hash)
        WHERE country_subdivision_idx.country_subdivision_id = u.country_subdivision_id
    "#;

    match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(query)
                .bind(&country_subdivision_ids)
                .bind(&country_ids)
                .bind(&code_hashes)
                .execute(&**pool)
                .await?;
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(query)
                .bind(&country_subdivision_ids)
                .bind(&country_ids)
                .bind(&code_hashes)
                .execute(&mut **tx)
                .await?;
        }
    }
    Ok(())
}
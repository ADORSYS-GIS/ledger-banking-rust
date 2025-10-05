use crate::repository::executor::Executor;
use heapless::String as HeaplessString;
use std::error::Error;
use uuid::Uuid;

pub type LocalityTuple = (
    Uuid,
    Uuid,
    HeaplessString<50>,
    HeaplessString<50>,
    Option<HeaplessString<50>>,
    Option<HeaplessString<50>>,
);

pub type LocalityIdxTuple = (Uuid, Uuid, i64);

pub async fn execute_locality_insert(
    executor: &Executor,
    values: Vec<LocalityTuple>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (ids, subdivision_ids, codes, names_l1, names_l2, names_l3) =
        values.into_iter().fold(
            (
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
            |mut acc, val| {
                acc.0.push(val.0);
                acc.1.push(val.1);
                acc.2.push(val.2.to_string());
                acc.3.push(val.3.to_string());
                acc.4.push(val.4.map(|s| s.to_string()));
                acc.5.push(val.5.map(|s| s.to_string()));
                acc
            },
        );

    let query = r#"
        INSERT INTO locality (id, country_subdivision_id, code, name_l1, name_l2, name_l3)
        SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::varchar[], $4::varchar[], $5::varchar[], $6::varchar[])
    "#;

    match executor {
        Executor::Pool(pool) => {
            sqlx::query(query)
                .bind(ids)
                .bind(subdivision_ids)
                .bind(codes)
                .bind(names_l1)
                .bind(names_l2)
                .bind(names_l3)
                .execute(&**pool)
                .await?;
        }
        Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(query)
                .bind(ids)
                .bind(subdivision_ids)
                .bind(codes)
                .bind(names_l1)
                .bind(names_l2)
                .bind(names_l3)
                .execute(&mut **tx)
                .await?;
        }
    }
    Ok(())
}

pub async fn execute_locality_idx_insert(
    executor: &Executor,
    values: Vec<LocalityIdxTuple>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (locality_ids, subdivision_ids, code_hashes) = values.into_iter().fold(
        (Vec::new(), Vec::new(), Vec::new()),
        |mut acc, val| {
            acc.0.push(val.0);
            acc.1.push(val.1);
            acc.2.push(val.2);
            acc
        },
    );

    let query = r#"
        INSERT INTO locality_idx (locality_id, country_subdivision_id, code_hash)
        SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::bigint[])
    "#;

    match executor {
        Executor::Pool(pool) => {
            sqlx::query(query)
                .bind(locality_ids)
                .bind(subdivision_ids)
                .bind(code_hashes)
                .execute(&**pool)
                .await?;
        }
        Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(query)
                .bind(locality_ids)
                .bind(subdivision_ids)
                .bind(code_hashes)
                .execute(&mut **tx)
                .await?;
        }
    }
    Ok(())
}

pub async fn execute_locality_update(
    executor: &Executor,
    values: Vec<LocalityTuple>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (ids, subdivision_ids, codes, names_l1, names_l2, names_l3) =
        values.into_iter().fold(
            (
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
            |mut acc, val| {
                acc.0.push(val.0);
                acc.1.push(val.1);
                acc.2.push(val.2.to_string());
                acc.3.push(val.3.to_string());
                acc.4.push(val.4.map(|s| s.to_string()));
                acc.5.push(val.5.map(|s| s.to_string()));
                acc
            },
        );

    let query = r#"
        UPDATE locality SET
            country_subdivision_id = u.country_subdivision_id,
            code = u.code,
            name_l1 = u.name_l1,
            name_l2 = u.name_l2,
            name_l3 = u.name_l3
        FROM (SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::varchar[], $4::varchar[], $5::varchar[], $6::varchar[]))
        AS u(id, country_subdivision_id, code, name_l1, name_l2, name_l3)
        WHERE locality.id = u.id
    "#;

    match executor {
        Executor::Pool(pool) => {
            sqlx::query(query)
                .bind(ids)
                .bind(subdivision_ids)
                .bind(codes)
                .bind(names_l1)
                .bind(names_l2)
                .bind(names_l3)
                .execute(&**pool)
                .await?;
        }
        Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(query)
                .bind(ids)
                .bind(subdivision_ids)
                .bind(codes)
                .bind(names_l1)
                .bind(names_l2)
                .bind(names_l3)
                .execute(&mut **tx)
                .await?;
        }
    }
    Ok(())
}

pub async fn execute_locality_idx_update(
    executor: &Executor,
    values: Vec<LocalityIdxTuple>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (locality_ids, subdivision_ids, code_hashes) = values.into_iter().fold(
        (Vec::new(), Vec::new(), Vec::new()),
        |mut acc, val| {
            acc.0.push(val.0);
            acc.1.push(val.1);
            acc.2.push(val.2);
            acc
        },
    );

    let query = r#"
        UPDATE locality_idx SET
            country_subdivision_id = u.country_subdivision_id,
            code_hash = u.code_hash
        FROM (SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::bigint[]))
        AS u(locality_id, country_subdivision_id, code_hash)
        WHERE locality_idx.locality_id = u.locality_id
    "#;

    match executor {
        Executor::Pool(pool) => {
            sqlx::query(query)
                .bind(locality_ids)
                .bind(subdivision_ids)
                .bind(code_hashes)
                .execute(&**pool)
                .await?;
        }
        Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(query)
                .bind(locality_ids)
                .bind(subdivision_ids)
                .bind(code_hashes)
                .execute(&mut **tx)
                .await?;
        }
    }
    Ok(())
}
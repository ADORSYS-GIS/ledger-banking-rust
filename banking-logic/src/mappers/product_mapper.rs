use banking_api::domain::{Product as ApiProduct, ProductType as ApiProductType};
use banking_db::models::{Product as DbProduct, ProductType as DbProductType};

pub trait DBMapper<A, D> {
    fn to_db(api_model: A) -> D;
}

pub trait ApiMapper<D, A> {
    fn to_api(db_model: D) -> A;
}

impl DBMapper<ApiProduct, DbProduct> for DbProduct {
    fn to_db(api_model: ApiProduct) -> DbProduct {
        DbProduct {
            id: api_model.id,
            product_type: match api_model.product_type {
                ApiProductType::CASA => DbProductType::CASA,
                ApiProductType::LOAN => DbProductType::LOAN,
            },
            name: api_model.name,
            description: api_model.description,
            is_active: api_model.is_active,
            valid_from: api_model.valid_from,
            valid_to: api_model.valid_to,
            rules: api_model.rules,
            created_at: api_model.created_at,
            last_updated_at: api_model.last_updated_at,
            updated_by_person_id: api_model.updated_by_person_id,
        }
    }
}

impl ApiMapper<DbProduct, ApiProduct> for ApiProduct {
    fn to_api(db_model: DbProduct) -> ApiProduct {
        ApiProduct {
            id: db_model.id,
            product_type: match db_model.product_type {
                DbProductType::CASA => ApiProductType::CASA,
                DbProductType::LOAN => ApiProductType::LOAN,
            },
            name: db_model.name,
            description: db_model.description,
            is_active: db_model.is_active,
            valid_from: db_model.valid_from,
            valid_to: db_model.valid_to,
            rules: db_model.rules,
            created_at: db_model.created_at,
            last_updated_at: db_model.last_updated_at,
            updated_by_person_id: db_model.updated_by_person_id,
        }
    }
}
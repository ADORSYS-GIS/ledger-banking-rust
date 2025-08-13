use banking_api::domain::{
    GlMapping as ApiGlMapping, InterestRateTier as ApiInterestRateTier, Product as ApiProduct,
    ProductRules as ApiProductRules, ProductType as ApiProductType,
    PostingFrequency as ApiPostingFrequency, ProductAccrualFrequency as ApiProductAccrualFrequency
};
use banking_db::models::{
    GlMappingModel as DbGlMapping, InterestRateTierModel as DbInterestRateTier,
    ProductModel as DbProduct, ProductRules as DbProductRules, ProductType as DbProductType,
    PostingFrequency as DbPostingFrequency, ProductAccrualFrequency as DbProductAccrualFrequency
};
pub struct ProductMapper;

impl ProductMapper {
    pub fn to_db(api_model: ApiProduct) -> DbProduct {
        DbProduct {
            id: api_model.id,
            name_l1: api_model.name_l1,
            name_l2: api_model.name_l2,
            name_l3: api_model.name_l3,
            description: api_model.description,
            is_active: api_model.is_active,
            valid_from: api_model.valid_from,
            valid_to: api_model.valid_to,
            product_type: match api_model.product_type {
                ApiProductType::CASA => DbProductType::CASA,
                ApiProductType::LOAN => DbProductType::LOAN,
            },
            rules: ProductRulesMapper::to_db(api_model.rules),
            created_at: api_model.created_at,
            last_updated_at: api_model.last_updated_at,
            updated_by_person_id: api_model.updated_by_person_id,
        }
    }

    pub fn from_db(db_model: DbProduct) -> ApiProduct {
        ApiProduct {
            id: db_model.id,
            name_l1: db_model.name_l1,
            name_l2: db_model.name_l2,
            name_l3: db_model.name_l3,
            description: db_model.description,
            is_active: db_model.is_active,
            valid_from: db_model.valid_from,
            valid_to: db_model.valid_to,
            product_type: match db_model.product_type {
                DbProductType::CASA => ApiProductType::CASA,
                DbProductType::LOAN => ApiProductType::LOAN,
            },
            rules: ProductRulesMapper::from_db(db_model.rules),
            created_at: db_model.created_at,
            last_updated_at: db_model.last_updated_at,
            updated_by_person_id: db_model.updated_by_person_id,
        }
    }
}

pub struct ProductRulesMapper;

impl ProductRulesMapper {
    pub fn to_db(api_model: ApiProductRules) -> DbProductRules {
        DbProductRules {
            minimum_balance: api_model.minimum_balance,
            maximum_balance: api_model.maximum_balance,
            daily_transaction_limit: api_model.daily_transaction_limit,
            monthly_transaction_limit: api_model.monthly_transaction_limit,
            overdraft_allowed: api_model.overdraft_allowed,
            overdraft_limit: api_model.overdraft_limit,
            interest_calculation_method: api_model.interest_calculation_method,
            interest_posting_frequency: match api_model.interest_posting_frequency {
                ApiPostingFrequency::Daily => DbPostingFrequency::Daily,
                ApiPostingFrequency::Weekly => DbPostingFrequency::Weekly,
                ApiPostingFrequency::Monthly => DbPostingFrequency::Monthly,
                ApiPostingFrequency::Quarterly => DbPostingFrequency::Quarterly,
                ApiPostingFrequency::Annually => DbPostingFrequency::Annually,
            },
            dormancy_threshold_days: api_model.dormancy_threshold_days,
            minimum_opening_balance: api_model.minimum_opening_balance,
            closure_fee: api_model.closure_fee,
            maintenance_fee: api_model.maintenance_fee,
            maintenance_fee_frequency: api_model.maintenance_fee_frequency,
            default_dormancy_days: api_model.default_dormancy_days,
            default_overdraft_limit: api_model.default_overdraft_limit,
            per_transaction_limit: api_model.per_transaction_limit,
            overdraft_interest_rate: api_model.overdraft_interest_rate,
            accrual_frequency: match api_model.accrual_frequency {
                ApiProductAccrualFrequency::Daily => DbProductAccrualFrequency::Daily,
                ApiProductAccrualFrequency::BusinessDaysOnly => DbProductAccrualFrequency::BusinessDaysOnly,
                ApiProductAccrualFrequency::None => DbProductAccrualFrequency::None,
            },
        }
    }

    pub fn from_db(db_model: DbProductRules) -> ApiProductRules {
        ApiProductRules {
            minimum_balance: db_model.minimum_balance,
            maximum_balance: db_model.maximum_balance,
            daily_transaction_limit: db_model.daily_transaction_limit,
            monthly_transaction_limit: db_model.monthly_transaction_limit,
            overdraft_allowed: db_model.overdraft_allowed,
            overdraft_limit: db_model.overdraft_limit,
            interest_calculation_method: db_model.interest_calculation_method,
            interest_posting_frequency: match db_model.interest_posting_frequency {
                DbPostingFrequency::Daily => ApiPostingFrequency::Daily,
                DbPostingFrequency::Weekly => ApiPostingFrequency::Weekly,
                DbPostingFrequency::Monthly => ApiPostingFrequency::Monthly,
                DbPostingFrequency::Quarterly => ApiPostingFrequency::Quarterly,
                DbPostingFrequency::Annually => ApiPostingFrequency::Annually,
            },
            dormancy_threshold_days: db_model.dormancy_threshold_days,
            minimum_opening_balance: db_model.minimum_opening_balance,
            closure_fee: db_model.closure_fee,
            maintenance_fee: db_model.maintenance_fee,
            maintenance_fee_frequency: db_model.maintenance_fee_frequency,
            default_dormancy_days: db_model.default_dormancy_days,
            default_overdraft_limit: db_model.default_overdraft_limit,
            per_transaction_limit: db_model.per_transaction_limit,
            overdraft_interest_rate: db_model.overdraft_interest_rate,
            accrual_frequency: match db_model.accrual_frequency {
                DbProductAccrualFrequency::Daily => ApiProductAccrualFrequency::Daily,
                DbProductAccrualFrequency::BusinessDaysOnly => ApiProductAccrualFrequency::BusinessDaysOnly,
                DbProductAccrualFrequency::None => ApiProductAccrualFrequency::None,
            },
        }
    }
}

pub struct GlMappingMapper;

impl GlMappingMapper {
    pub fn to_db(api_model: ApiGlMapping) -> DbGlMapping {
        DbGlMapping {
            product_id: api_model.product_id,
            customer_account_code: api_model.customer_account_code,
            interest_expense_code: api_model.interest_expense_code,
            fee_income_code: api_model.fee_income_code,
            overdraft_code: api_model.overdraft_code,
        }
    }

    pub fn from_db(db_model: DbGlMapping) -> ApiGlMapping {
        ApiGlMapping {
            product_id: db_model.product_id,
            customer_account_code: db_model.customer_account_code,
            interest_expense_code: db_model.interest_expense_code,
            fee_income_code: db_model.fee_income_code,
            overdraft_code: db_model.overdraft_code,
        }
    }
}

pub struct InterestRateTierMapper;

impl InterestRateTierMapper {
    pub fn to_db(api_model: ApiInterestRateTier) -> DbInterestRateTier {
        DbInterestRateTier {
            minimum_balance: api_model.minimum_balance,
            maximum_balance: api_model.maximum_balance,
            interest_rate: api_model.interest_rate,
            tier_name: api_model.tier_name,
        }
    }

    pub fn from_db(db_model: DbInterestRateTier) -> ApiInterestRateTier {
        ApiInterestRateTier {
            minimum_balance: db_model.minimum_balance,
            maximum_balance: db_model.maximum_balance,
            interest_rate: db_model.interest_rate,
            tier_name: db_model.tier_name,
        }
    }
}
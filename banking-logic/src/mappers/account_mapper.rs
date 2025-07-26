use banking_api::domain::Account;
use banking_db::models::AccountModel;

pub struct AccountMapper;

impl AccountMapper {
    /// Map from domain Account to database AccountModel
    pub fn to_model(account: Account) -> AccountModel {
        AccountModel {
            account_id: account.account_id,
            product_code: account.product_code,
            account_type: account.account_type,
            account_status: account.account_status,
            signing_condition: account.signing_condition,
            currency: account.currency,
            open_date: account.open_date,
            domicile_branch_id: account.domicile_branch_id,
            current_balance: account.current_balance,
            available_balance: account.available_balance,
            accrued_interest: account.accrued_interest,
            overdraft_limit: account.overdraft_limit,
            original_principal: account.original_principal,
            outstanding_principal: account.outstanding_principal,
            loan_interest_rate: account.loan_interest_rate,
            loan_term_months: account.loan_term_months,
            disbursement_date: account.disbursement_date,
            maturity_date: account.maturity_date,
            installment_amount: account.installment_amount,
            next_due_date: account.next_due_date,
            penalty_rate: account.penalty_rate,
            collateral_id: account.collateral_id,
            loan_purpose_id: account.loan_purpose_id,
            // Enhanced lifecycle fields
            close_date: account.close_date,
            last_activity_date: account.last_activity_date,
            dormancy_threshold_days: account.dormancy_threshold_days,
            reactivation_required: account.reactivation_required,
            pending_closure_reason_id: account.pending_closure_reason_id,
            status_changed_by: account.status_changed_by,
            status_change_reason_id: account.status_change_reason_id,
            status_change_timestamp: account.status_change_timestamp,
            // Audit fields
            created_at: account.created_at,
            last_updated_at: account.last_updated_at,
            updated_by: account.updated_by,
        }
    }

    /// Map from database AccountModel to domain Account
    pub fn from_model(model: AccountModel) -> banking_api::BankingResult<Account> {
        Ok(Account {
            account_id: model.account_id,
            product_code: model.product_code,
            account_type: model.account_type,
            account_status: model.account_status,
            signing_condition: model.signing_condition,
            currency: model.currency,
            open_date: model.open_date,
            domicile_branch_id: model.domicile_branch_id,
            current_balance: model.current_balance,
            available_balance: model.available_balance,
            accrued_interest: model.accrued_interest,
            overdraft_limit: model.overdraft_limit,
            original_principal: model.original_principal,
            outstanding_principal: model.outstanding_principal,
            loan_interest_rate: model.loan_interest_rate,
            loan_term_months: model.loan_term_months,
            disbursement_date: model.disbursement_date,
            maturity_date: model.maturity_date,
            installment_amount: model.installment_amount,
            next_due_date: model.next_due_date,
            penalty_rate: model.penalty_rate,
            collateral_id: model.collateral_id,
            loan_purpose_id: model.loan_purpose_id,
            // Enhanced lifecycle fields
            close_date: model.close_date,
            last_activity_date: model.last_activity_date,
            dormancy_threshold_days: model.dormancy_threshold_days,
            reactivation_required: model.reactivation_required,
            pending_closure_reason_id: model.pending_closure_reason_id,
            disbursement_instructions: None, // This would need to be loaded separately from related table
            status_changed_by: model.status_changed_by,
            status_change_reason_id: model.status_change_reason_id,
            status_change_timestamp: model.status_change_timestamp,
            // Audit fields
            created_at: model.created_at,
            last_updated_at: model.last_updated_at,
            updated_by: model.updated_by,
        })
    }
}
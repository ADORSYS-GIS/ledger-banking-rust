use banking_api::domain::{Account, AccountType, AccountStatus, SigningCondition};
use banking_db::models::AccountModel;

pub struct AccountMapper;

impl AccountMapper {
    /// Map from domain Account to database AccountModel
    pub fn to_model(account: Account) -> AccountModel {
        AccountModel {
            account_id: account.account_id,
            product_code: account.product_code,
            account_type: Self::account_type_to_string(account.account_type),
            account_status: Self::account_status_to_string(account.account_status),
            signing_condition: Self::signing_condition_to_string(account.signing_condition),
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
            loan_purpose: account.loan_purpose,
            // Enhanced lifecycle fields
            close_date: account.close_date,
            last_activity_date: account.last_activity_date,
            dormancy_threshold_days: account.dormancy_threshold_days,
            reactivation_required: account.reactivation_required,
            pending_closure_reason: account.pending_closure_reason,
            status_changed_by: account.status_changed_by,
            status_change_reason: account.status_change_reason,
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
            account_type: Self::string_to_account_type(&model.account_type)?,
            account_status: Self::string_to_account_status(&model.account_status)?,
            signing_condition: Self::string_to_signing_condition(&model.signing_condition)?,
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
            loan_purpose: model.loan_purpose,
            // Enhanced lifecycle fields
            close_date: model.close_date,
            last_activity_date: model.last_activity_date,
            dormancy_threshold_days: model.dormancy_threshold_days,
            reactivation_required: model.reactivation_required,
            pending_closure_reason: model.pending_closure_reason,
            disbursement_instructions: None, // This would need to be loaded separately from related table
            status_changed_by: model.status_changed_by,
            status_change_reason: model.status_change_reason,
            status_change_timestamp: model.status_change_timestamp,
            // Audit fields
            created_at: model.created_at,
            last_updated_at: model.last_updated_at,
            updated_by: model.updated_by,
        })
    }

    // Helper methods for enum conversions
    fn account_type_to_string(account_type: AccountType) -> String {
        match account_type {
            AccountType::Savings => "Savings".to_string(),
            AccountType::Current => "Current".to_string(),
            AccountType::Loan => "Loan".to_string(),
        }
    }

    fn string_to_account_type(s: &str) -> banking_api::BankingResult<AccountType> {
        match s {
            "Savings" => Ok(AccountType::Savings),
            "Current" => Ok(AccountType::Current),
            "Loan" => Ok(AccountType::Loan),
            _ => Err(banking_api::BankingError::InvalidEnumValue {
                field: "account_type".to_string(),
                value: s.to_string(),
            }),
        }
    }

    pub fn account_status_to_string(status: AccountStatus) -> String {
        match status {
            AccountStatus::PendingApproval => "PendingApproval".to_string(),
            AccountStatus::Active => "Active".to_string(),
            AccountStatus::Dormant => "Dormant".to_string(),
            AccountStatus::Frozen => "Frozen".to_string(),
            AccountStatus::PendingClosure => "PendingClosure".to_string(),
            AccountStatus::Closed => "Closed".to_string(),
            AccountStatus::PendingReactivation => "PendingReactivation".to_string(),
        }
    }

    fn string_to_account_status(s: &str) -> banking_api::BankingResult<AccountStatus> {
        match s {
            "PendingApproval" => Ok(AccountStatus::PendingApproval),
            "Active" => Ok(AccountStatus::Active),
            "Dormant" => Ok(AccountStatus::Dormant),
            "Frozen" => Ok(AccountStatus::Frozen),
            "PendingClosure" => Ok(AccountStatus::PendingClosure),
            "Closed" => Ok(AccountStatus::Closed),
            "PendingReactivation" => Ok(AccountStatus::PendingReactivation),
            _ => Err(banking_api::BankingError::InvalidEnumValue {
                field: "account_status".to_string(),
                value: s.to_string(),
            }),
        }
    }

    fn signing_condition_to_string(signing_condition: SigningCondition) -> String {
        match signing_condition {
            SigningCondition::None => "None".to_string(),
            SigningCondition::AnyOwner => "AnyOwner".to_string(),
            SigningCondition::AllOwners => "AllOwners".to_string(),
        }
    }

    fn string_to_signing_condition(s: &str) -> banking_api::BankingResult<SigningCondition> {
        match s {
            "None" => Ok(SigningCondition::None),
            "AnyOwner" => Ok(SigningCondition::AnyOwner),
            "AllOwners" => Ok(SigningCondition::AllOwners),
            _ => Err(banking_api::BankingError::InvalidEnumValue {
                field: "signing_condition".to_string(),
                value: s.to_string(),
            }),
        }
    }
}
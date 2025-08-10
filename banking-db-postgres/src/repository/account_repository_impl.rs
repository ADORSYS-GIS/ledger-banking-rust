use async_trait::async_trait;
use banking_api::{BankingError, BankingResult};
use banking_api::domain::{
    AccountStatus, AccountType, DisbursementMethod, EntityType, HoldPriority, HoldStatus,
    HoldType, MandateStatus, OwnershipType, PermissionType, RelationshipStatus, RelationshipType,
    SigningCondition,
};
use banking_db::models::{
    AccountBalanceCalculationModel, AccountFinalSettlementModel, AccountHoldExpiryJobModel,
    AccountHoldModel, AccountHoldReleaseRequestModel, AccountHoldSummaryModel,
    AccountMandateModel, AccountModel, AccountOwnershipModel, AccountRelationshipModel,
    PlaceHoldRequestModel, StatusChangeModel,
};
use banking_db::repository::{
    AccountRepository, HighHoldRatioAccount, HoldAgingBucket, HoldAnalyticsSummary,
    HoldOverrideRecord, HoldPrioritySummary, HoldTypeSummary, HoldValidationError,
    JudicialHoldReportData,
};
use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub struct AccountRepositoryImpl {
    pool: PgPool,
}

impl AccountRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccountRepository for AccountRepositoryImpl {
    async fn create(&self, account: AccountModel) -> BankingResult<AccountModel> {
        let result = sqlx::query(
            r#"
            INSERT INTO accounts (
                id, product_id, account_type, account_status, signing_condition,
                currency, open_date, domicile_agency_branch_id, current_balance, available_balance,
                accrued_interest, overdraft_limit, original_principal, outstanding_principal,
                loan_interest_rate, loan_term_months, disbursement_date, maturity_date,
                installment_amount, next_due_date, penalty_rate, collateral_id, loan_purpose_id,
                close_date, last_activity_date, dormancy_threshold_days, reactivation_required,
                pending_closure_reason_id, last_disbursement_instruction_id, status_changed_by_person_id,
                status_change_reason_id, status_change_timestamp, most_significant_account_hold_id, account_ownership_id,
                access01_account_relationship_id, access02_account_relationship_id, access03_account_relationship_id,
                access04_account_relationship_id, access05_account_relationship_id, access06_account_relationship_id,
                access07_account_relationship_id, access11_account_mandate_id, access12_account_mandate_id,
                access13_account_mandate_id, access14_account_mandate_id, access15_account_mandate_id,
                access16_account_mandate_id, access17_account_mandate_id, interest01_ultimate_beneficiary_id,
                interest02_ultimate_beneficiary_id, interest03_ultimate_beneficiary_id, interest04_ultimate_beneficiary_id,
                interest05_ultimate_beneficiary_id, interest06_ultimate_beneficiary_id, interest07_ultimate_beneficiary_id,
                updated_by_person_id
            )
            VALUES (
                $1, $2, $3::account_type, $4::account_status, $5::signing_condition, $6, $7, $8, $9, $10, $11, $12, $13, $14,
                $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30,
                $31, $32, $33, $34, $35, $36, $37, $38, $39, $40, $41, $42, $43, $44, $45, $46,
                $47, $48, $49, $50, $51, $52, $53, $54, $55, $56
            )
            RETURNING id, product_id, account_type::text as account_type, 
                     account_status::text as account_status, signing_condition::text as signing_condition,
                     currency, open_date, domicile_agency_branch_id, current_balance, available_balance,
                     accrued_interest, overdraft_limit, original_principal, outstanding_principal,
                     loan_interest_rate, loan_term_months, disbursement_date, maturity_date,
                     installment_amount, next_due_date, penalty_rate, collateral_id, loan_purpose_id,
                     close_date, last_activity_date, dormancy_threshold_days, reactivation_required,
                     pending_closure_reason_id, last_disbursement_instruction_id, status_changed_by_person_id,
                     status_change_reason_id, status_change_timestamp, most_significant_account_hold_id, account_ownership_id,
                     access01_account_relationship_id, access02_account_relationship_id, access03_account_relationship_id,
                     access04_account_relationship_id, access05_account_relationship_id, access06_account_relationship_id,
                     access07_account_relationship_id, access11_account_mandate_id, access12_account_mandate_id,
                     access13_account_mandate_id, access14_account_mandate_id, access15_account_mandate_id,
                     access16_account_mandate_id, access17_account_mandate_id, interest01_ultimate_beneficiary_id,
                     interest02_ultimate_beneficiary_id, interest03_ultimate_beneficiary_id, interest04_ultimate_beneficiary_id,
                     interest05_ultimate_beneficiary_id, interest06_ultimate_beneficiary_id, interest07_ultimate_beneficiary_id,
                     created_at, last_updated_at, updated_by_person_id
            "#,
        )
        .bind(account.id)
        .bind(account.product_id)
        .bind(account.account_type.to_string())
        .bind(account.account_status.to_string())
        .bind(account.signing_condition.to_string())
        .bind(account.currency.as_str())
        .bind(account.open_date)
        .bind(account.domicile_agency_branch_id)
        .bind(account.current_balance)
        .bind(account.available_balance)
        .bind(account.accrued_interest)
        .bind(account.overdraft_limit)
        .bind(account.original_principal)
        .bind(account.outstanding_principal)
        .bind(account.loan_interest_rate)
        .bind(account.loan_term_months)
        .bind(account.disbursement_date)
        .bind(account.maturity_date)
        .bind(account.installment_amount)
        .bind(account.next_due_date)
        .bind(account.penalty_rate)
        .bind(account.collateral_id)
        .bind(account.loan_purpose_id)
        .bind(account.close_date)
        .bind(account.last_activity_date)
        .bind(account.dormancy_threshold_days)
        .bind(account.reactivation_required)
        .bind(account.pending_closure_reason_id)
        .bind(account.last_disbursement_instruction_id)
        .bind(account.status_changed_by_person_id)
        .bind(account.status_change_reason_id)
        .bind(account.status_change_timestamp)
        .bind(account.most_significant_account_hold_id)
        .bind(account.account_ownership_id)
        .bind(account.access01_account_relationship_id)
        .bind(account.access02_account_relationship_id)
        .bind(account.access03_account_relationship_id)
        .bind(account.access04_account_relationship_id)
        .bind(account.access05_account_relationship_id)
        .bind(account.access06_account_relationship_id)
        .bind(account.access07_account_relationship_id)
        .bind(account.access11_account_mandate_id)
        .bind(account.access12_account_mandate_id)
        .bind(account.access13_account_mandate_id)
        .bind(account.access14_account_mandate_id)
        .bind(account.access15_account_mandate_id)
        .bind(account.access16_account_mandate_id)
        .bind(account.access17_account_mandate_id)
        .bind(account.interest01_ultimate_beneficiary_id)
        .bind(account.interest02_ultimate_beneficiary_id)
        .bind(account.interest03_ultimate_beneficiary_id)
        .bind(account.interest04_ultimate_beneficiary_id)
        .bind(account.interest05_ultimate_beneficiary_id)
        .bind(account.interest06_ultimate_beneficiary_id)
        .bind(account.interest07_ultimate_beneficiary_id)
        .bind(account.updated_by_person_id)
        .fetch_one(&self.pool)
        .await?;

        // Convert result back to AccountModel
        Ok(AccountModel::try_from_row(&result)?)
    }

    async fn update(&self, account: AccountModel) -> BankingResult<AccountModel> {
        let result = sqlx::query(
            r#"
            UPDATE accounts SET
                product_id = $2, account_type = $3::account_type, account_status = $4::account_status,
                signing_condition = $5::signing_condition, currency = $6, open_date = $7,
                domicile_agency_branch_id = $8, current_balance = $9, available_balance = $10,
                accrued_interest = $11, overdraft_limit = $12, original_principal = $13,
                outstanding_principal = $14, loan_interest_rate = $15, loan_term_months = $16,
                disbursement_date = $17, maturity_date = $18, installment_amount = $19,
                next_due_date = $20, penalty_rate = $21, collateral_id = $22, loan_purpose_id = $23,
                close_date = $24, last_activity_date = $25, dormancy_threshold_days = $26,
                reactivation_required = $27, pending_closure_reason_id = $28,
                last_disbursement_instruction_id = $29, status_changed_by_person_id = $30,
                status_change_reason_id = $31, status_change_timestamp = $32, most_significant_account_hold_id = $33,
                account_ownership_id = $34, access01_account_relationship_id = $35, access02_account_relationship_id = $36,
                access03_account_relationship_id = $37, access04_account_relationship_id = $38, access05_account_relationship_id = $39,
                access06_account_relationship_id = $40, access07_account_relationship_id = $41, access11_account_mandate_id = $42,
                access12_account_mandate_id = $43, access13_account_mandate_id = $44, access14_account_mandate_id = $45,
                access15_account_mandate_id = $46, access16_account_mandate_id = $47, access17_account_mandate_id = $48,
                interest01_ultimate_beneficiary_id = $49, interest02_ultimate_beneficiary_id = $50, interest03_ultimate_beneficiary_id = $51,
                interest04_ultimate_beneficiary_id = $52, interest05_ultimate_beneficiary_id = $53, interest06_ultimate_beneficiary_id = $54,
                interest07_ultimate_beneficiary_id = $55, last_updated_at = NOW(), updated_by_person_id = $56
            WHERE id = $1
            RETURNING id, product_id, account_type::text as account_type,
                     account_status::text as account_status, signing_condition::text as signing_condition,
                     currency, open_date, domicile_agency_branch_id, current_balance, available_balance,
                     accrued_interest, overdraft_limit, original_principal, outstanding_principal,
                     loan_interest_rate, loan_term_months, disbursement_date, maturity_date,
                     installment_amount, next_due_date, penalty_rate, collateral_id, loan_purpose_id,
                     close_date, last_activity_date, dormancy_threshold_days, reactivation_required,
                     pending_closure_reason_id, last_disbursement_instruction_id, status_changed_by_person_id,
                     status_change_reason_id, status_change_timestamp, most_significant_account_hold_id, account_ownership_id,
                     access01_account_relationship_id, access02_account_relationship_id, access03_account_relationship_id,
                     access04_account_relationship_id, access05_account_relationship_id, access06_account_relationship_id,
                     access07_account_relationship_id, access11_account_mandate_id, access12_account_mandate_id,
                     access13_account_mandate_id, access14_account_mandate_id, access15_account_mandate_id,
                     access16_account_mandate_id, access17_account_mandate_id, interest01_ultimate_beneficiary_id,
                     interest02_ultimate_beneficiary_id, interest03_ultimate_beneficiary_id, interest04_ultimate_beneficiary_id,
                     interest05_ultimate_beneficiary_id, interest06_ultimate_beneficiary_id, interest07_ultimate_beneficiary_id,
                     created_at, last_updated_at, updated_by_person_id
            "#,
        )
        .bind(account.id)
        .bind(account.product_id)
        .bind(account.account_type.to_string())
        .bind(account.account_status.to_string())
        .bind(account.signing_condition.to_string())
        .bind(account.currency.as_str())
        .bind(account.open_date)
        .bind(account.domicile_agency_branch_id)
        .bind(account.current_balance)
        .bind(account.available_balance)
        .bind(account.accrued_interest)
        .bind(account.overdraft_limit)
        .bind(account.original_principal)
        .bind(account.outstanding_principal)
        .bind(account.loan_interest_rate)
        .bind(account.loan_term_months)
        .bind(account.disbursement_date)
        .bind(account.maturity_date)
        .bind(account.installment_amount)
        .bind(account.next_due_date)
        .bind(account.penalty_rate)
        .bind(account.collateral_id)
        .bind(account.loan_purpose_id)
        .bind(account.close_date)
        .bind(account.last_activity_date)
        .bind(account.dormancy_threshold_days)
        .bind(account.reactivation_required)
        .bind(account.pending_closure_reason_id)
        .bind(account.last_disbursement_instruction_id)
        .bind(account.status_changed_by_person_id)
        .bind(account.status_change_reason_id)
        .bind(account.status_change_timestamp)
        .bind(account.most_significant_account_hold_id)
        .bind(account.account_ownership_id)
        .bind(account.access01_account_relationship_id)
        .bind(account.access02_account_relationship_id)
        .bind(account.access03_account_relationship_id)
        .bind(account.access04_account_relationship_id)
        .bind(account.access05_account_relationship_id)
        .bind(account.access06_account_relationship_id)
        .bind(account.access07_account_relationship_id)
        .bind(account.access11_account_mandate_id)
        .bind(account.access12_account_mandate_id)
        .bind(account.access13_account_mandate_id)
        .bind(account.access14_account_mandate_id)
        .bind(account.access15_account_mandate_id)
        .bind(account.access16_account_mandate_id)
        .bind(account.access17_account_mandate_id)
        .bind(account.interest01_ultimate_beneficiary_id)
        .bind(account.interest02_ultimate_beneficiary_id)
        .bind(account.interest03_ultimate_beneficiary_id)
        .bind(account.interest04_ultimate_beneficiary_id)
        .bind(account.interest05_ultimate_beneficiary_id)
        .bind(account.interest06_ultimate_beneficiary_id)
        .bind(account.interest07_ultimate_beneficiary_id)
        .bind(account.updated_by_person_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(AccountModel::try_from_row(&result)?)
    }

    async fn find_by_id(&self, account_id: Uuid) -> BankingResult<Option<AccountModel>> {
        let result = sqlx::query(
            r#"
            SELECT id, product_id, account_type::text as account_type,
                   account_status::text as account_status, signing_condition::text as signing_condition,
                   currency, open_date, domicile_agency_branch_id, current_balance, available_balance,
                   accrued_interest, overdraft_limit, original_principal, outstanding_principal,
                   loan_interest_rate, loan_term_months, disbursement_date, maturity_date,
                   installment_amount, next_due_date, penalty_rate, collateral_id, loan_purpose_id,
                   close_date, last_activity_date, dormancy_threshold_days, reactivation_required,
                   pending_closure_reason_id, last_disbursement_instruction_id, status_changed_by_person_id,
                   status_change_reason_id, status_change_timestamp,
                   most_significant_account_hold_id, account_ownership_id,
                   access01_account_relationship_id, access02_account_relationship_id, access03_account_relationship_id,
                   access04_account_relationship_id, access05_account_relationship_id, access06_account_relationship_id,
                   access07_account_relationship_id, access11_account_mandate_id, access12_account_mandate_id,
                   access13_account_mandate_id, access14_account_mandate_id, access15_account_mandate_id,
                   access16_account_mandate_id, access17_account_mandate_id, interest01_ultimate_beneficiary_id,
                   interest02_ultimate_beneficiary_id, interest03_ultimate_beneficiary_id, interest04_ultimate_beneficiary_id,
                   interest05_ultimate_beneficiary_id, interest06_ultimate_beneficiary_id, interest07_ultimate_beneficiary_id,
                   created_at, last_updated_at, updated_by_person_id
            FROM accounts WHERE id = $1
            "#,
        )
        .bind(account_id)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => Ok(Some(AccountModel::try_from_row(&row)?)),
            None => Ok(None),
        }
    }

    async fn find_by_customer_id(&self, customer_id: Uuid) -> BankingResult<Vec<AccountModel>> {
        let rows = sqlx::query(
            r#"
            SELECT a.id, a.product_id, a.account_type::text as account_type,
                   a.account_status::text as account_status, a.signing_condition::text as signing_condition,
                   a.currency, a.open_date, a.domicile_agency_branch_id, a.current_balance, a.available_balance,
                   a.accrued_interest, a.overdraft_limit, a.original_principal, a.outstanding_principal,
                   a.loan_interest_rate, a.loan_term_months, a.disbursement_date, a.maturity_date,
                   a.installment_amount, a.next_due_date, a.penalty_rate, a.collateral_id, a.loan_purpose_id,
                   a.close_date, a.last_activity_date, a.dormancy_threshold_days, a.reactivation_required,
                   a.pending_closure_reason_id, a.last_disbursement_instruction_id, a.status_changed_by_person_id,
                   a.status_change_reason_id, a.status_change_timestamp,
                   a.most_significant_account_hold_id, a.account_ownership_id,
                   a.access01_account_relationship_id, a.access02_account_relationship_id, a.access03_account_relationship_id,
                   a.access04_account_relationship_id, a.access05_account_relationship_id, a.access06_account_relationship_id,
                   a.access07_account_relationship_id, a.access11_account_mandate_id, a.access12_account_mandate_id,
                   a.access13_account_mandate_id, a.access14_account_mandate_id, a.access15_account_mandate_id,
                   a.access16_account_mandate_id, a.access17_account_mandate_id, a.interest01_ultimate_beneficiary_id,
                   a.interest02_ultimate_beneficiary_id, a.interest03_ultimate_beneficiary_id, a.interest04_ultimate_beneficiary_id,
                   a.interest05_ultimate_beneficiary_id, a.interest06_ultimate_beneficiary_id, a.interest07_ultimate_beneficiary_id,
                   a.created_at, a.last_updated_at, a.updated_by_person_id
            FROM accounts a
            INNER JOIN account_ownership ao ON a.id = ao.account_id
            WHERE ao.customer_id = $1
            ORDER BY a.created_at DESC
            "#,
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await?;

        let mut accounts = Vec::new();
        for row in rows {
            accounts.push(AccountModel::try_from_row(&row)?);
        }
        Ok(accounts)
    }

    async fn find_by_product_code(&self, product_code: &str) -> BankingResult<Vec<AccountModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, product_id, account_type::text as account_type,
                   account_status::text as account_status, signing_condition::text as signing_condition,
                   currency, open_date, domicile_agency_branch_id, current_balance, available_balance,
                   accrued_interest, overdraft_limit, original_principal, outstanding_principal,
                   loan_interest_rate, loan_term_months, disbursement_date, maturity_date,
                   installment_amount, next_due_date, penalty_rate, collateral_id, loan_purpose_id,
                   close_date, last_activity_date, dormancy_threshold_days, reactivation_required,
                   pending_closure_reason_id, last_disbursement_instruction_id, status_changed_by_person_id,
                   status_change_reason_id, status_change_timestamp,
                   most_significant_account_hold_id, account_ownership_id,
                   access01_account_relationship_id, access02_account_relationship_id, access03_account_relationship_id,
                   access04_account_relationship_id, access05_account_relationship_id, access06_account_relationship_id,
                   access07_account_relationship_id, access11_account_mandate_id, access12_account_mandate_id,
                   access13_account_mandate_id, access14_account_mandate_id, access15_account_mandate_id,
                   access16_account_mandate_id, access17_account_mandate_id, interest01_ultimate_beneficiary_id,
                   interest02_ultimate_beneficiary_id, interest03_ultimate_beneficiary_id, interest04_ultimate_beneficiary_id,
                   interest05_ultimate_beneficiary_id, interest06_ultimate_beneficiary_id, interest07_ultimate_beneficiary_id,
                   created_at, last_updated_at, updated_by_person_id
            FROM accounts WHERE product_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(product_code)
        .fetch_all(&self.pool)
        .await?;

        let mut accounts = Vec::new();
        for row in rows {
            accounts.push(AccountModel::try_from_row(&row)?);
        }
        Ok(accounts)
    }

    async fn find_by_status(&self, status: &str) -> BankingResult<Vec<AccountModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, product_id, account_type::text as account_type,
                   account_status::text as account_status, signing_condition::text as signing_condition,
                   currency, open_date, domicile_agency_branch_id, current_balance, available_balance,
                   accrued_interest, overdraft_limit, original_principal, outstanding_principal,
                   loan_interest_rate, loan_term_months, disbursement_date, maturity_date,
                   installment_amount, next_due_date, penalty_rate, collateral_id, loan_purpose_id,
                   close_date, last_activity_date, dormancy_threshold_days, reactivation_required,
                   pending_closure_reason_id, last_disbursement_instruction_id, status_changed_by_person_id,
                   status_change_reason_id, status_change_timestamp,
                   most_significant_account_hold_id, account_ownership_id,
                   access01_account_relationship_id, access02_account_relationship_id, access03_account_relationship_id,
                   access04_account_relationship_id, access05_account_relationship_id, access06_account_relationship_id,
                   access07_account_relationship_id, access11_account_mandate_id, access12_account_mandate_id,
                   access13_account_mandate_id, access14_account_mandate_id, access15_account_mandate_id,
                   access16_account_mandate_id, access17_account_mandate_id, interest01_ultimate_beneficiary_id,
                   interest02_ultimate_beneficiary_id, interest03_ultimate_beneficiary_id, interest04_ultimate_beneficiary_id,
                   interest05_ultimate_beneficiary_id, interest06_ultimate_beneficiary_id, interest07_ultimate_beneficiary_id,
                   created_at, last_updated_at, updated_by_person_id
            FROM accounts WHERE account_status::text = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(status)
        .fetch_all(&self.pool)
        .await?;

        let mut accounts = Vec::new();
        for row in rows {
            accounts.push(AccountModel::try_from_row(&row)?);
        }
        Ok(accounts)
    }

    async fn find_dormancy_candidates(&self, reference_date: NaiveDate, threshold_days: i32) -> BankingResult<Vec<AccountModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, product_id, account_type::text as account_type,
                   account_status::text as account_status, signing_condition::text as signing_condition,
                   currency, open_date, domicile_agency_branch_id, current_balance, available_balance,
                   accrued_interest, overdraft_limit, original_principal, outstanding_principal,
                   loan_interest_rate, loan_term_months, disbursement_date, maturity_date,
                   installment_amount, next_due_date, penalty_rate, collateral_id, loan_purpose_id,
                   close_date, last_activity_date, dormancy_threshold_days, reactivation_required,
                   pending_closure_reason_id, last_disbursement_instruction_id, status_changed_by_person_id,
                   status_change_reason_id, status_change_timestamp,
                   most_significant_account_hold_id, account_ownership_id,
                   access01_account_relationship_id, access02_account_relationship_id, access03_account_relationship_id,
                   access04_account_relationship_id, access05_account_relationship_id, access06_account_relationship_id,
                   access07_account_relationship_id, access11_account_mandate_id, access12_account_mandate_id,
                   access13_account_mandate_id, access14_account_mandate_id, access15_account_mandate_id,
                   access16_account_mandate_id, access17_account_mandate_id, interest01_ultimate_beneficiary_id,
                   interest02_ultimate_beneficiary_id, interest03_ultimate_beneficiary_id, interest04_ultimate_beneficiary_id,
                   interest05_ultimate_beneficiary_id, interest06_ultimate_beneficiary_id, interest07_ultimate_beneficiary_id,
                   created_at, last_updated_at, updated_by_person_id
            FROM accounts 
            WHERE account_status = 'Active'
              AND last_activity_date IS NOT NULL
              AND $1 - last_activity_date >= $2
            ORDER BY last_activity_date ASC
            "#,
        )
        .bind(reference_date)
        .bind(threshold_days)
        .fetch_all(&self.pool)
        .await?;

        let mut accounts = Vec::new();
        for row in rows {
            accounts.push(AccountModel::try_from_row(&row)?);
        }
        Ok(accounts)
    }

    async fn find_pending_closure(&self) -> BankingResult<Vec<AccountModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, product_id, account_type::text as account_type,
                   account_status::text as account_status, signing_condition::text as signing_condition,
                   currency, open_date, domicile_agency_branch_id, current_balance, available_balance,
                   accrued_interest, overdraft_limit, original_principal, outstanding_principal,
                   loan_interest_rate, loan_term_months, disbursement_date, maturity_date,
                   installment_amount, next_due_date, penalty_rate, collateral_id, loan_purpose_id,
                   close_date, last_activity_date, dormancy_threshold_days, reactivation_required,
                   pending_closure_reason_id, last_disbursement_instruction_id, status_changed_by_person_id,
                   status_change_reason_id, status_change_timestamp,
                   most_significant_account_hold_id, account_ownership_id,
                   access01_account_relationship_id, access02_account_relationship_id, access03_account_relationship_id,
                   access04_account_relationship_id, access05_account_relationship_id, access06_account_relationship_id,
                   access07_account_relationship_id, access11_account_mandate_id, access12_account_mandate_id,
                   access13_account_mandate_id, access14_account_mandate_id, access15_account_mandate_id,
                   access16_account_mandate_id, access17_account_mandate_id, interest01_ultimate_beneficiary_id,
                   interest02_ultimate_beneficiary_id, interest03_ultimate_beneficiary_id, interest04_ultimate_beneficiary_id,
                   interest05_ultimate_beneficiary_id, interest06_ultimate_beneficiary_id, interest07_ultimate_beneficiary_id,
                   created_at, last_updated_at, updated_by_person_id
            FROM accounts 
            WHERE account_status = 'PendingClosure'
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut accounts = Vec::new();
        for row in rows {
            accounts.push(AccountModel::try_from_row(&row)?);
        }
        Ok(accounts)
    }

    async fn find_interest_bearing_accounts(&self) -> BankingResult<Vec<AccountModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, product_id, account_type::text as account_type,
                   account_status::text as account_status, signing_condition::text as signing_condition,
                   currency, open_date, domicile_agency_branch_id, current_balance, available_balance,
                   accrued_interest, overdraft_limit, original_principal, outstanding_principal,
                   loan_interest_rate, loan_term_months, disbursement_date, maturity_date,
                   installment_amount, next_due_date, penalty_rate, collateral_id, loan_purpose_id,
                   close_date, last_activity_date, dormancy_threshold_days, reactivation_required,
                   pending_closure_reason_id, last_disbursement_instruction_id, status_changed_by_person_id,
                   status_change_reason_id, status_change_timestamp,
                   most_significant_account_hold_id, account_ownership_id,
                   access01_account_relationship_id, access02_account_relationship_id, access03_account_relationship_id,
                   access04_account_relationship_id, access05_account_relationship_id, access06_account_relationship_id,
                   access07_account_relationship_id, access11_account_mandate_id, access12_account_mandate_id,
                   access13_account_mandate_id, access14_account_mandate_id, access15_account_mandate_id,
                   access16_account_mandate_id, access17_account_mandate_id, interest01_ultimate_beneficiary_id,
                   interest02_ultimate_beneficiary_id, interest03_ultimate_beneficiary_id, interest04_ultimate_beneficiary_id,
                   interest05_ultimate_beneficiary_id, interest06_ultimate_beneficiary_id, interest07_ultimate_beneficiary_id,
                   created_at, last_updated_at, updated_by_person_id
            FROM accounts 
            WHERE account_type = 'Savings' 
               OR (account_type = 'Loan' AND loan_interest_rate > 0)
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut accounts = Vec::new();
        for row in rows {
            accounts.push(AccountModel::try_from_row(&row)?);
        }
        Ok(accounts)
    }

    async fn update_status(&self, account_id: Uuid, status: &str, reason: &str, changed_by_person_id: Uuid) -> BankingResult<()> {
        sqlx::query(
            r#"
            UPDATE accounts 
            SET account_status = $2::account_status,
                status_changed_by_person_id = $3,
                status_change_timestamp = NOW()
            WHERE id = $1
            "#,
        )
        .bind(account_id)
        .bind(status)
        .bind(changed_by_person_id)
        .execute(&self.pool)
        .await?;

        // Add status change to history
        sqlx::query(
            r#"
            INSERT INTO account_status_history (
                id, account_id, old_status, new_status, reason_id,
                additional_context, changed_by_person_id, changed_at, system_triggered
            )
            VALUES (gen_random_uuid(), $1, 
                    (SELECT account_status FROM accounts WHERE id = $1),
                    $2::account_status, NULL, $3, $4, NOW(), false)
            "#,
        )
        .bind(account_id)
        .bind(status)
        .bind(reason)
        .bind(changed_by_person_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_balance(&self, account_id: Uuid, current_balance: Decimal, available_balance: Decimal) -> BankingResult<()> {
        sqlx::query(
            r#"
            UPDATE accounts 
            SET current_balance = $2,
                available_balance = $3,
                last_updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(account_id)
        .bind(current_balance)
        .bind(available_balance)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_accrued_interest(&self, account_id: Uuid, accrued_interest: Decimal) -> BankingResult<()> {
        sqlx::query(
            r#"
            UPDATE accounts 
            SET accrued_interest = $2,
                last_updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(account_id)
        .bind(accrued_interest)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn reset_accrued_interest(&self, account_id: Uuid) -> BankingResult<()> {
        sqlx::query(
            r#"
            UPDATE accounts 
            SET accrued_interest = 0.00,
                last_updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(account_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_last_activity_date(&self, account_id: Uuid, activity_date: NaiveDate) -> BankingResult<()> {
        sqlx::query(
            r#"
            UPDATE accounts 
            SET last_activity_date = $2,
                last_updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(account_id)
        .bind(activity_date)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Account Ownership Operations
    async fn create_ownership(&self, ownership: AccountOwnershipModel) -> BankingResult<AccountOwnershipModel> {
        let result = sqlx::query(
            r#"
            INSERT INTO account_ownership (
                id, account_id, customer_id, ownership_type, ownership_percentage
            )
            VALUES ($1, $2, $3, $4::ownership_type, $5)
            RETURNING id, account_id, customer_id, ownership_type::text as ownership_type,
                     ownership_percentage, created_at
            "#,
        )
        .bind(ownership.id)
        .bind(ownership.account_id)
        .bind(ownership.customer_id)
        .bind(ownership.ownership_type.to_string())
        .bind(ownership.ownership_percentage)
        .fetch_one(&self.pool)
        .await?;

        Ok(AccountOwnershipModel::try_from_row(&result)?)
    }

    async fn find_ownership_by_account(&self, account_id: Uuid) -> BankingResult<Vec<AccountOwnershipModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, customer_id, ownership_type::text as ownership_type,
                   ownership_percentage, created_at
            FROM account_ownership 
            WHERE account_id = $1
            ORDER BY created_at
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await?;

        let mut ownerships = Vec::new();
        for row in rows {
            ownerships.push(AccountOwnershipModel::try_from_row(&row)?);
        }
        Ok(ownerships)
    }

    async fn find_accounts_by_owner(&self, customer_id: Uuid) -> BankingResult<Vec<AccountOwnershipModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, customer_id, ownership_type::text as ownership_type,
                   ownership_percentage, created_at
            FROM account_ownership 
            WHERE customer_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await?;

        let mut ownerships = Vec::new();
        for row in rows {
            ownerships.push(AccountOwnershipModel::try_from_row(&row)?);
        }
        Ok(ownerships)
    }

    async fn delete_ownership(&self, ownership_id: Uuid) -> BankingResult<()> {
        sqlx::query(
            "DELETE FROM account_ownership WHERE id = $1",
        )
        .bind(ownership_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Account Relationship Operations  
    async fn create_relationship(&self, relationship: AccountRelationshipModel) -> BankingResult<AccountRelationshipModel> {
        let result = sqlx::query(
            r#"
            INSERT INTO account_relationships (
                id, account_id, person_id, entity_type, relationship_type,
                status, start_date, end_date
            )
            VALUES ($1, $2, $3, $4::entity_type, $5::relationship_type, $6::relationship_status, $7, $8)
            RETURNING id, account_id, person_id, entity_type::text as entity_type,
                     relationship_type::text as relationship_type, status::text as status,
                     start_date, end_date
            "#,
        )
        .bind(relationship.id)
        .bind(relationship.account_id)
        .bind(relationship.person_id)
        .bind(relationship.entity_type.to_string())
        .bind(relationship.relationship_type.to_string())
        .bind(relationship.status.to_string())
        .bind(relationship.start_date)
        .bind(relationship.end_date)
        .fetch_one(&self.pool)
        .await?;

        Ok(AccountRelationshipModel::try_from_row(&result)?)
    }

    async fn find_relationships_by_account(&self, account_id: Uuid) -> BankingResult<Vec<AccountRelationshipModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, person_id, entity_type::text as entity_type,
                   relationship_type::text as relationship_type, status::text as status,
                   start_date, end_date
            FROM account_relationships 
            WHERE account_id = $1
            ORDER BY start_date DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await?;

        let mut relationships = Vec::new();
        for row in rows {
            relationships.push(AccountRelationshipModel::try_from_row(&row)?);
        }
        Ok(relationships)
    }

    async fn find_relationships_by_entity(&self, person_id: Uuid, entity_type: &str) -> BankingResult<Vec<AccountRelationshipModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, person_id, entity_type::text as entity_type,
                   relationship_type::text as relationship_type, status::text as status,
                   start_date, end_date
            FROM account_relationships 
            WHERE person_id = $1 AND entity_type::text = $2
            ORDER BY start_date DESC
            "#,
        )
        .bind(person_id)
        .bind(entity_type)
        .fetch_all(&self.pool)
        .await?;

        let mut relationships = Vec::new();
        for row in rows {
            relationships.push(AccountRelationshipModel::try_from_row(&row)?);
        }
        Ok(relationships)
    }

    async fn update_relationship(&self, relationship: AccountRelationshipModel) -> BankingResult<AccountRelationshipModel> {
        let result = sqlx::query(
            r#"
            UPDATE account_relationships 
            SET person_id = $3, entity_type = $4::entity_type, relationship_type = $5::relationship_type,
                status = $6::relationship_status, start_date = $7, end_date = $8
            WHERE id = $1
            RETURNING id, account_id, person_id, entity_type::text as entity_type,
                     relationship_type::text as relationship_type, status::text as status,
                     start_date, end_date
            "#,
        )
        .bind(relationship.id)
        .bind(relationship.account_id)
        .bind(relationship.person_id)
        .bind(relationship.entity_type.to_string())
        .bind(relationship.relationship_type.to_string())
        .bind(relationship.status.to_string())
        .bind(relationship.start_date)
        .bind(relationship.end_date)
        .fetch_one(&self.pool)
        .await?;

        Ok(AccountRelationshipModel::try_from_row(&result)?)
    }

    async fn delete_relationship(&self, relationship_id: Uuid) -> BankingResult<()> {
        sqlx::query(
            "DELETE FROM account_relationships WHERE id = $1",
        )
        .bind(relationship_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Account Mandate Operations
    async fn create_mandate(&self, mandate: AccountMandateModel) -> BankingResult<AccountMandateModel> {
        let result = sqlx::query(
            r#"
            INSERT INTO account_mandates (
                id, account_id, grantee_customer_id, permission_type,
                transaction_limit, approver01_person_id, approver02_person_id, approver03_person_id,
                approver04_person_id, approver05_person_id, approver06_person_id, approver07_person_id,
                required_signers_count, conditional_mandate_id, status, start_date, end_date
            )
            VALUES ($1, $2, $3, $4::permission_type, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15::mandate_status, $16, $17)
            RETURNING id, account_id, grantee_customer_id, permission_type::text as permission_type,
                     transaction_limit, approver01_person_id, approver02_person_id, approver03_person_id,
                     approver04_person_id, approver05_person_id, approver06_person_id, approver07_person_id,
                     required_signers_count, conditional_mandate_id, status::text as status, start_date, end_date
            "#,
        )
        .bind(mandate.id)
        .bind(mandate.account_id)
        .bind(mandate.grantee_customer_id)
        .bind(mandate.permission_type.to_string())
        .bind(mandate.transaction_limit)
        .bind(mandate.approver01_person_id)
        .bind(mandate.approver02_person_id)
        .bind(mandate.approver03_person_id)
        .bind(mandate.approver04_person_id)
        .bind(mandate.approver05_person_id)
        .bind(mandate.approver06_person_id)
        .bind(mandate.approver07_person_id)
        .bind(mandate.required_signers_count as i16)
        .bind(mandate.conditional_mandate_id)
        .bind(mandate.status.to_string())
        .bind(mandate.start_date)
        .bind(mandate.end_date)
        .fetch_one(&self.pool)
        .await?;

        Ok(AccountMandateModel::try_from_row(&result)?)
    }

    async fn find_mandates_by_account(&self, account_id: Uuid) -> BankingResult<Vec<AccountMandateModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, grantee_customer_id, permission_type::text as permission_type,
                   transaction_limit, approver01_person_id, approver02_person_id, approver03_person_id,
                   approver04_person_id, approver05_person_id, approver06_person_id, approver07_person_id,
                   required_signers_count, conditional_mandate_id, status::text as status, start_date, end_date
            FROM account_mandates 
            WHERE account_id = $1
            ORDER BY start_date DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await?;

        let mut mandates = Vec::new();
        for row in rows {
            mandates.push(AccountMandateModel::try_from_row(&row)?);
        }
        Ok(mandates)
    }

    async fn find_mandates_by_grantee(&self, grantee_customer_id: Uuid) -> BankingResult<Vec<AccountMandateModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, grantee_customer_id, permission_type::text as permission_type,
                   transaction_limit, approver01_person_id, approver02_person_id, approver03_person_id,
                   approver04_person_id, approver05_person_id, approver06_person_id, approver07_person_id,
                   required_signers_count, conditional_mandate_id, status::text as status, start_date, end_date
            FROM account_mandates 
            WHERE grantee_customer_id = $1
            ORDER BY start_date DESC
            "#,
        )
        .bind(grantee_customer_id)
        .fetch_all(&self.pool)
        .await?;

        let mut mandates = Vec::new();
        for row in rows {
            mandates.push(AccountMandateModel::try_from_row(&row)?);
        }
        Ok(mandates)
    }

    async fn update_mandate_status(&self, mandate_id: Uuid, status: &str) -> BankingResult<()> {
        sqlx::query(
            r#"
            UPDATE account_mandates 
            SET status = $2::mandate_status
            WHERE id = $1
            "#,
        )
        .bind(mandate_id)
        .bind(status)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_active_mandates(&self, account_id: Uuid) -> BankingResult<Vec<AccountMandateModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, account_id, grantee_customer_id, permission_type::text as permission_type,
                   transaction_limit, approver01_person_id, approver02_person_id, approver03_person_id,
                   approver04_person_id, approver05_person_id, approver06_person_id, approver07_person_id,
                   required_signers_count, conditional_mandate_id, status::text as status, start_date, end_date
            FROM account_mandates 
            WHERE account_id = $1 AND status = 'Active'
              AND start_date <= CURRENT_DATE 
              AND (end_date IS NULL OR end_date >= CURRENT_DATE)
            ORDER BY start_date DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await?;

        let mut mandates = Vec::new();
        for row in rows {
            mandates.push(AccountMandateModel::try_from_row(&row)?);
        }
        Ok(mandates)
    }
}

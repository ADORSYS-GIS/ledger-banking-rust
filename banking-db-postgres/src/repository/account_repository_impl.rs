use async_trait::async_trait;
use banking_api::{BankingResult, BankingError};
use banking_api::domain::{
    AccountType, AccountStatus, SigningCondition, DisbursementMethod, HoldType, HoldStatus, 
    HoldPriority, OwnershipType, EntityType, RelationshipType, RelationshipStatus, 
    PermissionType, MandateStatus
};
use banking_db::models::{AccountModel, AccountOwnershipModel, AccountRelationshipModel, AccountMandateModel, AccountHoldModel, StatusChangeModel, AccountFinalSettlementModel};
use banking_db::repository::AccountRepository;
use sqlx::PgPool;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, NaiveDate};
use heapless::String as HeaplessString;

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
                account_id, product_code, account_type, account_status, signing_condition,
                currency, open_date, domicile_branch_id, current_balance, available_balance,
                accrued_interest, overdraft_limit, original_principal, outstanding_principal,
                loan_interest_rate, loan_term_months, disbursement_date, maturity_date,
                installment_amount, next_due_date, penalty_rate, collateral_id, loan_purpose_id,
                close_date, last_activity_date, dormancy_threshold_days, reactivation_required,
                pending_closure_reason_id, last_disbursement_instruction_id, status_changed_by,
                status_change_reason_id, status_change_timestamp, updated_by
            )
            VALUES (
                $1, $2, $3::account_type, $4::account_status, $5::signing_condition, $6, $7, $8, $9, $10, $11, $12, $13, $14,
                $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30,
                $31, $32, $33
            )
            RETURNING account_id, product_code, account_type::text as account_type, 
                     account_status::text as account_status, signing_condition::text as signing_condition,
                     currency, open_date, domicile_branch_id, current_balance, available_balance,
                     accrued_interest, overdraft_limit, original_principal, outstanding_principal,
                     loan_interest_rate, loan_term_months, disbursement_date, maturity_date,
                     installment_amount, next_due_date, penalty_rate, collateral_id, loan_purpose_id,
                     close_date, last_activity_date, dormancy_threshold_days, reactivation_required,
                     pending_closure_reason_id, last_disbursement_instruction_id, status_changed_by,
                     status_change_reason_id, status_change_timestamp, created_at, last_updated_at, updated_by
            "#,
        )
        .bind(account.account_id)
        .bind(account.product_code.as_str())
        .bind(account.account_type.to_string())
        .bind(account.account_status.to_string())
        .bind(account.signing_condition.to_string())
        .bind(account.currency.as_str())
        .bind(account.open_date)
        .bind(account.domicile_branch_id)
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
        .bind(account.status_changed_by)
        .bind(account.status_change_reason_id)
        .bind(account.status_change_timestamp)
        .bind(account.updated_by)
        .fetch_one(&self.pool)
        .await?;

        // Convert result back to AccountModel
        Ok(AccountModel::try_from_row(&result)?)
    }

    async fn update(&self, account: AccountModel) -> BankingResult<AccountModel> {
        let result = sqlx::query(
            r#"
            UPDATE accounts SET
                product_code = $2, account_type = $3::account_type, account_status = $4::account_status,
                signing_condition = $5::signing_condition, currency = $6, open_date = $7,
                domicile_branch_id = $8, current_balance = $9, available_balance = $10,
                accrued_interest = $11, overdraft_limit = $12, original_principal = $13,
                outstanding_principal = $14, loan_interest_rate = $15, loan_term_months = $16,
                disbursement_date = $17, maturity_date = $18, installment_amount = $19,
                next_due_date = $20, penalty_rate = $21, collateral_id = $22, loan_purpose_id = $23,
                close_date = $24, last_activity_date = $25, dormancy_threshold_days = $26,
                reactivation_required = $27, pending_closure_reason_id = $28,
                last_disbursement_instruction_id = $29, status_changed_by = $30,
                status_change_reason_id = $31, status_change_timestamp = $32, updated_by = $33
            WHERE account_id = $1
            RETURNING account_id, product_code, account_type::text as account_type,
                     account_status::text as account_status, signing_condition::text as signing_condition,
                     currency, open_date, domicile_branch_id, current_balance, available_balance,
                     accrued_interest, overdraft_limit, original_principal, outstanding_principal,
                     loan_interest_rate, loan_term_months, disbursement_date, maturity_date,
                     installment_amount, next_due_date, penalty_rate, collateral_id, loan_purpose_id,
                     close_date, last_activity_date, dormancy_threshold_days, reactivation_required,
                     pending_closure_reason_id, last_disbursement_instruction_id, status_changed_by,
                     status_change_reason_id, status_change_timestamp, created_at, last_updated_at, updated_by
            "#,
        )
        .bind(account.account_id)
        .bind(account.product_code.as_str())
        .bind(account.account_type.to_string())
        .bind(account.account_status.to_string())
        .bind(account.signing_condition.to_string())
        .bind(account.currency.as_str())
        .bind(account.open_date)
        .bind(account.domicile_branch_id)
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
        .bind(account.status_changed_by)
        .bind(account.status_change_reason_id)
        .bind(account.status_change_timestamp)
        .bind(account.updated_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(AccountModel::try_from_row(&result)?)
    }

    async fn find_by_id(&self, account_id: Uuid) -> BankingResult<Option<AccountModel>> {
        let result = sqlx::query(
            r#"
            SELECT account_id, product_code, account_type::text as account_type,
                   account_status::text as account_status, signing_condition::text as signing_condition,
                   currency, open_date, domicile_branch_id, current_balance, available_balance,
                   accrued_interest, overdraft_limit, original_principal, outstanding_principal,
                   loan_interest_rate, loan_term_months, disbursement_date, maturity_date,
                   installment_amount, next_due_date, penalty_rate, collateral_id, loan_purpose_id,
                   close_date, last_activity_date, dormancy_threshold_days, reactivation_required,
                   pending_closure_reason_id, last_disbursement_instruction_id, status_changed_by,
                   status_change_reason_id, status_change_timestamp, created_at, last_updated_at, updated_by
            FROM accounts WHERE account_id = $1
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
            SELECT a.account_id, a.product_code, a.account_type::text as account_type,
                   a.account_status::text as account_status, a.signing_condition::text as signing_condition,
                   a.currency, a.open_date, a.domicile_branch_id, a.current_balance, a.available_balance,
                   a.accrued_interest, a.overdraft_limit, a.original_principal, a.outstanding_principal,
                   a.loan_interest_rate, a.loan_term_months, a.disbursement_date, a.maturity_date,
                   a.installment_amount, a.next_due_date, a.penalty_rate, a.collateral_id, a.loan_purpose_id,
                   a.close_date, a.last_activity_date, a.dormancy_threshold_days, a.reactivation_required,
                   a.pending_closure_reason_id, a.last_disbursement_instruction_id, a.status_changed_by,
                   a.status_change_reason_id, a.status_change_timestamp, a.created_at, a.last_updated_at, a.updated_by
            FROM accounts a
            INNER JOIN account_ownership ao ON a.account_id = ao.account_id
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
            SELECT account_id, product_code, account_type::text as account_type,
                   account_status::text as account_status, signing_condition::text as signing_condition,
                   currency, open_date, domicile_branch_id, current_balance, available_balance,
                   accrued_interest, overdraft_limit, original_principal, outstanding_principal,
                   loan_interest_rate, loan_term_months, disbursement_date, maturity_date,
                   installment_amount, next_due_date, penalty_rate, collateral_id, loan_purpose_id,
                   close_date, last_activity_date, dormancy_threshold_days, reactivation_required,
                   pending_closure_reason_id, last_disbursement_instruction_id, status_changed_by,
                   status_change_reason_id, status_change_timestamp, created_at, last_updated_at, updated_by
            FROM accounts WHERE product_code = $1
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
            SELECT account_id, product_code, account_type::text as account_type,
                   account_status::text as account_status, signing_condition::text as signing_condition,
                   currency, open_date, domicile_branch_id, current_balance, available_balance,
                   accrued_interest, overdraft_limit, original_principal, outstanding_principal,
                   loan_interest_rate, loan_term_months, disbursement_date, maturity_date,
                   installment_amount, next_due_date, penalty_rate, collateral_id, loan_purpose_id,
                   close_date, last_activity_date, dormancy_threshold_days, reactivation_required,
                   pending_closure_reason_id, last_disbursement_instruction_id, status_changed_by,
                   status_change_reason_id, status_change_timestamp, created_at, last_updated_at, updated_by
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
            SELECT account_id, product_code, account_type::text as account_type,
                   account_status::text as account_status, signing_condition::text as signing_condition,
                   currency, open_date, domicile_branch_id, current_balance, available_balance,
                   accrued_interest, overdraft_limit, original_principal, outstanding_principal,
                   loan_interest_rate, loan_term_months, disbursement_date, maturity_date,
                   installment_amount, next_due_date, penalty_rate, collateral_id, loan_purpose_id,
                   close_date, last_activity_date, dormancy_threshold_days, reactivation_required,
                   pending_closure_reason_id, last_disbursement_instruction_id, status_changed_by,
                   status_change_reason_id, status_change_timestamp, created_at, last_updated_at, updated_by
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
            SELECT account_id, product_code, account_type::text as account_type,
                   account_status::text as account_status, signing_condition::text as signing_condition,
                   currency, open_date, domicile_branch_id, current_balance, available_balance,
                   accrued_interest, overdraft_limit, original_principal, outstanding_principal,
                   loan_interest_rate, loan_term_months, disbursement_date, maturity_date,
                   installment_amount, next_due_date, penalty_rate, collateral_id, loan_purpose_id,
                   close_date, last_activity_date, dormancy_threshold_days, reactivation_required,
                   pending_closure_reason_id, last_disbursement_instruction_id, status_changed_by,
                   status_change_reason_id, status_change_timestamp, created_at, last_updated_at, updated_by
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
            SELECT account_id, product_code, account_type::text as account_type,
                   account_status::text as account_status, signing_condition::text as signing_condition,
                   currency, open_date, domicile_branch_id, current_balance, available_balance,
                   accrued_interest, overdraft_limit, original_principal, outstanding_principal,
                   loan_interest_rate, loan_term_months, disbursement_date, maturity_date,
                   installment_amount, next_due_date, penalty_rate, collateral_id, loan_purpose_id,
                   close_date, last_activity_date, dormancy_threshold_days, reactivation_required,
                   pending_closure_reason_id, last_disbursement_instruction_id, status_changed_by,
                   status_change_reason_id, status_change_timestamp, created_at, last_updated_at, updated_by
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

    async fn update_status(&self, account_id: Uuid, status: &str, reason: &str, changed_by: Uuid) -> BankingResult<()> {
        sqlx::query(
            r#"
            UPDATE accounts 
            SET account_status = $2::account_status,
                status_changed_by = $3,
                status_change_timestamp = NOW()
            WHERE account_id = $1
            "#,
        )
        .bind(account_id)
        .bind(status)
        .bind(changed_by)
        .execute(&self.pool)
        .await?;

        // Add status change to history
        sqlx::query(
            r#"
            INSERT INTO account_status_history (
                history_id, account_id, old_status, new_status, change_reason_id,
                additional_context, changed_by, changed_at, system_triggered
            )
            VALUES (uuid_generate_v4(), $1, 
                    (SELECT account_status FROM accounts WHERE account_id = $1),
                    $2::account_status, NULL, $3, $4, NOW(), false)
            "#,
        )
        .bind(account_id)
        .bind(status)
        .bind(reason)
        .bind(changed_by)
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
            WHERE account_id = $1
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
            WHERE account_id = $1
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
            WHERE account_id = $1
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
            WHERE account_id = $1
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
                ownership_id, account_id, customer_id, ownership_type, ownership_percentage
            )
            VALUES ($1, $2, $3, $4::ownership_type, $5)
            RETURNING ownership_id, account_id, customer_id, ownership_type::text as ownership_type,
                     ownership_percentage, created_at
            "#,
        )
        .bind(ownership.ownership_id)
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
            SELECT ownership_id, account_id, customer_id, ownership_type::text as ownership_type,
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
            SELECT ownership_id, account_id, customer_id, ownership_type::text as ownership_type,
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
            "DELETE FROM account_ownership WHERE ownership_id = $1",
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
                relationship_id, account_id, entity_id, entity_type, relationship_type,
                status, start_date, end_date
            )
            VALUES ($1, $2, $3, $4::entity_type, $5::relationship_type, $6::relationship_status, $7, $8)
            RETURNING relationship_id, account_id, entity_id, entity_type::text as entity_type,
                     relationship_type::text as relationship_type, status::text as status,
                     start_date, end_date
            "#,
        )
        .bind(relationship.relationship_id)
        .bind(relationship.account_id)
        .bind(relationship.entity_id)
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
            SELECT relationship_id, account_id, entity_id, entity_type::text as entity_type,
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

    async fn find_relationships_by_entity(&self, entity_id: Uuid, entity_type: &str) -> BankingResult<Vec<AccountRelationshipModel>> {
        let rows = sqlx::query(
            r#"
            SELECT relationship_id, account_id, entity_id, entity_type::text as entity_type,
                   relationship_type::text as relationship_type, status::text as status,
                   start_date, end_date
            FROM account_relationships 
            WHERE entity_id = $1 AND entity_type::text = $2
            ORDER BY start_date DESC
            "#,
        )
        .bind(entity_id)
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
            SET entity_id = $3, entity_type = $4::entity_type, relationship_type = $5::relationship_type,
                status = $6::relationship_status, start_date = $7, end_date = $8
            WHERE relationship_id = $1
            RETURNING relationship_id, account_id, entity_id, entity_type::text as entity_type,
                     relationship_type::text as relationship_type, status::text as status,
                     start_date, end_date
            "#,
        )
        .bind(relationship.relationship_id)
        .bind(relationship.account_id)
        .bind(relationship.entity_id)
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
            "DELETE FROM account_relationships WHERE relationship_id = $1",
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
                mandate_id, account_id, grantee_customer_id, permission_type,
                transaction_limit, approval_group_id, status, start_date, end_date
            )
            VALUES ($1, $2, $3, $4::permission_type, $5, $6, $7::mandate_status, $8, $9)
            RETURNING mandate_id, account_id, grantee_customer_id, permission_type::text as permission_type,
                     transaction_limit, approval_group_id, status::text as status, start_date, end_date
            "#,
        )
        .bind(mandate.mandate_id)
        .bind(mandate.account_id)
        .bind(mandate.grantee_customer_id)
        .bind(mandate.permission_type.to_string())
        .bind(mandate.transaction_limit)
        .bind(mandate.approval_group_id)
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
            SELECT mandate_id, account_id, grantee_customer_id, permission_type::text as permission_type,
                   transaction_limit, approval_group_id, status::text as status, start_date, end_date
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
            SELECT mandate_id, account_id, grantee_customer_id, permission_type::text as permission_type,
                   transaction_limit, approval_group_id, status::text as status, start_date, end_date
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
            WHERE mandate_id = $1
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
            SELECT mandate_id, account_id, grantee_customer_id, permission_type::text as permission_type,
                   transaction_limit, approval_group_id, status::text as status, start_date, end_date
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

    // Account Hold Operations
    async fn create_hold(&self, hold: AccountHoldModel) -> BankingResult<AccountHoldModel> {
        let result = sqlx::query(
            r#"
            INSERT INTO account_holds (
                hold_id, account_id, amount, hold_type, reason_id, additional_details,
                placed_by, placed_at, expires_at, status, released_at, released_by,
                priority, source_reference, automatic_release
            )
            VALUES ($1, $2, $3, $4::hold_type, $5, $6, $7, $8, $9, $10::hold_status, $11, $12, $13::hold_priority, $14, $15)
            RETURNING hold_id, account_id, amount, hold_type::text as hold_type, reason_id,
                     additional_details, placed_by, placed_at, expires_at, status::text as status,
                     released_at, released_by, priority::text as priority, source_reference, automatic_release
            "#,
        )
        .bind(hold.hold_id)
        .bind(hold.account_id)
        .bind(hold.amount)
        .bind(hold.hold_type.to_string())
        .bind(hold.reason_id)
        .bind(hold.additional_details.as_ref().map(|s| s.as_str()))
        .bind(hold.placed_by)
        .bind(hold.placed_at)
        .bind(hold.expires_at)
        .bind(hold.status.to_string())
        .bind(hold.released_at)
        .bind(hold.released_by)
        .bind(hold.priority.to_string())
        .bind(hold.source_reference.as_ref().map(|s| s.as_str()))
        .bind(hold.automatic_release)
        .fetch_one(&self.pool)
        .await?;

        Ok(AccountHoldModel::try_from_row(&result)?)
    }

    async fn find_holds_by_account(&self, account_id: Uuid) -> BankingResult<Vec<AccountHoldModel>> {
        let rows = sqlx::query(
            r#"
            SELECT hold_id, account_id, amount, hold_type::text as hold_type, reason_id,
                   additional_details, placed_by, placed_at, expires_at, status::text as status,
                   released_at, released_by, priority::text as priority, source_reference, automatic_release
            FROM account_holds 
            WHERE account_id = $1
            ORDER BY placed_at DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await?;

        let mut holds = Vec::new();
        for row in rows {
            holds.push(AccountHoldModel::try_from_row(&row)?);
        }
        Ok(holds)
    }

    async fn find_active_holds(&self, account_id: Uuid) -> BankingResult<Vec<AccountHoldModel>> {
        let rows = sqlx::query(
            r#"
            SELECT hold_id, account_id, amount, hold_type::text as hold_type, reason_id,
                   additional_details, placed_by, placed_at, expires_at, status::text as status,
                   released_at, released_by, priority::text as priority, source_reference, automatic_release
            FROM account_holds 
            WHERE account_id = $1 AND status = 'Active'
              AND (expires_at IS NULL OR expires_at > NOW())
            ORDER BY placed_at DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await?;

        let mut holds = Vec::new();
        for row in rows {
            holds.push(AccountHoldModel::try_from_row(&row)?);
        }
        Ok(holds)
    }

    async fn release_hold(&self, hold_id: Uuid, released_by: Uuid) -> BankingResult<()> {
        sqlx::query(
            r#"
            UPDATE account_holds 
            SET status = 'Released',
                released_at = NOW(),
                released_by = $2
            WHERE hold_id = $1
            "#,
        )
        .bind(hold_id)
        .bind(released_by)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn release_expired_holds(&self, reference_date: DateTime<Utc>) -> BankingResult<i64> {
        let result = sqlx::query(
            r#"
            UPDATE account_holds 
            SET status = 'Expired',
                released_at = NOW()
            WHERE status = 'Active' 
              AND expires_at IS NOT NULL 
              AND expires_at <= $1
              AND automatic_release = true
            "#,
        )
        .bind(reference_date)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() as i64)
    }

    // Final Settlement Operations
    async fn create_final_settlement(&self, settlement: AccountFinalSettlementModel) -> BankingResult<AccountFinalSettlementModel> {
        let result = sqlx::query(
            r#"
            INSERT INTO final_settlements (
                settlement_id, account_id, settlement_date, current_balance, accrued_interest,
                closure_fees, final_amount, disbursement_method, disbursement_reference, processed_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8::disbursement_method, $9, $10)
            RETURNING settlement_id, account_id, settlement_date, current_balance, accrued_interest,
                     closure_fees, final_amount, disbursement_method::text as disbursement_method,
                     disbursement_reference, processed_by, created_at
            "#,
        )
        .bind(settlement.settlement_id)
        .bind(settlement.account_id)
        .bind(settlement.settlement_date)
        .bind(settlement.current_balance)
        .bind(settlement.accrued_interest)
        .bind(settlement.closure_fees)
        .bind(settlement.final_amount)
        .bind(settlement.disbursement_method.to_string())
        .bind(settlement.disbursement_reference.as_ref().map(|s| s.as_str()))
        .bind(settlement.processed_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(AccountFinalSettlementModel::try_from_row(&result)?)
    }

    async fn find_settlement_by_account(&self, account_id: Uuid) -> BankingResult<Option<AccountFinalSettlementModel>> {
        let result = sqlx::query(
            r#"
            SELECT settlement_id, account_id, settlement_date, current_balance, accrued_interest,
                   closure_fees, final_amount, disbursement_method::text as disbursement_method,
                   disbursement_reference, processed_by, created_at
            FROM final_settlements
            WHERE account_id = $1
            "#,
        )
        .bind(account_id)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => Ok(Some(AccountFinalSettlementModel::try_from_row(&row)?)),
            None => Ok(None),
        }
    }

    async fn update_settlement_status(&self, _settlement_id: Uuid, _status: &str) -> BankingResult<()> {
        // Note: Status field not in current schema, but method required by trait
        // This would need to be added to the settlement table if status tracking is needed
        Ok(())
    }

    // Status History Operations
    async fn get_status_history(&self, account_id: Uuid) -> BankingResult<Vec<StatusChangeModel>> {
        let rows = sqlx::query(
            r#"
            SELECT history_id, account_id, old_status::text as old_status, new_status::text as new_status,
                   change_reason_id, additional_context, changed_by, changed_at, system_triggered, created_at
            FROM account_status_history 
            WHERE account_id = $1
            ORDER BY changed_at DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await?;

        let mut history = Vec::new();
        for row in rows {
            history.push(StatusChangeModel::try_from_row(&row)?);
        }
        Ok(history)
    }

    async fn add_status_change(&self, status_change: StatusChangeModel) -> BankingResult<StatusChangeModel> {
        let result = sqlx::query(
            r#"
            INSERT INTO account_status_history (
                history_id, account_id, old_status, new_status, change_reason_id,
                additional_context, changed_by, changed_at, system_triggered
            )
            VALUES ($1, $2, $3::account_status, $4::account_status, $5, $6, $7, $8, $9)
            RETURNING history_id, account_id, old_status::text as old_status, new_status::text as new_status,
                     change_reason_id, additional_context, changed_by, changed_at, system_triggered, created_at
            "#,
        )
        .bind(status_change.history_id)
        .bind(status_change.account_id)
        .bind(status_change.old_status.as_ref().map(|s| s.to_string()))
        .bind(status_change.new_status.to_string())
        .bind(status_change.change_reason_id)
        .bind(status_change.additional_context.as_ref().map(|s| s.as_str()))
        .bind(status_change.changed_by)
        .bind(status_change.changed_at)
        .bind(status_change.system_triggered)
        .fetch_one(&self.pool)
        .await?;

        Ok(StatusChangeModel::try_from_row(&result)?)
    }

    // Utility Operations
    async fn exists(&self, account_id: Uuid) -> BankingResult<bool> {
        let result = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM accounts WHERE account_id = $1)",
        )
        .bind(account_id)
        .fetch_one(&self.pool)
        .await?;

        use sqlx::Row;
        Ok(result.get::<bool, _>("exists"))
    }

    async fn count_by_customer(&self, customer_id: Uuid) -> BankingResult<i64> {
        let result = sqlx::query(
            r#"
            SELECT COUNT(a.account_id) as count
            FROM accounts a
            INNER JOIN account_ownership ao ON a.account_id = ao.account_id
            WHERE ao.customer_id = $1
            "#,
        )
        .bind(customer_id)
        .fetch_one(&self.pool)
        .await?;

        use sqlx::Row;
        Ok(result.get::<i64, _>("count"))
    }

    async fn count_by_product(&self, product_code: &str) -> BankingResult<i64> {
        let result = sqlx::query(
            "SELECT COUNT(*) as count FROM accounts WHERE product_code = $1",
        )
        .bind(product_code)
        .fetch_one(&self.pool)
        .await?;

        use sqlx::Row;
        Ok(result.get::<i64, _>("count"))
    }

    async fn list(&self, offset: i64, limit: i64) -> BankingResult<Vec<AccountModel>> {
        let rows = sqlx::query(
            r#"
            SELECT account_id, product_code, account_type::text as account_type,
                   account_status::text as account_status, signing_condition::text as signing_condition,
                   currency, open_date, domicile_branch_id, current_balance, available_balance,
                   accrued_interest, overdraft_limit, original_principal, outstanding_principal,
                   loan_interest_rate, loan_term_months, disbursement_date, maturity_date,
                   installment_amount, next_due_date, penalty_rate, collateral_id, loan_purpose_id,
                   close_date, last_activity_date, dormancy_threshold_days, reactivation_required,
                   pending_closure_reason_id, last_disbursement_instruction_id, status_changed_by,
                   status_change_reason_id, status_change_timestamp, created_at, last_updated_at, updated_by
            FROM accounts 
            ORDER BY created_at DESC, account_id ASC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let mut accounts = Vec::new();
        for row in rows {
            accounts.push(AccountModel::try_from_row(&row)?);
        }
        Ok(accounts)
    }

    async fn count(&self) -> BankingResult<i64> {
        let result = sqlx::query("SELECT COUNT(*) as count FROM accounts")
            .fetch_one(&self.pool)
            .await?;

        use sqlx::Row;
        Ok(result.get::<i64, _>("count"))
    }
}

// Helper trait for converting database rows to models
trait TryFromRow<T> {
    fn try_from_row(row: &T) -> BankingResult<Self>
    where
        Self: Sized;
}

impl TryFromRow<sqlx::postgres::PgRow> for AccountModel {
    fn try_from_row(row: &sqlx::postgres::PgRow) -> BankingResult<Self> {
        use sqlx::Row;
        
        let account_type_str: String = row.get("account_type");
        let account_type = match account_type_str.as_str() {
            "Savings" => AccountType::Savings,
            "Current" => AccountType::Current,
            "Loan" => AccountType::Loan,
            _ => return Err(BankingError::ValidationError {
                field: "account_type".to_string(),
                message: format!("Invalid account type: {account_type_str}"),
            }),
        };

        let account_status_str: String = row.get("account_status");
        let account_status = match account_status_str.as_str() {
            "PendingApproval" => AccountStatus::PendingApproval,
            "Active" => AccountStatus::Active,
            "Dormant" => AccountStatus::Dormant,
            "Frozen" => AccountStatus::Frozen,
            "PendingClosure" => AccountStatus::PendingClosure,
            "Closed" => AccountStatus::Closed,
            "PendingReactivation" => AccountStatus::PendingReactivation,
            _ => return Err(BankingError::ValidationError {
                field: "account_status".to_string(),
                message: format!("Invalid account status: {account_status_str}"),
            }),
        };

        let signing_condition_str: String = row.get("signing_condition");
        let signing_condition = match signing_condition_str.as_str() {
            "None" => SigningCondition::None,
            "AnyOwner" => SigningCondition::AnyOwner,
            "AllOwners" => SigningCondition::AllOwners,
            _ => return Err(BankingError::ValidationError {
                field: "signing_condition".to_string(),
                message: format!("Invalid signing condition: {signing_condition_str}"),
            }),
        };

        let product_code_str: String = row.get("product_code");
        let product_code = HeaplessString::try_from(product_code_str.as_str())
            .map_err(|_| BankingError::ValidationError {
                field: "product_code".to_string(),
                message: "Product code too long".to_string(),
            })?;

        let currency_str: String = row.get("currency");
        let currency = HeaplessString::try_from(currency_str.as_str())
            .map_err(|_| BankingError::ValidationError {
                field: "currency".to_string(),
                message: "Currency code too long".to_string(),
            })?;

        let collateral_id: Option<Uuid> = row.get("collateral_id");

        Ok(AccountModel {
            account_id: row.get("account_id"),
            product_code,
            account_type,
            account_status,
            signing_condition,
            currency,
            open_date: row.get("open_date"),
            domicile_branch_id: row.get("domicile_branch_id"),
            current_balance: row.get("current_balance"),
            available_balance: row.get("available_balance"),
            accrued_interest: row.get("accrued_interest"),
            overdraft_limit: row.get("overdraft_limit"),
            original_principal: row.get("original_principal"),
            outstanding_principal: row.get("outstanding_principal"),
            loan_interest_rate: row.get("loan_interest_rate"),
            loan_term_months: row.get("loan_term_months"),
            disbursement_date: row.get("disbursement_date"),
            maturity_date: row.get("maturity_date"),
            installment_amount: row.get("installment_amount"),
            next_due_date: row.get("next_due_date"),
            penalty_rate: row.get("penalty_rate"),
            collateral_id,
            loan_purpose_id: row.get("loan_purpose_id"),
            close_date: row.get("close_date"),
            last_activity_date: row.get("last_activity_date"),
            dormancy_threshold_days: row.get("dormancy_threshold_days"),
            reactivation_required: row.get("reactivation_required"),
            pending_closure_reason_id: row.get("pending_closure_reason_id"),
            last_disbursement_instruction_id: row.get("last_disbursement_instruction_id"),
            status_changed_by: row.get("status_changed_by"),
            status_change_reason_id: row.get("status_change_reason_id"),
            status_change_timestamp: row.get("status_change_timestamp"),
            created_at: row.get("created_at"),
            last_updated_at: row.get("last_updated_at"),
            updated_by: row.get("updated_by"),
        })
    }
}

impl TryFromRow<sqlx::postgres::PgRow> for AccountOwnershipModel {
    fn try_from_row(row: &sqlx::postgres::PgRow) -> BankingResult<Self> {
        use sqlx::Row;
        
        let ownership_type_str: String = row.get("ownership_type");
        let ownership_type = match ownership_type_str.as_str() {
            "Single" => OwnershipType::Single,
            "Joint" => OwnershipType::Joint,
            "Corporate" => OwnershipType::Corporate,
            _ => return Err(BankingError::ValidationError {
                field: "ownership_type".to_string(),
                message: format!("Invalid ownership type: {ownership_type_str}"),
            }),
        };

        Ok(AccountOwnershipModel {
            ownership_id: row.get("ownership_id"),
            account_id: row.get("account_id"),
            customer_id: row.get("customer_id"),
            ownership_type,
            ownership_percentage: row.get("ownership_percentage"),
            created_at: row.get("created_at"),
        })
    }
}

impl TryFromRow<sqlx::postgres::PgRow> for AccountRelationshipModel {
    fn try_from_row(row: &sqlx::postgres::PgRow) -> BankingResult<Self> {
        use sqlx::Row;
        
        let entity_type_str: String = row.get("entity_type");
        let entity_type = match entity_type_str.as_str() {
            "Branch" => EntityType::Branch,
            "Agent" => EntityType::Agent,
            "RiskManager" => EntityType::RiskManager,
            "ComplianceOfficer" => EntityType::ComplianceOfficer,
            "CustomerService" => EntityType::CustomerService,
            _ => return Err(BankingError::ValidationError {
                field: "entity_type".to_string(),
                message: format!("Invalid entity type: {entity_type_str}"),
            }),
        };

        let relationship_type_str: String = row.get("relationship_type");
        let relationship_type = match relationship_type_str.as_str() {
            "PrimaryHandler" => RelationshipType::PrimaryHandler,
            "BackupHandler" => RelationshipType::BackupHandler,
            "RiskOversight" => RelationshipType::RiskOversight,
            "ComplianceOversight" => RelationshipType::ComplianceOversight,
            _ => return Err(BankingError::ValidationError {
                field: "relationship_type".to_string(),
                message: format!("Invalid relationship type: {relationship_type_str}"),
            }),
        };

        let status_str: String = row.get("status");
        let status = match status_str.as_str() {
            "Active" => RelationshipStatus::Active,
            "Inactive" => RelationshipStatus::Inactive,
            "Suspended" => RelationshipStatus::Suspended,
            _ => return Err(BankingError::ValidationError {
                field: "status".to_string(),
                message: format!("Invalid relationship status: {status_str}"),
            }),
        };

        Ok(AccountRelationshipModel {
            relationship_id: row.get("relationship_id"),
            account_id: row.get("account_id"),
            entity_id: row.get("entity_id"),
            entity_type,
            relationship_type,
            status,
            start_date: row.get("start_date"),
            end_date: row.get("end_date"),
        })
    }
}

impl TryFromRow<sqlx::postgres::PgRow> for AccountMandateModel {
    fn try_from_row(row: &sqlx::postgres::PgRow) -> BankingResult<Self> {
        use sqlx::Row;
        
        let permission_type_str: String = row.get("permission_type");
        let permission_type = match permission_type_str.as_str() {
            "ViewOnly" => PermissionType::ViewOnly,
            "LimitedWithdrawal" => PermissionType::LimitedWithdrawal,
            "JointApproval" => PermissionType::JointApproval,
            "FullAccess" => PermissionType::FullAccess,
            _ => return Err(BankingError::ValidationError {
                field: "permission_type".to_string(),
                message: format!("Invalid permission type: {permission_type_str}"),
            }),
        };

        let status_str: String = row.get("status");
        let status = match status_str.as_str() {
            "Active" => MandateStatus::Active,
            "Suspended" => MandateStatus::Suspended,
            "Revoked" => MandateStatus::Revoked,
            "Expired" => MandateStatus::Expired,
            _ => return Err(BankingError::ValidationError {
                field: "status".to_string(),
                message: format!("Invalid mandate status: {status_str}"),
            }),
        };

        Ok(AccountMandateModel {
            mandate_id: row.get("mandate_id"),
            account_id: row.get("account_id"),
            grantee_customer_id: row.get("grantee_customer_id"),
            permission_type,
            transaction_limit: row.get("transaction_limit"),
            approval_group_id: row.get("approval_group_id"),
            status,
            start_date: row.get("start_date"),
            end_date: row.get("end_date"),
        })
    }
}

impl TryFromRow<sqlx::postgres::PgRow> for AccountHoldModel {
    fn try_from_row(row: &sqlx::postgres::PgRow) -> BankingResult<Self> {
        use sqlx::Row;
        
        let hold_type_str: String = row.get("hold_type");
        let hold_type = match hold_type_str.as_str() {
            "UnclearedFunds" => HoldType::UnclearedFunds,
            "JudicialLien" => HoldType::JudicialLien,
            "LoanPledge" => HoldType::LoanPledge,
            "ComplianceHold" => HoldType::ComplianceHold,
            "AdministrativeHold" => HoldType::AdministrativeHold,
            "FraudHold" => HoldType::FraudHold,
            "PendingAuthorization" => HoldType::PendingAuthorization,
            "OverdraftReserve" => HoldType::OverdraftReserve,
            "CardAuthorization" => HoldType::CardAuthorization,
            "Other" => HoldType::Other,
            _ => return Err(BankingError::ValidationError {
                field: "hold_type".to_string(),
                message: format!("Invalid hold type: {hold_type_str}"),
            }),
        };

        let status_str: String = row.get("status");
        let status = match status_str.as_str() {
            "Active" => HoldStatus::Active,
            "Released" => HoldStatus::Released,
            "Expired" => HoldStatus::Expired,
            "Cancelled" => HoldStatus::Cancelled,
            "PartiallyReleased" => HoldStatus::PartiallyReleased,
            _ => return Err(BankingError::ValidationError {
                field: "status".to_string(),
                message: format!("Invalid hold status: {status_str}"),
            }),
        };

        let priority_str: String = row.get("priority");
        let priority = match priority_str.as_str() {
            "Critical" => HoldPriority::Critical,
            "High" => HoldPriority::High,
            "Medium" => HoldPriority::Medium,
            "Low" => HoldPriority::Low,
            _ => return Err(BankingError::ValidationError {
                field: "priority".to_string(),
                message: format!("Invalid hold priority: {priority_str}"),
            }),
        };

        let additional_details: Option<String> = row.get("additional_details");
        let additional_details_heapless = additional_details.as_ref()
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| BankingError::ValidationError {
                field: "additional_details".to_string(),
                message: "Additional details too long".to_string(),
            })?;

        let source_reference: Option<String> = row.get("source_reference");
        let source_reference_heapless = source_reference.as_ref()
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| BankingError::ValidationError {
                field: "source_reference".to_string(),
                message: "Source reference too long".to_string(),
            })?;

        Ok(AccountHoldModel {
            hold_id: row.get("hold_id"),
            account_id: row.get("account_id"),
            amount: row.get("amount"),
            hold_type,
            reason_id: row.get("reason_id"),
            additional_details: additional_details_heapless,
            placed_by: row.get("placed_by"),
            placed_at: row.get("placed_at"),
            expires_at: row.get("expires_at"),
            status,
            released_at: row.get("released_at"),
            released_by: row.get("released_by"),
            priority,
            source_reference: source_reference_heapless,
            automatic_release: row.get("automatic_release"),
        })
    }
}

impl TryFromRow<sqlx::postgres::PgRow> for AccountFinalSettlementModel {
    fn try_from_row(row: &sqlx::postgres::PgRow) -> BankingResult<Self> {
        use sqlx::Row;
        
        let disbursement_method_str: String = row.get("disbursement_method");
        let disbursement_method = match disbursement_method_str.as_str() {
            "Transfer" => DisbursementMethod::Transfer,
            "CashWithdrawal" => DisbursementMethod::CashWithdrawal,
            "Check" => DisbursementMethod::Check,
            "HoldFunds" => DisbursementMethod::HoldFunds,
            "OverdraftFacility" => DisbursementMethod::OverdraftFacility,
            "StagedRelease" => DisbursementMethod::StagedRelease,
            _ => return Err(BankingError::ValidationError {
                field: "disbursement_method".to_string(),
                message: format!("Invalid disbursement method: {disbursement_method_str}"),
            }),
        };

        let disbursement_reference: Option<String> = row.get("disbursement_reference");
        let disbursement_reference_heapless = disbursement_reference.as_ref()
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| BankingError::ValidationError {
                field: "disbursement_reference".to_string(),
                message: "Disbursement reference too long".to_string(),
            })?;

        Ok(AccountFinalSettlementModel {
            settlement_id: row.get("settlement_id"),
            account_id: row.get("account_id"),
            settlement_date: row.get("settlement_date"),
            current_balance: row.get("current_balance"),
            accrued_interest: row.get("accrued_interest"),
            closure_fees: row.get("closure_fees"),
            final_amount: row.get("final_amount"),
            disbursement_method,
            disbursement_reference: disbursement_reference_heapless,
            processed_by: row.get("processed_by"),
            created_at: row.get("created_at"),
        })
    }
}

impl TryFromRow<sqlx::postgres::PgRow> for StatusChangeModel {
    fn try_from_row(row: &sqlx::postgres::PgRow) -> BankingResult<Self> {
        use sqlx::Row;
        
        let old_status_str: Option<String> = row.get("old_status");
        let old_status = old_status_str.as_ref()
            .map(|s| match s.as_str() {
                "PendingApproval" => Ok(AccountStatus::PendingApproval),
                "Active" => Ok(AccountStatus::Active),
                "Dormant" => Ok(AccountStatus::Dormant),
                "Frozen" => Ok(AccountStatus::Frozen),
                "PendingClosure" => Ok(AccountStatus::PendingClosure),
                "Closed" => Ok(AccountStatus::Closed),
                "PendingReactivation" => Ok(AccountStatus::PendingReactivation),
                _ => Err(BankingError::ValidationError {
                    field: "old_status".to_string(),
                    message: format!("Invalid old status: {s}"),
                }),
            })
            .transpose()?;

        let new_status_str: String = row.get("new_status");
        let new_status = match new_status_str.as_str() {
            "PendingApproval" => AccountStatus::PendingApproval,
            "Active" => AccountStatus::Active,
            "Dormant" => AccountStatus::Dormant,
            "Frozen" => AccountStatus::Frozen,
            "PendingClosure" => AccountStatus::PendingClosure,
            "Closed" => AccountStatus::Closed,
            "PendingReactivation" => AccountStatus::PendingReactivation,
            _ => return Err(BankingError::ValidationError {
                field: "new_status".to_string(),
                message: format!("Invalid new status: {new_status_str}"),
            }),
        };

        let additional_context: Option<String> = row.get("additional_context");
        let additional_context_heapless = additional_context.as_ref()
            .map(|s| HeaplessString::try_from(s.as_str()))
            .transpose()
            .map_err(|_| BankingError::ValidationError {
                field: "additional_context".to_string(),
                message: "Additional context too long".to_string(),
            })?;

        Ok(StatusChangeModel {
            history_id: row.get("history_id"),
            account_id: row.get("account_id"),
            old_status,
            new_status,
            change_reason_id: row.get("change_reason_id"),
            additional_context: additional_context_heapless,
            changed_by: row.get("changed_by"),
            changed_at: row.get("changed_at"),
            system_triggered: row.get("system_triggered"),
            created_at: row.get("created_at"),
        })
    }
}
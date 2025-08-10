use std::sync::Arc;

use async_trait::async_trait;
use banking_api::{
    domain::{
        Account, AccountBalanceCalculation, AccountMandate, AccountOwnership,
        AccountRelationship, AccountStatusChangeRecord, UltimateBeneficiary, AccountStatus,
    },
    error::BankingError,
    service::{AccountService, AccountServiceExt, HoldAuthorizationLevel, HoldAnalytics, HighHoldAccount, JudicialHoldReport},
    BankingResult,
};
use banking_db::{
    repository::{
        AccountRepository, AccountRepositoryExt, MockAccountRepository,
    },
    AccountRepositoryType,
};
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;


use crate::mappers::AccountMapper;

#[derive(Clone)]
pub struct AccountServiceImpl {
    account_repo: AccountRepositoryType,
}

impl AccountServiceImpl {
    pub fn new(account_repo: AccountRepositoryType) -> Self {
        Self { account_repo }
    }
}

#[async_trait]
impl AccountService for AccountServiceImpl {
    async fn create_account(&self, account: Account) -> BankingResult<Account> {
        let model = AccountMapper::to_model(account);
        let result = self.account_repo.create_account(model).await?;
        AccountMapper::from_model(result)
    }

    async fn find_account_by_id(&self, account_id: Uuid) -> BankingResult<Option<Account>> {
        let result = self.account_repo.find_account_by_id(account_id).await?;
        Ok(result.map(|m| AccountMapper::from_model(m).unwrap()))
    }

    async fn update_account_status(&self, account_id: Uuid, status: AccountStatus, authorized_by_person_id: Uuid) -> BankingResult<()> {
        unimplemented!()
    }

    async fn calculate_balance(&self, account_id: Uuid) -> BankingResult<Decimal> {
        unimplemented!()
    }

    async fn calculate_available_balance(&self, account_id: Uuid) -> BankingResult<Decimal> {
        unimplemented!()
    }

    async fn apply_hold(&self, account_id: Uuid, amount: Decimal, reason_id: Uuid, additional_details: Option<&str>) -> BankingResult<()> {
        unimplemented!()
    }

    async fn apply_hold_legacy(&self, account_id: Uuid, amount: Decimal, reason: String) -> BankingResult<()> {
        unimplemented!()
    }

    async fn refresh_product_rules(&self, account_id: Uuid) -> BankingResult<()> {
        unimplemented!()
    }

    async fn find_accounts_by_customer(&self, customer_id: Uuid) -> BankingResult<Vec<Account>> {
        unimplemented!()
    }

    async fn find_accounts_by_status(&self, status: AccountStatus) -> BankingResult<Vec<Account>> {
        unimplemented!()
    }

    async fn find_interest_bearing_accounts(&self) -> BankingResult<Vec<Account>> {
        unimplemented!()
    }

    async fn update_balance(&self, account_id: Uuid, new_balance: Decimal, updated_by_person_id: Uuid) -> BankingResult<()> {
        unimplemented!()
    }

    async fn reset_accrued_interest(&self, account_id: Uuid) -> BankingResult<()> {
        unimplemented!()
    }

    async fn update_accrued_interest(&self, account_id: Uuid, amount: Decimal) -> BankingResult<()> {
        unimplemented!()
    }

    async fn get_account_status(&self, account_id: Uuid) -> BankingResult<AccountStatus> {
        unimplemented!()
    }



    async fn find_dormancy_candidates(&self, threshold_days: i32) -> BankingResult<Vec<Account>> {
        unimplemented!()
    }

    async fn update_last_activity_date(&self, account_id: Uuid, activity_date: chrono::NaiveDate) -> BankingResult<()> {
        unimplemented!()
    }


    async fn calculate_available_balance_detailed(
        &self,
        account_id: Uuid,
    ) -> BankingResult<AccountBalanceCalculation> {
        unimplemented!()
    }

    async fn validate_transaction_against_holds(
        &self,
        account_id: Uuid,
        transaction_amount: Decimal,
        ignore_hold_types: Option<Vec<banking_api::domain::HoldType>>,
    ) -> BankingResult<bool> {
        unimplemented!()
    }

    async fn get_hold_amounts_by_priority(
        &self,
        account_id: Uuid,
    ) -> BankingResult<Vec<AccountHoldSummary>> {
        unimplemented!()
    }

    async fn validate_hold_placement(
        &self,
        account_id: Uuid,
        additional_hold_amount: Decimal,
        hold_priority: banking_api::domain::HoldPriority,
    ) -> BankingResult<bool> {
        unimplemented!()
    }
}
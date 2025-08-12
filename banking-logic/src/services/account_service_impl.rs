use std::sync::Arc;

use async_trait::async_trait;
use banking_api::{
    domain::{
        Account, AccountBalanceCalculation, AccountStatus, AccountHoldSummary,
    },
    service::{AccountService, HoldAuthorizationLevel, HoldAnalytics, HighHoldAccount, JudicialHoldReport},
    BankingResult,
};
use banking_db::{
    repository::{
        AccountRepository,
    },
};
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;


use crate::mappers::AccountMapper;

#[derive(Clone)]
pub struct AccountServiceImpl {
    account_repo: Arc<dyn AccountRepository>,
}

impl AccountServiceImpl {
    pub fn new(account_repo: Arc<dyn AccountRepository>) -> Self {
        Self { account_repo }
    }
}

#[async_trait]
impl AccountService for AccountServiceImpl {
    async fn create_account(&self, account: Account) -> BankingResult<Account> {
        let model = AccountMapper::to_model(account);
        let result = self.account_repo.create(model).await?;
        AccountMapper::from_model(result)
    }

    async fn find_account_by_id(&self, account_id: Uuid) -> BankingResult<Option<Account>> {
        let result = self.account_repo.find_by_id(account_id).await?;
        Ok(result.map(|m| AccountMapper::from_model(m).unwrap()))
    }

    async fn update_account_status(&self, _account_id: Uuid, _status: AccountStatus, _authorized_by_person_id: Uuid) -> BankingResult<()> {
        unimplemented!()
    }

    async fn calculate_balance(&self, _account_id: Uuid) -> BankingResult<Decimal> {
        unimplemented!()
    }

    async fn calculate_available_balance(&self, _account_id: Uuid) -> BankingResult<Decimal> {
        unimplemented!()
    }

    async fn apply_hold(&self, _account_id: Uuid, _amount: Decimal, _reason_id: Uuid, _additional_details: Option<&str>) -> BankingResult<()> {
        unimplemented!()
    }

    async fn apply_hold_legacy(&self, _account_id: Uuid, _amount: Decimal, _reason: String) -> BankingResult<()> {
        unimplemented!()
    }

    async fn refresh_product_rules(&self, _account_id: Uuid) -> BankingResult<()> {
        unimplemented!()
    }

    async fn find_accounts_by_customer(&self, _customer_id: Uuid) -> BankingResult<Vec<Account>> {
        unimplemented!()
    }

    async fn find_accounts_by_status(&self, _status: AccountStatus) -> BankingResult<Vec<Account>> {
        unimplemented!()
    }

    async fn find_interest_bearing_accounts(&self) -> BankingResult<Vec<Account>> {
        unimplemented!()
    }

    async fn update_balance(&self, _account_id: Uuid, _new_balance: Decimal, _updated_by_person_id: Uuid) -> BankingResult<()> {
        unimplemented!()
    }

    async fn reset_accrued_interest(&self, _account_id: Uuid) -> BankingResult<()> {
        unimplemented!()
    }

    async fn update_accrued_interest(&self, _account_id: Uuid, _amount: Decimal) -> BankingResult<()> {
        unimplemented!()
    }

    async fn get_account_status(&self, _account_id: Uuid) -> BankingResult<AccountStatus> {
        unimplemented!()
    }



    async fn find_dormancy_candidates(&self, _threshold_days: i32) -> BankingResult<Vec<Account>> {
        unimplemented!()
    }

    async fn update_last_activity_date(&self, _account_id: Uuid, _activity_date: chrono::NaiveDate) -> BankingResult<()> {
        unimplemented!()
    }


    async fn calculate_available_balance_detailed(
        &self,
        _account_id: Uuid,
    ) -> BankingResult<AccountBalanceCalculation> {
        unimplemented!()
    }

    async fn validate_transaction_against_holds(
        &self,
        _account_id: Uuid,
        _transaction_amount: Decimal,
        _ignore_hold_types: Option<Vec<banking_api::domain::HoldType>>,
    ) -> BankingResult<bool> {
        unimplemented!()
    }

    async fn get_hold_amounts_by_priority(
        &self,
        _account_id: Uuid,
    ) -> BankingResult<Vec<AccountHoldSummary>> {
        unimplemented!()
    }

    async fn validate_hold_placement(
        &self,
        _account_id: Uuid,
        _additional_hold_amount: Decimal,
        _hold_priority: banking_api::domain::HoldPriority,
    ) -> BankingResult<bool> {
        unimplemented!()
    }
    
    async fn process_expired_holds(
        &self,
        _processing_date: NaiveDate,
        _hold_types: Option<Vec<banking_api::domain::HoldType>>,
    ) -> BankingResult<banking_api::domain::AccountHoldExpiryJob> {
        todo!()
    }

    async fn process_automatic_releases(
        &self,
        _processing_date: NaiveDate,
    ) -> BankingResult<Vec<banking_api::domain::AccountHold>> {
        todo!()
    }

    async fn bulk_place_holds(
        &self,
        _account_ids: Vec<Uuid>,
        _hold_type: banking_api::domain::HoldType,
        _amount_per_account: Decimal,
        _reason_id: Uuid,
        _placed_by_person_id: Uuid,
        _expires_at: Option<DateTime<Utc>>,
    ) -> BankingResult<Vec<banking_api::domain::AccountHold>> {
        todo!()
    }

    async fn bulk_release_holds(
        &self,
        _hold_ids: Vec<Uuid>,
        _release_reason_id: Uuid,
        _released_by_person_id: Uuid,
    ) -> BankingResult<Vec<banking_api::domain::AccountHold>> {
        todo!()
    }

    async fn override_holds_for_transaction(
        &self,
        _account_id: Uuid,
        _transaction_amount: Decimal,
        _override_priority: banking_api::domain::HoldPriority,
        _authorized_by_person_id: Uuid,
        _override_reason_id: Uuid,
    ) -> BankingResult<Vec<banking_api::domain::AccountHold>> {
        todo!()
    }

    async fn reorder_hold_priorities(
        &self,
        _account_id: Uuid,
        _hold_priority_map: Vec<(Uuid, banking_api::domain::HoldPriority)>,
        _authorized_by_person_id: Uuid,
    ) -> BankingResult<Vec<banking_api::domain::AccountHold>> {
        todo!()
    }

    async fn get_required_authorization_level(
        &self,
        _hold_type: banking_api::domain::HoldType,
        _amount: Decimal,
    ) -> BankingResult<HoldAuthorizationLevel> {
        todo!()
    }

    async fn sync_judicial_holds(
        &self,
        _court_reference: String,
    ) -> BankingResult<Vec<banking_api::domain::AccountHold>> {
        todo!()
    }

    async fn update_loan_pledge_holds(
        &self,
        _loan_account_id: Uuid,
        _collateral_account_ids: Vec<Uuid>,
        _new_pledge_amount: Decimal,
    ) -> BankingResult<Vec<banking_api::domain::AccountHold>> {
        todo!()
    }

    async fn process_compliance_holds(
        &self,
        _compliance_alert_id: Uuid,
        _affected_accounts: Vec<Uuid>,
        _hold_amount_per_account: Decimal,
    ) -> BankingResult<Vec<banking_api::domain::AccountHold>> {
        todo!()
    }

    async fn get_hold_analytics(
        &self,
        _from_date: NaiveDate,
        _to_date: NaiveDate,
        _hold_types: Option<Vec<banking_api::domain::HoldType>>,
    ) -> BankingResult<HoldAnalytics> {
        todo!()
    }

    async fn get_high_hold_ratio_accounts(
        &self,
        _minimum_ratio: Decimal,
        _exclude_hold_types: Option<Vec<banking_api::domain::HoldType>>,
    ) -> BankingResult<Vec<HighHoldAccount>> {
        todo!()
    }

    async fn generate_judicial_hold_report(
        &self,
        _from_date: NaiveDate,
        _to_date: NaiveDate,
    ) -> BankingResult<JudicialHoldReport> {
        todo!()
    }
}
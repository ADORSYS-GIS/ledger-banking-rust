use std::sync::Arc;

use async_trait::async_trait;
use banking_api::{
    domain::{
        AccountHold, AccountHoldExpiryJob, AccountHoldReleaseRequest, AccountHoldSummary,
        HoldPriority, HoldStatus, HoldType, PlaceHoldRequest,
    },
    error::BankingError,
    service::{AccountHoldService, HighHoldAccount, HoldAnalytics, HoldAuthorizationLevel, JudicialHoldReport},
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

use crate::mappers::AccountHoldMapper;

#[derive(Clone)]
pub struct AccountHoldServiceImpl {
    account_repo: AccountRepositoryType,
}

impl AccountHoldServiceImpl {
    pub fn new(account_repo: AccountRepositoryType) -> Self {
        Self { account_repo }
    }
}

#[async_trait]
impl AccountHoldService for AccountHoldServiceImpl {
    async fn get_active_holds(&self, account_id: Uuid) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn release_hold(&self, hold_id: Uuid, released_by_person_id: Uuid) -> BankingResult<()> {
        unimplemented!()
    }

    async fn place_hold(
        &self,
        request: PlaceHoldRequest,
    ) -> BankingResult<AccountHold> {
        let id = Uuid::new_v4();
        let model = AccountHoldMapper::place_hold_request_to_model(request, id);
        // this should be calling a repo method to create a hold, which doesn't exist yet
        // for now, we'll just return a dummy response
        let hold = AccountHold {
            id,
            account_id: model.account_id,
            amount: model.amount,
            hold_type: model.hold_type,
            reason_id: model.reason_id,
            additional_details: model.additional_details,
            placed_by_person_id: model.placed_by_person_id,
            placed_at: Utc::now(),
            expires_at: model.expires_at,
            status: HoldStatus::Active,
            released_at: None,
            released_by_person_id: None,
            priority: model.priority,
            source_reference: model.source_reference,
            automatic_release: false,
        };
        Ok(hold)
    }

    async fn release_hold_with_request(
        &self,
        release_request: AccountHoldReleaseRequest,
    ) -> BankingResult<AccountHold> {
        unimplemented!()
    }

    async fn modify_hold(
        &self,
        hold_id: Uuid,
        new_amount: Option<Decimal>,
        new_expiry: Option<DateTime<Utc>>,
        new_reason_id: Option<Uuid>,
        modified_by_person_id: Uuid,
    ) -> BankingResult<AccountHold> {
        unimplemented!()
    }

    async fn cancel_hold(
        &self,
        hold_id: Uuid,
        cancellation_reason_id: Uuid,
        cancelled_by_person_id: Uuid,
    ) -> BankingResult<AccountHold> {
        unimplemented!()
    }

    async fn get_hold_amounts_by_priority(
        &self,
        account_id: Uuid,
    ) -> BankingResult<Vec<AccountHoldSummary>> {
        unimplemented!()
    }

    async fn get_active_holds_with_types(
        &self,
        account_id: Uuid,
        hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn get_hold_by_id(
        &self,
        hold_id: Uuid,
    ) -> BankingResult<Option<AccountHold>> {
        unimplemented!()
    }

    async fn get_holds_by_status(
        &self,
        account_id: Option<Uuid>,
        status: HoldStatus,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn get_holds_by_type(
        &self,
        hold_type: HoldType,
        status: Option<HoldStatus>,
        account_ids: Option<Vec<Uuid>>,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn get_hold_history(
        &self,
        account_id: Uuid,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn process_expired_holds(
        &self,
        processing_date: NaiveDate,
        hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<AccountHoldExpiryJob> {
        unimplemented!()
    }

    async fn process_automatic_releases(
        &self,
        processing_date: NaiveDate,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn bulk_place_holds(
        &self,
        account_ids: Vec<Uuid>,
        hold_type: HoldType,
        amount_per_account: Decimal,
        reason_id: Uuid,
        placed_by_person_id: Uuid,
        expires_at: Option<DateTime<Utc>>,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn bulk_release_holds(
        &self,
        hold_ids: Vec<Uuid>,
        release_reason_id: Uuid,
        released_by_person_id: Uuid,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn override_holds_for_transaction(
        &self,
        account_id: Uuid,
        transaction_amount: Decimal,
        override_priority: HoldPriority,
        authorized_by_person_id: Uuid,
        override_reason_id: Uuid,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn reorder_hold_priorities(
        &self,
        account_id: Uuid,
        hold_priority_map: Vec<(Uuid, HoldPriority)>,
        authorized_by_person_id: Uuid,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn get_required_authorization_level(
        &self,
        hold_type: HoldType,
        amount: Decimal,
    ) -> BankingResult<HoldAuthorizationLevel> {
        unimplemented!()
    }

    async fn sync_judicial_holds(
        &self,
        court_reference: String,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn update_loan_pledge_holds(
        &self,
        loan_account_id: Uuid,
        collateral_account_ids: Vec<Uuid>,
        new_pledge_amount: Decimal,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn process_compliance_holds(
        &self,
        compliance_alert_id: Uuid,
        affected_accounts: Vec<Uuid>,
        hold_amount_per_account: Decimal,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn get_hold_analytics(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
        hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<HoldAnalytics> {
        unimplemented!()
    }

    async fn get_high_hold_ratio_accounts(
        &self,
        minimum_ratio: Decimal,
        exclude_hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<Vec<HighHoldAccount>> {
        unimplemented!()
    }

    async fn generate_judicial_hold_report(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> BankingResult<JudicialHoldReport> {
        unimplemented!()
    }
}
use std::sync::Arc;

use async_trait::async_trait;
use banking_api::{
    domain::{
        AccountHold, AccountHoldExpiryJob, AccountHoldReleaseRequest, AccountHoldSummary,
        HoldPriority, HoldStatus, HoldType, PlaceHoldRequest,
    },

    service::{account_hold_service::AccountHoldService, HighHoldAccount, HoldAnalytics, HoldAuthorizationLevel, JudicialHoldReport},
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

use crate::mappers::account_hold_mapper::AccountHoldMapper;

#[derive(Clone)]
pub struct AccountHoldServiceImpl {
    #[allow(dead_code)]
    account_repo: Arc<dyn AccountRepository>,
}

impl AccountHoldServiceImpl {
    pub fn new(account_repo: Arc<dyn AccountRepository>) -> Self {
        Self { account_repo }
    }
}

#[async_trait]
impl AccountHoldService for AccountHoldServiceImpl {
    async fn get_active_holds(&self, _account_id: Uuid) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn release_hold(&self, _hold_id: Uuid, _released_by_person_id: Uuid) -> BankingResult<()> {
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
        _release_request: AccountHoldReleaseRequest,
    ) -> BankingResult<AccountHold> {
        unimplemented!()
    }

    async fn modify_hold(
        &self,
        _hold_id: Uuid,
        _new_amount: Option<Decimal>,
        _new_expiry: Option<DateTime<Utc>>,
        _new_reason_id: Option<Uuid>,
        _modified_by_person_id: Uuid,
    ) -> BankingResult<AccountHold> {
        unimplemented!()
    }

    async fn cancel_hold(
        &self,
        _hold_id: Uuid,
        _cancellation_reason_id: Uuid,
        _cancelled_by_person_id: Uuid,
    ) -> BankingResult<AccountHold> {
        unimplemented!()
    }

    async fn get_hold_amounts_by_priority(
        &self,
        _account_id: Uuid,
    ) -> BankingResult<Vec<AccountHoldSummary>> {
        unimplemented!()
    }

    async fn get_active_holds_with_types(
        &self,
        _account_id: Uuid,
        _hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn get_hold_by_id(
        &self,
        _hold_id: Uuid,
    ) -> BankingResult<Option<AccountHold>> {
        unimplemented!()
    }

    async fn get_holds_by_status(
        &self,
        _account_id: Option<Uuid>,
        _status: HoldStatus,
        _from_date: Option<NaiveDate>,
        _to_date: Option<NaiveDate>,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn get_holds_by_type(
        &self,
        _hold_type: HoldType,
        _status: Option<HoldStatus>,
        _account_ids: Option<Vec<Uuid>>,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn get_hold_history(
        &self,
        _account_id: Uuid,
        _from_date: Option<NaiveDate>,
        _to_date: Option<NaiveDate>,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn process_expired_holds(
        &self,
        _processing_date: NaiveDate,
        _hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<AccountHoldExpiryJob> {
        unimplemented!()
    }

    async fn process_automatic_releases(
        &self,
        _processing_date: NaiveDate,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn bulk_place_holds(
        &self,
        _account_ids: Vec<Uuid>,
        _hold_type: HoldType,
        _amount_per_account: Decimal,
        _reason_id: Uuid,
        _placed_by_person_id: Uuid,
        _expires_at: Option<DateTime<Utc>>,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn bulk_release_holds(
        &self,
        _hold_ids: Vec<Uuid>,
        _release_reason_id: Uuid,
        _released_by_person_id: Uuid,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn override_holds_for_transaction(
        &self,
        _account_id: Uuid,
        _transaction_amount: Decimal,
        _override_priority: HoldPriority,
        _authorized_by_person_id: Uuid,
        _override_reason_id: Uuid,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn reorder_hold_priorities(
        &self,
        _account_id: Uuid,
        _hold_priority_map: Vec<(Uuid, HoldPriority)>,
        _authorized_by_person_id: Uuid,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn get_required_authorization_level(
        &self,
        _hold_type: HoldType,
        _amount: Decimal,
    ) -> BankingResult<HoldAuthorizationLevel> {
        unimplemented!()
    }

    async fn sync_judicial_holds(
        &self,
        _court_reference: String,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn update_loan_pledge_holds(
        &self,
        _loan_account_id: Uuid,
        _collateral_account_ids: Vec<Uuid>,
        _new_pledge_amount: Decimal,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn process_compliance_holds(
        &self,
        _compliance_alert_id: Uuid,
        _affected_accounts: Vec<Uuid>,
        _hold_amount_per_account: Decimal,
    ) -> BankingResult<Vec<AccountHold>> {
        unimplemented!()
    }

    async fn get_hold_analytics(
        &self,
        _from_date: NaiveDate,
        _to_date: NaiveDate,
        _hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<HoldAnalytics> {
        unimplemented!()
    }

    async fn get_high_hold_ratio_accounts(
        &self,
        _minimum_ratio: Decimal,
        _exclude_hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<Vec<HighHoldAccount>> {
        unimplemented!()
    }

    async fn generate_judicial_hold_report(
        &self,
        _from_date: NaiveDate,
        _to_date: NaiveDate,
    ) -> BankingResult<JudicialHoldReport> {
        unimplemented!()
    }
}
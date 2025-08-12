use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    domain::{
        AccountHold, AccountHoldExpiryJob, AccountHoldReleaseRequest, AccountHoldSummary,
        HoldPriority, HoldStatus, HoldType, PlaceHoldRequest,
    },
    BankingResult,
};

use super::{HighHoldAccount, HoldAnalytics, HoldAuthorizationLevel, JudicialHoldReport};

#[async_trait]
pub trait AccountHoldService: Send + Sync {
    async fn get_active_holds(&self, account_id: Uuid) -> BankingResult<Vec<AccountHold>>;
    async fn release_hold(&self, hold_id: Uuid, released_by_person_id: Uuid) -> BankingResult<()>;
    async fn place_hold(&self, request: PlaceHoldRequest) -> BankingResult<AccountHold>;
    async fn release_hold_with_request(
        &self,
        release_request: AccountHoldReleaseRequest,
    ) -> BankingResult<AccountHold>;
    async fn modify_hold(
        &self,
        hold_id: Uuid,
        new_amount: Option<Decimal>,
        new_expiry: Option<DateTime<Utc>>,
        new_reason_id: Option<Uuid>,
        modified_by_person_id: Uuid,
    ) -> BankingResult<AccountHold>;
    async fn cancel_hold(
        &self,
        hold_id: Uuid,
        cancellation_reason_id: Uuid,
        cancelled_by_person_id: Uuid,
    ) -> BankingResult<AccountHold>;
    async fn get_hold_amounts_by_priority(
        &self,
        account_id: Uuid,
    ) -> BankingResult<Vec<AccountHoldSummary>>;
    async fn get_active_holds_with_types(
        &self,
        account_id: Uuid,
        hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<Vec<AccountHold>>;
    async fn get_hold_by_id(&self, hold_id: Uuid) -> BankingResult<Option<AccountHold>>;
    async fn get_holds_by_status(
        &self,
        account_id: Option<Uuid>,
        status: HoldStatus,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> BankingResult<Vec<AccountHold>>;
    async fn get_holds_by_type(
        &self,
        hold_type: HoldType,
        status: Option<HoldStatus>,
        account_ids: Option<Vec<Uuid>>,
    ) -> BankingResult<Vec<AccountHold>>;
    async fn get_hold_history(
        &self,
        account_id: Uuid,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> BankingResult<Vec<AccountHold>>;
    async fn process_expired_holds(
        &self,
        processing_date: NaiveDate,
        hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<AccountHoldExpiryJob>;
    async fn process_automatic_releases(
        &self,
        processing_date: NaiveDate,
    ) -> BankingResult<Vec<AccountHold>>;
    async fn bulk_place_holds(
        &self,
        account_ids: Vec<Uuid>,
        hold_type: HoldType,
        amount_per_account: Decimal,
        reason_id: Uuid,
        placed_by_person_id: Uuid,
        expires_at: Option<DateTime<Utc>>,
    ) -> BankingResult<Vec<AccountHold>>;
    async fn bulk_release_holds(
        &self,
        hold_ids: Vec<Uuid>,
        release_reason_id: Uuid,
        released_by_person_id: Uuid,
    ) -> BankingResult<Vec<AccountHold>>;
    async fn override_holds_for_transaction(
        &self,
        account_id: Uuid,
        transaction_amount: Decimal,
        override_priority: HoldPriority,
        authorized_by_person_id: Uuid,
        override_reason_id: Uuid,
    ) -> BankingResult<Vec<AccountHold>>;
    async fn reorder_hold_priorities(
        &self,
        account_id: Uuid,
        hold_priority_map: Vec<(Uuid, HoldPriority)>,
        authorized_by_person_id: Uuid,
    ) -> BankingResult<Vec<AccountHold>>;
    async fn get_required_authorization_level(
        &self,
        hold_type: HoldType,
        amount: Decimal,
    ) -> BankingResult<HoldAuthorizationLevel>;
    async fn sync_judicial_holds(&self, court_reference: String) -> BankingResult<Vec<AccountHold>>;
    async fn update_loan_pledge_holds(
        &self,
        loan_account_id: Uuid,
        collateral_account_ids: Vec<Uuid>,
        new_pledge_amount: Decimal,
    ) -> BankingResult<Vec<AccountHold>>;
    async fn process_compliance_holds(
        &self,
        compliance_alert_id: Uuid,
        affected_accounts: Vec<Uuid>,
        hold_amount_per_account: Decimal,
    ) -> BankingResult<Vec<AccountHold>>;
    async fn get_hold_analytics(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
        hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<HoldAnalytics>;
    async fn get_high_hold_ratio_accounts(
        &self,
        minimum_ratio: Decimal,
        exclude_hold_types: Option<Vec<HoldType>>,
    ) -> BankingResult<Vec<HighHoldAccount>>;
    async fn generate_judicial_hold_report(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> BankingResult<JudicialHoldReport>;
}
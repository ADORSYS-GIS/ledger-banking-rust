use banking_api::domain::{
    AccountHold, AccountHoldExpiryJob, AccountHoldReleaseRequest,
    AccountHoldSummary, PlaceHoldRequest,
};
use banking_db::models::{
    AccountHoldExpiryJobModel, AccountHoldModel,
    AccountHoldReleaseRequestModel, AccountHoldSummaryModel,
    PlaceHoldRequestModel,
};

pub struct AccountHoldMapper;

impl AccountHoldMapper {
    // Account Hold mappers
    pub fn account_hold_to_model(hold: AccountHold) -> AccountHoldModel {
        AccountHoldModel {
            id: hold.id,
            account_id: hold.account_id,
            amount: hold.amount,
            hold_type: hold.hold_type,
            reason_id: hold.reason_id,
            additional_details: hold.additional_details,
            placed_by_person_id: hold.placed_by_person_id,
            placed_at: hold.placed_at,
            expires_at: hold.expires_at,
            status: hold.status,
            released_at: hold.released_at,
            released_by_person_id: hold.released_by_person_id,
            priority: hold.priority,
            source_reference: hold.source_reference,
            automatic_release: hold.automatic_release,
            created_at: chrono::Utc::now(), // Database audit field
            updated_at: chrono::Utc::now(), // Database audit field
        }
    }

    pub fn account_hold_from_model(model: AccountHoldModel) -> AccountHold {
        AccountHold {
            id: model.id,
            account_id: model.account_id,
            amount: model.amount,
            hold_type: model.hold_type,
            reason_id: model.reason_id,
            additional_details: model.additional_details.map(|s| s.as_str().try_into().unwrap()),
            placed_by_person_id: model.placed_by_person_id,
            placed_at: model.placed_at,
            expires_at: model.expires_at,
            status: model.status,
            released_at: model.released_at,
            released_by_person_id: model.released_by_person_id,
            priority: model.priority,
            source_reference: model.source_reference.map(|s| s.as_str().try_into().unwrap()),
            automatic_release: model.automatic_release,
        }
    }

    // AccountHoldSummary mappers
    pub fn hold_summary_to_model(summary: AccountHoldSummary) -> AccountHoldSummaryModel {
        AccountHoldSummaryModel {
            id: summary.id,
            account_balance_calculation_id: summary.account_balance_calculation_id,
            hold_type: summary.hold_type,
            total_amount: summary.total_amount,
            hold_count: summary.hold_count,
            priority: summary.priority,
        }
    }

    pub fn hold_summary_from_model(model: AccountHoldSummaryModel) -> AccountHoldSummary {
        AccountHoldSummary {
            id: model.id,
            account_balance_calculation_id: model.account_balance_calculation_id,
            hold_type: model.hold_type,
            total_amount: model.total_amount,
            hold_count: model.hold_count,
            priority: model.priority,
        }
    }

    // AccountHoldReleaseRequest mappers
    pub fn hold_release_request_to_model(
        request: AccountHoldReleaseRequest,
    ) -> AccountHoldReleaseRequestModel {
        AccountHoldReleaseRequestModel {
            id: request.id,
            hold_id: request.hold_id,
            release_amount: request.release_amount,
            release_reason_id: request.release_reason_id,
            release_additional_details: request.release_additional_details,
            released_by_person_id: request.released_by_person_id,
            override_authorization: request.override_authorization,
        }
    }

    pub fn hold_release_request_from_model(
        model: AccountHoldReleaseRequestModel,
    ) -> AccountHoldReleaseRequest {
        AccountHoldReleaseRequest {
            id: model.id,
            hold_id: model.hold_id,
            release_amount: model.release_amount,
            release_reason_id: model.release_reason_id,
            release_additional_details: model.release_additional_details.map(|s| s.as_str().try_into().unwrap()),
            released_by_person_id: model.released_by_person_id,
            override_authorization: model.override_authorization,
        }
    }

    // AccountHoldExpiryJob mappers
    pub fn hold_expiry_job_to_model(job: AccountHoldExpiryJob) -> AccountHoldExpiryJobModel {
        AccountHoldExpiryJobModel {
            id: job.id,
            processing_date: job.processing_date,
            expired_holds_count: job.expired_holds_count,
            total_released_amount: job.total_released_amount,
            processed_at: job.processed_at,
            errors: job.errors.iter().map(|s| s.as_str().try_into().unwrap()).collect(),
        }
    }

    pub fn hold_expiry_job_from_model(model: AccountHoldExpiryJobModel) -> AccountHoldExpiryJob {
        AccountHoldExpiryJob {
            id: model.id,
            processing_date: model.processing_date,
            expired_holds_count: model.expired_holds_count,
            total_released_amount: model.total_released_amount,
            processed_at: model.processed_at,
            errors: model.errors.into_iter().map(|s| s.as_str().try_into().unwrap()).collect(),
        }
    }

    // PlaceHoldRequest mappers
    pub fn place_hold_request_to_model(
        request: PlaceHoldRequest,
        id: uuid::Uuid,
    ) -> PlaceHoldRequestModel {
        PlaceHoldRequestModel {
            id,
            account_id: request.account_id,
            hold_type: request.hold_type,
            amount: request.amount,
            reason_id: request.reason_id,
            additional_details: request.additional_details,
            placed_by_person_id: request.placed_by_person_id,
            expires_at: request.expires_at,
            priority: request.priority,
            source_reference: request.source_reference,
        }
    }

    pub fn place_hold_request_from_model(model: PlaceHoldRequestModel) -> PlaceHoldRequest {
        PlaceHoldRequest {
            account_id: model.account_id,
            hold_type: model.hold_type,
            amount: model.amount,
            reason_id: model.reason_id,
            additional_details: model.additional_details.map(|s| s.as_str().try_into().unwrap()),
            placed_by_person_id: model.placed_by_person_id,
            expires_at: model.expires_at,
            priority: model.priority,
            source_reference: model.source_reference.map(|s| s.as_str().try_into().unwrap()),
        }
    }
}
use banking_api::domain::fee::{
    FeeApplication, FeeWaiver, FeeProcessingJob, FeeCategory
};
use banking_db::models::fee::{
    FeeApplicationModel, FeeWaiverModel, FeeProcessingJobModel
};

pub struct FeeMapper;

impl FeeMapper {
    /// Map from domain FeeApplication to database FeeApplicationModel
    pub fn fee_application_to_model(fee: FeeApplication) -> FeeApplicationModel {
        FeeApplicationModel {
            fee_application_id: fee.fee_application_id,
            account_id: fee.account_id,
            transaction_id: fee.transaction_id,
            fee_type: fee.fee_type,
            fee_category: fee.fee_category,
            product_code: fee.product_code,
            fee_code: fee.fee_code,
            description: fee.description,
            amount: fee.amount,
            currency: fee.currency,
            calculation_method: fee.calculation_method,
            calculation_base_amount: fee.calculation_base_amount,
            fee_rate: fee.fee_rate,
            trigger_event: fee.trigger_event,
            status: fee.status,
            applied_at: fee.applied_at,
            value_date: fee.value_date,
            reversal_deadline: fee.reversal_deadline,
            waived: fee.waived,
            waived_by: fee.waived_by,
            waived_reason_id: fee.waived_reason_id,
            applied_by: fee.applied_by,
            created_at: fee.created_at,
        }
    }

    /// Map from database FeeApplicationModel to domain FeeApplication
    pub fn fee_application_from_model(model: FeeApplicationModel) -> banking_api::BankingResult<FeeApplication> {
        Ok(FeeApplication {
            fee_application_id: model.fee_application_id,
            account_id: model.account_id,
            transaction_id: model.transaction_id,
            fee_type: model.fee_type,
            fee_category: model.fee_category,
            product_code: model.product_code,
            fee_code: model.fee_code,
            description: model.description,
            amount: model.amount,
            currency: model.currency,
            calculation_method: model.calculation_method,
            calculation_base_amount: model.calculation_base_amount,
            fee_rate: model.fee_rate,
            trigger_event: model.trigger_event,
            status: model.status,
            applied_at: model.applied_at,
            value_date: model.value_date,
            reversal_deadline: model.reversal_deadline,
            waived: model.waived,
            waived_by: model.waived_by,
            waived_reason_id: model.waived_reason_id,
            applied_by: model.applied_by,
            created_at: model.created_at,
        })
    }

    /// Map from domain FeeWaiver to database FeeWaiverModel
    pub fn fee_waiver_to_model(waiver: FeeWaiver) -> FeeWaiverModel {
        FeeWaiverModel {
            waiver_id: waiver.waiver_id,
            fee_application_id: waiver.fee_application_id,
            account_id: waiver.account_id,
            waived_amount: waiver.waived_amount,
            reason_id: waiver.reason_id,
            additional_details: waiver.additional_details,
            waived_by: waiver.waived_by,
            waived_at: waiver.waived_at,
            approval_required: waiver.approval_required,
            approved_by: waiver.approved_by,
            approved_at: waiver.approved_at,
        }
    }

    /// Map from database FeeWaiverModel to domain FeeWaiver
    pub fn fee_waiver_from_model(model: FeeWaiverModel) -> FeeWaiver {
        FeeWaiver {
            waiver_id: model.waiver_id,
            fee_application_id: model.fee_application_id,
            account_id: model.account_id,
            waived_amount: model.waived_amount,
            reason_id: model.reason_id,
            additional_details: model.additional_details,
            waived_by: model.waived_by,
            waived_at: model.waived_at,
            approval_required: model.approval_required,
            approved_by: model.approved_by,
            approved_at: model.approved_at,
        }
    }

    /// Map from domain FeeProcessingJob to database FeeProcessingJobModel
    pub fn fee_processing_job_to_model(job: FeeProcessingJob) -> FeeProcessingJobModel {
        FeeProcessingJobModel {
            job_id: job.job_id,
            job_type: job.job_type,
            job_name: job.job_name,
            schedule_expression: job.schedule_expression,
            target_fee_categories: serde_json::to_string(&job.target_fee_categories).unwrap_or_default(),
            target_products: job.target_products.map(|p| serde_json::to_string(&p).unwrap_or_default()),
            processing_date: job.processing_date,
            status: job.status,
            started_at: job.started_at,
            completed_at: job.completed_at,
            accounts_processed: job.accounts_processed as i32,
            fees_applied: job.fees_applied as i32,
            total_amount: job.total_amount,
            errors: if job.errors.is_empty() { None } else { Some(serde_json::to_string(&job.errors).unwrap_or_default()) },
            created_at: chrono::Utc::now(),
        }
    }

    /// Map from database FeeProcessingJobModel to domain FeeProcessingJob
    pub fn fee_processing_job_from_model(model: FeeProcessingJobModel) -> banking_api::BankingResult<FeeProcessingJob> {
        let target_fee_categories: Vec<FeeCategory> = serde_json::from_str(&model.target_fee_categories).unwrap_or_default();
        let target_products: Option<Vec<String>> = model.target_products
            .map(|p| serde_json::from_str(&p).unwrap_or_default());
        let errors: Vec<String> = model.errors
            .map(|e| serde_json::from_str(&e).unwrap_or_default())
            .unwrap_or_default();

        Ok(FeeProcessingJob {
            job_id: model.job_id,
            job_type: model.job_type,
            job_name: model.job_name,
            schedule_expression: model.schedule_expression,
            target_fee_categories,
            target_products,
            processing_date: model.processing_date,
            status: model.status,
            started_at: model.started_at,
            completed_at: model.completed_at,
            accounts_processed: model.accounts_processed as u32,
            fees_applied: model.fees_applied as u32,
            total_amount: model.total_amount,
            errors,
        })
    }
}
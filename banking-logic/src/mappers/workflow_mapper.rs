use banking_api::domain::{
    AccountWorkflow, WorkflowType, WorkflowStep, WorkflowStatus, WorkflowStepRecord,
    AccountOpeningRequest, ClosureRequest, ClosureReason, FinalSettlement,
    DormancyAssessment, DocumentReference
};
use banking_db::models::{
    AccountWorkflowModel, WorkflowTypeModel, WorkflowStepModel, WorkflowStatusModel,
    WorkflowStepRecordModel, AccountOpeningRequestModel, ClosureRequestModel,
    ClosureReasonModel, WorkflowFinalSettlementModel, DormancyAssessmentModel,
    DocumentReferenceModel
};

pub struct WorkflowMapper;

impl WorkflowMapper {
    /// Map from domain AccountWorkflow to database AccountWorkflowModel
    pub fn to_model(workflow: AccountWorkflow) -> AccountWorkflowModel {
        AccountWorkflowModel {
            workflow_id: workflow.workflow_id,
            account_id: workflow.account_id,
            workflow_type: Self::workflow_type_to_db(workflow.workflow_type),
            current_step: Self::workflow_step_to_db(workflow.current_step),
            status: Self::workflow_status_to_db(workflow.status),
            initiated_by: workflow.initiated_by,
            initiated_at: workflow.initiated_at,
            completed_at: workflow.completed_at,
            next_action_required: workflow.next_action_required,
            timeout_at: workflow.timeout_at,
            created_at: workflow.initiated_at, // Use initiated_at as created_at
            last_updated_at: workflow.initiated_at, // Will be updated in DB
        }
    }

    /// Map from database AccountWorkflowModel to domain AccountWorkflow
    pub fn from_model(model: AccountWorkflowModel) -> banking_api::BankingResult<AccountWorkflow> {
        Ok(AccountWorkflow {
            workflow_id: model.workflow_id,
            account_id: model.account_id,
            workflow_type: Self::db_to_workflow_type(model.workflow_type),
            current_step: Self::db_to_workflow_step(model.current_step),
            status: Self::db_to_workflow_status(model.status),
            initiated_by: model.initiated_by,
            initiated_at: model.initiated_at,
            completed_at: model.completed_at,
            steps_completed: Vec::new(), // Will be populated separately
            next_action_required: model.next_action_required,
            timeout_at: model.timeout_at,
        })
    }

    /// Map from domain WorkflowStepRecord to database WorkflowStepRecordModel
    pub fn step_record_to_model(step_record: WorkflowStepRecord) -> WorkflowStepRecordModel {
        WorkflowStepRecordModel {
            step: Self::workflow_step_to_db(step_record.step),
            completed_at: step_record.completed_at,
            completed_by: step_record.completed_by,
            notes: step_record.notes,
            supporting_documents: step_record.supporting_documents,
        }
    }

    /// Map from database WorkflowStepRecordModel to domain WorkflowStepRecord
    pub fn step_record_from_model(model: WorkflowStepRecordModel) -> WorkflowStepRecord {
        WorkflowStepRecord {
            step: Self::db_to_workflow_step(model.step),
            completed_at: model.completed_at,
            completed_by: model.completed_by,
            notes: model.notes,
            supporting_documents: model.supporting_documents,
        }
    }

    /// Map from domain AccountOpeningRequest to database AccountOpeningRequestModel
    pub fn opening_request_to_model(request: AccountOpeningRequest) -> AccountOpeningRequestModel {
        AccountOpeningRequestModel {
            customer_id: request.customer_id,
            product_code: request.product_code,
            initial_deposit: request.initial_deposit,
            channel: request.channel,
            initiated_by: request.initiated_by,
            supporting_documents: request.supporting_documents
                .into_iter()
                .map(Self::document_reference_to_model)
                .collect(),
        }
    }

    /// Map from database AccountOpeningRequestModel to domain AccountOpeningRequest
    pub fn opening_request_from_model(model: AccountOpeningRequestModel) -> AccountOpeningRequest {
        AccountOpeningRequest {
            customer_id: model.customer_id,
            product_code: model.product_code,
            initial_deposit: model.initial_deposit,
            channel: model.channel,
            initiated_by: model.initiated_by,
            supporting_documents: model.supporting_documents
                .into_iter()
                .map(Self::document_reference_from_model)
                .collect(),
        }
    }

    /// Map from domain ClosureRequest to database ClosureRequestModel
    pub fn closure_request_to_model(request: ClosureRequest) -> ClosureRequestModel {
        ClosureRequestModel {
            reason: Self::closure_reason_to_db(request.reason),
            requested_by: request.requested_by,
            force_closure: request.force_closure,
        }
    }

    /// Map from database ClosureRequestModel to domain ClosureRequest
    pub fn closure_request_from_model(model: ClosureRequestModel) -> ClosureRequest {
        ClosureRequest {
            reason: Self::db_to_closure_reason(model.reason),
            disbursement_instructions: Default::default(), // Will be set separately
            requested_by: model.requested_by,
            force_closure: model.force_closure,
        }
    }

    /// Map from domain FinalSettlement to database WorkflowFinalSettlementModel
    pub fn final_settlement_to_model(settlement: FinalSettlement) -> WorkflowFinalSettlementModel {
        WorkflowFinalSettlementModel {
            current_balance: settlement.current_balance,
            accrued_interest: settlement.accrued_interest,
            pending_fees: settlement.pending_fees,
            closure_fees: settlement.closure_fees,
            final_amount: settlement.final_amount,
            requires_disbursement: settlement.requires_disbursement,
        }
    }

    /// Map from database WorkflowFinalSettlementModel to domain FinalSettlement
    pub fn final_settlement_from_model(model: WorkflowFinalSettlementModel) -> FinalSettlement {
        FinalSettlement {
            current_balance: model.current_balance,
            accrued_interest: model.accrued_interest,
            pending_fees: model.pending_fees,
            closure_fees: model.closure_fees,
            final_amount: model.final_amount,
            requires_disbursement: model.requires_disbursement,
        }
    }

    /// Map from domain DormancyAssessment to database DormancyAssessmentModel
    pub fn dormancy_assessment_to_model(assessment: DormancyAssessment) -> DormancyAssessmentModel {
        DormancyAssessmentModel {
            is_eligible: assessment.is_eligible,
            last_activity_date: assessment.last_activity_date,
            days_inactive: assessment.days_inactive,
            threshold_days: assessment.threshold_days,
            product_specific_rules: assessment.product_specific_rules,
        }
    }

    /// Map from database DormancyAssessmentModel to domain DormancyAssessment
    pub fn dormancy_assessment_from_model(model: DormancyAssessmentModel) -> DormancyAssessment {
        DormancyAssessment {
            is_eligible: model.is_eligible,
            last_activity_date: model.last_activity_date,
            days_inactive: model.days_inactive,
            threshold_days: model.threshold_days,
            product_specific_rules: model.product_specific_rules,
        }
    }

    /// Map from domain DocumentReference to database DocumentReferenceModel
    pub fn document_reference_to_model(doc_ref: DocumentReference) -> DocumentReferenceModel {
        DocumentReferenceModel {
            document_id: doc_ref.document_id,
            document_type: doc_ref.document_type,
            document_path: doc_ref.document_path,
        }
    }

    /// Map from database DocumentReferenceModel to domain DocumentReference
    pub fn document_reference_from_model(model: DocumentReferenceModel) -> DocumentReference {
        DocumentReference {
            document_id: model.document_id,
            document_type: model.document_type,
            document_path: model.document_path,
        }
    }

    // Domain to Database enum conversions
    fn workflow_type_to_db(workflow_type: WorkflowType) -> WorkflowTypeModel {
        match workflow_type {
            WorkflowType::AccountOpening => WorkflowTypeModel::AccountOpening,
            WorkflowType::AccountClosure => WorkflowTypeModel::AccountClosure,
            WorkflowType::AccountReactivation => WorkflowTypeModel::KycUpdate,
            WorkflowType::ComplianceVerification => WorkflowTypeModel::ComplianceCheck,
            WorkflowType::MultiPartyApproval => WorkflowTypeModel::TransactionApproval,
        }
    }

    fn workflow_step_to_db(step: WorkflowStep) -> WorkflowStepModel {
        match step {
            WorkflowStep::InitiateRequest => WorkflowStepModel::InitiateRequest,
            WorkflowStep::ComplianceCheck => WorkflowStepModel::ComplianceCheck,
            WorkflowStep::DocumentVerification => WorkflowStepModel::DocumentVerification,
            WorkflowStep::ApprovalRequired => WorkflowStepModel::ApprovalRequired,
            WorkflowStep::FinalSettlement => WorkflowStepModel::FinalSettlement,
            WorkflowStep::Completed => WorkflowStepModel::Completed,
        }
    }

    fn workflow_status_to_db(status: WorkflowStatus) -> WorkflowStatusModel {
        match status {
            WorkflowStatus::InProgress => WorkflowStatusModel::InProgress,
            WorkflowStatus::PendingAction => WorkflowStatusModel::PendingAction,
            WorkflowStatus::Completed => WorkflowStatusModel::Completed,
            WorkflowStatus::Failed => WorkflowStatusModel::Failed,
            WorkflowStatus::Cancelled => WorkflowStatusModel::Cancelled,
            WorkflowStatus::TimedOut => WorkflowStatusModel::TimedOut,
        }
    }

    fn closure_reason_to_db(reason: ClosureReason) -> ClosureReasonModel {
        match reason {
            ClosureReason::CustomerRequest => ClosureReasonModel::CustomerRequest,
            ClosureReason::Regulatory => ClosureReasonModel::Regulatory,
            ClosureReason::Compliance => ClosureReasonModel::Compliance,
            ClosureReason::Dormancy => ClosureReasonModel::Dormancy,
            ClosureReason::SystemMaintenance => ClosureReasonModel::SystemMaintenance,
        }
    }

    // Database to Domain enum conversions
    fn db_to_workflow_type(workflow_type: WorkflowTypeModel) -> WorkflowType {
        match workflow_type {
            WorkflowTypeModel::AccountOpening => WorkflowType::AccountOpening,
            WorkflowTypeModel::AccountClosure => WorkflowType::AccountClosure,
            WorkflowTypeModel::LoanApplication => WorkflowType::AccountOpening, // Map to closest domain equivalent
            WorkflowTypeModel::LoanDisbursement => WorkflowType::AccountOpening,
            WorkflowTypeModel::TransactionApproval => WorkflowType::MultiPartyApproval,
            WorkflowTypeModel::ComplianceCheck => WorkflowType::ComplianceVerification,
            WorkflowTypeModel::KycUpdate => WorkflowType::AccountReactivation,
            WorkflowTypeModel::DocumentVerification => WorkflowType::ComplianceVerification,
            WorkflowTypeModel::CreditDecision => WorkflowType::MultiPartyApproval,
            WorkflowTypeModel::CollateralValuation => WorkflowType::ComplianceVerification,
            WorkflowTypeModel::InterestRateChange => WorkflowType::MultiPartyApproval,
            WorkflowTypeModel::FeeWaiver => WorkflowType::MultiPartyApproval,
            WorkflowTypeModel::LimitChange => WorkflowType::MultiPartyApproval,
            WorkflowTypeModel::StatusChange => WorkflowType::MultiPartyApproval,
            WorkflowTypeModel::ManualIntervention => WorkflowType::MultiPartyApproval,
        }
    }

    fn db_to_workflow_step(step: WorkflowStepModel) -> WorkflowStep {
        match step {
            WorkflowStepModel::InitiateRequest => WorkflowStep::InitiateRequest,
            WorkflowStepModel::ComplianceCheck => WorkflowStep::ComplianceCheck,
            WorkflowStepModel::DocumentVerification => WorkflowStep::DocumentVerification,
            WorkflowStepModel::ApprovalRequired => WorkflowStep::ApprovalRequired,
            WorkflowStepModel::FinalSettlement => WorkflowStep::FinalSettlement,
            WorkflowStepModel::Completed => WorkflowStep::Completed,
        }
    }

    fn db_to_workflow_status(status: WorkflowStatusModel) -> WorkflowStatus {
        match status {
            WorkflowStatusModel::InProgress => WorkflowStatus::InProgress,
            WorkflowStatusModel::PendingAction => WorkflowStatus::PendingAction,
            WorkflowStatusModel::Completed => WorkflowStatus::Completed,
            WorkflowStatusModel::Failed => WorkflowStatus::Failed,
            WorkflowStatusModel::Cancelled => WorkflowStatus::Cancelled,
            WorkflowStatusModel::TimedOut => WorkflowStatus::TimedOut,
        }
    }

    fn db_to_closure_reason(reason: ClosureReasonModel) -> ClosureReason {
        match reason {
            ClosureReasonModel::CustomerRequest => ClosureReason::CustomerRequest,
            ClosureReasonModel::Regulatory => ClosureReason::Regulatory,
            ClosureReasonModel::Compliance => ClosureReason::Compliance,
            ClosureReasonModel::Dormancy => ClosureReason::Dormancy,
            ClosureReasonModel::SystemMaintenance => ClosureReason::SystemMaintenance,
        }
    }
}
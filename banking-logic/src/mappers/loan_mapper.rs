use banking_api::domain::{
    AmortizationSchedule, AmortizationEntry, InstallmentStatus, LoanDelinquency,
    DelinquencyStage, CollectionAction, CollectionActionType, ActionStatus,
    LoanPayment, PaymentType, PaymentMethod, PaymentAllocation, PrepaymentHandling,
    PrepaymentType, PaymentStatus, PaymentReversal, LoanRestructuring,
    RestructuringType, LoanApprovalStatus, LoanDelinquencyJob,
    ProcessingJobStatus
};
use banking_db::models::{
    AmortizationSchedule as DbAmortizationSchedule, AmortizationEntry as DbAmortizationEntry,
    InstallmentStatus as DbInstallmentStatus, LoanDelinquency as DbLoanDelinquency,
    DelinquencyStage as DbDelinquencyStage, CollectionAction as DbCollectionAction,
    CollectionActionType as DbCollectionActionType, ActionStatus as DbActionStatus,
    LoanPayment as DbLoanPayment, PaymentType as DbPaymentType, PaymentMethod as DbPaymentMethod,
    PaymentAllocation as DbPaymentAllocation, PrepaymentHandling as DbPrepaymentHandling,
    PrepaymentType as DbPrepaymentType, PaymentStatus as DbPaymentStatus,
    PaymentReversal as DbPaymentReversal, LoanRestructuring as DbLoanRestructuring,
    RestructuringType as DbRestructuringType, LoanApprovalStatus as DbLoanApprovalStatus,
    LoanDelinquencyJob as DbLoanDelinquencyJob,
    ProcessingJobStatus as DbProcessingJobStatus
};

pub struct LoanMapper;

impl LoanMapper {
    /// Map from domain AmortizationSchedule to database model
    pub fn amortization_schedule_to_model(schedule: AmortizationSchedule) -> DbAmortizationSchedule {
        DbAmortizationSchedule {
            id: schedule.id,
            loan_account_id: schedule.loan_account_id,
            original_principal: schedule.original_principal,
            interest_rate: schedule.interest_rate,
            term_months: schedule.term_months,
            installment_amount: schedule.installment_amount,
            first_payment_date: schedule.first_payment_date,
            maturity_date: schedule.maturity_date,
            total_interest: schedule.total_interest,
            total_payments: schedule.total_payments,
            schedule_entries: schedule.schedule_entries.into_iter().map(Self::amortization_entry_to_model).collect(),
            created_at: schedule.created_at,
            last_updated_at: schedule.last_updated_at,
        }
    }

    /// Map from database AmortizationSchedule to domain model
    pub fn amortization_schedule_from_model(model: DbAmortizationSchedule) -> AmortizationSchedule {
        AmortizationSchedule {
            id: model.id,
            loan_account_id: model.loan_account_id,
            original_principal: model.original_principal,
            interest_rate: model.interest_rate,
            term_months: model.term_months,
            installment_amount: model.installment_amount,
            first_payment_date: model.first_payment_date,
            maturity_date: model.maturity_date,
            total_interest: model.total_interest,
            total_payments: model.total_payments,
            schedule_entries: model.schedule_entries.into_iter().map(Self::amortization_entry_from_model).collect(),
            created_at: model.created_at,
            last_updated_at: model.last_updated_at,
        }
    }

    /// Map from domain AmortizationEntry to database model
    pub fn amortization_entry_to_model(entry: AmortizationEntry) -> DbAmortizationEntry {
        DbAmortizationEntry {
            id: entry.id,
            schedule_id: entry.schedule_id,
            installment_number: entry.installment_number,
            due_date: entry.due_date,
            opening_principal_balance: entry.opening_principal_balance,
            installment_amount: entry.installment_amount,
            principal_component: entry.principal_component,
            interest_component: entry.interest_component,
            closing_principal_balance: entry.closing_principal_balance,
            cumulative_principal_paid: entry.cumulative_principal_paid,
            cumulative_interest_paid: entry.cumulative_interest_paid,
            payment_status: Self::installment_status_to_db(entry.payment_status),
            paid_date: entry.paid_date,
            paid_amount: entry.paid_amount,
            days_overdue: entry.days_overdue,
        }
    }

    /// Map from database AmortizationEntry to domain model
    pub fn amortization_entry_from_model(model: DbAmortizationEntry) -> AmortizationEntry {
        AmortizationEntry {
            id: model.id,
            schedule_id: model.schedule_id,
            installment_number: model.installment_number,
            due_date: model.due_date,
            opening_principal_balance: model.opening_principal_balance,
            installment_amount: model.installment_amount,
            principal_component: model.principal_component,
            interest_component: model.interest_component,
            closing_principal_balance: model.closing_principal_balance,
            cumulative_principal_paid: model.cumulative_principal_paid,
            cumulative_interest_paid: model.cumulative_interest_paid,
            payment_status: Self::db_to_installment_status(model.payment_status),
            paid_date: model.paid_date,
            paid_amount: model.paid_amount,
            days_overdue: model.days_overdue,
        }
    }

    /// Map from domain LoanDelinquency to database model
    pub fn loan_delinquency_to_model(delinquency: LoanDelinquency) -> DbLoanDelinquency {
        DbLoanDelinquency {
            id: delinquency.id,
            loan_account_id: delinquency.loan_account_id,
            delinquency_start_date: delinquency.delinquency_start_date,
            current_dpd: delinquency.current_dpd,
            highest_dpd: delinquency.highest_dpd,
            delinquency_stage: Self::delinquency_stage_to_db(delinquency.delinquency_stage),
            overdue_principal: delinquency.overdue_principal,
            overdue_interest: delinquency.overdue_interest,
            penalty_interest_accrued: delinquency.penalty_interest_accrued,
            total_overdue_amount: delinquency.total_overdue_amount,
            last_payment_date: delinquency.last_payment_date,
            last_payment_amount: delinquency.last_payment_amount,
            collection_actions: delinquency.collection_actions.into_iter().map(Self::collection_action_to_model).collect(),
            restructuring_eligibility: delinquency.restructuring_eligibility,
            npl_date: delinquency.npl_date,
            provisioning_amount: delinquency.provisioning_amount,
            created_at: delinquency.created_at,
            last_updated_at: delinquency.last_updated_at,
        }
    }

    /// Map from database LoanDelinquency to domain model
    pub fn loan_delinquency_from_model(model: DbLoanDelinquency) -> LoanDelinquency {
        LoanDelinquency {
            id: model.id,
            loan_account_id: model.loan_account_id,
            delinquency_start_date: model.delinquency_start_date,
            current_dpd: model.current_dpd,
            highest_dpd: model.highest_dpd,
            delinquency_stage: Self::db_to_delinquency_stage(model.delinquency_stage),
            overdue_principal: model.overdue_principal,
            overdue_interest: model.overdue_interest,
            penalty_interest_accrued: model.penalty_interest_accrued,
            total_overdue_amount: model.total_overdue_amount,
            last_payment_date: model.last_payment_date,
            last_payment_amount: model.last_payment_amount,
            collection_actions: model.collection_actions.into_iter().map(Self::collection_action_from_model).collect(),
            restructuring_eligibility: model.restructuring_eligibility,
            npl_date: model.npl_date,
            provisioning_amount: model.provisioning_amount,
            created_at: model.created_at,
            last_updated_at: model.last_updated_at,
        }
    }

    /// Map from domain CollectionAction to database model
    pub fn collection_action_to_model(action: CollectionAction) -> DbCollectionAction {
        DbCollectionAction {
            id: action.id,
            delinquency_id: action.delinquency_id,
            loan_account_id: action.loan_account_id,
            action_type: Self::collection_action_type_to_db(action.action_type),
            action_date: action.action_date,
            due_date: action.due_date,
            description: action.description,
            amount_demanded: action.amount_demanded,
            response_received: action.response_received,
            response_date: action.response_date,
            response_details: action.response_details,
            follow_up_required: action.follow_up_required,
            follow_up_date: action.follow_up_date,
            action_status: Self::action_status_to_db(action.action_status),
            assigned_to: action.assigned_to,
            created_by_person_id: action.created_by_person_id,
            created_at: action.created_at,
        }
    }

    /// Map from database CollectionAction to domain model
    pub fn collection_action_from_model(model: DbCollectionAction) -> CollectionAction {
        CollectionAction {
            id: model.id,
            delinquency_id: model.delinquency_id,
            loan_account_id: model.loan_account_id,
            action_type: Self::db_to_collection_action_type(model.action_type),
            action_date: model.action_date,
            due_date: model.due_date,
            description: model.description,
            amount_demanded: model.amount_demanded,
            response_received: model.response_received,
            response_date: model.response_date,
            response_details: model.response_details,
            follow_up_required: model.follow_up_required,
            follow_up_date: model.follow_up_date,
            action_status: Self::db_to_action_status(model.action_status),
            assigned_to: model.assigned_to,
            created_by_person_id: model.created_by_person_id,
            created_at: model.created_at,
        }
    }

    /// Map from domain LoanPayment to database model
    pub fn loan_payment_to_model(payment: LoanPayment) -> DbLoanPayment {
        DbLoanPayment {
            id: payment.id,
            loan_account_id: payment.loan_account_id,
            payment_date: payment.payment_date,
            payment_amount: payment.payment_amount,
            payment_type: Self::payment_type_to_db(payment.payment_type),
            payment_method: Self::payment_method_to_db(payment.payment_method),
            allocation: Self::payment_allocation_to_model(payment.allocation),
            payment_status: Self::payment_status_to_db(payment.payment_status),
            external_reference: payment.external_reference,
            processed_by: payment.processed_by,
            processed_at: payment.processed_at,
            reversal_info: payment.reversal_info.map(Self::payment_reversal_to_model),
        }
    }

    /// Map from database LoanPayment to domain model
    pub fn loan_payment_from_model(model: DbLoanPayment) -> LoanPayment {
        LoanPayment {
            id: model.id,
            loan_account_id: model.loan_account_id,
            payment_date: model.payment_date,
            payment_amount: model.payment_amount,
            payment_type: Self::db_to_payment_type(model.payment_type),
            payment_method: Self::db_to_payment_method(model.payment_method),
            allocation: Self::payment_allocation_from_model(model.allocation),
            payment_status: Self::db_to_payment_status(model.payment_status),
            external_reference: model.external_reference,
            processed_by: model.processed_by,
            processed_at: model.processed_at,
            reversal_info: model.reversal_info.map(Self::payment_reversal_from_model),
        }
    }

    /// Map from domain PaymentAllocation to database model
    pub fn payment_allocation_to_model(allocation: PaymentAllocation) -> DbPaymentAllocation {
        DbPaymentAllocation {
            id: allocation.id,
            payment_id: allocation.payment_id,
            penalty_interest_payment: allocation.penalty_interest_payment,
            overdue_interest_payment: allocation.overdue_interest_payment,
            current_interest_payment: allocation.current_interest_payment,
            principal_payment: allocation.principal_payment,
            fees_payment: allocation.fees_payment,
            charges_payment: allocation.charges_payment,
            excess_amount: allocation.excess_amount,
            prepayment_handling: allocation.prepayment_handling.map(Self::prepayment_handling_to_model),
        }
    }

    /// Map from database PaymentAllocation to domain model
    pub fn payment_allocation_from_model(model: DbPaymentAllocation) -> PaymentAllocation {
        PaymentAllocation {
            id: model.id,
            payment_id: model.payment_id,
            penalty_interest_payment: model.penalty_interest_payment,
            overdue_interest_payment: model.overdue_interest_payment,
            current_interest_payment: model.current_interest_payment,
            principal_payment: model.principal_payment,
            fees_payment: model.fees_payment,
            charges_payment: model.charges_payment,
            excess_amount: model.excess_amount,
            prepayment_handling: model.prepayment_handling.map(Self::prepayment_handling_from_model),
        }
    }

    /// Map from domain PrepaymentHandling to database model
    pub fn prepayment_handling_to_model(handling: PrepaymentHandling) -> DbPrepaymentHandling {
        DbPrepaymentHandling {
            handling_type: Self::prepayment_type_to_db(handling.handling_type),
            excess_amount: handling.excess_amount,
            new_outstanding_principal: handling.new_outstanding_principal,
            term_reduction_months: handling.term_reduction_months,
            new_installment_amount: handling.new_installment_amount,
            new_maturity_date: handling.new_maturity_date,
            schedule_regenerated: handling.schedule_regenerated,
            customer_choice: handling.customer_choice,
        }
    }

    /// Map from database PrepaymentHandling to domain model
    pub fn prepayment_handling_from_model(model: DbPrepaymentHandling) -> PrepaymentHandling {
        PrepaymentHandling {
            handling_type: Self::db_to_prepayment_type(model.handling_type),
            excess_amount: model.excess_amount,
            new_outstanding_principal: model.new_outstanding_principal,
            term_reduction_months: model.term_reduction_months,
            new_installment_amount: model.new_installment_amount,
            new_maturity_date: model.new_maturity_date,
            schedule_regenerated: model.schedule_regenerated,
            customer_choice: model.customer_choice,
        }
    }

    /// Map from domain PaymentReversal to database model
    pub fn payment_reversal_to_model(reversal: PaymentReversal) -> DbPaymentReversal {
        DbPaymentReversal {
            id: reversal.id,
            original_payment_id: reversal.original_payment_id,
            reversal_reason_id: reversal.reversal_reason_id,
            additional_details: reversal.additional_details,
            reversed_amount: reversal.reversed_amount,
            reversed_by: reversal.reversed_by,
            reversed_at: reversal.reversed_at,
            schedule_adjusted: reversal.schedule_adjusted,
        }
    }

    /// Map from database PaymentReversal to domain model
    pub fn payment_reversal_from_model(model: DbPaymentReversal) -> PaymentReversal {
        PaymentReversal {
            id: model.id,
            original_payment_id: model.original_payment_id,
            reversal_reason_id: model.reversal_reason_id,
            additional_details: model.additional_details,
            reversed_amount: model.reversed_amount,
            reversed_by: model.reversed_by,
            reversed_at: model.reversed_at,
            schedule_adjusted: model.schedule_adjusted,
        }
    }

    /// Map from domain LoanRestructuring to database model
    pub fn loan_restructuring_to_model(restructuring: LoanRestructuring) -> DbLoanRestructuring {
        DbLoanRestructuring {
            id: restructuring.id,
            loan_account_id: restructuring.loan_account_id,
            restructuring_type: Self::restructuring_type_to_db(restructuring.restructuring_type),
            request_date: restructuring.request_date,
            effective_date: restructuring.effective_date,
            restructuring_reason_id: restructuring.restructuring_reason_id,
            additional_details: restructuring.additional_details,
            original_principal: restructuring.original_principal,
            original_interest_rate: restructuring.original_interest_rate,
            original_term_months: restructuring.original_term_months,
            original_installment: restructuring.original_installment,
            new_principal: restructuring.new_principal,
            new_interest_rate: restructuring.new_interest_rate,
            new_term_months: restructuring.new_term_months,
            new_installment: restructuring.new_installment,
            new_maturity_date: restructuring.new_maturity_date,
            moratorium_period: restructuring.moratorium_period,
            capitalized_interest: restructuring.capitalized_interest,
            waived_penalty_amount: restructuring.waived_penalty_amount,
            approval_status: Self::loan_approval_status_to_db(restructuring.approval_status),
            approved_by: restructuring.approved_by,
            approved_at: restructuring.approved_at,
            conditions: restructuring.conditions,
            created_by_person_id: restructuring.created_by_person_id,
            created_at: restructuring.created_at,
        }
    }

    /// Map from database LoanRestructuring to domain model
    pub fn loan_restructuring_from_model(model: DbLoanRestructuring) -> LoanRestructuring {
        LoanRestructuring {
            id: model.id,
            loan_account_id: model.loan_account_id,
            restructuring_type: Self::db_to_restructuring_type(model.restructuring_type),
            request_date: model.request_date,
            effective_date: model.effective_date,
            restructuring_reason_id: model.restructuring_reason_id,
            additional_details: model.additional_details,
            original_principal: model.original_principal,
            original_interest_rate: model.original_interest_rate,
            original_term_months: model.original_term_months,
            original_installment: model.original_installment,
            new_principal: model.new_principal,
            new_interest_rate: model.new_interest_rate,
            new_term_months: model.new_term_months,
            new_installment: model.new_installment,
            new_maturity_date: model.new_maturity_date,
            moratorium_period: model.moratorium_period,
            capitalized_interest: model.capitalized_interest,
            waived_penalty_amount: model.waived_penalty_amount,
            approval_status: Self::db_to_loan_approval_status(model.approval_status),
            approved_by: model.approved_by,
            approved_at: model.approved_at,
            conditions: model.conditions,
            created_by_person_id: model.created_by_person_id,
            created_at: model.created_at,
        }
    }

    /// Map from domain LoanDelinquencyJob to database model
    pub fn loan_delinquency_job_to_model(job: LoanDelinquencyJob) -> DbLoanDelinquencyJob {
        DbLoanDelinquencyJob {
            id: job.id,
            processing_date: job.processing_date,
            loans_processed: job.loans_processed,
            new_delinquent_loans: job.new_delinquent_loans,
            recovered_loans: job.recovered_loans,
            npl_classifications: job.npl_classifications,
            total_penalty_interest: job.total_penalty_interest,
            collection_actions_triggered: job.collection_actions_triggered,
            notifications_sent: job.notifications_sent,
            status: Self::processing_job_status_to_db(job.status),
            started_at: job.started_at,
            completed_at: job.completed_at,
            errors: job.errors,
        }
    }

    /// Map from database LoanDelinquencyJob to domain model
    pub fn loan_delinquency_job_from_model(model: DbLoanDelinquencyJob) -> LoanDelinquencyJob {
        LoanDelinquencyJob {
            id: model.id,
            processing_date: model.processing_date,
            loans_processed: model.loans_processed,
            new_delinquent_loans: model.new_delinquent_loans,
            recovered_loans: model.recovered_loans,
            npl_classifications: model.npl_classifications,
            total_penalty_interest: model.total_penalty_interest,
            collection_actions_triggered: model.collection_actions_triggered,
            notifications_sent: model.notifications_sent,
            status: Self::processing_job_status_from_db(model.status),
            started_at: model.started_at,
            completed_at: model.completed_at,
            errors: model.errors,
        }
    }

    // Enum conversion helper methods
    pub fn installment_status_to_db(status: InstallmentStatus) -> DbInstallmentStatus {
        match status {
            InstallmentStatus::Scheduled => DbInstallmentStatus::Scheduled,
            InstallmentStatus::Due => DbInstallmentStatus::Due,
            InstallmentStatus::PartiallyPaid => DbInstallmentStatus::PartiallyPaid,
            InstallmentStatus::Paid => DbInstallmentStatus::Paid,
            InstallmentStatus::Overdue => DbInstallmentStatus::Overdue,
            InstallmentStatus::WriteOff => DbInstallmentStatus::WriteOff,
        }
    }

    fn db_to_installment_status(status: DbInstallmentStatus) -> InstallmentStatus {
        match status {
            DbInstallmentStatus::Scheduled => InstallmentStatus::Scheduled,
            DbInstallmentStatus::Due => InstallmentStatus::Due,
            DbInstallmentStatus::PartiallyPaid => InstallmentStatus::PartiallyPaid,
            DbInstallmentStatus::Paid => InstallmentStatus::Paid,
            DbInstallmentStatus::Overdue => InstallmentStatus::Overdue,
            DbInstallmentStatus::WriteOff => InstallmentStatus::WriteOff,
        }
    }

    fn delinquency_stage_to_db(stage: DelinquencyStage) -> DbDelinquencyStage {
        match stage {
            DelinquencyStage::Current => DbDelinquencyStage::Current,
            DelinquencyStage::Stage1 => DbDelinquencyStage::Stage1,
            DelinquencyStage::Stage2 => DbDelinquencyStage::Stage2,
            DelinquencyStage::Stage3 => DbDelinquencyStage::Stage3,
            DelinquencyStage::NonPerforming => DbDelinquencyStage::NonPerforming,
            DelinquencyStage::WriteOff => DbDelinquencyStage::WriteOff,
        }
    }

    fn db_to_delinquency_stage(stage: DbDelinquencyStage) -> DelinquencyStage {
        match stage {
            DbDelinquencyStage::Current => DelinquencyStage::Current,
            DbDelinquencyStage::Stage1 => DelinquencyStage::Stage1,
            DbDelinquencyStage::Stage2 => DelinquencyStage::Stage2,
            DbDelinquencyStage::Stage3 => DelinquencyStage::Stage3,
            DbDelinquencyStage::NonPerforming => DelinquencyStage::NonPerforming,
            DbDelinquencyStage::WriteOff => DelinquencyStage::WriteOff,
        }
    }

    fn collection_action_type_to_db(action_type: CollectionActionType) -> DbCollectionActionType {
        match action_type {
            CollectionActionType::EmailReminder => DbCollectionActionType::EmailReminder,
            CollectionActionType::SmsNotification => DbCollectionActionType::SmsNotification,
            CollectionActionType::PhoneCall => DbCollectionActionType::PhoneCall,
            CollectionActionType::LetterNotice => DbCollectionActionType::LetterNotice,
            CollectionActionType::LegalNotice => DbCollectionActionType::LegalNotice,
            CollectionActionType::FieldVisit => DbCollectionActionType::FieldVisit,
            CollectionActionType::PaymentPlan => DbCollectionActionType::PaymentPlan,
            CollectionActionType::Restructuring => DbCollectionActionType::Restructuring,
            CollectionActionType::LegalAction => DbCollectionActionType::LegalAction,
            CollectionActionType::AssetRecovery => DbCollectionActionType::AssetRecovery,
        }
    }

    fn db_to_collection_action_type(action_type: DbCollectionActionType) -> CollectionActionType {
        match action_type {
            DbCollectionActionType::EmailReminder => CollectionActionType::EmailReminder,
            DbCollectionActionType::SmsNotification => CollectionActionType::SmsNotification,
            DbCollectionActionType::PhoneCall => CollectionActionType::PhoneCall,
            DbCollectionActionType::LetterNotice => CollectionActionType::LetterNotice,
            DbCollectionActionType::LegalNotice => CollectionActionType::LegalNotice,
            DbCollectionActionType::FieldVisit => CollectionActionType::FieldVisit,
            DbCollectionActionType::PaymentPlan => CollectionActionType::PaymentPlan,
            DbCollectionActionType::Restructuring => CollectionActionType::Restructuring,
            DbCollectionActionType::LegalAction => CollectionActionType::LegalAction,
            DbCollectionActionType::AssetRecovery => CollectionActionType::AssetRecovery,
        }
    }

    fn action_status_to_db(status: ActionStatus) -> DbActionStatus {
        match status {
            ActionStatus::Planned => DbActionStatus::Planned,
            ActionStatus::InProgress => DbActionStatus::InProgress,
            ActionStatus::Completed => DbActionStatus::Completed,
            ActionStatus::Failed => DbActionStatus::Failed,
            ActionStatus::Cancelled => DbActionStatus::Cancelled,
        }
    }

    fn db_to_action_status(status: DbActionStatus) -> ActionStatus {
        match status {
            DbActionStatus::Planned => ActionStatus::Planned,
            DbActionStatus::InProgress => ActionStatus::InProgress,
            DbActionStatus::Completed => ActionStatus::Completed,
            DbActionStatus::Failed => ActionStatus::Failed,
            DbActionStatus::Cancelled => ActionStatus::Cancelled,
        }
    }

    fn payment_type_to_db(payment_type: PaymentType) -> DbPaymentType {
        match payment_type {
            PaymentType::Regular => DbPaymentType::Regular,
            PaymentType::Prepayment => DbPaymentType::Prepayment,
            PaymentType::PartialPayment => DbPaymentType::PartialPayment,
            PaymentType::Restructured => DbPaymentType::Restructured,
            PaymentType::Settlement => DbPaymentType::Settlement,
            PaymentType::Recovery => DbPaymentType::Recovery,
        }
    }

    fn db_to_payment_type(payment_type: DbPaymentType) -> PaymentType {
        match payment_type {
            DbPaymentType::Regular => PaymentType::Regular,
            DbPaymentType::Prepayment => PaymentType::Prepayment,
            DbPaymentType::PartialPayment => PaymentType::PartialPayment,
            DbPaymentType::Restructured => PaymentType::Restructured,
            DbPaymentType::Settlement => PaymentType::Settlement,
            DbPaymentType::Recovery => PaymentType::Recovery,
        }
    }

    fn payment_method_to_db(method: PaymentMethod) -> DbPaymentMethod {
        match method {
            PaymentMethod::BankTransfer => DbPaymentMethod::BankTransfer,
            PaymentMethod::DirectDebit => DbPaymentMethod::DirectDebit,
            PaymentMethod::Check => DbPaymentMethod::Check,
            PaymentMethod::Cash => DbPaymentMethod::Cash,
            PaymentMethod::OnlinePayment => DbPaymentMethod::OnlinePayment,
            PaymentMethod::MobilePayment => DbPaymentMethod::MobilePayment,
            PaymentMethod::StandingInstruction => DbPaymentMethod::StandingInstruction,
        }
    }

    fn db_to_payment_method(method: DbPaymentMethod) -> PaymentMethod {
        match method {
            DbPaymentMethod::BankTransfer => PaymentMethod::BankTransfer,
            DbPaymentMethod::DirectDebit => PaymentMethod::DirectDebit,
            DbPaymentMethod::Check => PaymentMethod::Check,
            DbPaymentMethod::Cash => PaymentMethod::Cash,
            DbPaymentMethod::OnlinePayment => PaymentMethod::OnlinePayment,
            DbPaymentMethod::MobilePayment => PaymentMethod::MobilePayment,
            DbPaymentMethod::StandingInstruction => PaymentMethod::StandingInstruction,
        }
    }

    fn prepayment_type_to_db(prepayment_type: PrepaymentType) -> DbPrepaymentType {
        match prepayment_type {
            PrepaymentType::TermReduction => DbPrepaymentType::TermReduction,
            PrepaymentType::InstallmentReduction => DbPrepaymentType::InstallmentReduction,
            PrepaymentType::HoldInSuspense => DbPrepaymentType::HoldInSuspense,
            PrepaymentType::Refund => DbPrepaymentType::Refund,
        }
    }

    fn db_to_prepayment_type(prepayment_type: DbPrepaymentType) -> PrepaymentType {
        match prepayment_type {
            DbPrepaymentType::TermReduction => PrepaymentType::TermReduction,
            DbPrepaymentType::InstallmentReduction => PrepaymentType::InstallmentReduction,
            DbPrepaymentType::HoldInSuspense => PrepaymentType::HoldInSuspense,
            DbPrepaymentType::Refund => PrepaymentType::Refund,
        }
    }

    fn payment_status_to_db(status: PaymentStatus) -> DbPaymentStatus {
        match status {
            PaymentStatus::Processed => DbPaymentStatus::Processed,
            PaymentStatus::Pending => DbPaymentStatus::Pending,
            PaymentStatus::Failed => DbPaymentStatus::Failed,
            PaymentStatus::Reversed => DbPaymentStatus::Reversed,
            PaymentStatus::PartiallyAllocated => DbPaymentStatus::PartiallyAllocated,
        }
    }

    fn db_to_payment_status(status: DbPaymentStatus) -> PaymentStatus {
        match status {
            DbPaymentStatus::Processed => PaymentStatus::Processed,
            DbPaymentStatus::Pending => PaymentStatus::Pending,
            DbPaymentStatus::Failed => PaymentStatus::Failed,
            DbPaymentStatus::Reversed => PaymentStatus::Reversed,
            DbPaymentStatus::PartiallyAllocated => PaymentStatus::PartiallyAllocated,
        }
    }

    fn restructuring_type_to_db(restructuring_type: RestructuringType) -> DbRestructuringType {
        match restructuring_type {
            RestructuringType::PaymentHoliday => DbRestructuringType::PaymentHoliday,
            RestructuringType::TermExtension => DbRestructuringType::TermExtension,
            RestructuringType::RateReduction => DbRestructuringType::RateReduction,
            RestructuringType::PrincipalReduction => DbRestructuringType::PrincipalReduction,
            RestructuringType::InstallmentReduction => DbRestructuringType::InstallmentReduction,
            RestructuringType::InterestCapitalization => DbRestructuringType::InterestCapitalization,
            RestructuringType::FullRestructuring => DbRestructuringType::FullRestructuring,
        }
    }

    fn db_to_restructuring_type(restructuring_type: DbRestructuringType) -> RestructuringType {
        match restructuring_type {
            DbRestructuringType::PaymentHoliday => RestructuringType::PaymentHoliday,
            DbRestructuringType::TermExtension => RestructuringType::TermExtension,
            DbRestructuringType::RateReduction => RestructuringType::RateReduction,
            DbRestructuringType::PrincipalReduction => RestructuringType::PrincipalReduction,
            DbRestructuringType::InstallmentReduction => RestructuringType::InstallmentReduction,
            DbRestructuringType::InterestCapitalization => RestructuringType::InterestCapitalization,
            DbRestructuringType::FullRestructuring => RestructuringType::FullRestructuring,
        }
    }

    fn loan_approval_status_to_db(status: LoanApprovalStatus) -> DbLoanApprovalStatus {
        match status {
            LoanApprovalStatus::Pending => DbLoanApprovalStatus::Pending,
            LoanApprovalStatus::Approved => DbLoanApprovalStatus::Approved,
            LoanApprovalStatus::Rejected => DbLoanApprovalStatus::Rejected,
            LoanApprovalStatus::ConditionallyApproved => DbLoanApprovalStatus::ConditionallyApproved,
            LoanApprovalStatus::RequiresCommitteeApproval => DbLoanApprovalStatus::RequiresCommitteeApproval,
        }
    }

    fn db_to_loan_approval_status(status: DbLoanApprovalStatus) -> LoanApprovalStatus {
        match status {
            DbLoanApprovalStatus::Pending => LoanApprovalStatus::Pending,
            DbLoanApprovalStatus::Approved => LoanApprovalStatus::Approved,
            DbLoanApprovalStatus::Rejected => LoanApprovalStatus::Rejected,
            DbLoanApprovalStatus::ConditionallyApproved => LoanApprovalStatus::ConditionallyApproved,
            DbLoanApprovalStatus::RequiresCommitteeApproval => LoanApprovalStatus::RequiresCommitteeApproval,
        }
    }

    fn processing_job_status_to_db(status: ProcessingJobStatus) -> DbProcessingJobStatus {
        match status {
            ProcessingJobStatus::Scheduled => DbProcessingJobStatus::Scheduled,
            ProcessingJobStatus::Running => DbProcessingJobStatus::Running,
            ProcessingJobStatus::Completed => DbProcessingJobStatus::Completed,
            ProcessingJobStatus::Failed => DbProcessingJobStatus::Failed,
            ProcessingJobStatus::PartiallyCompleted => DbProcessingJobStatus::PartiallyCompleted,
        }
    }

    fn processing_job_status_from_db(status: DbProcessingJobStatus) -> ProcessingJobStatus {
        match status {
            DbProcessingJobStatus::Scheduled => ProcessingJobStatus::Scheduled,
            DbProcessingJobStatus::Running => ProcessingJobStatus::Running,
            DbProcessingJobStatus::Completed => ProcessingJobStatus::Completed,
            DbProcessingJobStatus::Failed => ProcessingJobStatus::Failed,
            DbProcessingJobStatus::PartiallyCompleted => ProcessingJobStatus::PartiallyCompleted,
        }
    }
}
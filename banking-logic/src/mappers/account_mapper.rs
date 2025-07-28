use banking_api::domain::{
    Account, AccountOwnership, AccountRelationship, AccountMandate, UltimateBeneficiary,
    AccountHold, DisbursementInstructions, StatusChangeRecord
};
use banking_db::models::{
    AccountModel, AccountOwnershipModel, AccountRelationshipModel, AccountMandateModel,
    UltimateBeneficiaryModel, AccountHoldModel, DisbursementInstructionsModel,
    AccountStatusHistoryModel
};

pub struct AccountMapper;

impl AccountMapper {
    /// Map from domain Account to database AccountModel
    pub fn to_model(account: Account) -> AccountModel {
        AccountModel {
            account_id: account.account_id,
            product_code: account.product_code,
            account_type: account.account_type,
            account_status: account.account_status,
            signing_condition: account.signing_condition,
            currency: account.currency,
            open_date: account.open_date,
            domicile_branch_id: account.domicile_branch_id,
            current_balance: account.current_balance,
            available_balance: account.available_balance,
            accrued_interest: account.accrued_interest,
            overdraft_limit: account.overdraft_limit,
            original_principal: account.original_principal,
            outstanding_principal: account.outstanding_principal,
            loan_interest_rate: account.loan_interest_rate,
            loan_term_months: account.loan_term_months,
            disbursement_date: account.disbursement_date,
            maturity_date: account.maturity_date,
            installment_amount: account.installment_amount,
            next_due_date: account.next_due_date,
            penalty_rate: account.penalty_rate,
            collateral_id: account.collateral_id,
            loan_purpose_id: account.loan_purpose_id,
            // Enhanced lifecycle fields
            close_date: account.close_date,
            last_activity_date: account.last_activity_date,
            dormancy_threshold_days: account.dormancy_threshold_days,
            reactivation_required: account.reactivation_required,
            pending_closure_reason_id: account.pending_closure_reason_id,
            disbursement_instructions: account.disbursement_instructions.map(Self::disbursement_instructions_to_model),
            status_changed_by: account.status_changed_by,
            status_change_reason_id: account.status_change_reason_id,
            status_change_timestamp: account.status_change_timestamp,
            // Audit fields
            created_at: account.created_at,
            last_updated_at: account.last_updated_at,
            updated_by: account.updated_by,
        }
    }

    /// Map from database AccountModel to domain Account
    pub fn from_model(model: AccountModel) -> banking_api::BankingResult<Account> {
        Ok(Account {
            account_id: model.account_id,
            product_code: model.product_code,
            account_type: model.account_type,
            account_status: model.account_status,
            signing_condition: model.signing_condition,
            currency: model.currency,
            open_date: model.open_date,
            domicile_branch_id: model.domicile_branch_id,
            current_balance: model.current_balance,
            available_balance: model.available_balance,
            accrued_interest: model.accrued_interest,
            overdraft_limit: model.overdraft_limit,
            original_principal: model.original_principal,
            outstanding_principal: model.outstanding_principal,
            loan_interest_rate: model.loan_interest_rate,
            loan_term_months: model.loan_term_months,
            disbursement_date: model.disbursement_date,
            maturity_date: model.maturity_date,
            installment_amount: model.installment_amount,
            next_due_date: model.next_due_date,
            penalty_rate: model.penalty_rate,
            collateral_id: model.collateral_id,
            loan_purpose_id: model.loan_purpose_id,
            // Enhanced lifecycle fields
            close_date: model.close_date,
            last_activity_date: model.last_activity_date,
            dormancy_threshold_days: model.dormancy_threshold_days,
            reactivation_required: model.reactivation_required,
            pending_closure_reason_id: model.pending_closure_reason_id,
            disbursement_instructions: model.disbursement_instructions.map(Self::disbursement_instructions_from_model),
            status_changed_by: model.status_changed_by,
            status_change_reason_id: model.status_change_reason_id,
            status_change_timestamp: model.status_change_timestamp,
            // Audit fields
            created_at: model.created_at,
            last_updated_at: model.last_updated_at,
            updated_by: model.updated_by,
        })
    }

    // Disbursement Instructions mappers
    pub fn disbursement_instructions_to_model(instructions: DisbursementInstructions) -> DisbursementInstructionsModel {
        DisbursementInstructionsModel {
            method: instructions.method,
            target_account: instructions.target_account,
            cash_pickup_branch_id: instructions.cash_pickup_branch_id,
            authorized_recipient: instructions.authorized_recipient,
        }
    }

    pub fn disbursement_instructions_from_model(model: DisbursementInstructionsModel) -> DisbursementInstructions {
        DisbursementInstructions {
            method: model.method,
            target_account: model.target_account,
            cash_pickup_branch_id: model.cash_pickup_branch_id,
            authorized_recipient: model.authorized_recipient,
        }
    }

    // Account Ownership mappers
    pub fn account_ownership_to_model(ownership: AccountOwnership) -> AccountOwnershipModel {
        AccountOwnershipModel {
            ownership_id: ownership.ownership_id,
            account_id: ownership.account_id,
            customer_id: ownership.customer_id,
            ownership_type: ownership.ownership_type,
            ownership_percentage: ownership.ownership_percentage,
            created_at: ownership.created_at,
        }
    }

    pub fn account_ownership_from_model(model: AccountOwnershipModel) -> AccountOwnership {
        AccountOwnership {
            ownership_id: model.ownership_id,
            account_id: model.account_id,
            customer_id: model.customer_id,
            ownership_type: model.ownership_type,
            ownership_percentage: model.ownership_percentage,
            created_at: model.created_at,
        }
    }

    // Account Relationship mappers
    pub fn account_relationship_to_model(relationship: AccountRelationship) -> AccountRelationshipModel {
        AccountRelationshipModel {
            relationship_id: relationship.relationship_id,
            account_id: relationship.account_id,
            entity_id: relationship.entity_id,
            entity_type: relationship.entity_type,
            relationship_type: relationship.relationship_type,
            status: relationship.status,
            start_date: relationship.start_date,
            end_date: relationship.end_date,
        }
    }

    pub fn account_relationship_from_model(model: AccountRelationshipModel) -> AccountRelationship {
        AccountRelationship {
            relationship_id: model.relationship_id,
            account_id: model.account_id,
            entity_id: model.entity_id,
            entity_type: model.entity_type,
            relationship_type: model.relationship_type,
            status: model.status,
            start_date: model.start_date,
            end_date: model.end_date,
        }
    }

    // Account Mandate mappers
    pub fn account_mandate_to_model(mandate: AccountMandate) -> AccountMandateModel {
        AccountMandateModel {
            mandate_id: mandate.mandate_id,
            account_id: mandate.account_id,
            grantee_customer_id: mandate.grantee_customer_id,
            permission_type: mandate.permission_type,
            transaction_limit: mandate.transaction_limit,
            approval_group_id: mandate.approval_group_id,
            status: mandate.status,
            start_date: mandate.start_date,
            end_date: mandate.end_date,
        }
    }

    pub fn account_mandate_from_model(model: AccountMandateModel) -> AccountMandate {
        AccountMandate {
            mandate_id: model.mandate_id,
            account_id: model.account_id,
            grantee_customer_id: model.grantee_customer_id,
            permission_type: model.permission_type,
            transaction_limit: model.transaction_limit,
            approval_group_id: model.approval_group_id,
            status: model.status,
            start_date: model.start_date,
            end_date: model.end_date,
        }
    }

    // Ultimate Beneficial Owner mappers
    pub fn ultimate_beneficiary_to_model(ubo: UltimateBeneficiary) -> UltimateBeneficiaryModel {
        UltimateBeneficiaryModel {
            ubo_link_id: ubo.ubo_link_id,
            corporate_customer_id: ubo.corporate_customer_id,
            beneficiary_customer_id: ubo.beneficiary_customer_id,
            ownership_percentage: ubo.ownership_percentage,
            control_type: ubo.control_type,
            description: ubo.description,
            status: ubo.status,
            verification_status: ubo.verification_status,
            created_at: ubo.created_at,
        }
    }

    pub fn ultimate_beneficiary_from_model(model: UltimateBeneficiaryModel) -> UltimateBeneficiary {
        UltimateBeneficiary {
            ubo_link_id: model.ubo_link_id,
            corporate_customer_id: model.corporate_customer_id,
            beneficiary_customer_id: model.beneficiary_customer_id,
            ownership_percentage: model.ownership_percentage,
            control_type: model.control_type,
            description: model.description,
            status: model.status,
            verification_status: model.verification_status,
            created_at: model.created_at,
        }
    }

    // Account Hold mappers
    pub fn account_hold_to_model(hold: AccountHold) -> AccountHoldModel {
        AccountHoldModel {
            hold_id: hold.hold_id,
            account_id: hold.account_id,
            amount: hold.amount,
            hold_type: hold.hold_type,
            reason_id: hold.reason_id,
            additional_details: hold.additional_details,
            placed_by: hold.placed_by,
            placed_at: hold.placed_at,
            expires_at: hold.expires_at,
            status: hold.status,
            released_at: hold.released_at,
            released_by: hold.released_by,
            priority: hold.priority,
            source_reference: hold.source_reference,
            automatic_release: hold.automatic_release,
        }
    }

    pub fn account_hold_from_model(model: AccountHoldModel) -> AccountHold {
        AccountHold {
            hold_id: model.hold_id,
            account_id: model.account_id,
            amount: model.amount,
            hold_type: model.hold_type,
            reason_id: model.reason_id,
            additional_details: model.additional_details,
            placed_by: model.placed_by,
            placed_at: model.placed_at,
            expires_at: model.expires_at,
            status: model.status,
            released_at: model.released_at,
            released_by: model.released_by,
            priority: model.priority,
            source_reference: model.source_reference,
            automatic_release: model.automatic_release,
        }
    }

    // Status Change Record mappers
    pub fn status_change_record_to_model(record: StatusChangeRecord) -> AccountStatusHistoryModel {
        AccountStatusHistoryModel {
            history_id: record.change_id,
            account_id: record.account_id,
            old_status: record.old_status,
            new_status: record.new_status,
            change_reason_id: record.reason_id,
            additional_context: record.additional_context,
            changed_by: record.changed_by,
            changed_at: record.changed_at,
            system_triggered: record.system_triggered,
            created_at: record.changed_at, // Use changed_at as created_at
        }
    }

    pub fn status_change_record_from_model(model: AccountStatusHistoryModel) -> StatusChangeRecord {
        StatusChangeRecord {
            change_id: model.history_id,
            account_id: model.account_id,
            old_status: model.old_status,
            new_status: model.new_status,
            reason_id: model.change_reason_id,
            additional_context: model.additional_context,
            changed_by: model.changed_by,
            changed_at: model.changed_at,
            system_triggered: model.system_triggered,
        }
    }
}
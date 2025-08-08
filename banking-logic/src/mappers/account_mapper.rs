use banking_api::domain::{
    Account, AccountOwnership, AccountRelationship, AccountMandate, UltimateBeneficiary,
    AccountHold, StatusChangeRecord
};
use heapless::{String as HeaplessString};
use banking_db::models::{
    AccountModel, AccountOwnershipModel, AccountRelationshipModel, AccountMandateModel,
    UltimateBeneficiaryModel, AccountHoldModel, AccountStatusHistoryModel
};

pub struct AccountMapper;

impl AccountMapper {
    /// Map from domain Account to database AccountModel
    pub fn to_model(account: Account) -> AccountModel {
        AccountModel {
            id: account.id,
            product_code: account.product_code,
            account_type: account.account_type,
            account_status: account.account_status,
            signing_condition: account.signing_condition,
            currency: account.currency,
            open_date: account.open_date,
            domicile_agency_branch_id: account.domicile_agency_branch_id,
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
            last_disbursement_instruction_id: account.last_disbursement_instruction_id,
            status_changed_by_person_id: account.status_changed_by_person_id,
            status_change_reason_id: account.status_change_reason_id,
            status_change_timestamp: account.status_change_timestamp,
            // Direct reference fields
            most_significant_account_hold_id: account.most_significant_account_hold_id,
            account_ownership_id: account.account_ownership_id,
            access01_account_relationship_id: account.access01_account_relationship_id,
            access02_account_relationship_id: account.access02_account_relationship_id,
            access03_account_relationship_id: account.access03_account_relationship_id,
            access04_account_relationship_id: account.access04_account_relationship_id,
            access05_account_relationship_id: account.access05_account_relationship_id,
            access06_account_relationship_id: account.access06_account_relationship_id,
            access07_account_relationship_id: account.access07_account_relationship_id,
            access11_account_mandate_id: account.access11_account_mandate_id,
            access12_account_mandate_id: account.access12_account_mandate_id,
            access13_account_mandate_id: account.access13_account_mandate_id,
            access14_account_mandate_id: account.access14_account_mandate_id,
            access15_account_mandate_id: account.access15_account_mandate_id,
            access16_account_mandate_id: account.access16_account_mandate_id,
            access17_account_mandate_id: account.access17_account_mandate_id,
            interest01_ultimate_beneficiary_id: account.interest01_ultimate_beneficiary_id,
            interest02_ultimate_beneficiary_id: account.interest02_ultimate_beneficiary_id,
            interest03_ultimate_beneficiary_id: account.interest03_ultimate_beneficiary_id,
            interest04_ultimate_beneficiary_id: account.interest04_ultimate_beneficiary_id,
            interest05_ultimate_beneficiary_id: account.interest05_ultimate_beneficiary_id,
            interest06_ultimate_beneficiary_id: account.interest06_ultimate_beneficiary_id,
            interest07_ultimate_beneficiary_id: account.interest07_ultimate_beneficiary_id,
            // Audit fields
            created_at: account.created_at,
            last_updated_at: account.last_updated_at,
            updated_by_person_id: account.updated_by_person_id,
        }
    }

    /// Map from database AccountModel to domain Account
    pub fn from_model(model: AccountModel) -> banking_api::BankingResult<Account> {
        Ok(Account {
            id: model.id,
            product_code: model.product_code,
            account_type: model.account_type,
            account_status: model.account_status,
            signing_condition: model.signing_condition,
            currency: model.currency,
            open_date: model.open_date,
            domicile_agency_branch_id: model.domicile_agency_branch_id,
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
            last_disbursement_instruction_id: model.last_disbursement_instruction_id,
            status_changed_by_person_id: model.status_changed_by_person_id,
            status_change_reason_id: model.status_change_reason_id,
            status_change_timestamp: model.status_change_timestamp,
            // Direct reference fields
            most_significant_account_hold_id: model.most_significant_account_hold_id,
            account_ownership_id: model.account_ownership_id,
            access01_account_relationship_id: model.access01_account_relationship_id,
            access02_account_relationship_id: model.access02_account_relationship_id,
            access03_account_relationship_id: model.access03_account_relationship_id,
            access04_account_relationship_id: model.access04_account_relationship_id,
            access05_account_relationship_id: model.access05_account_relationship_id,
            access06_account_relationship_id: model.access06_account_relationship_id,
            access07_account_relationship_id: model.access07_account_relationship_id,
            access11_account_mandate_id: model.access11_account_mandate_id,
            access12_account_mandate_id: model.access12_account_mandate_id,
            access13_account_mandate_id: model.access13_account_mandate_id,
            access14_account_mandate_id: model.access14_account_mandate_id,
            access15_account_mandate_id: model.access15_account_mandate_id,
            access16_account_mandate_id: model.access16_account_mandate_id,
            access17_account_mandate_id: model.access17_account_mandate_id,
            interest01_ultimate_beneficiary_id: model.interest01_ultimate_beneficiary_id,
            interest02_ultimate_beneficiary_id: model.interest02_ultimate_beneficiary_id,
            interest03_ultimate_beneficiary_id: model.interest03_ultimate_beneficiary_id,
            interest04_ultimate_beneficiary_id: model.interest04_ultimate_beneficiary_id,
            interest05_ultimate_beneficiary_id: model.interest05_ultimate_beneficiary_id,
            interest06_ultimate_beneficiary_id: model.interest06_ultimate_beneficiary_id,
            interest07_ultimate_beneficiary_id: model.interest07_ultimate_beneficiary_id,
            // Audit fields
            created_at: model.created_at,
            last_updated_at: model.last_updated_at,
            updated_by_person_id: model.updated_by_person_id,
        })
    }

    // Note: Disbursement instructions are now handled via last_disbursement_instruction_id reference

    // Account Ownership mappers
    pub fn account_ownership_to_model(ownership: AccountOwnership) -> AccountOwnershipModel {
        AccountOwnershipModel {
            id: ownership.id,
            account_id: ownership.id,
            customer_id: ownership.customer_id,
            ownership_type: ownership.ownership_type,
            ownership_percentage: ownership.ownership_percentage,
            created_at: ownership.created_at,
        }
    }

    pub fn account_ownership_from_model(model: AccountOwnershipModel) -> AccountOwnership {
        AccountOwnership {
            id: model.id,
            account_id: model.id,
            customer_id: model.customer_id,
            ownership_type: model.ownership_type,
            ownership_percentage: model.ownership_percentage,
            created_at: model.created_at,
        }
    }

    // Account Relationship mappers
    pub fn account_relationship_to_model(relationship: AccountRelationship) -> AccountRelationshipModel {
        AccountRelationshipModel {
            id: relationship.id,
            account_id: relationship.id,
            person_id: relationship.person_id,
            entity_type: relationship.entity_type,
            relationship_type: relationship.relationship_type,
            status: relationship.status,
            start_date: relationship.start_date,
            end_date: relationship.end_date,
        }
    }

    pub fn account_relationship_from_model(model: AccountRelationshipModel) -> AccountRelationship {
        AccountRelationship {
            id: model.id,
            account_id: model.id,
            person_id: model.person_id,
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
            id: mandate.id,
            account_id: mandate.account_id,
            grantee_customer_id: mandate.grantee_customer_id,
            permission_type: mandate.permission_type,
            transaction_limit: mandate.transaction_limit,
            approver01_person_id: mandate.approver01_person_id,
            approver02_person_id: mandate.approver02_person_id,
            approver03_person_id: mandate.approver03_person_id,
            approver04_person_id: mandate.approver04_person_id,
            approver05_person_id: mandate.approver05_person_id,
            approver06_person_id: mandate.approver06_person_id,
            approver07_person_id: mandate.approver07_person_id,
            required_signers_count: mandate.required_signers_count,
            conditional_mandate_id: mandate.conditional_mandate_id,
            status: mandate.status,
            start_date: mandate.start_date,
            end_date: mandate.end_date,
        }
    }

    pub fn account_mandate_from_model(model: AccountMandateModel) -> AccountMandate {
        AccountMandate {
            id: model.id,
            account_id: model.account_id,
            grantee_customer_id: model.grantee_customer_id,
            permission_type: model.permission_type,
            transaction_limit: model.transaction_limit,
            approver01_person_id: model.approver01_person_id,
            approver02_person_id: model.approver02_person_id,
            approver03_person_id: model.approver03_person_id,
            approver04_person_id: model.approver04_person_id,
            approver05_person_id: model.approver05_person_id,
            approver06_person_id: model.approver06_person_id,
            approver07_person_id: model.approver07_person_id,
            required_signers_count: model.required_signers_count,
            conditional_mandate_id: model.conditional_mandate_id,
            status: model.status,
            start_date: model.start_date,
            end_date: model.end_date,
        }
    }

    // Ultimate Beneficial Owner mappers
    pub fn ultimate_beneficiary_to_model(ubo: UltimateBeneficiary) -> UltimateBeneficiaryModel {
        UltimateBeneficiaryModel {
            id: ubo.id,
            corporate_customer_id: ubo.corporate_customer_id,
            beneficiary_customer_id: ubo.beneficiary_customer_id,
            ownership_percentage: ubo.ownership_percentage,
            control_type: ubo.control_type,
            description: ubo.description.map(|desc| {
                HeaplessString::try_from(desc.as_str()).unwrap_or_else(|_| HeaplessString::new())
            }),
            status: ubo.status,
            verification_status: ubo.verification_status,
            created_at: ubo.created_at,
        }
    }

    pub fn ultimate_beneficiary_from_model(model: UltimateBeneficiaryModel) -> UltimateBeneficiary {
        UltimateBeneficiary {
            id: model.id,
            corporate_customer_id: model.corporate_customer_id,
            beneficiary_customer_id: model.beneficiary_customer_id,
            ownership_percentage: model.ownership_percentage,
            control_type: model.control_type,
            description: model.description.map(|desc| {
                HeaplessString::try_from(desc.as_str()).unwrap_or_else(|_| HeaplessString::new())
            }),
            status: model.status,
            verification_status: model.verification_status,
            created_at: model.created_at,
        }
    }

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
            additional_details: model.additional_details,
            placed_by_person_id: model.placed_by_person_id,
            placed_at: model.placed_at,
            expires_at: model.expires_at,
            status: model.status,
            released_at: model.released_at,
            released_by_person_id: model.released_by_person_id,
            priority: model.priority,
            source_reference: model.source_reference,
            automatic_release: model.automatic_release,
        }
    }

    // Status Change Record mappers
    pub fn status_change_record_to_model(record: StatusChangeRecord) -> AccountStatusHistoryModel {
        AccountStatusHistoryModel {
            id: record.id,
            account_id: record.id,
            old_status: record.old_status,
            new_status: record.new_status,
            change_reason_id: record.reason_id,
            additional_context: record.additional_context,
            changed_by_person_id: record.changed_by_person_id,
            changed_at: record.changed_at,
            system_triggered: record.system_triggered,
            created_at: record.changed_at, // Use changed_at as created_at
        }
    }

    pub fn status_change_record_from_model(model: AccountStatusHistoryModel) -> StatusChangeRecord {
        StatusChangeRecord {
            id: model.id,
            account_id: model.id,
            old_status: model.old_status,
            new_status: model.new_status,
            reason_id: model.change_reason_id,
            additional_context: model.additional_context,
            changed_by_person_id: model.changed_by_person_id,
            changed_at: model.changed_at,
            system_triggered: model.system_triggered,
        }
    }
}
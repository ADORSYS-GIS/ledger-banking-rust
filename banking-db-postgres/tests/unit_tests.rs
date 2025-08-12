use banking_db::{DbAccountType, DbAccountStatus, DbSigningCondition};
use banking_db::models::AccountModel;
use chrono::{NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use std::str::FromStr;
use uuid::Uuid;

/// Test helper to create a sample account model for unit testing
fn create_test_account() -> AccountModel {
    let account_id = Uuid::new_v4();
    let updated_by_person_id = Uuid::new_v4();
    let domicile_agency_branch_id = Uuid::new_v4();
    
    AccountModel {
        id: account_id,
        product_id: Uuid::new_v4(),
        gl_code_suffix: None,
        account_type: DbAccountType::Savings,
        account_status: DbAccountStatus::Active,
        signing_condition: DbSigningCondition::AnyOwner,
        currency: HeaplessString::try_from("USD").unwrap(),
        open_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        domicile_agency_branch_id: domicile_agency_branch_id,
        current_balance: Decimal::from_str("1000.00").unwrap(),
        available_balance: Decimal::from_str("950.00").unwrap(),
        accrued_interest: Decimal::from_str("12.50").unwrap(),
        overdraft_limit: None,
        original_principal: None,
        outstanding_principal: None,
        loan_interest_rate: None,
        loan_term_months: None,
        disbursement_date: None,
        maturity_date: None,
        installment_amount: None,
        next_due_date: None,
        penalty_rate: None,
        collateral_id: None,
        loan_purpose_id: None,
        close_date: None,
        last_activity_date: Some(NaiveDate::from_ymd_opt(2024, 1, 10).unwrap()),
        dormancy_threshold_days: Some(365),
        reactivation_required: false,
        pending_closure_reason_id: None,
        last_disbursement_instruction_id: None,
        status_changed_by_person_id: None,
        status_change_reason_id: None,
        status_change_timestamp: None,
        // Reference fields (new)
        most_significant_account_hold_id: None,
        account_ownership_id: None,
        access01_account_relationship_id: None,
        access02_account_relationship_id: None,
        access03_account_relationship_id: None,
        access04_account_relationship_id: None,
        access05_account_relationship_id: None,
        access06_account_relationship_id: None,
        access07_account_relationship_id: None,
        access11_account_mandate_id: None,
        access12_account_mandate_id: None,
        access13_account_mandate_id: None,
        access14_account_mandate_id: None,
        access15_account_mandate_id: None,
        access16_account_mandate_id: None,
        access17_account_mandate_id: None,
        interest01_ultimate_beneficiary_id: None,
        interest02_ultimate_beneficiary_id: None,
        interest03_ultimate_beneficiary_id: None,
        interest04_ultimate_beneficiary_id: None,
        interest05_ultimate_beneficiary_id: None,
        interest06_ultimate_beneficiary_id: None,
        interest07_ultimate_beneficiary_id: None,
        created_at: Utc::now(),
        last_updated_at: Utc::now(),
        updated_by_person_id: updated_by_person_id,
    }
}

#[test]
fn test_account_model_creation() {
    let account = create_test_account();
    
    assert!(matches!(account.account_type, DbAccountType::Savings));
    assert!(matches!(account.account_status, DbAccountStatus::Active));
    assert!(matches!(account.signing_condition, DbSigningCondition::AnyOwner));
    assert!(!account.product_id.is_nil());
    assert_eq!(account.currency.as_str(), "USD");
    assert_eq!(account.current_balance, Decimal::from_str("1000.00").unwrap());
    assert_eq!(account.available_balance, Decimal::from_str("950.00").unwrap());
    assert_eq!(account.accrued_interest, Decimal::from_str("12.50").unwrap());
}

#[test]
fn test_heapless_string_constraints() {
    // Test Uuid size
    let product_id = Uuid::new_v4();
    assert_eq!(std::mem::size_of_val(&product_id), 16);
    
    // Test currency code length constraint (HeaplessString<3>)
    let valid_currency = "USD";
    let currency_valid = HeaplessString::<3>::try_from(valid_currency);
    assert!(currency_valid.is_ok());
    
    let too_long_currency = "DOLLARS"; // 7 chars > 3
    let currency_invalid = HeaplessString::<3>::try_from(too_long_currency);
    assert!(currency_invalid.is_err());
}

#[test]
fn test_loan_account_specific_fields() {
    let mut loan_account = create_test_account();
    loan_account.account_type = DbAccountType::Loan;
    loan_account.original_principal = Some(Decimal::from_str("10000.00").unwrap());
    loan_account.outstanding_principal = Some(Decimal::from_str("7500.00").unwrap());
    loan_account.loan_interest_rate = Some(Decimal::from_str("0.12").unwrap()); // 12%
    loan_account.loan_term_months = Some(24);
    loan_account.disbursement_date = Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap());
    loan_account.maturity_date = Some(NaiveDate::from_ymd_opt(2026, 1, 15).unwrap());
    loan_account.installment_amount = Some(Decimal::from_str("469.70").unwrap());
    loan_account.next_due_date = Some(NaiveDate::from_ymd_opt(2024, 2, 15).unwrap());
    loan_account.penalty_rate = Some(Decimal::from_str("0.05").unwrap()); // 5%
    
    assert!(matches!(loan_account.account_type, DbAccountType::Loan));
    assert_eq!(loan_account.original_principal, Some(Decimal::from_str("10000.00").unwrap()));
    assert_eq!(loan_account.outstanding_principal, Some(Decimal::from_str("7500.00").unwrap()));
    assert_eq!(loan_account.loan_interest_rate, Some(Decimal::from_str("0.12").unwrap()));
    assert_eq!(loan_account.loan_term_months, Some(24));
}

#[test]
fn test_current_account_overdraft() {
    let mut current_account = create_test_account();
    current_account.account_type = DbAccountType::Current;
    current_account.overdraft_limit = Some(Decimal::from_str("500.00").unwrap());
    current_account.current_balance = Decimal::from_str("-100.00").unwrap(); // Negative balance
    current_account.available_balance = Decimal::from_str("400.00").unwrap(); // Available from overdraft
    
    assert!(matches!(current_account.account_type, DbAccountType::Current));
    assert_eq!(current_account.overdraft_limit, Some(Decimal::from_str("500.00").unwrap()));
    assert_eq!(current_account.current_balance, Decimal::from_str("-100.00").unwrap());
    assert_eq!(current_account.available_balance, Decimal::from_str("400.00").unwrap());
}

#[test]
fn test_account_status_transitions() {
    let mut account = create_test_account();
    
    // Test status changes
    account.account_status = DbAccountStatus::PendingApproval;
    assert!(matches!(account.account_status, DbAccountStatus::PendingApproval));
    
    account.account_status = DbAccountStatus::Active;
    assert!(matches!(account.account_status, DbAccountStatus::Active));
    
    account.account_status = DbAccountStatus::Frozen;
    assert!(matches!(account.account_status, DbAccountStatus::Frozen));
    
    account.account_status = DbAccountStatus::Dormant;
    assert!(matches!(account.account_status, DbAccountStatus::Dormant));
    
    account.account_status = DbAccountStatus::PendingClosure;
    assert!(matches!(account.account_status, DbAccountStatus::PendingClosure));
    
    account.account_status = DbAccountStatus::Closed;
    assert!(matches!(account.account_status, DbAccountStatus::Closed));
    
    account.account_status = DbAccountStatus::PendingReactivation;
    assert!(matches!(account.account_status, DbAccountStatus::PendingReactivation));
}

#[test]
fn test_signing_conditions() {
    let mut account = create_test_account();
    
    // Test different signing conditions
    account.signing_condition = DbSigningCondition::None;
    assert!(matches!(account.signing_condition, DbSigningCondition::None));
    
    account.signing_condition = DbSigningCondition::AnyOwner;
    assert!(matches!(account.signing_condition, DbSigningCondition::AnyOwner));
    
    account.signing_condition = DbSigningCondition::AllOwners;
    assert!(matches!(account.signing_condition, DbSigningCondition::AllOwners));
}

#[test]
fn test_decimal_precision() {
    // Test that Decimal maintains precision for financial calculations
    let balance = Decimal::from_str("1000.12345").unwrap();
    let interest = Decimal::from_str("0.05").unwrap();
    let result = balance * interest;
    
    // Should maintain precision - correct mathematical result
    assert_eq!(result.to_string(), "50.0061725");
}

#[test]
fn test_uuid_generation() {
    // Test that UUIDs are properly generated and unique
    let uuid1 = Uuid::new_v4();
    let uuid2 = Uuid::new_v4();
    
    assert_ne!(uuid1, uuid2);
    assert_eq!(uuid1.to_string().len(), 36); // Standard UUID string length
}

#[test]
fn test_date_handling() {
    // Test date creation and validation
    let valid_date = NaiveDate::from_ymd_opt(2024, 1, 15);
    assert!(valid_date.is_some());
    
    let invalid_date = NaiveDate::from_ymd_opt(2024, 13, 35); // Invalid month/day
    assert!(invalid_date.is_none());
}

#[test]
fn test_optional_fields() {
    let account = create_test_account();
    
    // Test that savings account doesn't have loan-specific fields
    assert!(matches!(account.account_type, DbAccountType::Savings));
    assert!(account.original_principal.is_none());
    assert!(account.outstanding_principal.is_none());
    assert!(account.loan_interest_rate.is_none());
    assert!(account.loan_term_months.is_none());
    assert!(account.disbursement_date.is_none());
    assert!(account.maturity_date.is_none());
    assert!(account.installment_amount.is_none());
    assert!(account.next_due_date.is_none());
    assert!(account.penalty_rate.is_none());
    assert!(account.collateral_id.is_none());
    
    // Test that it doesn't have overdraft (for savings)
    assert!(account.overdraft_limit.is_none());
}

#[test]
fn test_account_lifecycle_fields() {
    let mut account = create_test_account();
    
    // Test lifecycle management fields
    account.close_date = Some(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
    account.reactivation_required = true;
    account.dormancy_threshold_days = Some(180); // 6 months
    
    assert_eq!(account.close_date, Some(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()));
    assert_eq!(account.reactivation_required, true);
    assert_eq!(account.dormancy_threshold_days, Some(180));
}

#[test]
fn test_audit_fields() {
    let account = create_test_account();
    
    // Test that audit fields are properly set
    assert!(account.created_at <= Utc::now());
    assert!(account.last_updated_at <= Utc::now());
    assert_ne!(account.updated_by_person_id, Uuid::nil());
}

#[test]
fn test_enum_debug_output() {
    // Test that enums have proper debug output
    assert_eq!(format!("{:?}", DbAccountType::Savings), "Savings");
    assert_eq!(format!("{:?}", DbAccountType::Current), "Current");
    assert_eq!(format!("{:?}", DbAccountType::Loan), "Loan");
    
    assert_eq!(format!("{:?}", DbAccountStatus::Active), "Active");
    assert_eq!(format!("{:?}", DbAccountStatus::Frozen), "Frozen");
    assert_eq!(format!("{:?}", DbAccountStatus::Closed), "Closed");
    
    assert_eq!(format!("{:?}", DbSigningCondition::None), "None");
    assert_eq!(format!("{:?}", DbSigningCondition::AnyOwner), "AnyOwner");
    assert_eq!(format!("{:?}", DbSigningCondition::AllOwners), "AllOwners");
}
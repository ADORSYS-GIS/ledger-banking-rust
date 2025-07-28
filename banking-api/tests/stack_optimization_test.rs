use serde_json;
use banking_api::domain::{Account, AccountType, AccountStatus, SigningCondition, Transaction, TransactionType, TransactionStatus, Customer, CustomerType, IdentityType, RiskRating, CustomerStatus};
use banking_api::domain::compliance::{KycCheck, CheckResult};
use banking_api::domain::workflow::DocumentReference;
use banking_api::domain::transaction::TransactionAudit;
use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;
use heapless::String as HeaplessString;

/// Test comprehensive serialization/deserialization of optimized stack-based types
#[cfg(test)]
mod stack_optimization_tests {
    use super::*;

    #[test]
    fn test_account_serialization_with_fixed_arrays() {
        // Create account with optimized field types
        let account = Account {
            account_id: Uuid::new_v4(),
            product_code: HeaplessString::try_from("SAVP0001").unwrap(),
            account_type: AccountType::Savings,
            account_status: AccountStatus::Active,
            signing_condition: SigningCondition::None,
            currency: HeaplessString::try_from("USD").unwrap(),
            open_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            domicile_branch_id: Uuid::new_v4(),
            current_balance: Decimal::new(150000, 2), // $1500.00
            available_balance: Decimal::new(150000, 2),
            accrued_interest: Decimal::new(1250, 2), // $12.50
            overdraft_limit: Some(Decimal::new(50000, 2)), // $500.00
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
            last_activity_date: Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
            dormancy_threshold_days: Some(90),
            reactivation_required: false,
            pending_closure_reason_id: None,
            disbursement_instructions: None,
            status_changed_by: Some(Uuid::new_v4()), // Changed to UUID for ReferencedPerson.person_id
            status_change_reason_id: None,  // Changed to UUID, using None for test
            status_change_timestamp: Some(Utc::now()),
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
            updated_by: Uuid::new_v4(), // Changed to UUID for ReferencedPerson.person_id
        };

        // Test serialization
        let json = serde_json::to_string(&account).expect("Serialization should succeed");
        println!("Account JSON: {}", json);

        // Verify key optimized fields are serialized correctly
        assert!(json.contains("\"product_code\":\"SAVP0001\""));
        assert!(json.contains("\"currency\":\"USD\""));
        assert!(json.contains("\"account_type\":\"Savings\""));
        assert!(json.contains("\"account_status\":\"Active\""));

        // Test deserialization  
        let deserialized: Account = serde_json::from_str(&json).expect("Deserialization should succeed");
        
        // Verify fixed array fields
        assert_eq!(deserialized.product_code.as_str(), "SAVP0001");
        let currency_str = deserialized.currency.as_str();
        assert_eq!(currency_str, "USD");
        assert!(matches!(deserialized.account_type, AccountType::Savings));
        assert!(matches!(deserialized.account_status, AccountStatus::Active));
        // updated_by is now a UUID, so we just verify it exists
        assert!(!deserialized.updated_by.is_nil());
    }

    #[test]
    fn test_transaction_serialization_with_heapless_strings() {
        let transaction = Transaction {
            transaction_id: Uuid::new_v4(),
            account_id: Uuid::new_v4(),
            transaction_code: HeaplessString::try_from("DEBIT1").unwrap(),
            transaction_type: TransactionType::Debit,
            amount: Decimal::new(25000, 2), // $250.00
            currency: HeaplessString::try_from("USD").unwrap(),
            description: HeaplessString::try_from("ATM withdrawal at Main Branch").unwrap(),
            channel_id: HeaplessString::try_from("ATM").unwrap(),
            terminal_id: Some(Uuid::new_v4()),
            agent_user_id: None,
            transaction_date: Utc::now(),
            value_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            status: TransactionStatus::Posted,
            reference_number: HeaplessString::try_from("TXN2024011500123").unwrap(),
            external_reference: Some(HeaplessString::try_from("ATM_REF_456").unwrap()),
            gl_code: HeaplessString::try_from("GL1100001").unwrap(),
            requires_approval: false,
            approval_status: None,
            risk_score: Some(Decimal::new(15, 2)), // 0.15
            created_at: Utc::now(),
        };

        // Test serialization
        let json = serde_json::to_string(&transaction).expect("Transaction serialization should succeed");
        println!("Transaction JSON: {}", json);

        // Verify HeaplessString fields are serialized correctly
        assert!(json.contains("\"description\":\"ATM withdrawal at Main Branch\""));
        assert!(json.contains("\"channel_id\":\"ATM\""));
        assert!(json.contains("\"reference_number\":\"TXN2024011500123\""));

        // Test deserialization
        let deserialized: Transaction = serde_json::from_str(&json).expect("Transaction deserialization should succeed");
        
        // Verify HeaplessString fields
        assert_eq!(deserialized.description.as_str(), "ATM withdrawal at Main Branch");
        assert_eq!(deserialized.channel_id.as_str(), "ATM");
        assert_eq!(deserialized.reference_number.as_str(), "TXN2024011500123");
        assert_eq!(deserialized.transaction_code.as_str(), "DEBIT1");
        assert_eq!(deserialized.gl_code.as_str(), "GL1100001");
    }

    #[test]
    fn test_customer_serialization_with_bounded_strings() {
        #[allow(deprecated)]
        let customer = Customer::new(
            Uuid::new_v4(),
            CustomerType::Individual,
            "Jane Smith-Johnson",
            IdentityType::NationalId,
            "ID789012345",
            RiskRating::Low,
            CustomerStatus::Active,
            Uuid::new_v4(),
        ).expect("Customer creation should succeed");

        // Test serialization
        let json = serde_json::to_string(&customer).expect("Customer serialization should succeed");
        println!("Customer JSON: {}", json);

        // Verify bounded string fields
        assert!(json.contains("\"full_name\":\"Jane Smith-Johnson\""));
        assert!(json.contains("\"id_number\":\"ID789012345\""));
        // updated_by is now a UUID, check it exists in JSON
        assert!(json.contains("\"updated_by\":\""));

        // Test deserialization
        let deserialized: Customer = serde_json::from_str(&json).expect("Customer deserialization should succeed");
        
        // Verify HeaplessString fields maintain data integrity
        assert_eq!(deserialized.full_name.as_str(), "Jane Smith-Johnson");
        assert_eq!(deserialized.id_number.as_str(), "ID789012345");
        // updated_by is now a UUID
        assert!(!deserialized.updated_by.is_nil());
        assert!(matches!(deserialized.customer_type, CustomerType::Individual));
        assert!(matches!(deserialized.risk_rating, RiskRating::Low));
    }

    #[test]
    fn test_blake3_hash_serialization() {
        // Test KycCheck with Blake3 hash
        let kyc_check = KycCheck::new(
            "identity_verification",
            CheckResult::Pass,
            Some(HeaplessString::try_from("Document verified successfully").unwrap()),
        );

        let json = serde_json::to_string(&kyc_check).expect("KycCheck serialization should succeed");
        println!("KycCheck JSON: {}", json);

        let deserialized: KycCheck = serde_json::from_str(&json).expect("KycCheck deserialization should succeed");
        assert!(matches!(deserialized.result, CheckResult::Pass));
        assert_eq!(deserialized.check_type, KycCheck::hash_from_type_name("identity_verification"));

        // Test DocumentReference with Blake3 hash
        let doc_ref = DocumentReference::new(
            "passport".to_string(),
            b"passport_scan_data_content_for_hashing",
        );

        let doc_json = serde_json::to_string(&doc_ref).expect("DocumentReference serialization should succeed");
        println!("DocumentReference JSON: {}", doc_json);

        let doc_deserialized: DocumentReference = serde_json::from_str(&doc_json).expect("DocumentReference deserialization should succeed");
        assert_eq!(doc_deserialized.document_type, "passport");
        // Hash should be deterministic for same content
        let expected_hash = blake3::hash(b"passport_scan_data_content_for_hashing");
        assert_eq!(doc_deserialized.document_id, expected_hash);

        // Test TransactionAudit with Blake3 hash
        let audit = TransactionAudit::new(
            Uuid::new_v4(),
            "status_change".to_string(),
            Uuid::new_v4(), // Changed to UUID for performed_by
            Some("Pending".to_string()),
            Some("Posted".to_string()),
            None,  // Changed to UUID for reason_id, using None for test
            Some("audit_details_content_for_verification"),
        );

        let audit_json = serde_json::to_string(&audit).expect("TransactionAudit serialization should succeed");
        println!("TransactionAudit JSON: {}", audit_json);

        let audit_deserialized: TransactionAudit = serde_json::from_str(&audit_json).expect("TransactionAudit deserialization should succeed");
        assert_eq!(audit_deserialized.action_type, "status_change");
        // performed_by is now a UUID
        assert!(!audit_deserialized.performed_by.is_nil());
        // Verify hash integrity
        let expected_audit_hash = blake3::hash(b"audit_details_content_for_verification");
        assert_eq!(audit_deserialized.details, Some(expected_audit_hash));
    }

    #[test]
    fn test_memory_efficiency_comparisons() {
        use std::mem;
        
        // Test fixed arrays vs String
        let string_product = String::from("SAVP0001");
        let array_product = {
            let mut code = [0u8; 12];
            code[..8].copy_from_slice(b"SAVP0001");
            code
        };
        
        println!("String product code: {} bytes", mem::size_of_val(&string_product));
        println!("Array product code: {} bytes", mem::size_of_val(&array_product));
        
        // Array should be smaller and predictable
        assert!(mem::size_of_val(&array_product) < mem::size_of_val(&string_product));
        assert_eq!(mem::size_of_val(&array_product), 12);

        // Test HeaplessString vs String for various sizes
        let string_desc = String::from("Transaction description");
        let heapless_desc: HeaplessString<500> = HeaplessString::try_from("Transaction description").unwrap();
        
        println!("String description: {} bytes", mem::size_of_val(&string_desc));
        println!("HeaplessString description: {} bytes", mem::size_of_val(&heapless_desc));
        
        // For short strings, String might be smaller due to heap allocation
        // But HeaplessString provides predictable stack allocation
        
        // Test Blake3 Hash vs String
        let string_id = String::from("document_id_1234567890abcdef");
        let hash_id = blake3::hash(b"document_content");
        
        println!("String document ID: {} bytes", mem::size_of_val(&string_id));
        println!("Blake3 hash ID: {} bytes", mem::size_of_val(&hash_id));
        
        // Hash should be exactly 32 bytes regardless of content size
        assert_eq!(mem::size_of_val(&hash_id), 32);
    }

    #[test]
    fn test_validation_with_stack_types() {
        // Test that validation still works with stack-based types
        
        // Note: Customer::new only validates length constraints, business validation happens in services
        // So empty name creation succeeds but would be caught by business logic validation
        #[allow(deprecated)]
        let empty_name_result = Customer::new(
            Uuid::new_v4(),
            CustomerType::Individual,
            "",
            IdentityType::NationalId,
            "ID123",
            RiskRating::Low,
            CustomerStatus::Active,
            Uuid::new_v4(),
        );
        assert!(empty_name_result.is_ok(), "Empty name creation should succeed (business validation handled elsewhere)");

        // Test valid customer creation
        #[allow(deprecated)]
        let valid_customer = Customer::new(
            Uuid::new_v4(),
            CustomerType::Corporate,
            "ACME Corporation Ltd",
            IdentityType::CompanyRegistration,
            "REG987654321",
            RiskRating::Medium,
            CustomerStatus::Active,
            Uuid::new_v4(),
        );
        assert!(valid_customer.is_ok(), "Valid customer should pass validation");

        // Test HeaplessString overflow protection
        let very_long_name = "A".repeat(300); // Longer than HeaplessString<255>
        #[allow(deprecated)]
        let overflow_result = Customer::new(
            Uuid::new_v4(),
            CustomerType::Individual,
            &very_long_name,
            IdentityType::NationalId,
            "ID123",
            RiskRating::Low,
            CustomerStatus::Active,
            Uuid::new_v4(),
        );
        assert!(overflow_result.is_err(), "Overflow should fail validation");
    }

    #[test]
    fn test_enum_serialization_compatibility() {
        // Test that enum serialization maintains database compatibility
        
        let account_type = AccountType::Savings;
        let json = serde_json::to_string(&account_type).expect("AccountType serialization should succeed");
        assert_eq!(json, "\"Savings\"");
        
        let deserialized: AccountType = serde_json::from_str(&json).expect("AccountType deserialization should succeed");
        assert!(matches!(deserialized, AccountType::Savings));

        let transaction_status = TransactionStatus::AwaitingApproval;
        let status_json = serde_json::to_string(&transaction_status).expect("TransactionStatus serialization should succeed");
        assert_eq!(status_json, "\"AwaitingApproval\"");
        
        let status_deserialized: TransactionStatus = serde_json::from_str(&status_json).expect("TransactionStatus deserialization should succeed");
        assert!(matches!(status_deserialized, TransactionStatus::AwaitingApproval));

        let customer_status = CustomerStatus::PendingVerification;
        let cust_json = serde_json::to_string(&customer_status).expect("CustomerStatus serialization should succeed");
        assert_eq!(cust_json, "\"PendingVerification\"");
        
        let cust_deserialized: CustomerStatus = serde_json::from_str(&cust_json).expect("CustomerStatus deserialization should succeed");
        assert!(matches!(cust_deserialized, CustomerStatus::PendingVerification));
    }
}
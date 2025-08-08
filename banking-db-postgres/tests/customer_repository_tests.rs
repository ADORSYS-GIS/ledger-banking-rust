#[cfg(feature = "postgres_tests")]
mod tests {
    use uuid::Uuid;
    use chrono::Utc;
    use heapless::String as HeaplessString;
    use banking_db::models::{
        CustomerModel, CustomerDocumentModel, CustomerAuditModel,
        CustomerType, IdentityType, RiskRating, CustomerStatus, DocumentStatus
    };
    use banking_db::repository::CustomerRepository;
    use banking_db_postgres::repository::customer_repository_impl::PostgresCustomerRepository;
    use sqlx::PgPool;

    async fn setup_test_db() -> PgPool {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/banking_db".to_string());
            
        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to create connection pool");

        // Create test person for foreign key references
        let test_person_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
        sqlx::query(
            r#"
            INSERT INTO persons (id, person_type, display_name, external_identifier)
            VALUES ($1, 'system', 'Test User', 'test-user')
            ON CONFLICT (id) DO NOTHING
            "#
        )
        .bind(test_person_id)
        .execute(&pool)
        .await
        .expect("Failed to create test person");

        pool
    }

    fn create_test_customer() -> CustomerModel {
        let test_person_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
        
        CustomerModel {
            id: Uuid::new_v4(),
            customer_type: CustomerType::Individual,
            full_name: HeaplessString::try_from("John Doe").unwrap(),
            id_type: IdentityType::NationalId,
            id_number: HeaplessString::try_from("ID123456789").unwrap(),
            risk_rating: RiskRating::Low,
            status: CustomerStatus::Active,
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
            updated_by_person_id: test_person_id,
        }
    }

    #[tokio::test]
    async fn test_customer_crud_operations() {
        let pool = setup_test_db().await;
        let repo = PostgresCustomerRepository::new(pool);
        let mut customer = create_test_customer();
        
        // Unique identity for this test
        let unique_id = Uuid::new_v4();
        let id_str = format!("ID{}", &unique_id.to_string()[0..8]);
        let name_str = format!("Test Customer {}", &unique_id.to_string()[0..8]);
        customer.id_number = HeaplessString::try_from(id_str.as_str()).unwrap();
        customer.full_name = HeaplessString::try_from(name_str.as_str()).unwrap();

        // Test CREATE
        let created_customer = repo.create(customer.clone()).await
            .expect("Failed to create customer");

        assert_eq!(created_customer.id, customer.id);
        assert_eq!(created_customer.full_name, customer.full_name);
        assert_eq!(created_customer.customer_type, customer.customer_type);

        // Test READ by ID
        let found_customer = repo.find_by_id(customer.id).await
            .expect("Failed to find customer")
            .expect("Customer not found");

        assert_eq!(found_customer.id, customer.id);
        assert_eq!(found_customer.full_name, customer.full_name);

        // Test READ by identity
        let found_by_identity = repo.find_by_identity(
            &customer.id_type.to_string(),
            customer.id_number.as_str()
        ).await
            .expect("Failed to find customer by identity")
            .expect("Customer not found by identity");

        assert_eq!(found_by_identity.id, customer.id);

        // Test UPDATE
        let mut updated_customer = found_customer;
        updated_customer.risk_rating = RiskRating::Medium;
        updated_customer.last_updated_at = Utc::now();

        let result = repo.update(updated_customer.clone()).await
            .expect("Failed to update customer");

        assert_eq!(result.risk_rating, RiskRating::Medium);

        // Test EXISTS
        let exists = repo.exists(customer.id).await
            .expect("Failed to check customer existence");
        assert!(exists);

        // Test soft DELETE
        repo.delete(customer.id, customer.updated_by_person_id).await
            .expect("Failed to delete customer");

        // Verify soft delete (status should be Deceased)
        let deleted_customer = repo.find_by_id(customer.id).await
            .expect("Failed to find deleted customer");
        
        // Customer should still exist but with Deceased status
        assert!(deleted_customer.is_some());
        assert_eq!(deleted_customer.unwrap().status, CustomerStatus::Deceased);
    }

    #[tokio::test]
    async fn test_find_by_risk_rating() {
        let pool = setup_test_db().await;
        let repo = PostgresCustomerRepository::new(pool);
        
        // Create customers with different risk ratings
        let mut customer1 = create_test_customer();
        let mut customer2 = create_test_customer();
        
        let unique_id1 = Uuid::new_v4();
        let unique_id2 = Uuid::new_v4();
        
        customer1.id = unique_id1;
        let id1_str = format!("HIGH{}", &unique_id1.to_string()[0..6]);
        let name1_str = format!("High Risk Customer {}", &unique_id1.to_string()[0..6]);
        customer1.id_number = HeaplessString::try_from(id1_str.as_str()).unwrap();
        customer1.full_name = HeaplessString::try_from(name1_str.as_str()).unwrap();
        customer1.risk_rating = RiskRating::High;
        
        customer2.id = unique_id2;
        let id2_str = format!("MED{}", &unique_id2.to_string()[0..6]);
        let name2_str = format!("Medium Risk Customer {}", &unique_id2.to_string()[0..6]);
        customer2.id_number = HeaplessString::try_from(id2_str.as_str()).unwrap();
        customer2.full_name = HeaplessString::try_from(name2_str.as_str()).unwrap();
        customer2.risk_rating = RiskRating::Medium;

        repo.create(customer1.clone()).await.expect("Failed to create high risk customer");
        repo.create(customer2.clone()).await.expect("Failed to create medium risk customer");

        // Test find by risk rating
        let high_risk_customers = repo.find_by_risk_rating("High").await
            .expect("Failed to find high risk customers");
        
        assert!(!high_risk_customers.is_empty());
        assert!(high_risk_customers.iter().any(|c| c.id == customer1.id));

        let medium_risk_customers = repo.find_by_risk_rating("Medium").await
            .expect("Failed to find medium risk customers");
        
        assert!(!medium_risk_customers.is_empty());
        assert!(medium_risk_customers.iter().any(|c| c.id == customer2.id));
    }

    #[tokio::test]
    async fn test_risk_rating_update_with_audit() {
        let pool = setup_test_db().await;
        let repo = PostgresCustomerRepository::new(pool);
        let mut customer = create_test_customer();
        
        let unique_id = Uuid::new_v4();
        let risk_id_str = format!("RISK{}", &unique_id.to_string()[0..6]);
        let risk_name_str = format!("Risk Update Customer {}", &unique_id.to_string()[0..6]);
        customer.id_number = HeaplessString::try_from(risk_id_str.as_str()).unwrap();
        customer.full_name = HeaplessString::try_from(risk_name_str.as_str()).unwrap();
        customer.risk_rating = RiskRating::Low;

        repo.create(customer.clone()).await.expect("Failed to create customer");

        // Update risk rating
        repo.update_risk_rating(customer.id, "High", customer.updated_by_person_id).await
            .expect("Failed to update risk rating");

        // Verify risk rating was updated
        let updated_customer = repo.find_by_id(customer.id).await
            .expect("Failed to find customer")
            .expect("Customer not found");
        
        assert_eq!(updated_customer.risk_rating, RiskRating::High);

        // Verify audit trail
        let audit_trail = repo.get_audit_trail(customer.id).await
            .expect("Failed to get audit trail");
        
        assert!(!audit_trail.is_empty());
        assert!(audit_trail.iter().any(|entry| 
            entry.field_name.as_str() == "risk_rating" && 
            entry.new_value.as_ref().map(|v| v.as_str()) == Some("High")
        ));
    }

    #[tokio::test]
    async fn test_customer_documents() {
        let pool = setup_test_db().await;
        let repo = PostgresCustomerRepository::new(pool);
        let mut customer = create_test_customer();
        
        let unique_id = Uuid::new_v4();
        let doc_id_str = format!("DOC{}", &unique_id.to_string()[0..6]);
        let doc_name_str = format!("Document Customer {}", &unique_id.to_string()[0..6]);
        customer.id_number = HeaplessString::try_from(doc_id_str.as_str()).unwrap();
        customer.full_name = HeaplessString::try_from(doc_name_str.as_str()).unwrap();

        repo.create(customer.clone()).await.expect("Failed to create customer");

        // Create a document
        let document = CustomerDocumentModel {
            id: Uuid::new_v4(),
            customer_id: customer.id,
            document_type: HeaplessString::try_from("passport").unwrap(),
            document_path: Some(HeaplessString::try_from("/documents/passport.pdf").unwrap()),
            status: DocumentStatus::Uploaded,
            uploaded_at: Utc::now(),
            uploaded_by: customer.updated_by_person_id,
            verified_at: None,
            verified_by: None,
        };

        // Add document
        let added_document = repo.add_document(document.clone()).await
            .expect("Failed to add document");
        
        assert_eq!(added_document.id, document.id);
        assert_eq!(added_document.status, DocumentStatus::Uploaded);

        // Get documents
        let documents = repo.get_documents(customer.id).await
            .expect("Failed to get documents");
        
        assert!(!documents.is_empty());
        assert!(documents.iter().any(|d| d.id == document.id));
    }

    #[tokio::test]
    async fn test_find_requiring_review() {
        let pool = setup_test_db().await;
        let repo = PostgresCustomerRepository::new(pool);
        
        // Create customer requiring review (pending verification)
        let mut customer = create_test_customer();
        let unique_id = Uuid::new_v4();
        customer.id = unique_id;
        let rev_id_str = format!("REV{}", &unique_id.to_string()[0..6]);
        let rev_name_str = format!("Review Customer {}", &unique_id.to_string()[0..6]);
        customer.id_number = HeaplessString::try_from(rev_id_str.as_str()).unwrap();
        customer.full_name = HeaplessString::try_from(rev_name_str.as_str()).unwrap();
        customer.status = CustomerStatus::PendingVerification;

        repo.create(customer.clone()).await.expect("Failed to create customer requiring review");

        // Find customers requiring review
        let customers_requiring_review = repo.find_requiring_review().await
            .expect("Failed to find customers requiring review");
        
        assert!(!customers_requiring_review.is_empty());
        assert!(customers_requiring_review.iter().any(|c| c.id == customer.id));
    }

    #[tokio::test]
    async fn test_customer_portfolio() {
        let pool = setup_test_db().await;
        let repo = PostgresCustomerRepository::new(pool);
        let mut customer = create_test_customer();
        
        let unique_id = Uuid::new_v4();
        customer.id = unique_id;
        let port_id_str = format!("PORT{}", &unique_id.to_string()[0..6]);
        let port_name_str = format!("Portfolio Customer {}", &unique_id.to_string()[0..6]);
        customer.id_number = HeaplessString::try_from(port_id_str.as_str()).unwrap();
        customer.full_name = HeaplessString::try_from(port_name_str.as_str()).unwrap();

        repo.create(customer.clone()).await.expect("Failed to create customer");

        // Get portfolio (should exist with default values)
        let portfolio = repo.get_portfolio(customer.id).await
            .expect("Failed to get portfolio")
            .expect("Portfolio not found");
        
        assert_eq!(portfolio.customer_id, customer.id);
        assert_eq!(portfolio.total_accounts, 0);
    }

    #[tokio::test]
    async fn test_pagination() {
        let pool = setup_test_db().await;
        let repo = PostgresCustomerRepository::new(pool);

        // Create test customers with predictable names for pagination testing
        let mut test_customers = Vec::new();
        let unique_suffix = Uuid::new_v4().to_string()[0..8].to_string(); // Use UUID for uniqueness
        for i in 1..=8 {
            let mut customer = create_test_customer();
            customer.id = Uuid::new_v4(); // Ensure unique ID
            customer.full_name = HeaplessString::try_from(format!("PaginatedCust{:02}_{}", i, unique_suffix).as_str()).unwrap();
            customer.id_number = HeaplessString::try_from(format!("PAG{}{:02}", unique_suffix, i).as_str()).unwrap();
            
            let created_customer = repo.create(customer).await
                .expect("Failed to create test customer");
            test_customers.push(created_customer);
        }

        // Test list with pagination - get first 3 customers
        let customers_page1 = repo.list(0, 3).await
            .expect("Failed to list customers page 1");
        
        let customers_page2 = repo.list(3, 3).await
            .expect("Failed to list customers page 2");
        
        // Should not have overlapping customers
        for customer1 in &customers_page1 {
            assert!(!customers_page2.iter().any(|c2| c2.id == customer1.id),
                "Customer {} appears in both page 1 and page 2", customer1.full_name.as_str());
        }

        // Verify we got the expected number of customers per page
        assert!(customers_page1.len() <= 3, "Page 1 should have at most 3 customers");
        assert!(customers_page2.len() <= 3, "Page 2 should have at most 3 customers");

        // Test count
        let total_count = repo.count().await
            .expect("Failed to count customers");
        
        assert!(total_count >= 8, "Should have at least 8 customers (our test data)");
    }

    #[tokio::test]
    async fn test_audit_trail() {
        let pool = setup_test_db().await;
        let repo = PostgresCustomerRepository::new(pool);
        let mut customer = create_test_customer();
        
        let unique_id = Uuid::new_v4();
        let aud_id_str = format!("AUD{}", &unique_id.to_string()[0..6]);
        let aud_name_str = format!("Audit Customer {}", &unique_id.to_string()[0..6]);
        customer.id_number = HeaplessString::try_from(aud_id_str.as_str()).unwrap();
        customer.full_name = HeaplessString::try_from(aud_name_str.as_str()).unwrap();

        repo.create(customer.clone()).await.expect("Failed to create customer");

        // Add manual audit entry
        let audit_entry = CustomerAuditModel {
            id: Uuid::new_v4(),
            customer_id: customer.id,
            field_name: HeaplessString::try_from("notes").unwrap(),
            old_value: None,
            new_value: Some(HeaplessString::try_from("Test note added").unwrap()),
            changed_at: Utc::now(),
            changed_by: customer.updated_by_person_id,
            reason: Some(HeaplessString::try_from("Manual test").unwrap()),
        };

        let added_audit = repo.add_audit_entry(audit_entry.clone()).await
            .expect("Failed to add audit entry");
        
        assert_eq!(added_audit.id, audit_entry.id);

        // Get audit trail
        let audit_trail = repo.get_audit_trail(customer.id).await
            .expect("Failed to get audit trail");
        
        assert!(!audit_trail.is_empty());
        assert!(audit_trail.iter().any(|entry| entry.id == audit_entry.id));
    }
}
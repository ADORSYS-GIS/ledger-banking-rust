#[cfg(feature = "postgres_tests")]
mod channel_repository_tests {
    use banking_api::BankingResult;
    use banking_db::models::channel::{ChannelModel, ChannelStatus};
    use banking_db::repository::ChannelRepository;
    use banking_db_postgres::repository::ChannelRepositoryImpl;
    use sqlx::PgPool;
    use uuid::Uuid;
    use heapless::String as HeaplessString;
    use chrono::Utc;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    async fn setup_test_db() -> BankingResult<PgPool> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://user:password@localhost:5432/mydb".to_string());
        
        let pool = PgPool::connect(&database_url).await.map_err(|e| {
            banking_api::BankingError::Internal(format!("Connection failed: {e}"))
        })?;
        
        Ok(pool)
    }

    async fn cleanup_database(pool: &PgPool) {
        let _ = sqlx::query("DELETE FROM channel_reconciliation_reports").execute(pool).await;
        let _ = sqlx::query("DELETE FROM channel_fees").execute(pool).await;
        let _ = sqlx::query("DELETE FROM channels").execute(pool).await;
    }

    fn create_test_channel() -> ChannelModel {
        let channel_id = Uuid::new_v4();
        let now = Utc::now();
        
        ChannelModel {
            id: channel_id,
            channel_code: HeaplessString::try_from("ATM001").unwrap(),
            channel_name: HeaplessString::try_from("ATM Branch 001").unwrap(),
            channel_type: "ATM".to_string(),
            status: ChannelStatus::Active,
            daily_limit: Some(Decimal::from_str("10000.00").unwrap()),
            per_transaction_limit: Some(Decimal::from_str("5000.00").unwrap()),
            supported_currencies: vec![
                HeaplessString::try_from("USD").unwrap(),
                HeaplessString::try_from("EUR").unwrap(),
            ],
            requires_additional_auth: false,
            fee_schedule_id: None,
            created_at: now,
            updated_at: now,
        }
    }

    fn create_unique_test_channel(suffix: &str) -> ChannelModel {
        let channel_id = Uuid::new_v4();
        let now = Utc::now();
        let unique_code = format!("CH{}", suffix);
        let unique_name = format!("Channel {}", suffix);
        
        ChannelModel {
            id: channel_id,
            channel_code: HeaplessString::try_from(unique_code.as_str()).unwrap(),
            channel_name: HeaplessString::try_from(unique_name.as_str()).unwrap(),
            channel_type: "MobileApp".to_string(),
            status: ChannelStatus::Active,
            daily_limit: Some(Decimal::from_str("20000.00").unwrap()),
            per_transaction_limit: Some(Decimal::from_str("1000.00").unwrap()),
            supported_currencies: vec![HeaplessString::try_from("USD").unwrap()],
            requires_additional_auth: true,
            fee_schedule_id: None,
            created_at: now,
            updated_at: now,
        }
    }

    #[tokio::test]
    async fn test_create_channel() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        cleanup_database(&pool).await;
        
        let repo = ChannelRepositoryImpl::new(pool);
        let channel = create_test_channel();
        let channel_id = channel.id;
        
        let created = repo.create(channel.clone()).await.expect("Failed to create channel");
        
        assert_eq!(created.id, channel_id);
        assert_eq!(created.channel_code.as_str(), "ATM001");
        assert_eq!(created.channel_name.as_str(), "ATM Branch 001");
        assert_eq!(created.channel_type, "ATM");
        assert!(matches!(created.status, ChannelStatus::Active));
        assert_eq!(created.daily_limit, Some(Decimal::from_str("10000.00").unwrap()));
        assert_eq!(created.per_transaction_limit, Some(Decimal::from_str("5000.00").unwrap()));
        assert_eq!(created.supported_currencies.len(), 2);
        assert!(!created.requires_additional_auth);
        
        cleanup_database(&repo.get_pool()).await;
    }

    #[tokio::test]
    async fn test_find_by_id() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        cleanup_database(&pool).await;
        
        let repo = ChannelRepositoryImpl::new(pool);
        let channel = create_test_channel();
        let channel_id = channel.id;
        
        let _created = repo.create(channel).await.expect("Failed to create channel");
        
        let found = repo.find_by_id(channel_id).await.expect("Failed to find channel by ID");
        assert!(found.is_some());
        let found_channel = found.unwrap();
        assert_eq!(found_channel.id, channel_id);
        assert_eq!(found_channel.channel_code.as_str(), "ATM001");
        
        cleanup_database(&repo.get_pool()).await;
    }

    #[tokio::test]
    async fn test_find_by_code() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        cleanup_database(&pool).await;
        
        let repo = ChannelRepositoryImpl::new(pool);
        let channel = create_test_channel();
        let channel_code = channel.channel_code.clone();
        
        let _created = repo.create(channel).await.expect("Failed to create channel");
        
        let found = repo.find_by_code(channel_code.as_str()).await.expect("Failed to find channel by code");
        assert!(found.is_some());
        let found_channel = found.unwrap();
        assert_eq!(found_channel.channel_code.as_str(), "ATM001");
        assert_eq!(found_channel.channel_name.as_str(), "ATM Branch 001");
        
        cleanup_database(&repo.get_pool()).await;
    }

    #[tokio::test]
    async fn test_update_channel() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        cleanup_database(&pool).await;
        
        let repo = ChannelRepositoryImpl::new(pool);
        let mut channel = create_test_channel();
        
        let _created = repo.create(channel.clone()).await.expect("Failed to create channel");
        
        channel.channel_name = HeaplessString::try_from("Updated ATM Branch").unwrap();
        channel.daily_limit = Some(Decimal::from_str("15000.00").unwrap());
        channel.status = ChannelStatus::Maintenance;
        channel.updated_at = Utc::now();
        
        let updated = repo.update(channel.clone()).await.expect("Failed to update channel");
        
        assert_eq!(updated.channel_name.as_str(), "Updated ATM Branch");
        assert_eq!(updated.daily_limit, Some(Decimal::from_str("15000.00").unwrap()));
        assert!(matches!(updated.status, ChannelStatus::Maintenance));
        
        cleanup_database(&repo.get_pool()).await;
    }

    #[tokio::test]
    async fn test_find_by_type() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        cleanup_database(&pool).await;
        
        let repo = ChannelRepositoryImpl::new(pool);
        
        let channel1 = create_unique_test_channel("001");
        let mut channel2 = create_unique_test_channel("002");
        channel2.channel_type = "ATM".to_string();
        
        let _created1 = repo.create(channel1).await.expect("Failed to create channel 1");
        let _created2 = repo.create(channel2).await.expect("Failed to create channel 2");
        
        let mobile_channels = repo.find_by_type("MobileApp").await.expect("Failed to find channels by type");
        assert_eq!(mobile_channels.len(), 1);
        assert_eq!(mobile_channels[0].channel_type, "MobileApp");
        
        let atm_channels = repo.find_by_type("ATM").await.expect("Failed to find ATM channels");
        assert_eq!(atm_channels.len(), 1);
        assert_eq!(atm_channels[0].channel_type, "ATM");
        
        cleanup_database(&repo.get_pool()).await;
    }

    #[tokio::test]
    async fn test_find_active() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        cleanup_database(&pool).await;
        
        let repo = ChannelRepositoryImpl::new(pool);
        
        let channel1 = create_unique_test_channel("001");
        let mut channel2 = create_unique_test_channel("002");
        channel2.status = ChannelStatus::Inactive;
        
        let _created1 = repo.create(channel1).await.expect("Failed to create active channel");
        let _created2 = repo.create(channel2).await.expect("Failed to create inactive channel");
        
        let active_channels = repo.find_active().await.expect("Failed to find active channels");
        assert_eq!(active_channels.len(), 1);
        assert!(matches!(active_channels[0].status, ChannelStatus::Active));
        
        cleanup_database(&repo.get_pool()).await;
    }

    #[tokio::test]
    async fn test_update_status() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        cleanup_database(&pool).await;
        
        let repo = ChannelRepositoryImpl::new(pool);
        let channel = create_test_channel();
        let channel_id = channel.id;
        
        let _created = repo.create(channel).await.expect("Failed to create channel");
        
        repo.update_status(channel_id, ChannelStatus::Suspended).await.expect("Failed to update status");
        
        let found = repo.find_by_id(channel_id).await.expect("Failed to find channel").unwrap();
        assert!(matches!(found.status, ChannelStatus::Suspended));
        
        cleanup_database(&repo.get_pool()).await;
    }

    #[tokio::test]
    async fn test_exists() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        cleanup_database(&pool).await;
        
        let repo = ChannelRepositoryImpl::new(pool);
        let channel = create_test_channel();
        let channel_id = channel.id;
        
        let exists_before = repo.exists(channel_id).await.expect("Failed to check existence");
        assert!(!exists_before);
        
        let _created = repo.create(channel).await.expect("Failed to create channel");
        
        let exists_after = repo.exists(channel_id).await.expect("Failed to check existence");
        assert!(exists_after);
        
        cleanup_database(&repo.get_pool()).await;
    }

    #[tokio::test]
    async fn test_find_by_currency() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        cleanup_database(&pool).await;
        
        let repo = ChannelRepositoryImpl::new(pool);
        
        let mut channel1 = create_unique_test_channel("001");
        channel1.supported_currencies = vec![
            HeaplessString::try_from("USD").unwrap(),
            HeaplessString::try_from("EUR").unwrap(),
        ];
        
        let mut channel2 = create_unique_test_channel("002");
        channel2.supported_currencies = vec![HeaplessString::try_from("GBP").unwrap()];
        
        let _created1 = repo.create(channel1).await.expect("Failed to create channel 1");
        let _created2 = repo.create(channel2).await.expect("Failed to create channel 2");
        
        let usd_channels = repo.find_by_currency("USD").await.expect("Failed to find USD channels");
        assert_eq!(usd_channels.len(), 1);
        assert!(usd_channels[0].supported_currencies.iter().any(|c| c.as_str() == "USD"));
        
        let gbp_channels = repo.find_by_currency("GBP").await.expect("Failed to find GBP channels");
        assert_eq!(gbp_channels.len(), 1);
        assert!(gbp_channels[0].supported_currencies.iter().any(|c| c.as_str() == "GBP"));
        
        cleanup_database(&repo.get_pool()).await;
    }

    #[tokio::test]
    async fn test_soft_delete() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        cleanup_database(&pool).await;
        
        let repo = ChannelRepositoryImpl::new(pool);
        let channel = create_test_channel();
        let channel_id = channel.id;
        
        let _created = repo.create(channel).await.expect("Failed to create channel");
        
        repo.soft_delete(channel_id).await.expect("Failed to soft delete channel");
        
        let found = repo.find_by_id(channel_id).await.expect("Failed to find channel").unwrap();
        assert!(matches!(found.status, ChannelStatus::Inactive));
        
        cleanup_database(&repo.get_pool()).await;
    }

    #[tokio::test]
    async fn test_find_all_paginated() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        cleanup_database(&pool).await;
        
        let repo = ChannelRepositoryImpl::new(pool);
        
        let channel1 = create_unique_test_channel("001");
        let channel2 = create_unique_test_channel("002");
        let channel3 = create_unique_test_channel("003");
        
        let _created1 = repo.create(channel1).await.expect("Failed to create channel 1");
        let _created2 = repo.create(channel2).await.expect("Failed to create channel 2");
        let _created3 = repo.create(channel3).await.expect("Failed to create channel 3");
        
        let first_page = repo.find_all_paginated(2, 0).await.expect("Failed to get first page");
        assert_eq!(first_page.len(), 2);
        
        let second_page = repo.find_all_paginated(2, 2).await.expect("Failed to get second page");
        assert_eq!(second_page.len(), 1);
        
        cleanup_database(&repo.get_pool()).await;
    }

    #[tokio::test]
    async fn test_count_all() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        cleanup_database(&pool).await;
        
        let repo = ChannelRepositoryImpl::new(pool);
        
        let count_before = repo.count_all().await.expect("Failed to count channels");
        assert_eq!(count_before, 0);
        
        let channel1 = create_unique_test_channel("001");
        let channel2 = create_unique_test_channel("002");
        
        let _created1 = repo.create(channel1).await.expect("Failed to create channel 1");
        let _created2 = repo.create(channel2).await.expect("Failed to create channel 2");
        
        let count_after = repo.count_all().await.expect("Failed to count channels");
        assert_eq!(count_after, 2);
        
        cleanup_database(&repo.get_pool()).await;
    }

    #[tokio::test]
    async fn test_get_channel_stats() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        cleanup_database(&pool).await;
        
        let repo = ChannelRepositoryImpl::new(pool);
        let channel = create_test_channel();
        let channel_id = channel.id;
        
        let _created = repo.create(channel).await.expect("Failed to create channel");
        
        // Note: This test will return empty stats since we don't have transactions
        // In a real scenario, we would need to create test transactions first
        let stats = repo.get_channel_stats(channel_id).await.expect("Failed to get channel stats");
        
        assert_eq!(stats.channel_id, channel_id);
        assert_eq!(stats.total_transactions, 0);
        assert_eq!(stats.daily_volume, Decimal::from_str("0").unwrap());
        assert_eq!(stats.monthly_volume, Decimal::from_str("0").unwrap());
        assert!(stats.last_transaction_date.is_none());
        assert_eq!(stats.success_rate, 0.0);
        
        cleanup_database(&repo.get_pool()).await;
    }

    #[tokio::test]
    async fn test_channel_not_found_operations() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        cleanup_database(&pool).await;
        
        let repo = ChannelRepositoryImpl::new(pool);
        let non_existent_id = Uuid::new_v4();
        
        let found = repo.find_by_id(non_existent_id).await.expect("Failed to search for non-existent channel");
        assert!(found.is_none());
        
        let exists = repo.exists(non_existent_id).await.expect("Failed to check existence");
        assert!(!exists);
        
        let update_result = repo.update_status(non_existent_id, ChannelStatus::Active).await;
        assert!(update_result.is_err());
        
        let delete_result = repo.soft_delete(non_existent_id).await;
        assert!(delete_result.is_err());
        
        cleanup_database(&repo.get_pool()).await;
    }

    #[tokio::test]
    async fn test_channel_validation_errors() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        cleanup_database(&pool).await;
        
        let repo = ChannelRepositoryImpl::new(pool);
        let mut channel = create_test_channel();
        
        // Test duplicate channel code
        let _created1 = repo.create(channel.clone()).await.expect("Failed to create first channel");
        
        channel.id = Uuid::new_v4(); // Different ID but same code
        let duplicate_result = repo.create(channel).await;
        assert!(duplicate_result.is_err());
        
        cleanup_database(&repo.get_pool()).await;
    }
}
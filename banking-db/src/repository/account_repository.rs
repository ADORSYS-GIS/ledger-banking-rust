use async_trait::async_trait;
use banking_api::BankingResult;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, NaiveDate};

use crate::models::{AccountModel, AccountOwnershipModel, AccountRelationshipModel, AccountMandateModel, AccountHoldModel, FinalSettlementModel, StatusChangeModel};

#[async_trait]
pub trait AccountRepository: Send + Sync {
    /// Create a new account record
    async fn create(&self, account: AccountModel) -> BankingResult<AccountModel>;
    
    /// Update existing account record
    async fn update(&self, account: AccountModel) -> BankingResult<AccountModel>;
    
    /// Find account by ID
    async fn find_by_id(&self, account_id: Uuid) -> BankingResult<Option<AccountModel>>;
    
    /// Find accounts by customer ID
    async fn find_by_customer_id(&self, customer_id: Uuid) -> BankingResult<Vec<AccountModel>>;
    
    /// Find accounts by product code
    async fn find_by_product_code(&self, product_code: &str) -> BankingResult<Vec<AccountModel>>;
    
    /// Find accounts by status
    async fn find_by_status(&self, status: &str) -> BankingResult<Vec<AccountModel>>;
    
    /// Find accounts eligible for dormancy
    async fn find_dormancy_candidates(&self, reference_date: NaiveDate, threshold_days: i32) -> BankingResult<Vec<AccountModel>>;
    
    /// Find accounts pending closure
    async fn find_pending_closure(&self) -> BankingResult<Vec<AccountModel>>;
    
    /// Find interest-bearing accounts
    async fn find_interest_bearing_accounts(&self) -> BankingResult<Vec<AccountModel>>;
    
    /// Update account status with audit trail
    /// @param changed_by - References Person.person_id
    async fn update_status(&self, account_id: Uuid, status: &str, reason: &str, changed_by: Uuid) -> BankingResult<()>;
    
    /// Update account balance
    async fn update_balance(&self, account_id: Uuid, current_balance: Decimal, available_balance: Decimal) -> BankingResult<()>;
    
    /// Update accrued interest
    async fn update_accrued_interest(&self, account_id: Uuid, accrued_interest: Decimal) -> BankingResult<()>;
    
    /// Reset accrued interest to zero (after capitalization)
    async fn reset_accrued_interest(&self, account_id: Uuid) -> BankingResult<()>;
    
    /// Account Ownership Operations
    async fn create_ownership(&self, ownership: AccountOwnershipModel) -> BankingResult<AccountOwnershipModel>;
    async fn find_ownership_by_account(&self, account_id: Uuid) -> BankingResult<Vec<AccountOwnershipModel>>;
    async fn find_accounts_by_owner(&self, customer_id: Uuid) -> BankingResult<Vec<AccountOwnershipModel>>;
    async fn delete_ownership(&self, ownership_id: Uuid) -> BankingResult<()>;
    
    /// Account Relationship Operations
    async fn create_relationship(&self, relationship: AccountRelationshipModel) -> BankingResult<AccountRelationshipModel>;
    async fn find_relationships_by_account(&self, account_id: Uuid) -> BankingResult<Vec<AccountRelationshipModel>>;
    async fn find_relationships_by_entity(&self, entity_id: Uuid, entity_type: &str) -> BankingResult<Vec<AccountRelationshipModel>>;
    async fn update_relationship(&self, relationship: AccountRelationshipModel) -> BankingResult<AccountRelationshipModel>;
    async fn delete_relationship(&self, relationship_id: Uuid) -> BankingResult<()>;
    
    /// Account Mandate Operations
    async fn create_mandate(&self, mandate: AccountMandateModel) -> BankingResult<AccountMandateModel>;
    async fn find_mandates_by_account(&self, account_id: Uuid) -> BankingResult<Vec<AccountMandateModel>>;
    async fn find_mandates_by_grantee(&self, grantee_customer_id: Uuid) -> BankingResult<Vec<AccountMandateModel>>;
    async fn update_mandate_status(&self, mandate_id: Uuid, status: &str) -> BankingResult<()>;
    async fn find_active_mandates(&self, account_id: Uuid) -> BankingResult<Vec<AccountMandateModel>>;
    
    /// Account Hold Operations
    async fn create_hold(&self, hold: AccountHoldModel) -> BankingResult<AccountHoldModel>;
    async fn find_holds_by_account(&self, account_id: Uuid) -> BankingResult<Vec<AccountHoldModel>>;
    async fn find_active_holds(&self, account_id: Uuid) -> BankingResult<Vec<AccountHoldModel>>;
    /// Release a hold
    /// @param released_by - References Person.person_id
    async fn release_hold(&self, hold_id: Uuid, released_by: Uuid) -> BankingResult<()>;
    async fn release_expired_holds(&self, reference_date: DateTime<Utc>) -> BankingResult<i64>;
    
    /// Final Settlement Operations
    async fn create_final_settlement(&self, settlement: FinalSettlementModel) -> BankingResult<FinalSettlementModel>;
    async fn find_settlement_by_account(&self, account_id: Uuid) -> BankingResult<Option<FinalSettlementModel>>;
    async fn update_settlement_status(&self, settlement_id: Uuid, status: &str) -> BankingResult<()>;
    
    /// Status History Operations
    async fn get_status_history(&self, account_id: Uuid) -> BankingResult<Vec<StatusChangeModel>>;
    async fn add_status_change(&self, status_change: StatusChangeModel) -> BankingResult<StatusChangeModel>;
    
    /// Utility Operations
    async fn exists(&self, account_id: Uuid) -> BankingResult<bool>;
    async fn count_by_customer(&self, customer_id: Uuid) -> BankingResult<i64>;
    async fn count_by_product(&self, product_code: &str) -> BankingResult<i64>;
    async fn list(&self, offset: i64, limit: i64) -> BankingResult<Vec<AccountModel>>;
    async fn count(&self) -> BankingResult<i64>;

    /// Update last activity date for account
    async fn update_last_activity_date(&self, account_id: Uuid, activity_date: chrono::NaiveDate) -> BankingResult<()>;
}
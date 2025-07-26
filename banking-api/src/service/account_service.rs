use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    domain::{Account, AccountStatus, AccountHold},
    error::BankingResult,
};

#[async_trait]
pub trait AccountService: Send + Sync {
    /// Create a new account
    async fn create_account(&self, account: Account) -> BankingResult<Account>;
    
    /// Find account by ID
    async fn find_account_by_id(&self, account_id: Uuid) -> BankingResult<Option<Account>>;
    
    /// Status updates with immediate enforcement
    /// @param authorized_by - References ReferencedPerson.person_id
    async fn update_account_status(&self, account_id: Uuid, status: AccountStatus, authorized_by: Uuid) -> BankingResult<()>;
    
    /// Balance operations with product rule integration
    async fn calculate_balance(&self, account_id: Uuid) -> BankingResult<Decimal>;
    async fn calculate_available_balance(&self, account_id: Uuid) -> BankingResult<Decimal>;
    /// Apply hold with reason ID validation
    async fn apply_hold(&self, account_id: Uuid, amount: Decimal, reason_id: Uuid, additional_details: Option<&str>) -> BankingResult<()>;
    
    /// Legacy method - deprecated, use apply_hold with reason_id instead
    #[deprecated(note = "Use apply_hold with reason_id instead")]
    async fn apply_hold_legacy(&self, account_id: Uuid, amount: Decimal, reason: String) -> BankingResult<()>;
    
    /// Product catalog integration
    async fn refresh_product_rules(&self, account_id: Uuid) -> BankingResult<()>;

    /// Find accounts by customer
    async fn find_accounts_by_customer(&self, customer_id: Uuid) -> BankingResult<Vec<Account>>;

    /// Find accounts by status
    async fn find_accounts_by_status(&self, status: AccountStatus) -> BankingResult<Vec<Account>>;

    /// Find interest bearing accounts
    async fn find_interest_bearing_accounts(&self) -> BankingResult<Vec<Account>>;

    /// Update account balance
    /// @param updated_by - References ReferencedPerson.person_id
    async fn update_balance(&self, account_id: Uuid, new_balance: Decimal, updated_by: Uuid) -> BankingResult<()>;

    /// Reset accrued interest to zero
    async fn reset_accrued_interest(&self, account_id: Uuid) -> BankingResult<()>;

    /// Update accrued interest
    async fn update_accrued_interest(&self, account_id: Uuid, amount: Decimal) -> BankingResult<()>;

    /// Get account status (for caching)
    async fn get_account_status(&self, account_id: Uuid) -> BankingResult<AccountStatus>;

    /// Get active holds for an account
    async fn get_active_holds(&self, account_id: Uuid) -> BankingResult<Vec<AccountHold>>;

    /// Release a hold
    /// @param released_by - References ReferencedPerson.person_id
    async fn release_hold(&self, hold_id: Uuid, released_by: Uuid) -> BankingResult<()>;

    /// Find accounts eligible for dormancy check
    async fn find_dormancy_candidates(&self, threshold_days: i32) -> BankingResult<Vec<Account>>;

    /// Update last activity date
    async fn update_last_activity_date(&self, account_id: Uuid, activity_date: chrono::NaiveDate) -> BankingResult<()>;
}
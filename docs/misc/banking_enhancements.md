# Banking System Enhancement Requirements

## Critical Additions to Domain Model

### 1. Enhanced Account Status Management

The current Account model needs these additional fields and status handling:

```rust
// Enhanced Account.rs
pub struct Account {
    // ... existing fields ...
    
    // Account lifecycle management
    pub close_date: Option<NaiveDate>,
    pub last_activity_date: Option<NaiveDate>, // Critical for dormancy calculation
    pub dormancy_threshold_days: Option<i32>,  // Product-specific override
    pub reactivation_required: bool,           // Tracks if mini-KYC needed
    pub pending_closure_reason: Option<String>,
    pub disbursement_instructions: Option<DisbursementInstructions>,
    
    // Enhanced audit trail
    pub status_changed_by: Option<String>,
    pub status_change_reason: Option<String>,
    pub status_change_timestamp: Option<DateTime<Utc>>,
}

// New supporting structures
pub struct DisbursementInstructions {
    pub method: DisbursementMethod,
    pub target_account: Option<Uuid>,
    pub cash_pickup_location: Option<String>,
    pub authorized_recipient: Option<String>,
}

pub enum DisbursementMethod {
    Transfer,
    CashWithdrawal,
    Check,
    HoldFunds, // For disputed closures
}

// Enhanced AccountStatus with workflow states
pub enum AccountStatus { 
    PendingApproval,    // New account pending KYC/CDD
    Active, 
    Dormant,            // Auto-triggered by inactivity
    Frozen,             // Risk & Compliance controlled
    PendingClosure,     // Closure initiated, awaiting settlement
    Closed,             // Permanently closed
    PendingReactivation, // Dormant account pending mini-KYC
}
```

### 2. Workflow Management System

Add a comprehensive workflow engine to handle multi-step processes:

```rust
// WorkflowEngine.rs - New domain object
pub struct AccountWorkflow {
    pub workflow_id: Uuid,
    pub account_id: Uuid,
    pub workflow_type: WorkflowType,
    pub current_step: WorkflowStep,
    pub status: WorkflowStatus,
    pub initiated_by: String,
    pub initiated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub steps_completed: Vec<WorkflowStepRecord>,
    pub next_action_required: Option<String>,
    pub timeout_at: Option<DateTime<Utc>>,
}

pub enum WorkflowType {
    AccountOpening,
    AccountClosure,
    AccountReactivation,
    ComplianceVerification,
    MultiPartyApproval,
}

pub enum WorkflowStep {
    InitiateRequest,
    ComplianceCheck,
    DocumentVerification,
    ApprovalRequired,
    FinalSettlement,
    Completed,
}

pub enum WorkflowStatus {
    InProgress,
    PendingAction,
    Completed,
    Failed,
    Cancelled,
    TimedOut,
}

pub struct WorkflowStepRecord {
    pub step: WorkflowStep,
    pub completed_at: DateTime<Utc>,
    pub completed_by: String,
    pub notes: Option<String>,
    pub supporting_documents: Vec<String>,
}
```

## Enhanced Service Traits

### 1. Account Lifecycle Service

```rust
// AccountLifecycleService.rs - New comprehensive service
pub trait AccountLifecycleService: Send + Sync {
    // Account origination workflow
    async fn initiate_account_opening(&self, request: AccountOpeningRequest) -> BankingResult<AccountWorkflow>;
    async fn complete_kyc_verification(&self, account_id: Uuid, verification_result: KycResult) -> BankingResult<()>;
    async fn activate_account(&self, account_id: Uuid, authorized_by: String) -> BankingResult<()>;
    
    // Dormancy management (automated)
    async fn check_dormancy_eligibility(&self, account_id: Uuid) -> BankingResult<DormancyAssessment>;
    async fn mark_account_dormant(&self, account_id: Uuid, system_triggered: bool) -> BankingResult<()>;
    
    // Reactivation workflow (requires human intervention)
    async fn initiate_reactivation(&self, account_id: Uuid, requested_by: String) -> BankingResult<AccountWorkflow>;
    async fn complete_mini_kyc(&self, account_id: Uuid, verification_result: KycResult) -> BankingResult<()>;
    
    // Account closure workflow
    async fn initiate_closure(&self, account_id: Uuid, closure_request: ClosureRequest) -> BankingResult<AccountWorkflow>;
    async fn calculate_final_settlement(&self, account_id: Uuid) -> BankingResult<FinalSettlement>;
    async fn process_final_disbursement(&self, account_id: Uuid, disbursement: DisbursementInstructions) -> BankingResult<()>;
    async fn finalize_closure(&self, account_id: Uuid) -> BankingResult<()>;
    
    // Status management
    async fn update_account_status(&self, account_id: Uuid, new_status: AccountStatus, reason: String, authorized_by: String) -> BankingResult<()>;
    async fn get_status_history(&self, account_id: Uuid) -> BankingResult<Vec<StatusChangeRecord>>;
}

// Supporting structures
pub struct AccountOpeningRequest {
    pub customer_id: Uuid,
    pub product_code: String,
    pub initial_deposit: Option<Decimal>,
    pub channel: String,
    pub initiated_by: String,
    pub supporting_documents: Vec<DocumentReference>,
}

pub struct ClosureRequest {
    pub reason: ClosureReason,
    pub disbursement_instructions: DisbursementInstructions,
    pub requested_by: String,
    pub force_closure: bool, // For regulatory/compliance closures
}

pub enum ClosureReason {
    CustomerRequest,
    Regulatory,
    Compliance,
    Dormancy,
    SystemMaintenance,
}

pub struct FinalSettlement {
    pub current_balance: Decimal,
    pub accrued_interest: Decimal,
    pub pending_fees: Decimal,
    pub closure_fees: Decimal,
    pub final_amount: Decimal,
    pub requires_disbursement: bool,
}

pub struct DormancyAssessment {
    pub is_eligible: bool,
    pub last_activity_date: Option<NaiveDate>,
    pub days_inactive: i32,
    pub threshold_days: i32,
    pub product_specific_rules: Vec<String>,
}
```

### 2. Enhanced Transaction Service with Status Awareness

```rust
// Enhanced TransactionService.rs
pub trait TransactionService: Send + Sync {
    // ... existing methods ...
    
    // Status-aware transaction validation
    async fn validate_account_transactional_status(&self, account_id: Uuid, transaction_type: TransactionType) -> BankingResult<ValidationResult>;
    async fn get_permitted_operations(&self, account_id: Uuid) -> BankingResult<Vec<PermittedOperation>>;
    
    // Final settlement operations
    async fn process_closure_transaction(&self, account_id: Uuid, settlement: FinalSettlement) -> BankingResult<Transaction>;
    async fn reverse_pending_transactions(&self, account_id: Uuid, reason: String) -> BankingResult<Vec<Transaction>>;
}

pub enum PermittedOperation {
    Credit,
    Debit,
    InterestPosting,
    FeeApplication,
    ClosureSettlement,
    None, // For frozen/pending accounts
}
```

## Enhanced End-of-Day Processing

### 1. Dormancy Management Job

```rust
// DormancyManagementJob.rs - New EOD component
pub struct DormancyManagementJob {
    account_service: Arc<dyn AccountLifecycleService>,
    product_catalog_client: Arc<ProductCatalogClient>,
    calendar_service: Arc<dyn CalendarService>,
}

impl DormancyManagementJob {
    pub async fn process_dormancy_candidates(&self, processing_date: NaiveDate) -> BankingResult<DormancyReport> {
        // 1. Query all ACTIVE accounts
        // 2. For each account, get product-specific dormancy rules
        // 3. Calculate days since last customer-initiated transaction
        // 4. Mark eligible accounts as DORMANT
        // 5. Generate regulatory notifications
    }
    
    async fn get_dormancy_threshold(&self, product_code: &str) -> BankingResult<i32> {
        // Fetch from Product Catalog, fallback to system default
    }
    
    async fn calculate_inactivity_period(&self, account_id: Uuid, reference_date: NaiveDate) -> BankingResult<i32> {
        // Count business days since last customer transaction
        // Exclude system-generated transactions (interest, fees)
    }
}

pub struct DormancyReport {
    pub processing_date: NaiveDate,
    pub accounts_evaluated: i32,
    pub accounts_marked_dormant: i32,
    pub accounts_by_product: HashMap<String, i32>,
    pub notifications_generated: i32,
    pub errors_encountered: Vec<String>,
}
```

### 2. Account Maintenance Job

```rust
// AccountMaintenanceJob.rs - Comprehensive maintenance
pub struct AccountMaintenanceJob {
    lifecycle_service: Arc<dyn AccountLifecycleService>,
    transaction_service: Arc<dyn TransactionService>,
}

impl AccountMaintenanceJob {
    pub async fn process_pending_closures(&self, processing_date: NaiveDate) -> BankingResult<MaintenanceReport> {
        // 1. Find all PENDING_CLOSURE accounts
        // 2. Check if final settlement can be completed
        // 3. Process automatic disbursements where configured
        // 4. Finalize closures where balance is zero
    }
    
    pub async fn cleanup_expired_workflows(&self, processing_date: NaiveDate) -> BankingResult<()> {
        // Clean up workflows that have exceeded timeout periods
    }
    
    pub async fn generate_regulatory_notifications(&self, processing_date: NaiveDate) -> BankingResult<Vec<RegulatoryNotification>> {
        // Generate required notifications for status changes
    }
}
```

## Enhanced Database Schema

### 1. Additional Tables Required

```sql
-- Account workflow tracking
CREATE TABLE account_workflows (
    workflow_id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    workflow_type VARCHAR(50) NOT NULL,
    current_step VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL,
    initiated_by VARCHAR(100) NOT NULL,
    initiated_at TIMESTAMP NOT NULL,
    completed_at TIMESTAMP,
    timeout_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Workflow step history
CREATE TABLE workflow_step_records (
    record_id UUID PRIMARY KEY,
    workflow_id UUID NOT NULL REFERENCES account_workflows(workflow_id),
    step_name VARCHAR(50) NOT NULL,
    completed_at TIMESTAMP NOT NULL,
    completed_by VARCHAR(100) NOT NULL,
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Account status history (immutable audit trail)
CREATE TABLE account_status_history (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    old_status VARCHAR(20),
    new_status VARCHAR(20) NOT NULL,
    change_reason VARCHAR(255) NOT NULL,
    changed_by VARCHAR(100) NOT NULL,
    changed_at TIMESTAMP NOT NULL,
    system_triggered BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Final settlement records for closed accounts
CREATE TABLE account_final_settlements (
    settlement_id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    settlement_date DATE NOT NULL,
    current_balance DECIMAL(15,2) NOT NULL,
    accrued_interest DECIMAL(15,2) NOT NULL,
    closure_fees DECIMAL(15,2) NOT NULL,
    final_amount DECIMAL(15,2) NOT NULL,
    disbursement_method VARCHAR(20) NOT NULL,
    disbursement_reference VARCHAR(100),
    processed_by VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Enhanced indexes for dormancy processing
CREATE INDEX idx_accounts_last_activity ON accounts(last_activity_date, account_status) 
WHERE account_status = 'Active';

CREATE INDEX idx_accounts_status_change ON account_status_history(account_id, changed_at);
```

### 2. Enhanced Account Table Constraints

```sql
-- Add new fields to existing accounts table
ALTER TABLE accounts ADD COLUMN close_date DATE;
ALTER TABLE accounts ADD COLUMN last_activity_date DATE;
ALTER TABLE accounts ADD COLUMN dormancy_threshold_days INTEGER;
ALTER TABLE accounts ADD COLUMN reactivation_required BOOLEAN DEFAULT FALSE;
ALTER TABLE accounts ADD COLUMN pending_closure_reason VARCHAR(255);
ALTER TABLE accounts ADD COLUMN status_changed_by VARCHAR(100);
ALTER TABLE accounts ADD COLUMN status_change_reason VARCHAR(255);
ALTER TABLE accounts ADD COLUMN status_change_timestamp TIMESTAMP;

-- Enhanced status constraints
ALTER TABLE accounts ADD CONSTRAINT check_closure_date 
CHECK (
    (account_status = 'Closed' AND close_date IS NOT NULL) OR 
    (account_status != 'Closed' AND close_date IS NULL)
);

-- Prevent transactions on non-transactional accounts
CREATE OR REPLACE FUNCTION validate_account_transaction()
RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM accounts 
        WHERE account_id = NEW.account_id 
        AND account_status IN ('PendingApproval', 'Frozen', 'Closed', 'PendingClosure')
        AND NEW.transaction_type != 'ClosureSettlement'
    ) THEN
        RAISE EXCEPTION 'Account % is not in a transactional state', NEW.account_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_validate_account_transaction
    BEFORE INSERT ON transactions
    FOR EACH ROW EXECUTE FUNCTION validate_account_transaction();
```

## Implementation Priority

### Phase 1: Core Lifecycle Management
1. Enhanced Account model with status fields
2. AccountLifecycleService implementation
3. Database schema updates
4. Basic workflow engine

### Phase 2: Automated Processing
1. Dormancy management EOD job
2. Account maintenance job
3. Status transition automation
4. Regulatory notification system

### Phase 3: Advanced Features
1. Mini-KYC reactivation workflow
2. Complex closure scenarios
3. Workflow timeout handling
4. Advanced reporting and analytics

## Testing Requirements Enhancement

### 1. Workflow Integration Tests

```rust
#[tokio::test]
async fn test_complete_account_opening_workflow() {
    // Test full workflow from initiation to activation
    // Including KYC completion and status transitions
}

#[tokio::test]
async fn test_dormancy_detection_and_reactivation() {
    // Test automated dormancy marking
    // Test mini-KYC reactivation process
}

#[tokio::test]
async fn test_account_closure_with_settlement() {
    // Test complete closure workflow
    // Including final interest calculation and disbursement
}

#[tokio::test]
async fn test_status_based_transaction_restrictions() {
    // Verify transactions are blocked for non-transactional statuses
    // Test exceptions for system operations
}
```

### 2. EOD Processing Tests

```rust
#[tokio::test]
async fn test_dormancy_eod_job() {
    // Test dormancy detection across multiple products
    // Verify product-specific threshold application
}

#[tokio::test]
async fn test_workflow_timeout_cleanup() {
    // Test automatic cleanup of expired workflows
}
```

This enhancement focuses on the critical gaps identified in the requirements document, particularly around account lifecycle management, automated dormancy processing, and workflow-driven operations that are essential for a production banking system.

## Core Financial Operations Engine

### 1. Transaction Processing Pipeline

The system requires a sophisticated, high-performance transaction processing engine with multi-stage validation:

```rust
// TransactionProcessor.rs - Core transaction processing engine
pub struct TransactionProcessor {
    validation_pipeline: Arc<ValidationPipeline>,
    posting_engine: Arc<PostingEngine>,
    fraud_detection_client: Arc<dyn FraudDetectionService>,
    product_catalog_client: Arc<ProductCatalogClient>,
    gl_integration: Arc<dyn GeneralLedgerService>,
}

impl TransactionProcessor {
    pub async fn process_transaction(&self, request: TransactionRequest) -> BankingResult<TransactionResult> {
        // Execute atomic transaction processing with ACID compliance
        let mut tx = self.begin_transaction().await?;
        
        // Stage 1: Pre-validation (fail-fast checks)
        self.validation_pipeline.pre_validate(&request).await?;
        
        // Stage 2: Comprehensive validation
        let validation_result = self.validation_pipeline.full_validate(&request).await?;
        
        // Stage 3: Financial posting
        let posting_result = self.posting_engine.post_transaction(&request, &validation_result).await?;
        
        // Stage 4: GL entry generation
        let gl_entries = self.generate_gl_entries(&request, &posting_result).await?;
        self.gl_integration.post_entries(gl_entries).await?;
        
        // Stage 5: Commit transaction
        tx.commit().await?;
        
        Ok(TransactionResult {
            transaction_id: posting_result.transaction_id,
            reference_number: posting_result.reference_number,
            gl_entries: posting_result.gl_entries,
            timestamp: Utc::now(),
        })
    }
}

// High-performance validation pipeline with parallel processing
pub struct ValidationPipeline {
    channel_validator: Arc<dyn ChannelValidator>,
    status_validator: Arc<dyn StatusValidator>,
    authorization_validator: Arc<dyn AuthorizationValidator>,
    limit_validator: Arc<dyn LimitValidator>,
    balance_validator: Arc<dyn BalanceValidator>,
    cache_layer: Arc<ValidationCache>,
}

impl ValidationPipeline {
    // Fast, sequential checks that can eliminate invalid requests quickly
    pub async fn pre_validate(&self, request: &TransactionRequest) -> BankingResult<()> {
        // 1. Channel/Terminal status (cached, <1ms)
        self.channel_validator.validate_channel_status(request).await?;
        
        // 2. Account status (cached, <1ms)
        self.status_validator.validate_account_status(request).await?;
        
        Ok(())
    }
    
    // Comprehensive validation with parallel execution where possible
    pub async fn full_validate(&self, request: &TransactionRequest) -> BankingResult<ValidationResult> {
        // Execute independent validations in parallel
        let (auth_result, limit_result, balance_result) = tokio::try_join!(
            self.authorization_validator.validate(request),
            self.limit_validator.validate(request),
            self.balance_validator.validate(request)
        )?;
        
        // Aggregate results
        Ok(ValidationResult {
            authorization: auth_result,
            limits: limit_result,
            balance: balance_result,
            risk_score: self.calculate_risk_score(request).await?,
        })
    }
}

// Supporting structures
pub struct TransactionRequest {
    pub account_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    pub currency: String,
    pub description: String,
    pub channel: ChannelType,
    pub terminal_id: Option<Uuid>,
    pub initiator_id: String,
    pub external_reference: Option<String>,
    pub metadata: HashMap<String, String>,
}

pub struct TransactionResult {
    pub transaction_id: Uuid,
    pub reference_number: String,
    pub gl_entries: Vec<GlEntry>,
    pub timestamp: DateTime<Utc>,
}

pub struct ValidationResult {
    pub authorization: AuthorizationResult,
    pub limits: LimitValidationResult,
    pub balance: BalanceValidationResult,
    pub risk_score: Decimal,
}
```

### 2. Specialized Validators with Caching

```rust
// StatusValidator.rs - High-performance status checking
pub struct StatusValidator {
    cache: Arc<StatusCache>,
    customer_service: Arc<dyn CustomerService>,
    account_service: Arc<dyn AccountService>,
}

impl StatusValidator {
    pub async fn validate_account_status(&self, request: &TransactionRequest) -> BankingResult<()> {
        let account_status = self.cache.get_account_status(request.account_id).await?;
        
        match account_status {
            AccountStatus::Active => Ok(()),
            AccountStatus::Frozen => Err(BankingError::AccountFrozen { 
                account_id: request.account_id, 
                frozen_reason: "Risk compliance freeze".to_string() 
            }),
            AccountStatus::Closed => Err(BankingError::AccountClosed { 
                account_id: request.account_id, 
                closure_date: self.get_closure_date(request.account_id).await? 
            }),
            AccountStatus::PendingApproval => Err(BankingError::AccountNotOperational { 
                account_id: request.account_id, 
                reason: "Account pending KYC approval".to_string() 
            }),
            _ => Err(BankingError::AccountNotTransactional { account_id: request.account_id }),
        }
    }
}

// LimitValidator.rs - Multi-tier limit validation
pub struct LimitValidator {
    product_catalog_client: Arc<ProductCatalogClient>,
    hierarchy_service: Arc<dyn HierarchyService>,
    fraud_service: Arc<dyn FraudDetectionService>,
}

impl LimitValidator {
    pub async fn validate(&self, request: &TransactionRequest) -> BankingResult<LimitValidationResult> {
        // Parallel limit checks
        let (terminal_result, product_result, customer_result) = tokio::try_join!(
            self.validate_terminal_limits(request),
            self.validate_product_limits(request),
            self.validate_customer_limits(request)
        )?;
        
        // Risk-based dynamic limits
        let risk_adjusted_limits = self.apply_risk_adjustments(request).await?;
        
        Ok(LimitValidationResult {
            terminal_limits: terminal_result,
            product_limits: product_result,
            customer_limits: customer_result,
            risk_adjustments: risk_adjusted_limits,
            overall_approved: true,
        })
    }
    
    async fn validate_terminal_limits(&self, request: &TransactionRequest) -> BankingResult<TerminalLimitResult> {
        if let Some(terminal_id) = request.terminal_id {
            let limits = self.hierarchy_service.get_terminal_limits(terminal_id).await?;
            let current_volume = self.hierarchy_service.get_current_daily_volume(terminal_id).await?;
            
            if current_volume + request.amount > limits.daily_limit {
                return Err(BankingError::TransactionLimitExceeded {
                    limit: limits.daily_limit,
                    attempted: current_volume + request.amount,
                    limit_type: LimitType::Terminal,
                });
            }
        }
        
        Ok(TerminalLimitResult::Approved)
    }
}

// BalanceValidator.rs - Real-time balance verification
pub struct BalanceValidator {
    account_service: Arc<dyn AccountService>,
    hold_service: Arc<dyn HoldService>,
}

impl BalanceValidator {
    pub async fn validate(&self, request: &TransactionRequest) -> BankingResult<BalanceValidationResult> {
        let account = self.account_service.find_account_by_id(request.account_id).await?
            .ok_or(BankingError::AccountNotFound(request.account_id))?;
        
        let active_holds = self.hold_service.get_active_holds(request.account_id).await?;
        let total_holds: Decimal = active_holds.iter().map(|h| h.amount).sum();
        
        let available_balance = account.current_balance - total_holds + account.overdraft_limit.unwrap_or(Decimal::ZERO);
        
        if request.transaction_type == TransactionType::Debit && request.amount > available_balance {
            return Err(BankingError::InsufficientFunds {
                account_id: request.account_id,
                requested: request.amount,
                available: available_balance,
            });
        }
        
        Ok(BalanceValidationResult {
            current_balance: account.current_balance,
            available_balance,
            holds_total: total_holds,
            overdraft_available: account.overdraft_limit.unwrap_or(Decimal::ZERO),
        })
    }
}
```

### 3. Interest Calculation Engine

```rust
// InterestEngine.rs - Comprehensive interest processing
pub struct InterestEngine {
    product_catalog_client: Arc<ProductCatalogClient>,
    calendar_service: Arc<dyn CalendarService>,
    account_service: Arc<dyn AccountService>,
    transaction_service: Arc<dyn TransactionService>,
}

impl InterestEngine {
    // Daily interest accrual - part of EOD processing
    pub async fn accrue_daily_interest(&self, processing_date: NaiveDate) -> BankingResult<AccrualReport> {
        let mut report = AccrualReport::new(processing_date);
        
        // Process all interest-bearing accounts
        let accounts = self.account_service.find_interest_bearing_accounts().await?;
        
        for account in accounts {
            let accrual_result = self.calculate_daily_accrual(&account, processing_date).await?;
            self.update_accrued_interest(account.account_id, accrual_result.amount).await?;
            
            report.add_account_accrual(account.account_id, accrual_result);
        }
        
        Ok(report)
    }
    
    // Interest capitalization - periodic posting to account balance
    pub async fn capitalize_interest(&self, processing_date: NaiveDate) -> BankingResult<CapitalizationReport> {
        let mut report = CapitalizationReport::new(processing_date);
        
        // Find accounts due for capitalization
        let eligible_accounts = self.find_capitalization_candidates(processing_date).await?;
        
        for account in eligible_accounts {
            if account.accrued_interest > Decimal::ZERO {
                let capitalization_result = self.process_capitalization(&account, processing_date).await?;
                report.add_capitalization(account.account_id, capitalization_result);
            }
        }
        
        Ok(report)
    }
    
    async fn calculate_daily_accrual(&self, account: &Account, date: NaiveDate) -> BankingResult<AccrualCalculation> {
        // Get product-specific interest rules
        let product_rules = self.product_catalog_client.get_interest_rules(&account.product_code).await?;
        
        // Determine calculation method and rate
        let (calculation_method, interest_rate) = match account.account_type {
            AccountType::Savings => {
                let rate = self.get_tiered_savings_rate(&account.product_code, account.current_balance).await?;
                (product_rules.calculation_method, rate)
            },
            AccountType::Loan => {
                (CalculationMethod::DailyBalance, account.loan_interest_rate.unwrap_or(Decimal::ZERO))
            },
            _ => return Ok(AccrualCalculation::zero()),
        };
        
        // Calculate daily interest
        let daily_interest = match calculation_method {
            CalculationMethod::DailyBalance => {
                self.calculate_simple_daily_interest(account.current_balance, interest_rate)
            },
            CalculationMethod::AverageDailyBalance => {
                let avg_balance = self.calculate_average_daily_balance(account.account_id, date).await?;
                self.calculate_simple_daily_interest(avg_balance, interest_rate)
            },
            CalculationMethod::CompoundDaily => {
                self.calculate_compound_daily_interest(account.current_balance, account.accrued_interest, interest_rate)
            },
        };
        
        Ok(AccrualCalculation {
            account_id: account.account_id,
            calculation_date: date,
            principal_balance: account.current_balance,
            interest_rate,
            calculation_method,
            daily_interest,
            days_calculated: 1,
        })
    }
    
    async fn process_capitalization(&self, account: &Account, date: NaiveDate) -> BankingResult<CapitalizationResult> {
        let capitalization_amount = account.accrued_interest;
        
        // Create interest credit transaction
        let interest_transaction = Transaction {
            transaction_id: Uuid::new_v4(),
            account_id: account.account_id,
            transaction_code: "INT_CAP".to_string(),
            transaction_type: TransactionType::Credit,
            amount: capitalization_amount,
            currency: account.currency.clone(),
            description: format!("Interest capitalization for period ending {}", date),
            channel_id: "SYSTEM".to_string(),
            terminal_id: None,
            agent_user_id: None,
            transaction_date: Utc::now(),
            value_date: date,
            status: TransactionStatus::Posted,
            reference_number: self.generate_reference_number().await?,
            external_reference: None,
            gl_code: self.get_interest_gl_code(&account.product_code).await?,
            requires_approval: false,
            approval_status: None,
            risk_score: Some(Decimal::ZERO), // System transaction, no risk
            created_at: Utc::now(),
        };
        
        // Post the transaction
        let posted_transaction = self.transaction_service.process_transaction(interest_transaction).await?;
        
        // Reset accrued interest to zero
        self.account_service.reset_accrued_interest(account.account_id).await?;
        
        Ok(CapitalizationResult {
            account_id: account.account_id,
            capitalization_date: date,
            amount_capitalized: capitalization_amount,
            transaction_id: posted_transaction.transaction_id,
            new_balance: account.current_balance + capitalization_amount,
        })
    }
    
    // Tiered interest rate calculation
    async fn get_tiered_savings_rate(&self, product_code: &str, balance: Decimal) -> BankingResult<Decimal> {
        let rate_tiers = self.product_catalog_client.get_interest_rate_tiers(product_code).await?;
        
        // Find applicable tier based on balance
        for tier in rate_tiers.iter().rev() { // Start from highest tier
            if balance >= tier.minimum_balance {
                return Ok(tier.interest_rate);
            }
        }
        
        // Default to base rate if no tier matches
        Ok(rate_tiers.first().map(|t| t.interest_rate).unwrap_or(Decimal::ZERO))
    }
    
    // Business day aware interest calculations
    async fn should_accrue_interest(&self, account: &Account, date: NaiveDate) -> BankingResult<bool> {
        // Get product-specific accrual rules
        let product_rules = self.product_catalog_client.get_interest_rules(&account.product_code).await?;
        
        match product_rules.accrual_frequency {
            AccrualFrequency::Daily => Ok(true),
            AccrualFrequency::BusinessDaysOnly => {
                self.calendar_service.is_business_day(date, &account.currency).await
            },
            AccrualFrequency::None => Ok(false),
        }
    }
}

// Supporting structures for interest processing
pub struct AccrualCalculation {
    pub account_id: Uuid,
    pub calculation_date: NaiveDate,
    pub principal_balance: Decimal,
    pub interest_rate: Decimal,
    pub calculation_method: CalculationMethod,
    pub daily_interest: Decimal,
    pub days_calculated: i32,
}

pub struct CapitalizationResult {
    pub account_id: Uuid,
    pub capitalization_date: NaiveDate,
    pub amount_capitalized: Decimal,
    pub transaction_id: Uuid,
    pub new_balance: Decimal,
}

pub enum CalculationMethod {
    DailyBalance,
    AverageDailyBalance,
    CompoundDaily,
}

pub enum AccrualFrequency {
    Daily,
    BusinessDaysOnly,
    None,
}

pub struct InterestRateTier {
    pub minimum_balance: Decimal,
    pub interest_rate: Decimal,
    pub tier_name: String,
}
```

### 4. General Ledger Integration

```rust
// GeneralLedgerService.rs - GL entry generation and posting
pub trait GeneralLedgerService: Send + Sync {
    async fn generate_transaction_entries(&self, transaction: &Transaction, account: &Account) -> BankingResult<Vec<GlEntry>>;
    async fn post_entries(&self, entries: Vec<GlEntry>) -> BankingResult<GlPostingResult>;
    async fn reverse_entries(&self, original_entries: Vec<GlEntry>, reason: String) -> BankingResult<Vec<GlEntry>>;
}

pub struct GlEntry {
    pub entry_id: Uuid,
    pub account_code: String,
    pub debit_amount: Option<Decimal>,
    pub credit_amount: Option<Decimal>,
    pub currency: String,
    pub description: String,
    pub reference_number: String,
    pub transaction_id: Uuid,
    pub value_date: NaiveDate,
    pub posting_date: DateTime<Utc>,
}

impl GeneralLedgerService for GlServiceImpl {
    async fn generate_transaction_entries(&self, transaction: &Transaction, account: &Account) -> BankingResult<Vec<GlEntry>> {
        let mut entries = Vec::new();
        
        // Get GL mapping from product catalog
        let gl_mapping = self.product_catalog_client.get_gl_mapping(&account.product_code).await?;
        
        match transaction.transaction_type {
            TransactionType::Credit => {
                // Credit the customer account
                entries.push(GlEntry {
                    entry_id: Uuid::new_v4(),
                    account_code: gl_mapping.customer_account_code,
                    debit_amount: None,
                    credit_amount: Some(transaction.amount),
                    currency: transaction.currency.clone(),
                    description: transaction.description.clone(),
                    reference_number: transaction.reference_number.clone(),
                    transaction_id: transaction.transaction_id,
                    value_date: transaction.value_date,
                    posting_date: transaction.transaction_date,
                });
                
                // Debit the source (varies by channel)
                let source_gl_code = self.get_source_gl_code(transaction).await?;
                entries.push(GlEntry {
                    entry_id: Uuid::new_v4(),
                    account_code: source_gl_code,
                    debit_amount: Some(transaction.amount),
                    credit_amount: None,
                    currency: transaction.currency.clone(),
                    description: transaction.description.clone(),
                    reference_number: transaction.reference_number.clone(),
                    transaction_id: transaction.transaction_id,
                    value_date: transaction.value_date,
                    posting_date: transaction.transaction_date,
                });
            },
            TransactionType::Debit => {
                // Debit the customer account
                entries.push(GlEntry {
                    entry_id: Uuid::new_v4(),
                    account_code: gl_mapping.customer_account_code,
                    debit_amount: Some(transaction.amount),
                    credit_amount: None,
                    currency: transaction.currency.clone(),
                    description: transaction.description.clone(),
                    reference_number: transaction.reference_number.clone(),
                    transaction_id: transaction.transaction_id,
                    value_date: transaction.value_date,
                    posting_date: transaction.transaction_date,
                });
                
                // Credit the destination
                let destination_gl_code = self.get_destination_gl_code(transaction).await?;
                entries.push(GlEntry {
                    entry_id: Uuid::new_v4(),
                    account_code: destination_gl_code,
                    debit_amount: None,
                    credit_amount: Some(transaction.amount),
                    currency: transaction.currency.clone(),
                    description: transaction.description.clone(),
                    reference_number: transaction.reference_number.clone(),
                    transaction_id: transaction.transaction_id,
                    value_date: transaction.value_date,
                    posting_date: transaction.transaction_date,
                });
            },
        }
        
        Ok(entries)
    }
}
```

### 5. Performance Optimization Requirements

```rust
// High-performance caching layer for validation pipeline
pub struct ValidationCache {
    account_status_cache: Arc<Cache<Uuid, AccountStatus>>,
    customer_status_cache: Arc<Cache<Uuid, CustomerStatus>>,
    terminal_limits_cache: Arc<Cache<Uuid, TerminalLimits>>,
    product_rules_cache: Arc<Cache<String, ProductRules>>,
}

impl ValidationCache {
    pub fn new() -> Self {
        Self {
            account_status_cache: Arc::new(
                Cache::builder()
                    .max_capacity(100_000)
                    .time_to_live(Duration::from_secs(300)) // 5 minutes
                    .build()
            ),
            customer_status_cache: Arc::new(
                Cache::builder()
                    .max_capacity(50_000)
                    .time_to_live(Duration::from_secs(600)) // 10 minutes
                    .build()
            ),
            // ... other caches
        }
    }
    
    pub async fn get_account_status(&self, account_id: Uuid) -> BankingResult<AccountStatus> {
        if let Some(status) = self.account_status_cache.get(&account_id) {
            return Ok(status);
        }
        
        // Cache miss - fetch from database
        let status = self.account_service.get_account_status(account_id).await?;
        self.account_status_cache.insert(account_id, status.clone());
        
        Ok(status)
    }
    
    // Cache invalidation on status changes
    pub async fn invalidate_account_status(&self, account_id: Uuid) {
        self.account_status_cache.invalidate(&account_id);
    }
}
```

This enhancement provides a comprehensive, high-performance financial operations engine that addresses the critical requirements for transaction processing, interest calculation, and GL integration while maintaining the product catalog-driven architecture and ACID transaction compliance.
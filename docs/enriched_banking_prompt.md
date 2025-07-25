# Create ledger-banking-rust: Multi-Product Core Banking System

## Project Overview

Create a high-performance Rust library called `ledger-banking-rust` for core banking account management. This system must support diverse banking products (savings, current, loans), extensive agent banking networks, and strict regulatory compliance (COBAC). The architecture shall be **API-first, cloud-native, and composable** to support rapid innovation and hyper-personalized customer experiences demanded by the modern financial landscape.

**Architecture Pattern**: Mirror the exact modular architecture of `ledger-posting-rust` using a Cargo workspace with layered "onion" architecture and clear separation of concerns.

## Required Workspace Structure

Create a Cargo workspace with these crates following the same pattern as ledger-posting-rust:

### Core Crates (Required)
- **`banking-api`**: Public interface with domain objects and service traits (equivalent to postings-api)
- **`banking-db`**: Database abstraction with models and repository traits (equivalent to postings-db)  
- **`banking-logic`**: Business logic implementation with services and mappers (equivalent to postings-logic)
- **`banking-db-postgres`**: PostgreSQL implementation (equivalent to postings-db-postgres)
- **`banking-db-mariadb`**: MariaDB implementation (equivalent to postings-db-mariadb)

### Additional Banking-Specific Crates
- **`banking-rules`**: Zen Engine integration for business rules management
- **`banking-compliance`**: KYC/AML compliance engine
- **`banking-channels`**: Multi-channel transaction processing
- **`banking-calendar`**: Business calendar and holiday management

## Core Domain Model (banking-api/src/domain/)

### 1. Customer Information File (CIF) - Master Repository
```rust
// Customer.rs - Single, unified Customer model as authoritative source
pub struct Customer {
    pub customer_id: Uuid, // Non-recyclable, system-generated UUID
    pub customer_type: CustomerType,
    pub full_name: String,
    pub id_type: IdentityType,
    pub id_number: String,
    pub risk_rating: RiskRating, // Read-only field - only updatable via Risk & Compliance API
    pub status: CustomerStatus, // Controls permissible actions on associated accounts
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: String, // Audit trail for all changes
}

pub enum CustomerType { Individual, Corporate }
pub enum IdentityType { NationalId, Passport, CompanyRegistration }
pub enum RiskRating { 
    Low, 
    Medium, 
    High, 
    Blacklisted // Immediate freeze of all associated accounts
}
pub enum CustomerStatus { 
    Active, 
    PendingVerification, 
    Deceased,      // Blocks new debits except approved settlement
    Dissolved,     // Blocks new debits except approved settlement  
    Blacklisted    // Immediate freeze of all associated accounts
}
```

### 2. Unified Account Model (UAM) - Single Flexible Table
```rust
// Account.rs - Single table for all account types with dynamic behavior via productCode
pub struct Account {
    pub account_id: Uuid,
    pub product_code: String, // Critical: Foreign key to Product Catalog - determines ALL business rules
    pub account_type: AccountType,
    pub account_status: AccountStatus,
    pub signing_condition: SigningCondition, // Security control for joint accounts
    pub currency: String,
    pub open_date: NaiveDate,
    pub domicile_branch_id: Uuid,
    
    // Balance fields
    pub current_balance: Decimal,
    pub available_balance: Decimal,
    pub accrued_interest: Decimal,
    pub overdraft_limit: Option<Decimal>,
    
    // Loan-specific fields (nullable for non-loan accounts)
    pub original_principal: Option<Decimal>,
    pub outstanding_principal: Option<Decimal>,
    pub loan_interest_rate: Option<Decimal>,
    pub loan_term_months: Option<i32>,
    pub disbursement_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    pub installment_amount: Option<Decimal>,
    pub next_due_date: Option<NaiveDate>,
    pub penalty_rate: Option<Decimal>,
    pub collateral_id: Option<String>,
    pub loan_purpose: Option<String>,
    
    // Audit fields
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: String,
}

pub enum AccountType { Savings, Current, Loan }
pub enum AccountStatus { 
    Active, 
    Dormant, 
    Frozen,           // Blocks ALL debit transactions - only settable by Risk & Compliance
    Closed, 
    PendingApproval 
}
pub enum SigningCondition { 
    None,       // Single-owner accounts
    AnyOwner,   // Either owner can authorize
    AllOwners   // All owners must jointly authorize - triggers multi-party workflow
}
```

### 3. Agent Banking & Institutional Hierarchy
```rust
// AgentNetwork.rs - Top-level network management
pub struct AgentNetwork {
    pub network_id: Uuid,
    pub network_name: String,
    pub network_type: NetworkType,
    pub status: NetworkStatus,
    pub contract_id: Option<Uuid>,
    pub aggregate_daily_limit: Decimal, // Network-level risk control
    pub current_daily_volume: Decimal,
    pub settlement_gl_code: String,
    pub created_at: DateTime<Utc>,
}

// AgencyBranch.rs - Hierarchical branch structure with cascading limits
pub struct AgencyBranch {
    pub branch_id: Uuid,
    pub network_id: Uuid,
    pub parent_branch_id: Option<Uuid>,
    pub branch_name: String,
    pub branch_code: String,
    pub branch_level: i32,
    pub gl_code_prefix: String, // Critical: Auto-constructs GL account numbers for transactions
    pub geolocation: Option<String>,
    pub status: BranchStatus,
    pub daily_transaction_limit: Decimal, // Branch-level risk control
    pub current_daily_volume: Decimal,
    pub created_at: DateTime<Utc>,
}

// AgentTerminal.rs - Individual terminal controls
pub struct AgentTerminal {
    pub terminal_id: Uuid,
    pub branch_id: Uuid,
    pub agent_user_id: Uuid,
    pub terminal_type: TerminalType,
    pub terminal_name: String,
    pub daily_transaction_limit: Decimal, // Terminal-level limit (most granular)
    pub current_daily_volume: Decimal,
    pub status: TerminalStatus,
    pub last_sync_at: DateTime<Utc>,
}

pub enum NetworkType { Internal, Partner, ThirdParty }
pub enum NetworkStatus { Active, Suspended, Terminated }
pub enum BranchStatus { Active, Suspended, Closed }
pub enum TerminalType { Pos, Mobile, Atm, WebPortal }
pub enum TerminalStatus { Active, Maintenance, Suspended, Decommissioned }
```

### 4. Account Relations - Granular Relationship Management
```rust
// AccountOwnership.rs - Legal title/ownership
pub struct AccountOwnership {
    pub ownership_id: Uuid,
    pub account_id: Uuid,
    pub customer_id: Uuid,
    pub ownership_type: OwnershipType, // Joint, Single, Corporate
    pub ownership_percentage: Option<Decimal>, // For joint accounts
    pub created_at: DateTime<Utc>,
}

// AccountRelationship.rs - Internal servicing responsibility
pub struct AccountRelationship {
    pub relationship_id: Uuid,
    pub account_id: Uuid,
    pub entity_id: Uuid,
    pub entity_type: EntityType, // Branch, Agent, RiskManager, etc.
    pub relationship_type: RelationshipType, // PrimaryHandler, BackupHandler, RiskOversight
    pub status: RelationshipStatus,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

// AccountMandate.rs - Operational rights and permissions
pub struct AccountMandate {
    pub mandate_id: Uuid,
    pub account_id: Uuid,
    pub grantee_customer_id: Uuid,
    pub permission_type: PermissionType, // Triggers specific workflows
    pub transaction_limit: Option<Decimal>,
    pub approval_group_id: Option<Uuid>,
    pub status: MandateStatus,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

// UltimateBeneficiary.rs - KYC/AML compliance for corporate accounts
pub struct UltimateBeneficiary {
    pub ubo_link_id: Uuid,
    pub corporate_customer_id: Uuid, // Must be CustomerType::Corporate
    pub beneficiary_customer_id: Uuid, // Must be CustomerType::Individual
    pub ownership_percentage: Option<Decimal>,
    pub control_type: ControlType,
    pub description: Option<String>,
    pub status: UboStatus,
    pub verification_status: VerificationStatus,
    pub created_at: DateTime<Utc>,
}

pub enum PermissionType {
    ViewOnly,
    LimitedWithdrawal,
    JointApproval,    // Triggers multi-party authorization workflow
    FullAccess,
}

pub enum ControlType { DirectOwnership, IndirectOwnership, SignificantInfluence, SeniorManagement }
pub enum VerificationStatus { Pending, Verified, Rejected, RequiresUpdate }
```

### 5. Business Calendar Management
```rust
// BankHoliday.rs - Definitive non-working days registry
pub struct BankHoliday {
    pub holiday_id: Uuid,
    pub jurisdiction: String, // Country/region code
    pub holiday_date: NaiveDate,
    pub holiday_name: String,
    pub holiday_type: HolidayType, // National, Religious, Banking, Custom
    pub is_recurring: bool, // Annual recurrence
    pub created_at: DateTime<Utc>,
}

pub enum HolidayType { National, Religious, Banking, Custom }

// DateCalculationRules.rs - Business day calculation logic
pub struct DateCalculationRules {
    pub default_shift_rule: DateShiftRule, // Default: NextBusinessDay
    pub weekend_treatment: WeekendTreatment,
    pub jurisdiction: String,
}

pub enum DateShiftRule { 
    NextBusinessDay,    // Default system-wide
    PreviousBusinessDay,
    NoShift,           // Override via Product Catalog
}

pub enum WeekendTreatment { SaturdaySunday, FridayOnly, Custom(Vec<chrono::Weekday>) }
```

### 6. Enhanced Transaction Processing
```rust
// Transaction.rs - Comprehensive transaction model
pub struct Transaction {
    pub transaction_id: Uuid,
    pub account_id: Uuid,
    pub transaction_code: String,
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    pub currency: String,
    pub description: String,
    pub channel_id: String,
    pub terminal_id: Option<Uuid>,
    pub agent_user_id: Option<Uuid>,
    pub transaction_date: DateTime<Utc>,
    pub value_date: NaiveDate, // Business day adjusted
    pub status: TransactionStatus,
    pub reference_number: String,
    pub external_reference: Option<String>,
    pub gl_code: String, // Auto-generated using branch gl_code_prefix
    pub requires_approval: bool, // Based on signing_condition
    pub approval_status: Option<ApprovalStatus>,
    pub risk_score: Option<Decimal>, // From compliance screening
    pub created_at: DateTime<Utc>,
}

pub enum TransactionType { Credit, Debit }
pub enum TransactionStatus { 
    Pending, 
    Posted, 
    Reversed, 
    Failed,
    AwaitingApproval, // Multi-party authorization required
    ApprovalRejected,
}
pub enum ApprovalStatus { Pending, Approved, Rejected, PartiallyApproved }
```

## Service Traits (banking-api/src/service/)

### Core Services Required
```rust
// CustomerService.rs - Customer Information File management
pub trait CustomerService: Send + Sync {
    async fn create_customer(&self, customer: Customer) -> BankingResult<Customer>;
    async fn update_customer(&self, customer: Customer) -> BankingResult<Customer>;
    async fn find_customer_by_id(&self, customer_id: Uuid) -> BankingResult<Option<Customer>>;
    
    // Risk rating updates - restricted to Risk & Compliance module only
    async fn update_risk_rating(&self, customer_id: Uuid, risk_rating: RiskRating, authorized_by: String) -> BankingResult<()>;
    
    // Status changes with cascade effects
    async fn update_customer_status(&self, customer_id: Uuid, status: CustomerStatus, reason: String) -> BankingResult<()>;
    
    // 360-degree customer view
    async fn get_customer_portfolio(&self, customer_id: Uuid) -> BankingResult<CustomerPortfolio>;
}

// AccountService.rs - Unified Account Model operations
pub trait AccountService: Send + Sync {
    async fn create_account(&self, account: Account) -> BankingResult<Account>;
    async fn find_account_by_id(&self, account_id: Uuid) -> BankingResult<Option<Account>>;
    
    // Status updates with immediate enforcement
    async fn update_account_status(&self, account_id: Uuid, status: AccountStatus, authorized_by: String) -> BankingResult<()>;
    
    // Balance operations with product rule integration
    async fn calculate_balance(&self, account_id: Uuid) -> BankingResult<Decimal>;
    async fn calculate_available_balance(&self, account_id: Uuid) -> BankingResult<Decimal>;
    async fn apply_hold(&self, account_id: Uuid, amount: Decimal, reason: String) -> BankingResult<()>;
    
    // Product catalog integration
    async fn refresh_product_rules(&self, account_id: Uuid) -> BankingResult<()>;
}

// TransactionService.rs - Multi-level validation and processing
pub trait TransactionService: Send + Sync {
    async fn process_transaction(&self, transaction: Transaction) -> BankingResult<Transaction>;
    async fn validate_transaction_limits(&self, transaction: &Transaction) -> BankingResult<ValidationResult>;
    async fn reverse_transaction(&self, transaction_id: Uuid, reason: String) -> BankingResult<()>;
    async fn find_transactions_by_account(&self, account_id: Uuid, from: NaiveDate, to: NaiveDate) -> BankingResult<Vec<Transaction>>;
    
    // Multi-party authorization workflow
    async fn initiate_approval_workflow(&self, transaction: Transaction) -> BankingResult<ApprovalWorkflow>;
    async fn approve_transaction(&self, transaction_id: Uuid, approver_id: Uuid) -> BankingResult<()>;
}

// InterestService.rs - Product catalog-driven calculations
pub trait InterestService: Send + Sync {
    async fn calculate_daily_interest(&self, account_id: Uuid) -> BankingResult<Decimal>;
    async fn post_periodic_interest(&self, account_id: Uuid) -> BankingResult<()>;
    async fn calculate_loan_installment(&self, principal: Decimal, rate: Decimal, term_months: i32) -> BankingResult<Decimal>;
    
    // Business day aware processing
    async fn calculate_accrued_interest(&self, account_id: Uuid, from_date: NaiveDate, to_date: NaiveDate) -> BankingResult<Decimal>;
    async fn should_post_interest(&self, account_id: Uuid, date: NaiveDate) -> BankingResult<bool>;
}

// CalendarService.rs - Business day calculations
pub trait CalendarService: Send + Sync {
    async fn is_business_day(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<bool>;
    async fn next_business_day(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<NaiveDate>;
    async fn previous_business_day(&self, date: NaiveDate, jurisdiction: &str) -> BankingResult<NaiveDate>;
    async fn add_business_days(&self, date: NaiveDate, days: i32, jurisdiction: &str) -> BankingResult<NaiveDate>;
    async fn count_business_days(&self, from: NaiveDate, to: NaiveDate, jurisdiction: &str) -> BankingResult<i32>;
}

// HierarchyService.rs - Agent network management
pub trait HierarchyService: Send + Sync {
    async fn validate_hierarchical_limits(&self, terminal_id: Uuid, amount: Decimal) -> BankingResult<ValidationResult>;
    async fn get_branch_gl_prefix(&self, branch_id: Uuid) -> BankingResult<String>;
    async fn update_daily_volumes(&self, terminal_id: Uuid, amount: Decimal) -> BankingResult<()>;
    async fn reset_daily_counters(&self) -> BankingResult<()>;
}
```

## Business Logic Implementation (banking-logic/src/)

### Service Implementations
- Implement all service traits from banking-api with comprehensive business rule validation
- **Critical**: Integrate with Product Catalog API for all financial logic (interest rates, fees, dormancy rules)
- Handle multi-signature workflows for joint accounts based on `signing_condition`
- Enforce immediate status changes (FROZEN accounts, DECEASED customers)
- Implement cascading limit validation (Terminal → Branch → Network)

### Mappers (Crucial Component)
Create mappers in `banking-logic/src/mappers/` to translate between:
- Domain objects (`banking-api::domain::*`) 
- Database models (`banking-db::models::*`)

### Product Catalog Integration Layer
```rust
// ProductCatalogClient.rs - Tier-1 critical service integration
pub struct ProductCatalogClient {
    http_client: reqwest::Client,
    base_url: String,
    timeout: Duration,
    retry_policy: RetryPolicy,
}

impl ProductCatalogClient {
    async fn get_product_rules(&self, product_code: &str) -> BankingResult<ProductRules>;
    async fn get_interest_rate(&self, product_code: &str, balance_tier: Decimal) -> BankingResult<Decimal>;
    async fn get_fee_schedule(&self, product_code: &str) -> BankingResult<FeeSchedule>;
    async fn get_gl_mapping(&self, product_code: &str) -> BankingResult<GlMapping>;
}
```

### Caching Layer with Cache-Aside Pattern
Implement decorator pattern caching using `moka` for:
- Customer data (with TTL based on risk rating)
- Account information and product rules
- Agent network hierarchy
- Bank holiday calendars

### Multi-Party Authorization Workflow
```rust
// ApprovalWorkflow.rs - Joint account transaction processing
pub struct ApprovalWorkflow {
    pub workflow_id: Uuid,
    pub transaction_id: Uuid,
    pub required_approvers: Vec<Uuid>,
    pub received_approvals: Vec<Approval>,
    pub status: WorkflowStatus,
    pub timeout_at: DateTime<Utc>,
}

pub enum WorkflowStatus { Pending, Approved, Rejected, TimedOut }
```

## Zen Engine Integration (banking-rules/)

### Product Rules Engine with Dynamic Rule Loading
```rust
// ProductRulesEngine.rs - External rule execution
pub struct ProductRulesEngine {
    zen_engine: ZenEngine,
    rule_cache: Cache<String, CompiledRule>,
}

impl ProductRulesEngine {
    // Interest rate calculation with tiered structures
    pub async fn calculate_interest_rate(&self, account: &Account, balance: Decimal, context: &RuleContext) -> BankingResult<Decimal>;
    
    // Fee calculation with product-specific triggers
    pub async fn calculate_fees(&self, transaction: &Transaction, account: &Account) -> BankingResult<Vec<Fee>>;
    
    // Multi-level limit validation
    pub async fn validate_transaction_limits(&self, transaction: &Transaction, account: &Account) -> BankingResult<ValidationResult>;
    
    // Delinquency status with grace period calculation
    pub async fn determine_delinquency_status(&self, account: &Account, last_payment_date: NaiveDate) -> BankingResult<DelinquencyStatus>;
    
    // Account eligibility with customer risk integration
    pub async fn validate_account_eligibility(&self, customer: &Customer, product_code: &str) -> BankingResult<EligibilityResult>;
}
```

### Rule Categories to Implement
- **Interest rate calculation**: Tiered rates, compound interest, promotional rates
- **Fee calculation**: Transaction fees, maintenance fees, penalty charges
- **Transaction limits**: Daily, monthly, per-transaction limits with customer risk overlay
- **Delinquency determination**: Grace periods, penalty application triggers
- **Account eligibility**: Product suitability based on customer profile and risk rating
- **Dormancy rules**: Account classification based on transaction frequency and product type

## Compliance Engine (banking-compliance/)

### KYC/AML Services with Real-time Monitoring
```rust
// ComplianceService.rs - Regulatory compliance enforcement
pub trait ComplianceService: Send + Sync {
    async fn perform_kyc_check(&self, customer: &Customer) -> BankingResult<KycResult>;
    async fn screen_against_sanctions(&self, customer: &Customer) -> BankingResult<ScreeningResult>;
    async fn monitor_transaction(&self, transaction: &Transaction) -> BankingResult<MonitoringResult>;
    async fn generate_sar_data(&self, customer_id: Uuid, reason: String) -> BankingResult<SarData>;
    
    // Ultimate Beneficial Owner verification
    async fn verify_ubo_chain(&self, corporate_customer_id: Uuid) -> BankingResult<UboVerificationResult>;
    async fn update_ubo_status(&self, ubo_link_id: Uuid, status: VerificationStatus) -> BankingResult<()>;
}

// Transaction monitoring with configurable rules
pub struct MonitoringRules {
    pub structuring_detection: bool,
    pub velocity_checks: bool,
    pub geographic_risk_assessment: bool,
    pub large_cash_threshold: Decimal,
    pub suspicious_pattern_detection: bool,
    pub cross_border_transaction_monitoring: bool,
}
```

## Channel Processing (banking-channels/)

### Multi-Channel Support with Channel-Specific Logic
```rust
// ChannelProcessor.rs - Channel-aware transaction processing
pub trait ChannelProcessor: Send + Sync {
    async fn validate_channel_limits(&self, transaction: &Transaction, channel: &Channel) -> BankingResult<bool>;
    async fn apply_channel_fees(&self, transaction: &Transaction, channel: &Channel) -> BankingResult<Vec<Fee>>;
    async fn log_channel_activity(&self, transaction: &Transaction, channel: &Channel) -> BankingResult<()>;
    async fn handle_channel_reconciliation(&self, channel_id: String, date: NaiveDate) -> BankingResult<ReconciliationReport>;
    
    // Channel-specific authorization workflows
    async fn requires_additional_auth(&self, transaction: &Transaction, channel: &Channel) -> BankingResult<bool>;
}

pub enum ChannelType {
    MobileApp,
    AgentTerminal,
    ATM,
    InternetBanking,
    BranchTeller,
    USSD,
    ApiGateway,
}
```

## End-of-Day Processing with Business Calendar Integration

### Batch Operations with Date-Aware Processing
```rust
// EodService.rs - Business day aware batch processing
pub trait EodService: Send + Sync {
    // Interest processing with business day rules
    async fn process_daily_interest_accrual(&self) -> BankingResult<EodReport>;
    async fn post_periodic_interest(&self, processing_date: NaiveDate) -> BankingResult<EodReport>;
    
    // Fee application with product-specific schedules
    async fn apply_periodic_fees(&self, processing_date: NaiveDate) -> BankingResult<EodReport>;
    
    // Loan management with grace period calculations
    async fn update_delinquent_loans(&self, processing_date: NaiveDate) -> BankingResult<EodReport>;
    
    // Regulatory reporting
    async fn generate_regulatory_reports(&self, processing_date: NaiveDate) -> BankingResult<Vec<RegulatoryReport>>;
    
    // System maintenance
    async fn reset_daily_counters(&self) -> BankingResult<()>;
    async fn archive_completed_workflows(&self) -> BankingResult<()>;
    
    // Business calendar operations
    async fn determine_next_processing_date(&self, current_date: NaiveDate) -> BankingResult<NaiveDate>;
}

// Date shift processing for scheduled events
pub struct DateShiftProcessor {
    calendar_service: Arc<dyn CalendarService>,
}

impl DateShiftProcessor {
    async fn apply_date_shift_rules(&self, scheduled_date: NaiveDate, product_code: &str) -> BankingResult<NaiveDate>;
    async fn handle_maturity_date_shift(&self, original_maturity: NaiveDate, loan_terms: &LoanTerms) -> BankingResult<NaiveDate>;
}
```

## Database Schema Requirements

### Migration Files with Comprehensive Constraints
Create SQL migration files in both `banking-db-postgres/migrations/` and `banking-db-mariadb/migrations/` for:

1. **Customer tables**: 
   - `customers` (with unique constraints on id_number per id_type)
   - `customer_documents` 
   - `customer_audit_trail`

2. **Account tables**: 
   - `accounts` (with check constraints for balance consistency)
   - `account_ownership` (with percentage sum validation for joint accounts)
   - `account_relationships`
   - `account_mandates`
   - `account_holds`

3. **Agent network tables**: 
   - `agent_networks` (with hierarchical integrity constraints)
   - `agency_branches` (with self-referential parent-child relationships)
   - `agent_terminals`

4. **Transaction tables**: 
   - `transactions` (with immutable audit fields)
   - `transaction_approvals`
   - `approval_workflows`

5. **Compliance tables**: 
   - `kyc_records`
   - `sanctions_screening`
   - `ultimate_beneficiaries` (with corporate/individual type constraints)
   - `compliance_alerts`

6. **Calendar and system tables**: 
   - `bank_holidays` (with jurisdiction-specific indexing)
   - `system_parameters`
   - `audit_trails`

### Key Constraints and Business Rules
- **Foreign key relationships** between all related entities with cascade options
- **Check constraints** for account balance consistency and UBO type validation
- **Unique indexes** on account numbers, customer IDs, terminal IDs
- **Partial indexes** for active records only to optimize query performance
- **Trigger-based audit trails** for all financial transactions
- **Row-level security** for multi-tenant agent network isolation

## Integration Requirements

### External System Interfaces with SLA Requirements
- **Product Catalog API**: 
  - Fetch product rules and parameters (SLA: <100ms, 99.9% availability)
  - Real-time rule updates and cache invalidation
  - Fallback to cached rules with configurable staleness tolerance

- **Risk & Compliance API**:
  - Customer risk rating updates (secured endpoint, audit trail required)
  - Real-time sanctions screening
  - Transaction monitoring alerts

- **Payment Switch Integration**: 
  - ISO 8583 message handling for ATM/POS transactions
  - Real-time authorization and settlement
  - Multi-network routing capabilities

- **Mobile Money APIs**: 
  - Push/pull transaction support with multiple providers
  - Real-time balance inquiries
  - Webhook-based transaction notifications

- **General Ledger System**: 
  - Automated journal entry creation using branch GL prefixes
  - Real-time posting for immediate transactions
  - Batch posting for end-of-day processing

## Error Handling & Logging

### Enhanced Error Types with Business Context
```rust
// BankingError.rs - Comprehensive error taxonomy
pub enum BankingError {
    // Account-related errors
    AccountNotFound(Uuid),
    AccountFrozen { account_id: Uuid, frozen_reason: String },
    AccountClosed { account_id: Uuid, closure_date: NaiveDate },
    InsufficientFunds { account_id: Uuid, requested: Decimal, available: Decimal },
    
    // Customer-related errors
    CustomerNotFound(Uuid),
    CustomerDeceased { customer_id: Uuid, date_of_death: NaiveDate },
    CustomerBlacklisted { customer_id: Uuid, blacklist_reason: String },
    
    // Transaction-related errors
    TransactionLimitExceeded { limit: Decimal, attempted: Decimal, limit_type: LimitType },
    InvalidSignature { required_signatories: Vec<Uuid>, provided_signatories: Vec<Uuid> },
    ApprovalRequired { transaction_id: Uuid, required_approvers: Vec<Uuid> },
    
    // Compliance-related errors
    ComplianceViolation { violation_type: String, customer_id: Option<Uuid> },
    KycIncomplete { customer_id: Uuid, missing_documents: Vec<String> },
    SanctionsMatch { customer_id: Uuid, match_details: String },
    
    // Product and system errors
    InvalidProductCode(String),
    ProductCatalogUnavailable { product_code: String, fallback_used: bool },
    BusinessDayCalculationError { date: NaiveDate, jurisdiction: String },
    
    // Network and infrastructure
    NetworkError { error_details: String, retry_possible: bool },
    DatabaseConstraintViolation { constraint: String, details: String },
}

pub enum LimitType { Daily, Monthly, PerTransaction, Terminal, Branch, Network }
```

### Comprehensive Audit Trail
- **Immutable transaction records** with cryptographic hash chaining
- **User attribution** for every operation (user, terminal, channel, timestamp)
- **Before/after state capture** for all account balance changes
- **Compliance event logging** with structured data for regulatory reporting
- **Performance metrics** for SLA monitoring and system optimization

## Testing Requirements

### Integration Tests with Realistic Scenarios
Create comprehensive integration tests in `banking-logic/tests/` covering:

1. **Complete Account Lifecycle**:
   - Account opening with KYC verification
   - Transaction processing across all channels
   - Interest accrual and posting over multiple periods
   - Account closure and settlement

2. **Multi-Signature Workflows**:
   - Joint account transaction authorization
   - Corporate mandate verification
   - Approval timeout handling

3. **Interest and Fee Calculations**:
   - Tiered interest rate application
   - Product-specific fee schedules
   - End-of-day processing accuracy

4. **Compliance Scenarios**:
   - Sanctions screening workflows
   - UBO verification chains
   - Transaction monitoring alerts

5. **Agent Network Operations**:
   - Hierarchical limit enforcement
   - GL code generation and settlement
   - Daily volume tracking and resets

6. **Business Calendar Integration**:
   - Holiday-aware date shifting
   - Interest accrual vs. posting differentiation
   - Weekend and holiday transaction handling

7. **Cross-Database Compatibility**:
   - Identical behavior across PostgreSQL and MariaDB
   - Migration script validation
   - Performance benchmarking

### Test Data with




 i generally do not like using Strings, in particular variable length strings, as they force ma struct to  │
│   be initialized on the heap. Are there string here that can be converted to                                │
│   - UUID                                                                                                    │
│   - Blake3 hash                                                                                             │
│   - Or limited length chain of character                                                                    │
│   My intention is to have most of the structs initialized and held on hte stack.  
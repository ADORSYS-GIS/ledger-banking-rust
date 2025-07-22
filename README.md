# Ledger Banking Rust - Multi-Product Core Banking System

A comprehensive, high-performance Rust library for core banking account management designed to support diverse banking products, extensive agent banking networks, and strict regulatory compliance (COBAC). The system follows API-first, cloud-native, and composable architecture principles.

## Architecture Overview

The system implements a modular **onion architecture** with clear separation of concerns, mirroring the design patterns of modern financial systems:

```
┌─────────────────────────────────────────────────────────────┐
│                    Banking Applications                      │
├─────────────────────────────────────────────────────────────┤
│               banking-logic (Business Logic)                │
├─────────────────────────────────────────────────────────────┤
│                 banking-api (Domain & Services)             │
├─────────────────────────────────────────────────────────────┤
│    banking-db-postgres  │  banking-db-mariadb  │ banking-db │
└─────────────────────────────────────────────────────────────┘
```

## Workspace Structure

### Core Crates (✅ Implemented)

- **`banking-api`**: Public interface with domain objects and service traits
- **`banking-db`**: Database abstraction with models and repository traits
- **`banking-logic`**: Business logic implementation with services and mappers (🚧 In Progress)
- **`banking-db-postgres`**: PostgreSQL implementation (🚧 Pending)
- **`banking-db-mariadb`**: MariaDB implementation (🚧 Pending)

### Banking-Specific Crates (🚧 Pending)

- **`banking-rules`**: Zen Engine integration for business rules management
- **`banking-compliance`**: KYC/AML compliance engine  
- **`banking-channels`**: Multi-channel transaction processing
- **`banking-calendar`**: Business calendar and holiday management

## Domain Model

### 1. Customer Information File (CIF)
Complete customer master repository with:
- Non-recyclable UUID identifiers
- Risk rating management (read-only, compliance-controlled)
- Comprehensive audit trails
- Status-based permission controls

### 2. Unified Account Model (UAM)
Single flexible account structure supporting:
- **Savings Accounts**: Interest-bearing with tiered rates
- **Current Accounts**: Transaction-focused with overdraft facilities
- **Loan Accounts**: Complete loan lifecycle management
- Enhanced lifecycle management with dormancy detection
- Multi-signature authorization workflows

### 3. Agent Banking Network
Hierarchical agent network structure with:
- **Networks**: Top-level partner organizations
- **Branches**: Regional/local branch hierarchy
- **Terminals**: Individual transaction points
- Cascading limit validation (Terminal → Branch → Network)
- GL code auto-generation for settlement

### 4. Account Relations
Granular relationship management:
- **Ownership**: Legal title and percentage holdings
- **Relationships**: Internal servicing assignments
- **Mandates**: Operational rights and permissions
- **UBO (Ultimate Beneficial Owner)**: KYC/AML compliance for corporate accounts

### 5. Transaction Processing
Comprehensive transaction model with:
- Multi-stage validation pipeline
- Business day awareness
- Risk scoring integration
- Multi-party approval workflows
- Immutable audit trails

### 6. Enhanced Features (From Banking Enhancements)
- **Account Lifecycle Management**: Complete workflow-driven account management
- **Workflow Engine**: Multi-step process automation
- **Dormancy Management**: Automated detection and processing
- **Transaction Processing Pipeline**: High-performance validation
- **Interest Calculation Engine**: Product catalog-driven calculations
- **General Ledger Integration**: Automated journal entry generation

## Service Architecture

### Core Services Implemented

1. **CustomerService**: CIF management with 360-degree customer view
2. **AccountService**: UAM operations with product rule integration
3. **TransactionService**: Multi-level validation and processing
4. **InterestService**: Product catalog-driven calculations
5. **CalendarService**: Business day calculations
6. **HierarchyService**: Agent network management
7. **ComplianceService**: KYC/AML enforcement
8. **ChannelProcessor**: Multi-channel transaction processing
9. **EodService**: Business day aware batch processing
10. **AccountLifecycleService**: Enhanced workflow-driven operations

## Key Features

### 🏦 Multi-Product Support
- Savings, Current, and Loan accounts in unified model
- Product catalog-driven business rules
- Tiered interest rate calculations

### 🌐 Agent Banking Network
- Hierarchical network management
- Cascading transaction limits
- Automated GL code generation
- Real-time settlement processing

### ✅ Regulatory Compliance
- COBAC compliance framework
- KYC/AML automated screening
- Ultimate Beneficial Owner verification
- Suspicious Activity Report (SAR) generation
- Immutable audit trails

### 🔄 Workflow Management
- Account opening workflows
- Multi-signature authorization
- Closure and settlement processes
- Dormancy detection and reactivation
- Timeout handling

### 📊 Business Calendar Integration
- Holiday-aware date calculations
- Business day shifting rules
- Multi-jurisdiction support
- Interest accrual vs. posting differentiation

### 🚀 Performance & Scalability
- High-performance caching with `moka`
- Parallel validation processing
- Batch operations for EOD processing
- Database abstraction for PostgreSQL/MariaDB

## Technology Stack

- **Core Language**: Rust 2021 Edition
- **Async Runtime**: Tokio
- **Database**: SQLx + Diesel (PostgreSQL/MariaDB support)
- **Caching**: Moka (high-performance async cache)
- **Business Rules**: Zen Engine integration
- **HTTP Client**: reqwest (for external integrations)
- **Serialization**: Serde with JSON support
- **Decimal Math**: rust_decimal (financial precision)
- **Error Handling**: thiserror + anyhow
- **Validation**: validator

## Database Schema

The system includes comprehensive migration files for both PostgreSQL and MariaDB with:

### Core Tables
- `customers` - Customer master data
- `accounts` - Unified account model
- `transactions` - Transaction records
- `agent_networks`, `agency_branches`, `agent_terminals` - Agent hierarchy
- `account_ownership`, `account_relationships`, `account_mandates` - Account relations

### Enhanced Tables (From Enhancements)
- `account_workflows` - Workflow tracking
- `workflow_step_records` - Step-by-step audit
- `account_status_history` - Immutable status audit
- `account_final_settlements` - Closure records
- `ultimate_beneficiaries` - UBO compliance
- `bank_holidays` - Business calendar
- `compliance_alerts` - KYC/AML monitoring

### Key Constraints
- Foreign key relationships with cascade options
- Check constraints for balance consistency
- Unique indexes for performance optimization
- Trigger-based audit trails
- Row-level security for multi-tenancy

## Integration Points

### External Systems
- **Product Catalog API**: Rules and parameters (SLA: <100ms)
- **Risk & Compliance API**: Customer risk rating updates
- **Payment Switch**: ISO 8583 message handling
- **Mobile Money APIs**: Multi-provider support
- **General Ledger**: Automated journal entries

## Current Implementation Status

### ✅ Completed
1. **Workspace Setup**: Full Cargo workspace configuration
2. **Domain Models**: Complete domain object definitions
3. **Service Traits**: Comprehensive service interface definitions
4. **Error Handling**: Banking-specific error taxonomy
5. **Database Models**: Core entity models for persistence

### 🚧 In Progress
1. **Database Repositories**: Repository trait implementations
2. **Business Logic**: Service implementations with rule validation

### 🚧 Pending
1. **Database Implementations**: PostgreSQL and MariaDB specific code
2. **Specialized Crates**: Rules, compliance, channels, calendar modules  
3. **Migration Scripts**: Database schema creation scripts
4. **Integration Tests**: Comprehensive test scenarios
5. **Performance Optimizations**: Caching and batch processing implementations

## Getting Started

```bash
# Clone the repository
git clone <repository-url>
cd ledger-banking-rust

# Build the workspace
cargo build

# Run tests
cargo test

# Check all crates
cargo check --workspace
```

## Development Guidelines

### Code Organization
- **Domain Logic**: Pure business logic in `banking-api`
- **Data Access**: Repository patterns in `banking-db`
- **Business Rules**: External rule engine integration in `banking-rules`
- **Infrastructure**: Database-specific implementations in `banking-db-*`

### Testing Strategy
- **Unit Tests**: Individual component testing
- **Integration Tests**: Cross-component workflows
- **Performance Tests**: Load and stress testing
- **Compliance Tests**: Regulatory requirement validation

### Security Principles
- Immutable transaction records
- Cryptographic audit trail chaining
- Role-based access control
- Input validation at all boundaries
- Secure credential management

## Contributing

1. Follow Rust best practices and idioms
2. Maintain comprehensive test coverage
3. Document all public APIs
4. Ensure backwards compatibility
5. Follow the established architecture patterns

## License

MIT OR Apache-2.0

## Compliance Notice

This system is designed to meet COBAC (Central African Banking Commission) regulatory requirements and includes comprehensive audit trails, KYC/AML screening, and reporting capabilities. Always consult with compliance experts for production deployment.
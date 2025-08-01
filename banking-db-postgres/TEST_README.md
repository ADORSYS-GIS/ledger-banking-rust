# AccountRepositoryImpl Testing Guide

This guide explains how to test the newly implemented AccountRepositoryImpl.

## Test Types

### 1. Unit Tests (No Database Required)
These tests validate the domain models, enum conversions, and business logic without requiring a database connection.

```bash
# Run unit tests only
cargo test --package banking-db-postgres unit_tests
```

**What's tested:**
- AccountModel creation and validation
- HeaplessString constraints 
- Enum conversions and debug output
- Decimal precision for financial calculations
- UUID generation
- Date validation
- Optional field handling
- Account lifecycle management

### 2. Integration Tests (Requires PostgreSQL)
These tests validate the actual database operations and require a running PostgreSQL instance.

#### Prerequisites
- Docker and Docker Compose installed
- PostgreSQL database running

#### Setup Database
```bash
# Start PostgreSQL with Docker Compose
docker-compose up -d postgres

# Wait for PostgreSQL to be ready
until docker-compose exec postgres pg_isready -U user -d mydb; do
    echo "Waiting for PostgreSQL..."
    sleep 2
done
```

#### Run Integration Tests
```bash
# Set environment variables
export DATABASE_URL="postgresql://user:password@localhost:5432/mydb"

# Run migrations first
cd banking-db-postgres
sqlx migrate run --source migrations

# Run integration tests
cargo test --features postgres_tests --test account_repository_tests -- --test-threads=1
```

#### Automated Setup
Use the provided script for automated setup:
```bash
# Run the test setup script
./test_setup.sh
```

## Test Coverage

### ✅ CRUD Operations
- [x] Create account
- [x] Update account
- [x] Find by ID
- [x] Check existence
- [x] Count accounts
- [x] List with pagination

### ✅ Balance Management
- [x] Update balance
- [x] Update accrued interest
- [x] Reset accrued interest
- [x] Update last activity date

### ✅ Status Management
- [x] Update account status
- [x] Status history tracking
- [x] Audit trail creation

### ✅ Advanced Queries
- [x] Find by customer ID
- [x] Find by product code
- [x] Find by status
- [x] Find dormancy candidates
- [x] Find pending closure accounts
- [x] Find interest-bearing accounts

### ✅ Related Entity Operations
- [x] Account ownership management
- [x] Account relationships
- [x] Account mandates
- [x] Account holds
- [x] Final settlements
- [x] Status change history

### ✅ Data Validation
- [x] Enum string conversion
- [x] HeaplessString length validation
- [x] UUID format validation
- [x] Decimal precision handling
- [x] Date validation

## Test Data Examples

### Savings Account
```rust
AccountModel {
    product_code: "SAV01",
    account_type: AccountType::Savings,
    current_balance: Decimal::from_str("1000.00"),
    available_balance: Decimal::from_str("950.00"),
    accrued_interest: Decimal::from_str("12.50"),
    overdraft_limit: None,
    // ... loan fields are None
}
```

### Loan Account
```rust
AccountModel {
    product_code: "LON01", 
    account_type: AccountType::Loan,
    current_balance: Decimal::from_str("-5000.00"), // Negative
    original_principal: Some(Decimal::from_str("10000.00")),
    outstanding_principal: Some(Decimal::from_str("5000.00")),
    loan_interest_rate: Some(Decimal::from_str("0.12")), // 12%
    loan_term_months: Some(24),
    // ... other loan fields
}
```

### Current Account with Overdraft
```rust
AccountModel {
    product_code: "CUR01",
    account_type: AccountType::Current,
    current_balance: Decimal::from_str("-100.00"), // Overdrawn
    available_balance: Decimal::from_str("400.00"), // From overdraft
    overdraft_limit: Some(Decimal::from_str("500.00")),
    // ... loan fields are None
}
```

## Performance Considerations

### Database Constraints Tested
- Balance consistency checks
- Loan field validation
- Currency code format validation
- Status transition rules

### Memory Optimization
- HeaplessString usage for bounded text fields
- Stack allocation for small strings
- Efficient enum representations

## Troubleshooting

### Common Issues

**1. Database Connection Failed**
```
Error: Failed to connect to PostgreSQL database
```
- Ensure PostgreSQL is running: `docker-compose up -d postgres`
- Check connection string: `postgresql://user:password@localhost:5432/mydb`

**2. Migration Failed**
```
Error: Failed to run migrations
```
- Ensure migrations directory exists: `banking-db-postgres/migrations`
- Run manually: `sqlx migrate run --source migrations`

**3. Feature Flag Missing**
```
Error: could not find `repository` in `banking_db_postgres`
```
- Ensure tests run with feature flag: `--features postgres_tests`

**4. Test Database Conflicts**
```
Error: relation "accounts" already exists
```
- Clean up database: `docker-compose down -v && docker-compose up -d postgres`
- Use single-threaded tests: `-- --test-threads=1`

## Continuous Integration

For CI/CD pipelines, add these steps:

```yaml
# .github/workflows/test.yml
- name: Start PostgreSQL
  run: docker-compose up -d postgres

- name: Wait for PostgreSQL
  run: |
    until docker-compose exec postgres pg_isready -U user -d mydb; do
      sleep 2
    done

- name: Run migrations
  run: |
    cd banking-db-postgres
    sqlx migrate run --source migrations
  env:
    DATABASE_URL: postgresql://user:password@localhost:5432/mydb

- name: Run tests
  run: cargo test --features postgres_tests --test account_repository_tests -- --test-threads=1
  env:
    DATABASE_URL: postgresql://user:password@localhost:5432/mydb
```

## Next Steps

After verifying AccountRepositoryImpl works correctly:

1. **TransactionRepositoryImpl** - Build on account operations
2. **ComplianceRepositoryImpl** - Add regulatory features  
3. **WorkflowRepositoryImpl** - Process automation
4. **Integration Testing** - Cross-repository operations
5. **Performance Testing** - Load and stress testing

The AccountRepositoryImpl provides the foundation for all other banking operations.
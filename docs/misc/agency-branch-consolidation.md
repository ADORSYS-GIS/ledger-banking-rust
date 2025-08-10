# Agency Branch Consolidation Instructions

## Overview
Consolidate `CashPickupLocation` functionality into the existing `AgencyBranch` struct, enriching it with location-specific fields while maintaining the agent network hierarchy. All references to `cash_pickup_location` will point to `AgencyBranch` entities.

## Step 1: Enrich AgencyBranch Struct

Update the `AgencyBranch` struct in your agent network module:

```rust
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use heapless::{String as HeaplessString, Vec as HeaplessVec};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgencyBranch {
    // === EXISTING FIELDS ===
    pub branch_id: Uuid,
    pub network_id: Uuid,
    pub parent_branch_id: Option<Uuid>,
    pub branch_name: HeaplessString<255>,
    pub branch_code: HeaplessString<8>,
    pub branch_level: i32,
    pub gl_code_prefix: HeaplessString<6>,
    pub status: BranchStatus,
    pub daily_transaction_limit: Decimal,
    pub current_daily_volume: Decimal,
    pub max_cash_limit: Decimal,
    pub current_cash_balance: Decimal,
    pub minimum_cash_balance: Decimal,
    pub created_at: DateTime<Utc>,
    
    // === NEW LOCATION FIELDS ===
    // Physical address
    pub address: Address,
    pub gps_coordinates: Option<GpsCoordinates>,
    pub landmark_description: Option<HeaplessString<200>>,
    
    // Operational details
    pub operating_hours: OperatingHours,
    pub holiday_schedule: HeaplessVec<HolidaySchedule, 20>,
    pub temporary_closure: Option<TemporaryClosure>,
    
    // Contact information
    pub primary_phone: HeaplessString<20>,
    pub secondary_phone: Option<HeaplessString<20>>,
    pub email: Option<HeaplessString<100>>,
    pub branch_manager_id: Option<Uuid>,
    
    // Services and capabilities
    pub branch_type: BranchType,  // Replaces LocationType
    pub supported_services: HeaplessVec<ServiceType, 20>,
    pub supported_currencies: HeaplessVec<[u8; 3], 10>,
    pub languages_spoken: HeaplessVec<[u8; 3], 5>,
    
    // Security and access
    pub security_features: SecurityFeatures,
    pub accessibility_features: AccessibilityFeatures,
    pub required_documents: HeaplessVec<RequiredDocument, 10>,
    
    // Customer capacity
    pub max_daily_customers: Option<u32>,
    pub average_wait_time_minutes: Option<u16>,
    
    // Transaction limits (enhanced from existing)
    pub per_transaction_limit: Decimal,
    pub monthly_transaction_limit: Option<Decimal>,
    
    // Compliance and risk
    pub risk_rating: RiskRating,
    pub last_audit_date: Option<NaiveDate>,
    pub compliance_certifications: HeaplessVec<ComplianceCert, 5>,
    
    // Metadata
    pub last_updated_at: DateTime<Utc>,
    pub updated_by_person_id: HeaplessString<100>,
}

// Extend BranchStatus if needed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BranchStatus { 
    Active, 
    Suspended, 
    Closed,
    TemporarilyClosed,  // New status
}

// New enum to replace LocationType
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BranchType {
    MainBranch,
    SubBranch,
    AgentOutlet,
    StandaloneKiosk,
    PartnerAgent,
    ATMLocation,
    MobileUnit,
}

// Supporting structs (add to the module)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street_line1: HeaplessString<100>,
    pub street_line2: Option<HeaplessString<100>>,
    pub city: HeaplessString<50>,
    pub state_province: HeaplessString<50>,
    pub postal_code: HeaplessString<20>,
    pub country_code: [u8; 2],
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GpsCoordinates {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy_meters: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatingHours {
    pub monday: Option<DayHours>,
    pub tuesday: Option<DayHours>,
    pub wednesday: Option<DayHours>,
    pub thursday: Option<DayHours>,
    pub friday: Option<DayHours>,
    pub saturday: Option<DayHours>,
    pub sunday: Option<DayHours>,
    pub timezone: HeaplessString<50>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DayHours {
    pub open_time: NaiveTime,
    pub close_time: NaiveTime,
    pub break_start: Option<NaiveTime>,
    pub break_end: Option<NaiveTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityFeatures {
    pub has_security_guard: bool,
    pub has_cctv: bool,
    pub has_panic_button: bool,
    pub has_safe: bool,
    pub has_biometric_verification: bool,
    pub police_station_distance_km: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityFeatures {
    pub wheelchair_accessible: bool,
    pub has_ramp: bool,
    pub has_braille_signage: bool,
    pub has_audio_assistance: bool,
    pub has_sign_language_support: bool,
    pub parking_available: bool,
    pub public_transport_nearby: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ServiceType {
    CashWithdrawal,
    CashDeposit,
    CashTransfer,
    BillPayment,
    AccountOpening,
    CardServices,
    CheckDeposit,
    ForeignExchange,
    RemittanceCollection,
    AgentBanking,  // Additional service
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RiskRating {
    Low,
    Medium,
    High,
    Critical,
}

// Additional types as needed...
```

## Step 2: Update AgencyBranch Implementation

Enhance the impl block with new methods:

```rust
impl AgencyBranch {
    // === EXISTING METHODS ===
    pub fn set_branch_code(&mut self, code: &str) -> Result<(), &'static str> {
        self.branch_code = HeaplessString::try_from(code)
            .map_err(|_| "Branch code too long")?;
        Ok(())
    }
    
    pub fn set_gl_code_prefix(&mut self, prefix: &str) -> Result<(), &'static str> {
        self.gl_code_prefix = HeaplessString::try_from(prefix)
            .map_err(|_| "GL code prefix too long")?;
        Ok(())
    }
    
    // === NEW METHODS ===
    /// Check if branch can be used for cash pickup
    pub fn is_cash_pickup_enabled(&self) -> bool {
        self.status == BranchStatus::Active
            && self.supported_services.contains(&ServiceType::CashWithdrawal)
            && self.current_cash_balance > self.minimum_cash_balance
    }
    
    /// Check if branch is currently open
    pub fn is_open_now(&self, current_time: DateTime<Utc>) -> bool {
        // Convert to branch timezone and check operating hours
        // Implementation depends on timezone handling library
        true // Placeholder
    }
    
    /// Validate cash operation against limits
    pub fn validate_cash_operation(
        &self, 
        amount: Decimal, 
        operation: CashOperationType
    ) -> CashLimitValidation {
        match operation {
            CashOperationType::Withdrawal | CashOperationType::CashOut => {
                if amount > self.current_cash_balance {
                    CashLimitValidation::InsufficientCash {
                        available: self.current_cash_balance,
                        required: amount,
                    }
                } else if self.current_cash_balance - amount < self.minimum_cash_balance {
                    CashLimitValidation::BelowMinimum {
                        current: self.current_cash_balance - amount,
                        minimum: self.minimum_cash_balance,
                    }
                } else if amount > self.per_transaction_limit {
                    CashLimitValidation::ExceedsMaxLimit {
                        current: amount,
                        max_limit: self.per_transaction_limit,
                    }
                } else {
                    CashLimitValidation::Approved
                }
            }
            CashOperationType::Deposit | CashOperationType::CashIn => {
                if self.current_cash_balance + amount > self.max_cash_limit {
                    CashLimitValidation::ExceedsMaxLimit {
                        current: self.current_cash_balance + amount,
                        max_limit: self.max_cash_limit,
                    }
                } else {
                    CashLimitValidation::Approved
                }
            }
        }
    }
    
    /// Get formatted address
    pub fn get_formatted_address(&self) -> String {
        format!(
            "{}, {}, {} {}, {}",
            self.address.street_line1,
            self.address.city,
            self.address.state_province,
            self.address.postal_code,
            std::str::from_utf8(&self.address.country_code).unwrap_or("??")
        )
    }
    
    /// Check if branch supports a specific service
    pub fn supports_service(&self, service: ServiceType) -> bool {
        self.supported_services.contains(&service)
    }
}
```

## Step 3: Find and Replace References

### Search for these patterns in your codebase:
```rust
// Common field names to replace:
cash_pickup_location: Option<HeaplessString<N>>
cash_pickup_location: Option<String>
pickup_location: Option<HeaplessString<N>>
pickup_location: Option<String>
pickup_location_id: Option<Uuid>
cash_collection_point: Option<String>
```

### Replace with:
```rust
/// References AgencyBranch.branch_id for cash pickup
pub cash_pickup_branch_id: Option<Uuid>,
```

### Example transformations:

```rust
// === BEFORE ===
pub struct Transaction {
    pub id: Uuid,
    pub cash_pickup_location: Option<HeaplessString<100>>,
    pub amount: Decimal,
}

// === AFTER ===
pub struct Transaction {
    pub id: Uuid,
    /// References AgencyBranch.branch_id for cash pickup
    pub cash_pickup_branch_id: Option<Uuid>,
    pub amount: Decimal,
}

// === BEFORE ===
pub struct DisbursementInstruction {
    pub id: Uuid,
    pub pickup_location: Option<String>,
    pub pickup_address: Option<String>,
}

// === AFTER ===
pub struct DisbursementInstruction {
    pub id: Uuid,
    /// References AgencyBranch.branch_id for cash pickup
    pub cash_pickup_branch_id: Option<Uuid>,
    // Remove pickup_address as it's now in AgencyBranch
}
```

## Step 4: Create Repository Extensions

Add methods to work with branches as pickup locations:

```rust
impl AgencyBranchRepository {
    /// Get all branches that support cash pickup
    pub fn get_cash_pickup_branches(&self) -> Vec<&AgencyBranch> {
        self.branches.values()
            .filter(|b| b.is_cash_pickup_enabled())
            .collect()
    }
    
    /// Find nearest cash pickup branch
    pub fn find_nearest_branch(
        &self, 
        coordinates: GpsCoordinates,
        service_type: ServiceType
    ) -> Option<&AgencyBranch> {
        self.branches.values()
            .filter(|b| b.is_cash_pickup_enabled() && b.supports_service(service_type))
            .filter_map(|b| {
                b.gps_coordinates.map(|coords| {
                    let distance = calculate_distance(coordinates, coords);
                    (b, distance)
                })
            })
            .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
            .map(|(branch, _)| branch)
    }
    
    /// Get branches by risk rating
    pub fn get_branches_by_risk(&self, rating: RiskRating) -> Vec<&AgencyBranch> {
        self.branches.values()
            .filter(|b| b.risk_rating == rating)
            .collect()
    }
}
```

## Step 5: Create View Models

For API responses that need to include branch details:

```rust
#[derive(Serialize, Deserialize)]
pub struct TransactionWithBranchView {
    pub transaction_id: Uuid,
    pub amount: Decimal,
    pub cash_pickup_branch: Option<BranchSummary>,
}

#[derive(Serialize, Deserialize)]
pub struct BranchSummary {
    pub branch_id: Uuid,
    pub branch_name: String,
    pub branch_code: String,
    pub branch_type: BranchType,
    pub address: String,
    pub is_open_now: bool,
    pub services_available: Vec<ServiceType>,
    pub wait_time_minutes: Option<u16>,
}

impl BranchSummary {
    pub fn from_branch(branch: &AgencyBranch, current_time: DateTime<Utc>) -> Self {
        Self {
            branch_id: branch.branch_id,
            branch_name: branch.branch_name.to_string(),
            branch_code: branch.branch_code.to_string(),
            branch_type: branch.branch_type,
            address: branch.get_formatted_address(),
            is_open_now: branch.is_open_now(current_time),
            services_available: branch.supported_services.to_vec(),
            wait_time_minutes: branch.average_wait_time_minutes,
        }
    }
}
```

## Step 6: Migration Strategy

1. **Add new fields to AgencyBranch table** with sensible defaults
2. **Create mapping** from old location strings to branch IDs:
   ```sql
   -- Create temporary mapping table
   CREATE TABLE location_to_branch_mapping (
       old_location_string VARCHAR(255),
       branch_id UUID,
       confidence_score DECIMAL
   );
   ```

3. **Update all references** using the mapping:
   ```sql
   UPDATE transactions t
   SET cash_pickup_branch_id = m.branch_id
   FROM location_to_branch_mapping m
   WHERE t.cash_pickup_location = m.old_location_string;
   ```

## Step 7: Benefits of Consolidation

1. **Unified hierarchy**: Cash pickup locations are now part of the agent network structure
2. **Inherited features**: Automatic cash balance tracking, daily limits, GL codes
3. **Network management**: Can manage pickup locations through existing agent network tools
4. **Reporting**: Unified reporting across all branch types
5. **Permissions**: Leverage existing branch-level permissions

## Testing Checklist

- [ ] AgencyBranch compiles with all new fields
- [ ] All cash_pickup_location references updated to branch_id
- [ ] Branch repository supports location-based queries
- [ ] Cash validation methods work correctly
- [ ] Operating hours and timezone handling works
- [ ] View models properly resolve branch data
- [ ] Migration script successfully maps old locations
- [ ] API maintains backward compatibility where needed
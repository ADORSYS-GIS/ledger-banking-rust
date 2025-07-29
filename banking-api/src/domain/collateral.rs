use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collateral {
    pub collateral_id: Uuid,
    pub collateral_type: CollateralType,
    pub collateral_category: CollateralCategory,
    pub description: HeaplessString<200>,
    pub external_reference: HeaplessString<100>,
    
    // Valuation information
    pub original_value: Decimal,
    pub current_market_value: Decimal,
    pub appraised_value: Option<Decimal>,
    pub currency: HeaplessString<3>,
    pub valuation_date: NaiveDate,
    pub next_valuation_date: Option<NaiveDate>,
    pub valuation_frequency_months: Option<i32>,
    
    // Collateral details
    pub pledged_value: Decimal,
    pub available_value: Decimal,
    pub lien_amount: Option<Decimal>,
    pub margin_percentage: Decimal,
    pub forced_sale_value: Option<Decimal>,
    
    // Location and custody
    pub custody_location: CustodyLocation,
    /// References Address.address_id for physical location
    pub physical_location: Option<Uuid>,
    pub custodian_details: Option<CustodianDetails>,
    
    // Legal and documentation
    /// References Person.person_id for legal title holder
    pub legal_title_holder: Uuid,
    pub perfection_status: PerfectionStatus,
    pub perfection_date: Option<NaiveDate>,
    pub perfection_expiry_date: Option<NaiveDate>,
    pub registration_number: Option<HeaplessString<100>>,
    /// References Person.person_id for registration authority
    pub registration_authority: Option<Uuid>,
    
    // Insurance and risk
    pub insurance_required: bool,
    pub insurance_coverage: Option<InsuranceCoverage>,
    pub risk_rating: CollateralRiskRating,
    pub environmental_risk: Option<EnvironmentalRisk>,
    
    // Status and lifecycle
    pub status: CollateralStatus,
    pub pledge_date: NaiveDate,
    pub release_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    
    // Audit and tracking
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    /// References Person.person_id
    pub created_by: Uuid,
    /// References Person.person_id
    pub updated_by: Uuid,
    pub last_valuation_by: Option<Uuid>,
    pub next_review_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollateralType {
    // Real Estate
    ResidentialProperty,
    CommercialProperty,
    IndustrialProperty,
    Land,
    
    // Vehicles
    PassengerVehicle,
    CommercialVehicle,
    Motorcycle,
    Boat,
    Aircraft,
    
    // Financial Assets
    CashDeposit,
    GovernmentSecurities,
    CorporateBonds,
    Stocks,
    MutualFunds,
    
    // Business Assets
    Inventory,
    AccountsReceivable,
    Equipment,
    Machinery,
    
    // Personal Property
    Jewelry,
    ArtAndAntiques,
    Electronics,
    
    // Commodities
    PreciousMetals,
    AgriculturalProducts,
    
    // Other
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollateralCategory {
    Immovable,
    Movable,
    Financial,
    Intangible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustodyLocation {
    BankVault,
    ThirdPartyCustodian,
    ClientPremises,
    RegisteredWarehouse,
    GovernmentRegistry,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustodianDetails {
    pub id: Uuid,
    pub person_id: Uuid,
    pub custodian_license_number: Option<HeaplessString<100>>,
    pub contact_person_id: Uuid,
    pub custody_agreement_reference: HeaplessString<100>,
    pub custody_fees: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerfectionStatus {
    Pending,
    Perfected,
    Imperfected,
    Expired,
    UnderDispute,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceCoverage {
    pub policy_number: HeaplessString<100>,
    pub insurance_company: HeaplessString<255>,
    pub coverage_amount: Decimal,
    pub premium_amount: Decimal,
    pub policy_start_date: NaiveDate,
    pub policy_end_date: NaiveDate,
    pub beneficiary: HeaplessString<255>,
    pub coverage_type: InsuranceCoverageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsuranceCoverageType {
    Comprehensive,
    FireAndTheft,
    AllRisk,
    SpecifiedPerils,
    MarineTransit,
    KeyPersonLife,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollateralRiskRating {
    Excellent,
    Good,
    Average,
    Poor,
    Unacceptable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalRisk {
    pub assessment_date: NaiveDate,
    pub risk_level: EnvironmentalRiskLevel,
    pub assessment_report_reference: HeaplessString<100>,
    pub remediation_required: bool,
    pub remediation_cost_estimate: Option<Decimal>,
    pub compliance_status: EnvironmentalComplianceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvironmentalRiskLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvironmentalComplianceStatus {
    Compliant,
    NonCompliant,
    UnderReview,
    RequiresRemediation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollateralStatus {
    Active,
    Released,
    PartiallyReleased,
    UnderReview,
    Disputed,
    Liquidated,
    Impaired,
    Substituted,
}

// Collateral valuation and monitoring structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollateralValuation {
    pub valuation_id: Uuid,
    pub collateral_id: Uuid,
    pub valuation_date: NaiveDate,
    pub valuation_method: ValuationMethod,
    pub market_value: Decimal,
    pub forced_sale_value: Option<Decimal>,
    pub appraiser_name: HeaplessString<255>,
    pub appraiser_license: Option<HeaplessString<100>>,
    pub valuation_report_reference: HeaplessString<100>,
    pub validity_period_months: i32,
    pub next_valuation_due: NaiveDate,
    pub valuation_notes: Option<HeaplessString<1000>>,
    
    // Audit trail
    pub created_at: DateTime<Utc>,
    /// References Person.person_id
    pub created_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValuationMethod {
    MarketComparison,
    IncomeApproach,
    CostApproach,
    ExpertAppraisal,
    AuctionValue,
    BookValue,
    MarkToMarket,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollateralPledge {
    pub pledge_id: Uuid,
    pub collateral_id: Uuid,
    pub loan_account_id: Uuid,
    pub pledged_amount: Decimal,
    pub pledge_percentage: Decimal,
    pub pledge_priority: PledgePriority,
    pub pledge_date: NaiveDate,
    pub release_conditions: Option<HeaplessString<500>>,
    pub status: PledgeStatus,
    
    // Legal documentation
    pub security_agreement_reference: HeaplessString<100>,
    pub pledge_registration_number: Option<HeaplessString<100>>,
    
    // Monitoring
    pub last_review_date: Option<NaiveDate>,
    pub next_review_date: Option<NaiveDate>,
    pub covenant_compliance: CovenantCompliance,
    
    // Audit trail
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    /// References Person.person_id
    pub created_by: Uuid,
    /// References Person.person_id
    pub updated_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PledgePriority {
    First,
    Second,
    Third,
    Subordinate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PledgeStatus {
    Active,
    Released,
    PartiallyReleased,
    Substituted,
    Disputed,
    UnderEnforcement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovenantCompliance {
    pub loan_to_value_ratio: Decimal,
    pub maximum_ltv_allowed: Decimal,
    pub debt_service_coverage: Option<Decimal>,
    pub minimum_dscr_required: Option<Decimal>,
    pub insurance_compliance: bool,
    pub maintenance_compliance: bool,
    pub reporting_compliance: bool,
    pub overall_compliance_status: CovenantComplianceStatus,
    pub last_compliance_check: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CovenantComplianceStatus {
    Compliant,
    MinorBreach,
    MaterialBreach,
    Critical,
}

// Collateral monitoring and alerts

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollateralAlert {
    pub alert_id: Uuid,
    pub collateral_id: Uuid,
    pub alert_type: CollateralAlertType,
    pub severity: AlertSeverity,
    pub message: HeaplessString<500>,
    pub trigger_date: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub status: CollateralAlertStatus,
    pub assigned_to: Option<Uuid>,
    pub resolution_notes: Option<HeaplessString<1000>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollateralAlertType {
    ValuationDue,
    ValuationOverdue,
    InsuranceExpiring,
    InsuranceExpired,
    PerfectionExpiring,
    PerfectionExpired,
    LtvBreach,
    CovenantBreach,
    MaintenanceRequired,
    DocumentationMissing,
    EnvironmentalRisk,
    MarketValueDecline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollateralAlertStatus {
    Open,
    InProgress,
    Resolved,
    Dismissed,
    Escalated,
}

// Collateral reporting and portfolio analysis

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollateralPortfolioSummary {
    pub portfolio_id: Uuid,
    pub as_of_date: NaiveDate,
    pub total_collateral_count: u32,
    pub total_market_value: Decimal,
    pub total_pledged_value: Decimal,
    pub total_available_value: Decimal,
    pub weighted_average_ltv: Decimal,
    pub collateral_concentration: Vec<ConcentrationAnalysis>,
    pub risk_distribution: Vec<RiskDistribution>,
    pub valuation_status: ValuationStatusSummary,
    pub covenant_compliance_summary: ComplianceSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcentrationAnalysis {
    pub category: ConcentrationCategory,
    pub category_value: HeaplessString<100>,
    pub count: u32,
    pub total_value: Decimal,
    pub percentage_of_portfolio: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConcentrationCategory {
    CollateralType,
    GeographicLocation,
    Industry,
    Borrower,
    CustodyLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskDistribution {
    pub risk_rating: CollateralRiskRating,
    pub count: u32,
    pub total_value: Decimal,
    pub percentage_of_portfolio: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationStatusSummary {
    pub current_valuations: u32,
    pub valuations_due_30_days: u32,
    pub overdue_valuations: u32,
    pub average_valuation_age_days: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceSummary {
    pub fully_compliant: u32,
    pub minor_breaches: u32,
    pub material_breaches: u32,
    pub critical_breaches: u32,
}

// Collateral liquidation and enforcement

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollateralEnforcement {
    pub enforcement_id: Uuid,
    pub collateral_id: Uuid,
    pub loan_account_id: Uuid,
    pub enforcement_type: EnforcementType,
    pub enforcement_date: NaiveDate,
    pub outstanding_debt: Decimal,
    pub estimated_recovery: Decimal,
    pub enforcement_method: EnforcementMethod,
    pub status: EnforcementStatus,
    /// References Person.person_id for legal counsel
    pub legal_counsel: Option<Uuid>,
    pub court_case_reference: Option<HeaplessString<100>>,
    pub expected_completion_date: Option<NaiveDate>,
    pub actual_completion_date: Option<NaiveDate>,
    pub recovery_amount: Option<Decimal>,
    pub enforcement_costs: Option<Decimal>,
    pub net_recovery: Option<Decimal>,
    
    // Audit trail
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    /// References Person.person_id
    pub created_by: Uuid,
    /// References Person.person_id
    pub updated_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnforcementType {
    PrivateSale,
    PublicAuction,
    CourtOrdered,
    SelfHelp,
    ForeClosureAction,
    Repossession,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnforcementMethod {
    DirectSale,
    AuctionHouse,
    OnlinePlatform,
    BrokerSale,
    CourtSale,
    AssetManagementCompany,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnforcementStatus {
    Initiated,
    InProgress,
    Completed,
    Suspended,
    Cancelled,
    UnderLegalReview,
}

impl Collateral {
    /// Calculate loan-to-value ratio for a given loan amount
    pub fn calculate_ltv(&self, loan_amount: Decimal) -> Decimal {
        if self.current_market_value.is_zero() {
            Decimal::MAX
        } else {
            (loan_amount / self.current_market_value) * Decimal::from(100)
        }
    }
    
    /// Check if collateral valuation is due
    pub fn is_valuation_due(&self, reference_date: NaiveDate) -> bool {
        self.next_valuation_date
            .map(|due_date| reference_date >= due_date)
            .unwrap_or(false)
    }
    
    /// Calculate available collateral value for additional pledging
    pub fn calculate_available_value(&self) -> Decimal {
        let margin_adjusted_value = self.current_market_value * (Decimal::ONE - self.margin_percentage / Decimal::from(100));
        margin_adjusted_value - self.pledged_value
    }
    
    /// Check insurance validity
    pub fn is_insurance_valid(&self, reference_date: NaiveDate) -> bool {
        if !self.insurance_required {
            return true;
        }
        
        self.insurance_coverage
            .as_ref()
            .map(|coverage| reference_date <= coverage.policy_end_date)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    
    #[test]
    fn test_ltv_calculation() {
        let collateral = Collateral {
            collateral_id: Uuid::new_v4(),
            collateral_type: CollateralType::ResidentialProperty,
            collateral_category: CollateralCategory::Immovable,
            description: HeaplessString::try_from("Test Property").unwrap(),
            external_reference: HeaplessString::try_from("REF001").unwrap(),
            original_value: Decimal::from(200000),
            current_market_value: Decimal::from(180000),
            appraised_value: Some(Decimal::from(175000)),
            currency: HeaplessString::try_from("USD").unwrap(),
            valuation_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            next_valuation_date: Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
            valuation_frequency_months: Some(12),
            pledged_value: Decimal::from(150000),
            available_value: Decimal::from(30000),
            lien_amount: None,
            margin_percentage: Decimal::from(20),
            forced_sale_value: Some(Decimal::from(160000)),
            custody_location: CustodyLocation::ClientPremises,
            physical_location: Some(Uuid::new_v4()),
            custodian_details: None,
            legal_title_holder: Uuid::new_v4(),
            perfection_status: PerfectionStatus::Perfected,
            perfection_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            perfection_expiry_date: None,
            registration_number: Some(HeaplessString::try_from("REG123").unwrap()),
            registration_authority: Some(Uuid::new_v4()),
            insurance_required: true,
            insurance_coverage: None,
            risk_rating: CollateralRiskRating::Good,
            environmental_risk: None,
            status: CollateralStatus::Active,
            pledge_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            release_date: None,
            maturity_date: None,
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
            created_by: Uuid::new_v4(),
            updated_by: Uuid::new_v4(),
            last_valuation_by: None,
            next_review_date: None,
        };
        
        let loan_amount = Decimal::from(144000);
        let ltv = collateral.calculate_ltv(loan_amount);
        assert_eq!(ltv, Decimal::from(80)); // 144000/180000 * 100 = 80%
    }
    
    #[test]
    fn test_available_value_calculation() {
        let collateral = Collateral {
            collateral_id: Uuid::new_v4(),
            collateral_type: CollateralType::ResidentialProperty,
            collateral_category: CollateralCategory::Immovable,
            description: HeaplessString::try_from("Test Property").unwrap(),
            external_reference: HeaplessString::try_from("REF001").unwrap(),
            original_value: Decimal::from(200000),
            current_market_value: Decimal::from(100000),
            appraised_value: None,
            currency: HeaplessString::try_from("USD").unwrap(),
            valuation_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            next_valuation_date: None,
            valuation_frequency_months: None,
            pledged_value: Decimal::from(50000),
            available_value: Decimal::ZERO,
            lien_amount: None,
            margin_percentage: Decimal::from(20), // 20% margin
            forced_sale_value: None,
            custody_location: CustodyLocation::ClientPremises,
            physical_location: None,
            custodian_details: None,
            legal_title_holder: Uuid::new_v4(),
            perfection_status: PerfectionStatus::Perfected,
            perfection_date: None,
            perfection_expiry_date: None,
            registration_number: None,
            registration_authority: None,
            insurance_required: false,
            insurance_coverage: None,
            risk_rating: CollateralRiskRating::Good,
            environmental_risk: None,
            status: CollateralStatus::Active,
            pledge_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            release_date: None,
            maturity_date: None,
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
            created_by: Uuid::new_v4(),
            updated_by: Uuid::new_v4(),
            last_valuation_by: None,
            next_review_date: None,
        };
        
        let available = collateral.calculate_available_value();
        // Market value 100000 * (1 - 0.20) - pledged 50000 = 80000 - 50000 = 30000
        assert_eq!(available, Decimal::from(30000));
    }
    
    #[test]
    fn test_valuation_due_check() {
        let collateral = Collateral {
            collateral_id: Uuid::new_v4(),
            collateral_type: CollateralType::ResidentialProperty,
            collateral_category: CollateralCategory::Immovable,
            description: HeaplessString::try_from("Test Property").unwrap(),
            external_reference: HeaplessString::try_from("REF001").unwrap(),
            original_value: Decimal::from(200000),
            current_market_value: Decimal::from(180000),
            appraised_value: None,
            currency: HeaplessString::try_from("USD").unwrap(),
            valuation_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            next_valuation_date: Some(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap()),
            valuation_frequency_months: Some(6),
            pledged_value: Decimal::ZERO,
            available_value: Decimal::ZERO,
            lien_amount: None,
            margin_percentage: Decimal::from(10),
            forced_sale_value: None,
            custody_location: CustodyLocation::ClientPremises,
            physical_location: None,
            custodian_details: None,
            legal_title_holder: Uuid::new_v4(),
            perfection_status: PerfectionStatus::Perfected,
            perfection_date: None,
            perfection_expiry_date: None,
            registration_number: None,
            registration_authority: None,
            insurance_required: false,
            insurance_coverage: None,
            risk_rating: CollateralRiskRating::Good,
            environmental_risk: None,
            status: CollateralStatus::Active,
            pledge_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            release_date: None,
            maturity_date: None,
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
            created_by: Uuid::new_v4(),
            updated_by: Uuid::new_v4(),
            last_valuation_by: None,
            next_review_date: None,
        };
        
        let before_due = NaiveDate::from_ymd_opt(2024, 5, 15).unwrap();
        let after_due = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        
        assert!(!collateral.is_valuation_due(before_due));
        assert!(collateral.is_valuation_due(after_due));
    }
}
use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Database model for collateral type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "collateral_type", rename_all = "PascalCase")]
pub enum CollateralType {
    ResidentialProperty,
    CommercialProperty,
    IndustrialProperty,
    Land,
    PassengerVehicle,
    CommercialVehicle,
    Motorcycle,
    Boat,
    Aircraft,
    CashDeposit,
    GovernmentSecurities,
    CorporateBonds,
    Stocks,
    MutualFunds,
    Inventory,
    AccountsReceivable,
    Equipment,
    Machinery,
    Jewelry,
    ArtAndAntiques,
    Electronics,
    PreciousMetals,
    AgriculturalProducts,
    Other,
}

/// Database model for collateral category enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "collateral_category", rename_all = "PascalCase")]
pub enum CollateralCategory {
    Immovable,
    Movable,
    Financial,
    Intangible,
}

/// Database model for custody location enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "custody_location", rename_all = "PascalCase")]
pub enum CustodyLocation {
    BankVault,
    ThirdPartyCustodian,
    ClientPremises,
    RegisteredWarehouse,
    GovernmentRegistry,
    Other,
}

/// Database model for perfection status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "perfection_status", rename_all = "PascalCase")]
pub enum PerfectionStatus {
    Pending,
    Perfected,
    Imperfected,
    Expired,
    UnderDispute,
}

/// Database model for collateral risk rating enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "collateral_risk_rating", rename_all = "PascalCase")]
pub enum CollateralRiskRating {
    Excellent,
    Good,
    Average,
    Poor,
    Unacceptable,
}

// Display implementations for collateral enums
impl std::fmt::Display for CollateralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CollateralType::ResidentialProperty => write!(f, "residentialproperty"),
            CollateralType::CommercialProperty => write!(f, "commercialproperty"),
            CollateralType::IndustrialProperty => write!(f, "industrialproperty"),
            CollateralType::Land => write!(f, "land"),
            CollateralType::PassengerVehicle => write!(f, "passengervehicle"),
            CollateralType::CommercialVehicle => write!(f, "commercialvehicle"),
            CollateralType::Motorcycle => write!(f, "motorcycle"),
            CollateralType::Boat => write!(f, "boat"),
            CollateralType::Aircraft => write!(f, "aircraft"),
            CollateralType::CashDeposit => write!(f, "cashdeposit"),
            CollateralType::GovernmentSecurities => write!(f, "governmentsecurities"),
            CollateralType::CorporateBonds => write!(f, "corporatebonds"),
            CollateralType::Stocks => write!(f, "stocks"),
            CollateralType::MutualFunds => write!(f, "mutualfunds"),
            CollateralType::Inventory => write!(f, "inventory"),
            CollateralType::AccountsReceivable => write!(f, "accountsreceivable"),
            CollateralType::Equipment => write!(f, "equipment"),
            CollateralType::Machinery => write!(f, "machinery"),
            CollateralType::Jewelry => write!(f, "jewelry"),
            CollateralType::ArtAndAntiques => write!(f, "artandantiques"),
            CollateralType::Electronics => write!(f, "electronics"),
            CollateralType::PreciousMetals => write!(f, "preciousmetals"),
            CollateralType::AgriculturalProducts => write!(f, "agriculturalproducts"),
            CollateralType::Other => write!(f, "other"),
        }
    }
}

impl std::fmt::Display for CollateralCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CollateralCategory::Immovable => write!(f, "immovable"),
            CollateralCategory::Movable => write!(f, "movable"),
            CollateralCategory::Financial => write!(f, "financial"),
            CollateralCategory::Intangible => write!(f, "intangible"),
        }
    }
}

impl std::fmt::Display for CustodyLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustodyLocation::BankVault => write!(f, "bankvault"),
            CustodyLocation::ThirdPartyCustodian => write!(f, "thirdpartycustodian"),
            CustodyLocation::ClientPremises => write!(f, "clientpremises"),
            CustodyLocation::RegisteredWarehouse => write!(f, "registeredwarehouse"),
            CustodyLocation::GovernmentRegistry => write!(f, "governmentregistry"),
            CustodyLocation::Other => write!(f, "other"),
        }
    }
}

impl std::fmt::Display for PerfectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PerfectionStatus::Pending => write!(f, "pending"),
            PerfectionStatus::Perfected => write!(f, "perfected"),
            PerfectionStatus::Imperfected => write!(f, "imperfected"),
            PerfectionStatus::Expired => write!(f, "expired"),
            PerfectionStatus::UnderDispute => write!(f, "underdispute"),
        }
    }
}

impl std::fmt::Display for CollateralRiskRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CollateralRiskRating::Excellent => write!(f, "excellent"),
            CollateralRiskRating::Good => write!(f, "good"),
            CollateralRiskRating::Average => write!(f, "average"),
            CollateralRiskRating::Poor => write!(f, "poor"),
            CollateralRiskRating::Unacceptable => write!(f, "unacceptable"),
        }
    }
}

/// Database model for collateral status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "collateral_status", rename_all = "PascalCase")]
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

use std::str::FromStr;

impl FromStr for CollateralStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(CollateralStatus::Active),
            "Released" => Ok(CollateralStatus::Released),
            "PartiallyReleased" => Ok(CollateralStatus::PartiallyReleased),
            "UnderReview" => Ok(CollateralStatus::UnderReview),
            "Disputed" => Ok(CollateralStatus::Disputed),
            "Liquidated" => Ok(CollateralStatus::Liquidated),
            "Impaired" => Ok(CollateralStatus::Impaired),
            "Substituted" => Ok(CollateralStatus::Substituted),
            _ => Err(()),
        }
    }
}

/// Database model for environmental risk level enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "environmental_risk_level", rename_all = "PascalCase")]
pub enum EnvironmentalRiskLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

/// Database model for environmental compliance status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "environmental_compliance_status", rename_all = "PascalCase")]
pub enum EnvironmentalComplianceStatus {
    Compliant,
    NonCompliant,
    UnderReview,
    RequiresRemediation,
}

/// Database model for insurance coverage type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "insurance_coverage_type", rename_all = "PascalCase")]
pub enum InsuranceCoverageType {
    Comprehensive,
    FireAndTheft,
    AllRisk,
    SpecifiedPerils,
    MarineTransit,
    KeyPersonLife,
}

/// Database model for enforcement type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "enforcement_type", rename_all = "PascalCase")]
pub enum EnforcementType {
    PrivateSale,
    PublicAuction,
    CourtOrdered,
    SelfHelp,
    ForeClosureAction,
    Repossession,
}

/// Database model for enforcement method enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "enforcement_method", rename_all = "PascalCase")]
pub enum EnforcementMethod {
    DirectSale,
    AuctionHouse,
    OnlinePlatform,
    BrokerSale,
    CourtSale,
    AssetManagementCompany,
}

/// Database model for enforcement status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "enforcement_status", rename_all = "PascalCase")]
pub enum EnforcementStatus {
    Initiated,
    InProgress,
    Completed,
    Suspended,
    Cancelled,
    UnderLegalReview,
}

/// Database model for valuation method enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "valuation_method", rename_all = "PascalCase")]
pub enum ValuationMethod {
    MarketComparison,
    IncomeApproach,
    CostApproach,
    ExpertAppraisal,
    AuctionValue,
    BookValue,
    MarkToMarket,
}

/// Database model for pledge priority enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "pledge_priority", rename_all = "PascalCase")]
pub enum PledgePriority {
    First,
    Second,
    Third,
    Subordinate,
}

/// Database model for pledge status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "pledge_status", rename_all = "PascalCase")]
pub enum PledgeStatus {
    Active,
    Released,
    PartiallyReleased,
    Substituted,
    Disputed,
    UnderEnforcement,
}

/// Database model for covenant compliance status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "covenant_compliance_status", rename_all = "PascalCase")]
pub enum CovenantComplianceStatus {
    Compliant,
    MinorBreach,
    MaterialBreach,
    Critical,
}

/// Database model for collateral alert type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "collateral_alert_type", rename_all = "PascalCase")]
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

/// Database model for alert severity enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "alert_severity", rename_all = "PascalCase")]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Database model for collateral alert status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "collateral_alert_status", rename_all = "PascalCase")]
pub enum CollateralAlertStatus {
    Open,
    InProgress,
    Resolved,
    Dismissed,
    Escalated,
}

/// Database model for concentration category enum  
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "concentration_category", rename_all = "PascalCase")]
pub enum ConcentrationCategory {
    CollateralType,
    GeographicLocation,
    Industry,
    Borrower,
    CustodyLocation,
}

/// Database model for Collateral
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CollateralModel {
    pub id: Uuid,
    #[serde(serialize_with = "serialize_collateral_type", deserialize_with = "deserialize_collateral_type")]
    pub collateral_type: CollateralType,
    #[serde(serialize_with = "serialize_collateral_category", deserialize_with = "deserialize_collateral_category")]
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
    #[serde(serialize_with = "serialize_custody_location", deserialize_with = "deserialize_custody_location")]
    pub custody_location: CustodyLocation,
    /// References LocationModel.location_id for physical location
    pub physical_location: Option<Uuid>,
    pub custodian_details_id: Option<Uuid>,
    
    // Legal and documentation
    /// References PersonModel.person_id for legal title holder
    pub legal_title_holder_person_id: Uuid,
    #[serde(serialize_with = "serialize_perfection_status", deserialize_with = "deserialize_perfection_status")]
    pub perfection_status: PerfectionStatus,
    pub perfection_date: Option<NaiveDate>,
    pub perfection_expiry_date: Option<NaiveDate>,
    pub registration_number: Option<HeaplessString<100>>,
    /// References PersonModel.person_id for registration authority
    pub registration_authority_person_id: Option<Uuid>,
    
    // Insurance and risk
    pub insurance_required: bool,
    pub insurance_coverage: Option<serde_json::Value>, // JSON field for InsuranceCoverage
    #[serde(serialize_with = "serialize_collateral_risk_rating", deserialize_with = "deserialize_collateral_risk_rating")]
    pub risk_rating: CollateralRiskRating,
    pub environmental_risk: Option<serde_json::Value>, // JSON field for EnvironmentalRisk
    
    // Status and lifecycle
    #[serde(serialize_with = "serialize_collateral_status", deserialize_with = "deserialize_collateral_status")]
    pub status: CollateralStatus,
    pub pledge_date: NaiveDate,
    pub release_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    
    // Audit and tracking
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    /// References PersonModel.person_id
    pub created_by_person_id: Uuid,
    /// References PersonModel.person_id
    pub updated_by_person_id: Uuid,
    pub last_valuation_by_person_id: Option<Uuid>,
    pub next_review_date: Option<NaiveDate>,
}

/// Database model for CollateralValuation
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CollateralValuationModel {
    pub id: Uuid,
    pub collateral_id: Uuid,
    pub valuation_date: NaiveDate,
    #[serde(serialize_with = "serialize_valuation_method", deserialize_with = "deserialize_valuation_method")]
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
    /// References PersonModel.person_id
    pub created_by_person_id: Uuid,
}

/// Database model for CollateralPledge
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CollateralPledgeModel {
    pub id: Uuid,
    pub collateral_id: Uuid,
    pub loan_account_id: Uuid,
    pub pledged_amount: Decimal,
    pub pledge_percentage: Decimal,
    #[serde(serialize_with = "serialize_pledge_priority", deserialize_with = "deserialize_pledge_priority")]
    pub pledge_priority: PledgePriority,
    pub pledge_date: NaiveDate,
    pub release_conditions: Option<HeaplessString<500>>,
    #[serde(serialize_with = "serialize_pledge_status", deserialize_with = "deserialize_pledge_status")]
    pub status: PledgeStatus,
    
    // Legal documentation
    pub security_agreement_reference: HeaplessString<100>,
    pub pledge_registration_number: Option<HeaplessString<100>>,
    
    // Monitoring
    pub last_review_date: Option<NaiveDate>,
    pub next_review_date: Option<NaiveDate>,
    pub covenant_compliance: serde_json::Value, // JSON field for CovenantCompliance
    
    // Audit trail
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    /// References PersonModel.person_id
    pub created_by_person_id: Uuid,
    /// References PersonModel.person_id
    pub updated_by_person_id: Uuid,
}

/// Database model for CollateralAlert
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CollateralAlertModel {
    pub id: Uuid,
    pub collateral_id: Uuid,
    #[serde(serialize_with = "serialize_collateral_alert_type", deserialize_with = "deserialize_collateral_alert_type")]
    pub alert_type: CollateralAlertType,
    #[serde(serialize_with = "serialize_alert_severity", deserialize_with = "deserialize_alert_severity")]
    pub severity: AlertSeverity,
    pub message: HeaplessString<500>,
    pub trigger_date: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    #[serde(serialize_with = "serialize_collateral_alert_status", deserialize_with = "deserialize_collateral_alert_status")]
    pub status: CollateralAlertStatus,
    pub assigned_to_person_id: Option<Uuid>,
    pub resolution_notes: Option<HeaplessString<1000>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by_person_id: Option<Uuid>,
}

/// Database model for CollateralEnforcement
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CollateralEnforcementModel {
    pub id: Uuid,
    pub collateral_id: Uuid,
    pub loan_account_id: Uuid,
    #[serde(serialize_with = "serialize_enforcement_type", deserialize_with = "deserialize_enforcement_type")]
    pub enforcement_type: EnforcementType,
    pub enforcement_date: NaiveDate,
    pub outstanding_debt: Decimal,
    pub estimated_recovery: Decimal,
    #[serde(serialize_with = "serialize_enforcement_method", deserialize_with = "deserialize_enforcement_method")]
    pub enforcement_method: EnforcementMethod,
    #[serde(serialize_with = "serialize_enforcement_status", deserialize_with = "deserialize_enforcement_status")]
    pub status: EnforcementStatus,
    /// References PersonModel.person_id for legal counsel
    pub legal_counsel_person_id: Option<Uuid>,
    pub court_case_reference: Option<HeaplessString<100>>,
    pub expected_completion_date: Option<NaiveDate>,
    pub actual_completion_date: Option<NaiveDate>,
    pub recovery_amount: Option<Decimal>,
    pub enforcement_costs: Option<Decimal>,
    pub net_recovery: Option<Decimal>,
    
    // Audit trail
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    /// References PersonModel.person_id
    pub created_by_person_id: Uuid,
    /// References PersonModel.person_id
    pub updated_by_person_id: Uuid,
}

// Serialization functions for CollateralType
fn serialize_collateral_type<S>(collateral_type: &CollateralType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match collateral_type {
        CollateralType::ResidentialProperty => "ResidentialProperty",
        CollateralType::CommercialProperty => "CommercialProperty",
        CollateralType::IndustrialProperty => "IndustrialProperty",
        CollateralType::Land => "Land",
        CollateralType::PassengerVehicle => "PassengerVehicle",
        CollateralType::CommercialVehicle => "CommercialVehicle",
        CollateralType::Motorcycle => "Motorcycle",
        CollateralType::Boat => "Boat",
        CollateralType::Aircraft => "Aircraft",
        CollateralType::CashDeposit => "CashDeposit",
        CollateralType::GovernmentSecurities => "GovernmentSecurities",
        CollateralType::CorporateBonds => "CorporateBonds",
        CollateralType::Stocks => "Stocks",
        CollateralType::MutualFunds => "MutualFunds",
        CollateralType::Inventory => "Inventory",
        CollateralType::AccountsReceivable => "AccountsReceivable",
        CollateralType::Equipment => "Equipment",
        CollateralType::Machinery => "Machinery",
        CollateralType::Jewelry => "Jewelry",
        CollateralType::ArtAndAntiques => "ArtAndAntiques",
        CollateralType::Electronics => "Electronics",
        CollateralType::PreciousMetals => "PreciousMetals",
        CollateralType::AgriculturalProducts => "AgriculturalProducts",
        CollateralType::Other => "Other",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_collateral_type<'de, D>(deserializer: D) -> Result<CollateralType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "ResidentialProperty" => Ok(CollateralType::ResidentialProperty),
        "CommercialProperty" => Ok(CollateralType::CommercialProperty),
        "IndustrialProperty" => Ok(CollateralType::IndustrialProperty),
        "Land" => Ok(CollateralType::Land),
        "PassengerVehicle" => Ok(CollateralType::PassengerVehicle),
        "CommercialVehicle" => Ok(CollateralType::CommercialVehicle),
        "Motorcycle" => Ok(CollateralType::Motorcycle),
        "Boat" => Ok(CollateralType::Boat),
        "Aircraft" => Ok(CollateralType::Aircraft),
        "CashDeposit" => Ok(CollateralType::CashDeposit),
        "GovernmentSecurities" => Ok(CollateralType::GovernmentSecurities),
        "CorporateBonds" => Ok(CollateralType::CorporateBonds),
        "Stocks" => Ok(CollateralType::Stocks),
        "MutualFunds" => Ok(CollateralType::MutualFunds),
        "Inventory" => Ok(CollateralType::Inventory),
        "AccountsReceivable" => Ok(CollateralType::AccountsReceivable),
        "Equipment" => Ok(CollateralType::Equipment),
        "Machinery" => Ok(CollateralType::Machinery),
        "Jewelry" => Ok(CollateralType::Jewelry),
        "ArtAndAntiques" => Ok(CollateralType::ArtAndAntiques),
        "Electronics" => Ok(CollateralType::Electronics),
        "PreciousMetals" => Ok(CollateralType::PreciousMetals),
        "AgriculturalProducts" => Ok(CollateralType::AgriculturalProducts),
        "Other" => Ok(CollateralType::Other),
        _ => Err(serde::de::Error::custom(format!("Unknown collateral type: {s}"))),
    }
}

// Add similar serialization functions for other enums...
fn serialize_collateral_category<S>(category: &CollateralCategory, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match category {
        CollateralCategory::Immovable => "Immovable",
        CollateralCategory::Movable => "Movable",
        CollateralCategory::Financial => "Financial",
        CollateralCategory::Intangible => "Intangible",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_collateral_category<'de, D>(deserializer: D) -> Result<CollateralCategory, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Immovable" => Ok(CollateralCategory::Immovable),
        "Movable" => Ok(CollateralCategory::Movable),
        "Financial" => Ok(CollateralCategory::Financial),
        "Intangible" => Ok(CollateralCategory::Intangible),
        _ => Err(serde::de::Error::custom(format!("Unknown collateral category: {s}"))),
    }
}

fn serialize_custody_location<S>(location: &CustodyLocation, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match location {
        CustodyLocation::BankVault => "BankVault",
        CustodyLocation::ThirdPartyCustodian => "ThirdPartyCustodian",
        CustodyLocation::ClientPremises => "ClientPremises",
        CustodyLocation::RegisteredWarehouse => "RegisteredWarehouse",
        CustodyLocation::GovernmentRegistry => "GovernmentRegistry",
        CustodyLocation::Other => "Other",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_custody_location<'de, D>(deserializer: D) -> Result<CustodyLocation, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "BankVault" => Ok(CustodyLocation::BankVault),
        "ThirdPartyCustodian" => Ok(CustodyLocation::ThirdPartyCustodian),
        "ClientPremises" => Ok(CustodyLocation::ClientPremises),
        "RegisteredWarehouse" => Ok(CustodyLocation::RegisteredWarehouse),
        "GovernmentRegistry" => Ok(CustodyLocation::GovernmentRegistry),
        "Other" => Ok(CustodyLocation::Other),
        _ => Err(serde::de::Error::custom(format!("Unknown custody location: {s}"))),
    }
}

fn serialize_perfection_status<S>(status: &PerfectionStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match status {
        PerfectionStatus::Pending => "Pending",
        PerfectionStatus::Perfected => "Perfected",
        PerfectionStatus::Imperfected => "Imperfected",
        PerfectionStatus::Expired => "Expired",
        PerfectionStatus::UnderDispute => "UnderDispute",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_perfection_status<'de, D>(deserializer: D) -> Result<PerfectionStatus, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Pending" => Ok(PerfectionStatus::Pending),
        "Perfected" => Ok(PerfectionStatus::Perfected),
        "Imperfected" => Ok(PerfectionStatus::Imperfected),
        "Expired" => Ok(PerfectionStatus::Expired),
        "UnderDispute" => Ok(PerfectionStatus::UnderDispute),
        _ => Err(serde::de::Error::custom(format!("Unknown perfection status: {s}"))),
    }
}

fn serialize_collateral_risk_rating<S>(rating: &CollateralRiskRating, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match rating {
        CollateralRiskRating::Excellent => "Excellent",
        CollateralRiskRating::Good => "Good",
        CollateralRiskRating::Average => "Average",
        CollateralRiskRating::Poor => "Poor",
        CollateralRiskRating::Unacceptable => "Unacceptable",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_collateral_risk_rating<'de, D>(deserializer: D) -> Result<CollateralRiskRating, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Excellent" => Ok(CollateralRiskRating::Excellent),
        "Good" => Ok(CollateralRiskRating::Good),
        "Average" => Ok(CollateralRiskRating::Average),
        "Poor" => Ok(CollateralRiskRating::Poor),
        "Unacceptable" => Ok(CollateralRiskRating::Unacceptable),
        _ => Err(serde::de::Error::custom(format!("Unknown collateral risk rating: {s}"))),
    }
}

fn serialize_collateral_status<S>(status: &CollateralStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match status {
        CollateralStatus::Active => "Active",
        CollateralStatus::Released => "Released",
        CollateralStatus::PartiallyReleased => "PartiallyReleased",
        CollateralStatus::UnderReview => "UnderReview",
        CollateralStatus::Disputed => "Disputed",
        CollateralStatus::Liquidated => "Liquidated",
        CollateralStatus::Impaired => "Impaired",
        CollateralStatus::Substituted => "Substituted",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_collateral_status<'de, D>(deserializer: D) -> Result<CollateralStatus, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Active" => Ok(CollateralStatus::Active),
        "Released" => Ok(CollateralStatus::Released),
        "PartiallyReleased" => Ok(CollateralStatus::PartiallyReleased),
        "UnderReview" => Ok(CollateralStatus::UnderReview),
        "Disputed" => Ok(CollateralStatus::Disputed),
        "Liquidated" => Ok(CollateralStatus::Liquidated),
        "Impaired" => Ok(CollateralStatus::Impaired),
        "Substituted" => Ok(CollateralStatus::Substituted),
        _ => Err(serde::de::Error::custom(format!("Unknown collateral status: {s}"))),
    }
}

fn serialize_enforcement_type<S>(enforcement_type: &EnforcementType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match enforcement_type {
        EnforcementType::PrivateSale => "PrivateSale",
        EnforcementType::PublicAuction => "PublicAuction",
        EnforcementType::CourtOrdered => "CourtOrdered",
        EnforcementType::SelfHelp => "SelfHelp",
        EnforcementType::ForeClosureAction => "ForeClosureAction",
        EnforcementType::Repossession => "Repossession",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_enforcement_type<'de, D>(deserializer: D) -> Result<EnforcementType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "PrivateSale" => Ok(EnforcementType::PrivateSale),
        "PublicAuction" => Ok(EnforcementType::PublicAuction),
        "CourtOrdered" => Ok(EnforcementType::CourtOrdered),
        "SelfHelp" => Ok(EnforcementType::SelfHelp),
        "ForeClosureAction" => Ok(EnforcementType::ForeClosureAction),
        "Repossession" => Ok(EnforcementType::Repossession),
        _ => Err(serde::de::Error::custom(format!("Unknown enforcement type: {s}"))),
    }
}

fn serialize_enforcement_method<S>(method: &EnforcementMethod, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match method {
        EnforcementMethod::DirectSale => "DirectSale",
        EnforcementMethod::AuctionHouse => "AuctionHouse",
        EnforcementMethod::OnlinePlatform => "OnlinePlatform",
        EnforcementMethod::BrokerSale => "BrokerSale",
        EnforcementMethod::CourtSale => "CourtSale",
        EnforcementMethod::AssetManagementCompany => "AssetManagementCompany",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_enforcement_method<'de, D>(deserializer: D) -> Result<EnforcementMethod, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "DirectSale" => Ok(EnforcementMethod::DirectSale),
        "AuctionHouse" => Ok(EnforcementMethod::AuctionHouse),
        "OnlinePlatform" => Ok(EnforcementMethod::OnlinePlatform),
        "BrokerSale" => Ok(EnforcementMethod::BrokerSale),
        "CourtSale" => Ok(EnforcementMethod::CourtSale),
        "AssetManagementCompany" => Ok(EnforcementMethod::AssetManagementCompany),
        _ => Err(serde::de::Error::custom(format!("Unknown enforcement method: {s}"))),
    }
}

fn serialize_enforcement_status<S>(status: &EnforcementStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match status {
        EnforcementStatus::Initiated => "Initiated",
        EnforcementStatus::InProgress => "InProgress",
        EnforcementStatus::Completed => "Completed",
        EnforcementStatus::Suspended => "Suspended",
        EnforcementStatus::Cancelled => "Cancelled",
        EnforcementStatus::UnderLegalReview => "UnderLegalReview",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_enforcement_status<'de, D>(deserializer: D) -> Result<EnforcementStatus, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Initiated" => Ok(EnforcementStatus::Initiated),
        "InProgress" => Ok(EnforcementStatus::InProgress),
        "Completed" => Ok(EnforcementStatus::Completed),
        "Suspended" => Ok(EnforcementStatus::Suspended),
        "Cancelled" => Ok(EnforcementStatus::Cancelled),
        "UnderLegalReview" => Ok(EnforcementStatus::UnderLegalReview),
        _ => Err(serde::de::Error::custom(format!("Unknown enforcement status: {s}"))),
    }
}

// Serialization functions for ValuationMethod
fn serialize_valuation_method<S>(method: &ValuationMethod, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let method_str = match method {
        ValuationMethod::MarketComparison => "MarketComparison",
        ValuationMethod::IncomeApproach => "IncomeApproach",
        ValuationMethod::CostApproach => "CostApproach",
        ValuationMethod::ExpertAppraisal => "ExpertAppraisal",
        ValuationMethod::AuctionValue => "AuctionValue",
        ValuationMethod::BookValue => "BookValue",
        ValuationMethod::MarkToMarket => "MarkToMarket",
    };
    serializer.serialize_str(method_str)
}

fn deserialize_valuation_method<'de, D>(deserializer: D) -> Result<ValuationMethod, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "MarketComparison" => Ok(ValuationMethod::MarketComparison),
        "IncomeApproach" => Ok(ValuationMethod::IncomeApproach),
        "CostApproach" => Ok(ValuationMethod::CostApproach),
        "ExpertAppraisal" => Ok(ValuationMethod::ExpertAppraisal),
        "AuctionValue" => Ok(ValuationMethod::AuctionValue),
        "BookValue" => Ok(ValuationMethod::BookValue),
        "MarkToMarket" => Ok(ValuationMethod::MarkToMarket),
        _ => Err(serde::de::Error::custom(format!("Unknown valuation method: {s}"))),
    }
}

// Serialization functions for PledgePriority
fn serialize_pledge_priority<S>(priority: &PledgePriority, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let priority_str = match priority {
        PledgePriority::First => "First",
        PledgePriority::Second => "Second",
        PledgePriority::Third => "Third",
        PledgePriority::Subordinate => "Subordinate",
    };
    serializer.serialize_str(priority_str)
}

fn deserialize_pledge_priority<'de, D>(deserializer: D) -> Result<PledgePriority, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "First" => Ok(PledgePriority::First),
        "Second" => Ok(PledgePriority::Second),
        "Third" => Ok(PledgePriority::Third),
        "Subordinate" => Ok(PledgePriority::Subordinate),
        _ => Err(serde::de::Error::custom(format!("Unknown pledge priority: {s}"))),
    }
}

// Serialization functions for PledgeStatus
fn serialize_pledge_status<S>(status: &PledgeStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let status_str = match status {
        PledgeStatus::Active => "Active",
        PledgeStatus::Released => "Released",
        PledgeStatus::PartiallyReleased => "PartiallyReleased",
        PledgeStatus::Substituted => "Substituted",
        PledgeStatus::Disputed => "Disputed",
        PledgeStatus::UnderEnforcement => "UnderEnforcement",
    };
    serializer.serialize_str(status_str)
}

fn deserialize_pledge_status<'de, D>(deserializer: D) -> Result<PledgeStatus, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Active" => Ok(PledgeStatus::Active),
        "Released" => Ok(PledgeStatus::Released),
        "PartiallyReleased" => Ok(PledgeStatus::PartiallyReleased),
        "Substituted" => Ok(PledgeStatus::Substituted),
        "Disputed" => Ok(PledgeStatus::Disputed),
        "UnderEnforcement" => Ok(PledgeStatus::UnderEnforcement),
        _ => Err(serde::de::Error::custom(format!("Unknown pledge status: {s}"))),
    }
}

// Serialization functions for CollateralAlertType
fn serialize_collateral_alert_type<S>(alert_type: &CollateralAlertType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match alert_type {
        CollateralAlertType::ValuationDue => "ValuationDue",
        CollateralAlertType::ValuationOverdue => "ValuationOverdue",
        CollateralAlertType::InsuranceExpiring => "InsuranceExpiring",
        CollateralAlertType::InsuranceExpired => "InsuranceExpired",
        CollateralAlertType::PerfectionExpiring => "PerfectionExpiring",
        CollateralAlertType::PerfectionExpired => "PerfectionExpired",
        CollateralAlertType::LtvBreach => "LtvBreach",
        CollateralAlertType::CovenantBreach => "CovenantBreach",
        CollateralAlertType::MaintenanceRequired => "MaintenanceRequired",
        CollateralAlertType::DocumentationMissing => "DocumentationMissing",
        CollateralAlertType::EnvironmentalRisk => "EnvironmentalRisk",
        CollateralAlertType::MarketValueDecline => "MarketValueDecline",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_collateral_alert_type<'de, D>(deserializer: D) -> Result<CollateralAlertType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "ValuationDue" => Ok(CollateralAlertType::ValuationDue),
        "ValuationOverdue" => Ok(CollateralAlertType::ValuationOverdue),
        "InsuranceExpiring" => Ok(CollateralAlertType::InsuranceExpiring),
        "InsuranceExpired" => Ok(CollateralAlertType::InsuranceExpired),
        "PerfectionExpiring" => Ok(CollateralAlertType::PerfectionExpiring),
        "PerfectionExpired" => Ok(CollateralAlertType::PerfectionExpired),
        "LtvBreach" => Ok(CollateralAlertType::LtvBreach),
        "CovenantBreach" => Ok(CollateralAlertType::CovenantBreach),
        "MaintenanceRequired" => Ok(CollateralAlertType::MaintenanceRequired),
        "DocumentationMissing" => Ok(CollateralAlertType::DocumentationMissing),
        "EnvironmentalRisk" => Ok(CollateralAlertType::EnvironmentalRisk),
        "MarketValueDecline" => Ok(CollateralAlertType::MarketValueDecline),
        _ => Err(serde::de::Error::custom(format!("Unknown collateral alert type: {s}"))),
    }
}

// Serialization functions for AlertSeverity
fn serialize_alert_severity<S>(severity: &AlertSeverity, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let severity_str = match severity {
        AlertSeverity::Low => "Low",
        AlertSeverity::Medium => "Medium",
        AlertSeverity::High => "High",
        AlertSeverity::Critical => "Critical",
    };
    serializer.serialize_str(severity_str)
}

fn deserialize_alert_severity<'de, D>(deserializer: D) -> Result<AlertSeverity, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Low" => Ok(AlertSeverity::Low),
        "Medium" => Ok(AlertSeverity::Medium),
        "High" => Ok(AlertSeverity::High),
        "Critical" => Ok(AlertSeverity::Critical),
        _ => Err(serde::de::Error::custom(format!("Unknown alert severity: {s}"))),
    }
}

// Serialization functions for CollateralAlertStatus
fn serialize_collateral_alert_status<S>(status: &CollateralAlertStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let status_str = match status {
        CollateralAlertStatus::Open => "Open",
        CollateralAlertStatus::InProgress => "InProgress",
        CollateralAlertStatus::Resolved => "Resolved",
        CollateralAlertStatus::Dismissed => "Dismissed",
        CollateralAlertStatus::Escalated => "Escalated",
    };
    serializer.serialize_str(status_str)
}

fn deserialize_collateral_alert_status<'de, D>(deserializer: D) -> Result<CollateralAlertStatus, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Open" => Ok(CollateralAlertStatus::Open),
        "InProgress" => Ok(CollateralAlertStatus::InProgress),
        "Resolved" => Ok(CollateralAlertStatus::Resolved),
        "Dismissed" => Ok(CollateralAlertStatus::Dismissed),
        "Escalated" => Ok(CollateralAlertStatus::Escalated),
        _ => Err(serde::de::Error::custom(format!("Unknown collateral alert status: {s}"))),
    }
}

/// Database model for CustodianDetails
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CustodianDetailsModel {
    pub id: Uuid,
    pub person_id: Uuid,
    pub custodian_license_number: Option<HeaplessString<100>>,
    pub contact_person_id: Uuid,
    pub custody_agreement_reference: HeaplessString<100>,
    pub custody_fees: Option<Decimal>,
}

/// Database model for ConcentrationAnalysis
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ConcentrationAnalysisModel {
    pub id: Uuid,
    pub collateral_portfolio_summary_id: Uuid,
    #[serde(serialize_with = "serialize_concentration_category", deserialize_with = "deserialize_concentration_category")]
    pub category: ConcentrationCategory,
    pub category_value: HeaplessString<100>,
    pub count: i32,
    pub total_value: Decimal,
    pub percentage_of_portfolio: Decimal,
}

/// Database model for RiskDistribution
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RiskDistributionModel {
    pub id: Uuid,
    pub collateral_portfolio_summary_id: Uuid,
    #[serde(serialize_with = "serialize_collateral_risk_rating", deserialize_with = "deserialize_collateral_risk_rating")]
    pub risk_rating: CollateralRiskRating,
    pub count: i32,
    pub total_value: Decimal,
    pub percentage_of_portfolio: Decimal,
}

/// Database model for CollateralPortfolioSummary
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CollateralPortfolioSummaryModel {
    pub id: Uuid,
    pub as_of_date: NaiveDate,
    pub total_collateral_count: i32,
    pub total_market_value: Decimal,
    pub total_pledged_value: Decimal,
    pub total_available_value: Decimal,
    pub weighted_average_ltv: Decimal,
    pub concentration_analysis_id_01: Option<Uuid>,
    pub concentration_analysis_id_02: Option<Uuid>,
    pub concentration_analysis_id_03: Option<Uuid>,
    pub concentration_analysis_id_04: Option<Uuid>,
    pub concentration_analysis_id_05: Option<Uuid>,
    pub concentration_analysis_id_06: Option<Uuid>,
    pub concentration_analysis_id_07: Option<Uuid>,
    pub risk_distribution_id_01: Option<Uuid>,
    pub risk_distribution_id_02: Option<Uuid>,
    pub risk_distribution_id_03: Option<Uuid>,
    pub risk_distribution_id_04: Option<Uuid>,
    pub risk_distribution_id_05: Option<Uuid>,
    pub risk_distribution_id_06: Option<Uuid>,
    pub risk_distribution_id_07: Option<Uuid>,
    pub valuation_status: serde_json::Value, // JSON field for ValuationStatusSummary
    pub covenant_compliance_summary: serde_json::Value, // JSON field for ComplianceSummary
}

// Serialization functions for ConcentrationCategory
fn serialize_concentration_category<S>(category: &ConcentrationCategory, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let category_str = match category {
        ConcentrationCategory::CollateralType => "CollateralType",
        ConcentrationCategory::GeographicLocation => "GeographicLocation",
        ConcentrationCategory::Industry => "Industry",
        ConcentrationCategory::Borrower => "Borrower",
        ConcentrationCategory::CustodyLocation => "CustodyLocation",
    };
    serializer.serialize_str(category_str)
}

fn deserialize_concentration_category<'de, D>(deserializer: D) -> Result<ConcentrationCategory, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "CollateralType" => Ok(ConcentrationCategory::CollateralType),
        "GeographicLocation" => Ok(ConcentrationCategory::GeographicLocation),
        "Industry" => Ok(ConcentrationCategory::Industry),
        "Borrower" => Ok(ConcentrationCategory::Borrower),
        "CustodyLocation" => Ok(ConcentrationCategory::CustodyLocation),
        _ => Err(serde::de::Error::custom(format!("Unknown concentration category: {s}"))),
    }
}
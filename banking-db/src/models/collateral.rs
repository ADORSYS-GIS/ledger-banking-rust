use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Database model for collateral type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "collateral_type", rename_all = "lowercase")]
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
#[sqlx(type_name = "collateral_category", rename_all = "lowercase")]
pub enum CollateralCategory {
    Immovable,
    Movable,
    Financial,
    Intangible,
}

/// Database model for custody location enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "custody_location", rename_all = "lowercase")]
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
#[sqlx(type_name = "perfection_status", rename_all = "lowercase")]
pub enum PerfectionStatus {
    Pending,
    Perfected,
    Imperfected,
    Expired,
    UnderDispute,
}

/// Database model for collateral risk rating enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "collateral_risk_rating", rename_all = "lowercase")]
pub enum CollateralRiskRating {
    Excellent,
    Good,
    Average,
    Poor,
    Unacceptable,
}

/// Database model for collateral status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "collateral_status", rename_all = "lowercase")]
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

/// Database model for environmental risk level enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "environmental_risk_level", rename_all = "lowercase")]
pub enum EnvironmentalRiskLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

/// Database model for environmental compliance status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "environmental_compliance_status", rename_all = "lowercase")]
pub enum EnvironmentalComplianceStatus {
    Compliant,
    NonCompliant,
    UnderReview,
    RequiresRemediation,
}

/// Database model for insurance coverage type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "insurance_coverage_type", rename_all = "lowercase")]
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
#[sqlx(type_name = "enforcement_type", rename_all = "lowercase")]
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
#[sqlx(type_name = "enforcement_method", rename_all = "lowercase")]
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
#[sqlx(type_name = "enforcement_status", rename_all = "lowercase")]
pub enum EnforcementStatus {
    Initiated,
    InProgress,
    Completed,
    Suspended,
    Cancelled,
    UnderLegalReview,
}

/// Database model for Collateral
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CollateralModel {
    pub collateral_id: Uuid,
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
    /// References AddressModel.address_id for physical location
    pub physical_location: Option<Uuid>,
    pub custodian_details: Option<serde_json::Value>, // JSON field for CustodianDetails
    
    // Legal and documentation
    /// References PersonModel.person_id for legal title holder
    pub legal_title_holder: Uuid,
    #[serde(serialize_with = "serialize_perfection_status", deserialize_with = "deserialize_perfection_status")]
    pub perfection_status: PerfectionStatus,
    pub perfection_date: Option<NaiveDate>,
    pub perfection_expiry_date: Option<NaiveDate>,
    pub registration_number: Option<HeaplessString<100>>,
    /// References PersonModel.person_id for registration authority
    pub registration_authority: Option<Uuid>,
    
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
    pub created_by: Uuid,
    /// References PersonModel.person_id
    pub updated_by: Uuid,
    pub last_valuation_by: Option<Uuid>,
    pub next_review_date: Option<NaiveDate>,
}

/// Database model for CollateralEnforcement
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CollateralEnforcementModel {
    pub enforcement_id: Uuid,
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
    /// References PersonModel.person_id
    pub created_by: Uuid,
    /// References PersonModel.person_id
    pub updated_by: Uuid,
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
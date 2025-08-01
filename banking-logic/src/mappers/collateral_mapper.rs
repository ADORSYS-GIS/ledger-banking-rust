use banking_api::domain::{
    Collateral, CollateralType, CollateralCategory, CustodyLocation, PerfectionStatus,
    CollateralRiskRating, CollateralStatus, ValuationMethod, PledgePriority, PledgeStatus,
    CollateralAlertType, AlertSeverity, CollateralAlertStatus,
    EnforcementType, EnforcementMethod, EnforcementStatus,
    CollateralValuation, CollateralPledge, CollateralAlert, CollateralEnforcement
};
use banking_db::models::{
    CollateralModel, CollateralValuationModel, CollateralPledgeModel, CollateralAlertModel, CollateralEnforcementModel,
    CollateralType as DbCollateralType, CollateralCategory as DbCollateralCategory, 
    CustodyLocation as DbCustodyLocation, PerfectionStatus as DbPerfectionStatus,
    CollateralRiskRating as DbCollateralRiskRating, CollateralStatus as DbCollateralStatus,
    ValuationMethod as DbValuationMethod,
    PledgePriority as DbPledgePriority, PledgeStatus as DbPledgeStatus, 
    CollateralAlertType as DbCollateralAlertType,
    AlertSeverity as DbAlertSeverity, CollateralAlertStatus as DbCollateralAlertStatus,
    EnforcementType as DbEnforcementType,
    EnforcementMethod as DbEnforcementMethod, EnforcementStatus as DbEnforcementStatus
};

pub struct CollateralMapper;

impl CollateralMapper {
    /// Map from domain Collateral to database CollateralModel
    pub fn to_model(collateral: Collateral) -> CollateralModel {
        CollateralModel {
            collateral_id: collateral.collateral_id,
            collateral_type: Self::collateral_type_to_db(collateral.collateral_type),
            collateral_category: Self::collateral_category_to_db(collateral.collateral_category),
            description: collateral.description,
            external_reference: collateral.external_reference,
            original_value: collateral.original_value,
            current_market_value: collateral.current_market_value,
            appraised_value: collateral.appraised_value,
            currency: collateral.currency,
            valuation_date: collateral.valuation_date,
            next_valuation_date: collateral.next_valuation_date,
            valuation_frequency_months: collateral.valuation_frequency_months,
            pledged_value: collateral.pledged_value,
            available_value: collateral.available_value,
            lien_amount: collateral.lien_amount,
            margin_percentage: collateral.margin_percentage,
            forced_sale_value: collateral.forced_sale_value,
            custody_location: Self::custody_location_to_db(collateral.custody_location),
            physical_location: collateral.physical_location,
            custodian_details: collateral.custodian_details.map(|details| serde_json::to_value(details).unwrap_or_default()),
            legal_title_holder: collateral.legal_title_holder,
            perfection_status: Self::perfection_status_to_db(collateral.perfection_status),
            perfection_date: collateral.perfection_date,
            perfection_expiry_date: collateral.perfection_expiry_date,
            registration_number: collateral.registration_number,
            registration_authority: collateral.registration_authority,
            insurance_required: collateral.insurance_required,
            insurance_coverage: collateral.insurance_coverage.map(|coverage| serde_json::to_value(coverage).unwrap_or_default()),
            risk_rating: Self::collateral_risk_rating_to_db(collateral.risk_rating),
            environmental_risk: collateral.environmental_risk.map(|risk| serde_json::to_value(risk).unwrap_or_default()),
            status: Self::collateral_status_to_db(collateral.status),
            pledge_date: collateral.pledge_date,
            release_date: collateral.release_date,
            maturity_date: collateral.maturity_date,
            created_at: collateral.created_at,
            last_updated_at: collateral.last_updated_at,
            created_by: collateral.created_by,
            updated_by: collateral.updated_by,
            last_valuation_by: collateral.last_valuation_by,
            next_review_date: collateral.next_review_date,
        }
    }

    /// Map from database CollateralModel to domain Collateral
    pub fn from_model(model: CollateralModel) -> banking_api::BankingResult<Collateral> {
        Ok(Collateral {
            collateral_id: model.collateral_id,
            collateral_type: Self::db_to_collateral_type(model.collateral_type),
            collateral_category: Self::db_to_collateral_category(model.collateral_category),
            description: model.description,
            external_reference: model.external_reference,
            original_value: model.original_value,
            current_market_value: model.current_market_value,
            appraised_value: model.appraised_value,
            currency: model.currency,
            valuation_date: model.valuation_date,
            next_valuation_date: model.next_valuation_date,
            valuation_frequency_months: model.valuation_frequency_months,
            pledged_value: model.pledged_value,
            available_value: model.available_value,
            lien_amount: model.lien_amount,
            margin_percentage: model.margin_percentage,
            forced_sale_value: model.forced_sale_value,
            custody_location: Self::db_to_custody_location(model.custody_location),
            physical_location: model.physical_location,
            custodian_details: model.custodian_details.and_then(|v| serde_json::from_value(v).ok()),
            legal_title_holder: model.legal_title_holder,
            perfection_status: Self::db_to_perfection_status(model.perfection_status),
            perfection_date: model.perfection_date,
            perfection_expiry_date: model.perfection_expiry_date,
            registration_number: model.registration_number,
            registration_authority: model.registration_authority,
            insurance_required: model.insurance_required,
            insurance_coverage: model.insurance_coverage.and_then(|v| serde_json::from_value(v).ok()),
            risk_rating: Self::db_to_collateral_risk_rating(model.risk_rating),
            environmental_risk: model.environmental_risk.and_then(|v| serde_json::from_value(v).ok()),
            status: Self::db_to_collateral_status(model.status),
            pledge_date: model.pledge_date,
            release_date: model.release_date,
            maturity_date: model.maturity_date,
            created_at: model.created_at,
            last_updated_at: model.last_updated_at,
            created_by: model.created_by,
            updated_by: model.updated_by,
            last_valuation_by: model.last_valuation_by,
            next_review_date: model.next_review_date,
        })
    }

    /// Map from domain CollateralValuation to database CollateralValuationModel
    pub fn valuation_to_model(valuation: CollateralValuation) -> CollateralValuationModel {
        CollateralValuationModel {
            valuation_id: valuation.valuation_id,
            collateral_id: valuation.collateral_id,
            valuation_date: valuation.valuation_date,
            valuation_method: Self::valuation_method_to_db(valuation.valuation_method),
            market_value: valuation.market_value,
            forced_sale_value: valuation.forced_sale_value,
            appraiser_name: valuation.appraiser_name,
            appraiser_license: valuation.appraiser_license,
            valuation_report_reference: valuation.valuation_report_reference,
            validity_period_months: valuation.validity_period_months,
            next_valuation_due: valuation.next_valuation_due,
            valuation_notes: valuation.valuation_notes,
            created_at: valuation.created_at,
            created_by: valuation.created_by,
        }
    }

    /// Map from database CollateralValuationModel to domain CollateralValuation
    pub fn valuation_from_model(model: CollateralValuationModel) -> banking_api::BankingResult<CollateralValuation> {
        Ok(CollateralValuation {
            valuation_id: model.valuation_id,
            collateral_id: model.collateral_id,
            valuation_date: model.valuation_date,
            valuation_method: Self::db_to_valuation_method(model.valuation_method),
            market_value: model.market_value,
            forced_sale_value: model.forced_sale_value,
            appraiser_name: model.appraiser_name,
            appraiser_license: model.appraiser_license,
            valuation_report_reference: model.valuation_report_reference,
            validity_period_months: model.validity_period_months,
            next_valuation_due: model.next_valuation_due,
            valuation_notes: model.valuation_notes,
            created_at: model.created_at,
            created_by: model.created_by,
        })
    }

    /// Map from domain CollateralPledge to database CollateralPledgeModel
    pub fn pledge_to_model(pledge: CollateralPledge) -> CollateralPledgeModel {
        CollateralPledgeModel {
            pledge_id: pledge.pledge_id,
            collateral_id: pledge.collateral_id,
            loan_account_id: pledge.loan_account_id,
            pledged_amount: pledge.pledged_amount,
            pledge_percentage: pledge.pledge_percentage,
            pledge_priority: Self::pledge_priority_to_db(pledge.pledge_priority),
            pledge_date: pledge.pledge_date,
            release_conditions: pledge.release_conditions,
            status: Self::pledge_status_to_db(pledge.status),
            security_agreement_reference: pledge.security_agreement_reference,
            pledge_registration_number: pledge.pledge_registration_number,
            last_review_date: pledge.last_review_date,
            next_review_date: pledge.next_review_date,
            covenant_compliance: serde_json::to_value(pledge.covenant_compliance).unwrap_or_default(),
            created_at: pledge.created_at,
            last_updated_at: pledge.last_updated_at,
            created_by: pledge.created_by,
            updated_by: pledge.updated_by,
        }
    }

    /// Map from database CollateralPledgeModel to domain CollateralPledge
    pub fn pledge_from_model(model: CollateralPledgeModel) -> banking_api::BankingResult<CollateralPledge> {
        Ok(CollateralPledge {
            pledge_id: model.pledge_id,
            collateral_id: model.collateral_id,
            loan_account_id: model.loan_account_id,
            pledged_amount: model.pledged_amount,
            pledge_percentage: model.pledge_percentage,
            pledge_priority: Self::db_to_pledge_priority(model.pledge_priority),
            pledge_date: model.pledge_date,
            release_conditions: model.release_conditions,
            status: Self::db_to_pledge_status(model.status),
            security_agreement_reference: model.security_agreement_reference,
            pledge_registration_number: model.pledge_registration_number,
            last_review_date: model.last_review_date,
            next_review_date: model.next_review_date,
            covenant_compliance: serde_json::from_value(model.covenant_compliance).unwrap_or_default(),
            created_at: model.created_at,
            last_updated_at: model.last_updated_at,
            created_by: model.created_by,
            updated_by: model.updated_by,
        })
    }

    /// Map from domain CollateralAlert to database CollateralAlertModel
    pub fn alert_to_model(alert: CollateralAlert) -> CollateralAlertModel {
        CollateralAlertModel {
            alert_id: alert.alert_id,
            collateral_id: alert.collateral_id,
            alert_type: Self::collateral_alert_type_to_db(alert.alert_type),
            severity: Self::alert_severity_to_db(alert.severity),
            message: alert.message,
            trigger_date: alert.trigger_date,
            due_date: alert.due_date,
            status: Self::collateral_alert_status_to_db(alert.status),
            assigned_to: alert.assigned_to,
            resolution_notes: alert.resolution_notes,
            resolved_at: alert.resolved_at,
            resolved_by: alert.resolved_by,
        }
    }

    /// Map from database CollateralAlertModel to domain CollateralAlert
    pub fn alert_from_model(model: CollateralAlertModel) -> banking_api::BankingResult<CollateralAlert> {
        Ok(CollateralAlert {
            alert_id: model.alert_id,
            collateral_id: model.collateral_id,
            alert_type: Self::db_to_collateral_alert_type(model.alert_type),
            severity: Self::db_to_alert_severity(model.severity),
            message: model.message,
            trigger_date: model.trigger_date,
            due_date: model.due_date,
            status: Self::db_to_collateral_alert_status(model.status),
            assigned_to: model.assigned_to,
            resolution_notes: model.resolution_notes,
            resolved_at: model.resolved_at,
            resolved_by: model.resolved_by,
        })
    }

    /// Map from domain CollateralEnforcement to database CollateralEnforcementModel
    pub fn enforcement_to_model(enforcement: CollateralEnforcement) -> CollateralEnforcementModel {
        CollateralEnforcementModel {
            enforcement_id: enforcement.enforcement_id,
            collateral_id: enforcement.collateral_id,
            loan_account_id: enforcement.loan_account_id,
            enforcement_type: Self::enforcement_type_to_db(enforcement.enforcement_type),
            enforcement_date: enforcement.enforcement_date,
            outstanding_debt: enforcement.outstanding_debt,
            estimated_recovery: enforcement.estimated_recovery,
            enforcement_method: Self::enforcement_method_to_db(enforcement.enforcement_method),
            status: Self::enforcement_status_to_db(enforcement.status),
            legal_counsel: enforcement.legal_counsel,
            court_case_reference: enforcement.court_case_reference,
            expected_completion_date: enforcement.expected_completion_date,
            actual_completion_date: enforcement.actual_completion_date,
            recovery_amount: enforcement.recovery_amount,
            enforcement_costs: enforcement.enforcement_costs,
            net_recovery: enforcement.net_recovery,
            created_at: enforcement.created_at,
            last_updated_at: enforcement.last_updated_at,
            created_by: enforcement.created_by,
            updated_by: enforcement.updated_by,
        }
    }

    /// Map from database CollateralEnforcementModel to domain CollateralEnforcement
    pub fn enforcement_from_model(model: CollateralEnforcementModel) -> banking_api::BankingResult<CollateralEnforcement> {
        Ok(CollateralEnforcement {
            enforcement_id: model.enforcement_id,
            collateral_id: model.collateral_id,
            loan_account_id: model.loan_account_id,
            enforcement_type: Self::db_to_enforcement_type(model.enforcement_type),
            enforcement_date: model.enforcement_date,
            outstanding_debt: model.outstanding_debt,
            estimated_recovery: model.estimated_recovery,
            enforcement_method: Self::db_to_enforcement_method(model.enforcement_method),
            status: Self::db_to_enforcement_status(model.status),
            legal_counsel: model.legal_counsel,
            court_case_reference: model.court_case_reference,
            expected_completion_date: model.expected_completion_date,
            actual_completion_date: model.actual_completion_date,
            recovery_amount: model.recovery_amount,
            enforcement_costs: model.enforcement_costs,
            net_recovery: model.net_recovery,
            created_at: model.created_at,
            last_updated_at: model.last_updated_at,
            created_by: model.created_by,
            updated_by: model.updated_by,
        })
    }

    // Enum conversion helpers
    fn collateral_type_to_db(ct: CollateralType) -> DbCollateralType {
        match ct {
            CollateralType::ResidentialProperty => DbCollateralType::ResidentialProperty,
            CollateralType::CommercialProperty => DbCollateralType::CommercialProperty,
            CollateralType::IndustrialProperty => DbCollateralType::IndustrialProperty,
            CollateralType::Land => DbCollateralType::Land,
            CollateralType::PassengerVehicle => DbCollateralType::PassengerVehicle,
            CollateralType::CommercialVehicle => DbCollateralType::CommercialVehicle,
            CollateralType::Motorcycle => DbCollateralType::Motorcycle,
            CollateralType::Boat => DbCollateralType::Boat,
            CollateralType::Aircraft => DbCollateralType::Aircraft,
            CollateralType::CashDeposit => DbCollateralType::CashDeposit,
            CollateralType::GovernmentSecurities => DbCollateralType::GovernmentSecurities,
            CollateralType::CorporateBonds => DbCollateralType::CorporateBonds,
            CollateralType::Stocks => DbCollateralType::Stocks,
            CollateralType::MutualFunds => DbCollateralType::MutualFunds,
            CollateralType::Inventory => DbCollateralType::Inventory,
            CollateralType::AccountsReceivable => DbCollateralType::AccountsReceivable,
            CollateralType::Equipment => DbCollateralType::Equipment,
            CollateralType::Machinery => DbCollateralType::Machinery,
            CollateralType::Jewelry => DbCollateralType::Jewelry,
            CollateralType::ArtAndAntiques => DbCollateralType::ArtAndAntiques,
            CollateralType::Electronics => DbCollateralType::Electronics,
            CollateralType::PreciousMetals => DbCollateralType::PreciousMetals,
            CollateralType::AgriculturalProducts => DbCollateralType::AgriculturalProducts,
            CollateralType::Other => DbCollateralType::Other,
        }
    }

    fn db_to_collateral_type(ct: DbCollateralType) -> CollateralType {
        match ct {
            DbCollateralType::ResidentialProperty => CollateralType::ResidentialProperty,
            DbCollateralType::CommercialProperty => CollateralType::CommercialProperty,
            DbCollateralType::IndustrialProperty => CollateralType::IndustrialProperty,
            DbCollateralType::Land => CollateralType::Land,
            DbCollateralType::PassengerVehicle => CollateralType::PassengerVehicle,
            DbCollateralType::CommercialVehicle => CollateralType::CommercialVehicle,
            DbCollateralType::Motorcycle => CollateralType::Motorcycle,
            DbCollateralType::Boat => CollateralType::Boat,
            DbCollateralType::Aircraft => CollateralType::Aircraft,
            DbCollateralType::CashDeposit => CollateralType::CashDeposit,
            DbCollateralType::GovernmentSecurities => CollateralType::GovernmentSecurities,
            DbCollateralType::CorporateBonds => CollateralType::CorporateBonds,
            DbCollateralType::Stocks => CollateralType::Stocks,
            DbCollateralType::MutualFunds => CollateralType::MutualFunds,
            DbCollateralType::Inventory => CollateralType::Inventory,
            DbCollateralType::AccountsReceivable => CollateralType::AccountsReceivable,
            DbCollateralType::Equipment => CollateralType::Equipment,
            DbCollateralType::Machinery => CollateralType::Machinery,
            DbCollateralType::Jewelry => CollateralType::Jewelry,
            DbCollateralType::ArtAndAntiques => CollateralType::ArtAndAntiques,
            DbCollateralType::Electronics => CollateralType::Electronics,
            DbCollateralType::PreciousMetals => CollateralType::PreciousMetals,
            DbCollateralType::AgriculturalProducts => CollateralType::AgriculturalProducts,
            DbCollateralType::Other => CollateralType::Other,
        }
    }

    fn collateral_category_to_db(cc: CollateralCategory) -> DbCollateralCategory {
        match cc {
            CollateralCategory::Immovable => DbCollateralCategory::Immovable,
            CollateralCategory::Movable => DbCollateralCategory::Movable,
            CollateralCategory::Financial => DbCollateralCategory::Financial,
            CollateralCategory::Intangible => DbCollateralCategory::Intangible,
        }
    }

    fn db_to_collateral_category(cc: DbCollateralCategory) -> CollateralCategory {
        match cc {
            DbCollateralCategory::Immovable => CollateralCategory::Immovable,
            DbCollateralCategory::Movable => CollateralCategory::Movable,
            DbCollateralCategory::Financial => CollateralCategory::Financial,
            DbCollateralCategory::Intangible => CollateralCategory::Intangible,
        }
    }

    fn custody_location_to_db(cl: CustodyLocation) -> DbCustodyLocation {
        match cl {
            CustodyLocation::BankVault => DbCustodyLocation::BankVault,
            CustodyLocation::ThirdPartyCustodian => DbCustodyLocation::ThirdPartyCustodian,
            CustodyLocation::ClientPremises => DbCustodyLocation::ClientPremises,
            CustodyLocation::RegisteredWarehouse => DbCustodyLocation::RegisteredWarehouse,
            CustodyLocation::GovernmentRegistry => DbCustodyLocation::GovernmentRegistry,
            CustodyLocation::Other => DbCustodyLocation::Other,
        }
    }

    fn db_to_custody_location(cl: DbCustodyLocation) -> CustodyLocation {
        match cl {
            DbCustodyLocation::BankVault => CustodyLocation::BankVault,
            DbCustodyLocation::ThirdPartyCustodian => CustodyLocation::ThirdPartyCustodian,
            DbCustodyLocation::ClientPremises => CustodyLocation::ClientPremises,
            DbCustodyLocation::RegisteredWarehouse => CustodyLocation::RegisteredWarehouse,
            DbCustodyLocation::GovernmentRegistry => CustodyLocation::GovernmentRegistry,
            DbCustodyLocation::Other => CustodyLocation::Other,
        }
    }

    fn perfection_status_to_db(ps: PerfectionStatus) -> DbPerfectionStatus {
        match ps {
            PerfectionStatus::Pending => DbPerfectionStatus::Pending,
            PerfectionStatus::Perfected => DbPerfectionStatus::Perfected,
            PerfectionStatus::Imperfected => DbPerfectionStatus::Imperfected,
            PerfectionStatus::Expired => DbPerfectionStatus::Expired,
            PerfectionStatus::UnderDispute => DbPerfectionStatus::UnderDispute,
        }
    }

    fn db_to_perfection_status(ps: DbPerfectionStatus) -> PerfectionStatus {
        match ps {
            DbPerfectionStatus::Pending => PerfectionStatus::Pending,
            DbPerfectionStatus::Perfected => PerfectionStatus::Perfected,
            DbPerfectionStatus::Imperfected => PerfectionStatus::Imperfected,
            DbPerfectionStatus::Expired => PerfectionStatus::Expired,
            DbPerfectionStatus::UnderDispute => PerfectionStatus::UnderDispute,
        }
    }

    fn collateral_risk_rating_to_db(crr: CollateralRiskRating) -> DbCollateralRiskRating {
        match crr {
            CollateralRiskRating::Excellent => DbCollateralRiskRating::Excellent,
            CollateralRiskRating::Good => DbCollateralRiskRating::Good,
            CollateralRiskRating::Average => DbCollateralRiskRating::Average,
            CollateralRiskRating::Poor => DbCollateralRiskRating::Poor,
            CollateralRiskRating::Unacceptable => DbCollateralRiskRating::Unacceptable,
        }
    }

    fn db_to_collateral_risk_rating(crr: DbCollateralRiskRating) -> CollateralRiskRating {
        match crr {
            DbCollateralRiskRating::Excellent => CollateralRiskRating::Excellent,
            DbCollateralRiskRating::Good => CollateralRiskRating::Good,
            DbCollateralRiskRating::Average => CollateralRiskRating::Average,
            DbCollateralRiskRating::Poor => CollateralRiskRating::Poor,
            DbCollateralRiskRating::Unacceptable => CollateralRiskRating::Unacceptable,
        }
    }

    fn collateral_status_to_db(cs: CollateralStatus) -> DbCollateralStatus {
        match cs {
            CollateralStatus::Active => DbCollateralStatus::Active,
            CollateralStatus::Released => DbCollateralStatus::Released,
            CollateralStatus::PartiallyReleased => DbCollateralStatus::PartiallyReleased,
            CollateralStatus::UnderReview => DbCollateralStatus::UnderReview,
            CollateralStatus::Disputed => DbCollateralStatus::Disputed,
            CollateralStatus::Liquidated => DbCollateralStatus::Liquidated,
            CollateralStatus::Impaired => DbCollateralStatus::Impaired,
            CollateralStatus::Substituted => DbCollateralStatus::Substituted,
        }
    }

    fn db_to_collateral_status(cs: DbCollateralStatus) -> CollateralStatus {
        match cs {
            DbCollateralStatus::Active => CollateralStatus::Active,
            DbCollateralStatus::Released => CollateralStatus::Released,
            DbCollateralStatus::PartiallyReleased => CollateralStatus::PartiallyReleased,
            DbCollateralStatus::UnderReview => CollateralStatus::UnderReview,
            DbCollateralStatus::Disputed => CollateralStatus::Disputed,
            DbCollateralStatus::Liquidated => CollateralStatus::Liquidated,
            DbCollateralStatus::Impaired => CollateralStatus::Impaired,
            DbCollateralStatus::Substituted => CollateralStatus::Substituted,
        }
    }

    fn valuation_method_to_db(vm: ValuationMethod) -> DbValuationMethod {
        match vm {
            ValuationMethod::MarketComparison => DbValuationMethod::MarketComparison,
            ValuationMethod::IncomeApproach => DbValuationMethod::IncomeApproach,
            ValuationMethod::CostApproach => DbValuationMethod::CostApproach,
            ValuationMethod::ExpertAppraisal => DbValuationMethod::ExpertAppraisal,
            ValuationMethod::AuctionValue => DbValuationMethod::AuctionValue,
            ValuationMethod::BookValue => DbValuationMethod::BookValue,
            ValuationMethod::MarkToMarket => DbValuationMethod::MarkToMarket,
        }
    }

    fn db_to_valuation_method(vm: DbValuationMethod) -> ValuationMethod {
        match vm {
            DbValuationMethod::MarketComparison => ValuationMethod::MarketComparison,
            DbValuationMethod::IncomeApproach => ValuationMethod::IncomeApproach,
            DbValuationMethod::CostApproach => ValuationMethod::CostApproach,
            DbValuationMethod::ExpertAppraisal => ValuationMethod::ExpertAppraisal,
            DbValuationMethod::AuctionValue => ValuationMethod::AuctionValue,
            DbValuationMethod::BookValue => ValuationMethod::BookValue,
            DbValuationMethod::MarkToMarket => ValuationMethod::MarkToMarket,
        }
    }

    fn pledge_priority_to_db(pp: PledgePriority) -> DbPledgePriority {
        match pp {
            PledgePriority::First => DbPledgePriority::First,
            PledgePriority::Second => DbPledgePriority::Second,
            PledgePriority::Third => DbPledgePriority::Third,
            PledgePriority::Subordinate => DbPledgePriority::Subordinate,
        }
    }

    fn db_to_pledge_priority(pp: DbPledgePriority) -> PledgePriority {
        match pp {
            DbPledgePriority::First => PledgePriority::First,
            DbPledgePriority::Second => PledgePriority::Second,
            DbPledgePriority::Third => PledgePriority::Third,
            DbPledgePriority::Subordinate => PledgePriority::Subordinate,
        }
    }

    fn pledge_status_to_db(ps: PledgeStatus) -> DbPledgeStatus {
        match ps {
            PledgeStatus::Active => DbPledgeStatus::Active,
            PledgeStatus::Released => DbPledgeStatus::Released,
            PledgeStatus::PartiallyReleased => DbPledgeStatus::PartiallyReleased,
            PledgeStatus::Substituted => DbPledgeStatus::Substituted,
            PledgeStatus::Disputed => DbPledgeStatus::Disputed,
            PledgeStatus::UnderEnforcement => DbPledgeStatus::UnderEnforcement,
        }
    }

    fn db_to_pledge_status(ps: DbPledgeStatus) -> PledgeStatus {
        match ps {
            DbPledgeStatus::Active => PledgeStatus::Active,
            DbPledgeStatus::Released => PledgeStatus::Released,
            DbPledgeStatus::PartiallyReleased => PledgeStatus::PartiallyReleased,
            DbPledgeStatus::Substituted => PledgeStatus::Substituted,
            DbPledgeStatus::Disputed => PledgeStatus::Disputed,
            DbPledgeStatus::UnderEnforcement => PledgeStatus::UnderEnforcement,
        }
    }

    fn collateral_alert_type_to_db(cat: CollateralAlertType) -> DbCollateralAlertType {
        match cat {
            CollateralAlertType::ValuationDue => DbCollateralAlertType::ValuationDue,
            CollateralAlertType::ValuationOverdue => DbCollateralAlertType::ValuationOverdue,
            CollateralAlertType::InsuranceExpiring => DbCollateralAlertType::InsuranceExpiring,
            CollateralAlertType::InsuranceExpired => DbCollateralAlertType::InsuranceExpired,
            CollateralAlertType::PerfectionExpiring => DbCollateralAlertType::PerfectionExpiring,
            CollateralAlertType::PerfectionExpired => DbCollateralAlertType::PerfectionExpired,
            CollateralAlertType::LtvBreach => DbCollateralAlertType::LtvBreach,
            CollateralAlertType::CovenantBreach => DbCollateralAlertType::CovenantBreach,
            CollateralAlertType::MaintenanceRequired => DbCollateralAlertType::MaintenanceRequired,
            CollateralAlertType::DocumentationMissing => DbCollateralAlertType::DocumentationMissing,
            CollateralAlertType::EnvironmentalRisk => DbCollateralAlertType::EnvironmentalRisk,
            CollateralAlertType::MarketValueDecline => DbCollateralAlertType::MarketValueDecline,
        }
    }

    fn db_to_collateral_alert_type(cat: DbCollateralAlertType) -> CollateralAlertType {
        match cat {
            DbCollateralAlertType::ValuationDue => CollateralAlertType::ValuationDue,
            DbCollateralAlertType::ValuationOverdue => CollateralAlertType::ValuationOverdue,
            DbCollateralAlertType::InsuranceExpiring => CollateralAlertType::InsuranceExpiring,
            DbCollateralAlertType::InsuranceExpired => CollateralAlertType::InsuranceExpired,
            DbCollateralAlertType::PerfectionExpiring => CollateralAlertType::PerfectionExpiring,
            DbCollateralAlertType::PerfectionExpired => CollateralAlertType::PerfectionExpired,
            DbCollateralAlertType::LtvBreach => CollateralAlertType::LtvBreach,
            DbCollateralAlertType::CovenantBreach => CollateralAlertType::CovenantBreach,
            DbCollateralAlertType::MaintenanceRequired => CollateralAlertType::MaintenanceRequired,
            DbCollateralAlertType::DocumentationMissing => CollateralAlertType::DocumentationMissing,
            DbCollateralAlertType::EnvironmentalRisk => CollateralAlertType::EnvironmentalRisk,
            DbCollateralAlertType::MarketValueDecline => CollateralAlertType::MarketValueDecline,
        }
    }

    fn alert_severity_to_db(als: AlertSeverity) -> DbAlertSeverity {
        match als {
            AlertSeverity::Low => DbAlertSeverity::Low,
            AlertSeverity::Medium => DbAlertSeverity::Medium,
            AlertSeverity::High => DbAlertSeverity::High,
            AlertSeverity::Critical => DbAlertSeverity::Critical,
        }
    }

    fn db_to_alert_severity(als: DbAlertSeverity) -> AlertSeverity {
        match als {
            DbAlertSeverity::Low => AlertSeverity::Low,
            DbAlertSeverity::Medium => AlertSeverity::Medium,
            DbAlertSeverity::High => AlertSeverity::High,
            DbAlertSeverity::Critical => AlertSeverity::Critical,
        }
    }

    fn collateral_alert_status_to_db(cas: CollateralAlertStatus) -> DbCollateralAlertStatus {
        match cas {
            CollateralAlertStatus::Open => DbCollateralAlertStatus::Open,
            CollateralAlertStatus::InProgress => DbCollateralAlertStatus::InProgress,
            CollateralAlertStatus::Resolved => DbCollateralAlertStatus::Resolved,
            CollateralAlertStatus::Dismissed => DbCollateralAlertStatus::Dismissed,
            CollateralAlertStatus::Escalated => DbCollateralAlertStatus::Escalated,
        }
    }

    fn db_to_collateral_alert_status(cas: DbCollateralAlertStatus) -> CollateralAlertStatus {
        match cas {
            DbCollateralAlertStatus::Open => CollateralAlertStatus::Open,
            DbCollateralAlertStatus::InProgress => CollateralAlertStatus::InProgress,
            DbCollateralAlertStatus::Resolved => CollateralAlertStatus::Resolved,
            DbCollateralAlertStatus::Dismissed => CollateralAlertStatus::Dismissed,
            DbCollateralAlertStatus::Escalated => CollateralAlertStatus::Escalated,
        }
    }

    fn enforcement_type_to_db(et: EnforcementType) -> DbEnforcementType {
        match et {
            EnforcementType::PrivateSale => DbEnforcementType::PrivateSale,
            EnforcementType::PublicAuction => DbEnforcementType::PublicAuction,
            EnforcementType::CourtOrdered => DbEnforcementType::CourtOrdered,
            EnforcementType::SelfHelp => DbEnforcementType::SelfHelp,
            EnforcementType::ForeClosureAction => DbEnforcementType::ForeClosureAction,
            EnforcementType::Repossession => DbEnforcementType::Repossession,
        }
    }

    fn db_to_enforcement_type(et: DbEnforcementType) -> EnforcementType {
        match et {
            DbEnforcementType::PrivateSale => EnforcementType::PrivateSale,
            DbEnforcementType::PublicAuction => EnforcementType::PublicAuction,
            DbEnforcementType::CourtOrdered => EnforcementType::CourtOrdered,
            DbEnforcementType::SelfHelp => EnforcementType::SelfHelp,
            DbEnforcementType::ForeClosureAction => EnforcementType::ForeClosureAction,
            DbEnforcementType::Repossession => EnforcementType::Repossession,
        }
    }

    fn enforcement_method_to_db(em: EnforcementMethod) -> DbEnforcementMethod {
        match em {
            EnforcementMethod::DirectSale => DbEnforcementMethod::DirectSale,
            EnforcementMethod::AuctionHouse => DbEnforcementMethod::AuctionHouse,
            EnforcementMethod::OnlinePlatform => DbEnforcementMethod::OnlinePlatform,
            EnforcementMethod::BrokerSale => DbEnforcementMethod::BrokerSale,
            EnforcementMethod::CourtSale => DbEnforcementMethod::CourtSale,
            EnforcementMethod::AssetManagementCompany => DbEnforcementMethod::AssetManagementCompany,
        }
    }

    fn db_to_enforcement_method(em: DbEnforcementMethod) -> EnforcementMethod {
        match em {
            DbEnforcementMethod::DirectSale => EnforcementMethod::DirectSale,
            DbEnforcementMethod::AuctionHouse => EnforcementMethod::AuctionHouse,
            DbEnforcementMethod::OnlinePlatform => EnforcementMethod::OnlinePlatform,
            DbEnforcementMethod::BrokerSale => EnforcementMethod::BrokerSale,
            DbEnforcementMethod::CourtSale => EnforcementMethod::CourtSale,
            DbEnforcementMethod::AssetManagementCompany => EnforcementMethod::AssetManagementCompany,
        }
    }

    fn enforcement_status_to_db(es: EnforcementStatus) -> DbEnforcementStatus {
        match es {
            EnforcementStatus::Initiated => DbEnforcementStatus::Initiated,
            EnforcementStatus::InProgress => DbEnforcementStatus::InProgress,
            EnforcementStatus::Completed => DbEnforcementStatus::Completed,
            EnforcementStatus::Suspended => DbEnforcementStatus::Suspended,
            EnforcementStatus::Cancelled => DbEnforcementStatus::Cancelled,
            EnforcementStatus::UnderLegalReview => DbEnforcementStatus::UnderLegalReview,
        }
    }

    fn db_to_enforcement_status(es: DbEnforcementStatus) -> EnforcementStatus {
        match es {
            DbEnforcementStatus::Initiated => EnforcementStatus::Initiated,
            DbEnforcementStatus::InProgress => EnforcementStatus::InProgress,
            DbEnforcementStatus::Completed => EnforcementStatus::Completed,
            DbEnforcementStatus::Suspended => EnforcementStatus::Suspended,
            DbEnforcementStatus::Cancelled => EnforcementStatus::Cancelled,
            DbEnforcementStatus::UnderLegalReview => EnforcementStatus::UnderLegalReview,
        }
    }
}
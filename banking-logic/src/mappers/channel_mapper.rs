use banking_api::domain::channel::{
    Channel, ChannelStatus, FeeSchedule, ReconciliationReport, 
    Discrepancy, ChannelFeeType, ChannelFeeCalculationMethod, ReconciliationStatus,
    FeeItem, ChannelFeeTier
};
use banking_db::models::channel::{
    ChannelModel, FeeScheduleModel, FeeItemModel, FeeTierModel,
    ChannelReconciliationReportModel, ReconciliationDiscrepancyModel
};
use chrono::Utc;

/// Channel mapper for converting between domain and database models
pub struct ChannelMapper;

impl ChannelMapper {
    /// Convert domain Channel to database ChannelModel
    pub fn to_channel_model(channel: Channel) -> ChannelModel {
        let status = match channel.status {
            ChannelStatus::Active => banking_db::models::channel::ChannelStatus::Active,
            ChannelStatus::Inactive => banking_db::models::channel::ChannelStatus::Inactive,
            ChannelStatus::Maintenance => banking_db::models::channel::ChannelStatus::Maintenance,
            ChannelStatus::Suspended => banking_db::models::channel::ChannelStatus::Suspended,
        };

        ChannelModel {
            id: channel.id,
            channel_code: channel.channel_code,
            channel_name: channel.channel_name,
            channel_type: format!("{:?}", channel.channel_type),
            status,
            daily_limit: channel.daily_limit,
            per_transaction_limit: channel.per_transaction_limit,
            supported_currencies: channel.supported_currencies,
            requires_additional_auth: channel.requires_additional_auth,
            fee_schedule_id: channel.fee_schedule_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Convert database ChannelModel to domain Channel
    pub fn from_channel_model(model: ChannelModel) -> Result<Channel, String> {
        use banking_api::domain::channel::Channel;
        use banking_api::domain::transaction::ChannelType;

        let channel_type = match model.channel_type.as_str() {
            "BranchTeller" => ChannelType::BranchTeller,
            "ATM" => ChannelType::ATM,
            "InternetBanking" => ChannelType::InternetBanking,
            "MobileApp" => ChannelType::MobileApp,
            "AgentTerminal" => ChannelType::AgentTerminal,
            "USSD" => ChannelType::USSD,
            "ApiGateway" => ChannelType::ApiGateway,
            _ => return Err(format!("Unknown channel type: {}", model.channel_type)),
        };

        let status = match model.status {
            banking_db::models::channel::ChannelStatus::Active => ChannelStatus::Active,
            banking_db::models::channel::ChannelStatus::Inactive => ChannelStatus::Inactive,
            banking_db::models::channel::ChannelStatus::Maintenance => ChannelStatus::Maintenance,
            banking_db::models::channel::ChannelStatus::Suspended => ChannelStatus::Suspended,
        };

        Ok(Channel {
            id: model.id,
            channel_code: model.channel_code,
            channel_name: model.channel_name,
            channel_type,
            status,
            daily_limit: model.daily_limit,
            per_transaction_limit: model.per_transaction_limit,
            supported_currencies: model.supported_currencies,
            requires_additional_auth: model.requires_additional_auth,
            fee_schedule_id: model.fee_schedule_id,
        })
    }


    /// Convert domain FeeSchedule to database FeeScheduleModel
    pub fn to_fee_schedule_model(schedule: FeeSchedule) -> FeeScheduleModel {
        FeeScheduleModel {
            id: schedule.id,
            schedule_name: schedule.schedule_name,
            channel_id: Some(schedule.channel_id),
            effective_date: schedule.effective_date,
            expiry_date: schedule.expiry_date,
            currency: schedule.currency,
            is_active: schedule.is_active,
            created_at: schedule.created_at,
            updated_at: schedule.updated_at,
        }
    }

    /// Convert database FeeScheduleModel to domain FeeSchedule
    pub fn from_fee_schedule_model(model: FeeScheduleModel, fee_items: Vec<FeeItem>) -> Result<FeeSchedule, String> {
        Ok(FeeSchedule {
            id: model.id,
            schedule_name: model.schedule_name,
            channel_id: model.channel_id.ok_or("Channel ID is required for fee schedule")?,
            effective_date: model.effective_date,
            expiry_date: model.expiry_date,
            currency: model.currency,
            fee_items,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        })
    }

    /// Convert domain ReconciliationReport to database ChannelReconciliationReportModel
    pub fn to_reconciliation_report_model(report: ReconciliationReport) -> ChannelReconciliationReportModel {
        let status = match report.status {
            ReconciliationStatus::InProgress => banking_db::models::channel::ReconciliationStatus::InProgress,
            ReconciliationStatus::Completed => banking_db::models::channel::ReconciliationStatus::Completed,
            ReconciliationStatus::Failed => banking_db::models::channel::ReconciliationStatus::Failed,
            ReconciliationStatus::RequiresManualReview => banking_db::models::channel::ReconciliationStatus::RequiresManualReview,
        };

        ChannelReconciliationReportModel {
            id: report.id,
            channel_id: report.channel_id,
            reconciliation_date: report.reconciliation_date,
            total_transactions: report.total_transactions,
            total_amount: report.total_amount,
            status,
            generated_at: report.generated_at,
            completed_at: None,
            created_at: Utc::now(),
        }
    }

    /// Convert database ChannelReconciliationReportModel to domain ReconciliationReport
    pub fn from_reconciliation_report_model(
        model: ChannelReconciliationReportModel,
        discrepancies: Vec<Discrepancy>
    ) -> ReconciliationReport {
        let status = match model.status {
            banking_db::models::channel::ReconciliationStatus::InProgress => ReconciliationStatus::InProgress,
            banking_db::models::channel::ReconciliationStatus::Completed => ReconciliationStatus::Completed,
            banking_db::models::channel::ReconciliationStatus::Failed => ReconciliationStatus::Failed,
            banking_db::models::channel::ReconciliationStatus::RequiresManualReview => ReconciliationStatus::RequiresManualReview,
        };

        ReconciliationReport {
            id: model.id,
            channel_id: model.channel_id,
            reconciliation_date: model.reconciliation_date,
            total_transactions: model.total_transactions,
            total_amount: model.total_amount,
            discrepancies,
            status,
            generated_at: model.generated_at,
        }
    }

    /// Convert database ReconciliationDiscrepancyModel to domain Discrepancy
    pub fn from_discrepancy_model(model: ReconciliationDiscrepancyModel) -> Discrepancy {
        use heapless::String as HeaplessString;
        
        // Convert HeaplessString<500> to HeaplessString<200> by truncating if necessary
        let description: HeaplessString<200> = if model.description.len() > 200 {
            let truncated = model.description.as_str().chars().take(200).collect::<String>();
            HeaplessString::try_from(truncated.as_str()).unwrap_or_default()
        } else {
            HeaplessString::try_from(model.description.as_str()).unwrap_or_default()
        };

        Discrepancy {
            transaction_id: model.id,
            description,
            expected_amount: model.expected_amount,
            actual_amount: model.actual_amount,
            difference: model.difference,
        }
    }

    /// Convert domain FeeItem to database FeeItemModel
    pub fn to_fee_item_model(item: FeeItem, schedule_id: uuid::Uuid) -> FeeItemModel {
        let fee_type = match item.fee_type {
            ChannelFeeType::TransactionFee => banking_db::models::channel::ChannelFeeType::TransactionFee,
            ChannelFeeType::MaintenanceFee => banking_db::models::channel::ChannelFeeType::MaintenanceFee,
            ChannelFeeType::ServiceFee => banking_db::models::channel::ChannelFeeType::ServiceFee,
            ChannelFeeType::PenaltyFee => banking_db::models::channel::ChannelFeeType::PenaltyFee,
            ChannelFeeType::ProcessingFee => banking_db::models::channel::ChannelFeeType::ProcessingFee,
            ChannelFeeType::ComplianceFee => banking_db::models::channel::ChannelFeeType::ComplianceFee,
            ChannelFeeType::InterchangeFee => banking_db::models::channel::ChannelFeeType::InterchangeFee,
            ChannelFeeType::NetworkFee => banking_db::models::channel::ChannelFeeType::NetworkFee,
        };

        let calculation_method = match item.calculation_method {
            ChannelFeeCalculationMethod::Fixed => banking_db::models::channel::ChannelFeeCalculationMethod::Fixed,
            ChannelFeeCalculationMethod::Percentage => banking_db::models::channel::ChannelFeeCalculationMethod::Percentage,
            ChannelFeeCalculationMethod::Tiered => banking_db::models::channel::ChannelFeeCalculationMethod::Tiered,
            ChannelFeeCalculationMethod::BalanceBased => banking_db::models::channel::ChannelFeeCalculationMethod::BalanceBased,
            ChannelFeeCalculationMethod::RuleBased => banking_db::models::channel::ChannelFeeCalculationMethod::RuleBased,
            ChannelFeeCalculationMethod::Hybrid => banking_db::models::channel::ChannelFeeCalculationMethod::Hybrid,
        };

        FeeItemModel {
            id: item.id,
            schedule_id,
            fee_code: item.fee_code,
            fee_name: item.fee_name,
            fee_type,
            calculation_method,
            fee_amount: item.fee_amount,
            fee_percentage: item.fee_percentage,
            minimum_fee: item.minimum_fee,
            maximum_fee: item.maximum_fee,
            applies_to_transaction_types: item.applies_to_transaction_types,
            is_waivable: item.is_waivable,
            requires_approval_for_waiver: item.requires_approval_for_waiver,
            created_at: Utc::now(),
        }
    }

    /// Convert database FeeItemModel to domain FeeItem
    pub fn from_fee_item_model(model: FeeItemModel, fee_tiers: Vec<ChannelFeeTier>) -> FeeItem {
        let fee_type = match model.fee_type {
            banking_db::models::channel::ChannelFeeType::TransactionFee => ChannelFeeType::TransactionFee,
            banking_db::models::channel::ChannelFeeType::MaintenanceFee => ChannelFeeType::MaintenanceFee,
            banking_db::models::channel::ChannelFeeType::ServiceFee => ChannelFeeType::ServiceFee,
            banking_db::models::channel::ChannelFeeType::PenaltyFee => ChannelFeeType::PenaltyFee,
            banking_db::models::channel::ChannelFeeType::ProcessingFee => ChannelFeeType::ProcessingFee,
            banking_db::models::channel::ChannelFeeType::ComplianceFee => ChannelFeeType::ComplianceFee,
            banking_db::models::channel::ChannelFeeType::InterchangeFee => ChannelFeeType::InterchangeFee,
            banking_db::models::channel::ChannelFeeType::NetworkFee => ChannelFeeType::NetworkFee,
        };

        let calculation_method = match model.calculation_method {
            banking_db::models::channel::ChannelFeeCalculationMethod::Fixed => ChannelFeeCalculationMethod::Fixed,
            banking_db::models::channel::ChannelFeeCalculationMethod::Percentage => ChannelFeeCalculationMethod::Percentage,
            banking_db::models::channel::ChannelFeeCalculationMethod::Tiered => ChannelFeeCalculationMethod::Tiered,
            banking_db::models::channel::ChannelFeeCalculationMethod::BalanceBased => ChannelFeeCalculationMethod::BalanceBased,
            banking_db::models::channel::ChannelFeeCalculationMethod::RuleBased => ChannelFeeCalculationMethod::RuleBased,
            banking_db::models::channel::ChannelFeeCalculationMethod::Hybrid => ChannelFeeCalculationMethod::Hybrid,
        };

        FeeItem {
            id: model.id,
            fee_code: model.fee_code,
            fee_name: model.fee_name,
            fee_type,
            calculation_method,
            fee_amount: model.fee_amount,
            fee_percentage: model.fee_percentage,
            minimum_fee: model.minimum_fee,
            maximum_fee: model.maximum_fee,
            fee_tiers,
            applies_to_transaction_types: model.applies_to_transaction_types,
            is_waivable: model.is_waivable,
            requires_approval_for_waiver: model.requires_approval_for_waiver,
        }
    }

    /// Convert database FeeTierModel to domain ChannelFeeTier
    pub fn from_fee_tier_model(model: FeeTierModel) -> ChannelFeeTier {
        ChannelFeeTier {
            id: model.id,
            tier_name: model.tier_name,
            min_amount: model.min_amount,
            max_amount: model.max_amount,
            fee_amount: model.fee_amount,
            fee_percentage: model.fee_percentage,
            tier_order: model.tier_order,
        }
    }

    /// Convert domain ChannelFeeTier to database FeeTierModel
    pub fn to_fee_tier_model(tier: ChannelFeeTier, fee_item_id: uuid::Uuid) -> FeeTierModel {
        FeeTierModel {
            id: tier.id,
            fee_item_id,
            tier_name: tier.tier_name,
            min_amount: tier.min_amount,
            max_amount: tier.max_amount,
            fee_amount: tier.fee_amount,
            fee_percentage: tier.fee_percentage,
            tier_order: tier.tier_order,
            created_at: Utc::now(),
        }
    }
}
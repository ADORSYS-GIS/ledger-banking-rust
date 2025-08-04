use banking_api::domain::{ReasonAndPurpose, ComplianceMetadata, ReasonReference};
use banking_db::models::{ReasonAndPurpose as ReasonAndPurposeModel, ComplianceMetadata as ComplianceMetadataModel, ReasonReference as ReasonReferenceModel};

pub struct ReasonAndPurposeMapper;

impl ReasonAndPurposeMapper {
    /// Map from domain ReasonAndPurpose to database ReasonAndPurposeModel
    pub fn to_model(reason: ReasonAndPurpose) -> ReasonAndPurposeModel {
        ReasonAndPurposeModel {
            id: reason.id,
            code: reason.code,
            category: reason.category,
            context: reason.context,
            l1_content: reason.l1_content,
            l2_content: reason.l2_content,
            l3_content: reason.l3_content,
            l1_language_code: reason.l1_language_code,
            l2_language_code: reason.l2_language_code,
            l3_language_code: reason.l3_language_code,
            requires_details: reason.requires_details,
            is_active: reason.is_active,
            severity: reason.severity,
            display_order: reason.display_order,
            compliance_metadata: reason.compliance_metadata.map(Self::compliance_metadata_to_model),
            created_at: reason.created_at,
            updated_at: reason.updated_at,
            created_by: reason.created_by,
            updated_by: reason.updated_by,
        }
    }

    /// Map from database ReasonAndPurposeModel to domain ReasonAndPurpose
    pub fn to_domain(model: ReasonAndPurposeModel) -> ReasonAndPurpose {
        ReasonAndPurpose {
            id: model.id,
            code: model.code,
            category: model.category,
            context: model.context,
            l1_content: model.l1_content,
            l2_content: model.l2_content,
            l3_content: model.l3_content,
            l1_language_code: model.l1_language_code,
            l2_language_code: model.l2_language_code,
            l3_language_code: model.l3_language_code,
            requires_details: model.requires_details,
            is_active: model.is_active,
            severity: model.severity,
            display_order: model.display_order,
            compliance_metadata: model.compliance_metadata.map(Self::compliance_metadata_to_domain),
            created_at: model.created_at,
            updated_at: model.updated_at,
            created_by: model.created_by,
            updated_by: model.updated_by,
        }
    }

    /// Map from domain ComplianceMetadata to database ComplianceMetadataModel
    pub fn compliance_metadata_to_model(metadata: ComplianceMetadata) -> ComplianceMetadataModel {
        ComplianceMetadataModel {
            regulatory_code: metadata.regulatory_code,
            reportable: metadata.reportable,
            requires_sar: metadata.requires_sar,
            requires_ctr: metadata.requires_ctr,
            retention_years: metadata.retention_years,
            escalation_required: metadata.escalation_required,
            risk_score_impact: metadata.risk_score_impact,
            no_tipping_off: metadata.no_tipping_off,
            jurisdictions: metadata.jurisdictions,
        }
    }

    /// Map from database ComplianceMetadataModel to domain ComplianceMetadata
    pub fn compliance_metadata_to_domain(model: ComplianceMetadataModel) -> ComplianceMetadata {
        ComplianceMetadata {
            regulatory_code: model.regulatory_code,
            reportable: model.reportable,
            requires_sar: model.requires_sar,
            requires_ctr: model.requires_ctr,
            retention_years: model.retention_years,
            escalation_required: model.escalation_required,
            risk_score_impact: model.risk_score_impact,
            no_tipping_off: model.no_tipping_off,
            jurisdictions: model.jurisdictions,
        }
    }

    /// Map from domain ReasonReference to database ReasonReferenceModel
    pub fn reason_reference_to_model(reference: ReasonReference) -> ReasonReferenceModel {
        ReasonReferenceModel {
            reason_id: reference.reason_id,
            additional_details: reference.additional_details,
            referenced_at: reference.referenced_at,
            referenced_by: reference.referenced_by,
        }
    }

    /// Map from database ReasonReferenceModel to domain ReasonReference
    pub fn reason_reference_to_domain(model: ReasonReferenceModel) -> ReasonReference {
        ReasonReference {
            reason_id: model.reason_id,
            additional_details: model.additional_details,
            referenced_at: model.referenced_at,
            referenced_by: model.referenced_by,
        }
    }

    /// Map a vector of database models to domain objects
    pub fn to_domain_list(models: Vec<ReasonAndPurposeModel>) -> Vec<ReasonAndPurpose> {
        models.into_iter().map(Self::to_domain).collect()
    }

    /// Map a vector of domain objects to database models
    pub fn to_model_list(reasons: Vec<ReasonAndPurpose>) -> Vec<ReasonAndPurposeModel> {
        reasons.into_iter().map(Self::to_model).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use banking_api::domain::{ReasonCategory, ReasonContext, ReasonSeverity};
    use chrono::Utc;
    use heapless::{String as HeaplessString, Vec as HeaplessVec};
    use uuid::Uuid;

    #[test]
    fn test_reason_and_purpose_mapping() {
        let original = ReasonAndPurpose {
            id: Uuid::new_v4(),
            code: HeaplessString::try_from("TEST_REASON").unwrap(),
            category: ReasonCategory::LoanPurpose,
            context: ReasonContext::Loan,
            l1_content: Some(HeaplessString::try_from("Test Reason").unwrap()),
            l2_content: None,
            l3_content: None,
            l1_language_code: Some([b'e', b'n', b'g']),
            l2_language_code: None,
            l3_language_code: None,
            requires_details: true,
            is_active: true,
            severity: Some(ReasonSeverity::High),
            display_order: 10,
            compliance_metadata: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: HeaplessString::try_from("test_user").unwrap(),
            updated_by: HeaplessString::try_from("test_user").unwrap(),
        };

        // Test domain -> model -> domain roundtrip
        let model = ReasonAndPurposeMapper::to_model(original.clone());
        let converted_back = ReasonAndPurposeMapper::to_domain(model);

        assert_eq!(original.id, converted_back.id);
        assert_eq!(original.code, converted_back.code);
        assert_eq!(original.category, converted_back.category);
        assert_eq!(original.context, converted_back.context);
        assert_eq!(original.requires_details, converted_back.requires_details);
        assert_eq!(original.is_active, converted_back.is_active);
        assert_eq!(original.severity, converted_back.severity);
        assert_eq!(original.display_order, converted_back.display_order);
    }

    #[test]
    fn test_compliance_metadata_mapping() {
        let mut jurisdictions = HeaplessVec::new();
        jurisdictions.push([b'U', b'S']).unwrap();
        jurisdictions.push([b'C', b'A']).unwrap();

        let original = ComplianceMetadata {
            regulatory_code: Some(HeaplessString::try_from("BSA-3.14").unwrap()),
            reportable: true,
            requires_sar: true,
            requires_ctr: false,
            retention_years: 7,
            escalation_required: true,
            risk_score_impact: Some(25),
            no_tipping_off: true,
            jurisdictions,
        };

        // Test domain -> model -> domain roundtrip
        let model = ReasonAndPurposeMapper::compliance_metadata_to_model(original.clone());
        let converted_back = ReasonAndPurposeMapper::compliance_metadata_to_domain(model);

        assert_eq!(original.regulatory_code, converted_back.regulatory_code);
        assert_eq!(original.reportable, converted_back.reportable);
        assert_eq!(original.requires_sar, converted_back.requires_sar);
        assert_eq!(original.requires_ctr, converted_back.requires_ctr);
        assert_eq!(original.retention_years, converted_back.retention_years);
        assert_eq!(original.escalation_required, converted_back.escalation_required);
        assert_eq!(original.risk_score_impact, converted_back.risk_score_impact);
        assert_eq!(original.no_tipping_off, converted_back.no_tipping_off);
        assert_eq!(original.jurisdictions.len(), converted_back.jurisdictions.len());
    }
}
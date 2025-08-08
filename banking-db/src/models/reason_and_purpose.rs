use chrono::{DateTime, Utc};
use heapless::{String as HeaplessString, Vec as HeaplessVec};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;
use banking_api::domain::{ReasonCategory, ReasonContext, ReasonSeverity};

/// Database model for ReasonAndPurpose table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct ReasonAndPurpose {
    pub id: Uuid,
    
    /// Unique identifier code for programmatic reference
    pub code: HeaplessString<50>,  // e.g., "LOAN_PURPOSE_HOME_PURCHASE"
    
    /// Category to group related reasons
    #[serde(
        serialize_with = "serialize_reason_category",
        deserialize_with = "deserialize_reason_category"
    )]
    pub category: ReasonCategory,
    
    /// Context where this reason is used
    #[serde(
        serialize_with = "serialize_reason_context",
        deserialize_with = "deserialize_reason_context"
    )]
    pub context: ReasonContext,
    
    /// Language content - up to 3 languages supported
    pub l1_content: Option<HeaplessString<100>>,  // Primary language
    pub l2_content: Option<HeaplessString<100>>,  // Secondary language
    pub l3_content: Option<HeaplessString<100>>,  // Tertiary language
    
    /// Language codes for each content field
    pub l1_language_code: Option<[u8; 3]>,  // e.g., "eng"
    pub l2_language_code: Option<[u8; 3]>,  // e.g., "fra"
    pub l3_language_code: Option<[u8; 3]>,  // e.g., "swa"
    
    /// Whether this reason requires additional details
    pub requires_details: bool,
    
    /// Whether this reason is currently active
    pub is_active: bool,
    
    /// Severity or importance level
    #[serde(
        serialize_with = "serialize_reason_severity_option",
        deserialize_with = "deserialize_reason_severity_option"
    )]
    pub severity: Option<ReasonSeverity>,
    
    /// Sort order for UI display
    pub display_order: i32,
    
    /// Compliance-specific metadata (for AML/CTF/KYC reasons)
    pub compliance_metadata: Option<ComplianceMetadata>,
    
    /// Audit fields
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by_person_id: HeaplessString<100>,
    pub updated_by_person_id: HeaplessString<100>,
}

// Custom serialization functions for database compatibility
fn serialize_reason_category<S>(category: &ReasonCategory, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&category.to_string())
}

fn deserialize_reason_category<'de, D>(deserializer: D) -> Result<ReasonCategory, D::Error>
where
    D: Deserializer<'de>,
{
    let category_str = String::deserialize(deserializer)?;
    category_str.parse().map_err(serde::de::Error::custom)
}

fn serialize_reason_context<S>(context: &ReasonContext, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&context.to_string())
}

fn deserialize_reason_context<'de, D>(deserializer: D) -> Result<ReasonContext, D::Error>
where
    D: Deserializer<'de>,
{
    let context_str = String::deserialize(deserializer)?;
    context_str.parse().map_err(serde::de::Error::custom)
}

fn serialize_reason_severity_option<S>(severity: &Option<ReasonSeverity>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match severity {
        Some(severity) => serializer.serialize_str(&severity.to_string()),
        None => serializer.serialize_none(),
    }
}

fn deserialize_reason_severity_option<'de, D>(deserializer: D) -> Result<Option<ReasonSeverity>, D::Error>
where
    D: Deserializer<'de>,
{
    let option_str: Option<String> = Option::deserialize(deserializer)?;
    match option_str {
        Some(severity_str) => {
            let severity = severity_str.parse().map_err(serde::de::Error::custom)?;
            Ok(Some(severity))
        }
        None => Ok(None),
    }
}

// Note: ReasonCategory, ReasonContext, and ReasonSeverity enums are imported from banking_api::domain

/// Compliance-specific metadata for AML/CTF/KYC reasons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceMetadata {
    /// Regulatory reference code (e.g., "FATF-R.16", "BSA-3.14")
    pub regulatory_code: Option<HeaplessString<20>>,
    
    /// Whether this reason requires regulatory reporting
    pub reportable: bool,
    
    /// Whether this triggers a Suspicious Activity Report
    pub requires_sar: bool,
    
    /// Whether this triggers a Currency Transaction Report
    pub requires_ctr: bool,
    
    /// Minimum retention period in years for audit
    pub retention_years: u8,
    
    /// Whether management escalation is required
    pub escalation_required: bool,
    
    /// Risk score impact (0-100)
    pub risk_score_impact: Option<u8>,
    
    /// Whether customer notification is prohibited (tipping off)
    pub no_tipping_off: bool,
    
    /// Relevant jurisdiction codes
    pub jurisdictions: HeaplessVec<[u8; 2], 5>,  // Country codes
}

/// Additional details that can be attached to a reason reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonReference {
    pub reason_id: Uuid,
    pub additional_details: Option<HeaplessString<200>>,
    pub referenced_at: DateTime<Utc>,
    pub referenced_by: HeaplessString<100>,
}

impl ReasonAndPurpose {
    /// Get content in specified language, fallback to primary if not available
    pub fn get_content(&self, language_code: &[u8; 3]) -> Option<&str> {
        if self.l1_language_code.as_ref() == Some(language_code) {
            self.l1_content.as_deref()
        } else if self.l2_language_code.as_ref() == Some(language_code) {
            self.l2_content.as_deref()
        } else if self.l3_language_code.as_ref() == Some(language_code) {
            self.l3_content.as_deref()
        } else {
            // Fallback to primary language
            self.l1_content.as_deref()
        }
    }
    
    /// Get content with fallback chain
    pub fn get_content_with_fallback(&self, preferred_languages: &[[u8; 3]]) -> Option<&str> {
        for lang in preferred_languages {
            if let Some(content) = self.get_content(lang) {
                return Some(content);
            }
        }
        // Final fallback to any available content
        self.l1_content.as_deref()
            .or(self.l2_content.as_deref())
            .or(self.l3_content.as_deref())
    }
    
    /// Check if reason has content in specified language
    pub fn has_language(&self, language_code: &[u8; 3]) -> bool {
        self.l1_language_code.as_ref() == Some(language_code) ||
        self.l2_language_code.as_ref() == Some(language_code) ||
        self.l3_language_code.as_ref() == Some(language_code)
    }
}

/// Repository for managing reasons
pub struct ReasonAndPurposeRepository {
    reasons: std::collections::HashMap<Uuid, ReasonAndPurpose>,
    by_code: std::collections::HashMap<HeaplessString<50>, Uuid>,
    by_category: std::collections::HashMap<ReasonCategory, Vec<Uuid>>,
}

impl ReasonAndPurposeRepository {
    pub fn new() -> Self {
        Self {
            reasons: std::collections::HashMap::new(),
            by_code: std::collections::HashMap::new(),
            by_category: std::collections::HashMap::new(),
        }
    }
    
    pub fn get(&self, id: &Uuid) -> Option<&ReasonAndPurpose> {
        self.reasons.get(id)
    }
    
    pub fn get_by_code(&self, code: &str) -> Option<&ReasonAndPurpose> {
        let heap_code = HeaplessString::try_from(code).ok()?;
        self.by_code.get(&heap_code)
            .and_then(|id| self.reasons.get(id))
    }
    
    pub fn get_by_category(&self, category: ReasonCategory) -> Vec<&ReasonAndPurpose> {
        self.by_category.get(&category)
            .map(|ids| ids.iter()
                .filter_map(|id| self.reasons.get(id))
                .filter(|r| r.is_active)
                .collect())
            .unwrap_or_default()
    }
    
    pub fn get_for_context(&self, context: ReasonContext, category: ReasonCategory) -> Vec<&ReasonAndPurpose> {
        self.get_by_category(category)
            .into_iter()
            .filter(|r| r.context == context)
            .collect()
    }
    
    /// Get all compliance-related reasons that require reporting
    pub fn get_reportable_compliance_reasons(&self) -> Vec<&ReasonAndPurpose> {
        self.reasons.values()
            .filter(|r| {
                r.compliance_metadata.as_ref()
                    .map(|m| m.reportable)
                    .unwrap_or(false)
            })
            .collect()
    }
    
    /// Get reasons that trigger SAR (Suspicious Activity Report)
    pub fn get_sar_triggering_reasons(&self) -> Vec<&ReasonAndPurpose> {
        self.reasons.values()
            .filter(|r| {
                r.compliance_metadata.as_ref()
                    .map(|m| m.requires_sar)
                    .unwrap_or(false)
            })
            .collect()
    }
    
    /// Add a reason to the repository
    pub fn add(&mut self, reason: ReasonAndPurpose) -> Result<(), &'static str> {
        if self.reasons.contains_key(&reason.id) {
            return Err("Reason with this ID already exists");
        }
        
        if self.by_code.contains_key(&reason.code) {
            return Err("Reason with this code already exists");
        }
        
        let id = reason.id;
        let code = reason.code.clone();
        let category = reason.category;
        
        self.reasons.insert(id, reason);
        self.by_code.insert(code, id);
        
        self.by_category
            .entry(category)
            .or_default()
            .push(id);
        
        Ok(())
    }
}

impl Default for ReasonAndPurposeRepository {
    fn default() -> Self {
        Self::new()
    }
}
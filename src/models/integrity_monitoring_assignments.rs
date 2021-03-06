/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrityMonitoringAssignments {
    /// IDs of IntegrityMonitoringRules assigned to the computer or policy.
    #[serde(rename = "assignedRuleIDs", skip_serializing_if = "Option::is_none")]
    pub assigned_rule_ids: Option<Vec<i32>>,
    /// Status of the last recommendation scan done on a computer or policy.
    #[serde(
        rename = "recommendationScanStatus",
        skip_serializing_if = "Option::is_none"
    )]
    pub recommendation_scan_status: Option<RecommendationScanStatus>,
    /// Timestamp of the last recommendation scan date, in milliseconds since epoch.
    #[serde(
        rename = "lastRecommendationScanDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_recommendation_scan_date: Option<i64>,
    /// IntegrityMonitoringRules separated by commas, that were recommended to be assigned to the computer or policy by a recommendation scan.
    #[serde(
        rename = "recommendedToAssignRuleIDs",
        skip_serializing_if = "Option::is_none"
    )]
    pub recommended_to_assign_rule_ids: Option<Vec<i32>>,
    /// IntegrityMonitoringRules, separated by commas, that were recommended to be unassigned from the computer or policy by a recommendation scan.
    #[serde(
        rename = "recommendedToUnassignRuleIDs",
        skip_serializing_if = "Option::is_none"
    )]
    pub recommended_to_unassign_rule_ids: Option<Vec<i32>>,
}

impl IntegrityMonitoringAssignments {
    pub fn new() -> IntegrityMonitoringAssignments {
        IntegrityMonitoringAssignments {
            assigned_rule_ids: None,
            recommendation_scan_status: None,
            last_recommendation_scan_date: None,
            recommended_to_assign_rule_ids: None,
            recommended_to_unassign_rule_ids: None,
        }
    }
}

/// Status of the last recommendation scan done on a computer or policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RecommendationScanStatus {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "out-of-date")]
    OutOfDate,
    #[serde(rename = "unknown")]
    Unknown,
}

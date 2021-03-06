/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// LogInspectionRuleRights : Log inspection rule right details.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogInspectionRuleRights {
    /// Right to create new log inspection rules.
    #[serde(
        rename = "canCreateNewLogInspectionRules",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_create_new_log_inspection_rules: Option<bool>,
    /// Right to delete log inspection rules.
    #[serde(
        rename = "canDeleteLogInspectionRules",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_delete_log_inspection_rules: Option<bool>,
    /// Right to edit log inspection rule properties.
    #[serde(
        rename = "canEditLogInspectionRuleProperties",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_edit_log_inspection_rule_properties: Option<bool>,
}

impl LogInspectionRuleRights {
    /// Log inspection rule right details.
    pub fn new() -> LogInspectionRuleRights {
        LogInspectionRuleRights {
            can_create_new_log_inspection_rules: None,
            can_delete_log_inspection_rules: None,
            can_edit_log_inspection_rule_properties: None,
        }
    }
}

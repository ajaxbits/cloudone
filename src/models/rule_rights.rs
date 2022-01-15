/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RuleRights : Rule right details.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleRights {
    /// Right to create new rules.
    #[serde(rename = "canCreateNewAppControlRules", skip_serializing_if = "Option::is_none")]
    pub can_create_new_app_control_rules: Option<bool>,
    /// Right to delete a rule.
    #[serde(rename = "canDeleteAppControlRules", skip_serializing_if = "Option::is_none")]
    pub can_delete_app_control_rules: Option<bool>,
    /// Right to edit rule properties.
    #[serde(rename = "canEditAppControlRulesProperties", skip_serializing_if = "Option::is_none")]
    pub can_edit_app_control_rules_properties: Option<bool>,
    /// Right to view rules.
    #[serde(rename = "canViewAppControlRules", skip_serializing_if = "Option::is_none")]
    pub can_view_app_control_rules: Option<bool>,
}

impl RuleRights {
    /// Rule right details.
    pub fn new() -> RuleRights {
        RuleRights {
            can_create_new_app_control_rules: None,
            can_delete_app_control_rules: None,
            can_edit_app_control_rules_properties: None,
            can_view_app_control_rules: None,
        }
    }
}


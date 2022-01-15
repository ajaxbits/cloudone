/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IntrusionPreventionPolicyExtension : Policy-level configuration for the Intrusion Prevention module



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntrusionPreventionPolicyExtension {
    /// Module state. The 'inherited' value is write-only.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[serde(rename = "moduleStatus", skip_serializing_if = "Option::is_none")]
    pub module_status: Option<Box<crate::models::PolicyModuleStatus>>,
    /// IDs of the assigned Intrusion Prevention rules.
    #[serde(rename = "ruleIDs", skip_serializing_if = "Option::is_none")]
    pub rule_ids: Option<Vec<i32>>,
    /// IDs of the assigned Application Types.
    #[serde(rename = "applicationTypeIDs", skip_serializing_if = "Option::is_none")]
    pub application_type_ids: Option<Vec<i32>>,
}

impl IntrusionPreventionPolicyExtension {
    /// Policy-level configuration for the Intrusion Prevention module
    pub fn new() -> IntrusionPreventionPolicyExtension {
        IntrusionPreventionPolicyExtension {
            state: None,
            module_status: None,
            rule_ids: None,
            application_type_ids: None,
        }
    }
}

/// Module state. The 'inherited' value is write-only.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "inherited")]
    Inherited,
    #[serde(rename = "prevent")]
    Prevent,
    #[serde(rename = "detect")]
    Detect,
    #[serde(rename = "off")]
    Off,
}


/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// IntegrityMonitoringPolicyExtension : Policy-level configuration for the Integrity Monitoring module

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrityMonitoringPolicyExtension {
    /// Module state. The 'inherited' value is write-only.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[serde(rename = "moduleStatus", skip_serializing_if = "Option::is_none")]
    pub module_status: Option<Box<crate::models::PolicyModuleStatus>>,
    /// IDs of the assigned Integrity Monitoring rules.
    #[serde(rename = "ruleIDs", skip_serializing_if = "Option::is_none")]
    pub rule_ids: Option<Vec<i32>>,
}

impl IntegrityMonitoringPolicyExtension {
    /// Policy-level configuration for the Integrity Monitoring module
    pub fn new() -> IntegrityMonitoringPolicyExtension {
        IntegrityMonitoringPolicyExtension {
            state: None,
            module_status: None,
            rule_ids: None,
        }
    }
}

/// Module state. The 'inherited' value is write-only.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "inherited")]
    Inherited,
    #[serde(rename = "real-time")]
    RealTime,
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FirewallRights : Firewall rights details.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRights {
    #[serde(rename = "contextRights", skip_serializing_if = "Option::is_none")]
    pub context_rights: Option<Box<crate::models::ContextRights>>,
    #[serde(rename = "firewallRuleRights", skip_serializing_if = "Option::is_none")]
    pub firewall_rule_rights: Option<Box<crate::models::FirewallRuleRights>>,
    #[serde(rename = "statefulConfigurationRights", skip_serializing_if = "Option::is_none")]
    pub stateful_configuration_rights: Option<Box<crate::models::StatefulConfigurationRights>>,
    #[serde(rename = "macListRights", skip_serializing_if = "Option::is_none")]
    pub mac_list_rights: Option<Box<crate::models::MacListRights>>,
}

impl FirewallRights {
    /// Firewall rights details.
    pub fn new() -> FirewallRights {
        FirewallRights {
            context_rights: None,
            firewall_rule_rights: None,
            stateful_configuration_rights: None,
            mac_list_rights: None,
        }
    }
}


/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ComputerStatus : Computer status details.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerStatus {
    /// Agent status.
    #[serde(rename = "agentStatus", skip_serializing_if = "Option::is_none")]
    pub agent_status: Option<AgentStatus>,
    /// Agent status messages.
    #[serde(rename = "agentStatusMessages", skip_serializing_if = "Option::is_none")]
    pub agent_status_messages: Option<Vec<String>>,
    /// Appliance status.
    #[serde(rename = "applianceStatus", skip_serializing_if = "Option::is_none")]
    pub appliance_status: Option<ApplianceStatus>,
    /// Appliance status messages.
    #[serde(rename = "applianceStatusMessages", skip_serializing_if = "Option::is_none")]
    pub appliance_status_messages: Option<Vec<String>>,
}

impl ComputerStatus {
    /// Computer status details.
    pub fn new() -> ComputerStatus {
        ComputerStatus {
            agent_status: None,
            agent_status_messages: None,
            appliance_status: None,
            appliance_status_messages: None,
        }
    }
}

/// Agent status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AgentStatus {
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "not-supported")]
    NotSupported,
}
/// Appliance status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApplianceStatus {
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "not-supported")]
    NotSupported,
}

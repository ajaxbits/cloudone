/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// ApplicationControlComputerExtension : Computer-level configuration for the Application Control module. Null for tenants for whom Application Control is hidden.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationControlComputerExtension {
    /// Module state. The 'inherited' value is write-only.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[serde(rename = "moduleStatus", skip_serializing_if = "Option::is_none")]
    pub module_status: Option<Box<crate::models::ComputerModuleStatus>>,
    /// Controls whether to block unrecognized software until it is explicitly allowed. Set to true to block.
    #[serde(rename = "blockUnrecognized", skip_serializing_if = "Option::is_none")]
    pub block_unrecognized: Option<bool>,
    /// ID of the shared whitelist ruleset.
    #[serde(rename = "rulesetID", skip_serializing_if = "Option::is_none")]
    pub ruleset_id: Option<i64>,
    /// ID of the trust ruleset. An empty string will set it to \"None\" and assigning a value of 0 will set it to \"Inherited\".
    #[serde(rename = "trustRulesetID", skip_serializing_if = "Option::is_none")]
    pub trust_ruleset_id: Option<String>,
    /// Maintenance mode status.
    #[serde(
        rename = "maintenanceModeStatus",
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_mode_status: Option<MaintenanceModeStatus>,
    /// Duration of maintenance mode in minutes.
    #[serde(
        rename = "maintenanceModeDuration",
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_mode_duration: Option<i32>,
    /// Timestamp of the date the maintenanceMode was started, in milliseconds since epoch.
    #[serde(
        rename = "maintenanceModeStartTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_mode_start_time: Option<i64>,
    /// Timestamp of the date the maintenanceMode was ended, in milliseconds since epoch.
    #[serde(
        rename = "maintenanceModeEndTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_mode_end_time: Option<i64>,
}

impl ApplicationControlComputerExtension {
    /// Computer-level configuration for the Application Control module. Null for tenants for whom Application Control is hidden.
    pub fn new() -> ApplicationControlComputerExtension {
        ApplicationControlComputerExtension {
            state: None,
            module_status: None,
            block_unrecognized: None,
            ruleset_id: None,
            trust_ruleset_id: None,
            maintenance_mode_status: None,
            maintenance_mode_duration: None,
            maintenance_mode_start_time: None,
            maintenance_mode_end_time: None,
        }
    }
}

/// Module state. The 'inherited' value is write-only.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "inherited")]
    Inherited,
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}
/// Maintenance mode status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MaintenanceModeStatus {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "start-requested")]
    StartRequested,
    #[serde(rename = "on")]
    On,
    #[serde(rename = "stop-requested")]
    StopRequested,
    #[serde(rename = "reset-duration-requested")]
    ResetDurationRequested,
}

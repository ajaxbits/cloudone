/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// AlertRights : Alert right details.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRights {
    /// Right to dismiss global alerts.
    #[serde(
        rename = "canDismissGlobalAlerts",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_dismiss_global_alerts: Option<bool>,
}

impl AlertRights {
    /// Alert right details.
    pub fn new() -> AlertRights {
        AlertRights {
            can_dismiss_global_alerts: None,
        }
    }
}

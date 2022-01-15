/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ServerLogRights : Server log right details.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerLogRights {
    /// Right to view server logs.
    #[serde(rename = "canViewServerLogs", skip_serializing_if = "Option::is_none")]
    pub can_view_server_logs: Option<bool>,
}

impl ServerLogRights {
    /// Server log right details.
    pub fn new() -> ServerLogRights {
        ServerLogRights {
            can_view_server_logs: None,
        }
    }
}


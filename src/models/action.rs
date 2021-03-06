/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// Action : The supported action of a gcp connector.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Action {
    /// Type of the GCPConnectorAction.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// Status of the GCPConnectorAction.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Submitted time of the GCPConnectorAction.
    #[serde(rename = "submittedTime", skip_serializing_if = "Option::is_none")]
    pub submitted_time: Option<i64>,
    /// ID of the GCPConnectorAction.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<i64>,
}

impl Action {
    /// The supported action of a gcp connector.
    pub fn new() -> Action {
        Action {
            _type: None,
            status: None,
            submitted_time: None,
            ID: None,
        }
    }
}

/// Type of the GCPConnectorAction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "synchronize")]
    Synchronize,
}
/// Status of the GCPConnectorAction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "SUBMITTED")]
    SUBMITTED,
    #[serde(rename = "RUNNING")]
    RUNNING,
    #[serde(rename = "SUCCESS")]
    SUCCESS,
    #[serde(rename = "FAILED")]
    FAILED,
}

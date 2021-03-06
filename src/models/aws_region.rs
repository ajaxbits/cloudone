/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// AwsRegion : A region in AWS that instances and workspaces can be synchronized to.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsRegion {
    /// The Amazon region name.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The number of instances in the region during the last sync.
    #[serde(rename = "computersSynced", skip_serializing_if = "Option::is_none")]
    pub computers_synced: Option<i64>,
    /// The number of workspaces in the region during the last sync.
    #[serde(rename = "workspacesSynced", skip_serializing_if = "Option::is_none")]
    pub workspaces_synced: Option<i64>,
}

impl AwsRegion {
    /// A region in AWS that instances and workspaces can be synchronized to.
    pub fn new() -> AwsRegion {
        AwsRegion {
            region: None,
            computers_synced: None,
            workspaces_synced: None,
        }
    }
}

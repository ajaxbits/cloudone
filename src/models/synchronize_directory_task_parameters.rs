/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// SynchronizeDirectoryTaskParameters : Controls the behavior of a scheduled task of type synchronize-cloud-account.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SynchronizeDirectoryTaskParameters {
    /// Identifies the top-level computer group for the directory.
    #[serde(rename = "computerGroupID", skip_serializing_if = "Option::is_none")]
    pub computer_group_id: Option<i32>,
}

impl SynchronizeDirectoryTaskParameters {
    /// Controls the behavior of a scheduled task of type synchronize-cloud-account.
    pub fn new() -> SynchronizeDirectoryTaskParameters {
        SynchronizeDirectoryTaskParameters {
            computer_group_id: None,
        }
    }
}

/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AvailableVersion : Available version details.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableVersion {
    /// version of the available version. Searchable as String.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// `true` if the the version is the latest. Searchable as Boolean.
    #[serde(rename = "isLatest", skip_serializing_if = "Option::is_none")]
    pub is_latest: Option<bool>,
    /// `true` if the the version is the latest LTS. Searchable as Boolean.
    #[serde(rename = "isLatestLTS", skip_serializing_if = "Option::is_none")]
    pub is_latest_lts: Option<bool>,
}

impl AvailableVersion {
    /// Available version details.
    pub fn new() -> AvailableVersion {
        AvailableVersion {
            version: None,
            is_latest: None,
            is_latest_lts: None,
        }
    }
}


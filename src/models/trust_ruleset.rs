/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrustRuleset {
    /// Name of the trust ruleset. Searchable as String.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description of the trust ruleset. Searchable as String.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Version number of the trust ruleset. Incremented by one whenever the ruleset rules are modified. Searchable as Numeric.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// Timestamp of the trust ruleset's creation, in milliseconds since epoch. Searchable as Date.
    #[serde(rename = "createdTime", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    /// Timestamp of when the trust ruleset was last updated, in milliseconds since epoch. Searchable as Date.
    #[serde(rename = "lastUpdatedTime", skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    /// Count for trust ruleset usage.
    #[serde(rename = "usageCount", skip_serializing_if = "Option::is_none")]
    pub usage_count: Option<i32>,
    /// ID of the trust ruleset. Searchable as ID.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<i64>,
}

impl TrustRuleset {
    pub fn new() -> TrustRuleset {
        TrustRuleset {
            name: None,
            description: None,
            version: None,
            created_time: None,
            last_updated_time: None,
            usage_count: None,
            ID: None,
        }
    }
}

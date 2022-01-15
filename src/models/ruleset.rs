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
pub struct Ruleset {
    /// Name of the ruleset. Searchable as String.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description of the ruleset. Searchable as String.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Version number of the ruleset. Incremented by one whenever the ruleset rules are modified. Searchable as Numeric.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// Type of the ruleset. Searchable as Choice.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// Timestamp of the ruleset's creation, in milliseconds since epoch. Searchable as Date.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    /// Timestamp of when the ruleset was last updated, in milliseconds since epoch. Searchable as Date.
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<i64>,
    /// ID of the ruleset. Searchable as ID.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<i64>,
}

impl Ruleset {
    pub fn new() -> Ruleset {
        Ruleset {
            name: None,
            description: None,
            version: None,
            _type: None,
            created: None,
            last_updated: None,
            ID: None,
        }
    }
}

/// Type of the ruleset. Searchable as Choice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "local-file")]
    LocalFile,
    #[serde(rename = "local-hash")]
    LocalHash,
    #[serde(rename = "shared-file")]
    SharedFile,
    #[serde(rename = "shared-hash")]
    SharedHash,
}


/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// TaggingRights : Tagging right details.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaggingRights {
    /// Right to delete auto tag rules.
    #[serde(
        rename = "canDeleteAutoTagRules",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_delete_auto_tag_rules: Option<bool>,
    /// Right to update auto tag rules.
    #[serde(
        rename = "canUpdateAutoTagRules",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_update_auto_tag_rules: Option<bool>,
    /// Right to run auto tag rules.
    #[serde(rename = "canRunAutoTagRules", skip_serializing_if = "Option::is_none")]
    pub can_run_auto_tag_rules: Option<bool>,
    /// Right to delete tags.
    #[serde(rename = "canDeleteTags", skip_serializing_if = "Option::is_none")]
    pub can_delete_tags: Option<bool>,
    /// Right to tag.
    #[serde(rename = "canTag", skip_serializing_if = "Option::is_none")]
    pub can_tag: Option<bool>,
}

impl TaggingRights {
    /// Tagging right details.
    pub fn new() -> TaggingRights {
        TaggingRights {
            can_delete_auto_tag_rules: None,
            can_update_auto_tag_rules: None,
            can_run_auto_tag_rules: None,
            can_delete_tags: None,
            can_tag: None,
        }
    }
}

/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Role : Administrator role details.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Role {
    /// Name of the administrator role. Searchable as String.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description of the administrator role. Searchable as String.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Uniform resource name.
    #[serde(rename = "urn", skip_serializing_if = "Option::is_none")]
    pub urn: Option<String>,
    /// The default immutable role in Deep Security Manager.
    #[serde(rename = "immutable", skip_serializing_if = "Option::is_none")]
    pub immutable: Option<bool>,
    /// Controls whether or not the role can only manipulate users with equal or lesser rights. Searchable as Boolean.
    #[serde(rename = "canOnlyManipulateUsersWithEqualOrLesserRights", skip_serializing_if = "Option::is_none")]
    pub can_only_manipulate_users_with_equal_or_lesser_rights: Option<bool>,
    /// Controls whether or not the role is allowed to access all computers. Searchable as Boolean.
    #[serde(rename = "allComputers", skip_serializing_if = "Option::is_none")]
    pub all_computers: Option<bool>,
    /// Controls whether or not the role is allowed to access all policies. Searchable as Boolean.
    #[serde(rename = "allPolicies", skip_serializing_if = "Option::is_none")]
    pub all_policies: Option<bool>,
    /// Controls whether or not the role is allowed to use the user interface. Searchable as Boolean.
    #[serde(rename = "allowUserInterface", skip_serializing_if = "Option::is_none")]
    pub allow_user_interface: Option<bool>,
    /// Controls whether or not the role is allowed to use the web service API. Searchable as Boolean.
    #[serde(rename = "allowWebService", skip_serializing_if = "Option::is_none")]
    pub allow_web_service: Option<bool>,
    #[serde(rename = "rights", skip_serializing_if = "Option::is_none")]
    pub rights: Option<Box<crate::models::Rights>>,
    /// List of computer IDs that the role can access. Ignored if 'allComputers' is true.
    #[serde(rename = "computerIDs", skip_serializing_if = "Option::is_none")]
    pub computer_ids: Option<Vec<i32>>,
    /// List of computer group IDs that the role can access. A group ID of '0' allows access to computers not in a computer group. Note that groups must be identified individually and that access to sub-groups is not automatically granted. Ignored if 'allComputers' is true.
    #[serde(rename = "computerGroupIDs", skip_serializing_if = "Option::is_none")]
    pub computer_group_ids: Option<Vec<i32>>,
    /// List of policy IDs that the role can access. Ignored if 'allPolicies' is true.
    #[serde(rename = "policyIDs", skip_serializing_if = "Option::is_none")]
    pub policy_ids: Option<Vec<i32>>,
    /// ID of the administrator role. Searchable as ID.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<i32>,
}

impl Role {
    /// Administrator role details.
    pub fn new() -> Role {
        Role {
            name: None,
            description: None,
            urn: None,
            immutable: None,
            can_only_manipulate_users_with_equal_or_lesser_rights: None,
            all_computers: None,
            all_policies: None,
            allow_user_interface: None,
            allow_web_service: None,
            rights: None,
            computer_ids: None,
            computer_group_ids: None,
            policy_ids: None,
            ID: None,
        }
    }
}



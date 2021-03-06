/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// MacListRights : MAC list right details.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MacListRights {
    /// Right to create new MAC List objects.
    #[serde(
        rename = "canCreateNewMACLists",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_create_new_mac_lists: Option<bool>,
    /// Right to delete MAC List objects.
    #[serde(rename = "canDeleteMACLists", skip_serializing_if = "Option::is_none")]
    pub can_delete_mac_lists: Option<bool>,
    /// Right to edit MAC List object properties.
    #[serde(
        rename = "canEditMACListProperties",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_edit_mac_list_properties: Option<bool>,
}

impl MacListRights {
    /// MAC list right details.
    pub fn new() -> MacListRights {
        MacListRights {
            can_create_new_mac_lists: None,
            can_delete_mac_lists: None,
            can_edit_mac_list_properties: None,
        }
    }
}

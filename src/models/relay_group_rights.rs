/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// RelayGroupRights : Relay group right details.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelayGroupRights {
    /// Right to create new relay groups.
    #[serde(
        rename = "canCreateNewRelayGroups",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_create_new_relay_groups: Option<bool>,
    /// Right to delete relay groups.
    #[serde(
        rename = "canDeleteRelayGroups",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_delete_relay_groups: Option<bool>,
    /// Right to edit relay group properties.
    #[serde(
        rename = "canEditRelayGroupProperties",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_edit_relay_group_properties: Option<bool>,
}

impl RelayGroupRights {
    /// Relay group right details.
    pub fn new() -> RelayGroupRights {
        RelayGroupRights {
            can_create_new_relay_groups: None,
            can_delete_relay_groups: None,
            can_edit_relay_group_properties: None,
        }
    }
}

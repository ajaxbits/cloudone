/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// DataCenterGatewayRights : Data Center Gateway right details.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataCenterGatewayRights {
    /// Right to create new data center gateways.
    #[serde(
        rename = "canCreateNewDataCenterGateways",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_create_new_data_center_gateways: Option<bool>,
    /// Right to view data center gateways.
    #[serde(
        rename = "canViewDataCenterGateways",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_view_data_center_gateways: Option<bool>,
    /// Right to edit data center gateways.
    #[serde(
        rename = "canEditDataCenterGateways",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_edit_data_center_gateways: Option<bool>,
    /// Right to delete data center gateways.
    #[serde(
        rename = "canDeleteDataCenterGateways",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_delete_data_center_gateways: Option<bool>,
}

impl DataCenterGatewayRights {
    /// Data Center Gateway right details.
    pub fn new() -> DataCenterGatewayRights {
        DataCenterGatewayRights {
            can_create_new_data_center_gateways: None,
            can_view_data_center_gateways: None,
            can_edit_data_center_gateways: None,
            can_delete_data_center_gateways: None,
        }
    }
}

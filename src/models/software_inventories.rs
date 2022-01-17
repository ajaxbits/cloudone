/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// SoftwareInventories : Software inventories.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoftwareInventories {
    #[serde(
        rename = "softwareInventories",
        skip_serializing_if = "Option::is_none"
    )]
    pub software_inventories: Option<Vec<crate::models::SoftwareInventory>>,
}

impl SoftwareInventories {
    /// Software inventories.
    pub fn new() -> SoftwareInventories {
        SoftwareInventories {
            software_inventories: None,
        }
    }
}

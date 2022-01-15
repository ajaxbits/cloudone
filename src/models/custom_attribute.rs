/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CustomAttribute : Custom Attribute of an ESX.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomAttribute {
    /// Name of the custom attribute.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Value of the custom attribute.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CustomAttribute {
    /// Custom Attribute of an ESX.
    pub fn new() -> CustomAttribute {
        CustomAttribute {
            name: None,
            value: None,
        }
    }
}



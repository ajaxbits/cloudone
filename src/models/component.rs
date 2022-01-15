/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Component : Security update component.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Component {
    /// Name of the component.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Name and version of the component consumer.
    #[serde(rename = "forUseBy", skip_serializing_if = "Option::is_none")]
    pub for_use_by: Option<String>,
    /// Platform of the component.
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// Version of the component.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// True if the component is the latest version.
    #[serde(rename = "latest", skip_serializing_if = "Option::is_none")]
    pub latest: Option<bool>,
}

impl Component {
    /// Security update component.
    pub fn new() -> Component {
        Component {
            name: None,
            for_use_by: None,
            platform: None,
            version: None,
            latest: None,
        }
    }
}



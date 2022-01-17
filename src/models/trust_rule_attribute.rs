/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// TrustRuleAttribute : Attribute that defines the trust rule.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrustRuleAttribute {
    /// The name of the attribute.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The value of the attribute.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl TrustRuleAttribute {
    /// Attribute that defines the trust rule.
    pub fn new() -> TrustRuleAttribute {
        TrustRuleAttribute {
            name: None,
            value: None,
        }
    }
}

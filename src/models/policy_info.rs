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
pub struct PolicyInfo {
    /// Policy id
    #[serde(rename = "policyID", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<i32>,
    /// Policy name
    #[serde(rename = "policyName", skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

impl PolicyInfo {
    pub fn new() -> PolicyInfo {
        PolicyInfo {
            policy_id: None,
            policy_name: None,
        }
    }
}



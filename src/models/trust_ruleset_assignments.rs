/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TrustRulesetAssignments : Trust ruleset assignments.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrustRulesetAssignments {
    #[serde(rename = "computers", skip_serializing_if = "Option::is_none")]
    pub computers: Option<Vec<crate::models::TrustruleComputerAssignment>>,
    #[serde(rename = "policies", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<crate::models::TrustrulePolicyAssignment>>,
}

impl TrustRulesetAssignments {
    /// Trust ruleset assignments.
    pub fn new() -> TrustRulesetAssignments {
        TrustRulesetAssignments {
            computers: None,
            policies: None,
        }
    }
}


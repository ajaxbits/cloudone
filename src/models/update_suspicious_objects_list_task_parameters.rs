/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateSuspiciousObjectsListTaskParameters : Controls the behavior of a scheduled task of type update-suspicious-objects-list.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSuspiciousObjectsListTaskParameters {
    #[serde(rename = "computerFilter", skip_serializing_if = "Option::is_none")]
    pub computer_filter: Option<Box<crate::models::ComputerFilter>>,
}

impl UpdateSuspiciousObjectsListTaskParameters {
    /// Controls the behavior of a scheduled task of type update-suspicious-objects-list.
    pub fn new() -> UpdateSuspiciousObjectsListTaskParameters {
        UpdateSuspiciousObjectsListTaskParameters {
            computer_filter: None,
        }
    }
}



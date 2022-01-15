/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ComputersToBeDeletedIdList : The ID list of the computers to be deleted



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputersToBeDeletedIdList {
    /// The list of IDs of computers to be deleted. The maximum number of IDs can be passed at one request is 1000
    #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i32>>,
}

impl ComputersToBeDeletedIdList {
    /// The ID list of the computers to be deleted
    pub fn new() -> ComputersToBeDeletedIdList {
        ComputersToBeDeletedIdList {
            ids: None,
        }
    }
}


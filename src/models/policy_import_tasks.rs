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
pub struct PolicyImportTasks {
    #[serde(rename = "policyImportTask", skip_serializing_if = "Option::is_none")]
    pub policy_import_task: Option<Vec<crate::models::PolicyImportTask>>,
}

impl PolicyImportTasks {
    pub fn new() -> PolicyImportTasks {
        PolicyImportTasks {
            policy_import_task: None,
        }
    }
}

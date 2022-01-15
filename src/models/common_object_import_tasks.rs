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
pub struct CommonObjectImportTasks {
    #[serde(rename = "commonObjectImportTasks", skip_serializing_if = "Option::is_none")]
    pub common_object_import_tasks: Option<Vec<crate::models::CommonObjectImportTask>>,
}

impl CommonObjectImportTasks {
    pub fn new() -> CommonObjectImportTasks {
        CommonObjectImportTasks {
            common_object_import_tasks: None,
        }
    }
}



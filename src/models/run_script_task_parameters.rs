/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RunScriptTaskParameters : Controls the behavior of a scheduled task of type run-script.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunScriptTaskParameters {
    /// Name of the script.
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

impl RunScriptTaskParameters {
    /// Controls the behavior of a scheduled task of type run-script.
    pub fn new() -> RunScriptTaskParameters {
        RunScriptTaskParameters {
            filename: None,
        }
    }
}


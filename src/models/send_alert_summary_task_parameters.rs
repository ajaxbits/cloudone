/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SendAlertSummaryTaskParameters : Controls the behavior of a scheduled task of type send-alert-summary.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendAlertSummaryTaskParameters {
    #[serde(rename = "recipients", skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Box<crate::models::Recipients>>,
}

impl SendAlertSummaryTaskParameters {
    /// Controls the behavior of a scheduled task of type send-alert-summary.
    pub fn new() -> SendAlertSummaryTaskParameters {
        SendAlertSummaryTaskParameters {
            recipients: None,
        }
    }
}



/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// LogInspectionRights : Log inspection rights details.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogInspectionRights {
    #[serde(
        rename = "logInspectionDecoderRights",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_inspection_decoder_rights: Option<Box<crate::models::LogInspectionDecoderRights>>,
    #[serde(
        rename = "logInspectionRuleRights",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_inspection_rule_rights: Option<Box<crate::models::LogInspectionRuleRights>>,
}

impl LogInspectionRights {
    /// Log inspection rights details.
    pub fn new() -> LogInspectionRights {
        LogInspectionRights {
            log_inspection_decoder_rights: None,
            log_inspection_rule_rights: None,
        }
    }
}

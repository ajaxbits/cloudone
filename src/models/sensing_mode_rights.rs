/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SensingModeRights : Activity monitoring rights details.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensingModeRights {
    #[serde(rename = "sensingModeConfigurationRights", skip_serializing_if = "Option::is_none")]
    pub sensing_mode_configuration_rights: Option<Box<crate::models::SensingModeConfigurationRights>>,
}

impl SensingModeRights {
    /// Activity monitoring rights details.
    pub fn new() -> SensingModeRights {
        SensingModeRights {
            sensing_mode_configuration_rights: None,
        }
    }
}



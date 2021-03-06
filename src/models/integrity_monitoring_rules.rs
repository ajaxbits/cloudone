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
pub struct IntegrityMonitoringRules {
    #[serde(
        rename = "integrityMonitoringRules",
        skip_serializing_if = "Option::is_none"
    )]
    pub integrity_monitoring_rules: Option<Vec<crate::models::IntegrityMonitoringRule>>,
}

impl IntegrityMonitoringRules {
    pub fn new() -> IntegrityMonitoringRules {
        IntegrityMonitoringRules {
            integrity_monitoring_rules: None,
        }
    }
}

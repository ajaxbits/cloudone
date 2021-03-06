/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// IntrusionPreventionRights : Intrusion prevention rights details.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntrusionPreventionRights {
    #[serde(
        rename = "applicationTypeRights",
        skip_serializing_if = "Option::is_none"
    )]
    pub application_type_rights: Option<Box<crate::models::ApplicationTypeRights>>,
    #[serde(
        rename = "intrusionPreventionRuleRights",
        skip_serializing_if = "Option::is_none"
    )]
    pub intrusion_prevention_rule_rights: Option<Box<crate::models::IntrusionPreventionRuleRights>>,
}

impl IntrusionPreventionRights {
    /// Intrusion prevention rights details.
    pub fn new() -> IntrusionPreventionRights {
        IntrusionPreventionRights {
            application_type_rights: None,
            intrusion_prevention_rule_rights: None,
        }
    }
}

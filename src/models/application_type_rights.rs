/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ApplicationTypeRights : Application type right details.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationTypeRights {
    /// Right to create new application types.
    #[serde(rename = "canCreateNewApplicationTypes", skip_serializing_if = "Option::is_none")]
    pub can_create_new_application_types: Option<bool>,
    /// Right to delete application types.
    #[serde(rename = "canDeleteApplicationTypes", skip_serializing_if = "Option::is_none")]
    pub can_delete_application_types: Option<bool>,
    /// Right to edit application type properties.
    #[serde(rename = "canEditApplicationTypeProperties", skip_serializing_if = "Option::is_none")]
    pub can_edit_application_type_properties: Option<bool>,
}

impl ApplicationTypeRights {
    /// Application type right details.
    pub fn new() -> ApplicationTypeRights {
        ApplicationTypeRights {
            can_create_new_application_types: None,
            can_delete_application_types: None,
            can_edit_application_type_properties: None,
        }
    }
}


/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// NetworkSecurityRights : Network Security right details.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkSecurityRights {
    /// Right to create Network Security Device authorization tokens.
    #[serde(
        rename = "canCreateDeviceAuthorizationTokens",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_create_device_authorization_tokens: Option<bool>,
}

impl NetworkSecurityRights {
    /// Network Security right details.
    pub fn new() -> NetworkSecurityRights {
        NetworkSecurityRights {
            can_create_device_authorization_tokens: None,
        }
    }
}

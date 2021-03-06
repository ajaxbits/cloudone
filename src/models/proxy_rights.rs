/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// ProxyRights : Proxy right details.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyRights {
    /// Right to create new proxies.
    #[serde(
        rename = "canCreateNewProxies",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_create_new_proxies: Option<bool>,
    /// Right to delete proxies.
    #[serde(rename = "canDeleteProxies", skip_serializing_if = "Option::is_none")]
    pub can_delete_proxies: Option<bool>,
    /// Right to edit proxy properties.
    #[serde(
        rename = "canEditProxyProperties",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_edit_proxy_properties: Option<bool>,
}

impl ProxyRights {
    /// Proxy right details.
    pub fn new() -> ProxyRights {
        ProxyRights {
            can_create_new_proxies: None,
            can_delete_proxies: None,
            can_edit_proxy_properties: None,
        }
    }
}

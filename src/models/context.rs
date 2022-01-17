/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// Context : Firewall context information.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Context {
    /// Name of the context. Searchable as String.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description of the context. Searchable as String.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Minimum supported agent version.
    #[serde(
        rename = "minimumAgentVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_agent_version: Option<String>,
    /// Specifies whether the context applies to connections that are locally conntected to a domain. Set to true to apply to locally connected.
    #[serde(
        rename = "localConnectionsEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub local_connections_enabled: Option<bool>,
    /// Specifies whether the context applies to connections that are remotely conntected to a domain. Set to true to apply to remotely connected.
    #[serde(
        rename = "remoteConnectionsEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_connections_enabled: Option<bool>,
    /// Specifies whether the context applies to connections that are no connection enabled. Set to true to apply to no connection enabled.
    #[serde(
        rename = "noConnectionEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub no_connection_enabled: Option<bool>,
    /// Specifies whether the context applies to connections that are neither connection nor Internet enabled. Set to true to apply.
    #[serde(rename = "noInternetEnabled", skip_serializing_if = "Option::is_none")]
    pub no_internet_enabled: Option<bool>,
    /// Controls if the firewall contents are restricted from view and duplication. Set to true if it's restricted. Searchable as Boolean.
    #[serde(
        rename = "restrictedInterfacesEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub restricted_interfaces_enabled: Option<bool>,
    /// ID of the context. Searchable as ID.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<i32>,
}

impl Context {
    /// Firewall context information.
    pub fn new() -> Context {
        Context {
            name: None,
            description: None,
            minimum_agent_version: None,
            local_connections_enabled: None,
            remote_connections_enabled: None,
            no_connection_enabled: None,
            no_internet_enabled: None,
            restricted_interfaces_enabled: None,
            ID: None,
        }
    }
}

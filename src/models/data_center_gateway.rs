/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// DataCenterGateway : DataCenterGateway details. A DataCenterGateway object represents a connection between the gateway client and Cloud One Workload Security.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataCenterGateway {
    /// Data center gateway's display name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Public key of the data center gateway.
    #[serde(rename = "publicKey", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// Resource name of data center gateway.
    #[serde(rename = "gatewayResource", skip_serializing_if = "Option::is_none")]
    pub gateway_resource: Option<String>,
    /// Control endpoint for the data center gateway service.
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// Cloud One account ID.
    #[serde(rename = "cloudOneAccountID", skip_serializing_if = "Option::is_none")]
    pub cloud_one_account_id: Option<String>,
    /// Timestamp of the creation of the data center gateway, in milliseconds since epoch.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    /// Timestamp of the last updated to the data center gateway, in milliseconds since epoch.
    #[serde(rename = "lastModified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<i64>,
    /// Certificate of the data center gateway. Only returned when creating a new Data Center Gateway.
    #[serde(rename = "certificate", skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// Private key of the data center gateway. Only returned when creating a new Data Center Gateway.
    #[serde(rename = "privateKey", skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// Root CA for data center gateway service. Only returned when creating a new Data Center Gateway.
    #[serde(rename = "rootCA", skip_serializing_if = "Option::is_none")]
    pub root_ca: Option<String>,
    /// ID of data center gateway.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<i32>,
}

impl DataCenterGateway {
    /// DataCenterGateway details. A DataCenterGateway object represents a connection between the gateway client and Cloud One Workload Security.
    pub fn new() -> DataCenterGateway {
        DataCenterGateway {
            name: None,
            public_key: None,
            gateway_resource: None,
            endpoint: None,
            cloud_one_account_id: None,
            created: None,
            last_modified: None,
            certificate: None,
            private_key: None,
            root_ca: None,
            ID: None,
        }
    }
}

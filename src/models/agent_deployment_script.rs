/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AgentDeploymentScript : Agent deployment script.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentDeploymentScript {
    /// Platform type for agent deployment.
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
    /// Validate if Deep Security Manager is using a valid TLS certificate from a trusted certificate authority (CA) when downloading the agent software.
    #[serde(rename = "validateCertificateRequired", skip_serializing_if = "Option::is_none")]
    pub validate_certificate_required: Option<bool>,
    /// Validate digital signature of Deep Security Agent installer.
    #[serde(rename = "validateDigitalSignatureRequired", skip_serializing_if = "Option::is_none")]
    pub validate_digital_signature_required: Option<bool>,
    /// Activate the agent at startup.
    #[serde(rename = "activationRequired", skip_serializing_if = "Option::is_none")]
    pub activation_required: Option<bool>,
    /// ID of the proxy server for contacting Deep Security Manager.
    #[serde(rename = "dsmProxyID", skip_serializing_if = "Option::is_none")]
    pub dsm_proxy_id: Option<i32>,
    /// ID of the proxy server for contacting Relay(s).
    #[serde(rename = "relayProxyID", skip_serializing_if = "Option::is_none")]
    pub relay_proxy_id: Option<i32>,
    /// ID of the policy assigned to the computer.
    #[serde(rename = "policyID", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<i32>,
    /// ID of the relay group to which the computer belongs.
    #[serde(rename = "relayGroupID", skip_serializing_if = "Option::is_none")]
    pub relay_group_id: Option<i32>,
    /// ID of the computer group to which the computer belongs.
    #[serde(rename = "computerGroupID", skip_serializing_if = "Option::is_none")]
    pub computer_group_id: Option<i32>,
    /// Agent deployment script.
    #[serde(rename = "scriptBody", skip_serializing_if = "Option::is_none")]
    pub script_body: Option<String>,
}

impl AgentDeploymentScript {
    /// Agent deployment script.
    pub fn new() -> AgentDeploymentScript {
        AgentDeploymentScript {
            platform: None,
            validate_certificate_required: None,
            validate_digital_signature_required: None,
            activation_required: None,
            dsm_proxy_id: None,
            relay_proxy_id: None,
            policy_id: None,
            relay_group_id: None,
            computer_group_id: None,
            script_body: None,
        }
    }
}

/// Platform type for agent deployment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Platform {
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "solaris")]
    Solaris,
    #[serde(rename = "aix")]
    Aix,
    #[serde(rename = "macos")]
    Macos,
}


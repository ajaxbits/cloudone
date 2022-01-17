/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// GcpVirtualMachineSummary : Details of a GCP virtual machine.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcpVirtualMachineSummary {
    /// Cloud provider: \"GCP\".
    #[serde(rename = "cloudProvider", skip_serializing_if = "Option::is_none")]
    pub cloud_provider: Option<String>,
    /// Power state, for example, \"POWER ON\".
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Operating system, for example: \"Microsoft Windows (64 bit)\". Searchable as String.
    #[serde(rename = "operatingSystem", skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// Instance ID, for example: \"1234567890123456789\". Searchable as String.
    #[serde(rename = "instanceID", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// Private IP address. Searchable as String.
    #[serde(rename = "privateIPAddress", skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    /// Public IP address. Searchable as String.
    #[serde(rename = "publicIPAddress", skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
    /// Zone is a deployment area, for example: \"us-east1-d\". Searchable as String.
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    /// Number of vCPU. Searchable as Numeric.
    #[serde(rename = "vCPUs", skip_serializing_if = "Option::is_none")]
    pub v_cpus: Option<i32>,
    /// Memory size in megabyte. Searchable as Numeric.
    #[serde(rename = "memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<i32>,
    /// List of key/value labels.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::models::VirtualMachineGcpLabel>>,
    /// List of network tags.
    #[serde(rename = "networkTags", skip_serializing_if = "Option::is_none")]
    pub network_tags: Option<Vec<String>>,
}

impl GcpVirtualMachineSummary {
    /// Details of a GCP virtual machine.
    pub fn new() -> GcpVirtualMachineSummary {
        GcpVirtualMachineSummary {
            cloud_provider: None,
            state: None,
            operating_system: None,
            instance_id: None,
            private_ip_address: None,
            public_ip_address: None,
            zone: None,
            v_cpus: None,
            memory: None,
            labels: None,
            network_tags: None,
        }
    }
}

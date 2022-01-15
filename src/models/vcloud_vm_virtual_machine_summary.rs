/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// VcloudVmVirtualMachineSummary : Details of a vCloud virtual machine.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VcloudVmVirtualMachineSummary {
    /// Cloud provider: \"vCloud\".
    #[serde(rename = "cloudProvider", skip_serializing_if = "Option::is_none")]
    pub cloud_provider: Option<String>,
    /// Operating system, for example: \"Microsoft Windows Server 2012 (64-bit)\". Searchable as String.
    #[serde(rename = "operatingSystem", skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// Instance ID. Searchable as String.
    #[serde(rename = "instanceID", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// Hardware type. Searchable as String.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// Power state, for example: \"Powered On\".
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "IPAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "DNSName", skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
}

impl VcloudVmVirtualMachineSummary {
    /// Details of a vCloud virtual machine.
    pub fn new() -> VcloudVmVirtualMachineSummary {
        VcloudVmVirtualMachineSummary {
            cloud_provider: None,
            operating_system: None,
            instance_id: None,
            _type: None,
            state: None,
            ip_address: None,
            dns_name: None,
        }
    }
}


/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NoConnectorVirtualMachineSummary : Details of an AWS virtual machine that was added to Deep Security Manager using the legacy Add Cloud Account wizard.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NoConnectorVirtualMachineSummary {
    /// Agent-reported account ID. Searchable as String.
    #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Agent-reported directory ID. Searchable as String.
    #[serde(rename = "directoryID", skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// Agent-reported user ID. Searchable as String.
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// Agent-reported instance ID. Searchable as String.
    #[serde(rename = "instanceID", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// Agent-reported region. Searchable as String.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl NoConnectorVirtualMachineSummary {
    /// Details of an AWS virtual machine that was added to Deep Security Manager using the legacy Add Cloud Account wizard.
    pub fn new() -> NoConnectorVirtualMachineSummary {
        NoConnectorVirtualMachineSummary {
            account_id: None,
            directory_id: None,
            user_name: None,
            instance_id: None,
            region: None,
        }
    }
}


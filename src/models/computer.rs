/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Computer : Computer details. Settings set on the computer override settings on an assigned policy. When the computer is virtual, additional information about the virtual machine is provided.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Computer {
    /// Hostname of the computer. Searchable as String.
    #[serde(rename = "hostName", skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    /// Display name of the computer. Searchable as String.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Description of the computer. Searchable as String.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Most recent IP address used by the computer. This value is null when the computer is not activated. Searchable as String.
    #[serde(rename = "lastIPUsed", skip_serializing_if = "Option::is_none")]
    pub last_ip_used: Option<String>,
    /// Platform of the computer. Automatically detected and set. Searchable as String
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// ID of the computer group to which the computer belongs. Set to 0 to remove any assignment. Searchable as Numeric.
    #[serde(rename = "groupID", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i32>,
    /// ID of the policy assigned to the computer. Set to 0 to remove any assignment. Searchable as Numeric.
    #[serde(rename = "policyID", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<i32>,
    /// ID of the asset importance assigned to the computer. Set to 0 to remove any assignment. Searchable as Numeric.
    #[serde(rename = "assetImportanceID", skip_serializing_if = "Option::is_none")]
    pub asset_importance_id: Option<i32>,
    /// ID of the relay list that is assigned to the computer. Set to 0 to remove any assignment. Searchable as Numeric.
    #[serde(rename = "relayListID", skip_serializing_if = "Option::is_none")]
    pub relay_list_id: Option<i32>,
    /// FingerPrint of the Agent on the computer. This value is null when no agent is installed or activated. Searchable as String.
    #[serde(rename = "agentFingerPrint", skip_serializing_if = "Option::is_none")]
    pub agent_finger_print: Option<String>,
    /// FingerPrint of the Appliance protecting the computer. This value is null when no Deep Security Virtual Appliance is protecting the computer.
    #[serde(rename = "applianceFingerPrint", skip_serializing_if = "Option::is_none")]
    pub appliance_finger_print: Option<String>,
    /// Timestamp of the last agent communication, in milliseconds since epoch. Searchable as Date.
    #[serde(rename = "lastAgentCommunication", skip_serializing_if = "Option::is_none")]
    pub last_agent_communication: Option<i64>,
    /// Timestamp of the last appliance communication, in milliseconds since epoch.
    #[serde(rename = "lastApplianceCommunication", skip_serializing_if = "Option::is_none")]
    pub last_appliance_communication: Option<i64>,
    /// Timestamp of the last policy request sent, in milliseconds since epoch. Searchable as Date.
    #[serde(rename = "lastSendPolicyRequest", skip_serializing_if = "Option::is_none")]
    pub last_send_policy_request: Option<i64>,
    /// Timestamp of last successful policy sent, in milliseconds since epoch.  Searchable as Date.
    #[serde(rename = "lastSendPolicySuccess", skip_serializing_if = "Option::is_none")]
    pub last_send_policy_success: Option<i64>,
    /// Version of Deep Security Agent that is installed on the computer. When no agent software is installed or activated, the version is '0.0.0.0'. Searchable as String, however to search for computers without an agent, search for computers that have no agent fingerprint.'.
    #[serde(rename = "agentVersion", skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "computerStatus", skip_serializing_if = "Option::is_none")]
    pub computer_status: Option<Box<crate::models::ComputerStatus>>,
    #[serde(rename = "tasks", skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Box<crate::models::ComputerTasks>>,
    #[serde(rename = "securityUpdates", skip_serializing_if = "Option::is_none")]
    pub security_updates: Option<Box<crate::models::SecurityUpdates>>,
    #[serde(rename = "computerSettings", skip_serializing_if = "Option::is_none")]
    pub computer_settings: Option<Box<crate::models::ComputerSettings>>,
    #[serde(rename = "interfaces", skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<Box<crate::models::Interfaces>>,
    /// BIOS UUID, for example: \"421e1137-5c49-56e4-246d-9bf7637e69f5\". Only applies to VMware VMs.
    #[serde(rename = "biosUUID", skip_serializing_if = "Option::is_none")]
    pub bios_uuid: Option<String>,
    #[serde(rename = "azureARMVirtualMachineSummary", skip_serializing_if = "Option::is_none")]
    pub azure_arm_virtual_machine_summary: Option<Box<crate::models::AzureArmVirtualMachineSummary>>,
    #[serde(rename = "azureVMVirtualMachineSummary", skip_serializing_if = "Option::is_none")]
    pub azure_vm_virtual_machine_summary: Option<Box<crate::models::AzureVmVirtualMachineSummary>>,
    #[serde(rename = "ec2VirtualMachineSummary", skip_serializing_if = "Option::is_none")]
    pub ec2_virtual_machine_summary: Option<Box<crate::models::Ec2VirtualMachineSummary>>,
    #[serde(rename = "noConnectorVirtualMachineSummary", skip_serializing_if = "Option::is_none")]
    pub no_connector_virtual_machine_summary: Option<Box<crate::models::NoConnectorVirtualMachineSummary>>,
    #[serde(rename = "vmwareVMVirtualMachineSummary", skip_serializing_if = "Option::is_none")]
    pub vmware_vm_virtual_machine_summary: Option<Box<crate::models::VmwareVmVirtualMachineSummary>>,
    #[serde(rename = "vcloudVMVirtualMachineSummary", skip_serializing_if = "Option::is_none")]
    pub vcloud_vm_virtual_machine_summary: Option<Box<crate::models::VcloudVmVirtualMachineSummary>>,
    #[serde(rename = "workspaceVirtualMachineSummary", skip_serializing_if = "Option::is_none")]
    pub workspace_virtual_machine_summary: Option<Box<crate::models::WorkspaceVirtualMachineSummary>>,
    #[serde(rename = "gcpVirtualMachineSummary", skip_serializing_if = "Option::is_none")]
    pub gcp_virtual_machine_summary: Option<Box<crate::models::GcpVirtualMachineSummary>>,
    /// HostGUID of the computer. Searchable as String.
    #[serde(rename = "hostGUID", skip_serializing_if = "Option::is_none")]
    pub host_guid: Option<String>,
    /// ID of the computer. Searchable as ID.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<i32>,
    #[serde(rename = "antiMalware", skip_serializing_if = "Option::is_none")]
    pub anti_malware: Option<Box<crate::models::AntiMalwareComputerExtension>>,
    #[serde(rename = "webReputation", skip_serializing_if = "Option::is_none")]
    pub web_reputation: Option<Box<crate::models::WebReputationComputerExtension>>,
    #[serde(rename = "activityMonitoring", skip_serializing_if = "Option::is_none")]
    pub activity_monitoring: Option<Box<crate::models::ActivityMonitoringComputerExtension>>,
    #[serde(rename = "firewall", skip_serializing_if = "Option::is_none")]
    pub firewall: Option<Box<crate::models::FirewallComputerExtension>>,
    #[serde(rename = "intrusionPrevention", skip_serializing_if = "Option::is_none")]
    pub intrusion_prevention: Option<Box<crate::models::IntrusionPreventionComputerExtension>>,
    #[serde(rename = "integrityMonitoring", skip_serializing_if = "Option::is_none")]
    pub integrity_monitoring: Option<Box<crate::models::IntegrityMonitoringComputerExtension>>,
    #[serde(rename = "logInspection", skip_serializing_if = "Option::is_none")]
    pub log_inspection: Option<Box<crate::models::LogInspectionComputerExtension>>,
    #[serde(rename = "applicationControl", skip_serializing_if = "Option::is_none")]
    pub application_control: Option<Box<crate::models::ApplicationControlComputerExtension>>,
    #[serde(rename = "ESXSummary", skip_serializing_if = "Option::is_none")]
    pub esx_summary: Option<Box<crate::models::EsxSummary>>,
    #[serde(rename = "SAP", skip_serializing_if = "Option::is_none")]
    pub SAP: Option<Box<crate::models::SapComputerExtension>>,
}

impl Computer {
    /// Computer details. Settings set on the computer override settings on an assigned policy. When the computer is virtual, additional information about the virtual machine is provided.
    pub fn new() -> Computer {
        Computer {
            host_name: None,
            display_name: None,
            description: None,
            last_ip_used: None,
            platform: None,
            group_id: None,
            policy_id: None,
            asset_importance_id: None,
            relay_list_id: None,
            agent_finger_print: None,
            appliance_finger_print: None,
            last_agent_communication: None,
            last_appliance_communication: None,
            last_send_policy_request: None,
            last_send_policy_success: None,
            agent_version: None,
            computer_status: None,
            tasks: None,
            security_updates: None,
            computer_settings: None,
            interfaces: None,
            bios_uuid: None,
            azure_arm_virtual_machine_summary: None,
            azure_vm_virtual_machine_summary: None,
            ec2_virtual_machine_summary: None,
            no_connector_virtual_machine_summary: None,
            vmware_vm_virtual_machine_summary: None,
            vcloud_vm_virtual_machine_summary: None,
            workspace_virtual_machine_summary: None,
            gcp_virtual_machine_summary: None,
            host_guid: None,
            ID: None,
            anti_malware: None,
            web_reputation: None,
            activity_monitoring: None,
            firewall: None,
            intrusion_prevention: None,
            integrity_monitoring: None,
            log_inspection: None,
            application_control: None,
            esx_summary: None,
            SAP: None,
        }
    }
}



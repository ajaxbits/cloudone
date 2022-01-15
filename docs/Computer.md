# Computer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**host_name** | Option<**String**> | Hostname of the computer. Searchable as String. | [optional]
**display_name** | Option<**String**> | Display name of the computer. Searchable as String. | [optional]
**description** | Option<**String**> | Description of the computer. Searchable as String. | [optional]
**last_ip_used** | Option<**String**> | Most recent IP address used by the computer. This value is null when the computer is not activated. Searchable as String. | [optional][readonly]
**platform** | Option<**String**> | Platform of the computer. Automatically detected and set. Searchable as String | [optional][readonly]
**group_id** | Option<**i32**> | ID of the computer group to which the computer belongs. Set to 0 to remove any assignment. Searchable as Numeric. | [optional]
**policy_id** | Option<**i32**> | ID of the policy assigned to the computer. Set to 0 to remove any assignment. Searchable as Numeric. | [optional]
**asset_importance_id** | Option<**i32**> | ID of the asset importance assigned to the computer. Set to 0 to remove any assignment. Searchable as Numeric. | [optional]
**relay_list_id** | Option<**i32**> | ID of the relay list that is assigned to the computer. Set to 0 to remove any assignment. Searchable as Numeric. | [optional]
**agent_finger_print** | Option<**String**> | FingerPrint of the Agent on the computer. This value is null when no agent is installed or activated. Searchable as String. | [optional][readonly]
**appliance_finger_print** | Option<**String**> | FingerPrint of the Appliance protecting the computer. This value is null when no Deep Security Virtual Appliance is protecting the computer. | [optional][readonly]
**last_agent_communication** | Option<**i64**> | Timestamp of the last agent communication, in milliseconds since epoch. Searchable as Date. | [optional][readonly]
**last_appliance_communication** | Option<**i64**> | Timestamp of the last appliance communication, in milliseconds since epoch. | [optional][readonly]
**last_send_policy_request** | Option<**i64**> | Timestamp of the last policy request sent, in milliseconds since epoch. Searchable as Date. | [optional][readonly]
**last_send_policy_success** | Option<**i64**> | Timestamp of last successful policy sent, in milliseconds since epoch.  Searchable as Date. | [optional][readonly]
**agent_version** | Option<**String**> | Version of Deep Security Agent that is installed on the computer. When no agent software is installed or activated, the version is '0.0.0.0'. Searchable as String, however to search for computers without an agent, search for computers that have no agent fingerprint.'. | [optional][readonly]
**computer_status** | Option<[**crate::models::ComputerStatus**](computerStatus.md)> |  | [optional]
**tasks** | Option<[**crate::models::ComputerTasks**](computerTasks.md)> |  | [optional]
**security_updates** | Option<[**crate::models::SecurityUpdates**](securityUpdates.md)> |  | [optional]
**computer_settings** | Option<[**crate::models::ComputerSettings**](ComputerSettings.md)> |  | [optional]
**interfaces** | Option<[**crate::models::Interfaces**](Interfaces.md)> |  | [optional]
**bios_uuid** | Option<**String**> | BIOS UUID, for example: \"421e1137-5c49-56e4-246d-9bf7637e69f5\". Only applies to VMware VMs. | [optional][readonly]
**azure_arm_virtual_machine_summary** | Option<[**crate::models::AzureArmVirtualMachineSummary**](azureARMVirtualMachineSummary.md)> |  | [optional]
**azure_vm_virtual_machine_summary** | Option<[**crate::models::AzureVmVirtualMachineSummary**](azureVMVirtualMachineSummary.md)> |  | [optional]
**ec2_virtual_machine_summary** | Option<[**crate::models::Ec2VirtualMachineSummary**](ec2VirtualMachineSummary.md)> |  | [optional]
**no_connector_virtual_machine_summary** | Option<[**crate::models::NoConnectorVirtualMachineSummary**](noConnectorVirtualMachineSummary.md)> |  | [optional]
**vmware_vm_virtual_machine_summary** | Option<[**crate::models::VmwareVmVirtualMachineSummary**](vmwareVMVirtualMachineSummary.md)> |  | [optional]
**vcloud_vm_virtual_machine_summary** | Option<[**crate::models::VcloudVmVirtualMachineSummary**](vcloudVMVirtualMachineSummary.md)> |  | [optional]
**workspace_virtual_machine_summary** | Option<[**crate::models::WorkspaceVirtualMachineSummary**](workspaceVirtualMachineSummary.md)> |  | [optional]
**gcp_virtual_machine_summary** | Option<[**crate::models::GcpVirtualMachineSummary**](gcpVirtualMachineSummary.md)> |  | [optional]
**host_guid** | Option<**String**> | HostGUID of the computer. Searchable as String. | [optional][readonly]
**ID** | Option<**i32**> | ID of the computer. Searchable as ID. | [optional][readonly]
**anti_malware** | Option<[**crate::models::AntiMalwareComputerExtension**](AntiMalwareComputerExtension.md)> |  | [optional]
**web_reputation** | Option<[**crate::models::WebReputationComputerExtension**](WebReputationComputerExtension.md)> |  | [optional]
**activity_monitoring** | Option<[**crate::models::ActivityMonitoringComputerExtension**](ActivityMonitoringComputerExtension.md)> |  | [optional]
**firewall** | Option<[**crate::models::FirewallComputerExtension**](FirewallComputerExtension.md)> |  | [optional]
**intrusion_prevention** | Option<[**crate::models::IntrusionPreventionComputerExtension**](IntrusionPreventionComputerExtension.md)> |  | [optional]
**integrity_monitoring** | Option<[**crate::models::IntegrityMonitoringComputerExtension**](IntegrityMonitoringComputerExtension.md)> |  | [optional]
**log_inspection** | Option<[**crate::models::LogInspectionComputerExtension**](LogInspectionComputerExtension.md)> |  | [optional]
**application_control** | Option<[**crate::models::ApplicationControlComputerExtension**](ApplicationControlComputerExtension.md)> |  | [optional]
**esx_summary** | Option<[**crate::models::EsxSummary**](ESXSummary.md)> |  | [optional]
**SAP** | Option<[**crate::models::SapComputerExtension**](SAPComputerExtension.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# Policy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**parent_id** | Option<**i32**> | ID of the parent policy. Searchable as Numeric. | [optional]
**name** | Option<**String**> | Name of the policy. Searchable as String. | [optional]
**description** | Option<**String**> | Description of the policy. Searchable as String. | [optional]
**policy_settings** | Option<[**crate::models::PolicySettings**](PolicySettings.md)> |  | [optional]
**recommendation_scan_mode** | Option<**String**> | Enable or disable ongoing recommendation scans for computers assigned this policy. Searchable as Choice. | [optional]
**auto_requires_update** | Option<**String**> | Automatically update computers assigned this policy when the configuration changes. Searchable as Choice. | [optional]
**interface_types** | Option<[**crate::models::InterfaceTypes**](InterfaceTypes.md)> |  | [optional]
**ID** | Option<**i32**> | ID of the policy. Searchable as ID. | [optional][readonly]
**anti_malware** | Option<[**crate::models::AntiMalwarePolicyExtension**](AntiMalwarePolicyExtension.md)> |  | [optional]
**web_reputation** | Option<[**crate::models::WebReputationPolicyExtension**](WebReputationPolicyExtension.md)> |  | [optional]
**activity_monitoring** | Option<[**crate::models::ActivityMonitoringPolicyExtension**](ActivityMonitoringPolicyExtension.md)> |  | [optional]
**firewall** | Option<[**crate::models::FirewallPolicyExtension**](FirewallPolicyExtension.md)> |  | [optional]
**intrusion_prevention** | Option<[**crate::models::IntrusionPreventionPolicyExtension**](IntrusionPreventionPolicyExtension.md)> |  | [optional]
**integrity_monitoring** | Option<[**crate::models::IntegrityMonitoringPolicyExtension**](IntegrityMonitoringPolicyExtension.md)> |  | [optional]
**log_inspection** | Option<[**crate::models::LogInspectionPolicyExtension**](LogInspectionPolicyExtension.md)> |  | [optional]
**application_control** | Option<[**crate::models::ApplicationControlPolicyExtension**](ApplicationControlPolicyExtension.md)> |  | [optional]
**SAP** | Option<[**crate::models::SapPolicyExtension**](SAPPolicyExtension.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



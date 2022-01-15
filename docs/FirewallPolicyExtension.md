# FirewallPolicyExtension

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | Option<**String**> | Module state. The 'inherited' value is write-only. | [optional]
**module_status** | Option<[**crate::models::PolicyModuleStatus**](policyModuleStatus.md)> |  | [optional]
**global_stateful_configuration_id** | Option<**i32**> | ID of the Global Stateful Configuration. Set to 0 to remove any assignment. | [optional]
**stateful_configuration_assignments** | Option<[**crate::models::StatefulConfigurationAssignments**](StatefulConfigurationAssignments.md)> |  | [optional]
**rule_ids** | Option<**Vec<i32>**> | IDs of the assigned firewall rules. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



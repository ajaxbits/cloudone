# ApplicationControlPolicyExtension

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | Option<**String**> | Module state. The 'inherited' value is write-only. | [optional]
**module_status** | Option<[**crate::models::PolicyModuleStatus**](policyModuleStatus.md)> |  | [optional]
**block_unrecognized** | Option<**bool**> | Block unrecognized software until it is explicitly allowed. | [optional]
**ruleset_id** | Option<**i64**> | ID of the shared whitelist ruleset. | [optional]
**trust_ruleset_id** | Option<**String**> | ID of the trust ruleset. An empty string will set it to \"None\" and assigning a value of 0 will set it to \"Inherited\". | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



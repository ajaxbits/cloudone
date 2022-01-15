# ApplicationControlComputerExtension

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | Option<**String**> | Module state. The 'inherited' value is write-only. | [optional]
**module_status** | Option<[**crate::models::ComputerModuleStatus**](computerModuleStatus.md)> |  | [optional]
**block_unrecognized** | Option<**bool**> | Controls whether to block unrecognized software until it is explicitly allowed. Set to true to block. | [optional]
**ruleset_id** | Option<**i64**> | ID of the shared whitelist ruleset. | [optional]
**trust_ruleset_id** | Option<**String**> | ID of the trust ruleset. An empty string will set it to \"None\" and assigning a value of 0 will set it to \"Inherited\". | [optional]
**maintenance_mode_status** | Option<**String**> | Maintenance mode status. | [optional]
**maintenance_mode_duration** | Option<**i32**> | Duration of maintenance mode in minutes. | [optional]
**maintenance_mode_start_time** | Option<**i64**> | Timestamp of the date the maintenanceMode was started, in milliseconds since epoch. | [optional][readonly]
**maintenance_mode_end_time** | Option<**i64**> | Timestamp of the date the maintenanceMode was ended, in milliseconds since epoch. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# PolicyImportTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**policy_content** | Option<**String**> | Policy content zip and encoded by base64, write only. | [optional]
**status** | Option<**String**> | status | [optional][readonly]
**created** | Option<**i64**> | Timestamp of the created time, in milliseconds since epoch. Searchable as Date. | [optional][readonly]
**last_updated** | Option<**i64**> | Timestamp of the last updated time, in milliseconds since epoch. Searchable as Date. | [optional][readonly]
**policy_mappings** | Option<[**Vec<crate::models::PolicyMapping>**](policyMapping.md)> | Policy mappings on Workload Security | [optional][readonly]
**error_code** | Option<**i32**> | Type of import failed | [optional][readonly]
**ID** | Option<**i32**> | ID of the PolicyImportTask. | [optional][readonly]
**task_guid** | Option<**String**> | GUID of the task. Searchable as String. | [optional][readonly]
**source_host_name** | Option<**String**> | (Optional) Source Deep Security Manager hostname. Searchable as String. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



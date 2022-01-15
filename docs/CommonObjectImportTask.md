# CommonObjectImportTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**common_object_content** | **String** | Common object content zipped and base64-encoded, write only. | 
**_type** | **String** | Type of the common objects to import. Searchable as Choice. | 
**status** | Option<**String**> | Status of the task. | [optional][readonly]
**created** | Option<**i64**> | Timestamp of the created time, in milliseconds since epoch. Searchable as Date. | [optional][readonly]
**last_updated** | Option<**i64**> | Timestamp of the last updated time, in milliseconds since epoch. Searchable as Date. | [optional][readonly]
**common_object_mappings** | Option<[**Vec<crate::models::CommonObjectMapping>**](commonObjectMapping.md)> | Common object mapping between Deep Security Manager and Workload Security. | [optional][readonly]
**error_code** | Option<**i32**> | Type of failed import. | [optional][readonly]
**ID** | Option<**i32**> | ID of the CommonObjectImportTask. | [optional][readonly]
**task_guid** | Option<**String**> | GUID of the task. Searchable as String. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



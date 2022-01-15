# ApiUsageMetric

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_type** | Option<**String**> | Type of API called (\"REST\" or \"SOAP\"). Searchable as String. | [optional]
**action** | Option<**String**> | Http action (\"GET\", \"PUT\", \"POST\" or \"DELETE\"). Searchable as String. | [optional]
**method** | Option<**String**> | Method or API Endpoint called. Searchable as String. | [optional]
**date** | Option<**i64**> | Date of API call in milliseconds since epoch. Searchable as Date. | [optional]
**tenant_guid** | Option<**String**> | GUID of the tenant who called the API. Searchable as String. | [optional]
**duration** | Option<**i64**> | Duration of the API call from request to response in milliseconds. Searchable as Numeric. | [optional]
**status_code** | Option<**i32**> | Http status code returned by the API. Searchable as Numeric. | [optional]
**authentication_method** | Option<**String**> | Method used to authenticate the API call (\"sID\", \"API-Key\", or empty string). Searchable as String. | [optional]
**expand_parameter** | Option<**String**> | \"expand\" parameter passed to /computers endpoint. Searchable as String. | [optional]
**response_item_count** | Option<**i32**> | Number of items returned to the client from a list/search operation. Searchable as Numeric. | [optional]
**ID** | Option<**i64**> | ID of the APIUsageMetric. Searchable as ID. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



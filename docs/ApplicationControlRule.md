# ApplicationControlRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sha256** | Option<**String**> | SHA-256 hash of the executable. Searchable as String. | [optional][readonly]
**size** | Option<**i64**> | Size of the executable in bytes. Searchable as Numeric. | [optional][readonly]
**action** | Option<**String**> | Action to take when a user launches the executable. Searchable as Choice. | [optional]
**path** | Option<**String**> | Full path of the executable. Searchable as String. | [optional][readonly]
**file_name** | Option<**String**> | File name of the executable. | [optional][readonly]
**last_updated_administrator** | Option<**i32**> | ID of the last administrator to update the rule. Searchable as Numeric. | [optional][readonly]
**last_updated** | Option<**i64**> | Timestamp of the last rule modification, in milliseconds since epoch. Searchable as Date. | [optional][readonly]
**ID** | Option<**i64**> | ID of the rule. Searchable as ID. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# ApplicationControlGlobalRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sha256** | Option<**String**> | SHA-256 hash of the executable named in the rule. Searchable as String. | [optional]
**description** | Option<**String**> | Description of the rule. Searchable as String. | [optional]
**action** | Option<**String**> | Action to take when a user attempts to launch the executable named in the rule. Searchable as Choice. | [optional][readonly]
**last_updated_administrator** | Option<**i32**> | ID of the last administrator to update the rule. Searchable as Numeric. | [optional][readonly]
**last_updated** | Option<**i64**> | Timestamp of the last rule modification, in milliseconds since epoch. Searchable as Date. | [optional][readonly]
**ID** | Option<**i64**> | ID of the application control rule. Searchable as ID. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# ApiKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key_name** | Option<**String**> | Display name of the APIKey. Searchable as String. | [optional]
**description** | Option<**String**> | Description of the APIKey. Searchable as String. | [optional]
**locale** | Option<**String**> | Country and language for the APIKey. | [optional]
**role_id** | Option<**i32**> | ID of the role assigned to the APIKey. Searchable as Numeric. | [optional]
**time_zone** | Option<**String**> | Display name of the APIKey's time zone, e.g. America/New_York. Searchable as String. | [optional]
**active** | Option<**bool**> | If true, the APIKey can be used to authenticate. If false, the APIKey is locked out. Searchable as Boolean. | [optional]
**created** | Option<**i64**> | Timestamp of the APIKey's creation, in milliseconds since epoch. Searchable as Date. | [optional]
**last_sign_in** | Option<**i64**> | Timestamp of the APIKey's last successful authentication, in milliseconds since epoch. Searchable as Date. | [optional]
**unlock_time** | Option<**i64**> | Timestamp of when a locked out APIKey will be unlocked, in milliseconds since epoch. Searchable as Date. | [optional]
**unsuccessful_sign_in_attempts** | Option<**i32**> | Number of unsuccessful authentication attempts made since the last successful authentication. Searchable as Numeric. | [optional]
**expiry_date** | Option<**i64**> | Timestamp of the APIKey's expiry date, in milliseconds since epoch. Searchable as Date. | [optional]
**secret_key** | Option<**String**> | Secret key used to authenticate API requests. Only returned when creating a new APIKey or regenerating the secret key. | [optional][readonly]
**service_account** | Option<**bool**> | If true, the APIKey was created by the primary tenant (T0) to authenticate API calls against other tenants' databases. Searchable as Boolean. | [optional][readonly]
**ID** | Option<**i32**> | ID of the APIKey. Searchable as ID. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



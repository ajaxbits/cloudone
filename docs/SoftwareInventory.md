# SoftwareInventory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**computer_id** | Option<**i32**> | ID of the computer that the inventory scan was performed on (or is being performed on). Searchable as Numeric. | [optional]
**name** | Option<**String**> | Name of the inventory. Searchable as String. | [optional]
**description** | Option<**String**> | Description of the inventory. Searchable as String. | [optional]
**state** | Option<**String**> | State of the inventory scan. Searchable as Choice. | [optional][readonly]
**start_date** | Option<**i64**> | Time the inventory scan was started, in milliseconds since epoch. Searchable as Date. | [optional][readonly]
**completed_date** | Option<**i64**> | Time the inventory scan was completed, in milliseconds since epoch. Searchable as Date. | [optional][readonly]
**initiator_id** | Option<**i32**> | Either the ID of the administrator that initiated the inventory scan or the ID of the API key if the inventory scan was initiated using an API Key. Searchable as Numeric. | [optional][readonly]
**ID** | Option<**i64**> | ID of the software inventory. Searchable as ID. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



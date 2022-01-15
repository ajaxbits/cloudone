# EventBasedTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of event based task. Searchable as String. | [optional]
**_type** | Option<**String**> | Type of scheduled task. Searchable as Choice. | [optional]
**enabled** | Option<**bool**> | Indicates whether or not the event based task is enabled. Searchable as Boolean. | [optional]
**last_run_time** | Option<**i64**> | The last time this event based task was run, or null if never run. Searchable as Date. | [optional]
**actions** | Option<[**Vec<crate::models::EventBasedTaskAction>**](EventBasedTaskAction.md)> | List of actions to perform when an event based task condition is met. | [optional]
**conditions** | Option<[**Vec<crate::models::EventBasedTaskCondition>**](EventBasedTaskCondition.md)> | List of conditions that must be met in order for an event based task to run its actions. | [optional]
**ID** | Option<**i32**> | Event based task identifier. Searchable as ID. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



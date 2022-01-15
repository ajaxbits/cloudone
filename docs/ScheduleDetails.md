# ScheduleDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**time_zone** | Option<**String**> | The timezone used to interpret the scheduled task schedule. | [optional]
**recurrence_type** | Option<**String**> | Recurrence type. | [optional]
**recurrence_count** | Option<**i32**> | Number of times the task should execute. | [optional]
**hourly_schedule_parameters** | Option<[**crate::models::HourlyScheduleParameters**](HourlyScheduleParameters.md)> |  | [optional]
**daily_schedule_parameters** | Option<[**crate::models::DailyScheduleParameters**](DailyScheduleParameters.md)> |  | [optional]
**weekly_schedule_parameters** | Option<[**crate::models::WeeklyScheduleParameters**](WeeklyScheduleParameters.md)> |  | [optional]
**monthly_schedule_parameters** | Option<[**crate::models::MonthlyScheduleParameters**](MonthlyScheduleParameters.md)> |  | [optional]
**once_only_schedule_parameters** | Option<[**crate::models::OnceOnlyScheduleParameters**](OnceOnlyScheduleParameters.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



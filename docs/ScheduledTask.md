# ScheduledTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of scheduled task. Searchable as String. | [optional]
**_type** | Option<**String**> | Type of scheduled task. Searchable as Choice. | [optional]
**schedule_details** | Option<[**crate::models::ScheduleDetails**](ScheduleDetails.md)> |  | [optional]
**enabled** | Option<**bool**> | Indicates whether or not the scheduled task is enabled. Searchable as Boolean. | [optional]
**last_run_time** | Option<**i64**> | The last time this scheduled task was run, or null if never run. Searchable as Date. | [optional]
**next_run_time** | Option<**i64**> | The next time this scheduled task is scheduled to run, or null if it not scheduled to run in the future. Searchable as Date. | [optional]
**run_now** | Option<**bool**> | Indicates that the scheduled task should execute immediately. | [optional]
**scan_for_open_ports_task_parameters** | Option<[**crate::models::ScanForOpenPortsTaskParameters**](ScanForOpenPortsTaskParameters.md)> |  | [optional]
**send_alert_summary_task_parameters** | Option<[**crate::models::SendAlertSummaryTaskParameters**](SendAlertSummaryTaskParameters.md)> |  | [optional]
**discover_computers_task_parameters** | Option<[**crate::models::DiscoverComputersTaskParameters**](DiscoverComputersTaskParameters.md)> |  | [optional]
**run_script_task_parameters** | Option<[**crate::models::RunScriptTaskParameters**](RunScriptTaskParameters.md)> |  | [optional]
**send_policy_task_parameters** | Option<[**crate::models::SendPolicyTaskParameters**](SendPolicyTaskParameters.md)> |  | [optional]
**generate_report_task_parameters** | Option<[**crate::models::GenerateReportTaskParameters**](GenerateReportTaskParameters.md)> |  | [optional]
**synchronize_directory_task_parameters** | Option<[**crate::models::SynchronizeDirectoryTaskParameters**](SynchronizeDirectoryTaskParameters.md)> |  | [optional]
**scan_for_recommendations_task_parameters** | Option<[**crate::models::ScanForRecommendationsTaskParameters**](ScanForRecommendationsTaskParameters.md)> |  | [optional]
**synchronize_v_center_task_parameters** | Option<[**crate::models::SynchronizeVCenterTaskParameters**](SynchronizeVCenterTaskParameters.md)> |  | [optional]
**scan_for_integrity_changes_task_parameters** | Option<[**crate::models::ScanForIntegrityChangesTaskParameters**](ScanForIntegrityChangesTaskParameters.md)> |  | [optional]
**scan_for_malware_task_parameters** | Option<[**crate::models::ScanForMalwareTaskParameters**](ScanForMalwareTaskParameters.md)> |  | [optional]
**check_for_security_updates_task_parameters** | Option<[**crate::models::CheckForSecurityUpdatesTaskParameters**](CheckForSecurityUpdatesTaskParameters.md)> |  | [optional]
**synchronize_cloud_account_task_parameters** | Option<[**crate::models::SynchronizeCloudAccountTaskParameters**](SynchronizeCloudAccountTaskParameters.md)> |  | [optional]
**update_suspicious_objects_list_task_parameters** | Option<[**crate::models::UpdateSuspiciousObjectsListTaskParameters**](UpdateSuspiciousObjectsListTaskParameters.md)> |  | [optional]
**ID** | Option<**i32**> | Scheduled task identifier. Searchable as ID. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



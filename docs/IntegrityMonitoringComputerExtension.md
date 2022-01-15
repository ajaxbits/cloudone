# IntegrityMonitoringComputerExtension

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | Option<**String**> | Module state. The 'inherited' value is write-only. | [optional]
**module_status** | Option<[**crate::models::ComputerModuleStatus**](computerModuleStatus.md)> |  | [optional]
**last_integrity_scan** | Option<**i64**> | Timestamp of the last full scan for integrity, in milliseconds since epoch. | [optional][readonly]
**last_baseline_created** | Option<**i64**> | Timestamp of the last integrity baseline, in milliseconds since epoch. | [optional][readonly]
**rule_ids** | Option<**Vec<i32>**> | IDs of the assigned Integrity Monitoring rules. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



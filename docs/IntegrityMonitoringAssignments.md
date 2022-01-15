# IntegrityMonitoringAssignments

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assigned_rule_ids** | Option<**Vec<i32>**> | IDs of IntegrityMonitoringRules assigned to the computer or policy. | [optional]
**recommendation_scan_status** | Option<**String**> | Status of the last recommendation scan done on a computer or policy. | [optional][readonly]
**last_recommendation_scan_date** | Option<**i64**> | Timestamp of the last recommendation scan date, in milliseconds since epoch. | [optional][readonly]
**recommended_to_assign_rule_ids** | Option<**Vec<i32>**> | IntegrityMonitoringRules separated by commas, that were recommended to be assigned to the computer or policy by a recommendation scan. | [optional][readonly]
**recommended_to_unassign_rule_ids** | Option<**Vec<i32>**> | IntegrityMonitoringRules, separated by commas, that were recommended to be unassigned from the computer or policy by a recommendation scan. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



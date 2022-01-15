# IntrusionPreventionAssignments

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assigned_rule_ids** | Option<**Vec<i32>**> | IDs of the intrusion prevention rules currently assigned. | [optional]
**assigned_application_type_ids** | Option<**Vec<i32>**> | IDs of the application types currently assigned. | [optional][readonly]
**recommendation_scan_status** | Option<**String**> | Status of the last recommendation scan. | [optional]
**last_recommendation_scan_date** | Option<**i64**> | Timestamp of the last recommendation scan, in milliseconds since epoch. | [optional]
**recommended_to_assign_rule_ids** | Option<**Vec<i32>**> | IDs of the intrusion prevention rules recommended for assignment. | [optional]
**recommended_to_unassign_rule_ids** | Option<**Vec<i32>**> | IDs of the intrusion prevention rules recommended for unassignment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



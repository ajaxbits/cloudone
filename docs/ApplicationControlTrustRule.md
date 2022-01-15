# ApplicationControlTrustRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | Option<**String**> | Type of the rule. Searchable as Choice. | [optional]
**name** | Option<**String**> | Name of the trust rule. Searchable as String. | [optional]
**description** | Option<**String**> | Description of the trust rule. Searchable as String. | [optional]
**is_assigned** | Option<**bool**> | Whether the rule is assigned to any rulesets | [optional][readonly]
**created_time** | Option<**i64**> | Timestamp of the trust rule's creation, in milliseconds since epoch. Searchable as Date. | [optional][readonly]
**last_updated_time** | Option<**i64**> | Timestamp of when the trust rule was last updated, in milliseconds since epoch. Searchable as Date. | [optional][readonly]
**ruleset_ids** | Option<**Vec<i64>**> | IDs of all rulesets that the rule is assigned to. | [optional][readonly]
**attributes** | Option<[**Vec<crate::models::TrustRuleAttribute>**](TrustRuleAttribute.md)> | Attributes that define the trust rule. | [optional]
**ID** | Option<**i64**> | ID of the trust rule. Searchable as ID. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



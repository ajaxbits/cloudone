# SecurityUpdates

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**update_status** | Option<[**crate::models::UpdateStatus**](updateStatus.md)> |  | [optional]
**last_changed** | Option<**i64**> | Date when components were last updated, in milliseconds since epoch. | [optional][readonly]
**rules** | Option<[**Vec<crate::models::Component>**](component.md)> | Security update components: rules. | [optional][readonly]
**anti_malware** | Option<[**Vec<crate::models::Component>**](component.md)> | Security update components: anti-malware. | [optional][readonly]
**web_reputation_service** | Option<[**Vec<crate::models::Component>**](component.md)> | Security update components: web reputation service. | [optional][readonly]
**manifests** | Option<[**Vec<crate::models::Component>**](component.md)> | Security update components: manifests. | [optional][readonly]
**other** | Option<[**Vec<crate::models::Component>**](component.md)> | Security update components: other. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



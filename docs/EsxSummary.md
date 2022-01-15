# EsxSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**manufacturer** | Option<**String**> | Manufacturer of the ESX. Searchable as String. | [optional][readonly]
**model** | Option<**String**> | Model of the ESX. Searchable as String. | [optional][readonly]
**processors** | Option<**String**> | Quantity and processor speed of the ESX's processors. Searchable as Numeric. | [optional][readonly]
**processor_type** | Option<**String**> | Detailed information on the make and model of the ESX's processor. Searchable as String. | [optional][readonly]
**number_of_nics** | Option<**String**> | Number of Network interface controllers the ESX has. Searchable as Numeric. | [optional][readonly]
**state** | Option<**String**> | State of the ESX. | [optional][readonly]
**virtual_machines** | Option<**String**> | Number of virtual machines hosted on the ESX. | [optional][readonly]
**v_motion_enabled** | Option<**String**> | Indicates whether vMotion is enabled on the ESX. The is value is true if enabled. Searchable as Boolean. | [optional][readonly]
**custom_attributes** | Option<[**Vec<crate::models::CustomAttribute>**](customAttribute.md)> | List of name/value pairs of ESX Custom Attributes | [optional][readonly]
**tpm_enabled** | Option<**bool**> |  | [optional]
**tpm_alerts_enabled** | Option<**bool**> |  | [optional]
**tpm_has_data** | Option<**bool**> |  | [optional]
**tpm_last_checked** | Option<**i64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



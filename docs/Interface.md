# Interface

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**interface_type_id** | Option<**i32**> | ID of the InterfaceType to which the Interface is mapped. Searchable as Numeric. | [optional]
**name** | Option<**String**> | Name of the Interface. Set automatically by the DSM. Searchable as String. | [optional][readonly]
**display_name** | Option<**String**> | Display name of the Interface. Optionally set by the user. Searchable as String. | [optional]
**detected** | Option<**bool**> | Indicates whether or not the interface is currently detected. Searchable as Boolean. | [optional][readonly]
**ID** | Option<**i32**> | ID of the Interface. | [optional][readonly]
**MAC** | Option<**String**> | MAC Address of the interface. Searchable as String. | [optional][readonly]
**DHCP** | Option<**bool**> | Indicates whether the interface uses DHCP. The value is true if it uses DHCP. Searchable as Boolean. | [optional][readonly]
**ips** | Option<**Vec<String>**> | List of IPs used by the interface. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# IpList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the IP list. Searchable as String. | [optional]
**description** | Option<**String**> | Description of the IP list. Searchable as String. | [optional]
**items** | Option<**Vec<String>**> | List of comma-delimited IP addresses. Can contain individual IPs, masked IPs (for example: \"192.168.2.0./24\", \"192.168.2.0/255.255.255.0\" or \"2001:0DB8::CD30:0:0:0:0/60\") or IP ranges (for example: \"192.168.0.2 - 192.168.0.125\" or \"FF01::101 - FF01::102\"). | [optional]
**ID** | Option<**i32**> | ID of the IP list. Searchable as ID. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



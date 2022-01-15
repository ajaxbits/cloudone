# StatefulConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the stateful configuration. Searchable as String. | [optional]
**description** | Option<**String**> | Description of the stateful configuration. Searchable as String. | [optional]
**deny_fragmented_packets_enabled** | Option<**bool**> | Controls if fragmented packets are denied. Set to true to deny fragmented packets. Searchable as Boolean. | [optional]
**deny_packets_containing_cwr_or_ece_enabled** | Option<**bool**> | Controls if TCP CWR, ECE flags are denied. Set to true to enable CWR or ECE flags. Searchable as Boolean. | [optional]
**max_incoming_connections** | Option<**i32**> | Maximum allowed incoming connections. Searchable as Numeric. | [optional]
**max_outgoing_connections** | Option<**i32**> | Maximum allowed outgoing connections. Searchable as Numeric. | [optional]
**max_half_open_connections** | Option<**i32**> | Maximum allowed half open connections. Searchable as Numeric. | [optional]
**tcpstateful_inspection_enabled** | Option<**bool**> |  | [optional]
**tcpstateful_logging_enabled** | Option<**bool**> |  | [optional]
**udpstateful_inspection_enabled** | Option<**bool**> |  | [optional]
**udpstateful_logging_enabled** | Option<**bool**> |  | [optional]
**icmpstateful_inspection_enabled** | Option<**bool**> |  | [optional]
**icmpstateful_logging_enabled** | Option<**bool**> |  | [optional]
**ID** | Option<**i32**> | ID of the stateful configuration. Searchable as ID. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



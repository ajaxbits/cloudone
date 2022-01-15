# Context

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the context. Searchable as String. | [optional]
**description** | Option<**String**> | Description of the context. Searchable as String. | [optional]
**minimum_agent_version** | Option<**String**> | Minimum supported agent version. | [optional]
**local_connections_enabled** | Option<**bool**> | Specifies whether the context applies to connections that are locally conntected to a domain. Set to true to apply to locally connected. | [optional]
**remote_connections_enabled** | Option<**bool**> | Specifies whether the context applies to connections that are remotely conntected to a domain. Set to true to apply to remotely connected. | [optional]
**no_connection_enabled** | Option<**bool**> | Specifies whether the context applies to connections that are no connection enabled. Set to true to apply to no connection enabled. | [optional]
**no_internet_enabled** | Option<**bool**> | Specifies whether the context applies to connections that are neither connection nor Internet enabled. Set to true to apply. | [optional]
**restricted_interfaces_enabled** | Option<**bool**> | Controls if the firewall contents are restricted from view and duplication. Set to true if it's restricted. Searchable as Boolean. | [optional]
**ID** | Option<**i32**> | ID of the context. Searchable as ID. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



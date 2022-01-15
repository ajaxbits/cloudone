# ApplicationType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Display name of the ApplicationType. Searchable as String. | [optional]
**description** | Option<**String**> | Description of the ApplicationType. Searchable as String. | [optional]
**minimum_agent_version** | Option<**String**> | Version of the Deep Security agent or appliance required to support the ApplicationType. Searchable as String. | [optional]
**direction** | Option<**String**> | Direction of the initial communication for the ApplicationType (e.g. 'outgoing' for web browsers). Searchable as Choice. | [optional]
**protocol** | Option<**String**> | Protocol used by the ApplicationType. Searchable as Choice. | [optional]
**port_type** | Option<**String**> | Port number configuration type. Searchable as Choice. | [optional]
**port_multiple** | Option<**Vec<String>**> | If portType is multiple, the list of port numbers the ApplicationType monitors. Searchable as String. | [optional]
**port_list_id** | Option<**i32**> | If portType is port-list, ID of the PortList containing the port numbers the ApplicationType monitors. Set to 0 to remove any assignment. Searchable as Numeric. | [optional]
**recommendations_mode** | Option<**String**> | Indicates whether recommendation scans consider the ApplicationType. Create an ApplicationType computer or policy override to modify this value. Searchable as Choice. | [optional]
**ID** | Option<**i32**> | ID of the ApplicationType. Searchable as ID. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



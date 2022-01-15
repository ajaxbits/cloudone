# GcpVirtualMachineSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cloud_provider** | Option<**String**> | Cloud provider: \"GCP\". | [optional]
**state** | Option<**String**> | Power state, for example, \"POWER ON\". | [optional]
**operating_system** | Option<**String**> | Operating system, for example: \"Microsoft Windows (64 bit)\". Searchable as String. | [optional]
**instance_id** | Option<**String**> | Instance ID, for example: \"1234567890123456789\". Searchable as String. | [optional]
**private_ip_address** | Option<**String**> | Private IP address. Searchable as String. | [optional]
**public_ip_address** | Option<**String**> | Public IP address. Searchable as String. | [optional]
**zone** | Option<**String**> | Zone is a deployment area, for example: \"us-east1-d\". Searchable as String. | [optional]
**v_cpus** | Option<**i32**> | Number of vCPU. Searchable as Numeric. | [optional]
**memory** | Option<**i32**> | Memory size in megabyte. Searchable as Numeric. | [optional]
**labels** | Option<[**Vec<crate::models::VirtualMachineGcpLabel>**](VirtualMachineGCPLabel.md)> | List of key/value labels. | [optional]
**network_tags** | Option<**Vec<String>**> | List of network tags. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



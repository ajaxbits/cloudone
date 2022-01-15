# AzureArmVirtualMachineSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cloud_provider** | Option<**String**> | Cloud provider: \"Azure\". | [optional]
**subscription_id** | Option<**String**> | Subscription ID. Searchable as String. | [optional]
**deployment_model** | Option<**String**> | Deployment model: \"Classic\" or \"Resource Manager\". | [optional]
**resource_group** | Option<**String**> | Name of resource group. Searchable as String. | [optional]
**state** | Option<**String**> | Power state, for example, \"POWER ON\". | [optional]
**location** | Option<**String**> | Physical location of the resource, for example: \"East US\". Searchable as String. | [optional]
**_type** | Option<**String**> | Hardware type, for example: \"Standard_DS1_v2\". Searchable as String. | [optional]
**operating_system** | Option<**String**> | Operating system, for example: \"Microsoft Windows\". Searchable as String. | [optional]
**public_ip_address** | Option<**String**> | Public IP address. Searchable as String. | [optional]
**private_ip_address** | Option<**String**> | Private IP address. Searchable as String. | [optional]
**cloud_service** | Option<**String**> | Cloud service, for example: \"DH-DC\". Searchable as String. | [optional]
**deployment_id** | Option<**String**> | Deployment ID, for example: \"76ab36a0fb8d4c4ab6b802acdf58b3a4\". Searchable as String. | [optional]
**image_id** | Option<**String**> | Image ID, for example: \"a699494373c04fc0bc8f2bb1389d6106__Windows-Server-2012-R2-201503.01-en.us-127GB.vhd\". Searchable as String. | [optional]
**instance_id** | Option<**String**> | Instance ID. Searchable as String. | [optional]
**security_group** | Option<**String**> | Network security group, for example: \"bh-Win10Pro-1-nsg\". Searchable as String. | [optional]
**metadata** | Option<[**Vec<crate::models::VirtualMachineMetadata>**](VirtualMachineMetadata.md)> | List of tag name/value metadata pairs. | [optional]
**azure_resource_id** | Option<**String**> |  | [optional]
**dns_name** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



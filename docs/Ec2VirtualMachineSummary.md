# Ec2VirtualMachineSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cloud_provider** | Option<**String**> | Cloud provider: \"AWS\". | [optional]
**account_id** | Option<**String**> | Account ID. Searchable as String. | [optional]
**operating_system** | Option<**String**> | Operating system, for example: \"Microsoft Windows (64 bit)\". Searchable as String. | [optional]
**private_ip_address** | Option<**String**> | Private IP address. Searchable as String. | [optional]
**public_ip_address** | Option<**String**> | Public IP address. Searchable as String. | [optional]
**availability_zone** | Option<**String**> | Availability Zone, for example: \"us-east-1a\". Searchable as String. | [optional]
**instance_id** | Option<**String**> | Instance ID, for example: \"i-0e80f75f9532ad1ba\". Searchable as String. | [optional]
**security_groups** | Option<[**Vec<crate::models::VirtualMachineSecurityGroup>**](VirtualMachineSecurityGroup.md)> | List of security groups. | [optional]
**_type** | Option<**String**> | Instance type, for example: \"t2.micro\". Searchable as String. | [optional]
**virtualization_type** | Option<**String**> | Virtualization type, for example: \"hvm\". Searchable as String. | [optional]
**state** | Option<**String**> | Power state, for example, \"POWER ON\". | [optional]
**metadata** | Option<[**Vec<crate::models::VirtualMachineMetadata>**](VirtualMachineMetadata.md)> | List of name/value metadata pairs. | [optional]
**dns_name** | Option<**String**> |  | [optional]
**ami_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



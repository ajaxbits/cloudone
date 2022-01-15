# ComputerGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | Option<**String**> | Specifies what type the ComputerGroup is. Defaults to `folder` | [optional][readonly]
**name** | Option<**String**> | Name of the ComputerGroup. Searchable as String. | [optional]
**description** | Option<**String**> | Description of the ComputerGroup. Searchable as String. | [optional]
**parent_group_id** | Option<**i32**> | ID of the ComputerGroup's parent group. Empty if the parent is a root ComputerGroup. ComputerGroup will be a root group unless a valid value for `parentGroupID` is set. Searchable as Numeric. | [optional]
**directory_id** | Option<**i32**> | ID of the ComputerGroup's directory server. Set to `0` if the group is not a directory server. Searchable as Numeric. | [optional][readonly]
**external_id** | Option<**String**> | External ID of the ComputerGroup. Empty if the ComputerGroup is not created/managed by a cloud account. Searchable as String. | [optional][readonly]
**virtual_id** | Option<**i32**> | ID of the ComputerGroup as it exists in VMware vCloud. Set to `0` if the ComputerGroup is not from vCloud. Searchable as Numeric. | [optional][readonly]
**virtual_type** | Option<**i32**> | Type of the ComputerGroup as it exists in VMware vCloud. Set to `0` if the ComputerGroup is not from vCloud. Searchable as Numeric. | [optional][readonly]
**virtual_name** | Option<**String**> | Name of the ComputerGroup as it exists in VMware vCloud. Ignored if the ComputerGroup is not from vCloud. Searchable as String. | [optional][readonly]
**cloud_type** | Option<**String**> | Cloud platform of the ComputerGroup.  Ignored if the ComputerGroup does not represent a cloud container. Searchable as Choice. | [optional][readonly]
**cloud_resource_type** | Option<**String**> | Cloud container type of the ComputerGroup. This is platform dependent. Ignored if the ComputerGroup does not represent a cloud container. Searchable as Numeric. | [optional][readonly]
**cloud_id** | Option<**i32**> | Cloud container ID of the ComputerGroup. Ignored if the ComputerGroup does not represent a cloud container. Searchable as Numeric. | [optional][readonly]
**amazon_account_id** | Option<**i32**> | Amazon Web Services account ID of the ComputerGroup. Set to `0` if the ComputerGroup does not represent an Amazon Web Services account. Searchable as Numeric. | [optional][readonly]
**amazon_region_id** | Option<**i64**> | Amazon Web Services region ID of the ComputerGroup. Set to `0` if the ComputerGroup does not represent an Amazon Web Services region. amazonWorkspacesID will be used instead if the ComputerGroup represents an Amazon Web Services WorkSpaces node. Searchable as Numeric. | [optional][readonly]
**amazon_vpcid** | Option<**i64**> | Amazon Web Services Virtual Private Cloud ID of the ComputerGroup. Set to `0` if the ComputerGroup does not represent an Amazon Web Services Virtual Private Cloud. Searchable as Numeric. | [optional][readonly]
**amazon_subnet_id** | Option<**i64**> | Amazon Web Services subnet ID of the ComputerGroup. Set to `0` if the ComputerGroup does not represent an Amazon Web Services subnet. Searchable as Numeric. | [optional][readonly]
**amazon_workspaces_id** | Option<**i64**> | Amazon Web Services WorkSpaces ID of the ComputerGroup. Set to `0` if the ComputerGroup does not represent an Amazon Web Services WorkSpace. Will be used instead of amazonRegionID if the ComputerGroup represents a WorkSpaces node under a region. Searchable as Numeric. | [optional][readonly]
**amazon_directory_id** | Option<**i64**> | Amazon Web Services directory ID of the ComputerGroup. Set to `0` if the ComputerGroup does not represent an Amazon Web Services directory. Searchable as Numeric. | [optional][readonly]
**ID** | Option<**i32**> | ID of the ComputerGroup. Searchable as ID. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



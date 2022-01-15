# Role

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the administrator role. Searchable as String. | [optional]
**description** | Option<**String**> | Description of the administrator role. Searchable as String. | [optional]
**urn** | Option<**String**> | Uniform resource name. | [optional][readonly]
**immutable** | Option<**bool**> | The default immutable role in Deep Security Manager. | [optional][readonly]
**can_only_manipulate_users_with_equal_or_lesser_rights** | Option<**bool**> | Controls whether or not the role can only manipulate users with equal or lesser rights. Searchable as Boolean. | [optional]
**all_computers** | Option<**bool**> | Controls whether or not the role is allowed to access all computers. Searchable as Boolean. | [optional]
**all_policies** | Option<**bool**> | Controls whether or not the role is allowed to access all policies. Searchable as Boolean. | [optional]
**allow_user_interface** | Option<**bool**> | Controls whether or not the role is allowed to use the user interface. Searchable as Boolean. | [optional]
**allow_web_service** | Option<**bool**> | Controls whether or not the role is allowed to use the web service API. Searchable as Boolean. | [optional]
**rights** | Option<[**crate::models::Rights**](rights.md)> |  | [optional]
**computer_ids** | Option<**Vec<i32>**> | List of computer IDs that the role can access. Ignored if 'allComputers' is true. | [optional]
**computer_group_ids** | Option<**Vec<i32>**> | List of computer group IDs that the role can access. A group ID of '0' allows access to computers not in a computer group. Note that groups must be identified individually and that access to sub-groups is not automatically granted. Ignored if 'allComputers' is true. | [optional]
**policy_ids** | Option<**Vec<i32>**> | List of policy IDs that the role can access. Ignored if 'allPolicies' is true. | [optional]
**ID** | Option<**i32**> | ID of the administrator role. Searchable as ID. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



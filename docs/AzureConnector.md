# AzureConnector

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The Azure Connector's display name in DSM. Searchable as String. | [optional]
**azure_tenant_id** | Option<**String**> | The Azure Tenant/Active Directory ID. Searchable as String. | [optional]
**subscription_id** | Option<**String**> | The Azure Subscription ID. Searchable as String. | [optional]
**azure_ad_application_id** | Option<**String**> | The Azure Active Directory Application ID. Searchable as String. | [optional]
**azure_ad_application_secret** | Option<**String**> | The Azure Active Directory Application secret/password. Not present in returned objects. | [optional]
**resource_api_end_point** | Option<**String**> | The optional Azure Resource REST API endpoint. Will auto detect by DSM VM location if not provided. This is an advanced option used if you want to access private or confidential cloud. Be aware that managing Azure Government computers outside Azure Government would break ITAR compliance. | [optional]
**login_api_end_point** | Option<**String**> | The optional Azure Active Directory Login/Authentication API endpoint. Will auto detect by DSM VM location if not provided. This is an advanced option used if you want to access private or confidential cloud. Be aware that managing Azure Government computers outside Azure Government would break ITAR compliance. | [optional]
**last_sync_time** | Option<**i64**> | Timestamp of the last time the Azure Connector was successfully synchronized, in milliseconds since epoch. | [optional][readonly]
**ID** | Option<**i32**> | Azure Connector ID. Searchable as ID. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



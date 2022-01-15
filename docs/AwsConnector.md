# AwsConnector

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_name** | Option<**String**> | The AWS Connector's display name in DSM. | [optional]
**account_id** | Option<**String**> | The AWS Account ID. Searchable as String. | [optional]
**account_alias** | Option<**String**> | The AWS Account Alias. Searchable as String. | [optional]
**access_key** | Option<**String**> | The AWS Access Key of the account. If used, Cross Account Role ARN is not required. Searchable as String. | [optional]
**secret_key** | Option<**String**> | The AWS Secret Key required to add the connector using an Access Key. Not present in returned objects. | [optional]
**seed_region** | Option<**String**> | The region to initialize the EC2 client in. This is an advanced option used if you want to access special regions. Searchable as String. | [optional]
**use_instance_role** | Option<**bool**> | Specifies whether or not to use the DSM instance role to add the AWS Connector instead of an Access Key or a Cross Account Role ARN. | [optional]
**cross_account_role_arn** | Option<**String**> | The Cross Account Role ARN of the AWS account. If used, Access Key is not required. Searchable as String. | [optional]
**last_sync_time** | Option<**i64**> | Timestamp of the last time the AWS Connector was successfully synchronized, in milliseconds since epoch. Searchable as Date. | [optional]
**synced_regions** | Option<[**Vec<crate::models::AwsRegion>**](AWSRegion.md)> | The list of AWS regions that have been synchronized for the connector. | [optional]
**workspaces_enabled** | Option<**bool**> | A flag controlling whether or not Amazon Workspaces are enabled for the connector. Searchable as Boolean. Default is false. | [optional]
**ID** | Option<**i32**> | The Deep Security internal ID of the AWS Cloud Connector. Searchable as ID. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



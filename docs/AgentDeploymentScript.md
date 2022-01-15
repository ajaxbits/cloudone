# AgentDeploymentScript

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**platform** | Option<**String**> | Platform type for agent deployment. | [optional]
**validate_certificate_required** | Option<**bool**> | Validate if Deep Security Manager is using a valid TLS certificate from a trusted certificate authority (CA) when downloading the agent software. | [optional]
**validate_digital_signature_required** | Option<**bool**> | Validate digital signature of Deep Security Agent installer. | [optional]
**activation_required** | Option<**bool**> | Activate the agent at startup. | [optional]
**dsm_proxy_id** | Option<**i32**> | ID of the proxy server for contacting Deep Security Manager. | [optional]
**relay_proxy_id** | Option<**i32**> | ID of the proxy server for contacting Relay(s). | [optional]
**policy_id** | Option<**i32**> | ID of the policy assigned to the computer. | [optional]
**relay_group_id** | Option<**i32**> | ID of the relay group to which the computer belongs. | [optional]
**computer_group_id** | Option<**i32**> | ID of the computer group to which the computer belongs. | [optional]
**script_body** | Option<**String**> | Agent deployment script. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



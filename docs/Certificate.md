# Certificate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ID** | Option<**i32**> | ID of the SSL certificate. This ID is set automatically. | [optional][readonly]
**certificate** | Option<**String**> | The certificate. It's a PEM formatted string | [optional]
**certificate_details** | Option<[**crate::models::CertificateDetails**](certificateDetails.md)> |  | [optional]
**trusted** | Option<**bool**> | True if the certificate is trusted by Deep Security Manager or verified by a CA. | [optional][readonly]
**purpose** | Option<**String**> | The type of connections for which the certificate is to be used. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



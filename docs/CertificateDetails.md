# CertificateDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**issuer_dn** | Option<**String**> | The certificate issuer. | [optional][readonly]
**subject_dn** | Option<**String**> | The certificate subject (owner). | [optional][readonly]
**not_before** | Option<**i64**> | The date on which the certificate validity period begins. | [optional][readonly]
**not_after** | Option<**i64**> | The date on which the certificate validity period ends. | [optional][readonly]
**serial_number** | Option<**String**> | A number that uniquely identifies the certificate and is issued by the certification authority. | [optional][readonly]
**sha1_fingerprint** | Option<**String**> | The sha1 fingerprint of the certificate. | [optional][readonly]
**sha256_fingerprint** | Option<**String**> | The sha256 fingerprint of the certificate. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



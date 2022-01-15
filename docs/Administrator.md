# Administrator

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**username** | Option<**String**> | Username of the Administrator. Searchable as String. Not modifiable by Trend Micro Cloud One users. | [optional]
**password** | Option<**String**> | Password of the Administrator. Not modifiable by Trend Micro Cloud One users. | [optional]
**full_name** | Option<**String**> | Full name of the Administrator. Searchable as String. Not modifiable by Trend Micro Cloud One users. | [optional]
**description** | Option<**String**> | Description of the Administrator. Searchable as String. Not modifiable by Trend Micro Cloud One users. | [optional]
**role_id** | Option<**i32**> | ID of the role assigned to the Administrator. Searchable as Numeric. Not modifiable by Trend Micro Cloud One users. | [optional]
**locale** | Option<**String**> | Locale of the Administrator. Not modifiable by Trend Micro Cloud One users. | [optional]
**time_zone** | Option<**String**> | Time zone of the Administrator. Searchable as String. Not modifiable by Trend Micro Cloud One users. | [optional]
**time_format** | Option<**String**> | Time format preference of the Administrator. Can be either the 12-hour format or the 24-hour format. Searchable as Choice. Not modifiable by Trend Micro Cloud One users. | [optional]
**password_never_expires** | Option<**bool**> | Enabled if the Administrator's password never expires. Defaults to `false`. Searchable as Boolean. Not modifiable by Trend Micro Cloud One users. | [optional]
**active** | Option<**bool**> | If set to `true`, the Administrator can authenticate. If set to `false`, the Administrator is locked out. Searchable as Boolean. Not modifiable by Trend Micro Cloud One users. | [optional]
**mfa_type** | Option<**String**> | Specifies the type of multi-factor authentication used to authenticate the Administrator. Defaults to `none`. Searchable as Choice. Not modifiable by Trend Micro Cloud One users. | [optional]
**phone_number** | Option<**String**> | Phone number of the Administrator. Searchable as String. | [optional]
**mobile_number** | Option<**String**> | Mobile number of the Administrator. Searchable as String. | [optional]
**pager_number** | Option<**String**> | Pager number of the Administrator. Searchable as String. | [optional]
**email_address** | Option<**String**> | Email address of the Administrator. Searchable as String. | [optional]
**primary_contact** | Option<**bool**> | If set to `true`, the administrator is a primary contact. Primary contacts receive Deep Security as a Service account-related emails for that tenant. A valid `emailAddress` is required. Searchable as Boolean. | [optional]
**receive_notifications** | Option<**bool**> | If set to `true`, alert emails will be sent to the Administrator. A valid `emailAddress` is required. Searchable as Boolean. | [optional]
**report_pdf_password_enabled** | Option<**bool**> | Controls whether the reports that the Administrator generates are password-protected. Set to `true` to password-protect, and `false` otherwise. Defaults to `false`. Searchable as Boolean. | [optional]
**report_pdf_password** | Option<**String**> | Password that protects the reports that the Administrator generates. Ignored when `reportPDFPasswordEnabled` is `false`. | [optional]
**created** | Option<**i64**> | Timestamp when the Administrator was created, in milliseconds since epoch. Searchable as Date. | [optional][readonly]
**last_password_change** | Option<**i64**> | Timestamp when the Administrator's password was last changed, in milliseconds since epoch. Searchable as Date. | [optional][readonly]
**last_sign_in** | Option<**i64**> | Timestamp when the Administrator last signed in, in milliseconds since epoch. Searchable as Date. | [optional][readonly]
**unlock_time** | Option<**i64**> | Timestamp at which the Administrator will be unlocked, in milliseconds since epoch. Ignored if the Administrator is not locked out using a time-based lock out scheme. Searchable as Date. | [optional][readonly]
**unsuccessful_sign_in_attempts** | Option<**i32**> | Number of unsuccessful sign-in attempts for the Administrator. This number is reset to `0` when a successful authentication occurs. Searchable as Numeric. | [optional][readonly]
**directory_name** | Option<**String**> | Security Account Manager (SAM) account name for the Administrator. Ignored if the Administrator is not managed by an identity provider. Searchable as String. | [optional][readonly]
**directory_info** | Option<**String**> | Unique ID used for single sign-on using a Security Account Manager (SAM) identity provider. Ignored if the Administrator is not managed by an identity provider. | [optional][readonly]
**external** | Option<**bool**> | If set to `true` the Administrator is externally authenticated using SAML. Defaults to `false`. Ignored if the Administrator is not externally authenticated. Searchable as Boolean. | [optional][readonly]
**external_user_id** | Option<**String**> | SAML User ID of the Administrator. Used to support external authentication of the Administrator. Ignored if the Administrator is not externally authenticated. Searchable as String. | [optional][readonly]
**_type** | Option<**String**> | Administrator account type. Can either be `normal` or `temporary`. Defaults to `normal`. Searchable as Choice. | [optional][readonly]
**read_only** | Option<**bool**> | Set to `true` if the Administrator is read-only. Defaults to `false`. Searchable as Boolean. | [optional][readonly]
**ID** | Option<**i32**> | ID of the Administrator. Searchable as ID. | [optional][readonly]
**utc_offset** | Option<**String**> | UTC offset of the Administrator. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



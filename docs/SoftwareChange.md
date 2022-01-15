# SoftwareChange

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**computer_id** | Option<**i32**> | ID of the computer on which the software change was found. Searchable as Numeric. | [optional][readonly]
**computer_group_id** | Option<**i32**> | ID of the computer group where the software change was found. Searchable as Numeric. | [optional][readonly]
**vendor_name** | Option<**String**> | Vendor name associated with the changed software. Searchable as String. | [optional][readonly]
**product_name** | Option<**String**> | Product name associated with the changed software. Searchable as String. | [optional][readonly]
**product_version** | Option<**String**> | Product version associated with the changed software. Searchable as String. | [optional][readonly]
**file_version** | Option<**String**> | File version associated with the changed software. Searchable as String. | [optional][readonly]
**file_description** | Option<**String**> | File description associated with the changed software. Searchable as String. | [optional][readonly]
**sha1** | Option<**String**> | SHA1 hash calculated from the changed software. Searchable as String. | [optional][readonly]
**sha256** | Option<**String**> | SHA256 hash calculated from the changed software. Searchable as String. | [optional][readonly]
**md5** | Option<**String**> | MD5 hash calculated from the changed software. Searchable as String. | [optional][readonly]
**file_name** | Option<**String**> | File name of the changed software. Searchable as String. | [optional][readonly]
**install_path** | Option<**String**> | Path on which the software change was found. Searchable as String. | [optional][readonly]
**file_size** | Option<**i64**> | File size of the changed software in bytes. Searchable as Numeric. | [optional][readonly]
**change_event_time** | Option<**i64**> | Time the software change was discovered, in milliseconds since epoch. Searchable as Date. | [optional][readonly]
**change_by_user** | Option<**String**> | Username of the user that introduced the software change. Searchable as String. | [optional][readonly]
**change_by_user_id** | Option<**String**> | User ID of the user that introduced the software change. Searchable as String. | [optional][readonly]
**change_by_process** | Option<**String**> | Name of the process that introduced the software change. Searchable as String. | [optional][readonly]
**change_by_process_id** | Option<**String**> | Process ID of the process that introduced the software change. Searchable as String. | [optional][readonly]
**ID** | Option<**i64**> | ID of software change. Searchable as Numeric. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



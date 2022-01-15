# IntegrityMonitoringRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the IntegrityMonitoringRule. Searchable as String. | [optional]
**description** | Option<**String**> | Description of the IntegrityMonitoringRule. Searchable as String. | [optional]
**minimum_agent_version** | Option<**String**> | Minimum Deep Security Agent version that supports the IntegrityMonitoringRule. This value is provided in the X.X.X.X format. Defaults to `6.0.0.0`. If an agent is not the minimum required version, the manager does not send the rule to the agent, and generates an alert. Searchable as String. | [optional][readonly]
**minimum_manager_version** | Option<**String**> | Minimum Deep Security Manager version that supports the IntegrityMonitoringRule. This value is provided in the X.X.X format. Defaults to `6.0.0`. An alert will be raised if a manager that fails to meet the minimum manager version value tries to assign this rule to a host or profile. Searchable as String. | [optional][readonly]
**severity** | Option<**String**> | Severity level of the event is multiplied by the computer's asset value to determine ranking. Ranking can be used to sort events with more business impact. Searchable as Choice. | [optional]
**_type** | Option<**String**> | Type of the IntegrityMonitoringRule. If the rule is predefined by Trend Micro, it is set to `2`. If it is user created, it is set to `1`. Searchable as String. | [optional][readonly]
**original_issue** | Option<**i64**> | Timestamp when the IntegrityMonitoringRule was originally issued by Trend Micro, in milliseconds since epoch.  Empty if the IntegrityMonitoringRule is user created. Searchable as Date. | [optional][readonly]
**last_updated** | Option<**i64**> | Timestamp when the IntegrityMonitoringRule was last updated, in milliseconds since epoch. Searchable as Date. | [optional][readonly]
**identifier** | Option<**String**> | Identifier of the IntegrityMonitoringRule from Trend Micro. Empty if the IntegrityMonitoringRule is user created. Searchable as String. | [optional][readonly]
**template** | Option<**String**> | Template which the IntegrityMonitoringRule follows. | [optional]
**registry_key_root** | Option<**String**> | Registry hive which is monitored by the IntegrityMonitoringRule. Empty if the IntegrityMonitoringRule does not monitor a registry key. | [optional]
**registry_key_value** | Option<**String**> | Registry key which is monitored by the IntegrityMonitoringRule. Empty if the IntegrityMonitoringRule does not monitor a registry key. Ignored if the IntegrityMonitoringRule does not monitor a registry key. | [optional]
**registry_include_sub_keys** | Option<**bool**> | Controls whether the IntegrityMonitoringRule should also include subkeys of the registry key it monitors. Defaults to `false`. Ignored if the IntegrityMonitoringRule does not monitor a registry key. | [optional]
**registry_included_values** | Option<**Vec<String>**> | Registry key values to be monitored by the IntegrityMonitoringRule. JSON array or delimited by `\\n`. `?` matches a single character, while `*` matches zero or more characters. Ignored if the IntegrityMonitoringRule does not monitor a registry key. | [optional]
**registry_include_default_value** | Option<**bool**> | Controls whether the rule should monitor default registry key values. Defaults to `true`. Ignored if the IntegrityMonitoringRule does not monitor a registry key. | [optional]
**registry_excluded_values** | Option<**Vec<String>**> | Registry key values to be ignored by the IntegrityMonitoringRule. JSON array or delimited by `\\n`. `?` matches a single character, while `*` matches zero or more characters. Ignored if the IntegrityMonitoringRule does not monitor a registry key. | [optional]
**registry_attributes** | Option<**Vec<String>**> | Registry key attributes to be monitored by the IntegrityMonitoringRule. JSON array or delimited by `\\n`. Defaults to `STANDARD` which will monitor changes in registry size, content and type. Ignored if the IntegrityMonitoringRule does not monitor a registry key. | [optional]
**file_base_directory** | Option<**String**> | Base of the file directory to be monitored by the IntegrityMonitoringRule. Ignored if the IntegrityMonitoringRule does not monitor a file directory. | [optional]
**file_include_sub_directories** | Option<**bool**> | Controls whether the IntegrityMonitoringRule should also monitor sub-directories of the base file directory that is associated with it. Defaults to `false`. Ignored if the IntegrityMonitoringRule does not monitor a file directory. | [optional]
**file_included_values** | Option<**Vec<String>**> | File name values to be monitored by the IntegrityMonitoringRule. JSON array or delimited by `\\n`. `?` matches a single character, while `*` matches zero or more characters. Leaving this field blank when monitoring file directories will cause the IntegrityMonitoringRule to monitor all files in a directory. This can use significant system resources if the base directory contains numerous or large files. Ignored if the IntegrityMonitoringRule does not monitor a file directory. | [optional]
**file_excluded_values** | Option<**Vec<String>**> | File name values to be ignored by the IntegrityMonitoringRule. JSON array or delimited by `\\n`. `?` matches a single character, while `*` matches zero or more characters. Ignored if the IntegrityMonitoringRule does not monitor a file directory. | [optional]
**file_attributes** | Option<**Vec<String>**> | File attributes to be monitored by the IntegrityMonitoringRule. JSON array or delimited by `\\n`. Defaults to `STANDARD` which will monitor changes in file creation date, last modified date, permissions, owner, group, size, content, flags (Windows) and SymLinkPath (Linux). Ignored if the IntegrityMonitoringRule does not monitor a file directory. | [optional]
**custom_xml** | Option<**String**> | Custom XML rules to be used by the IntegrityMonitoringRule. Custom XML rules must be encoded in the Base64 format. Ignored if the IntegrityMonitoringRule does not follow the `custom` template. | [optional]
**alert_enabled** | Option<**bool**> | Controls whether an alert should be made if an event related to the IntegrityMonitoringRule is logged. Defaults to `false`. Searchable as Boolean. | [optional]
**real_time_monitoring_enabled** | Option<**bool**> | Controls whether the IntegrityMonitoringRule is monitored in real time or during every scan. Defaults to `true` which indicates that it is monitored in real time. A value of `false` indicates that it will only be checked during scans. Searchable as Boolean. | [optional]
**recommendations_mode** | Option<**String**> | Indicates whether recommendation scans consider the IntegrityMonitoringRule. Can be set to enabled or ignored. Custom rules cannot be recommended. Searchable as Choice. | [optional]
**ID** | Option<**i32**> | ID of the IntegrityMonitoringRule. Searchable as ID. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



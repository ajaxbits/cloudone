# IntrusionPreventionRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the IntrusionPreventionRule. Searchable as String. | [optional]
**description** | Option<**String**> | Description of the IntrusionPreventionRule. Searchable as String. | [optional]
**minimum_agent_version** | Option<**String**> | Version of the Deep Security agent or appliance required to support the rule. Searchable as String. | [optional]
**application_type_id** | Option<**i32**> | ID of the application type for the IntrusionPreventionRule. Searchable as Numeric. | [optional]
**priority** | Option<**String**> | Priority level of the rule. Higher priority rules are applied before lower priority rules. Searchable as Choice. | [optional]
**severity** | Option<**String**> | Severity level of the rule. Severity levels can be used as sorting criteria and affect event rankings. Searchable as Choice. | [optional]
**detect_only** | Option<**bool**> | In detect mode, the rule creates an event log and does not interfere with traffic. | [optional]
**event_logging_disabled** | Option<**bool**> | Enable to prevent event logs from being created when the rule is triggered. Not available if detectOnly is true. Searchable as Boolean. | [optional]
**generate_event_on_packet_drop** | Option<**bool**> | Generate an event every time a packet is dropped for the rule. Not available if eventLoggingDisabled is true. Searchable as Boolean. | [optional]
**always_include_packet_data** | Option<**bool**> | Enabled to include package data in the event logs. Not available if eventLoggingDisabled is true. Searchable as Boolean. | [optional]
**debug_mode_enabled** | Option<**bool**> | Enable to log additional packets preceeding and following the packet that the rule detected. Not available if eventLoggingDisabled is true. Searchable as Boolean. | [optional]
**_type** | Option<**String**> | Type of IntrusionPreventionRule. Searchable as Choice. | [optional]
**original_issue** | Option<**i64**> | Timestamp of the date the rule was released, in milliseconds since epoch. Searchable as Date. | [optional]
**last_updated** | Option<**i64**> | Timestamp of the last rule modification, in milliseconds since epoch. Searchable as Date. | [optional]
**identifier** | Option<**String**> | Unique identification tag of the rule. Searchable as String. | [optional][readonly]
**template** | Option<**String**> | Type of template for the IntrusionPreventionRule. Applicable only to custom rules. | [optional]
**signature** | Option<**String**> | Signature of the rule. Applicable to custom rules with template type signature. | [optional]
**start** | Option<**String**> | Start pattern of the rule. Applicable to custom rules with template type start-end-patterns. | [optional]
**patterns** | Option<**Vec<String>**> | Body patterns of the rule, which must be found between start and end patterns. Applicable to custom rules with template type start-end-patterns. | [optional]
**end** | Option<**String**> | End pattern of the rule. Applicable to custom rules with template type start-end-patterns. | [optional]
**case_sensitive** | Option<**bool**> | Enable to make signatures and patterns case sensitive. Applicable to custom rules with template type signature or start-end-patterns. | [optional]
**condition** | Option<**String**> | Condition to determine if the rule is triggered. Applicable to custom rules with template type start-end-patterns. | [optional]
**action** | Option<**String**> | Action to apply if the rule is triggered. Applicable to custom rules with template type signature or start-end-patterns. | [optional]
**custom_xml** | Option<**String**> | The custom XML used to define the rule. Applicable to custom rules with template type custom. | [optional]
**alert_enabled** | Option<**bool**> | Enable to raise an alert when the rule logs an event. Searchable as Boolean. | [optional]
**schedule_id** | Option<**i32**> | ID of the schedule which defines times during which the rule is active. Set to 0 to remove any assignment. Searchable as Numeric. | [optional]
**context_id** | Option<**i32**> | ID of the context in which the rule is applied. Set to 0 to remove any assignment. Searchable as Numeric. | [optional]
**recommendations_mode** | Option<**String**> | Indicates whether recommendation scans consider the IntrusionPreventionRule. Can be set to enabled or ignored. Custom rules cannot be recommended. Searchable as Choice. | [optional]
**can_be_assigned_alone** | Option<**bool**> | True when the rule has no dependencies. | [optional][readonly]
**depends_on_rule_ids** | Option<**Vec<i32>**> | IDs of intrusion prevention rules the rule depends on, which will be automatically assigned if this rule is assigned. | [optional]
**ID** | Option<**i32**> | ID of the IntrusionPreventionRule. Searchable as ID. | [optional][readonly]
**cvss_score** | Option<**String**> | A measure of the severity of the vulnerability according the National Vulnerability Database. Searchable as String or as Numeric. | [optional]
**CVE** | Option<**Vec<String>**> | List of CVEs associated with the IntrusionPreventionRule. Searchable as String. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



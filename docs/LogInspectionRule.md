# LogInspectionRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the LogInspectionRule. Searchable as String. | [optional]
**description** | Option<**String**> | Description of the LogInspectionRule that appears in search results, and on the General tab in the Deep Security Manager user interface. Searchable as String. | [optional]
**minimum_agent_version** | Option<**String**> | Minimum Deep Security Agent version required by the LogInspectionRule. Searchable as String. | [optional]
**minimum_manager_version** | Option<**String**> | Minimumn Deep Security Manager version required by the LogInspectionRule. Searchable as String. | [optional]
**_type** | Option<**String**> | Type of the LogInspectionRule. The value 'Defined' is used for LogInspectionRules provided by Trend Micro. Searchable as String. | [optional]
**original_issue** | Option<**i64**> | Creation timestamp of the LogInspectionRule, measured in milliseconds since epoch. Searchable as Date. | [optional]
**last_updated** | Option<**i64**> | Update timestamp of the LogInspectionRule, measured in milliseconds since epoch. Searchable as Date. | [optional]
**identifier** | Option<**String**> | Indentifier of the LogInspectionRule used in the Deep Security Manager user interface. Searchable as String. | [optional]
**template** | Option<**String**> | Template used to create this rule. | [optional]
**rule_id** | Option<**i32**> | ID of the LogInspectionRule sent to the Deep Security Agent. The values 100000 - 109999 are reserved for user-definded rules. | [optional]
**level** | Option<**i32**> | Log level of the LogInspectionRule indicates severity of attack. Level 0 is the least severe and will not log an event. Level 15 is the most severe. | [optional]
**groups** | Option<**Vec<String>**> | Groups that the LogInspectionRule is assigned to, separated by commas. Useful when dependency is used as it's possible to create a LogInspectionRule that fires when another LogInspectionRule belonging to a specific group fires. | [optional]
**rule_description** | Option<**String**> | Description of the LogInspectionRule that appears on events and the Content tab in the Deep Security Manager user interface. Alternatively, you can configure this by inserting a description in 'ruleXML'. | [optional]
**pattern** | Option<**String**> | Regular expression pattern the LogInspectionRule will look for in the logs. The rule will be triggered on a match. Open Source HIDS SEcurity (OSSEC) regular expression syntax is supported, see http://www.ossec.net/docs/syntax/regex.html. | [optional]
**pattern_type** | Option<**String**> | Pattern the LogInspectionRule will look for in the logs. The string matching pattern is faster than the regex pattern. | [optional]
**dependency** | Option<**String**> | Indicates if a dependant rule or dependency group is set or not. If set, the LogInspectionRule will only log an event if the dependency is triggered. Available for user-defined rules. | [optional]
**dependency_rule_id** | Option<**i32**> | If dependency is configured, the ID of the rule that this rule is dependant on. Ignored if the rule is from Trend Micro, which uses `dependsOnRuleIDs` instead. | [optional]
**dependency_group** | Option<**String**> | If dependency is configured, the dependancy groups that this rule is dependant on. | [optional]
**frequency** | Option<**i32**> | Number of times the dependant rule has to match within a specific time frame before the rule is triggered. | [optional]
**time_frame** | Option<**i32**> | Time period for the frequency of LogInspectionRule triggers that will generate an event, in seconds. | [optional]
**rule_xml** | Option<**String**> | LogInspectionRule in an XML format. For information on the XML format, see http://ossec-docs.readthedocs.io/en/latest/syntax/head_rules.html | [optional]
**log_files** | Option<[**crate::models::LogFiles**](LogFiles.md)> |  | [optional]
**alert_enabled** | Option<**bool**> | Controls whether to raise an alert when a LogInspectionRule logs an event. Use true to raise an alert. Searchable as Boolean. | [optional]
**alert_minimum_severity** | Option<**i32**> | Severity level that will trigger an alert. Ignored unless `ruleXML` contains multiple rules with different severities, and so you must indicate which severity level to use. Searchable as Numeric. | [optional]
**recommendations_mode** | Option<**String**> | Indicates whether recommendation scans consider the LogInspectionRule. Can be set to enabled or ignored. Custom rules cannot be recommended. Searchable as Choice. | [optional]
**sort_order** | Option<**i32**> | Order in which LogInspectionRules are sent to the Deep Security Agent. Log inspeciton rules are sent in ascending order. Valid values are between 10000 and 20000. | [optional]
**can_be_assigned_alone** | Option<**bool**> | Indicates whether this LogInspectionRule can be allocated without allocating any additional LogInspectionRules. Ignored if the rule is user-defined, which uses `dependency` instead. | [optional][readonly]
**depends_on_rule_ids** | Option<**Vec<i32>**> | IDs of LogInspectionRules, separated by commas, that are required by this rule. Ignored if the rule is user-defined, which uses `dependencyRuleID` or `dependencyGroup` instead. | [optional][readonly]
**ID** | Option<**i32**> | ID of the LogInspectionRule. This number is set automatically. Searchable as ID. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



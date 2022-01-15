/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IntegrityMonitoringRule : Integrity monitoring rules describe how Deep Security Agents should scan for and detect changes to a computer's files, directories and registry keys and values as well as changes in installed software, processes, listening ports and running services. Integrity monitoring rules can be assigned directly to computers or can be made part of a policy.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrityMonitoringRule {
    /// Name of the IntegrityMonitoringRule. Searchable as String.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description of the IntegrityMonitoringRule. Searchable as String.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Minimum Deep Security Agent version that supports the IntegrityMonitoringRule. This value is provided in the X.X.X.X format. Defaults to `6.0.0.0`. If an agent is not the minimum required version, the manager does not send the rule to the agent, and generates an alert. Searchable as String.
    #[serde(rename = "minimumAgentVersion", skip_serializing_if = "Option::is_none")]
    pub minimum_agent_version: Option<String>,
    /// Minimum Deep Security Manager version that supports the IntegrityMonitoringRule. This value is provided in the X.X.X format. Defaults to `6.0.0`. An alert will be raised if a manager that fails to meet the minimum manager version value tries to assign this rule to a host or profile. Searchable as String.
    #[serde(rename = "minimumManagerVersion", skip_serializing_if = "Option::is_none")]
    pub minimum_manager_version: Option<String>,
    /// Severity level of the event is multiplied by the computer's asset value to determine ranking. Ranking can be used to sort events with more business impact. Searchable as Choice.
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<Severity>,
    /// Type of the IntegrityMonitoringRule. If the rule is predefined by Trend Micro, it is set to `2`. If it is user created, it is set to `1`. Searchable as String.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// Timestamp when the IntegrityMonitoringRule was originally issued by Trend Micro, in milliseconds since epoch.  Empty if the IntegrityMonitoringRule is user created. Searchable as Date.
    #[serde(rename = "originalIssue", skip_serializing_if = "Option::is_none")]
    pub original_issue: Option<i64>,
    /// Timestamp when the IntegrityMonitoringRule was last updated, in milliseconds since epoch. Searchable as Date.
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<i64>,
    /// Identifier of the IntegrityMonitoringRule from Trend Micro. Empty if the IntegrityMonitoringRule is user created. Searchable as String.
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Template which the IntegrityMonitoringRule follows.
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<Template>,
    /// Registry hive which is monitored by the IntegrityMonitoringRule. Empty if the IntegrityMonitoringRule does not monitor a registry key.
    #[serde(rename = "registryKeyRoot", skip_serializing_if = "Option::is_none")]
    pub registry_key_root: Option<String>,
    /// Registry key which is monitored by the IntegrityMonitoringRule. Empty if the IntegrityMonitoringRule does not monitor a registry key. Ignored if the IntegrityMonitoringRule does not monitor a registry key.
    #[serde(rename = "registryKeyValue", skip_serializing_if = "Option::is_none")]
    pub registry_key_value: Option<String>,
    /// Controls whether the IntegrityMonitoringRule should also include subkeys of the registry key it monitors. Defaults to `false`. Ignored if the IntegrityMonitoringRule does not monitor a registry key.
    #[serde(rename = "registryIncludeSubKeys", skip_serializing_if = "Option::is_none")]
    pub registry_include_sub_keys: Option<bool>,
    /// Registry key values to be monitored by the IntegrityMonitoringRule. JSON array or delimited by `\\n`. `?` matches a single character, while `*` matches zero or more characters. Ignored if the IntegrityMonitoringRule does not monitor a registry key.
    #[serde(rename = "registryIncludedValues", skip_serializing_if = "Option::is_none")]
    pub registry_included_values: Option<Vec<String>>,
    /// Controls whether the rule should monitor default registry key values. Defaults to `true`. Ignored if the IntegrityMonitoringRule does not monitor a registry key.
    #[serde(rename = "registryIncludeDefaultValue", skip_serializing_if = "Option::is_none")]
    pub registry_include_default_value: Option<bool>,
    /// Registry key values to be ignored by the IntegrityMonitoringRule. JSON array or delimited by `\\n`. `?` matches a single character, while `*` matches zero or more characters. Ignored if the IntegrityMonitoringRule does not monitor a registry key.
    #[serde(rename = "registryExcludedValues", skip_serializing_if = "Option::is_none")]
    pub registry_excluded_values: Option<Vec<String>>,
    /// Registry key attributes to be monitored by the IntegrityMonitoringRule. JSON array or delimited by `\\n`. Defaults to `STANDARD` which will monitor changes in registry size, content and type. Ignored if the IntegrityMonitoringRule does not monitor a registry key.
    #[serde(rename = "registryAttributes", skip_serializing_if = "Option::is_none")]
    pub registry_attributes: Option<Vec<String>>,
    /// Base of the file directory to be monitored by the IntegrityMonitoringRule. Ignored if the IntegrityMonitoringRule does not monitor a file directory.
    #[serde(rename = "fileBaseDirectory", skip_serializing_if = "Option::is_none")]
    pub file_base_directory: Option<String>,
    /// Controls whether the IntegrityMonitoringRule should also monitor sub-directories of the base file directory that is associated with it. Defaults to `false`. Ignored if the IntegrityMonitoringRule does not monitor a file directory.
    #[serde(rename = "fileIncludeSubDirectories", skip_serializing_if = "Option::is_none")]
    pub file_include_sub_directories: Option<bool>,
    /// File name values to be monitored by the IntegrityMonitoringRule. JSON array or delimited by `\\n`. `?` matches a single character, while `*` matches zero or more characters. Leaving this field blank when monitoring file directories will cause the IntegrityMonitoringRule to monitor all files in a directory. This can use significant system resources if the base directory contains numerous or large files. Ignored if the IntegrityMonitoringRule does not monitor a file directory.
    #[serde(rename = "fileIncludedValues", skip_serializing_if = "Option::is_none")]
    pub file_included_values: Option<Vec<String>>,
    /// File name values to be ignored by the IntegrityMonitoringRule. JSON array or delimited by `\\n`. `?` matches a single character, while `*` matches zero or more characters. Ignored if the IntegrityMonitoringRule does not monitor a file directory.
    #[serde(rename = "fileExcludedValues", skip_serializing_if = "Option::is_none")]
    pub file_excluded_values: Option<Vec<String>>,
    /// File attributes to be monitored by the IntegrityMonitoringRule. JSON array or delimited by `\\n`. Defaults to `STANDARD` which will monitor changes in file creation date, last modified date, permissions, owner, group, size, content, flags (Windows) and SymLinkPath (Linux). Ignored if the IntegrityMonitoringRule does not monitor a file directory.
    #[serde(rename = "fileAttributes", skip_serializing_if = "Option::is_none")]
    pub file_attributes: Option<Vec<String>>,
    /// Custom XML rules to be used by the IntegrityMonitoringRule. Custom XML rules must be encoded in the Base64 format. Ignored if the IntegrityMonitoringRule does not follow the `custom` template.
    #[serde(rename = "customXML", skip_serializing_if = "Option::is_none")]
    pub custom_xml: Option<String>,
    /// Controls whether an alert should be made if an event related to the IntegrityMonitoringRule is logged. Defaults to `false`. Searchable as Boolean.
    #[serde(rename = "alertEnabled", skip_serializing_if = "Option::is_none")]
    pub alert_enabled: Option<bool>,
    /// Controls whether the IntegrityMonitoringRule is monitored in real time or during every scan. Defaults to `true` which indicates that it is monitored in real time. A value of `false` indicates that it will only be checked during scans. Searchable as Boolean.
    #[serde(rename = "realTimeMonitoringEnabled", skip_serializing_if = "Option::is_none")]
    pub real_time_monitoring_enabled: Option<bool>,
    /// Indicates whether recommendation scans consider the IntegrityMonitoringRule. Can be set to enabled or ignored. Custom rules cannot be recommended. Searchable as Choice.
    #[serde(rename = "recommendationsMode", skip_serializing_if = "Option::is_none")]
    pub recommendations_mode: Option<RecommendationsMode>,
    /// ID of the IntegrityMonitoringRule. Searchable as ID.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<i32>,
}

impl IntegrityMonitoringRule {
    /// Integrity monitoring rules describe how Deep Security Agents should scan for and detect changes to a computer's files, directories and registry keys and values as well as changes in installed software, processes, listening ports and running services. Integrity monitoring rules can be assigned directly to computers or can be made part of a policy.
    pub fn new() -> IntegrityMonitoringRule {
        IntegrityMonitoringRule {
            name: None,
            description: None,
            minimum_agent_version: None,
            minimum_manager_version: None,
            severity: None,
            _type: None,
            original_issue: None,
            last_updated: None,
            identifier: None,
            template: None,
            registry_key_root: None,
            registry_key_value: None,
            registry_include_sub_keys: None,
            registry_included_values: None,
            registry_include_default_value: None,
            registry_excluded_values: None,
            registry_attributes: None,
            file_base_directory: None,
            file_include_sub_directories: None,
            file_included_values: None,
            file_excluded_values: None,
            file_attributes: None,
            custom_xml: None,
            alert_enabled: None,
            real_time_monitoring_enabled: None,
            recommendations_mode: None,
            ID: None,
        }
    }
}

/// Severity level of the event is multiplied by the computer's asset value to determine ranking. Ranking can be used to sort events with more business impact. Searchable as Choice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Severity {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "critical")]
    Critical,
}
/// Template which the IntegrityMonitoringRule follows.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Template {
    #[serde(rename = "registry")]
    Registry,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "custom")]
    Custom,
}
/// Indicates whether recommendation scans consider the IntegrityMonitoringRule. Can be set to enabled or ignored. Custom rules cannot be recommended. Searchable as Choice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RecommendationsMode {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "disabled")]
    Disabled,
}

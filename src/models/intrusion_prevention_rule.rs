/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IntrusionPreventionRule : Intrusion Prevention rule details. Intrusion prevention rules define a set of conditions that are compared to the payload session and application layers of network packets (such as DNS, HTTP, SSL, and SMTP), as well as the sequence of those packets according to those higher-layer protocols.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntrusionPreventionRule {
    /// Name of the IntrusionPreventionRule. Searchable as String.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description of the IntrusionPreventionRule. Searchable as String.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Version of the Deep Security agent or appliance required to support the rule. Searchable as String.
    #[serde(rename = "minimumAgentVersion", skip_serializing_if = "Option::is_none")]
    pub minimum_agent_version: Option<String>,
    /// ID of the application type for the IntrusionPreventionRule. Searchable as Numeric.
    #[serde(rename = "applicationTypeID", skip_serializing_if = "Option::is_none")]
    pub application_type_id: Option<i32>,
    /// Priority level of the rule. Higher priority rules are applied before lower priority rules. Searchable as Choice.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    /// Severity level of the rule. Severity levels can be used as sorting criteria and affect event rankings. Searchable as Choice.
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<Severity>,
    /// In detect mode, the rule creates an event log and does not interfere with traffic.
    #[serde(rename = "detectOnly", skip_serializing_if = "Option::is_none")]
    pub detect_only: Option<bool>,
    /// Enable to prevent event logs from being created when the rule is triggered. Not available if detectOnly is true. Searchable as Boolean.
    #[serde(rename = "eventLoggingDisabled", skip_serializing_if = "Option::is_none")]
    pub event_logging_disabled: Option<bool>,
    /// Generate an event every time a packet is dropped for the rule. Not available if eventLoggingDisabled is true. Searchable as Boolean.
    #[serde(rename = "generateEventOnPacketDrop", skip_serializing_if = "Option::is_none")]
    pub generate_event_on_packet_drop: Option<bool>,
    /// Enabled to include package data in the event logs. Not available if eventLoggingDisabled is true. Searchable as Boolean.
    #[serde(rename = "alwaysIncludePacketData", skip_serializing_if = "Option::is_none")]
    pub always_include_packet_data: Option<bool>,
    /// Enable to log additional packets preceeding and following the packet that the rule detected. Not available if eventLoggingDisabled is true. Searchable as Boolean.
    #[serde(rename = "debugModeEnabled", skip_serializing_if = "Option::is_none")]
    pub debug_mode_enabled: Option<bool>,
    /// Type of IntrusionPreventionRule. Searchable as Choice.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// Timestamp of the date the rule was released, in milliseconds since epoch. Searchable as Date.
    #[serde(rename = "originalIssue", skip_serializing_if = "Option::is_none")]
    pub original_issue: Option<i64>,
    /// Timestamp of the last rule modification, in milliseconds since epoch. Searchable as Date.
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<i64>,
    /// Unique identification tag of the rule. Searchable as String.
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Type of template for the IntrusionPreventionRule. Applicable only to custom rules.
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<Template>,
    /// Signature of the rule. Applicable to custom rules with template type signature.
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// Start pattern of the rule. Applicable to custom rules with template type start-end-patterns.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// Body patterns of the rule, which must be found between start and end patterns. Applicable to custom rules with template type start-end-patterns.
    #[serde(rename = "patterns", skip_serializing_if = "Option::is_none")]
    pub patterns: Option<Vec<String>>,
    /// End pattern of the rule. Applicable to custom rules with template type start-end-patterns.
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Enable to make signatures and patterns case sensitive. Applicable to custom rules with template type signature or start-end-patterns.
    #[serde(rename = "caseSensitive", skip_serializing_if = "Option::is_none")]
    pub case_sensitive: Option<bool>,
    /// Condition to determine if the rule is triggered. Applicable to custom rules with template type start-end-patterns.
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    /// Action to apply if the rule is triggered. Applicable to custom rules with template type signature or start-end-patterns.
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    /// The custom XML used to define the rule. Applicable to custom rules with template type custom.
    #[serde(rename = "customXML", skip_serializing_if = "Option::is_none")]
    pub custom_xml: Option<String>,
    /// Enable to raise an alert when the rule logs an event. Searchable as Boolean.
    #[serde(rename = "alertEnabled", skip_serializing_if = "Option::is_none")]
    pub alert_enabled: Option<bool>,
    /// ID of the schedule which defines times during which the rule is active. Set to 0 to remove any assignment. Searchable as Numeric.
    #[serde(rename = "scheduleID", skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<i32>,
    /// ID of the context in which the rule is applied. Set to 0 to remove any assignment. Searchable as Numeric.
    #[serde(rename = "contextID", skip_serializing_if = "Option::is_none")]
    pub context_id: Option<i32>,
    /// Indicates whether recommendation scans consider the IntrusionPreventionRule. Can be set to enabled or ignored. Custom rules cannot be recommended. Searchable as Choice.
    #[serde(rename = "recommendationsMode", skip_serializing_if = "Option::is_none")]
    pub recommendations_mode: Option<RecommendationsMode>,
    /// True when the rule has no dependencies.
    #[serde(rename = "canBeAssignedAlone", skip_serializing_if = "Option::is_none")]
    pub can_be_assigned_alone: Option<bool>,
    /// IDs of intrusion prevention rules the rule depends on, which will be automatically assigned if this rule is assigned.
    #[serde(rename = "dependsOnRuleIDs", skip_serializing_if = "Option::is_none")]
    pub depends_on_rule_ids: Option<Vec<i32>>,
    /// ID of the IntrusionPreventionRule. Searchable as ID.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<i32>,
    /// A measure of the severity of the vulnerability according the National Vulnerability Database. Searchable as String or as Numeric.
    #[serde(rename = "CVSSScore", skip_serializing_if = "Option::is_none")]
    pub cvss_score: Option<String>,
    /// List of CVEs associated with the IntrusionPreventionRule. Searchable as String.
    #[serde(rename = "CVE", skip_serializing_if = "Option::is_none")]
    pub CVE: Option<Vec<String>>,
}

impl IntrusionPreventionRule {
    /// Intrusion Prevention rule details. Intrusion prevention rules define a set of conditions that are compared to the payload session and application layers of network packets (such as DNS, HTTP, SSL, and SMTP), as well as the sequence of those packets according to those higher-layer protocols.
    pub fn new() -> IntrusionPreventionRule {
        IntrusionPreventionRule {
            name: None,
            description: None,
            minimum_agent_version: None,
            application_type_id: None,
            priority: None,
            severity: None,
            detect_only: None,
            event_logging_disabled: None,
            generate_event_on_packet_drop: None,
            always_include_packet_data: None,
            debug_mode_enabled: None,
            _type: None,
            original_issue: None,
            last_updated: None,
            identifier: None,
            template: None,
            signature: None,
            start: None,
            patterns: None,
            end: None,
            case_sensitive: None,
            condition: None,
            action: None,
            custom_xml: None,
            alert_enabled: None,
            schedule_id: None,
            context_id: None,
            recommendations_mode: None,
            can_be_assigned_alone: None,
            depends_on_rule_ids: None,
            ID: None,
            cvss_score: None,
            CVE: None,
        }
    }
}

/// Priority level of the rule. Higher priority rules are applied before lower priority rules. Searchable as Choice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Priority {
    #[serde(rename = "lowest")]
    Lowest,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "highest")]
    Highest,
}
/// Severity level of the rule. Severity levels can be used as sorting criteria and affect event rankings. Searchable as Choice.
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
/// Type of IntrusionPreventionRule. Searchable as Choice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "smart")]
    Smart,
    #[serde(rename = "vulnerability")]
    Vulnerability,
    #[serde(rename = "exploit")]
    Exploit,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "policy")]
    Policy,
    #[serde(rename = "info")]
    Info,
}
/// Type of template for the IntrusionPreventionRule. Applicable only to custom rules.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Template {
    #[serde(rename = "signature")]
    Signature,
    #[serde(rename = "start-end-patterns")]
    StartEndPatterns,
    #[serde(rename = "custom")]
    Custom,
}
/// Condition to determine if the rule is triggered. Applicable to custom rules with template type start-end-patterns.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Condition {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "none")]
    None,
}
/// Action to apply if the rule is triggered. Applicable to custom rules with template type signature or start-end-patterns.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "drop")]
    Drop,
    #[serde(rename = "log-only")]
    LogOnly,
}
/// Indicates whether recommendation scans consider the IntrusionPreventionRule. Can be set to enabled or ignored. Custom rules cannot be recommended. Searchable as Choice.
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


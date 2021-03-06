/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// TimeRange : Details for a time range. A time range identifies the period of time for which a report is generated via a scheduled task.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeRange {
    /// Units for the time range
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<Units>,
    /// Value of the time range. With the units, defines the report's time range as the length of time previous to the current time.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

impl TimeRange {
    /// Details for a time range. A time range identifies the period of time for which a report is generated via a scheduled task.
    pub fn new() -> TimeRange {
        TimeRange {
            units: None,
            value: None,
        }
    }
}

/// Units for the time range
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Units {
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
}

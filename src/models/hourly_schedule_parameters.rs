/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HourlyScheduleParameters : Details for an hourly schedule for a scheduled task.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HourlyScheduleParameters {
    /// Minutes past the hour when the task should run.
    #[serde(rename = "minutesPastTheHour", skip_serializing_if = "Option::is_none")]
    pub minutes_past_the_hour: Option<MinutesPastTheHour>,
}

impl HourlyScheduleParameters {
    /// Details for an hourly schedule for a scheduled task.
    pub fn new() -> HourlyScheduleParameters {
        HourlyScheduleParameters {
            minutes_past_the_hour: None,
        }
    }
}

/// Minutes past the hour when the task should run.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MinutesPastTheHour {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "5")]
    _5,
    #[serde(rename = "10")]
    _10,
    #[serde(rename = "15")]
    _15,
    #[serde(rename = "20")]
    _20,
    #[serde(rename = "25")]
    _25,
    #[serde(rename = "30")]
    _30,
    #[serde(rename = "35")]
    _35,
    #[serde(rename = "40")]
    _40,
    #[serde(rename = "45")]
    _45,
    #[serde(rename = "50")]
    _50,
    #[serde(rename = "55")]
    _55,
}

